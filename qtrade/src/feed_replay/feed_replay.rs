//! Backtest-only feed mechanics: everything about turning a real MCX
//! recorded capture file into decoded events, that a live feed source
//! would never need. Deliberately declares nothing about `cache`,
//! `book`, or `execution` (same D10 independence `simulator.rs` already
//! established) -- this module only knows how to read bytes off disk and
//! resolve a day's own reference data, nothing about what a strategy
//! does with either.
//!
//! Split out of `dummy_strategy.rs` because one piece of it --
//! `scan_snapshot_for_bands` -- is *structurally* backtest-only: it
//! pre-scans a whole day's recorded file ahead of the main replay, which
//! has no live equivalent (live just listens for the one real broadcast,
//! in real time, same as the exchange's other real subscribers). A
//! strategy calling this directly would be calling something meaningless
//! outside backtest, which breaks the "same code goes live" goal. See
//! `feed_replay_user_doc.md` for the full account and `main.rs` for the
//! orchestrator that wires this module's output into `cache`/
//! `execution`.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::decoder;
use crate::refdata;
use crate::types::{InstrumentId, Venue};

/// Extracts `DD_MM_YYYY` from a real capture filename
/// (`mcx_feeder_Increment_capture_DD_MM_YYYY_1_N.bin`) and builds the
/// matching `CONTRACT` directory path, so pointing this at *any* real
/// capture file finds its own day's real reference data automatically --
/// no second date argument to keep in sync with the first, and no
/// hardcoded date. Real tokens are not stable across days (FR-16), so
/// which day's `MCXScrips.bcp` gets loaded matters for correctness, not
/// just convenience.
pub fn contract_dir_for(capture_path: &str) -> Option<String> {
    let name = Path::new(capture_path).file_name()?.to_str()?;
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() < 7 {
        return None;
    }
    Some(format!("/mnt/MCX_Recording_Files/CONTRACT/{}_{}_{}/MCXScrips.bcp", parts[4], parts[5], parts[6]))
}

/// Loads whichever day's real reference data `capture_path`'s own
/// filename implies -- the one place `contract_dir_for` and
/// `refdata::InstrumentMaster::load_mcx` meet, with a real error message
/// either can fail with.
pub fn load_refdata(capture_path: &str) -> io::Result<refdata::InstrumentMaster> {
    let refdata_path_string = contract_dir_for(capture_path).ok_or_else(|| {
        io::Error::new(
            ErrorKind::InvalidInput,
            format!("could not extract a date from {capture_path}'s filename (expected .../mcx_feeder_Increment_capture_DD_MM_YYYY_1_N.bin)"),
        )
    })?;
    refdata::InstrumentMaster::load_mcx(Path::new(&refdata_path_string))
        .map_err(|e| io::Error::new(ErrorKind::Other, format!("failed to load refdata at {refdata_path_string}: {e}")))
}

/// Resolves an underlying's name (e.g. "CRUDEOIL") to *that day's* real
/// front-month future token, from reference data already loaded for the
/// day in question. A strategy declares names, never tokens -- see e.g.
/// `limit_order_book_generator.rs`'s `UNDERLYINGS` -- since a token is
/// only ever meaningful for one specific day (FR-16).
pub fn resolve_front_month(master: &refdata::InstrumentMaster, underlying: &str) -> Option<InstrumentId> {
    master.instruments().venue(Venue::Mcx).underlying(underlying).kind_is_future().front_n_expiries(1).collect().into_iter().next()
}

/// The paired snapshot file's path for a real `Increment_capture` file,
/// or `None` if `capture_path` doesn't look like one (in which case
/// there's no full-session `InstrumentInfo` re-broadcast to scan --
/// see `scan_snapshot_for_bands`).
pub fn snapshot_path_for(capture_path: &str) -> Option<String> {
    let snapshot_path = capture_path.replacen("Increment_capture", "snapshot_capture", 1);
    if snapshot_path != capture_path {
        Some(snapshot_path)
    } else {
        None
    }
}

/// Streams one capture file's outer records off disk one at a time --
/// never holds more than one record's payload in memory at once (a full
/// day's file can run to 60GB+).
struct RecordSource {
    reader: BufReader<File>,
}

impl RecordSource {
    fn open(path: &str) -> io::Result<Self> {
        Ok(RecordSource { reader: BufReader::with_capacity(1 << 20, File::open(path)?) })
    }

    /// Reads the next outer record's payload into `payload`, replacing
    /// its contents. Returns `None` at a clean end-of-file, otherwise
    /// `Some(recorder_capture_ts)` -- the outer header's own second field,
    /// the recording server's real receipt time for this packet (dual-
    /// clock replay, 2026-08-27 planning session). Previously read into a
    /// local buffer and discarded; now a real value every caller can use.
    fn next_record(&mut self, payload: &mut Vec<u8>) -> io::Result<Option<i64>> {
        let mut hdr = [0u8; 16]; // [8B u64 LE length][8B i64 LE local capture timestamp]
        match self.reader.read_exact(&mut hdr) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }
        let length = u64::from_le_bytes(hdr[0..8].try_into().unwrap()) as usize;
        if length < 8 {
            return Ok(None);
        }
        let recorder_capture_ts = i64::from_le_bytes(hdr[8..16].try_into().unwrap());
        let payload_len = length - 8;
        payload.resize(payload_len, 0);
        self.reader.read_exact(payload)?;
        Ok(Some(recorder_capture_ts))
    }
}

/// Peeks an outer record's own starting `exchange_ts` from the leading
/// `PacketHeader` (template 13003, `TransactTime` at body offset 24 --
/// matches `decoder::decode_message`'s own `13003 => transact_time:
/// u64_le(m, 24)`) without a full decode. `last` carries the running
/// per-stream value forward for the (empirically never-seen, but
/// defended) case of a record with no leading PacketHeader, exactly
/// mirroring `replay`'s own inner-loop `exchange_ts` tracking -- so a
/// single-stream merge yields values byte-identical to the pre-merge
/// code.
fn peek_exchange_ts(payload: &[u8], last: &mut u64) -> u64 {
    if payload.len() >= 8 {
        let body_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
        let template_id = u16::from_le_bytes([payload[2], payload[3]]);
        if template_id == 13003 && body_len >= 32 && payload.len() >= 32 {
            *last = u64::from_le_bytes(payload[24..32].try_into().unwrap());
        }
    }
    *last
}

/// One stream's buffered head, plus the bookkeeping the merge needs.
/// `head_payload` and the caller's decode buffer swap back and forth
/// every `next()` (no per-record allocation after warmup, same
/// bounded-memory property as a single `RecordSource`).
struct MergeSlot {
    source: RecordSource,
    source_id: usize,
    last_exchange_ts: u64,
    head_payload: Vec<u8>,
    head_recorder_ts: i64,
    head_exchange_ts: u64,
    exhausted: bool,
}

/// K-way merge of N `RecordSource`s into one totally-ordered stream,
/// keyed on each outer record's own starting `exchange_ts`
/// (`PacketHeader.TransactTime`), tie-broken on `source_id` (the order
/// paths appear in the config). `exchange_ts` is monotonic non-decreasing
/// within one MCX stream, so the merged sequence is monotonic
/// non-decreasing too -- which is exactly the invariant `main.rs`'s
/// lookahead-drain already assumes, so it needs no change for N > 1.
/// The `(exchange_ts, source_id)` key is pure data, never thread/IO
/// timing, so a merge is fully reproducible (NFR-01).
///
/// N == 1 degrades to a plain one-record read-ahead over the single
/// source: same record order, same `(recorder_ts, exchange_ts)` values,
/// byte-identical `on_event` sequence as the pre-merge code.
struct MergeSource {
    slots: Vec<MergeSlot>,
}

impl MergeSource {
    fn open(paths: &[String]) -> io::Result<Self> {
        let mut slots = Vec::with_capacity(paths.len());
        for (source_id, path) in paths.iter().enumerate() {
            let mut slot = MergeSlot {
                source: RecordSource::open(path)?,
                source_id,
                last_exchange_ts: 0,
                head_payload: Vec::new(),
                head_recorder_ts: 0,
                head_exchange_ts: 0,
                exhausted: false,
            };
            Self::refill(&mut slot)?;
            slots.push(slot);
        }
        Ok(MergeSource { slots })
    }

    fn refill(slot: &mut MergeSlot) -> io::Result<()> {
        match slot.source.next_record(&mut slot.head_payload)? {
            Some(ts) => {
                slot.head_recorder_ts = ts;
                slot.head_exchange_ts = peek_exchange_ts(&slot.head_payload, &mut slot.last_exchange_ts);
            }
            None => slot.exhausted = true,
        }
        Ok(())
    }

    /// Yields the next outer record in merged order, swapping its payload
    /// into `out`. Returns `(recorder_ts, exchange_ts)` -- the same pair
    /// `RecordSource::next_record` + `peek_exchange_ts` would have given
    /// for that record read alone. `None` once every source is drained.
    fn next(&mut self, out: &mut Vec<u8>) -> io::Result<Option<(i64, u64)>> {
        let pick = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.exhausted)
            .min_by_key(|(_, s)| (s.head_exchange_ts, s.source_id))
            .map(|(i, _)| i);
        let Some(i) = pick else { return Ok(None) };
        let slot = &mut self.slots[i];
        std::mem::swap(out, &mut slot.head_payload);
        let yielded = (slot.head_recorder_ts, slot.head_exchange_ts);
        Self::refill(slot)?;
        Ok(Some(yielded))
    }
}

/// Streams a full real snapshot file (the whole thing, not a bounded
/// prefix -- a mid-session revision can arrive anywhere, per real
/// evidence already found for NATURALGAS on `19_01_2026`, which revised
/// six times across the session) and returns each tracked instrument's
/// real, full-session band as `(lower_raw, upper_raw, record_count)` --
/// the union across every real `InstrumentInfo` (13603) seen for it,
/// widest-so-far kept, same "widen, never narrow" rule `book`'s own
/// `learn_band` uses. Skips implausible records the same way `book`'s
/// own `plausible_band` does (a real, confirmed End-of-Day sentinel
/// artifact carries `i64::MIN`-adjacent fields, not a real band).
///
/// This is the piece that only makes sense in backtest: it looks ahead
/// through a whole day's recorded file before the main replay starts,
/// which has no live equivalent.
pub fn scan_snapshot_for_bands(path: &str, tracked_ids: &[InstrumentId]) -> io::Result<HashMap<InstrumentId, (i64, i64, u32)>> {
    let mut source = RecordSource::open(path)?;
    let mut payload = Vec::new();
    let mut bands: HashMap<InstrumentId, (i64, i64, u32)> = HashMap::new();
    let mut records = 0u64;
    let scan_started = Instant::now();

    while source.next_record(&mut payload)?.is_some() {
        records += 1;
        if records % 20_000_000 == 0 {
            eprintln!("  ... scanned {records} snapshot records, {:.1}s elapsed", scan_started.elapsed().as_secs_f64());
        }
        let mut off = 0usize;
        while off + 8 <= payload.len() {
            let body_len = u16::from_le_bytes([payload[off], payload[off + 1]]) as usize;
            let template_id = u16::from_le_bytes([payload[off + 2], payload[off + 3]]);
            if body_len == 0 || off + body_len > payload.len() {
                break;
            }
            if template_id == 13603 && body_len >= 48 {
                let sec_id = i64::from_le_bytes(payload[off + 8..off + 16].try_into().unwrap());
                let id = InstrumentId(sec_id as u32);
                if tracked_ids.contains(&id) {
                    let upper = i64::from_le_bytes(payload[off + 32..off + 40].try_into().unwrap());
                    let lower = i64::from_le_bytes(payload[off + 40..off + 48].try_into().unwrap());
                    let plausible = lower > i64::MIN / 2 && upper > i64::MIN / 2 && lower < upper;
                    if plausible {
                        bands
                            .entry(id)
                            .and_modify(|(l, u, c)| {
                                *l = (*l).min(lower);
                                *u = (*u).max(upper);
                                *c += 1;
                            })
                            .or_insert((lower, upper, 1));
                    }
                }
            }
            off += body_len;
        }
    }
    Ok(bands)
}

/// One decoded message from the main increment replay, plus the two real
/// clocks a caller needs that aren't the message itself -- a running
/// event counter, and the pair of genuine per-packet timestamps that
/// drive the dual-clock replay (2026-08-27 planning session):
///
/// `exchange_ts` is `PacketHeader.TransactTime` (template 13003) -- the
/// one per-message timestamp source this project has verified is safe to
/// use directly: some resting orders carry a sentinel instead of a real
/// time, and `Trade`'s own `event_time` field isn't a timestamp at all
/// (see `book_user_doc.md`'s `apply_trade` finding). This is what
/// `SimExchange` builds its book on -- the true, instant-by-instant
/// market.
///
/// `recorder_ts` is the outer record's own second header field -- the
/// recording server's real receipt time for this packet. Previously
/// decoded and discarded (`RecordSource::next_record` read it into a
/// buffer nobody consumed); now real. This is what `Cache`/`Strategy`
/// advance on -- the honest, always-lagging view a live strategy would
/// actually have. `recorder_ts - exchange_ts`, verified against real
/// `19_08_2026` data (500k packets), is real feed latency: always
/// positive, p50 ~2.5ms, p99 ~14.3ms, max ~59.4ms.
///
/// There is no synthetic clock left in this module at all -- both values
/// are genuine, monotonic, nanosecond-resolution numbers straight from
/// the capture file. The caller (`main.rs`) is responsible for validating
/// the delta (D20 fail-fast on an outlier) and for all scheduling
/// (`scheduler::Scheduler`) -- this function's own job stays exactly what
/// it was: stream the file, decode, hand back what was read.
///
/// `recorder_ts` stays `i64`, matching the outer record's own wire type,
/// deliberately -- a corrupt/negative value must stay representable as
/// negative for the caller's outlier check (D20) to catch it. Casting to
/// `u64` here would wrap a negative value into a huge positive one and
/// silently defeat that check.
pub struct ReplayEvent<'a> {
    pub event: &'a decoder::DecodedMessage,
    pub seq_no: u64,
    pub exchange_ts: u64,
    pub recorder_ts: i64,
}

/// What a completed `replay` call processed, for the caller's own
/// summary reporting.
pub struct ReplayStats {
    pub outer_records: u64,
    pub events: u64,
    pub elapsed: Duration,
}

/// Streams `capture_paths` record-by-record, decoding every message and
/// invoking `on_event` for each one -- the entire outer `[len][ts]
/// [payload]` / inner `[body_len][template_id][seq]` framing and
/// `decoder::decode_message` dispatch lives here, so a caller (the
/// backtest orchestrator) only ever sees already-decoded events, never
/// raw bytes. `max_outer_records == 0` means no limit -- stream the
/// whole thing, start to end.
///
/// One path is the common case. Two-plus paths are k-way merged on each
/// outer record's own `exchange_ts` (tie-broken on the path's index) --
/// for a strategy whose instruments live on different MCX stream files
/// the same trading day. With one path this is byte-identical to the
/// pre-merge single-source loop.
pub fn replay(capture_paths: &[String], max_outer_records: u64, mut on_event: impl FnMut(ReplayEvent)) -> io::Result<ReplayStats> {
    let mut source = MergeSource::open(capture_paths)?;
    let started = Instant::now();
    let mut events = 0u64;
    let mut outer_records = 0u64;
    let mut payload = Vec::new();

    loop {
        let (recorder_ts, base_exchange_ts) = match source.next(&mut payload) {
            Ok(Some(pair)) => pair,
            Ok(None) => break,
            // A read error partway through a real multi-GB file is
            // treated as a soft stop, not a hard failure: whatever
            // `on_event` already did (mutating the caller's `cache`/
            // `engine`) is real and worth reporting on, same as this
            // project's original inline replay loop did (print, then
            // fall through to the summary/report code with whatever
            // was accumulated so far).
            Err(e) => {
                eprintln!("read error after {outer_records} records: {e}");
                break;
            }
        };
        outer_records += 1;
        if outer_records % 5_000_000 == 0 {
            eprintln!("  ... {outer_records} outer records, {events} messages, {:.1}s elapsed", started.elapsed().as_secs_f64());
        }

        // Reset per record to the merge's authoritative base (the record's
        // own first `PacketHeader`, or the carried-forward per-stream
        // value for a headerless one). A mid-payload `PacketHeader` can
        // still bump this *up* below, exactly as the pre-merge code did;
        // it just never carries across records here -- the merge owns
        // that, so a headerless record can't inherit a different stream's
        // timestamp and break the monotonic ordering.
        let mut exchange_ts = base_exchange_ts;

        let mut off = 0usize;
        while off + 8 <= payload.len() {
            let body_len = u16::from_le_bytes([payload[off], payload[off + 1]]) as usize;
            let template_id = u16::from_le_bytes([payload[off + 2], payload[off + 3]]);
            let seq = u32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap());
            if body_len == 0 || off + body_len > payload.len() {
                break;
            }
            let decoded = decoder::decode_message(template_id, seq, &payload[off..off + body_len]);
            off += body_len;

            if let decoder::DecodedMessage::PacketHeader(hdr) = &decoded {
                exchange_ts = hdr.transact_time;
            }
            events += 1;

            on_event(ReplayEvent { event: &decoded, seq_no: events, exchange_ts, recorder_ts });
        }

        if max_outer_records != 0 && outer_records >= max_outer_records {
            break;
        }
    }

    Ok(ReplayStats { outer_records, events, elapsed: started.elapsed() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Writes a minimal-but-real capture file: each outer record is
    /// `[len:u64 LE][recorder_ts:i64 LE][32-byte payload]`, the payload a
    /// single `PacketHeader` (template 13003) with `transact_time` at
    /// body offset 24 -- exactly the bytes `peek_exchange_ts` /
    /// `decoder::decode_message`'s `13003` arm read.
    fn write_capture(path: &std::path::Path, records: &[(i64, u64)]) {
        let mut f = File::create(path).unwrap();
        for &(recorder_ts, exchange_ts) in records {
            let mut msg = [0u8; 32];
            msg[0..2].copy_from_slice(&32u16.to_le_bytes()); // body_len
            msg[2..4].copy_from_slice(&13003u16.to_le_bytes()); // template_id
            msg[4..8].copy_from_slice(&7u32.to_le_bytes()); // seq (arbitrary)
            msg[24..32].copy_from_slice(&exchange_ts.to_le_bytes()); // transact_time
            let length = (8 + msg.len()) as u64;
            f.write_all(&length.to_le_bytes()).unwrap();
            f.write_all(&recorder_ts.to_le_bytes()).unwrap();
            f.write_all(&msg).unwrap();
        }
        f.flush().unwrap();
    }

    fn drain(paths: &[String]) -> Vec<(i64, u64)> {
        let mut src = MergeSource::open(paths).unwrap();
        let mut buf = Vec::new();
        let mut out = Vec::new();
        while let Some(pair) = src.next(&mut buf).unwrap() {
            out.push(pair);
        }
        out
    }

    #[test]
    fn one_source_is_a_plain_passthrough_in_file_order() {
        let dir = std::env::temp_dir();
        let p = dir.join("qtrade_merge_test_single.bin");
        // exchange_ts deliberately increasing, recorder_ts = exchange_ts + a latency
        write_capture(&p, &[(105, 100), (112, 108), (150, 145)]);
        let got = drain(&[p.to_str().unwrap().to_string()]);
        assert_eq!(got, vec![(105, 100), (112, 108), (150, 145)]);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn two_sources_merge_in_exchange_ts_order() {
        let dir = std::env::temp_dir();
        let a = dir.join("qtrade_merge_test_a.bin");
        let b = dir.join("qtrade_merge_test_b.bin");
        // Each stream is internally monotonic on exchange_ts; interleaved
        // they must come out globally monotonic on exchange_ts.
        write_capture(&a, &[(101, 100), (121, 120), (141, 140)]);
        write_capture(&b, &[(111, 110), (131, 130), (151, 150)]);
        let got = drain(&[a.to_str().unwrap().to_string(), b.to_str().unwrap().to_string()]);
        let exchange_order: Vec<u64> = got.iter().map(|(_, e)| *e).collect();
        assert_eq!(exchange_order, vec![100, 110, 120, 130, 140, 150]);
        // and the recorder_ts came along with its own record, not reordered
        assert_eq!(got, vec![(101, 100), (111, 110), (121, 120), (131, 130), (141, 140), (151, 150)]);
        let _ = std::fs::remove_file(&a);
        let _ = std::fs::remove_file(&b);
    }

    #[test]
    fn exact_exchange_ts_tie_breaks_on_path_index_deterministically() {
        let dir = std::env::temp_dir();
        let a = dir.join("qtrade_merge_test_tie_a.bin");
        let b = dir.join("qtrade_merge_test_tie_b.bin");
        write_capture(&a, &[(200, 100), (210, 100)]); // two records, same exchange_ts
        write_capture(&b, &[(205, 100)]); // one record, same exchange_ts
        // path[0] (a) wins every tie, so a's two records come before b's one.
        let got = drain(&[a.to_str().unwrap().to_string(), b.to_str().unwrap().to_string()]);
        assert_eq!(got, vec![(200, 100), (210, 100), (205, 100)]);
        // reversed order -> b wins the tie -> b's record first
        let got_rev = drain(&[b.to_str().unwrap().to_string(), a.to_str().unwrap().to_string()]);
        assert_eq!(got_rev, vec![(205, 100), (200, 100), (210, 100)]);
        let _ = std::fs::remove_file(&a);
        let _ = std::fs::remove_file(&b);
    }

    #[test]
    fn one_source_ending_early_does_not_stall_the_other() {
        let dir = std::env::temp_dir();
        let a = dir.join("qtrade_merge_test_short_a.bin");
        let b = dir.join("qtrade_merge_test_short_b.bin");
        write_capture(&a, &[(101, 100)]); // ends after one
        write_capture(&b, &[(111, 110), (121, 120), (131, 130)]);
        let got = drain(&[a.to_str().unwrap().to_string(), b.to_str().unwrap().to_string()]);
        assert_eq!(got, vec![(101, 100), (111, 110), (121, 120), (131, 130)]);
        let _ = std::fs::remove_file(&a);
        let _ = std::fs::remove_file(&b);
    }

    #[test]
    fn merged_replay_emits_events_in_nondecreasing_exchange_ts() {
        let dir = std::env::temp_dir();
        let a = dir.join("qtrade_merge_test_replay_a.bin");
        let b = dir.join("qtrade_merge_test_replay_b.bin");
        write_capture(&a, &[(101, 100), (161, 160), (181, 180)]);
        write_capture(&b, &[(131, 130), (171, 170), (191, 190)]);
        let mut seen: Vec<u64> = Vec::new();
        replay(&[a.to_str().unwrap().to_string(), b.to_str().unwrap().to_string()], 0, |ev| {
            seen.push(ev.exchange_ts);
        })
        .unwrap();
        assert_eq!(seen, vec![100, 130, 160, 170, 180, 190], "one PacketHeader per record, merged on exchange_ts");
        assert!(seen.windows(2).all(|w| w[0] <= w[1]), "the invariant main.rs's lookahead-drain depends on");
        let _ = std::fs::remove_file(&a);
        let _ = std::fs::remove_file(&b);
    }
}

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

/// Streams `capture_path` record-by-record, decoding every message and
/// invoking `on_event` for each one -- the entire outer `[len][ts]
/// [payload]` / inner `[body_len][template_id][seq]` framing and
/// `decoder::decode_message` dispatch lives here, so a caller (the
/// backtest orchestrator) only ever sees already-decoded events, never
/// raw bytes. `max_outer_records == 0` means no limit -- stream the
/// whole file, start to end.
pub fn replay(capture_path: &str, max_outer_records: u64, mut on_event: impl FnMut(ReplayEvent)) -> io::Result<ReplayStats> {
    let mut source = RecordSource::open(capture_path)?;
    let started = Instant::now();
    let mut events = 0u64;
    let mut outer_records = 0u64;
    let mut exchange_ts: u64 = 0;
    let mut payload = Vec::new();

    loop {
        let recorder_ts = match source.next_record(&mut payload) {
            Ok(Some(ts)) => ts,
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

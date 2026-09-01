# Feed replay — component documentation

**What this component does, in one sentence:** everything about turning a real recorded MCX capture file into decoded events that a live feed source would never need — day-specific token resolution and pre-scanning a day's own recorded price bands ahead of the replay.

Code: [`feed_replay.rs`](feed_replay.rs) (this folder). Included by `main.rs` (`[[bin]] qtrade` — see `../main_user_doc.md`), which is the orchestrator that wires this module's output into `cache`/`execution`. Own harness: `feed-replay-validate` (`validate.rs`, this folder).

---

## 1. Why this is its own component

Split out of `dummy_strategy.rs` on 2026-08-24. That file used to do CLI/location config, feed mechanics, generic instrumentation, and trade decisions all in one place. One piece of the feed mechanics — pre-scanning a whole day's recorded snapshot file for circuit limits before the main replay starts (§3) — is *structurally* backtest-only: there is no live equivalent, because live just listens for the one real broadcast in real time. A strategy calling that logic directly would be calling something meaningless outside backtest, which breaks the project's "same code goes live" goal. So this module holds every piece of "how do I make sense of a recorded file" — even the pieces that happen to also be meaningful in live (name → token resolution) — deliberately apart from both the strategy and the orchestrator, and declares nothing about `cache`/`book`/`execution` (same independence `simulator.rs` already established for its own reasons).

## 2. What it exposes

- `contract_dir_for(capture_path) -> Option<String>` — parses `DD_MM_YYYY` out of a real `mcx_feeder_Increment_capture_DD_MM_YYYY_1_N.bin` filename and builds that day's real `CONTRACT/.../MCXScrips.bcp` path. Real MCX tokens are not stable across days (FR-16) — this is why a day is never hardcoded anywhere downstream.
- `load_refdata(capture_path) -> io::Result<refdata::InstrumentMaster>` — `contract_dir_for` + `InstrumentMaster::load_mcx`, with a real error message for either failure mode.
- `resolve_front_month(master, underlying) -> Option<InstrumentId>` — resolves a name (e.g. `"CRUDEOIL"`) to that day's real front-month future token. A strategy declares names, never tokens (see e.g. `limit_order_book_generator.rs`'s `UNDERLYINGS`).
- `snapshot_path_for(capture_path) -> Option<String>` — the paired `snapshot_capture` file's path for an `Increment_capture` file, or `None` if it isn't one.
- `scan_snapshot_for_bands(path, tracked_ids) -> io::Result<HashMap<InstrumentId, (lower_raw, upper_raw, count)>>` — see §3.
- `replay(capture_paths: &[String], max_outer_records, on_event)` — streams the capture file(s) record by record, decodes every message, and calls `on_event(ReplayEvent { event, seq_no, exchange_ts, recorder_ts })` for each one. Owns all outer/inner wire framing; a caller never touches raw bytes. One path is the common case; two-plus paths are k-way merged (§2b).

## 2a. Two real clocks now, no synthetic one (dual-clock replay, 2026-08-27)

`ReplayEvent` used to carry a synthetic `now_ns` (`+= 1_000` per message, unrelated to real time) plus the exchange's own `packet_transact_time_ns`. Both fields are gone; in their place, `exchange_ts` and `recorder_ts` are **both real**, straight from the capture file:

- **`exchange_ts`** — `PacketHeader.TransactTime` (template 13003), unchanged source, just renamed for symmetry with `recorder_ts`.
- **`recorder_ts`** — the outer record's own second header field (`[8B length][8B local capture timestamp]`), decoded by `RecordSource::next_record` and handed back for real for the first time. It was always there; it was read into a local buffer and discarded, every record, since this project's first day. `main.rs` is what actually schedules two deliveries of the same event from these two numbers (`SimExchange` at `exchange_ts`, `Cache`/`Strategy` at `recorder_ts`) — see `main_user_doc.md` and `scheduler_user_doc.md`.

**Why this matters, concretely: `recorder_ts - exchange_ts` is real, measured feed latency**, not a modeled distribution — a read-only check against `19_08_2026`'s real data (500,000 packets) showed a clean, always-positive shape (p50 ≈ 2.5ms, p99 ≈ 14.3ms, max ≈ 59.4ms).

### The pitfall this file's own callers must not repeat

**MCX capture files recorded before ~2026-08-20 can contain a negative `recorder_ts - exchange_ts` delta**, and `main.rs`'s own hard-failure check (D20 fail-fast, never clamped) will stop the run the moment it finds one. This is not this module's bug and not a reason to loosen that check — it's a real fact about the recording rig: two physical servers (`192.168.xx.11`/`192.168.xx.7`) capture in parallel, a monitoring script substitutes a row from the other server on certain errors, and **the two servers' clocks were not NTP-synced to the same reference until ~2026-08-20** (one pointed at an AWS time source, the other at India NPL, before that). A substituted row from the "other" clock, right at a boundary, is enough to produce a small (tens to low-hundreds of nanoseconds) negative reading — confirmed for real: `19_08_2026`'s own file hit exactly this, `-135ns`, about a fifth of the way through the session.

**Pick a capture day at or after `21_08_2026`** for anything exercising the dual-clock replay against real data. `21_08_2026` itself is verified clean (zero negative deltas across a 60-million-record real scan) and is what `naturalgas_bracket`'s own real run now uses — see `naturalgas_bracket.md`. This isn't a property `feed_replay.rs` can detect or fix on its own (it has no way to know which side of the sync date a given file falls on); it's stated here so the next person picking a day for a real run doesn't have to rediscover it by hitting the same fail-fast.

## 2b. Multi-stream k-way merge (2026-08-31)

One MCX trading day is split across up to 8 `mcx_feeder_Increment_capture_DD_MM_YYYY_1_N.bin` stream files, and a given instrument's data lives on exactly one of them (`21_08_2026`: CRUDEOIL on stream 2, NATURALGAS on stream 4). A strategy watching instruments on different streams needs all of them in one run.

`replay` takes `&[String]` now. With **one** path it's a plain read-ahead over that file — byte-identical `on_event` sequence to the old single-source loop (verified: `multi_instrument_bracket` on `21_08_2026` stream 4 alone reproduces the exact prior result — 101 round trips, 202 fills, `net_pnl = -17,437.20`, `events.log` 2,327 lines). With **two-plus** paths, `MergeSource` k-way merges them:

- **Merge key: each outer record's own starting `exchange_ts`** (the leading `PacketHeader`'s `TransactTime`, or the carried-forward per-stream value for a headerless record). `exchange_ts` is monotonic non-decreasing within one MCX stream, so the merged sequence is monotonic non-decreasing too — which is exactly the invariant `main.rs`'s lookahead-drain already assumes, so **it needs no change for N > 1** (this is why the merge is on `exchange_ts` and not `recorder_ts`: merging on `recorder_ts` would let `exchange_ts` arrive slightly out of order between streams and could drive `SimClock` backwards).
- **Tie-break: the path's index in the config list** (`recording_paths[0]` wins). Pure data, never IO/thread timing → fully reproducible (NFR-01).
- **Bounded memory** unchanged: one buffered record per stream (N is tiny), payload buffers swap with the caller's rather than reallocating.
- **Per-record feed-latency check (D20) still runs per merged record** — a bad delta on *any* stream fails the run, same as a single-stream run.

`main.rs` scans one paired snapshot file per stream (§3) and unions the bands before the replay — so CRUDEOIL's band comes from stream 2's snapshot, NATURALGAS's from stream 4's, automatically.

Verified end to end: `multi_instrument_bracket` over `21_08_2026` streams 2 + 4 merged — 721.7M outer records, 204 round trips (**103 CRUDEOIL + 101 NATURALGAS**, both trading off the one merged feed), 408 fills, `denied=0 rejected=0`. The NATURALGAS side is identical to the standalone stream-4 run (same 101 round trips, same entry/exit prices) — the merge is a clean superset, it adds the other stream without perturbing what each instrument sees.

Unit-tested in `feed_replay.rs`'s own `mod tests`: file-order passthrough for N=1, global `exchange_ts` ordering across two sources, deterministic path-index tie-break (both directions), early-EOF of one source not stalling the other, and `replay` itself emitting non-decreasing `exchange_ts` from a merged pair.

This whole mechanism is backtest-only, same as everything else in this file — live combines its multicast streams by arrival order, not by a timestamp merge over files on disk.

## 3. The price-band pre-scan, and why it exists

MCX broadcasts one `InstrumentInfo` (template 13603) message per instrument at start-of-day, carrying that instrument's upper/lower circuit limit for the day, and again on every intraday revision. Real, confirmed finding: the **increment** capture files (the ones replayed tick-by-tick) start recording *after* that first broadcast, on every real day checked so far (`19_01_2026`, `15_06_2026`) — so playing the increment stream alone means `book.rs` sees real orders before it's ever told an instrument's band, and it correctly panics rather than guess.

The **snapshot** file is a separate, parallel capture that re-broadcasts the full current state of every instrument every cycle, including a fresh `InstrumentInfo` each time. `scan_snapshot_for_bands` streams that whole file once, before the main replay, keeping the widest lower/upper bound seen per tracked instrument (bands can revise intraday — NATURALGAS revised 6 times on `19_01_2026`). Every number it returns is a real broadcast that really happened that day; there is no inference beyond "min of lowers, max of uppers, skip the corrupted end-of-day sentinel" (`i64::MIN`-adjacent fields, the same `plausible_band` check `book.rs` uses).

Real, verified values (checked by `feed-replay-validate`, and cross-checked independently against a from-scratch Python byte parser reading the same real files):

| Instrument | Day | Band (Rs) | InstrumentInfo records |
|---|---|---|---|
| CRUDEOIL (467013) | 19_01_2026 | [5,232.00, 5,666.00] | 8,025 |
| NATURALGAS (465849) | 19_01_2026 | [221.60, 339.20] | 1,271 |
| CRUDEOIL (499095) | 15_06_2026 | [7,347.00, 8,799.00] | 6,584 |

## 4. Running the regression check

```bash
cd qtrade
cargo build --release
./target/release/feed-replay-validate
```
Re-derives all three real bands above from the real snapshot files and fails loudly (non-zero exit) on any mismatch.

## 5. What this component deliberately does not do

- Does not touch `cache`, `book`, `execution`, or any engine construction — it only produces facts (tokens, bands, decoded events); the orchestrator (`main.rs`) is what wires those facts into engines.
- Has no live equivalent for `scan_snapshot_for_bands` — by design; live doesn't need one.
- Does not decide *what* a strategy trades — `UNDERLYINGS` (the list of names) lives in the strategy, not here.

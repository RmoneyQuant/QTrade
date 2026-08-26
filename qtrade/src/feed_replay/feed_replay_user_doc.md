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
- `replay(capture_path, max_outer_records, on_event)` — streams the capture file record by record, decodes every message, and calls `on_event(ReplayEvent { event, seq_no, now_ns, packet_transact_time_ns })` for each one. Owns all outer/inner wire framing; a caller never touches raw bytes.

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

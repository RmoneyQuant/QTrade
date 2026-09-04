# qtrade (main.rs) — component documentation

**What this component does, in one sentence:** the one real entry point — reads a `[run]`/`[deployment]` config file, constructs the shared engines, drives a real capture file through them via `feed_replay`, calls the strategy's decision code once per real wake, and writes the run's reports into a fresh timestamped folder.

Code: [`main.rs`](main.rs) (crate root — this is the one file not in its own folder, hence this doc sitting alongside it rather than in a subfolder). `[[bin]] qtrade`, the crate's only binary target other than each component's own `*-validate` regression harness.

---

## 1. Why this file looks the way it does — the history, briefly

Three real things happened here, in order, and this file is what's left after all three:

1. **`main.rs` used to be decode-only.** While `types`/`refdata`/`decoder`/`book`/`scheduler`/`cache`/`simulator`/`execution` (T00–T07) were built in sequence, `main.rs` was deliberately frozen as a minimal CLI that only called `decoder::decode_file` (raw bytes → per-template-ID counts) — every `*-validate.rs` harness's own header comment says why: a **concurrency-safety convention**, avoiding multiple build passes editing the one shared entry-point file at once. It was never a design decision, and `STATUS.md` flagged folding it back as known, deferred cleanup.
2. **A separate `backtester` binary became the real orchestrator instead**, by accretion — first as `dummy_strategy.rs`'s own `main()`, then split (2026-08-24) into `feed_replay`/`backtester`/`dummy_strategy`. This worked, but left two competing "start here" binaries where `ARCHITECTURE-DECISIONS.md` explicitly wants one — it **retires** the word "backtester" as a system name in favor of **qtrade** (the engine) / **Backtest Mode** (a run mode of that one engine).
3. **The backtester/main.rs merge (2026-08-25)** folds `backtester.rs`'s content into `main.rs`, retires the decode-only mode (`decoder::decode_file`/`decoder::Summary` deleted — `decoder::decode_message`, the real per-message decode logic, is untouched and is exactly what `feed_replay::replay` still calls on every message), and switches the CLI surface from bare positional args to the config file `ARCHITECTURE-DECISIONS.md` D39 and `BACKTEST-PHASE1.md` §2.3 already specified.
4. **Event Dispatcher / Control Dispatcher (2026-08-25, same day, two passes)** replaces the hardcoded `cache.subscribe(...)` loop and the `WakeRecorder`/drain-loop plumbing this file used to own with `event_dispatcher::EventDispatcher` and `control_dispatcher::ControlDispatcher`, both constructed here and driven explicitly per event — D07/D33's own two-dispatcher design, finally real. Subscription itself moves into the strategy's own `on_start` (D33: `Strategy -> subscribe() -> Control Dispatcher -> Data Engine`); `main.rs` still resolves instrument names for its *own* filter/engine construction (an unavoidable ordering constraint — `Cache::new` needs a complete filter up front, per D32), but no longer decides what the strategy subscribes to. A second pass the same day added real fill/order-update delivery: `engine.on_market_event(...)` now returns an `ExecOutcome`, forwarded to `control_dispatcher.dispatch(...)` every event. See `event_dispatcher/event_dispatcher_user_doc.md` and `control_dispatcher/control_dispatcher_user_doc.md`.
5. **Dual-clock replay (2026-08-27)** replaces the entire straight-line `feed_replay::replay(..., |ev| { ... })` closure above with a real `scheduler::Scheduler`-driven loop, and moves `SimExchange` out of `ExecutionEngine` to live here directly. This is the biggest single change this file has had since the backtester merge — see §3 item 6 and §5 for the full account, and `scheduler_user_doc.md`/`feed_replay_user_doc.md`/`execution_user_doc.md` for each component's own side of it. `engine.on_market_event(...)` (item 4, above) no longer exists — replaced by `venue.apply_market_event(...)` called directly from this file's own `dispatch_event` function, per the new architecture.

## 2. How to run it

```bash
cd qtrade
cargo build --release --bin qtrade
./target/release/qtrade <config-file>
```

One positional argument: the config file. See `config/config_user_doc.md` for the full schema. Minimal real example:

```toml
[run]
mode            = "backtest"
session_id      = 1
recording_path  = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin"
report_dir      = "logs/qtrade"

[deployment]
```

`mode` lives **in the file**, not a CLI flag or subcommand — the same "does the CLI ask backtest or live" question from earlier in this project's life, now answered by a config field per D22/D39. Only `"backtest"` is implemented; anything else fails cleanly.

Real measured cost, two real days (pre-dual-clock numbers — decode/replay-only cost, still representative of `feed_replay::replay`'s own per-message overhead; unaffected by the Scheduler work below since that's pure interleaving logic, not decode cost):

| Day / stream | Outer records | Messages | Elapsed |
|---|---|---|---|
| `19_01_2026`, stream 4 (CRUDEOIL, 6.8GB) | 56,602,508 | 114,423,913 | ~37s |
| `15_06_2026`, stream 2 (CRUDEOIL, 58.9GB) | 465,071,910 | 931,105,905 | ~290s |

A real dual-clock run against `21_08_2026` (NATURALGAS, `naturalgas_bracket` strategy, ~60M outer records) completed end-to-end with 44 order events / 21 fills / 10 real round trips logged — see `strategy/naturalgas_bracket/naturalgas_bracket.md` for the trade-level detail and §5 below for what the dual-clock timing itself looked like on that run.

## 3. What it wires together, in order

1. `feed_replay::load_refdata(capture_path)` — that day's real `MCXScrips.bcp`.
2. `feed_replay::resolve_front_month(&master, name)` for each name in the strategy's own `UNDERLYINGS` (currently `naturalgas_bracket::UNDERLYINGS`) — names to real tokens, never a hardcoded token. This resolution is for `main.rs`'s *own* construction below (filter/engine), independent of whatever the strategy's `on_start` does with the same names.
3. `InstrumentFilter`, `Cache::new`, `ExecutionEngine::new` (with `RunConfig.session_id` now sourced from `cfg.run.session_id`), `RunConfig` — the shared engines, constructed once, regardless of whether the plugged-in strategy ever actually trades.
4. `feed_replay::scan_snapshot_for_bands` + `cache.seed_book_band` per tracked instrument — the real, backtest-only circuit-limit pre-seed.
5. `EventDispatcher::new()` / `ControlDispatcher::new()`, then `event_dispatcher.register(strategy)` (as `Rc<RefCell<dyn strategy::Strategy>>`) and one call to the strategy's own `on_start(&mut start_ctx)` — this is where subscription actually happens now (`ctx.resolve`/`ctx.subscribe`, D33), not a hardcoded loop here.
6. **The dual-clock `Scheduler` loop (2026-08-27, replaces the old closure entirely).** `feed_replay::replay(capture_path, cfg.run.max_outer_records, |ev| { ... })` is still what streams and decodes the capture file, but its callback no longer calls `engine`/`cache` directly — it validates `ev`'s own `exchange_ts`/`recorder_ts` pair against the Q1 outlier policy (negative delta, or delta beyond `cfg.run.max_feed_delta_ns`, is a hard failure — `std::process::exit(1)` with a `FATAL:` message, never clamped), then schedules **two** `scheduler::Event`s on a `Scheduler` this file owns: `EventPayload::MarketData{target: Target::SimExchange, ...}` at `exchange_ts`, and `EventPayload::MarketData{target: Target::Cache, ...}` at `recorder_ts`. Before scheduling those two, it first performs a **lookahead-drain**: pops and dispatches everything already on the Scheduler whose timestamp is `< exchange_ts` of the message just read (safe because `exchange_ts` is monotonic non-decreasing for one venue's stream — see `scheduler_user_doc.md` §4). Each popped event is handled by a free function, `dispatch_event` (defined in this file, not a `Scheduler` method — keeps D07's "routing knowledge lives in startup wiring" intact and keeps `scheduler.rs` itself free of `cache`/`execution`/`simulator` dependencies): `MarketData{target: SimExchange, ...}` calls `venue.apply_market_event(...)`; `MarketData{target: Cache, ...}` calls `cache.apply`/`event_dispatcher.on_book_touched`/`on_trade` (the old body, essentially unchanged, just fed by `recorder_ts` instead of the retired synthetic clock); `OrderArrival{op_id}` looks up `pending_ops` (a `HashMap<u64, PendingVenueOp>`, this file's own state) and calls `engine.deliver_order`/`deliver_cancel_to_venue`/`deliver_modify_to_venue` against `&mut sim_venue`; `ReportDelivery{op_id}` looks up `pending_reports` and finally calls `control_dispatcher.dispatch(...)` — the only place that call happens now. Any `ExecOutcome` produced by an `OrderArrival` delivery or by `venue.apply_market_event` is stashed in `pending_reports` and scheduled as a `ReportDelivery` at `now + latency_ns` via the shared `schedule_report_if_needed` helper, rather than dispatched immediately — this is D36/Q9's "gate check is synchronous, venue response is a scheduled event," now real rather than a zero-latency accident. After end-of-file, a final `while let Some(event) = sched.pop_earliest() { dispatch_event(...) }` drains whatever's left. `SimExchange` itself is now constructed and owned directly by `main()` (`let mut sim_venue = SimExchange::new(...)`), not hidden inside `ExecutionEngine` — see `execution_user_doc.md`.
7. After the loop: `strategy.borrow_mut().on_stop(&mut stop_ctx)` — real, wired (the one of `Strategy`'s ten methods beyond the five Phase A-C built that has anything calling it; see `strategy/README.md`). Building `stop_ctx` now requires constructing a `strategy::RunHandles { venue: &sim_venue, scheduler: &mut sched, pending_ops: &mut pending_ops, latency_ns }` and passing it into `strategy::Ctx::new(...)` — the same handle bundle every `on_book`/`on_trade`/`dispatch` call threads through during the loop, since `on_stop` can still call `ctx.submit()`/`cancel()`/`modify()` and needs the same scheduling capability. Then prints the summary, writes `orders.log`/`fills.log`/`report.txt` into `{cfg.run.report_dir}/<run-timestamp>/` from `engine`'s own already-existing report methods (`engine.tier1_report(&sim_venue)` now takes the venue as a parameter too, for its OTR admission/rejection counts).

**`feed.csv` generation does *not* live here** — it used to (as `FeedLogger`, "generic instrumentation"), and that was wrong: it made `feed.csv` unconditional for *any* strategy plugged into this orchestrator, whether that strategy wanted it or not. Corrected the same day it was noticed. It lives inside whichever strategy actually wants it (`limit_order_book_generator.rs`, not currently compiled in) — a different strategy dropped in here simply doesn't produce one. See `strategy/limit_order_book_generator/limit_order_book_generator.md`.

## 4. What this component deliberately does not do

- Does not decode any bytes itself — `feed_replay::replay` owns all wire framing; this file only ever sees already-decoded `DecodedMessage`s.
- Does not decide *what* a strategy does on a book change or a trade, and does not decide what it subscribes to — `event_dispatcher` calls the currently plugged-in strategy's own `on_book`/`on_trade` methods with a `Ctx` handle; the strategy's own `on_start` decides what it watches. `main.rs` only constructs and drives the dispatchers, never routes on their behalf (D07: "routing knowledge lives in startup wiring, never inside either dispatcher" — and, as of this rewrite, not inside `main.rs`'s own loop body either, beyond calling the two dispatchers).
- Does deliver fills/order-updates, on a real scheduled delay now (2026-08-27): every `ExecOutcome` — whether from a market event hitting the venue or from a delivered order/cancel/modify — is stashed and forwarded to `control_dispatcher.dispatch(...)` only when its scheduled `ReportDelivery` event fires, `latency_ns` after the triggering delivery. This is proven for real against `21_08_2026`: every real `Submitted`→`Filled`/`Rejected` pair in that run's `orders.log` is exactly `latency_ns` (100,000ns in that config) apart, direct evidence the scheduled-delivery mechanism (not a synchronous shortcut) is what actually ran.
- Does not implement live mode — the config file's `mode` field is asked honestly, and fails if it isn't `"backtest"`, rather than pretending.
- Does not support more than one strategy, or expose `CostConfig`/a probabilistic latency model as config fields yet — real future work (D08, D18); `latency_ns`/`max_feed_delta_ns` are flat constants for now (see `config/config_user_doc.md`), not the eventual `LatencyModel` trait's `Fixed`/`Sampled` variants.
- **Does now use `scheduler.rs` for real (2026-08-27)** — this reverses the previous version of this bullet. `main.rs` owns the one real `Scheduler` this run drives, schedules every `MarketData`/`OrderArrival`/`ReportDelivery` event on it, and is the only place that pattern-matches a popped event (via its own `dispatch_event` function) into a call against `cache`/`engine`/`sim_venue`/`control_dispatcher`. See §3 item 6 above, and `scheduler_user_doc.md` for the module's own side of this.
- Does not pick a real capture day for you, and does not detect whether one is safe for the dual-clock model. **Any file recorded before `21_08_2026` may contain a negative `recorder_ts - exchange_ts` delta** (a real, confirmed artifact of two recording servers whose clocks weren't NTP-synced to a common reference until ~2026-08-20 — full account in `feed_replay/feed_replay_user_doc.md` §2a) and will trip the Q1 fail-fast the instant it's hit, mid-run. Pick a day at or after `21_08_2026` for any real dual-clock run; `21_08_2026` itself is verified clean (60M records, zero negative deltas) and is what every real run in this project now uses.

## 5a. Own-order injection (2026-09-03)

`sync_venue_alarms` (§3 item 6) got a companion, `drain_cache_injections`,
called from the exact same two spots right after it: the lookahead-drain
loop and the final drain loop. It drains `ExecutionEngine::take_pending_cache_injections()`
— synthetic `DecodedMessage`s `execution.rs` builds from our own
`ExecReport`s (a resting order, a fill, a cancel) — and schedules each
toward `Target::Cache` at `now`, same as a real message. This is what
makes a strategy's own resting order, and its own fills, visible to
`cache.book(id).queue_position(...)`, not just to `SimExchange`'s own
internal bookkeeping. See `execution_user_doc.md` §12 for the full
account, including a known, documented edge case (a real trade's
fallback-cascade match landing on our own injected slot).

## 5. Real evidence the dual-clock/latency mechanism actually ran (2026-08-27)

From the `21_08_2026` NATURALGAS run's real `orders.log`: every order's `Submitted` line and its matching `Filled`/`Rejected` line are exactly `100,000ns` apart (that run's configured `latency_ns`) — not zero, not some other value. That gap only exists because `Ctx::submit()` schedules a real `OrderArrival` event and the venue's response is only learned about via a later `ReportDelivery` event, both real `Scheduler` entries popped in real timestamp order — a synchronous shortcut could not produce this. The same run also logged real `recorder_ts`-driven wall-clock strings (via `naturalgas_bracket.rs`'s own `fmt_ist` helper) for every `on_start`/`ALARM`/`PLACING ORDER`/`ORDER UPDATE`/`FILL RECEIVED`/`PORTFOLIO updated` line — see `strategy/naturalgas_bracket/naturalgas_bracket.md` for the full trade-by-trade account (10 round trips: 5 TP, 5 SL, `net_pnl=-903.1182`).

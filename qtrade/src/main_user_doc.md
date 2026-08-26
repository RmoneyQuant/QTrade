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

Real measured cost, two real days (unchanged by this merge — byte-identical to the pre-merge numbers):

| Day / stream | Outer records | Messages | Elapsed |
|---|---|---|---|
| `19_01_2026`, stream 4 (CRUDEOIL, 6.8GB) | 56,602,508 | 114,423,913 | ~37s |
| `15_06_2026`, stream 2 (CRUDEOIL, 58.9GB) | 465,071,910 | 931,105,905 | ~290s |

## 3. What it wires together, in order

1. `feed_replay::load_refdata(capture_path)` — that day's real `MCXScrips.bcp`.
2. `feed_replay::resolve_front_month(&master, name)` for each name in the strategy's own `UNDERLYINGS` (currently `naturalgas_bracket::UNDERLYINGS`) — names to real tokens, never a hardcoded token. This resolution is for `main.rs`'s *own* construction below (filter/engine), independent of whatever the strategy's `on_start` does with the same names.
3. `InstrumentFilter`, `Cache::new`, `ExecutionEngine::new` (with `RunConfig.session_id` now sourced from `cfg.run.session_id`), `RunConfig` — the shared engines, constructed once, regardless of whether the plugged-in strategy ever actually trades.
4. `feed_replay::scan_snapshot_for_bands` + `cache.seed_book_band` per tracked instrument — the real, backtest-only circuit-limit pre-seed.
5. `EventDispatcher::new()` / `ControlDispatcher::new()`, then `event_dispatcher.register(strategy)` (as `Rc<RefCell<dyn strategy::Strategy>>`) and one call to the strategy's own `on_start(&mut start_ctx)` — this is where subscription actually happens now (`ctx.resolve`/`ctx.subscribe`, D33), not a hardcoded loop here.
6. `feed_replay::replay(capture_path, cfg.run.max_outer_records, |ev| { ... })` — the actual loop. Per decoded event, in this order (Phase C, Q1 — reordered from the original apply-then-engine order so the venue and the strategy's view of the book always agree): `engine.on_market_event` first, its `ExecOutcome` forwarded to `control_dispatcher.dispatch`; then `cache.apply` and (if it touched a book) `event_dispatcher.on_book_touched`, or (if the event was a real `Trade`) `event_dispatcher.on_trade` unconditionally — either one's own returned `ExecOutcome` forwarded to `control_dispatcher.dispatch` too.
7. After the loop: `strategy.borrow_mut().on_stop(&mut stop_ctx)` — real, wired (the one of `Strategy`'s ten methods beyond the five Phase A-C built that has anything calling it; see `strategy/README.md`). Then prints the summary, writes `orders.log`/`fills.log`/`report.txt` into `{cfg.run.report_dir}/<run-timestamp>/` from `engine`'s own already-existing report methods.

**`feed.csv` generation does *not* live here** — it used to (as `FeedLogger`, "generic instrumentation"), and that was wrong: it made `feed.csv` unconditional for *any* strategy plugged into this orchestrator, whether that strategy wanted it or not. Corrected the same day it was noticed. It lives inside whichever strategy actually wants it (`limit_order_book_generator.rs`, not currently compiled in) — a different strategy dropped in here simply doesn't produce one. See `strategy/limit_order_book_generator/limit_order_book_generator.md`.

## 4. What this component deliberately does not do

- Does not decode any bytes itself — `feed_replay::replay` owns all wire framing; this file only ever sees already-decoded `DecodedMessage`s.
- Does not decide *what* a strategy does on a book change or a trade, and does not decide what it subscribes to — `event_dispatcher` calls the currently plugged-in strategy's own `on_book`/`on_trade` methods with a `Ctx` handle; the strategy's own `on_start` decides what it watches. `main.rs` only constructs and drives the dispatchers, never routes on their behalf (D07: "routing knowledge lives in startup wiring, never inside either dispatcher" — and, as of this rewrite, not inside `main.rs`'s own loop body either, beyond calling the two dispatchers).
- Does deliver fills/order-updates now (2026-08-25, same day as the dispatcher work): `engine.on_market_event(...)`'s `ExecOutcome` return value is forwarded to `control_dispatcher.dispatch(&cache, &outcome)` every event — but the one real strategy submits no orders, so this is proven with synthetic data in `control_dispatcher`'s own tests, not by anything the real `19_08_2026` run actually exercises.
- Does not implement live mode — the config file's `mode` field is asked honestly, and fails if it isn't `"backtest"`, rather than pretending.
- Does not support more than one strategy, or expose `CostConfig`/latency-model parameters as config fields yet — real future work (D08, D18), not part of this pass.
- Does not use `scheduler.rs` — declared in this bin (so its own tests keep running) but not wired into the real order/event path. See the discussion in this project's own history: the `LatencyModel` types exist (`simulator.rs`) but nothing calls them, so today's fills are effectively zero-latency. Deliberately deferred, not forgotten.

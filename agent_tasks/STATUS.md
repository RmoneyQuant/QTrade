# Status tracker

**Single-writer file — updated from what actually got built; task files are read, not edited, to track progress.**

## Phase 1 (BACKTEST-PHASE1.md, M1–M7): all 8 components done and independently verified

All of `types`/`refdata`/`decoder`/`book`/`scheduler`/`cache`/`simulator`/`execution` are built, wired into `main.rs`, and passing. Whole-workspace `cargo test --release` (every `[[bin]]` target: `mcx-decoder`, `book-validate`, `cache-validate`, `simulator-validate`, `execution-validate`): **155 tests, 0 failures**, confirmed directly, not taken from any agent's self-report. Every component's real-data acceptance gate (FR-B03/B09/B11/B18/B24/B27-31 as applicable) was independently re-run against real MCX capture data, not just read from a write-up.

**Known cleanup, not correctness-blocking:** the four extra `[[bin]]` targets (`book-validate`/`cache-validate`/`simulator-validate`/`execution-validate`) exist only because each component was built before it could be wired into `main.rs` (to avoid concurrent agents editing the same file) — each one's own header comment says it's "safe to remove... once wired into `main.rs` and folded into a normal integration test." Now that all 8 are wired in, folding these four into real `#[cfg(test)]`-only integration tests (dropping the redundant `[[bin]]` entries) is reasonable follow-up housekeeping, not required for phase 1's definition of done.

| Component | Milestone | Status | Notes |
|---|---|---|---|
| `types` | — | **Done** | [T00_types.md](T00_types.md) — `cargo build` clean, wired into `main.rs` as `#[allow(dead_code)] mod types;` |
| `refdata` | M1 | **Done, verified** | [T01_refdata.md](T01_refdata.md) — real `19_01_2026` file: 50,081 rows, 140 FUTCOM instruments, independently re-run against the real file (not just the agent's report) with identical numbers. DPR bounds resolved as a percentage circuit band, not absolute rupee range — `price_band` left `None`, documented in `refdata_user_doc.md` |
| `decoder` | M2 | **Done, validated** | Real 20GB/164M-record file, byte-exact accounting. Public iterator API + `pub` on `DecodedMessage` deferred to `book` (T03) itself per [T02_decoder.md](T02_decoder.md)'s own instruction, not a separate task |
| `book` | M3 | **Done, verified** | [T03_book.md](T03_book.md) — the real gate, passed. Full-session FR-B11 run, independently re-run and watched to completion (not just the agent's report): CRUDEOIL 8,024/8,024 cycles, 0 divergences; NATURALGAS 1,270/1,270 cycles, 0 divergences — full order-level depth. Root cause of an earlier 103-divergence NATURALGAS bug: `Trade`'s `event_time` field is actually the matched resting order's own `priority_ts`, not a timestamp — fixed in `apply_trade`, 14 unit tests pass. `decoder.rs` extended with a public iterator API + typed snapshot messages (13600/13601/13602) as part of this task, per T02's note. Stream mapping empirically confirmed: CRUDEOIL=stream 4, NATURALGAS=stream 5 (corrects my own earlier column-based guess) |
| `scheduler` | M4 | **Done, verified** | [T04_scheduler.md](T04_scheduler.md) — priority queue + `SimClock`, `(timestamp, event_class, seq)` ordering key, tie-break independently re-run via `cargo test` (not just the agent's report), determinism + tie-break tests pass, wired into `main.rs` |
| `cache` | M5 | **Done, verified** | [T05_cache.md](T05_cache.md) — full real session, both instruments: 298.9M outer records, 603.2M messages, 155.77s, ~1.92M records/s. Dispatch path confirmed zero-allocation (873,727 wakes, 0 allocs) via a real counting allocator, independently re-run by me on a truncated slice with consistent results. Roll-trap-safe filter resolves 12 instruments but the acceptance run itself was narrowed to the 2 `book` has validated price bands for — disclosed honestly, not hidden. 23/23 tests pass |
| `simulator` | M6 | **Done, verified** | [T06_simulator.md](T06_simulator.md) — highest-risk component, passed. Hand-trace and full FR-B24 invariant sweep (6/6 pass) both independently re-run by me against real CRUDEOIL data, not just the agent's report: invariant #1 (strongest — fills never exceed real traded volume) checked unconditionally across 10,801 real trades, 0 violations. Zero dependency on `cache` confirmed by grep. Own book implementation (`SimBookImpl`, `BTreeMap`-based) deliberately independent of `book`'s, per D10 — but reuses `book`'s hard-won `Trade.event_time`-is-`priority_ts` finding, since it's a real MCX data fact, not a `book`-specific implementation detail. Found and fixed one doc overclaim: invariant #5's real session never actually exercised the "residual rests" branch (all 125 real `MarketToLimit`s filled completely) — corrected to disclose this, covered separately by a synthetic unit test. 18/18 tests pass |
| `execution` | M7 | **Done, verified** | [T07_execution.md](T07_execution.md) — final milestone, phase 1 build complete. Eleven-state machine matches STRATEGY-GUIDE.md §7a; `PendingCancel → Filled` race, `Denied`-vs-`Rejected` distinction, cost-model asymmetry (buy Rs 236.0102 vs sell Rs 236.0502 on the same order), two-level accounting (firm nets sub-accounts: 10 - 4 = 6), and Tier 1 report with run identity all independently re-run by me with matching real output. One real bug found+fixed along the way: `queue_position_at_fill` was reading post-fill state (0) instead of the genuine pre-fill value (10) when a fill spanned two market events — fixed, confirmed via real `cargo test` |

## Post-phase-1: `dummy_strategy` split into `feed_replay` / `backtester` / `dummy_strategy` (2026-08-24)

Not a new spec'd phase — a refactor of already-shipped demo code, so no new T-numbered task files were created. `dummy_strategy.rs` used to be simultaneously the CLI, the backtest-only feed reader (token resolution, circuit-band pre-scan, streaming the capture file), the `feed.csv` writer, and the strategy's trade decision — mixing backtest-only mechanics into a file meant to hold strategy logic, which breaks the "same code goes live" goal (the price-band pre-scan specifically has no live equivalent). Split into three components, `backtester`/`feed_replay`/`dummy_strategy` (see each folder's own `*_user_doc.md`), following this project's own established convention. `dummy_strategy` no longer has a `[[bin]]` target; `backtester` is the new runnable entry point, `feed-replay-validate` is `feed_replay`'s own regression harness. Verified: whole-workspace `cargo test --release` across all 7 `[[bin]]` targets (`mcx-decoder`, `book-validate`, `cache-validate`, `simulator-validate`, `execution-validate`, `feed-replay-validate`, `backtester`) passes with 0 failures; the new `backtester` bin reproduces byte-identical real P&L against `19_01_2026` (`gross_pnl=-1300.0000`) and a real second day, `15_06_2026` (see `backtester_user_doc.md`), to what the pre-split code produced.

## Post-phase-1: `backtester` merged into `main.rs`; package renamed `mcx-decoder` → `qtrade` (2026-08-25)

Also not a new spec'd phase. `main.rs` had been frozen early as a decode-only CLI purely as a concurrency-safety convention while T00–T07 were built in sequence (every `*-validate.rs` harness's own header comment says so); `backtester` became the real orchestrator by accretion instead, leaving two competing "start here" binaries where `ARCHITECTURE-DECISIONS.md` explicitly wants one (it retires "backtester" as a system name in favor of **qtrade**). Discussed and settled with the user: `backtester.rs`'s content moved into `main.rs`; the decode-only mode was retired (`decoder::decode_file`/`decoder::Summary` deleted — confirmed zero real callers beyond that one old CLI path; `decoder::decode_message`, the real per-message decode logic, is untouched); the CLI surface changed from bare positional args to the config file `ARCHITECTURE-DECISIONS.md` D39 and `BACKTEST-PHASE1.md` §2.3 already specify (`qtrade <config-file>`, `[run]`/`[deployment]` sections, mode as a field inside the file rather than a flag). New `config` component (`qtrade/src/config/`) — a hand-rolled parser for this project's own small fixed schema, not a `toml` crate, keeping the project's zero-external-dependency record intact (a real choice, not a network-access constraint — both were checked). Package and its one real binary renamed `mcx-decoder` → `qtrade`. Verified: whole-workspace `cargo test --release` across all 6 remaining `[[bin]]` targets (`qtrade`, `book-validate`, `cache-validate`, `simulator-validate`, `execution-validate`, `feed-replay-validate`) passes with 0 failures (190 tests; `scheduler`'s own 5 tests, previously only compiled via the old decode-only `main.rs`, were nearly lost in the merge and restored by explicitly declaring `mod scheduler` — unused, but tested — in the new `main.rs`); the merged `qtrade` binary reproduces byte-identical real P&L against both `19_01_2026` (`gross_pnl=-1300.0000`) and `15_06_2026` (`gross_pnl=-14200.0000`) to what the pre-merge `backtester` binary produced. See `main_user_doc.md` and `config/config_user_doc.md`.

## Post-phase-1: `dummy_strategy` renamed `limit_order_book_generator`; `feed.csv` moved from `main.rs` into it (2026-08-25)

Also not a new spec'd phase. Real bug found and fixed the same day: `feed.csv` generation (then `FeedLogger`, living in `main.rs` as "generic instrumentation") was unconditional for *any* strategy plugged into the orchestrator, regardless of whether that strategy wanted it — backwards, since it should be a strategy's own choice. Moved into the strategy itself, which is also renamed from `dummy_strategy` to `limit_order_book_generator` — `dummy_strategy` was never meant to be a permanent name (it described the first strategy's very basic test-order behavior, now removed entirely: this strategy submits no orders at all, so `orders.log`/`fills.log`/`report.txt` legitimately come out empty when it runs). Each future strategy plugged into `main.rs` gets its own name and folder, following this precedent, rather than accumulating unrelated behaviors under one generic name. `main.rs`'s own subscription depth now reads `limit_order_book_generator::DEPTH_LEVELS` (the strategy's own declared depth of interest, D25) rather than an independent constant that could drift. Verified: whole-workspace `cargo test --release` still 190 tests, 0 failures; the renamed strategy produces a byte-identical `feed.csv` to the pre-rename run (`diff` clean, 697,270 rows, `19_01_2026`) with orders/fills now correctly empty. See `strategy/limit_order_book_generator/limit_order_book_generator.md`. (2026-08-25, later: nested under a new `strategy/` parent folder, so each future strategy — e.g. a planned `crudeoil_5_percent` — gets its own subfolder there rather than sitting loose under `src/`.)

## Post-phase-1: Event Dispatcher + a minimal Control Dispatcher; `on_start`/`on_book`/`on_trade` real (2026-08-25)

Also not a new spec'd phase. A dedicated design session (10 settled questions, then reconciled against this project's own pre-existing D07/D33) concluded: `cache::Dispatcher`/`Subscriber::on_wake` — real but narrow (FR-B18/D25 only, never told a strategy about a `Trade` message's own fields, since `Cache::apply` discards message identity the instant it's done routing) — should become two real, separate components matching D07/D33's own named design, not one. Built:

- **`event_dispatcher`** (`qtrade/src/event_dispatcher/`) — relocates `cache::Dispatcher`'s keying/snapshot-diffing logic unchanged, generalized to call a real (if still thin) strategy trait, `MarketHandler`: `on_start` (no default), `on_book` (was `on_wake`), and new — `on_trade`, firing unconditionally on every real `Trade` message regardless of book-state change, bypassing the snapshot-diff machinery entirely (a trade is a fact to report once, not a comparison). `Cache` no longer owns or knows about dispatch at all; `main.rs` owns `EventDispatcher` as a sibling and drives both explicitly, per event.
- **`control_dispatcher`** (`qtrade/src/control_dispatcher/`) — new, deliberately minimal this pass: only forwards a strategy's `on_start`-time `subscribe()` call into `event_dispatcher`, per D33's own diagram (`Strategy -> subscribe() -> Control Dispatcher -> Data Engine`). Real fill/order-update delivery (`ControlHandler`, `ExecutionEngine::handle_exec_reports` returning what it produces instead of only accumulating it) is explicit, disclosed follow-up work — not built, since only one strategy exists to exercise `strategy_id`-keyed routing against.
- **`strategy::Ctx`/`StartCtx`** (`qtrade/src/strategy/strategy.rs`) — the shared context-handle types both dispatchers' traits are written in terms of, in a neutral home (not either dispatcher's own module, since `StartCtx` has to reach both). `Ctx` exposes only `book`/`refdata` today — `submit`/`cost`/`position` deliberately absent until their backing machinery (execution access) exists.

Subscription registration moved out of `main.rs` (a hardcoded `cache.subscribe(...)` loop) into `LimitOrderBookGenerator::on_start` — the strategy now declares its own subscriptions via `ctx.resolve`/`ctx.subscribe`, matching D33. `main.rs` still independently resolves the same underlying names for its own filter/engine construction (an unavoidable ordering constraint, D32), a legitimate second use of the same constant, not a contradiction. Verified: whole-workspace `cargo test --release` — 196 tests, 0 failures (6 new: 6 dispatch/wake tests relocated + extended under `event_dispatcher::tests`, 1 new forwarding test under `control_dispatcher::tests`, replacing the 5 that used to live in `cache::tests`); `cache-validate`'s own acceptance binary updated to drive `EventDispatcher` externally rather than through `Cache`. The full `19_08_2026` NATURALGAS run reproduces a byte-identical `feed.csv` (`md5sum` match, 991,128 rows) to the pre-rewrite run — the dispatch mechanism changed, `LimitOrderBookGenerator`'s actual output didn't. See `event_dispatcher/event_dispatcher_user_doc.md`, `control_dispatcher/control_dispatcher_user_doc.md`, `cache/cache_user_doc.md` §4 (historical), `strategy/README.md`.

## Post-phase-1: Phase B — `ControlHandler`, `ExecutionEngine` returning what it produced (2026-08-25, same day as the dispatcher work)

Also not a new spec'd phase. The other half of D07/D33's design, deferred that same morning as "Phase B" in `control_dispatcher_user_doc.md`/`STATUS.md`: delivering fills and order-state changes to a strategy live, not only via `ExecutionEngine::fills()`/`order_events()` after a run ends. Two real design forks were discussed and settled explicitly before building:

- **Where the "what's new" comparison lives.** Confirmed choice: inside `ExecutionEngine` itself, not computed externally by `main.rs`. Each of the seven mutating methods that can produce a fill or order-event (`submit_order`, `on_market_event`, `request_cancel`, `deliver_cancel_to_venue`, `request_modify`, `deliver_modify_to_venue`, `mark_expired`) had its body renamed to a private `..._inner` (unchanged logic) and gained a thin public wrapper returning a new `ExecOutcome { fills, order_events }` alongside its original result — a before/after length snapshot around the untouched original body. Accepted cost: ~50 existing call sites (execution.rs's own ~35, execution-validate's ~14, 1 real one in main.rs) needed mechanical signature updates (e.g. `eng.submit_order(...)` → `eng.submit_order(...).0` where only the original result is used); no test assertions changed.
- **`ControlHandler`'s payload types.** Reuses `execution::FillRecord`/`OrderEventRecord` directly rather than inventing parallel `Fill`/`OrderUpdate` structs to match `STRATEGY-GUIDE.md`'s illustrative naming — those records are already real, tested, and richer than the guide's example.

Built: `control_dispatcher::ControlHandler` (`on_fill`/`on_order_update`, mirroring `event_dispatcher::MarketHandler`, both default-empty), `ControlDispatcher::register`/`dispatch` (still "one destination," same YAGNI reasoning as Phase A's event-dispatcher decisions — no `strategy_id`-keyed routing yet, nothing to exercise it against), `LimitOrderBookGenerator`'s trivial all-default `impl ControlHandler`, and `main.rs`'s replay loop forwarding `engine.on_market_event(...)`'s `ExecOutcome` to `control_dispatcher.dispatch(...)` every event. Two gaps disclosed, not fixed: live delivery is coupled to `tier2_enabled`; `OrderEventRecord` has no structured `CancelReason` field the way `Order.cancel_reason` does. Verified: whole-workspace `cargo test --release` — 235 tests, 0 failures (one new test, `a_real_fill_reaches_on_fill_through_control_dispatcher`, builds a real `ExecutionEngine`, submits a real order, fills it with a real trade, and confirms `on_fill` fires through `ControlDispatcher::dispatch` — proof by synthetic data, since the one real strategy submits no orders and the real `19_08_2026` run's own `ExecOutcome` is always empty); that same real run's `feed.csv` remains byte-identical to the established baseline, `orders.log`/`fills.log` still legitimately empty. See `control_dispatcher/control_dispatcher_user_doc.md` §4, `execution/execution_user_doc.md` §6.1, `strategy/README.md`.

## Post-phase-1: Phase C — `Ctx` can submit/cancel/modify orders and query order/position/PnL/cost (2026-08-25, same day as Phases A/B)

Also not a new spec'd phase. Requested directly ahead of writing a real trading strategy: until now a strategy could see everything but cause nothing — `Ctx` only wrapped `&Cache`. `ExecutionEngine` (gates, order state machine, cost model, two-level accounting, `SimExchange` matching) was already fully built and tested; what was missing was `Ctx`'s own doorway to it. Four real design forks were discussed and settled explicitly before building:

- **Loop order.** `main.rs`'s replay loop now applies `engine.on_market_event` *before* `cache.apply`/dispatching `on_book`/`on_trade`, for every event — previously the other way around. Otherwise a strategy submitting from `on_book` would be acting against a one-event-stale venue book (`Cache` already reflecting the event, `SimExchange` not yet).
- **Return shape.** `ctx.submit()`/`cancel()`/`modify()` return only an acknowledgment (a client order id, or nothing) — never fill data directly, even for a fill that happens synchronously inside the call. Every fill/state-change reaches a strategy exactly one way: through `on_fill`/`on_order_update`.
- **Scope.** Writes are supported only from `on_book`/`on_trade` this pass. Supporting them from `on_fill`/`on_order_update` would need recursive delivery (a reaction's own fill needing to fire *this same event*, with real risk of never terminating) — deferred until a real strategy actually needs it. Reads (`ctx.order`/`position`/`pnl`/`cost`) work from every callback, since they carry none of that risk.
- **Failure mode.** Calling a write method from `on_fill`/`on_order_update` (nothing at the type level stops it, since `Ctx` is one shared type) returns `Err(CtxError::SubmitNotAllowedHere)` — fail loudly, not silent data loss.

Built: `strategy::Ctx` gained `now()`/`order()`/`position()`/`pnl()`/`cost()` (all thin forwards to already-real `ExecutionEngine`/`Portfolio`/`CostModel` accessors) and `submit()`/`cancel()`/`modify()` (gated by a `can_submit` flag set per-callback, accumulating whatever they produce into a `pending: ExecOutcome` drained by whichever dispatcher constructed the `Ctx`). `event_dispatcher::EventDispatcher::on_book_touched`/`on_trade` and `control_dispatcher::ControlDispatcher::dispatch` all gained an `&mut ExecutionEngine` parameter to build that `Ctx`; the first two now return the `ExecOutcome` a strategy's own writes produced, which `main.rs` forwards straight to `control_dispatcher.dispatch`. `MarketHandler`/`ControlHandler`'s own signatures — what a strategy actually implements — did not change at all. `LimitOrderBookGenerator` needed zero changes, since it never touches any of this. Verified: whole-workspace `cargo test --release` — 241 tests, 0 failures (3 new: `ctx.submit()` from `on_book` producing a real, returned `ExecOutcome`; the full loop — submit from `on_book`, a real trade fills it, delivered via `on_fill` — proven with a synthetic `MarketHandler`+`ControlHandler`, since `LimitOrderBookGenerator` never generates one; `ctx.submit()` from `on_fill` failing loudly as designed); the real `19_08_2026` NATURALGAS run's `feed.csv` remains byte-identical to the established baseline, `orders.log`/`fills.log` still legitimately empty. See `strategy/strategy.rs`'s own header, `event_dispatcher_user_doc.md`, `control_dispatcher_user_doc.md` §4, `strategy/README.md`.

## Post-phase-1: `naturalgas_bracket` — the first real, order-placing strategy (2026-08-25/26)

Also not a new spec'd phase. First real use of the `Ctx`/`MarketHandler`/`ControlHandler` machinery built in Phases A-C — a strategy that actually trades. Compiled into `main.rs` in place of `limit_order_book_generator` (source-edit swap, per the established convention — that file is untouched, still selectable). Real design work done before coding: verified `PacketHeader.TransactTime` is a genuine Unix-epoch nanosecond timestamp against the real `19_08_2026` capture file (a real sample decodes to a plausible `09:00:00 IST` session-open time); derived and checked a real session-end time (~23:30 IST) from the tail of the same file; tightened the originally-requested 95%/105% stop-loss/take-profit bracket to 98%/102% after checking it against the day's real, scanned circuit band (`[Rs 254.30, Rs 275.30]`) — the original band would have sat outside it.

Real run against `19_08_2026`: one completed round trip (buy Rs 266.50, take-profit sell Rs 271.90 at 17:25 IST), realized net **Rs 6,631.42** after costs. Re-entered at Rs 271.50 per the 5-minute re-entry rule; the end-of-day force-close was genuinely **rejected** (`NoLiquidityForResidual` — the book had no resting orders left at all by 23:30 IST), leaving that second lot open, unmarked, and excluded from the reported P&L — disclosed, not silently absorbed, since this strategy has no retry/`on_order_update` handling for a rejection (a real, named limitation, not a bug). See `strategy/naturalgas_bracket/naturalgas_bracket.md` for the full account.

## Post-phase-1: `strategy::Strategy` — one name for both trait halves (2026-08-26)

Also not a new spec'd phase. `STRATEGY-GUIDE.md` §2 names one `Strategy` trait; this codebase had split its five real callbacks across two (`MarketHandler`/`ControlHandler`) — a deliberate call from Phase A's own design session (D33's "these were never going to be one thing," extended from the two dispatchers to the trait each one calls), but it meant no type named `Strategy` existed anywhere, which the user flagged directly against the guide. Resolved with a pure-vocabulary supertrait added to `strategy.rs`: `pub trait Strategy: MarketHandler + ControlHandler {}` plus a blanket `impl<T: MarketHandler + ControlHandler> Strategy for T {}` — implement both halves and `Strategy` comes free, with no change to which dispatcher calls which methods. A new test (`implementing_both_halves_satisfies_strategy_for_free`) proves the blanket impl compiles, using a local dummy type rather than naming `LimitOrderBookGenerator`/`NaturalGasBracket` directly, since `strategy.rs` is shared across `[[bin]]` targets that don't all declare every real strategy's module. Verified: whole-workspace `cargo test --release` — 243 tests, 0 failures.

## Post-phase-1: `strategy::Strategy` merged into one real trait; `on_stop` wired (2026-08-26)

Also not a new spec'd phase. The previous entry's `MarketHandler`/`ControlHandler` split was directly challenged: `STRATEGY-GUIDE.md` §2 defines one `Strategy` trait, and D33's real argument (different lookup, different cardinality, different delivery guarantee) is about the two *dispatcher components*, not about the callback *interface* — extending it to split the trait too was a stylistic overreach, not something the dispatcher split actually required. Reverted the same session: `MarketHandler`/`ControlHandler` deleted outright; their 5 real methods (`on_start`/`on_book`/`on_trade`/`on_fill`/`on_order_update`) moved onto one `pub trait Strategy` in `strategy.rs`, alongside the 5 the guide names but nothing here backs (`on_warmup_complete`/`on_timer`/`on_session_change`/`on_book_state_change`/`on_stop`) — all ten declared now, only `on_start` without a default, each of the five unbacked ones documented honestly as such. `on_stop` turned out to be cheap to make real rather than leaving it a stub: `main.rs` already has the exact moment "shutting down" happens (right after the replay loop ends), so it now genuinely calls it.

`EventDispatcher`/`ControlDispatcher` themselves did not change — same registries, same keying, same `ExecOutcome` accumulation, same reason they're two components; only the type each one's registry holds changed, from two separate trait objects to `Rc<RefCell<dyn Strategy>>` each. `LimitOrderBookGenerator`/`NaturalGasBracket` each collapsed their two `impl` blocks into one `impl Strategy for X`. `LimitOrderBookGenerator` isn't currently compiled into `main.rs` (swapped out for `naturalgas_bracket` the previous session) — its own `Strategy` impl was verified for real anyway, by temporarily swapping `main.rs` back to it, confirming a clean build, then reverting, rather than leaving unverified source sitting in the tree. Verified: whole-workspace `cargo test --release` — 243 tests, 0 failures; the real `19_08_2026` NATURALGAS run reproduces byte-identical results to the pre-merge run (same one round trip, same `net_pnl=6631.4223`, same EOD rejection) — the refactor is structural only, no behavior changed. See `strategy.rs`'s own header, `strategy/README.md`, `event_dispatcher_user_doc.md` §5, `control_dispatcher_user_doc.md` §2-3.

## Post-phase-1: Dual-clock replay — `scheduler::Scheduler` wired in for real, real feed latency, tightened bracket + detailed logging (2026-08-27)

Also not a new spec'd phase — the largest single architectural change since the backtester/main.rs merge. User-originated insight: every real MCX capture record already carries **two** genuine timestamps — the exchange's own `PacketHeader.TransactTime` (already decoded) and the recording server's own receipt time (the outer record's second header field, decoded into a local buffer and discarded every record since this project's first day). Settled via a full `/grill-with-docs` session (14 numbered decisions) into: `SimExchange` builds its book on the exchange's own clock (`exchange_ts`); `Cache`/`EventDispatcher`/`Strategy` advance on the recorder's clock (`recorder_ts`) — the honest, always-lagging view a live strategy would actually have. The gap between the two, for any real message, is genuine measured feed latency, not a modeled distribution. This is also what finally gave `scheduler::Scheduler` (built at M4, never wired into `main.rs`'s own loop until now) a real caller — order/report latency (previously zero, "a money printer") is a real scheduled delay for the first time.

Key decisions: a negative `recorder_ts - exchange_ts` delta, or one past a configurable `max_feed_delta_ns` ceiling (default 250ms), is a hard run failure (D20 fail-fast) — **never clamped**, even under real-data pressure (see below). `ctx.submit()`/`cancel()`/`modify()` are now genuinely two-phase: a local gate check (synchronous, as before) plus a venue-reaching delivery scheduled `latency_ns` later as a real `OrderArrival` event; the venue's own response (fill/reject/resting) only reaches a strategy's `on_fill`/`on_order_update` via a further scheduled `ReportDelivery` event, not in the same call. `SimExchange` moved out of `ExecutionEngine` to be owned directly by `main.rs`, passed in as `&SimExchange`/`&mut SimExchange` per call — the same borrowed-through pattern `ControlDispatcher::subscribe` already used for `EventDispatcher`. Full implementation touched `config.rs` (`latency_ns`/`max_feed_delta_ns`), `feed_replay.rs` (both real timestamps, synthetic clock deleted), `scheduler.rs` (`Target`, real `MarketData`/`OrderArrival`/`ReportDelivery` payloads, `peek_earliest_timestamp`), `execution.rs` (`submit_order_local`/`deliver_order` split, `venue` parameterized everywhere), `strategy.rs` (`RunHandles`/`PendingVenueOp`), `main.rs` (the new `dispatch_event`/lookahead-drain loop — see `main_user_doc.md` §3 item 6 for the full shape), `event_dispatcher.rs`/`control_dispatcher.rs` (`RunHandles` threaded through). Plan: `/home/vaibhav/.claude/plans/crispy-imagining-sutherland.md`.

**A genuine, real finding during verification — not a bug, the fail-fast working as designed.** The first real re-run, against `19_08_2026`, hit `FATAL: implausible feed-latency delta at seq=47056580: ... delta=-135ns` partway through the session. Root cause (from the user, who runs the recording rig): two physical servers (`192.168.xx.11`/`192.168.xx.7`) capture in parallel, with a monitoring script substituting a row from the other server on certain errors — and **the two servers' clocks were not NTP-synced to a common reference until ~2026-08-20** (one pointed at AWS time, the other at India NPL, before that). `21_08_2026` was checked and found clean (zero negative deltas across a real 60-million-record scan) and is what every real run in this project uses from this point forward — `19_08_2026`, and by the same reasoning anything before `20_08_2026`, is disclosed as unsafe input for the dual-clock model, documented prominently in `feed_replay/feed_replay_user_doc.md` §2a specifically so this isn't rediscovered by hitting the same fail-fast again.

**Real verification against `21_08_2026`**: the full dual-clock loop completes end-to-end; every real `Submitted`→`Filled`/`Rejected` pair in `orders.log` is exactly `100,000ns` (the configured `latency_ns`) apart — direct proof the scheduled-delivery mechanism, not a same-instant shortcut, is what ran. `naturalgas_bracket`'s own SL/TP bracket was tightened again the same day, from ±2% (98/102, percent) to ±0.5% (`SL_PER_MILLE`=995/`TP_PER_MILLE`=1005) — the real `21_08_2026` session showed **zero** triggers all day at ±2%. The tightened strategy also gained comprehensive, timestamped logging (`on_start` with subscribed instrument name + real MCX token id, `"ALARM: ..."` on every threshold crossing, `"PLACING ORDER: ..."`, `"ORDER UPDATE: ..."`, `"FILL RECEIVED: ..."`, `"PORTFOLIO updated: ..."`) — every line tagged with `ctx.now()`, a genuine physical/simulated timestamp (recorder time for market events, scheduled delivery time for fills/order-updates), never wall-clock-at-log-time. Real result: **10 completed round trips** (5 TP, 5 SL, all entries/exits Rs 263.50–267.50), `net_pnl=-903.1182`, `gross_pnl≈-0.0000` (a symmetric ±0.5% band nearly cancels in price terms — the real cost of 22 orders is what actually shows up), 1 EOD force-close still rejected (`NoLiquidityForResidual`, same disclosed limitation as before). See `strategy/naturalgas_bracket/naturalgas_bracket.md` for the full trade-by-trade table.

**File cleanup, same day**: all pre-dual-clock diagram files (`architecture-overview.svg`, `mtbt-architecture.html`, `qtrade-current-architecture.html`, `qtrade-dispatch-map.html`, `qtrade-wiring-diagram.svg`, `qtrade-file-map.svg`, `order-state-machine.svg`) moved into a new `old_design/` folder at the repo root, superseded by 3 new SVGs reflecting the current dual-clock architecture (in progress).

Verified: `cargo build`/`cargo test --release` clean across the whole workspace (248 tests passing) after every mechanical signature update this refactor required; `execution-validate`'s 5 real acceptance scenarios still pass. Docs updated same day across `scheduler_user_doc.md`, `feed_replay_user_doc.md`, `main_user_doc.md`, `execution_user_doc.md`, `strategy/README.md`, `naturalgas_bracket.md`, `control_dispatcher_user_doc.md`, `event_dispatcher_user_doc.md`, `config_user_doc.md`, and this entry.

## Post-phase-1: multi-stream k-way merge in `feed_replay` (2026-08-31)

Not a new spec'd phase — closes a disclosed limitation. One MCX trading day is split across up to 8 `Increment_capture` stream files, and any one instrument's data lives on exactly one stream (`21_08_2026`: CRUDEOIL on stream 2, NATURALGAS on stream 4). A strategy watching instruments on different streams (`multi_instrument_bracket`'s `["NATURALGAS", "CRUDEOIL"]`) could resolve and subscribe to both but only ever received one stream's data — CRUDEOIL saw nothing all session. Architecturally this was always the plan (D05's "Sequencer", merge on `(capture_ts, source_id, seq)`), just never built because phase-1 validation used single instruments.

Change is ~180 lines across 3 backtest-only files, **zero in any strategy**:
- `config.rs` — `recording_path: String` → `recording_paths: Vec<String>`. Config accepts either `recording_path = "one.bin"` (single, unchanged) or `recording_paths = "a.bin, b.bin"` (comma list); exactly one of the two keys, both/neither is a hard error. 3 new tests.
- `feed_replay.rs` — `RecordSource` gains a `MergeSource` wrapper: k-way merge of N sources keyed on each outer record's own `exchange_ts` (leading `PacketHeader.TransactTime`), tie-broken on the path's config index. Merge on `exchange_ts` (not `recorder_ts`) is deliberate — it keeps the merged sequence monotonic non-decreasing on `exchange_ts`, which is exactly the invariant `main.rs`'s lookahead-drain already assumes, so that logic needs **no change** for N > 1. `replay(capture_paths: &[String], ...)`. One buffered record per stream, payload buffers swap with the caller's (bounded memory unchanged). 5 new unit tests (`mod tests`, this file's first). See `feed_replay_user_doc.md` §2b.
- `main.rs` — `capture_paths` / `primary_path` (`recording_paths[0]`, resolves the day's `MCXScrips.bcp`); the snapshot band-scan loops one paired snapshot file per stream and unions the bands before replay.

Verified: `cargo test --release` — **294 tests, 0 failures** (281 baseline + 8 in the `qtrade` bin + 5 in `feed-replay-validate`). **Single-stream (N=1) is byte-identical to the pre-merge code**: `multi_instrument_bracket` on `21_08_2026` stream 4 alone reproduces the exact prior result — 233,019,217 outer records, 101 round trips (all TIMEOUT), 202 fills, `gross_pnl=-8625.0000 net_pnl=-17437.1953 total_cost=8812.1953`, `events.log` 2,327 lines / `orders.log` 405 / `fills.log` 203. **Multi-stream (streams 2 + 4 merged, full day)**: 721,746,658 outer records, 1,445,026,749 messages, ~17 min — **204 round trips, 103 CRUDEOIL + 101 NATURALGAS**, both trading off the one merged feed, 408 fills, `denied=0 rejected=0`, `gross_pnl=32475 net_pnl=8481.18`. The NATURALGAS side of the merged run is identical to the standalone stream-4 run (same 101 round trips, same entry/exit prices) — the merge is a clean superset, not a perturbation. Same config run twice → identical output (determinism / NFR-01 holds). Docs: `config_user_doc.md` §2, `feed_replay_user_doc.md` §2 + new §2b, this entry.

Backtest-only, same as everything else in `feed_replay` / the `Scheduler`-driven loop: live combines its multicast streams by arrival order, not a timestamp merge over files. The config field is just unread in live, same as `[deployment]` keys are unread in backtest.

## Post-phase-1: partial-fill events now report `PartiallyFilled` (2026-09-01)

Small correction found while building a throwaway `order_lifecycle_demo` strategy (deliberately walks an order through every reachable `OrderState`). `execution.rs` always tracked `order.state` correctly through a partial fill, but the two `log_event` calls that build the `OrderEventRecord` for `on_order_update` / `orders.log` each passed a **hardcoded literal**: the fill site emitted `OrderState::Filled`, the `Resting` site emitted `OrderState::Accepted` — so a partial fill surfaced to a strategy as `Filled` then `Accepted`, never `PartiallyFilled` (a strategy would read the first event as "whole order filled"; an `Accepted` after `Filled` reads as a terminal→non-terminal regression). Fixed: both sites capture `order.state` after updating it and pass that; unknown/already-terminal orders keep the old fallback. Descriptions clarified (`"partially filled qty=N kind=… (leaves=M)"` / `"remainder working after partial fill"`). Verified against real `21_08_2026` stream-4 data via the demo strategy (a ~900-lot touch-only `LimitDay` that fills ~81 lots and rests the rest — now emits `PartiallyFilled` twice); `cargo test --release` unchanged at **294 passing, 0 failed** (no test asserted the old hardcoded value). Docs: `execution_user_doc.md` §1, `strategy/order_lifecycle_demo/order_lifecycle_demo.md`. `main.rs` currently compiles `order_lifecycle_demo` in place of `multi_instrument_bracket` — a local demo swap, not committed.

## Post-phase-1: `SimExchange` is now a fill estimator, not a matching engine, on the aggressive path (2026-09-03)

Revises part of D21's accepted phase-1 approximation (leak #3, "you would
have absorbed flow that historically went elsewhere"), not a bug fix: an
aggressive fill (IOC / MarketToLimit / a marketable Limit) used to
*physically* remove the swept quantity from the real resting order it
matched — `simulator.rs`'s own book, independent of `cache` per D10, was
nonetheless being genuinely mutated by our own trading, so every
subsequent replayed event (a Delete, a Modify, a real Trade) touching
that same order was interacting with a book state the recording never
actually produced. The passive fill path (a real `Trade` message
attributed to one of our resting orders) was already correct — it only
ever credits a bounded fill, never rewrites the real trade.

Fix, `simulator.rs` only: `SweptSlot`'s real-order leg is now read-only —
`sweep_opposite` tracks what we've virtually taken per real order
(`SimBookImpl::consumed_by_us`, keyed by the order's own `priority_ts`)
instead of popping/decrementing its FIFO slot. `best_bid`/`best_ask`/
`depth`/`qty_at_price` now always reflect the replay's ground truth,
unperturbed by our own orders; `qty_ahead_of`/`MboBook::queue_position`
net a real slot's quantity against `consumed_by_us` so a *different*
resting order of ours behind that slot correctly sees less ahead of it.
The ledger entry for a real order is dropped the moment that order's
identity is genuinely gone (deleted, price-changed, fully traded, mass-
deleted) — cheap hygiene, never load-bearing (`priority_ts` is never
reused). Our own resting sim orders caught in a sweep (a mechanical
self-trade, FR-B25/STP still out of scope) are unaffected — still
genuinely mutated, since that's our own bookkeeping, not the recording's.

Known, accepted residual leak, documented rather than hidden: nothing
stops the same genuinely-resting real quantity from also being consumed
by the historical tape's own future trades, since the recording plays
out exactly as captured regardless of what we do (the "no market impact"
assumption `simulator_user_doc.md` §8 now names explicitly). No
participation cap or displacement model yet.

Verified: `cargo test --release` — 4 new unit tests (real book depth
untouched by our own aggressive fill; a second aggressive order can't
re-claim liquidity we already virtually took; the ledger is forgotten
once its real order is deleted, with no bleed onto an unrelated new order
at the same price; a resting order of ours sees reduced `qty_ahead` for a
real slot we'd already drawn down) — all existing tests pass unchanged,
112 in the `qtrade` bin. Real-data re-run of `simulator-validate
full-session` against the same file `simulator_user_doc.md` §7's original
table used (`mcx_feeder_Increment_capture_19_01_2026_1_4.bin`, 114.4M
records, 1.13M for CRUDEOIL): all 6 FR-B24 invariants still **PASS**,
including 1b under the new per-slot accounting (6,381 aggressive fill
legs, 0 violations) and #4 (10,063 qty-ahead observations, 0 violations).
Raw counts differ from the historical §7 table (intervening feature work
changed how much the harness's strategy trades) — reported as a separate
entry, not overwriting it. Docs: `simulator_user_doc.md` §5, §6.1, §7,
§8, this entry.

## Post-phase-1: own-order injection -- `cache` learns about our own orders from the tape (2026-09-03, Phase 2 of the same day's SimExchange work)

Closes the other half of the idea Phase 1 (above) opened: SimExchange
is now a fill estimator that never mutates the replay's own book, but
until this change `cache` -- and therefore a strategy reading
`cache.book(id).queue_position(...)` -- had **no idea our own orders
existed at all**. The only way to learn "how far ahead am I" was the
`main.rs`-bridged `ExecutionEngine::prepare_for_market_event` /
`SimExchange::resting_qty_ahead` side channel.

Fix, `execution.rs` + `main.rs`, ~230 lines, no strategy-facing API
change: every `ExecReport` that changes what our own order contributes to
a book (`Resting`/`Filled`/`Canceled`) is converted into the same kind of
`DecodedMessage` a real MCX order would produce (`OrderAdd`,
`OrderModifySamePriority`, `OrderDelete`) and queued in a new
`ExecutionEngine::pending_cache_injections` outbox. `main.rs` gained
`drain_cache_injections`, called from the same two spots
`sync_venue_alarms` already is, scheduling each queued message toward
`Target::Cache` at `now` -- `cache`'s book applies it through the exact
same `book::BookBuilder::apply` pipeline a real message uses, no
special-casing. The identity used is `ExecReport::Resting`'s own
`handle.priority_ts` (`simulator`'s `sim_id`, `simulator::SIM_ID_BASE`
made `pub` for this) -- already collision-free against real
`priority_ts` values, already carrying the right priority semantics
(retained across a same-price qty-only modify, fresh across a price
change or qty increase), so nothing new had to be minted. `Order` gained
one field, `cache_injected_at: Option<(Price, u64)>`, tracking what's
currently published so a resting/fill/cancel report can tell "first
time", "same slot, smaller qty", and "identity moved" apart.

One known, accepted edge case, documented not hidden: `book.rs`'s own
fallback-cascade match (used when a real trade's `event_time` doesn't
name a specific resting order -- rare, since real MCX execution reports
reliably do per D21 finding #3) can now walk into *our* injected slot
even though that specific real trade never named it, since our order
shares one FIFO with real ones. Not solved here -- see
`execution_user_doc.md` §12.3.

Verified: `cargo test --release` -- 8 new unit tests (one per row of the
ExecReport-to-message table, including the two "nothing happens" cases:
a pure aggressive fill against real liquidity, and a rejected modify)
plus one end-to-end test that feeds *only* what `ExecutionEngine`
actually queues into a real `book::BookBuilder` and confirms
`queue_position` comes back correct for both our own order and a real
order arriving after it. `execution/validate.rs` gained a `mod book`
declaration so that test module compiles under `cargo test --bin
execution-validate` too. Whole workspace clean, 0 failures across all 7
binaries (`qtrade` bin: 120 passing, up from 112). Real-data run:
`order_lifecycle_demo` against `21_08_2026` stream-4, full file (40M
outer records, 80M messages, ~31s) -- terminal counts unchanged
(`denied=1 rejected=1 filled=1 canceled=2 expired=0`), and a
`BOOK_DEBUG_MISSES=1` re-run produced **zero** `[MISS]` lines from
`book.rs`'s `remove_order`/`modify_same_priority` across the order's
full real lifecycle (rest, same-price modify, cancel, partial fill,
remainder cancel, final fill) -- every injected message found exactly
the slot it expected. Docs: `execution_user_doc.md` §12,
`main_user_doc.md` §5a, this entry.

## Environment

- Rust toolchain: `rustc`/`cargo` 1.98.0, via rustup, user-local under `~/.cargo`.
- Project: `/home/vaibhav/QTrade/qtrade/` — single Cargo package (not a workspace), builds as `qtrade`.
- Convention: one folder per component under `qtrade/src/`, each holding `<component>.rs` + `<component>_user_doc.md`.
- Real recorded data confirmed available (read-only) at `/mnt/MCX_Recording_Files/` — increment + snapshot captures per stream, `CONTRACT/<date>/MCXScrips.bcp`. Validated dates so far: `19_01_2026`, `15_06_2026`, `19_08_2026` (pre-NTP-sync, unsafe for the dual-clock model — see feed_replay §2a), `21_08_2026` (first verified-clean post-sync day, real 60M-record scan, zero negative feed-latency deltas — used for every dual-clock run since 2026-08-27).

## Superseded

An earlier multi-crate plan (separate `qtrade-types`/`qtrade-refdata`/`qtrade-book`/`adapters/qtrade-adapter-mcx` crates in a Cargo workspace) was built once, then abandoned in favor of the single-package, folder-per-component convention above. Nothing from that plan applies anymore.

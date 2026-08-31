# Scheduler — component documentation

**What this component does, in one sentence:** a priority-queue event loop and `SimClock` that make a backtest reproducible — pop the earliest event, advance simulated time to it, dispatch; nothing else in qtrade is allowed to move time.

Code: [`scheduler.rs`](scheduler.rs) (this folder). **Wired into `main.rs` for real as of the dual-clock replay pass (2026-08-27)** — `main.rs`'s own replay loop constructs `Scheduler::new()` directly and drives it (`schedule`/`pop_earliest`/`peek_earliest_timestamp`); this file went from "built, zero real callers" to the actual mechanism `main.rs` runs on, unchanged internally except for `EventPayload` finally carrying real data (§4) and one new method (`peek_earliest_timestamp`, §2). See `main_user_doc.md` for the full account of *why* (real, measured feed latency; see `feed_replay_user_doc.md`) and exactly how `main.rs`'s loop uses it.

---

## 1. How to run it

Ordinary `cargo test` now — no standalone `rustc` invocation needed, `scheduler` is declared in `main.rs`'s own module tree the same way `decoder`/`types` are:

```bash
cd qtrade
cargo test --release scheduler::
```

Real output, captured after the dual-clock replay pass (see §6 for what each test is proving):

```
running 5 tests
test scheduler::tests::clock_now_before_start_panics_instead_of_defaulting - should panic ... ok
test scheduler::tests::clock_rejects_moving_backward - should panic ... ok
test scheduler::tests::determinism_two_runs_are_byte_identical ... --- run A ---
now=1000                 dispatch: t=1000                 class=MARKET_DATA seq=0      MarketData(target=Cache, seq=1, exchange_ts=0, recorder_ts=0)
now=1000                 dispatch: t=1000                 class=STRATEGY_TIMER seq=1      StrategyTimer(quote_refresh)
now=1500                 dispatch: t=1500                 class=MARKET_DATA seq=2      MarketData(target=Cache, seq=2, exchange_ts=0, recorder_ts=0)
now=1750                 dispatch: t=1750                 class=ORDER_ARRIVAL seq=5      OrderArrival(op_id=42)
now=2000                 dispatch: t=2000                 class=MARKET_DATA seq=4      MarketData(target=Cache, seq=3, exchange_ts=0, recorder_ts=0)
now=2000                 dispatch: t=2000                 class=STRATEGY_TIMER seq=3      StrategyTimer(risk_check)

--- run B --- (identical to run A, omitted)

final clock A=2000 B=2000
ok
test scheduler::tests::pop_earliest_on_empty_queue_is_none_and_ends_the_loop ... ok
test scheduler::tests::ties_resolve_deterministically_and_repeatably ... --- tie-break run 1 ---
now=5000 seq=0 class=MARKET_DATA MarketData(target=Cache, seq=100, exchange_ts=0, recorder_ts=0)
now=5000 seq=1 class=MARKET_DATA MarketData(target=Cache, seq=101, exchange_ts=0, recorder_ts=0)
now=6000 seq=3 class=MARKET_DATA MarketData(target=Cache, seq=200, exchange_ts=0, recorder_ts=0)
now=6000 seq=2 class=ORDER_ARRIVAL OrderArrival(op_id=7)

--- tie-break run 2 --- (identical, omitted)

ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`op_id` (renamed from `client_order_id`, dual-clock replay 2026-08-27) is deliberately opaque -- a cancel/modify delivery isn't "a new order," and this module never interprets the value either way; `main.rs` looks it up in its own pending-ops table. `MarketData`'s `target`/`exchange_ts`/`recorder_ts` fields are real now too (§4) -- these tests just leave them at defaults (`Cache`/`0`/`0`) since they're testing ordering, not real timestamps.

Run A and run B are the literal `String` outputs of two independent calls to the same function with the same fixed input; the test asserts them equal with `assert_eq!` before printing them, so `ok` above already means byte-identical — the printed logs are shown for a human to double-check, not in place of the assertion.

---

## 2. The loop (FR-B12)

```rust
loop {
    let Some(event) = scheduler.pop_earliest() else { break };
    clock.set(event.timestamp);        // time moves ONLY here
    dispatch(event);                   // handlers may enqueue more
}
```

`Scheduler::run()` in `scheduler.rs` is this loop, verbatim, with `dispatch` as a caller-supplied closure `FnMut(&mut Scheduler, &SimClock, &Event)`. The closure gets `&mut Scheduler` back specifically so a handler can enqueue further events — a strategy timer rearming itself, a market event causing an order that arrives later — which is exactly how the loop keeps running: it isn't fed from one static list, it's fed by its own handlers, and it stops only when the queue is genuinely empty.

**What `dispatch` is not, here:** this component does not know about strategies, books, or the Cache. `run()`'s closure parameter is generic on purpose — T05 (Cache, filter and dispatch) supplies the real one later. Building that logic here would be scope creep past "the loop, the clock, and the priority queue," which is this milestone's entire deliverable.

---

## 3. Ordering key (FR-B13) — and why ties are guaranteed, not an edge case

Every `Event` carries `(timestamp, event_class, seq)`, and `pop_earliest()` always returns the smallest such tuple in the queue (lexicographic: `timestamp` first, then `event_class`, then `seq`).

**Ties are routine.** FR-B13 says this explicitly, and it's easy to see why: a strategy timer set for a round number like `09:20:00.000000000` is entirely likely to land on the exact same nanosecond as a market data event, a heartbeat timeout, or another timer — there is nothing about simulated time that keeps independently-scheduled events apart. An event loop whose behavior on a tie is "whatever the underlying heap does today" is not reproducible: the same logical input could dispatch in a different order on a different build, or after an unrelated refactor of the enqueue call sites, and every one of BACKTEST-PHASE1.md's equality assertions (FR-11 through FR-14, and this milestone's own FR-B15) rests on that never happening. So the tie-break isn't an afterthought bolted onto the ordering key — it *is* the ordering key's second and third components, each fully specified:

### 3.1 `event_class` — exogenous before endogenous

`EventClass` (in `scheduler.rs`) ranks *why* an event exists, lowest rank dispatched first:

| Rank | `EventClass` | Why this rank |
|---|---|---|
| 0 | `MarketData` | The venue's ground truth. Everything else reacts to it, so at a shared timestamp it must be visible first. |
| 1 | `SessionTransition` | Venue-declared state (pre-open/continuous/auction/close) — exogenous, same tier as market data. |
| 2 | `ReportDelivery` | A fill already happened at the venue; this is qtrade *learning about it* at `fill_time + inbound latency`. Still an exogenous fact, just a delayed one. |
| 3 | `StalenessOrHeartbeatTimeout` | Fires when nothing arrives — an absence is still a fact about the world, not something qtrade scheduled for itself. |
| 4 | `WatchdogExpiry` | Declared-dependency staleness detection (D28) — same "system noticed a fact" tier. |
| 5 | `OrderArrival` | qtrade's own order becoming visible to the venue at `submit_time + outbound latency`. Endogenous: exists only because qtrade decided to submit something. |
| 6 | `StrategyTimer` | A strategy's own `set_timer`/`set_alarm` firing. Endogenous. |
| 7 | `OffloadCompletion` | An expensive strategy computation's result returning as a scheduled event (D04's offload mechanism), rather than a blocking call. Endogenous, and last. |

The rule behind the ranking, in one sentence: **exogenous facts about the world outrank endogenous facts qtrade generated for itself**, so that a strategy can never perceive its own pending action as if it had landed *before* real market activity at the identical instant. This is not an invented rule — it's the literal scenario ARCHITECTURE.md §5.4's worked trace walks through: real participants arriving during the 250µs outbound-latency window are processed *before* the `OrderArrival` they are racing against, even though both could in principle share a timestamp. `EventClass`'s variant order in `scheduler.rs` is declared with explicit `#[repr(u8)]` discriminants matching this table exactly, so a future cosmetic reordering of the `enum` body can't silently change the tie-break by accident (`derive(Ord)` on a fieldless enum compares by declaration order, so pinning the discriminants pins the meaning too).

### 3.2 `seq` — enqueue order, assigned once

Within the same `(timestamp, event_class)`, the event enqueued first wins. `Scheduler::schedule()` assigns `seq` from an internal counter that only ever increments, once per call, and the assigned value never changes afterward. Because qtrade is single-threaded (**D04**), "which call happened first" is not a race between threads — it's a deterministic fact about the order the (single) call stack made those calls in, which is exactly the same on every run given the same input. That's what makes `seq` a valid third tie-break component rather than merely "insertion order that happens to work today": it's pinned to something that cannot vary between runs, by construction, not by convention.

**Cross-class ties are decided by `event_class` alone, regardless of `seq`.** `scheduler.rs`'s `ties_resolve_deterministically_and_repeatably` test enqueues an `OrderArrival` *before* a `MarketData` event sharing the same timestamp — enqueue order alone would fire `OrderArrival` first — and confirms `MarketData` still wins, because it ranks lower. See §6 for the full test and its actual output.

---

## 4. What can be scheduled

`EventPayload` (in `scheduler.rs`) is the data an `Event` carries; `EventClass` (§3.1) is only the ordering tag. FR-B14 lists every source this queue accepts; three are real and wired as of the dual-clock replay pass (2026-08-27), the rest remain shape only:

| Source (FR-B14) | `EventClass` | `EventPayload` variant | Status |
|---|---|---|---|
| Market data, per real event | `MarketData` | `MarketData { target: Target, message: DecodedMessage, seq_no, exchange_ts, recorder_ts }` | **Real.** `target` (`SimExchange` or `Cache`, a new local enum) is the one addition beyond a plain decoded message -- the same real event is scheduled *twice*, once per target, at two different real timestamps (`main.rs` does this; see `main_user_doc.md`). `DecodedMessage` is the one dependency this module takes on beyond `std` (`use crate::decoder::DecodedMessage;`) -- still well below `cache`/`execution`/`simulator` in the stack. |
| Order arrival (`T + outbound latency`) | `OrderArrival` | `OrderArrival { op_id }` | **Real.** `op_id` is deliberately opaque -- `strategy::Ctx::submit`/`cancel`/`modify` schedule these (at `ctx.now() + latency_ns`, a flat config constant for now, D18's `LatencyModel` later), keyed into `main.rs`'s own pending-ops table (`strategy::PendingVenueOp`), which this module never reads. |
| Report delivery (`T + inbound latency`) | `ReportDelivery` | `ReportDelivery { op_id }` | **Real**, same opaque convention. `main.rs` schedules these whenever a venue call (market-event-driven or order-driven) produces a non-empty `ExecOutcome`, keyed by its own monotonic counter (not `client_order_id` -- more than one order's report can be in flight at once). |
| Strategy timers and alarms | `StrategyTimer` | `StrategyTimer { label }` | **Synthesized in tests only.** Represents a strategy's `set_timer`/`set_alarm` -- still unbacked; nothing in `main.rs` calls it. |
| Session transitions | `SessionTransition` | `SessionTransition { session }` | **Shape only** — no session-state source exists yet. |
| Staleness / heartbeat timeouts | `StalenessOrHeartbeatTimeout` | `StalenessTimeout` | **Shape only.** |
| Watchdog expiry | `WatchdogExpiry` | `WatchdogExpiry` | **Shape only.** |
| Offload completion | `OffloadCompletion` | `OffloadCompletion` | **Shape only** — matches FR-B14's own table, which lists this as "scaffold only" even for phase 1's full scope. |

**Dispatch pattern-matching lives in `main.rs`, not here.** `main.rs`'s own `dispatch_event` function (not a method on `Scheduler`) matches a popped `Event`'s payload and calls the right thing -- `venue.apply_market_event(...)`, `event_dispatcher.on_book_touched(...)`, `engine.deliver_order(...)`, `control_dispatcher.dispatch(...)`. This keeps D07's rule intact ("routing knowledge lives in startup wiring, never inside either dispatcher") extended to this component too: `scheduler.rs` never gained a `cache`/`execution`/`simulator` dependency, only `decoder`.

**A new read-only method, added for `main.rs`'s own use:** `peek_earliest_timestamp(&self) -> Option<Timestamp>` -- the earliest queued event's timestamp without popping it, letting `main.rs` decide *when* it's safe to drain (see `main_user_doc.md`'s account of the read-ahead/drain interleaving between file records and scheduled events) without a pop-then-push-back dance.

---

## 5. `SimClock` and wall-clock time (FR-B15, D30)

`SimClock::now()` returns the timestamp of whichever event is currently being dispatched. `SimClock::set()` is the only method that changes it, and `Scheduler::run()` is the only place that calls it — matching FR-B12's loop and the invariant ARCHITECTURE.md §4.6 states outright: *"Time only moves forward."* `set()` asserts this and panics rather than silently letting a run corrupt itself (see the `clock_rejects_moving_backward` test, §6) — an invariant that only shows up in an error message, rather than being enforced, isn't an invariant.

`SimClock` never sleeps and never performs a real-time wait. In backtest mode, ARCHITECTURE.md §5.4 puts it plainly: *"time is free"* — the clock jumps from event to event however far apart their timestamps are, and how long the *wall-clock* run takes is throughput, a completely different quantity `SimClock` has no opinion about.

**`Timestamp` (a type alias for `i64`) is never a wall-clock/epoch value by itself — D30's whole point.** It's a value on a monotonic axis, meaningful only for "earlier/later than another `Timestamp` from the same run," exactly like `decoder`'s own `capture_ts` (see `decoder/user_doc.md` §3.1: *"this is not a wall-clock date... it's only meaningful for ordering records relative to each other"*). D30's hazard section explains why this distinction is load-bearing rather than pedantic: an NTP correction can step a wall clock *backward* mid-session; if `Timestamp` were wall-clock-derived, a correction could silently violate the Scheduler's forward-only invariant and break every downstream ordering guarantee this file exists to provide. Nothing in `scheduler.rs` converts a `Timestamp` to or from a calendar date or an epoch value — that mapping, where it's needed at all (for display, or correlating against exchange timestamps), belongs to a per-session wall-clock anchor kept elsewhere, per D30, not to this clock.

**`SimClock` vs. a future `LiveClock`:** D30 anticipates one clock *interface*, two sources — a simulated one here, a real-time one in live mode, with "no mode branch in calling code." `LiveClock` doesn't exist yet (phase 1 is backtest-only, and building it now would be exactly the kind of speculative plumbing this milestone's "out of scope" list rules out) — but nothing in `SimClock`'s public surface (`now()`, `is_started()`) assumes it will never have a sibling; a trait extraction later, if `simulator`/live wiring needs one, is mechanical, not a redesign.

---

## 6. Determinism — the acceptance bar (FR-B15), demonstrated

Two `#[test]` functions in `scheduler.rs` are the actual demonstration, not an argument for why determinism *should* hold:

- **`determinism_two_runs_are_byte_identical`** — schedules a fixed sequence of five events (market data and strategy timers, both sharing timestamps at points, plus one handler that enqueues a sixth event — an `OrderArrival` 250 (simulated) nanoseconds after a market event, proving the T+latency shape from §4 for real), runs the FR-B12 loop twice from scratch, and asserts the two recorded dispatch logs are equal with `assert_eq!` before printing either — see the captured output in §1. It also pins the exact dispatched order line-by-line, so a change that broke the tie-break rule but happened to still be self-consistent between two runs of one build would still fail.
- **`ties_resolve_deterministically_and_repeatably`** — constructs two events sharing the identical `(timestamp, event_class)` (both `MarketData` at `t=5000`), enqueued in a known order, and checks `seq` alone decides the winner; then constructs a same-timestamp pair from *different* classes (`OrderArrival` enqueued before `MarketData`, both at `t=6000`) and checks `event_class` overrides enqueue order — `MarketData` dispatches first despite being enqueued second. Runs the whole case twice and asserts the logs match.

Two supporting tests check the invariants the above rely on: `clock_rejects_moving_backward` (a `set()` to an earlier timestamp panics rather than silently corrupting ordering) and `clock_now_before_start_panics_instead_of_defaulting` (`now()` before the first dispatched event panics rather than returning a fabricated `0`, which would misrepresent "no event yet" as a real moment in time).

All five tests, run twice each by construction (`--test-threads=1` just serializes the terminal output; nothing about the tests themselves depends on ordering across each other) are shown passing in §1.

---

## 7. What this component deliberately does not do

- No Cache, no read model for strategies, no `cache`/`execution`/`simulator` dependency — dispatch pattern-matching lives in `main.rs`'s own `dispatch_event` function (§4), not here. `Scheduler::run()`'s `dispatch` parameter stays a generic closure; nothing here knows what a `Strategy` is. (`main.rs`'s real loop doesn't even use `run()` — it needs to interleave popping with reading more file records, which `run()`'s "drain everything" shape doesn't fit; `run()` stays available, just not this particular caller's choice.)
- No real `LatencyModel` (D18) yet — `main.rs` computes `OrderArrival`/`ReportDelivery` timestamps from one flat, config-driven constant (`cfg.run.latency_ns`), not a real per-venue/per-direction model. The event *shape* has always supported an arbitrary future timestamp regardless of how it's computed; swapping in `Fixed`/`Sampled` later touches the caller, not this file.
- No `LiveClock` — D30 anticipates one, phase 1 doesn't need it, and building it now would be exactly the speculative work this milestone excludes.
- No wall-clock conversion anywhere — `Timestamp` stays on its own monotonic axis end to end (§5).

These are scope boundaries, not missing features waiting to be discovered as bugs.

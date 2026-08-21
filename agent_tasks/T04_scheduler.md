# T04 — `scheduler`

**Folder:** `qtrade/src/scheduler/` → `scheduler.rs` + `scheduler_user_doc.md`
**Depends on:** `types` (T00)
**Milestone:** M4 — Scheduler and clock

---

## What it is

The one thing that makes a backtest reproducible. A priority queue of events ordered by `(timestamp, event_class, monotonic_seq)`; popping the earliest event, setting the clock to it, dispatching — and *only* the loop advances time. Nothing else in the system is allowed to.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M4 in full — FR-B12 (the loop, given verbatim), FR-B13 (total ordering — ties are routine, not hypothetical), FR-B14 (event sources beyond market data), FR-B15 (`SimClock`)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D04 (single-threaded qtrade — this is *why* reproducibility is achievable at all) and D30 (monotonic clock for ordering, wall clock only for display — never conflate the two)
- [../ARCHITECTURE.md](../ARCHITECTURE.md) §4.6 (Scheduler and Clock component spec) and §5.1 (the worked trace — read this for the *shape* of what gets enqueued and when, even though the venue/latency pieces it references aren't built yet)

## Build

```rust
loop {
    let Some(event) = scheduler.pop_earliest() else { break };
    clock.set(event.timestamp);        // time moves ONLY here
    dispatch(event);                   // handlers may enqueue more
}
```

- Event ordering key: `(timestamp, event_class, monotonic_seq)`, sequence assigned at enqueue time — **an undefined tie makes every equality assertion in the test plan meaningless**, so pick a deterministic, documented tie-break and don't leave it to insertion order accidentally.
- `SimClock`: `now()` returns the current event's timestamp. Never sleeps, never waits — backtest time is free, wall-clock runtime is throughput.
- Event sources this phase actually needs: market data (from `decoder`/`book`), strategy timers/alarms. Order-arrival and report-delivery events exist as a *concept* here (FR-B14) but have nothing to feed them until `simulator` (T06) — don't build the simulator's latency plumbing now, just make sure the scheduler's event type can represent "this fires at T+latency" generically.

## Out of scope

Anything that isn't the loop, the clock, and the priority queue itself. No Cache, no dispatch-to-strategy logic (that's T05). No latency model (T06).

## Acceptance (FR-B15)

Replay the same recording twice through `decoder` → `scheduler`, assert byte-identical output. This is the determinism foundation every later milestone's equality assertions rest on — don't skip it because "nothing depends on it yet."

## Done when

- [ ] Priority queue with the exact ordering key above, tie-break documented
- [ ] `SimClock` — no sleeps, no real-time waits anywhere
- [ ] Two runs of the same input, byte-identical
- [ ] `scheduler_user_doc.md` written — how the loop works, what can be scheduled, why ties are guaranteed rather than an edge case

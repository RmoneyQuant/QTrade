# T06 — `simulator`

**Folder:** `qtrade/src/simulator/` → `simulator.rs` + `simulator_user_doc.md`
**Depends on:** `types` (T00), `book`'s traits (T03) for shape reuse, `decoder`'s message stream (T02) — **not** `cache` (see below, this is deliberate)
**Milestone:** M6 — Simulated Exchange. **The highest-risk component in the whole project** — it's the one part of the system that can lie silently. A wrong book crashes or diverges visibly; a wrong fill model produces plausible numbers that get trusted for months.

---

## What it is

Stands in for MCX during a backtest. Builds its **own** books, independently, from the same normalized message stream `book` (T03) consumes — with **zero read access** to `cache`.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M6 in full — FR-B19 through FR-B25
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D10 (why independence is non-negotiable — a real exchange doesn't get uncertain because your receiver dropped packets; if the simulator read your Cache it would inherit corruption that shouldn't exist for it), D18 (latency model, swappable trait), D19 (OTR, two independent counters), D21 (modify/cancel-replace priority, MMP, markout), D31 Layer 2 (the invariants, below)
- [../OPEN-QUESTIONS.md](../OPEN-QUESTIONS.md) §1 (the four-layer validation approach, D31's origin — Layers 1/2 are what this task actually builds; Layer 4, shadow comparison, needs live trading and is out of scope)

## Why it doesn't depend on `cache`

**This is the one dependency inversion in the whole list, and it's deliberate, not an oversight.** `simulator` consumes `decoder`'s output directly, same as `book` does — it does not read anything `cache`/`book` computed. Reading the strategy's own view would let the simulator inherit that view's corruption (a `STALE` book after a packet gap) and fill orders against a book that never existed in production. Build it as if it were a completely separate process, because in live trading it is.

## Build

- `pub trait LatencyModel { fn outbound(&mut self, venue: Venue) -> Duration; fn inbound(&mut self, venue: Venue) -> Duration; }` — `Fixed` and `Sampled` (seeded), per venue and direction (FR-B20).
- Its own books over the whole filtered instrument set (same filter predicate as `cache`'s, D32 — coordinate the shared filter logic rather than re-deriving it, but this component must not *read* `cache`'s filtered set, just apply the same rule independently).
- Queue position: insert an arriving order at the back of the queue at its price, track `qty_ahead`, decrement on executions/deletions ahead of it (FR-B21) — this is the entire reason MBO data matters.
- Order types: `Limit+Day` rests; `BookOrCancel` rejected outright if it would cross (never filled); `IOC` fills what it can; `MarketToLimit` executes available then **rests the residual as a limit**, never sweeps (FR-B22).
- Modify semantics: quantity reduction keeps queue priority; price change or quantity increase loses it (FR-B23) — this is most of a market maker's P&L, get it exact, cross-check against `MCX_Feeder.cpp`'s modify handling same as `book` did.

## The invariants — run these on every backtest, not just once (FR-B24)

Strongest first: **simulated fills at a price/time must never exceed the volume that actually traded there in the recording.** Filling 100 lots where 20 traded means fabricated liquidity — every downstream result is worthless. Supporting: BOC-crosses-and-rejects (never fills), fill price at-or-better-than limit, queue position never improves except through consumption ahead, `MarketToLimit` residual rests rather than vanishing, simulated OTR/message-rate never exceed configured limits.

## Out of scope

STP, MMP, watchdog cancellations, Lean-order end-of-day cancellation (FR-B25) — real requirements, but layer them in after the core fill logic and invariants above are solid, not simultaneously with them.

## Acceptance

Run a strategy with analytically predictable behaviour (a fixed quote, known queue depth) and verify fills by hand against the recording. Then assert all FR-B24 invariants hold across a full real session.

## Done when

- [ ] Independent books, zero dependency on `cache`
- [ ] `LatencyModel` trait, `Fixed` + `Sampled`, seeded
- [ ] Queue position exact, modify-priority rules match `MCX_Feeder.cpp`'s documented behaviour
- [ ] BOC/IOC/MarketToLimit modelled per FR-B22
- [ ] All FR-B24 invariants pass on a full real session
- [ ] `simulator_user_doc.md` written — why it's independent, what the invariants check and why each one matters, worked example of the queue-position trace

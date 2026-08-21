# T07 — `execution`

**Folder:** `qtrade/src/execution/` → `execution.rs` + `execution_user_doc.md`
**Depends on:** `types` (T00), `cache` (T05), `simulator` (T06)
**Milestone:** M7 — Execution, accounting and reporting. The last milestone in BACKTEST-PHASE1.md's build order — completing this satisfies the phase-1 definition of done.

---

## What it is

Order lifecycle, two-level accounting, transaction costs, and the run's output — bundled as one component because BACKTEST-PHASE1.md itself treats them as one milestone (FR-B26–B31). If this grows unwieldy once built, splitting `execution` from `reporting` afterward is a cheap, justified refactor — don't pre-split now on a guess.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M7 in full — FR-B26 through FR-B31
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D34 (RMS is a trait, phase-1 impl always says yes — same swappable-trait pattern as `LatencyModel` in T06), D36 (local gate rejections are synchronous; venue rejections arrive as events — genuinely different, not two flavours of one), D08 (two-level accounting: per-strategy sub-account + firm aggregate), D23 (Cost Model — direction-asymmetric, CTT on sell side/stamp duty on buy side, no fee concession so round-trip cost is a hard floor on quotable spread), D40 (client order ID: injected session id + monotonic counter — **not** wall-clock, or two identical backtest runs produce different IDs and break determinism)
- [../STRATEGY-GUIDE.md](../STRATEGY-GUIDE.md) §7a — the eleven order states and their transition table, including the two easy-to-miss ones: `PendingCancel → Filled` is a real race, not an edge case; `Denied` (local gate, never left qtrade) is distinct from `Rejected` (venue refused it)

## Build

1. **Order state machine** — eleven states per STRATEGY-GUIDE.md §7a. `is_open()`/`is_inflight()`/`is_terminal()` groupings.
2. **Three gates, two rejection paths** (FR-B27, D36): Validation → RMS → OTR governor, in that order. Local rejections return synchronously with a reason; venue rejections arrive as scheduled events (needs `scheduler`, T04). RMS (D34): a trait, phase-1 implementation always returns yes — the call site must exist now so a real implementation slots in later without touching this component.
3. **Client order ID** (FR-B28, D40): `(session_id, counter)` — `session_id` injected (deterministic value from run config in backtest), `counter` monotonic because the SimClock doesn't advance within a callback (a bid and ask submitted in the same event get the identical timestamp).
4. **Two-level accounting** (FR-B29, D08): per-strategy sub-account (position, inventory, P&L) and firm aggregate (netted). A strategy skews on its own inventory, reads the firm view to degrade gracefully.
5. **Cost Model** (FR-B30, D23): `round_trip(instrument, qty, side) -> Cost`, queryable pre-trade and applied to fills — same model both times, so quoting assumption and realised accounting can't disagree. Rates are config, not hardcoded; direction-asymmetric (CTT sell-side, stamp duty buy-side) is a hard requirement, not a simplification to skip.
6. **Reporting** (FR-B31, D26): Tier 1 always-on summary (P&L both levels, inventory, markout, OTR consumed, message counts). Tier 2 switchable per-event detail (**queue position at fill and markout at fixed horizons are not optional — retrofitting them means re-running everything already trusted**). Tier 3 strategy-published series, deferred until a real `Strategy` trait exists to publish from.

## Out of scope

Margin and cash (later RMS implementation, D34's own deferral). A full `Strategy` trait / strategy authoring API — this task closes out M7's machinery; a real strategy to drive it through is separate follow-on work once M1–M7 all exist.

## Acceptance

Rejects and partial fills drive the state machine correctly (verify the `PendingCancel → Filled` race explicitly, not just the happy path). Report produced with run identity `(config hash, build hash)` printed and embedded — even a placeholder hash scheme is fine here, D22's full config-file infrastructure isn't required to satisfy this milestone's gate.

## Done when

- [ ] Eleven-state order machine, transitions match STRATEGY-GUIDE.md §7a exactly
- [ ] Three gates wired, local-vs-venue rejection distinction real (not both synchronous)
- [ ] Client order ID scheme uses injected `session_id`, never a wall-clock read
- [ ] Two-level accounting, cost model direction-asymmetric and pre-trade queryable
- [ ] Tier 1 report always emitted; queue position + markout present on fill records from day one
- [ ] `execution_user_doc.md` written — the gate sequence, the rejection-path distinction, why client order IDs are constructed this way

# T05 — `cache`

**Folder:** `qtrade/src/cache/` → `cache.rs` + `cache_user_doc.md`
**Depends on:** `types` (T00), `book` (T03), `scheduler` (T04), `refdata` (T01)
**Milestone:** M5 — Cache, filter and dispatch

---

## What it is

Three things bundled under one milestone in BACKTEST-PHASE1.md, and kept as one component here for the same reason: the shared read model strategies see (`Cache`), the instrument filter that decides which events even reach the book (`filter`), and the mechanism that wakes a strategy only when something it cares about changed (`dispatch`).

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M5 in full — FR-B16 (instrument filter), FR-B17 (Cache contents), FR-B18 (depth-scoped dispatch)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D32 (filter is a strategy-declared predicate, applied immediately after decode — and note the "roll trap": the predicate must cover contracts not yet rolled into, or a mid-run subscription finds an empty book) and D25 (depth-scoped subscription — waking, not access; the full book stays available on demand regardless of subscribed depth)

## Build

1. **Filter** — applied right after `decoder` produces a message, keyed on the native `SecurityID`/token, before anything touches `book`. A recording contains every MCX product; without this, building books for everything while only two contracts are quoted dominates runtime (D32).
2. **Cache** — holds: books per filtered instrument (one shared instance, D06 — already the shape `book` produces), book state, reference data for the day (from `refdata`), own orders/positions (stub — `execution`, T07, owns writing these; `cache` just holds them). Read-only to strategies.
3. **Dispatch** — subscriber lists keyed by `(instrument, depth)`. A strategy subscribed at BBO wakes only when the best bid/ask changes, never on a deeper-level-only change — but the full book is still reachable on demand.

## Out of scope

Strategy trait itself (not in phase-1 scope for this task set — `execution`/reporting close out M7 without a real strategy author-facing API yet; that's a later addition). Simulated Exchange (T06). No allocation on the dispatch/book path is the target (NFR-05) but only needs to be *measured* here, not micro-optimized blind.

## Acceptance (per BACKTEST-PHASE1.md §5.1)

A no-op strategy (subscribes, does nothing) runs a full real session end to end through `decoder → filter → book → cache → dispatch`. Record throughput. Profile the dispatch and book paths for allocations — assert zero, per NFR-05.

## Done when

- [ ] Filter applied immediately post-decode, predicate-based, covers roll-forward contracts
- [ ] Cache holds books/state/refdata, read-only surface
- [ ] Dispatch wakes only on subscribed-depth changes, full book still reachable
- [ ] No-op strategy runs a full session; throughput and zero-allocation both measured, not assumed
- [ ] `cache_user_doc.md` written — how filter/cache/dispatch fit together, what "waking vs access" means concretely

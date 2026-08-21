# T03 — MCX Book Builder + snapshot validation

**Wave:** 2
**Depends on:** the `qtrade_types::Event` shape (already defined, done — not on T02's code)
**Owns:** `qtrade/crates/qtrade-book/src/lib.rs` — a standalone crate, not touched by anyone else
**Language:** Rust (see [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D01)

**This crate must never depend on `qtrade-adapter-mcx`.** Check `Cargo.toml` before you start and after you finish — `qtrade-book`'s only dependency should be `qtrade-types`. If you find yourself wanting to import anything from the adapter crate, that's the exchange abstraction leaking (NFR-06) — stop and work out what's missing from `qtrade_types::Event` instead of reaching around it. You are building against the *interface* T02 produces (a stream of `Event`s), not against T02's code.

---

## Context

This is the milestone everything downstream depends on. Per [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M3, the book you build from the incremental stream must match every arriving snapshot cycle at full depth — and until that holds with zero divergences, nothing built on top of it (simulator, execution, strategies) can be trusted, because a wrong book doesn't crash, it produces plausible wrong fills.

`references/MCX_Feeder.cpp` contains a full legacy book-building implementation (roughly lines 19–1600) using a nested price-bucket structure (`ORDER_BOOK_THOUSAND_RANGE` → `Hundred` → `TenRupee` → `OneRupee`, with a dynamically-resizing "DPR" price window). **Do not port this data structure.** [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) FR-B08 already specifies a simpler design for the new system — a dense array indexed by tick offset over the day's price band — and that's the right call: MCX publishes circuit limits, so the range is bounded, and a dense array is both simpler and faster than reimplementing the legacy bucket scheme.

**What the legacy code *is* useful for:** it encodes real, working business rules that aren't fully spelled out in the EOBI spec text — how a modify that crosses the current price-range boundary is handled, exactly when priority is lost vs retained, how mass-delete and session-reset interact with resting orders. Read it for those rules; don't copy its structure.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M3 in full — FR-B08 (MBO book construction), FR-B09 (crossed books are legal, do not assert against them), FR-B10 (book state machine), FR-B11 (snapshot-cycle validation — **the actual gate for this task**)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D06 (one shared book per instrument) and D31 Layer 1 (why the snapshot channel, not `TopOfBook`/`13504`, is the continuous validation source — `13504` is post-trading-only, confirmed from the spec, not usable during the session)
- `../references/MCX_Feeder.cpp`: the modify-handling section starting around line 484 (`OrderModify` case) through roughly line 620+ for the priority/queue-shift logic — read for the *rules*, not the implementation
- Your own crate's decoded event types from T02 — coordinate via `STATUS.md` if T02 hasn't fully landed when you start; you can build against a stubbed subset of its output (Add/Modify/ModifySamePriority/Delete/MassDelete plus the three snapshot templates) if needed

## What to build (Rust)

The `Book`/`MboBook` traits and the `MboBookImpl` struct are already scaffolded in `qtrade-book/src/lib.rs` — signatures only, every method body is `todo!()`. Your job is filling those in, not designing the shape:

```rust
pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    fn depth(&self, n: usize) -> Vec<PriceLevel>;
    fn qty_at_price(&self, side: Side, price: Price) -> Qty;
    fn state(&self) -> BookState;
}

pub trait MboBook: Book {
    fn queue_position(&self, handle: OrderHandle) -> Option<i64>;
}
```

`MboBookImpl::apply(&mut self, event: &qtrade_types::Event)` is where the real work is: handle `Event::OrderAdded`, `OrderModified` (check `priority_retained`), `OrderDeleted`, `OrderMassDeleted`. `Event::Trade` and `Event::SnapshotOrder` also reach this crate — `SnapshotOrder` is what `compare_to_snapshot` consumes, not `apply`.

One `MboBookImpl` per instrument, fed by a stream of `qtrade_types::Event` (however that stream is produced is not this crate's concern — in the near term that's a small test harness calling `qtrade_adapter_mcx::decode` + `normalize` directly, not something `qtrade-book` links against). Price levels as a dense array over the day's tick range (from the `Instrument`'s `price_band` — nothing here needs to know that value came from MCX specifically), each level holding a FIFO of resting order slots plus aggregate quantity/count, per FR-B08's structure guidance.

**FR-B09 is not optional:** `best_bid >= best_ask` is a normal, expected transient state on an order-by-order feed (an aggressive order publishes before the trade it causes). If your implementation panics or asserts on a crossed book, it is wrong, not the data.

**Book state machine** per FR-B10: `Uninit | Recovering | Ok | Stale`. For this round (single-file replay, no live gap recovery yet), you mainly need `Uninit` (before the first event) and `Ok` — full gap-recovery semantics come with the live Transport later.

## The actual gate

Per FR-B11: **replay a full real session, and at every arriving snapshot cycle (a run of `Event::SnapshotOrder`, originating from templates `13600`/`13601`/`13602` in the raw feed — that mapping is the adapter's concern, not yours), your incrementally-built book must equal the snapshot at full depth, for every instrument you're tracking — zero divergences.** A BBO-only check is not sufficient and does not satisfy this milestone; a book can be correct at the top and wrong at depth three. Use the paired snapshot capture file (`mcx_feeder_snapshot_capture_<date>_1_<stream>.bin`) alongside the matching increment file for the same date and stream, decoded and normalized via `qtrade-adapter-mcx` in your test harness.

If divergences occur, do not weaken the check to make it pass — the divergence is telling you something about either the modify/priority rules (cross-check against the legacy code's handling) or a misunderstanding of `qtrade_types::Event`'s semantics. Report exactly where and how it diverges rather than silently narrowing the assertion.

## Out of scope

The dense-array performance optimisation work (NFR-05's zero-allocation requirement matters eventually, but correctness against the snapshot gate comes first this round). Gap recovery / `Recovering` state (needs live Transport, not in scope). Cache, Scheduler, dispatch, Simulated Exchange, execution. Anything in `qtrade-adapter-mcx` — you consume its output type (`qtrade_types::Event`) in a test harness, you don't read or modify its source.

## Constraints

- **Read-only on `/mnt/*` and `references/*`** — no exceptions.
- Test against real, matched increment+snapshot file pairs for the same date and stream under `/mnt/MCX_Recording_Files/` (state which date/stream you used).

## Acceptance

Full-depth book-vs-snapshot comparison passes with zero divergences across a full real session for at least the CRUDEOIL and NATURALGAS instruments T01 identifies. Report the number of snapshot cycles checked and confirm zero divergences, or report exactly what diverged and your working hypothesis for why.

## Done when

- [ ] `MboBookImpl` built from a stream of `qtrade_types::Event`, dense tick-indexed, no crash on crossed books
- [ ] Book state machine implemented (`Uninit`/`Ok` minimum for this round)
- [ ] Snapshot-cycle comparison harness built and run against a real session
- [ ] Zero divergences reported, or a precise, investigated account of what diverged
- [ ] `cargo tree -p qtrade-book` still shows only `qtrade-types` as a dependency — no adapter crate crept in

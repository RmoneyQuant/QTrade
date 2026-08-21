# T03 — `book`

**Folder:** `qtrade/src/book/` → `book.rs` + `book_user_doc.md`
**Depends on:** `types` (T00), `decoder` (T02 — needs its follow-up API, see [T02_decoder.md](T02_decoder.md))
**Milestone:** M3 — Book builder. **This is the milestone everything downstream depends on** — a wrong book doesn't crash, it produces plausible wrong fills.

---

## What it is

One order book per instrument, built incrementally from `decoder`'s message stream, provably correct against MCX's own snapshot cycles.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M3 in full — FR-B08 (construction, trait shape given verbatim), FR-B09 (crossed books are legal — do not assert against them), FR-B10 (state machine), **FR-B11 (snapshot-cycle validation — the actual gate, not a formality)**
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D06 (one shared book per instrument) and D31 Layer 1 (why the snapshot channel, not `TopOfBook`/`13504`, is the validation source — `13504` is post-trading-only, already confirmed and reflected in `decoder`)
- `qtrade/src/decoder/user_doc.md` — the message-type table and price/qty scaling are your input contract; don't re-derive them
- `../references/MCX_Feeder.cpp` — the modify-handling section (~line 484 on) for the **business rules** only (exactly when priority is lost vs retained, how mass-delete interacts with resting orders). **Do not port its data structure** — the nested price-bucket scheme is a legacy design; FR-B08 already specifies a simpler dense array, which is the right call now that MCX's circuit limits bound the price range

## Build

```rust
pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    fn depth(&self, n: usize) -> Vec<PriceLevel>;
    fn qty_at_price(&self, side: Side, price: Price) -> Qty;
    fn state(&self) -> BookState;
}

pub trait MboBook: Book {
    fn queue_position(&self, handle: OrderHandle) -> Option<i64>;   // MBO-only, deliberately not on Book
}
```

One implementation, `MboBookImpl`, dense array indexed by tick offset over the day's price band (from `refdata`'s `Instrument.tick_size`/DPR bounds once T01 lands — coordinate if building in parallel, or stub the range for now). Each level: a FIFO of resting order slots plus aggregate qty/count.

**FR-B09 is not optional:** `best_bid >= best_ask` is a normal transient state on an order-by-order feed (an aggressive order publishes before the trade it causes). A panic or assert on a crossed book is a bug in `book`, not in the data.

**Book state machine** (FR-B10): `Uninit | Recovering | Ok | Stale`. This round, only `Uninit`→`Ok` matters — gap recovery needs a live Transport, out of scope here.

## The actual gate

**FR-B11:** replay a full real session; at every arriving snapshot cycle, the incrementally-built book must equal the snapshot at **full depth** — zero divergences. A BBO-only check is not sufficient. Use the paired files for the same date/stream:
```
mcx_feeder_Increment_capture_<date>_1_<stream>.bin   (build the book from this)
mcx_feeder_snapshot_capture_<date>_1_<stream>.bin    (compare against this)
```
Template IDs `13600`/`13601`/`13602` carry the snapshot content — `decoder` currently reports these as counted but not individually typed; extend it minimally if needed, following the same pattern as its existing message types.

If divergences occur, do not narrow the check to make it pass — report exactly where and why, and check it against the priority/modify rules in `MCX_Feeder.cpp` first.

## Out of scope

Gap recovery / `Recovering` state (needs live Transport). Cache, Scheduler, dispatch, Simulated Exchange, execution. Performance tuning beyond "don't do anything obviously wasteful" — NFR-05's zero-allocation requirement matters at M5, not here.

## Constraints

Read-only on `/mnt/*` and `references/*`. State which date/stream pair you tested against.

## Acceptance

Full-depth book-vs-snapshot comparison, zero divergences, across a full real session, for at least CRUDEOIL and NATURALGAS. Report the number of snapshot cycles checked.

## Done when

- [ ] `MboBook` built from `decoder`'s message stream, dense tick-indexed, tolerates crossed state
- [ ] Book state machine implemented (`Uninit`/`Ok` minimum)
- [ ] Snapshot-cycle comparison harness run against a real session, zero divergences (or a precise, investigated account of what diverged)
- [ ] `book_user_doc.md` written — same depth as `decoder`'s: how it works, which files it reads, what the snapshot validation actually checks and why it's the real gate

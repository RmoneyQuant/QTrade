# Cache — component documentation

**What this component does, in one sentence:** the strategy-declared instrument filter that runs immediately after `decoder` (FR-B16), and the shared read model every strategy sees (FR-B17).

**Dispatch (FR-B18/D25) moved out (2026-08-25).** It used to be a third piece bundled in here (§4/§5 below describe that era and its real, measured numbers — kept as history, not deleted, since the measurements are still true statements about that code). A dedicated design session concluded market-data dispatch was never really this component's job, just built here first because there was nowhere else for it to live — D33 ("two dispatchers, because they are two different lookups") makes the case explicitly. It's now `event_dispatcher::EventDispatcher`, generalized with a real strategy-facing trait (`MarketHandler`) and a second callback (`on_trade`). See `event_dispatcher/event_dispatcher_user_doc.md`. `Cache` no longer knows dispatch exists at all — the same independence `ExecutionEngine`/`Portfolio` already have.

Code: [`cache.rs`](cache.rs) (this folder). Verified standalone the same way `book` was: a second `[[bin]]` target, `cache-validate` (`validate.rs`, this folder) — see §5.

---

## 1. What it builds

Two pieces now (was three — see the Dispatch note above), matching BACKTEST-PHASE1.md §M5 minus the part that moved:

```rust
pub struct InstrumentFilter { /* resolved native-SecurityID set */ }
impl InstrumentFilter {
    pub fn from_predicate(refdata: &InstrumentMaster, predicate: impl FnMut(&Instrument) -> bool) -> Self;
    pub fn from_native_ids(native_ids: impl IntoIterator<Item = i64>) -> Self;
    pub fn passes(&self, security_id: i64) -> bool; // FR-B16's "one comparison"
    pub fn instrument_ids(&self) -> Vec<InstrumentId>;
}

pub struct Cache { /* filter, BookBuilder, InstrumentMaster, own-orders stub */ }
impl Cache {
    pub fn new(refdata: InstrumentMaster, filter: InstrumentFilter) -> Self;
    pub fn apply(&mut self, event: &DecodedMessage) -> Option<InstrumentId>;   // filter -> book
    pub fn seed_book_band(&mut self, instrument: InstrumentId, band_min_raw: i64, band_max_raw: i64); // see book_user_doc.md
    pub fn book(&self, instrument: InstrumentId) -> Option<&dyn Book>;        // full access, always
    pub fn book_state(&self, instrument: InstrumentId) -> Option<BookState>;
    pub fn refdata(&self) -> &InstrumentMaster;
    pub fn own_orders(&self) -> &OwnOrdersAndPositions;                       // stub, read-only
    pub fn filter(&self) -> &InstrumentFilter;
}
```

`Cache::apply` is the one call a caller makes per decoded message: filter, then (if it passes) book work. It returns the touched `InstrumentId` so a caller (`main.rs`, today) can decide whether to also drive `event_dispatcher::EventDispatcher::on_book_touched` — `Cache` itself no longer combines the two steps (there used to be an `on_message` that did; it went with `Dispatcher`).

---

## 2. Filter — FR-B16 / D32, and the roll trap

**The predicate is strategy-declared, resolved once against the day's `refdata::InstrumentMaster`, into a flat `HashSet<i64>` of native `SecurityID`s.** After that, every event costs exactly one hash-set membership test (`InstrumentFilter::passes`) — the predicate closure itself is never re-evaluated per event. This is what FR-B16 means by "an event for an unfiltered instrument costs one comparison": the expensive part (walking `Instrument` records, matching on `kind`/`underlying`) happens once at `on_start`, not on the hot path.

**The roll trap, concretely.** D32's own words: a filter keyed to today's front-month instrument ids works right up until the strategy rolls into next month's contract — at which point a mid-run subscription finds an empty book, because nothing in the filtered set ever admitted that contract's events, so no book was ever built for it. The fix D32 prescribes is to declare the predicate over **symbol/underlying**, not a list of ids resolved once for today only.

Checked against a real row from `/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp` (the same file `refdata` parses):

```
7305,1768605378,3,0,294,467013,CRUDEOIL     ,XX ,...
7305,1768605378,4,0,401,465849,NATURALGAS   ,XX ,...
```

`refdata.rs`'s own `Symbol` extraction (`parts[6]`, whitespace-stripped) turns these into the exact strings `"CRUDEOIL"` and `"NATURALGAS"` — and, separately confirmed over the whole file, `"CRUDEOILM"` (the mini contract) is a **distinct** string, never equal to `"CRUDEOIL"`. So the predicate this component actually uses:

```rust
InstrumentFilter::from_predicate(&master, |i| match &i.kind {
    InstrumentKind::Future { underlying, .. } => underlying == "CRUDEOIL" || underlying == "NATURALGAS",
    _ => false,
})
```

admits **every expiry** of pure CRUDEOIL and pure NATURALGAS futures the day's master lists — including whichever contract the strategy hasn't rolled into yet — while still excluding the mini contract and every unrelated MCX product (SILVER, GOLD, COPPER, ...) by construction, with no expiry cutoff to keep re-deriving. `Cache::new` then constructs `BookBuilder` over the **whole** resolved set up front (D32's amendment to D10), not just what's quoted today, so by the time a strategy's roll logic actually subscribes to next month's contract, that book already has a full session's history.

`cache.rs`'s own unit test `filter_covers_a_contract_not_yet_rolled_into` proves this mechanically: two CRUDEOIL expiries exist in a synthetic master, the predicate admits both, events for the *second* (not-yet-subscribed) contract are applied for hours before anything ever calls `Cache::subscribe` on it, and `Cache::book` for that contract already shows the full resting-order history the moment it's finally looked at — never an empty book.

### 2.1 A real limitation this surfaced — why the acceptance run's filter is narrower than the mechanism above

Running the broad predicate above against the real `19_01_2026` master resolves to **12** native `SecurityID`s (several CRUDEOIL/NATURALGAS expiries, not just the two hand-picked ones `book` validates). Feeding all 12 into `BookBuilder::new` was tried, and it panics on real data:

```
book[InstrumentId(475111)]: price 23760000000 is outside the configured band
[0, 100000000000000] or off the 100000000-wide tick grid -- band needs widening
```

`book.rs`'s own `band_config` (private, not part of this task's edit scope) only has a real, hand-tuned tick/band entry for `CRUDEOIL_ID` (467013) and `NATURALGAS_ID` (465849) — every other instrument id falls through to a generic fallback band with a flat 1-rupee tick. Native id 475111 is a real different NATURALGAS-family contract that trades at sub-rupee (paise-level) prices; those don't land on the fallback's 1-rupee tick grid, and `MboBookImpl::idx_of` panics rather than silently misplacing the order — by design (its own doc comment: this is exactly the "silent wrong book" failure mode the assert exists to catch, not a bug to route around).

Widening `band_config` to cover every real MCX instrument is `book`'s job, not `cache`'s, and `book.rs` is outside this task's edit scope (types/decoder/book/refdata/scheduler are all off-limits per the brief). So this component's real acceptance run (§5) constructs `Cache` with a filter **narrowed to the two instruments `book` actually has validated bands for** —

```rust
InstrumentFilter::from_predicate(&master, |i| i.native_id == 467_013 || i.native_id == 465_849)
```

— still resolved against the real day's master (so it's still a real D32-style predicate, not a hardcoded id list baked in at compile time), while the broader 12-id predicate is computed and reported alongside it purely to make the contrast, and the finding, visible. **This is a real, current limitation of `book`'s instrument coverage, discovered by actually running the broader filter against real data — not a flaw in the filter mechanism itself**, which is proven correct and general by the unit test in §2 above using a synthetic contract with a safe band.

**Update, a later task: this limitation is now resolved in `book` itself.** `band_config` (the private per-id hardcoded match this section describes) is gone; `book::BookBuilder` now derives tick size generically from `refdata::Instrument.tick_size` (correct for every instrument, including 475111) and learns its price band from a real `InstrumentInfo` (13603) message in the applied stream — see `book_user_doc.md`'s "generic price band and tick size" section, which also proves this generalizes to a third, previously-unsupported instrument (ALUMINIUM) end to end. `Cache::new` was touched minimally to source each filtered instrument's tick size from `refdata` (it used to just pass bare `InstrumentId`s to `BookBuilder::new`, which took the band from `band_config` on its own); a new pass-through, `Cache::seed_book_band`, exists for a caller whose own feed can't supply a real `InstrumentInfo` in time (this component's own `cache-validate`/`dummy-strategy` binaries, both increment-file-only consumers, use it for exactly that real reason — see their own comments at the `Cache::new` call site). Callers that already resolve their filter against `refdata` (as this section's own predicate does) need no other change to build a book over an instrument `book` previously couldn't.

---

## 3. Cache — FR-B17

Holds, of what this task set has a real shape for:

| Holds | How |
|---|---|
| Books per filtered instrument | one shared `book::BookBuilder` instance (D06), constructed over the filter's resolved set |
| Book state | `Cache::book_state`, reads through to `MboBookImpl::state()` |
| Reference data for the trading day | `Cache::refdata()` → `&refdata::InstrumentMaster`, loaded once per session |
| Own orders / positions | `OwnOrdersAndPositions` — an intentionally empty stub; `execution` (T07) doesn't exist yet, and FR-B17's table describes contents (OMS state, sub-account P&L, firm aggregate) with no wire shape defined anywhere in this task set. Guessing at one before the writer exists would only have to be redone, so `Cache` just holds an inert placeholder and exposes it read-only. No mutation method exists yet, not even `pub(crate)` — T07 adds them |

**Two id spaces, on purpose, not by accident.** `book`'s `BookBuilder` (for this milestone, per its own doc comment) keys books by `InstrumentId(native_SecurityID as u32)` directly — `refdata`'s dense per-day interning (FR-B02) isn't wired into `book` yet. `Cache` follows the same convention rather than inventing a third scheme: `InstrumentFilter::instrument_ids()` hands `BookBuilder::new` native-token-derived ids, while `Cache::refdata().get(dense_id)` is the separate path for resolving a full `Instrument` record by refdata's own dense id when a caller actually needs a metadata field (tick size, lot size, expiry, ...). Two different jobs, two different keys — same shape `refdata_user_doc.md` already documents for its own callers.

**Read-only to strategies.** Every `Cache` accessor returns either a shared reference (`&dyn Book`, `&InstrumentMaster`, `&OwnOrdersAndPositions`) or a `Copy` value (`BookState`). The only method that mutates anything (`apply`) is what the filter → book pipeline itself calls — standing in for what `BookBuilder` (book work, already `book`'s job) and a future `ExecutionEngine` (order/position work, T07's job) actually write, per FR-B17's own division of labor.

---

## 4. Dispatch — FR-B18 / D25: waking vs access, concretely (historical — moved to `event_dispatcher`, 2026-08-25)

**This section describes `cache::Dispatcher`, which no longer exists in this file.** Kept as-is below because every number and claim in it is still a true statement about a real run of that code — it's the record of how `event_dispatcher::EventDispatcher` (its direct successor, same keying and snapshot-diffing logic, unchanged) was proven correct before the relocation. See `event_dispatcher/event_dispatcher_user_doc.md` for the current, real component and its own tests.

Subscriber lists keyed by **`(instrument, depth)`** — `Depth::Bbo` or `Depth::Top(n)`. What "waking, not access" means in this code, exactly:

- **Waking** — `Cache::dispatch(instrument)` (called after every book-mutating event that passes the filter) checks every depth this instrument has subscribers registered at, compares the current top-of-book (or top-`n`) against the *last observed* snapshot for that `(instrument, depth)` key, and calls `Subscriber::on_wake` only for the depths that actually changed value. An order added ten price levels deep touches the book but changes nothing about the top-of-book snapshot, so a BBO subscriber's `on_wake` is never called for it — proven by `cache.rs`'s own `bbo_subscriber_wakes_on_top_change_not_on_deeper_level` test.
- **Access** — `Cache::book(instrument)` returns `Option<&dyn Book>` unconditionally, with no reference to what (if anything) is subscribed. A strategy subscribed at BBO only — or not subscribed to that instrument at all — can call `cache.book(id).unwrap().depth(50)` any time and get the true current state at any depth. Subscription depth never gates this call; it only gates whether `on_wake` gets invoked automatically. Proven by `full_book_reachable_regardless_of_subscribed_depth`.

### 4.1 Allocation profile of the wake check (measured, not assumed)

The `Depth::Bbo` branch touches only `Option<PriceLevel>` values (the type is `Copy`) and pre-existing `HashMap` slots via `get_mut` — every key this run visits was inserted once, at `subscribe()` time (setup), so the hot-path `get_mut` calls never trigger a table resize. No heap operation occurs in this branch in steady state.

The `Depth::Top(n)` branch calls `book::Book::depth(n)`, which (per `book.rs`) builds a fresh `Vec<PriceLevel>` on every single call — the only depth-returning method `Book` exposes. A `Top(n)` subscription's wake-check therefore **does** allocate, once per touched instrument per event, regardless of whether the top-`n` slice actually changed. This is inherent to `book`'s current trait shape, not something introduced here, and this component doesn't touch `book.rs` to fix it. The acceptance run's own no-op strategy (§5) subscribes at `Depth::Bbo` only for exactly this reason — it exercises the branch this component can actually make allocation-free today.

---

## 5. The acceptance run

`validate.rs` (this folder) is the harness: a no-op strategy (`NoOpStrategy`, counts its own wakes, takes no other action — the acceptance bar's own words) subscribed at `Depth::Bbo` on both `book::CRUDEOIL_ID` and `book::NATURALGAS_ID`, run against the same two full real-session capture files `book`'s own FR-B11 harness validated against:

```
/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin   (CRUDEOIL, stream 4)
/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_5.bin   (NATURALGAS, stream 5)
```

streamed one outer record at a time off a `BufReader<File>` (never the whole file resident — `free -h` showed only ~800MB free before this run started, and the files are 6.8GB/30GB), same discipline `book/validate.rs`'s own `RecordSource` used. Reference data is the real `MCXScrips.bcp` for the same session date (`/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp`, 140 accepted instruments).

### 5.1 How to run it

```bash
source "$HOME/.cargo/env"
cd qtrade
cargo build --release --bin cache-validate
./target/release/cache-validate
```

`cargo test --release --bin cache-validate` runs this file's own `#[cfg(test)]` suite plus everything else compiled into the same binary (`book.rs`'s and `refdata.rs`'s own unit tests come along for free, same as `book-validate` does for `book`/`decoder`) — 23 tests, all passing.

`CACHE_DEBUG_STOP_AFTER_RECORDS=<n>` truncates each file's replay after `n` outer records, for a fast sanity pass instead of the full multi-hundred-million-record run.

### 5.2 A real allocation-counting method

A `#[global_allocator]` wraps `std::alloc::System`, counting every `alloc`/`alloc_zeroed`/`realloc` call and byte, process-wide, via `AtomicU64`s (`Ordering::Relaxed` — this is a single-threaded run, no ordering guarantee needed beyond "reads what was just written"). The harness snapshots this counter immediately before and after `Cache::apply` and, separately, immediately before and after `Cache::dispatch`, for every single decoded message, and accumulates the deltas into per-stage running totals. This is what "measured, not assumed" means here: the claim isn't "I didn't write a `Vec::new()` in the dispatch path", it's "the process-wide allocator's own counters recorded zero calls during every one of N dispatch invocations across a full real session."

### 5.3 Real numbers

Full, untruncated run against both real capture files, `cargo build --release`, single-threaded, on this machine (`free -h` before the run: ~800MB free, 32GB "available" via reclaimable page cache — comfortably enough headroom for streamed, not resident, 6.8GB/30GB files):

| | CRUDEOIL (stream 4) | NATURALGAS (stream 5) | Total |
|---|---|---|---|
| Outer records | 56,602,508 | 242,321,672 | 298,924,180 |
| Decoded messages | 114,423,913 | 488,770,778 | 603,194,691 |
| Wall time | 32.46s | 123.31s | 155.77s |
| Throughput | 1,743,567 records/s, 3,524,681 msgs/s | 1,965,171 records/s, 3,963,815 msgs/s | 1,918,987 records/s, 3,872,297 msgs/s |
| `apply()` allocations | count=988, bytes=207,808 | count=3,156, bytes=1,130,560 | count=4,144, bytes=1,338,368 |
| `dispatch()` allocations | count=0, bytes=0 | count=0, bytes=0 | **count=0, bytes=0** |

Dispatch stats over the combined run: **3,805,167 book touches**, **873,727 wakes fired** (CRUDEOIL subscriber: 225,469; NATURALGAS subscriber: 648,258).

The record/message counts match `book`'s own FR-B11 run exactly (56,602,508 / 242,321,672 outer records — see `book_user_doc.md` §7) — expected, since this is the same two files streamed the same way; it also cross-confirms this harness's own framing/decoding logic against an independently-validated component.

### 5.4 What the numbers mean

**Dispatch: zero allocations, confirmed over 603 million real messages, not assumed.** Every one of the 3,805,167 `Cache::dispatch` calls (one per book-mutating, filter-passing event) ran through the `Depth::Bbo` branch — `best_bid()`/`best_ask()` plus a `HashMap::get_mut` on a pre-populated key — and the global counting allocator recorded exactly zero `alloc`/`alloc_zeroed`/`realloc` calls across all of them. NFR-05's zero-allocation bar for the dispatch path is met, measured, on this component's own hot path.

**Waking vs touching, with a real ratio.** Only **873,727 of 3,805,167** book-mutating events (~23%) actually moved the subscribed BBO — the other ~77% touched some deeper level and correctly woke nobody. This is D25's design argument made concrete with real session data: a strategy subscribed at BBO was spared roughly four out of every five book-mutating events on these two contracts, on this session, without missing a single real top-of-book change (per the unit tests in §2/§6, which pin the exact wake/no-wake boundary).

**`apply()` (the book path) is *not* zero-allocation — 4,144 allocations over 603 million messages, and that's expected, not a bug in this component.** `book::MboBookImpl`'s `Level.orders` is a `VecDeque<OrderSlot>` that starts empty (`Default::default()`) for every one of the ~10,000+ tick-indexed price levels each side's dense array holds; the first order that ever rests at a given price level, and periodic capacity growth as a busy level accumulates more resting orders than its `VecDeque`'s current buffer holds, are real heap operations. 4,144 allocations against 603 million messages is roughly one allocation per 145,600 messages — heavily front-loaded at each level's first use and at each doubling-growth point, not a per-event cost; `dispatch()`'s own zero count on the exact same run confirms these allocations originate in `apply()` (i.e. inside `book`'s own code), not in anything this component adds. `book.rs`'s own scope note says performance tuning "beyond obviously wasteful" is explicitly this milestone's job, not `book`'s — but `book.rs` is outside this task's edit scope, so this is reported as a real, measured, traced-to-cause finding rather than silently patched or silently ignored.

**Throughput.** ~1.9M outer records/s, ~3.9M decoded messages/s sustained across a combined 156-second run over ~37GB of real streamed data — dominated by file I/O and per-message decode/book-apply/dispatch work together, not an isolated microbenchmark. Both files' throughput (1.74M and 1.97M records/s) are consistent with each other, suggesting the pipeline is not accidentally quadratic or bottlenecked by one file's larger scale.

---

## 6. Unit tests

`cache.rs`'s own `#[cfg(test)] mod tests` (run via `cargo test --release --bin cache-validate`, the only binary target that currently compiles `cache.rs` — see §5.1) covers: the filter admitting a matching underlying and rejecting an unrelated one (including the mini-contract near-miss `"CRUDEOILM"` vs `"CRUDEOIL"`), the roll-trap scenario end to end (a not-yet-subscribed contract's book already has full history), and an unfiltered instrument never reaching `BookBuilder` at all. **The dispatch tests (BBO waking on a real change but not a deeper-level-only one, a moving-ask `OrderModify` waking exactly once, `DispatchStats`/now `EventDispatcherStats` counting touches and wakes separately) moved to `event_dispatcher::tests` (2026-08-25)** — same assertions, same synthetic data, now constructing `EventDispatcher` directly instead of going through `Cache`. All pass.

---

## 7. Scope notes

- The `Strategy` trait itself (D24: `on_start`/`on_book`/`on_trade`/...) has real, if still thin, pieces now — `event_dispatcher::MarketHandler`, `strategy::Ctx`/`StartCtx` — built 2026-08-25. `ExecutionEngine` delivering fills/order-updates live (`ControlHandler`, Phase B) is still later work.
- No read or write path for the Simulated Exchange (T06) — D32/FR-B19 are explicit that it has no read path into `Cache` at all; nothing here assumes otherwise.
- `OwnOrdersAndPositions` carries no fields and no mutation methods. Real content and a real writer arrive with `execution` (T07).
- Confirmed: nothing under `/mnt/` or `references/` was modified by this task. `book.rs`, `decoder.rs`, `refdata.rs`, `types.rs`, `scheduler.rs`, and `main.rs` were read but not edited — everything for this component lives in `qtrade/src/cache/`.

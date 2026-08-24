//! The cache component: three things bundled under one milestone, kept
//! as one component here for the same reason BACKTEST-PHASE1.md §M5
//! bundles them --
//!
//! - **Filter** (FR-B16 / D32): a strategy-declared predicate over the
//!   native `SecurityID`, resolved once against the day's `refdata`
//!   master into a flat set, and checked immediately after `decoder`
//!   produces each event -- before anything reaches `book`.
//! - **Cache** (FR-B17): the shared read model -- books per filtered
//!   instrument (via `book::BookBuilder`, reused, not reinvented), book
//!   state, the day's reference data, and a stub slot for a strategy's
//!   own orders/positions that `execution` (T07, not built yet) will
//!   write into later.
//! - **Dispatch** (FR-B18 / D25): subscriber lists keyed by
//!   `(instrument, depth)`. A strategy subscribed at BBO wakes only when
//!   the best bid/ask actually changes value, never on a deeper-level-
//!   only change -- but full book access is never gated by subscription
//!   depth; `Cache::book` reaches any depth on demand regardless of what
//!   a strategy is "subscribed" at. Subscription governs *waking*, not
//!   *access*.
//!
//! See `cache_user_doc.md` in this folder for the full account: why the
//! filter predicate is symbol/underlying-based rather than a fixed
//! instrument-id list (the "roll trap", D32), what "waking vs access"
//! means concretely, and the real throughput/allocation numbers from a
//! full real-session acceptance run.
//!
//! ## Convention carried over from `decoder`/`book`/`scheduler`
//!
//! `Debug` derived everywhere, no exceptions. `Display` is not written
//! here for the same reason `book` skips it -- nothing in this module is
//! meant to be read as a sentence by a person.
//!
//! ## Scope (deliberately out, this milestone)
//!
//! The `Strategy` trait itself (later work -- `Subscriber` below is a
//! deliberately thin stand-in, just enough to prove the dispatch shape
//! works end to end). Simulated Exchange (T06, separate component,
//! explicitly has no read path into this cache per D32/FR-B19). Real
//! writes into the own-orders/positions stub (T07, `execution`, doesn't
//! exist yet) -- `cache` only holds the slot.

use crate::book::{Book, BookBuilder};
use crate::decoder::DecodedMessage;
use crate::refdata::InstrumentMaster;
use crate::types::{BookState, Instrument, InstrumentId, PriceLevel};
use std::collections::HashMap;
use std::collections::HashSet;

// ---------------------------------------------------------------------
// Filter -- FR-B16 / D32.
// ---------------------------------------------------------------------

/// A strategy-declared predicate over the day's `Instrument` metadata,
/// resolved **once** (at construction, standing in for a strategy's
/// `on_start`) into a flat `HashSet<i64>` of native `SecurityID`s. Every
/// subsequent check (`passes`) is a single hash-set membership test --
/// this is the "one comparison" FR-B16 asks for; the predicate closure
/// itself is never called again after construction.
///
/// **The roll trap (D32), and how this avoids it:** a filter keyed to
/// *today's front-month instrument ids* would work perfectly until the
/// strategy rolls into next month's contract -- at which point a
/// mid-run subscription finds an empty book, because nothing in the
/// filtered set ever admitted that contract's events, so `book` never
/// built a history for it. The fix is what D32 itself prescribes:
/// declare the predicate over **symbol/underlying**, not over a fixed
/// list of ids resolved once for today only. `Instrument.kind`'s
/// `Future { underlying, .. }` field is exactly a symbol string (e.g.
/// `"CRUDEOIL"`, distinct from the mini contract `"CRUDEOILM"` --
/// checked against a real MCXScrips.bcp row) that names the *product*,
/// independent of which expiry is currently front month. A predicate
/// like `underlying == "CRUDEOIL"` (no expiry restriction at all) admits
/// every CRUDEOIL future the day's master lists -- including the one the
/// strategy hasn't rolled into yet -- so `BookBuilder` is constructed
/// over the whole filtered set up front (this module's own `Cache::new`
/// does exactly that) and every one of those books already has full
/// intraday history by the time a strategy's roll logic subscribes to
/// it. See `cache_user_doc.md` for the concrete predicate this
/// component's acceptance run uses and why "all expiries of the
/// underlying" was chosen over a narrower "front two expiries" that
/// would also technically satisfy D32 but needs re-deriving `n` well
/// enough in advance never to fall short.
pub struct InstrumentFilter {
    native_ids: HashSet<i64>,
}

impl InstrumentFilter {
    /// Resolves `predicate` against every instrument in `refdata` once,
    /// keeping only the native `SecurityID`s (not the dense per-day
    /// `InstrumentId`s refdata assigns -- see the note on `Cache::new`
    /// for why this component works in native-token space, matching
    /// `book`'s own stopgap convention for this milestone).
    pub fn from_predicate(refdata: &InstrumentMaster, mut predicate: impl FnMut(&Instrument) -> bool) -> Self {
        let native_ids = refdata
            .all()
            .iter()
            .filter(|i| predicate(i))
            .map(|i| i.native_id)
            .collect();
        InstrumentFilter { native_ids }
    }

    /// Build directly from an explicit native-id set -- used by tests and
    /// by any caller that already has the resolved set (e.g. from a
    /// config file) rather than a live `InstrumentMaster` to resolve
    /// against.
    pub fn from_native_ids(native_ids: impl IntoIterator<Item = i64>) -> Self {
        InstrumentFilter {
            native_ids: native_ids.into_iter().collect(),
        }
    }

    /// FR-B16's "one comparison": a hash-set membership test, no
    /// allocation, no predicate re-evaluation.
    #[inline]
    pub fn passes(&self, security_id: i64) -> bool {
        self.native_ids.contains(&security_id)
    }

    /// The resolved set as `InstrumentId`s in `book`'s own convention for
    /// this milestone (native token cast directly -- see `book.rs`'s
    /// `CRUDEOIL_ID`/`NATURALGAS_ID` doc comment). Used to construct the
    /// `BookBuilder` over the *whole* filtered set (D32's amendment to
    /// D10), not just instruments actually quoted today.
    pub fn instrument_ids(&self) -> Vec<InstrumentId> {
        self.native_ids.iter().map(|&t| InstrumentId(t as u32)).collect()
    }

    pub fn len(&self) -> usize {
        self.native_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.native_ids.is_empty()
    }
}

/// This instrument's native `SecurityID` for the message types that
/// carry one -- `None` for framing/informational types (`PacketHeader`,
/// `Heartbeat`, `ExecutionSummary`, `TopOfBook`, the `Snapshot*`
/// variants, `Unknown`), which are not instrument-filterable at all
/// (they either aren't book-mutating or don't carry a routable token in
/// the shape this milestone needs). Deliberately a separate copy of the
/// same match `book.rs`'s own (private) `security_id_of` performs --
/// that one isn't `pub`, and this module doesn't touch `book.rs`.
fn security_id_of(event: &DecodedMessage) -> Option<i64> {
    use DecodedMessage as D;
    match event {
        D::OrderAdd(o) => Some(o.security_id),
        D::OrderModify(o) => Some(o.security_id),
        D::OrderModifySamePriority(o) => Some(o.security_id),
        D::OrderDelete(o) => Some(o.security_id),
        D::OrderMassDelete(o) => Some(o.security_id),
        D::Trade(t) => Some(t.security_id),
        _ => None,
    }
}

// ---------------------------------------------------------------------
// Dispatch -- FR-B18 / D25. Depth-scoped subscriber lists; waking, not
// access.
// ---------------------------------------------------------------------

/// What a subscription wakes on. `Bbo` is the zero-allocation path this
/// component's acceptance run actually exercises (`best_bid`/`best_ask`
/// return `Option<PriceLevel>` by value, no heap involved). `Top(n)`
/// wakes on a change anywhere in the best `n` levels each side, matching
/// `book::Book::depth(n)`'s own shape -- but note `depth(n)` builds a
/// fresh `Vec` every call (see `book.rs`), so a `Top(n)` subscription's
/// wake-check is **not** allocation-free; that cost is inherent to the
/// only depth-returning method `Book` exposes, not something this
/// component introduces. See `cache_user_doc.md` §"waking vs access" and
/// its measured allocation numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Depth {
    Bbo,
    Top(u8),
}

pub type SubscriberId = usize;

/// The strategy-facing wake callback. Deliberately thin -- the real
/// `Strategy` trait (D24: `on_start`/`on_book`/`on_trade`/... ) is later
/// work, out of scope here (see this file's header). This is just enough
/// surface for a no-op strategy to prove the dispatch shape end to end,
/// and realistic enough for a scheduler-driven loop to call into (a
/// vtable call through `Box<dyn Subscriber>`, same cost shape D24
/// documents for `dyn Strategy`).
pub trait Subscriber {
    fn on_wake(&mut self, instrument: InstrumentId, depth: Depth);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BboSnapshot {
    bid: Option<PriceLevel>,
    ask: Option<PriceLevel>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DispatchStats {
    /// How many times `on_book_touched` was called (one per book-
    /// mutating, filter-passing event actually applied to a book).
    pub book_touches: u64,
    /// How many individual subscriber wake calls actually fired (a
    /// no-change touch fires zero; one BBO change wakes every BBO
    /// subscriber on that instrument).
    pub wakes_fired: u64,
}

/// Subscriber lists keyed by `(instrument, depth)` (FR-B18, verbatim).
/// Owns the "last observed snapshot" per key needed to detect a real
/// value change, so a wake only fires when the subscribed-to slice of
/// the book actually changed -- not on every event that merely touched
/// that instrument's book at all (an order added ten levels deep must
/// not wake a BBO subscriber; D25).
pub struct Dispatcher {
    subs_by_key: HashMap<(InstrumentId, Depth), Vec<SubscriberId>>,
    by_instrument: HashMap<InstrumentId, Vec<Depth>>,
    subscribers: Vec<Box<dyn Subscriber>>,
    bbo_snapshots: HashMap<(InstrumentId, Depth), BboSnapshot>,
    depth_snapshots: HashMap<(InstrumentId, Depth), Vec<PriceLevel>>,
    pub stats: DispatchStats,
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            subs_by_key: HashMap::new(),
            by_instrument: HashMap::new(),
            subscribers: Vec::new(),
            bbo_snapshots: HashMap::new(),
            depth_snapshots: HashMap::new(),
            stats: DispatchStats::default(),
        }
    }

    /// Registers a subscriber at `(instrument, depth)`. This is setup
    /// work (a strategy's `on_start`, or a later dynamic subscribe/
    /// unsubscribe per D15) -- allocation here is expected and fine;
    /// NFR-05 targets the steady-state per-event hot path
    /// (`on_book_touched`), not one-time registration. Pre-populates this
    /// key's snapshot slot too, so the hot path's `get_mut` below always
    /// finds an existing entry rather than inserting one on first touch.
    pub fn subscribe(&mut self, instrument: InstrumentId, depth: Depth, subscriber: Box<dyn Subscriber>) -> SubscriberId {
        let id = self.subscribers.len();
        self.subscribers.push(subscriber);
        self.subs_by_key.entry((instrument, depth)).or_default().push(id);
        self.by_instrument.entry(instrument).or_default().push(depth);
        match depth {
            Depth::Bbo => {
                self.bbo_snapshots.entry((instrument, depth)).or_default();
            }
            Depth::Top(_) => {
                self.depth_snapshots.entry((instrument, depth)).or_default();
            }
        }
        id
    }

    /// Call once per event that actually touched `instrument`'s book
    /// (i.e. after `BookBuilder::apply`). Checks every depth this
    /// instrument has subscribers at; wakes only the ones whose
    /// subscribed slice of the book actually changed value.
    ///
    /// **Allocation profile (measured, see cache_user_doc.md):** the
    /// `Depth::Bbo` branch below touches only `Option<PriceLevel>`
    /// (`Copy`) values and pre-existing `HashMap` slots via `get_mut` --
    /// no allocation, in steady state, once `subscribe` has pre-
    /// populated every key this run ever visits. The `Depth::Top(n)`
    /// branch calls `book.depth(n)`, which builds a fresh `Vec` every
    /// call (inherent to `Book`'s only depth-returning method, not
    /// introduced here) -- not allocation-free. This run's own no-op
    /// strategy subscribes at `Depth::Bbo` only, so the acceptance run's
    /// hot path is the zero-allocation branch.
    pub fn on_book_touched(&mut self, book: &dyn Book, instrument: InstrumentId) {
        self.stats.book_touches += 1;
        let Some(depths) = self.by_instrument.get(&instrument) else {
            return;
        };
        for &depth in depths {
            let changed = match depth {
                Depth::Bbo => {
                    let bid = book.best_bid();
                    let ask = book.best_ask();
                    match self.bbo_snapshots.get_mut(&(instrument, depth)) {
                        Some(prev) if prev.bid != bid || prev.ask != ask => {
                            prev.bid = bid;
                            prev.ask = ask;
                            true
                        }
                        _ => false,
                    }
                }
                Depth::Top(n) => {
                    let cur = book.depth(n as usize); // allocates -- see doc comment above
                    match self.depth_snapshots.get_mut(&(instrument, depth)) {
                        Some(prev) if *prev != cur => {
                            *prev = cur;
                            true
                        }
                        _ => false,
                    }
                }
            };
            if changed {
                if let Some(ids) = self.subs_by_key.get(&(instrument, depth)) {
                    self.stats.wakes_fired += ids.len() as u64;
                    for &id in ids {
                        self.subscribers[id].on_wake(instrument, depth);
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// Own orders/positions -- FR-B17's stub slot.
// ---------------------------------------------------------------------

/// Deliberately empty: `execution` (T07) doesn't exist yet, and FR-B17's
/// own table lists "own orders (OMS state)", "sub-account positions and
/// P&L" and "firm aggregate (netted)" without a wire shape this task set
/// has defined. `Cache` just holds the slot -- guessing at a shape
/// before the component that writes it exists would only have to be
/// redone. No mutation methods are exposed even `pub(crate)`; T07 adds
/// them when it lands. Read-only to strategies via `Cache::own_orders`.
#[derive(Debug, Default)]
pub struct OwnOrdersAndPositions;

// ---------------------------------------------------------------------
// Cache -- FR-B17. The shared read model.
// ---------------------------------------------------------------------

/// Holds everything FR-B17 lists that this task set has a real shape
/// for: books per filtered instrument (one shared `BookBuilder`
/// instance, D06), book state (reachable per instrument through it), the
/// day's reference data (`refdata::InstrumentMaster`), and the
/// own-orders/positions stub. Read-only to strategies -- every accessor
/// below returns a shared reference or a `Copy` value; the only mutating
/// entry points (`apply`/`dispatch`/`on_message`) are what the
/// filter -> book -> dispatch pipeline itself calls, standing in for
/// what `BookBuilder` (book work) and a future `ExecutionEngine` (order/
/// position work) are the real writers of, per FR-B17.
///
/// **Native-token id space, not refdata's dense per-day ids:** `book`'s
/// `BookBuilder` (this milestone, per its own doc comment) keys books by
/// `InstrumentId(native_SecurityID as u32)` directly -- `refdata`'s
/// interning (a dense counter assigned at load time, FR-B02) isn't wired
/// into `book` yet. `Cache` follows the same convention here rather than
/// introducing a third id scheme: `InstrumentFilter::instrument_ids`
/// hands `BookBuilder::new` native-token-derived ids, and callers
/// resolve metadata by going through `Cache::refdata().get(...)` with
/// refdata's own dense id when they need a full `Instrument` record --
/// two different id spaces for two different jobs, same as today.
pub struct Cache {
    filter: InstrumentFilter,
    books: BookBuilder,
    refdata: InstrumentMaster,
    own_orders: OwnOrdersAndPositions,
    dispatch: Dispatcher,
}

impl Cache {
    /// Constructs the `BookBuilder` over the **whole** filtered set
    /// (D32's amendment to D10) -- every instrument `filter` admits gets
    /// a book from the first event onward, not just the ones a strategy
    /// happens to subscribe to today. This is what actually defeats the
    /// roll trap: by the time a strategy rolls and subscribes to next
    /// month's contract, that contract's book already has a full day's
    /// history, because it was being built all along.
    ///
    /// **Tick size, now real and generic** (`book`'s own price-band/
    /// tick-size mechanism, see book_user_doc.md): each filtered
    /// instrument's tick size comes straight from `refdata`'s own
    /// `Instrument.tick_size` (correct for every instrument, not just
    /// CRUDEOIL/NATURALGAS -- see refdata_user_doc.md), not a hardcoded
    /// per-id match inside `book` any more. The price *band* is
    /// deliberately not resolved here -- `book::BookBuilder` learns it
    /// from a real `InstrumentInfo` (13603) in the applied message
    /// stream, or a caller seeds it explicitly via `Cache::seed_book_band`
    /// when its own feed can't supply one in time (see that method's doc
    /// comment for the real, verified case this exists for).
    pub fn new(refdata: InstrumentMaster, filter: InstrumentFilter) -> Self {
        let instruments: Vec<(InstrumentId, i64)> = filter
            .instrument_ids()
            .into_iter()
            .map(|id| {
                let tick_raw = refdata
                    .get(id)
                    .unwrap_or_else(|| panic!("cache: filter admitted instrument {id:?} with no matching refdata record -- can't size its book without a real tick size"))
                    .tick_size
                    .0;
                (id, tick_raw)
            })
            .collect();
        let books = BookBuilder::new(&instruments);
        Cache {
            filter,
            books,
            refdata,
            own_orders: OwnOrdersAndPositions::default(),
            dispatch: Dispatcher::new(),
        }
    }

    /// Forwards to `book::BookBuilder::seed_band` for one instrument --
    /// see that method's doc comment for the real gap it closes: a
    /// caller whose own feed is increment-only (this crate's
    /// `cache-validate`/`dummy-strategy` binaries, both of which only
    /// ever read an `Increment_capture` file, never the paired
    /// `snapshot_capture` file that also carries `InstrumentInfo`) can't
    /// always learn a real instrument's band from `apply` alone --
    /// confirmed empirically for CRUDEOIL specifically (its real
    /// `19_01_2026` increment capture never carries a valid 13603 at all
    /// during the session). Those two binaries call this once per
    /// instrument, right after `Cache::new`, with real numbers sourced
    /// from the paired snapshot file's own 13603 stream -- not a guess.
    pub fn seed_book_band(&mut self, id: InstrumentId, band_min_raw: i64, band_max_raw: i64) {
        self.books.seed_band(id, band_min_raw, band_max_raw);
    }

    /// FR-B16: filter, then (if it passes) book work. Returns the
    /// instrument the event applied to, so a caller can decide whether
    /// to also run dispatch -- kept as a separate step from `dispatch`
    /// below so a profiler (or a scheduler that wants to batch several
    /// book-touches before waking anyone) can measure/call them
    /// independently. An event for an unfiltered instrument, or one that
    /// carries no routable `SecurityID` at all in this filtered event's
    /// case, costs one hash-set lookup (or one match arm) and nothing
    /// else -- no book work.
    pub fn apply(&mut self, event: &DecodedMessage) -> Option<InstrumentId> {
        let sid = security_id_of(event)?;
        if !self.filter.passes(sid) {
            return None;
        }
        self.books.apply(event);
        Some(InstrumentId(sid as u32))
    }

    /// FR-B18: run the depth-scoped wake check for `instrument`'s book.
    /// Separate from `apply` for the same profiling/batching reason.
    pub fn dispatch(&mut self, instrument: InstrumentId) {
        if let Some(book) = self.books.get(instrument) {
            self.dispatch.on_book_touched(book, instrument);
        }
    }

    /// The realistic single call a scheduler-driven loop makes per
    /// decoded message: filter -> book -> dispatch, in that order,
    /// exactly the pipeline this task brief names.
    pub fn on_message(&mut self, event: &DecodedMessage) {
        if let Some(instrument) = self.apply(event) {
            self.dispatch(instrument);
        }
    }

    pub fn subscribe(&mut self, instrument: InstrumentId, depth: Depth, subscriber: Box<dyn Subscriber>) -> SubscriberId {
        self.dispatch.subscribe(instrument, depth, subscriber)
    }

    /// Full book access, **not gated by subscription depth** -- D25's
    /// "waking, not access". A strategy subscribed at BBO (or not
    /// subscribed to this instrument at all) can still call this and
    /// read any depth on demand; subscription only controls whether it
    /// gets woken automatically.
    pub fn book(&self, instrument: InstrumentId) -> Option<&dyn Book> {
        self.books.get(instrument)
    }

    pub fn book_state(&self, instrument: InstrumentId) -> Option<BookState> {
        self.books.get(instrument).map(|b| b.state())
    }

    pub fn refdata(&self) -> &InstrumentMaster {
        &self.refdata
    }

    /// Read-only: see `OwnOrdersAndPositions`'s doc comment for why this
    /// is currently an inert placeholder.
    pub fn own_orders(&self) -> &OwnOrdersAndPositions {
        &self.own_orders
    }

    pub fn dispatch_stats(&self) -> DispatchStats {
        self.dispatch.stats
    }

    pub fn filter(&self) -> &InstrumentFilter {
        &self.filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{OrderAdd, OrderModify, Price as DPrice, Qty as DQty, Side as DSide};
    use crate::types::{Currency, Date, InstrumentKind, Price, Settlement, Venue, YearMonth};

    /// A generous synthetic band, `[0, 10_000]` rupees at a 1-rupee tick
    /// (raw units = rupees * `RUPEE_RAW`), covers every synthetic price
    /// used below. Since `book`'s price band is no longer hardcoded per
    /// instrument (see book_user_doc.md), every test that applies a real
    /// order-mutating event first calls `Cache::seed_book_band` with this
    /// band -- otherwise `book::BookBuilder::apply` panics loudly by
    /// design (the instrument's band would still be `Pending`; see
    /// `book.rs`'s doc comment on `BookBuilder::apply`).
    const RUPEE_RAW: i64 = 100_000_000;
    const TEST_BAND: (i64, i64) = (0, 10_000 * RUPEE_RAW);

    fn add(id: i64, side: DSide, price: i64, qty: i64, prio: u64) -> DecodedMessage {
        DecodedMessage::OrderAdd(OrderAdd {
            seq: 0,
            security_id: id,
            side,
            price: DPrice(price),
            qty: DQty(qty),
            priority_ts: prio,
            event_time: 0,
        })
    }

    fn future_instrument(native_id: i64, underlying: &str) -> Instrument {
        Instrument {
            id: InstrumentId(native_id as u32), // dense id irrelevant for these tests
            venue: Venue::Mcx,
            native_id,
            kind: InstrumentKind::Future {
                underlying: underlying.to_string(),
                expiry: Date(0),
                contract_month: YearMonth { year: 2026, month: 1 },
                settlement: Settlement::Cash,
            },
            tick_size: Price(RUPEE_RAW), // Rs 1.00 -- matches the RUPEE_RAW-scaled prices every test below uses
            lot_size: 1,
            multiplier: 1,
            freeze_qty: 0,
            price_band: None,
            currency: Currency::Inr,
        }
    }

    // ---- Filter / roll-trap ------------------------------------------------

    #[test]
    fn filter_admits_matching_underlying_rejects_others() {
        let master = InstrumentMaster::new(vec![
            future_instrument(467_013, "CRUDEOIL"),
            future_instrument(467_099, "CRUDEOILM"), // mini contract -- must NOT match
            future_instrument(465_849, "NATURALGAS"),
            future_instrument(999_999, "SILVER"),
        ]);
        let filter = InstrumentFilter::from_predicate(&master, |i| match &i.kind {
            InstrumentKind::Future { underlying, .. } => underlying == "CRUDEOIL",
            _ => false,
        });
        assert!(filter.passes(467_013));
        assert!(!filter.passes(467_099));
        assert!(!filter.passes(465_849));
        assert!(!filter.passes(999_999));
    }

    #[test]
    fn filter_covers_a_contract_not_yet_rolled_into() {
        // D32's roll trap, concretely: two CRUDEOIL expiries exist in the
        // day's master. A naive "today's front month only" filter would
        // admit only one of them; the underlying-based predicate admits
        // both up front, so the second contract's book has been building
        // all along by the time a strategy rolls into it.
        let master = InstrumentMaster::new(vec![
            future_instrument(467_013, "CRUDEOIL"), // front month
            future_instrument(467_020, "CRUDEOIL"), // next month -- not yet rolled into
        ]);
        let filter = InstrumentFilter::from_predicate(&master, |i| match &i.kind {
            InstrumentKind::Future { underlying, .. } => underlying == "CRUDEOIL",
            _ => false,
        });
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(InstrumentId(467_020), TEST_BAND.0, TEST_BAND.1);

        // The not-yet-rolled-into contract trades for hours before the
        // strategy ever subscribes to it...
        cache.apply(&add(467_020, DSide::Buy, 5_400 * 100_000_000, 10, 1));
        cache.apply(&add(467_020, DSide::Sell, 5_410 * 100_000_000, 5, 2));

        // ...and when the strategy finally does subscribe/look, the full
        // history (both resting orders) is already there -- not an empty
        // book.
        let next_month = InstrumentId(467_020);
        let book = cache.book(next_month).expect("book exists -- was built from the start, not on first subscribe");
        assert_eq!(book.best_bid().unwrap().qty, crate::types::Qty(10));
        assert_eq!(book.best_ask().unwrap().qty, crate::types::Qty(5));
    }

    #[test]
    fn unfiltered_instrument_never_touches_a_book() {
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let filter = InstrumentFilter::from_predicate(&master, |i| match &i.kind {
            InstrumentKind::Future { underlying, .. } => underlying == "CRUDEOIL",
            _ => false,
        });
        let mut cache = Cache::new(master, filter);
        assert!(cache.apply(&add(999_999, DSide::Buy, 100, 1, 1)).is_none());
        assert!(cache.book(InstrumentId(999_999)).is_none());
    }

    // ---- Dispatch: waking vs access ---------------------------------------

    struct CountingSubscriber {
        wakes: std::rc::Rc<std::cell::Cell<u32>>,
    }
    impl Subscriber for CountingSubscriber {
        fn on_wake(&mut self, _instrument: InstrumentId, _depth: Depth) {
            self.wakes.set(self.wakes.get() + 1);
        }
    }

    #[test]
    fn bbo_subscriber_wakes_on_top_change_not_on_deeper_level() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);

        let wakes = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let id = InstrumentId(467_013);
        cache.subscribe(id, Depth::Bbo, Box::new(CountingSubscriber { wakes: wakes.clone() }));

        // First bid ever -- best_bid changes from None to Some: wakes.
        cache.on_message(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1));
        assert_eq!(wakes.get(), 1);

        // A second order at a strictly worse (lower) bid price: best_bid
        // is untouched (still 5000 x 10) -- must NOT wake a BBO subscriber.
        cache.on_message(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 50, 2));
        assert_eq!(wakes.get(), 1, "a deeper-level-only change must not wake a BBO subscriber");

        // A new best bid: wakes again.
        cache.on_message(&add(467_013, DSide::Buy, 5_010 * RUPEE_RAW, 1, 3));
        assert_eq!(wakes.get(), 2);
    }

    #[test]
    fn full_book_reachable_regardless_of_subscribed_depth() {
        // D25: subscription governs waking, not access. Subscribe at
        // BBO only, then confirm depth-3 access still works on demand.
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let id = InstrumentId(467_013);
        cache.subscribe(id, Depth::Bbo, Box::new(CountingSubscriber { wakes: std::rc::Rc::new(std::cell::Cell::new(0)) }));

        cache.on_message(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1));
        cache.on_message(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 5, 2));
        cache.on_message(&add(467_013, DSide::Buy, 4_980 * RUPEE_RAW, 3, 3));

        let book = cache.book(id).unwrap();
        let depth = book.depth(10);
        // Three distinct bid levels reachable even though only a BBO
        // subscription exists -- access was never gated by it.
        assert!(depth.len() >= 3, "expected at least 3 levels reachable on demand, got {depth:?}");
    }

    #[test]
    fn modify_moving_bbo_wakes_exactly_once() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let wakes = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let id = InstrumentId(467_013);
        cache.subscribe(id, Depth::Bbo, Box::new(CountingSubscriber { wakes: wakes.clone() }));

        cache.on_message(&add(467_013, DSide::Sell, 5_200 * RUPEE_RAW, 10, 1));
        assert_eq!(wakes.get(), 1);
        cache.on_message(&DecodedMessage::OrderModify(OrderModify {
            seq: 0,
            security_id: 467_013,
            side: DSide::Sell,
            prev_price: DPrice(5_200 * RUPEE_RAW),
            prev_qty: DQty(10),
            price: DPrice(5_190 * RUPEE_RAW), // moves best_ask
            qty: DQty(10),
            prev_priority_ts: 1,
            priority_ts: 2,
            event_time: 0,
        }));
        assert_eq!(wakes.get(), 2);
    }

    #[test]
    fn dispatch_stats_count_touches_and_wakes_separately() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let id = InstrumentId(467_013);
        cache.subscribe(id, Depth::Bbo, Box::new(CountingSubscriber { wakes: std::rc::Rc::new(std::cell::Cell::new(0)) }));

        cache.on_message(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1)); // touch + wake (None->Some)
        cache.on_message(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 5, 2)); // touch, no wake (deeper level)
        let stats = cache.dispatch_stats();
        assert_eq!(stats.book_touches, 2);
        assert_eq!(stats.wakes_fired, 1);
    }
}

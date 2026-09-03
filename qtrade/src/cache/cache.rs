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
//!
//! **Dispatch (FR-B18/D25) used to be a third thing bundled in here --
//! it has moved to `event_dispatcher::EventDispatcher`.** Relocated,
//! generalized, and given a real strategy-facing trait
//! (`MarketHandler`) in a dedicated design session (see
//! `event_dispatcher_user_doc.md`): D33 ("two dispatchers, because they
//! are two different lookups") makes clear market-data dispatch was
//! never really part of *this* component's job, just built here first
//! because there was nowhere else for it to live yet. `Cache` no longer
//! knows dispatch exists at all -- same independence `ExecutionEngine`/
//! `Portfolio` already have ("accounting reacts to prices it's handed,
//! it does not go looking for them").
//!
//! See `cache_user_doc.md` in this folder for the full account: why the
//! filter predicate is symbol/underlying-based rather than a fixed
//! instrument-id list (the "roll trap", D32).
//!
//! ## Convention carried over from `decoder`/`book`/`scheduler`
//!
//! `Debug` derived everywhere, no exceptions. `Display` is not written
//! here for the same reason `book` skips it -- nothing in this module is
//! meant to be read as a sentence by a person.
//!
//! ## Scope (deliberately out, this milestone)
//!
//! The `Strategy` trait itself (later work -- see `strategy::Ctx`/
//! `StartCtx` and `event_dispatcher::MarketHandler` for how far that's
//! come). Simulated Exchange (T06, separate component, explicitly has
//! no read path into this cache per D32/FR-B19). Real writes into the
//! own-orders/positions stub (T07, `execution`, doesn't exist yet) --
//! `cache` only holds the slot.

use crate::book::{Book, BookBuilder};
use crate::decoder::DecodedMessage;
use crate::refdata::InstrumentMaster;
use crate::types::{BookState, Instrument, InstrumentId};
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
    /// to also drive `event_dispatcher::EventDispatcher::on_book_touched`
    /// -- kept as its own step (not a combined `on_message`) so `main.rs`
    /// can wire dispatch explicitly, rather than `Cache` doing it
    /// internally (see this file's header: `Cache` no longer knows
    /// dispatch exists at all). An event for an unfiltered instrument, or
    /// one that carries no routable `SecurityID` at all in this filtered
    /// event's case, costs one hash-set lookup (or one match arm) and
    /// nothing else -- no book work.
    pub fn apply(&mut self, event: &DecodedMessage) -> Option<InstrumentId> {
        let sid = security_id_of(event)?;
        if !self.filter.passes(sid) {
            return None;
        }
        self.books.apply(event);
        Some(InstrumentId(sid as u32))
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

    pub fn filter(&self) -> &InstrumentFilter {
        &self.filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{OrderAdd, Price as DPrice, Qty as DQty, Side as DSide};
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
            max_single_order_qty: 0,
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

    // Dispatch (waking vs access) tests moved to
    // `event_dispatcher::tests` -- see that module. `Cache` no longer
    // has anything to test on this front; `apply` alone (no dispatch
    // step) is covered by the filter/roll-trap tests above.
}

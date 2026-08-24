//! The book component: one incremental MBO order book per instrument,
//! built from `decoder`'s message stream, and the `BookBuilder` that
//! owns one such book per subscribed instrument (ARCHITECTURE.md §4.8,
//! BACKTEST-PHASE1.md §M3 FR-B08/FR-B09/FR-B10/FR-B11).
//!
//! See `book_user_doc.md` in this folder for the full account: how the
//! dense array is sized, why a crossed book is tolerated rather than
//! asserted against, and -- the actual point of this component -- how
//! the incrementally-built book was checked against every snapshot cycle
//! of a real recorded session with zero divergences (FR-B11).
//!
//! ## Convention carried over from `decoder`/`scheduler`
//!
//! `Debug` derived everywhere, no exceptions. `Display` is not written
//! here -- nothing in this module is meant to be read as a sentence by a
//! person; `PriceLevel`/`OrderHandle` (from `types`) are already
//! `Debug`-only for the same reason (see `types_user_doc.md`).
//!
//! ## Scope (deliberately out, this milestone)
//!
//! `Recovering`/`Stale` states (need a live Transport). Cache, Scheduler
//! wiring, dispatch, Simulated Exchange, execution. Performance tuning
//! beyond "don't do anything obviously wasteful" (NFR-05 is M5, not
//! here) -- `best_bid`/`best_ask`/`depth` scan the dense array linearly;
//! at a few thousand ticks per instrument this is not the wasteful thing
//! NFR-05 is about.

use crate::decoder;
use crate::types::{BookState, InstrumentId, OrderHandle, Price, PriceLevel, Qty, Side};
use std::collections::{HashMap, VecDeque};

// ---------------------------------------------------------------------
// Public traits -- FR-B08, shape given verbatim by BACKTEST-PHASE1.md
// and the task brief. `Book` and `MboBook` live in this module (not
// `types`) because they have behavior; `types` is data-only by design
// (see types_user_doc.md).
// ---------------------------------------------------------------------

pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    /// **Convention chosen here** (the trait shape in BACKTEST-PHASE1.md
    /// and the task brief gives `depth(n)` with no `side` parameter,
    /// unlike `qty_at_price`): the best `n` bid levels, best-to-worst,
    /// followed by the best `n` ask levels, best-to-worst. Callers that
    /// need one side specifically, or literally everything (used by this
    /// component's own FR-B11 validation harness), should use the
    /// inherent `MboBookImpl::full_depth` instead -- see
    /// book_user_doc.md.
    fn depth(&self, n: usize) -> Vec<PriceLevel>;
    fn qty_at_price(&self, side: Side, price: Price) -> Qty;
    fn state(&self) -> BookState;
}

pub trait MboBook: Book {
    /// Aggregate quantity resting **strictly ahead** of this order at its
    /// price (not an ordinal slot index) -- the actionable number for a
    /// market maker estimating time-to-fill. `None` if the handle
    /// doesn't identify a currently-resting order in this book (wrong
    /// instrument, wrong price, or already filled/cancelled).
    fn queue_position(&self, handle: OrderHandle) -> Option<i64>;
}

// ---------------------------------------------------------------------
// Dense-array book, one per instrument.
// ---------------------------------------------------------------------

/// One resting order's FIFO slot. Identified by `priority_ts`
/// (`TrdRegTSTimePriority` off the wire) -- MCX has no broadcast order
/// id (FR-B05), so this is the only thing that tells two orders resting
/// at the same price apart. `qty` is kept in the *raw* wire scale
/// (rupees * 10^8 for price, lots * 10^4 for qty -- `decoder`'s scaling,
/// never re-derived here), same as everything else in this module.
#[derive(Debug, Clone, Copy)]
struct OrderSlot {
    priority_ts: u64,
    qty: i64,
}

#[derive(Debug, Default)]
struct Level {
    qty: i64,
    orders: VecDeque<OrderSlot>,
}

/// One instrument's book: a dense array of `Level`s indexed by tick
/// offset from `band_min_raw`, one array per side. See
/// `book_user_doc.md` §"price band" for exactly which band/tick size was
/// used for which instrument and why.
pub struct MboBookImpl {
    instrument: InstrumentId,
    tick_raw: i64,
    band_min_raw: i64,
    n_ticks: usize,
    bid_levels: Vec<Level>,
    ask_levels: Vec<Level>,
    state: BookState,
    /// Diagnostic counters, not used by book logic itself: how many
    /// times `remove_order`/`modify_same_priority` were asked to find an
    /// order that wasn't in the FIFO, and how many times `apply_trade`
    /// was asked to consume a level with nothing resting on it. Both
    /// should be zero on a correctly-replayed book; a nonzero count
    /// means some earlier event was silently dropped or misrouted (see
    /// book_user_doc.md's account of diagnosing the FR-B11 harness).
    diag_remove_misses: u64,
    diag_trade_misses: u64,
}

impl MboBookImpl {
    pub fn new(instrument: InstrumentId, tick_raw: i64, band_min_raw: i64, band_max_raw: i64) -> Self {
        assert!(tick_raw > 0, "book[{instrument:?}]: tick_raw must be positive");
        assert!(
            band_max_raw > band_min_raw,
            "book[{instrument:?}]: band_max_raw must exceed band_min_raw"
        );
        let n_ticks = ((band_max_raw - band_min_raw) / tick_raw) as usize + 1;
        MboBookImpl {
            instrument,
            tick_raw,
            band_min_raw,
            n_ticks,
            bid_levels: (0..n_ticks).map(|_| Level::default()).collect(),
            ask_levels: (0..n_ticks).map(|_| Level::default()).collect(),
            state: BookState::Uninit,
            diag_remove_misses: 0,
            diag_trade_misses: 0,
        }
    }

    /// See the doc comment on the `diag_*` fields.
    pub fn diagnostics(&self) -> (u64, u64) {
        (self.diag_remove_misses, self.diag_trade_misses)
    }

    /// Strict index lookup used when *applying* a real event: a raw
    /// price that falls outside the configured band or off the tick
    /// grid means the band was sized wrong (a bug in this component's
    /// setup, not a normal data condition) -- panics loudly rather than
    /// silently dropping the event, which is exactly the "silent wrong
    /// book" failure mode this whole milestone exists to catch. See
    /// book_user_doc.md for why the chosen bands were verified never to
    /// trigger this across a full real session.
    fn idx_of(&self, price_raw: i64) -> usize {
        let offset = price_raw - self.band_min_raw;
        assert!(
            offset >= 0 && offset % self.tick_raw == 0,
            "book[{:?}]: price {} is outside the configured band [{}, {}] or off the {}-wide tick grid -- band needs widening, see book_user_doc.md",
            self.instrument,
            price_raw,
            self.band_min_raw,
            self.band_min_raw + self.tick_raw * (self.n_ticks as i64 - 1),
            self.tick_raw
        );
        let idx = (offset / self.tick_raw) as usize;
        assert!(
            idx < self.n_ticks,
            "book[{:?}]: price {} is beyond the configured band's upper edge -- band needs widening",
            self.instrument,
            price_raw
        );
        idx
    }

    /// Non-panicking counterpart for read queries (`qty_at_price`,
    /// `queue_position`): a price outside the band just has no
    /// liquidity, which is a legitimate answer, not a bug.
    fn idx_of_checked(&self, price_raw: i64) -> Option<usize> {
        let offset = price_raw - self.band_min_raw;
        if offset < 0 || offset % self.tick_raw != 0 {
            return None;
        }
        let idx = (offset / self.tick_raw) as usize;
        if idx >= self.n_ticks {
            return None;
        }
        Some(idx)
    }

    fn price_of(&self, idx: usize) -> i64 {
        self.band_min_raw + (idx as i64) * self.tick_raw
    }

    fn levels(&self, side: Side) -> &Vec<Level> {
        match side {
            Side::Buy => &self.bid_levels,
            Side::Sell => &self.ask_levels,
        }
    }

    fn levels_mut(&mut self, side: Side) -> &mut Vec<Level> {
        match side {
            Side::Buy => &mut self.bid_levels,
            Side::Sell => &mut self.ask_levels,
        }
    }

    fn add_order(&mut self, side: Side, price_raw: i64, priority_ts: u64, qty_raw: i64) {
        let idx = self.idx_of(price_raw);
        let lvl = &mut self.levels_mut(side)[idx];
        lvl.qty += qty_raw;
        lvl.orders.push_back(OrderSlot { priority_ts, qty: qty_raw });
        self.state = BookState::Ok;
    }

    /// Removes the order identified by `(side, price_raw, priority_ts)`.
    /// If not found (e.g. the order was added before this book's replay
    /// window started), this is a no-op, not a panic -- FR-B11's
    /// snapshot comparison is the mechanism that would surface the
    /// resulting divergence; this method staying quiet keeps a single
    /// bad event from taking down the whole book, matching FR-B09's
    /// "don't assert on a data anomaly" spirit.
    fn remove_order(&mut self, side: Side, price_raw: i64, priority_ts: u64) {
        let idx = self.idx_of(price_raw);
        let instrument = self.instrument;
        let lvl = &mut self.levels_mut(side)[idx];
        if let Some(pos) = lvl.orders.iter().position(|s| s.priority_ts == priority_ts) {
            let slot = lvl.orders.remove(pos).expect("position() just found it");
            lvl.qty -= slot.qty;
        } else {
            if std::env::var("BOOK_DEBUG_MISSES").is_ok() {
                eprintln!(
                    "[MISS] remove_order: book[{instrument:?}] side={side:?} price_raw={price_raw} priority_ts={priority_ts} not found (level has {} orders: {:?})",
                    lvl.orders.len(), lvl.orders
                );
            }
            self.diag_remove_misses += 1;
        }
        self.state = BookState::Ok;
    }

    /// `OrderModifySamePriority` (13106): quantity changes in place,
    /// FIFO position (and therefore priority) is untouched.
    fn modify_same_priority(&mut self, side: Side, price_raw: i64, priority_ts: u64, new_qty_raw: i64) {
        let idx = self.idx_of(price_raw);
        let instrument = self.instrument;
        let lvl = &mut self.levels_mut(side)[idx];
        if let Some(slot) = lvl.orders.iter_mut().find(|s| s.priority_ts == priority_ts) {
            lvl.qty += new_qty_raw - slot.qty;
            slot.qty = new_qty_raw;
        } else {
            if std::env::var("BOOK_DEBUG_MISSES").is_ok() {
                eprintln!(
                    "[MISS] modify_same_priority: book[{instrument:?}] side={side:?} price_raw={price_raw} priority_ts={priority_ts} not found (level has {} orders: {:?})",
                    lvl.orders.len(), lvl.orders
                );
            }
            self.diag_remove_misses += 1;
        }
        self.state = BookState::Ok;
    }

    /// `OrderMassDelete` (13103): every resting order for this
    /// instrument, both sides, gone at once -- confirmed against
    /// `references/MCX_Feeder.cpp`'s mass-delete handling, which clears
    /// the whole per-token book rather than one side.
    fn mass_delete(&mut self) {
        for lvl in self.bid_levels.iter_mut().chain(self.ask_levels.iter_mut()) {
            lvl.qty = 0;
            lvl.orders.clear();
        }
        self.state = BookState::Ok;
    }

    /// `Trade` (13104 full / 13105 partial). **Business-rule finding**
    /// (see the doc comment on `decoder::Trade`): the `Side` on a trade
    /// message identifies the *resting* order's side.
    ///
    /// **The real matching key, found only by root-causing the
    /// NATURALGAS FR-B11 divergences (see book_user_doc.md §5.7): a
    /// trade does not simply hit the FIFO front.** `decoder::Trade`'s
    /// `event_time` field (wire offset 24, named `TransactTime` in
    /// `references/MCX_Feeder.h`'s struct) does **not** carry a
    /// wall-clock time on a real `13104`/`13105` record -- its value is
    /// the *specific resting order's own* `priority_ts`
    /// (`TrdRegTSTimePriority`) that this trade actually matched,
    /// confirmed empirically: e.g. four consecutive real trades at one
    /// NATURALGAS price level carried qtys 10000/10000/10000/60000 (sum
    /// 90000) all stamped with the exact same `event_time`, which was
    /// exactly the `priority_ts` of the one order added moments earlier
    /// with qty 90000 -- and separately, a different order at the very
    /// same price/side was independently, correctly reduced by trades
    /// carrying *its own* distinct `event_time`, while sitting behind the
    /// first order in arrival order the whole time. A busy level can have
    /// many resting orders open at once, each targeted independently by
    /// its own stream of trades, not necessarily oldest-first.
    ///
    /// This component's original design (and its first fix, preserved
    /// below in the `book_user_doc.md` §5.5 account of finding the
    /// negative-quantity zombie) consumed blindly from the FIFO front.
    /// That coincidentally works as long as the front order happens to be
    /// the one actually targeted -- true often enough that CRUDEOIL's
    /// full-session validation never caught the gap -- but once the
    /// *targeted* order is fully consumed, blind-front logic starts
    /// eating into whichever *different* order now happens to sit at the
    /// front, corrupting an order the trade was never meant to touch:
    /// exactly the shape of every NATURALGAS miss and divergence traced
    /// (see §5.7).
    ///
    /// **Fix:** look the targeted order up directly by `matched_priority_ts`
    /// first. Consumption is still magnitude-based and cascades into
    /// *subsequent* FIFO slots (in arrival order, starting from the
    /// target) if the trade quantity exceeds what the target has left --
    /// unchanged in spirit from the original fix, just anchored at the
    /// order MCX's own data says was hit instead of assumed to be the
    /// front. If `matched_priority_ts` isn't resting in this book at all
    /// (e.g. a genuine race, or an order from before this replay's
    /// bootstrap window), fall back to the old FIFO-front policy rather
    /// than dropping the trade's effect on the book entirely -- strictly
    /// a fallback now, not the primary rule.
    fn apply_trade(&mut self, side: Side, price_raw: i64, matched_priority_ts: u64, mut trade_qty_raw: i64, _full: bool) {
        let idx = self.idx_of(price_raw);
        let instrument = self.instrument;
        let lvl = &mut self.levels_mut(side)[idx];
        let mut consumed_any = false;

        if let Some(start) = lvl.orders.iter().position(|s| s.priority_ts == matched_priority_ts) {
            let mut pos = start;
            while trade_qty_raw > 0 && pos < lvl.orders.len() {
                consumed_any = true;
                let slot_qty = lvl.orders[pos].qty;
                if slot_qty > trade_qty_raw {
                    lvl.orders[pos].qty -= trade_qty_raw;
                    lvl.qty -= trade_qty_raw;
                    trade_qty_raw = 0;
                } else {
                    trade_qty_raw -= slot_qty;
                    lvl.qty -= slot_qty;
                    lvl.orders.remove(pos); // shifts the next slot into `pos` -- don't advance
                }
            }
        } else {
            // Fallback only -- see doc comment above.
            while trade_qty_raw > 0 {
                let Some(front) = lvl.orders.front_mut() else { break };
                consumed_any = true;
                if front.qty > trade_qty_raw {
                    front.qty -= trade_qty_raw;
                    lvl.qty -= trade_qty_raw;
                    trade_qty_raw = 0;
                } else {
                    let front_qty = front.qty;
                    trade_qty_raw -= front_qty;
                    lvl.qty -= front_qty;
                    lvl.orders.pop_front();
                }
            }
        }
        if !consumed_any {
            if std::env::var("BOOK_DEBUG_MISSES").is_ok() {
                eprintln!(
                    "[MISS] apply_trade: book[{instrument:?}] side={side:?} price_raw={price_raw} matched_priority_ts={matched_priority_ts} trade_qty_raw={trade_qty_raw} -- level empty, nothing to consume (level has 0 orders)"
                );
            }
            self.diag_trade_misses += 1;
        }
        self.state = BookState::Ok;
    }

    /// Feeds one decoded increment-stream event into this book. No-ops
    /// on message types that don't mutate book state (`PacketHeader`,
    /// `Heartbeat`, `ExecutionSummary`, `TopOfBook`, the `Snapshot*`
    /// variants, `Unknown`) -- confirmed against
    /// `references/MCX_Feeder.cpp`, which likewise never touches its own
    /// order-book state for `13202`/`13504`/`13003`.
    pub fn apply(&mut self, event: &decoder::DecodedMessage) {
        use decoder::DecodedMessage as D;
        match event {
            D::OrderAdd(o) => {
                if let Some(side) = conv_side(o.side) {
                    self.add_order(side, o.price.0, o.priority_ts, o.qty.0);
                }
            }
            D::OrderModify(o) => {
                if let Some(side) = conv_side(o.side) {
                    // Priority LOST: remove the old (price, priority_ts)
                    // identity entirely, re-add at the back of whatever
                    // level the new price maps to.
                    self.remove_order(side, o.prev_price.0, o.prev_priority_ts);
                    self.add_order(side, o.price.0, o.priority_ts, o.qty.0);
                }
            }
            D::OrderModifySamePriority(o) => {
                if let Some(side) = conv_side(o.side) {
                    self.modify_same_priority(side, o.price.0, o.priority_ts, o.qty.0);
                }
            }
            D::OrderDelete(o) => {
                if let Some(side) = conv_side(o.side) {
                    self.remove_order(side, o.price.0, o.priority_ts);
                }
            }
            D::OrderMassDelete(_) => self.mass_delete(),
            D::Trade(t) => {
                if let Some(side) = conv_side(t.aggressor_side) {
                    // `t.event_time`: see `apply_trade`'s doc comment --
                    // on a real Trade record this is the matched resting
                    // order's own `priority_ts`, not a wall-clock time.
                    self.apply_trade(side, t.price.0, t.event_time, t.qty.0, t.full);
                }
            }
            _ => {}
        }
    }

    /// Every non-empty level on one side, best-to-worst. Not part of the
    /// `Book` trait (whose `depth(n)` has no `side` parameter to give it
    /// meaning at "all of them") -- used by this component's own tests
    /// and by the FR-B11 validation harness, which needs the true full
    /// depth, not just the top `n`.
    pub fn full_depth(&self, side: Side) -> Vec<PriceLevel> {
        self.levels(side)
            .iter()
            .enumerate()
            .filter(|(_, lvl)| !lvl.orders.is_empty())
            .map(|(idx, lvl)| PriceLevel {
                price: Price(self.price_of(idx)),
                qty: Qty(lvl.qty),
                order_count: lvl.orders.len() as u32,
            })
            .collect()
    }

    /// Every individual resting order on one side, as `(price_raw,
    /// priority_ts, qty_raw)` triples, in no particular order. Used by
    /// the FR-B11 harness for the order-level (not just aggregate)
    /// comparison against the snapshot's own per-order `13602` records.
    pub fn resting_orders(&self, side: Side) -> Vec<(i64, u64, i64)> {
        self.levels(side)
            .iter()
            .enumerate()
            .flat_map(|(idx, lvl)| {
                let price_raw = self.price_of(idx);
                lvl.orders.iter().map(move |s| (price_raw, s.priority_ts, s.qty))
            })
            .collect()
    }

    /// Seeds this book directly from a snapshot's resting-order list,
    /// bypassing `apply()` -- **this is D14's real initialization path**
    /// (`Uninit` -> `Ok` via a snapshot), not a workaround: an increment
    /// stream that starts recording partway through a book's life (or at
    /// the start of a session that still carries GTC/multi-day resting
    /// orders from *before* the capture began) has no `OrderAdd` for
    /// those pre-existing orders at all -- there is nothing to replay.
    /// FR-B11's own validation harness needed this for exactly that
    /// reason: the very first live snapshot cycle for both validated
    /// instruments contained resting orders with `priority_ts` values
    /// from days earlier than the capture date, which a pure
    /// empty-book-plus-increments replay can never reconstruct. Orders
    /// are inserted in ascending `priority_ts` order so each price
    /// level's FIFO ends up in correct arrival-order, exactly as if they
    /// had been added one at a time in that order.
    pub fn bootstrap(&mut self, orders: &mut [(Side, i64, u64, i64)]) {
        orders.sort_by_key(|(_, _, priority_ts, _)| *priority_ts);
        for &(side, price_raw, priority_ts, qty_raw) in orders.iter() {
            self.add_order(side, price_raw, priority_ts, qty_raw);
        }
    }

    /// Widens this book's dense array to cover `[new_min_raw,
    /// new_max_raw]`, preserving every currently-resting order on both
    /// sides -- never shrinks (always takes the union of the current band
    /// and the new one). This is a real, not hypothetical, need: MCX's
    /// real `19_01_2026` NATURALGAS (465849) DPR genuinely widened six
    /// separate times over the session (both bounds moving outward --
    /// e.g. lower/upper 269.20/291.60 at first observation down/up to
    /// 221.60/339.20 by session's end, each step a real, distinct
    /// `InstrumentInfo` (13603) -- see book_user_doc.md's "generic price
    /// band" section for the full, real value sequence). A book sized
    /// only from the *first* band it ever saw would panic in
    /// `MboBookImpl::idx_of` the moment a real order landed outside it.
    ///
    /// `tick_raw` is assumed unchanged (an instrument's tick size doesn't
    /// change intraday); `new_min_raw`/`new_max_raw` are assumed to
    /// already sit on this book's tick grid, true for any two real
    /// `InstrumentInfo` for the same instrument (MCX always publishes DPR
    /// bounds on-tick, confirmed against every real value observed).
    fn widen_band_if_needed(&mut self, new_min_raw: i64, new_max_raw: i64) {
        let cur_max_raw = self.band_min_raw + self.tick_raw * (self.n_ticks as i64 - 1);
        let min = self.band_min_raw.min(new_min_raw);
        let max = cur_max_raw.max(new_max_raw);
        if min == self.band_min_raw && max == cur_max_raw {
            return; // already covers it -- the common case (repeat/unchanged DPR)
        }
        let new_n_ticks = ((max - min) / self.tick_raw) as usize + 1;
        let mut new_bid: Vec<Level> = (0..new_n_ticks).map(|_| Level::default()).collect();
        let mut new_ask: Vec<Level> = (0..new_n_ticks).map(|_| Level::default()).collect();
        let shift = ((self.band_min_raw - min) / self.tick_raw) as usize;
        for (idx, lvl) in self.bid_levels.drain(..).enumerate() {
            if !lvl.orders.is_empty() {
                new_bid[idx + shift] = lvl;
            }
        }
        for (idx, lvl) in self.ask_levels.drain(..).enumerate() {
            if !lvl.orders.is_empty() {
                new_ask[idx + shift] = lvl;
            }
        }
        self.band_min_raw = min;
        self.n_ticks = new_n_ticks;
        self.bid_levels = new_bid;
        self.ask_levels = new_ask;
    }
}

impl Book for MboBookImpl {
    fn best_bid(&self) -> Option<PriceLevel> {
        (0..self.n_ticks).rev().find_map(|idx| {
            let lvl = &self.bid_levels[idx];
            (!lvl.orders.is_empty()).then(|| PriceLevel {
                price: Price(self.price_of(idx)),
                qty: Qty(lvl.qty),
                order_count: lvl.orders.len() as u32,
            })
        })
    }

    fn best_ask(&self) -> Option<PriceLevel> {
        // FR-B09: no cross-check against best_bid here, deliberately --
        // a crossed book (best_bid >= best_ask) is a normal transient
        // state on an order-by-order feed, not something to assert on.
        (0..self.n_ticks).find_map(|idx| {
            let lvl = &self.ask_levels[idx];
            (!lvl.orders.is_empty()).then(|| PriceLevel {
                price: Price(self.price_of(idx)),
                qty: Qty(lvl.qty),
                order_count: lvl.orders.len() as u32,
            })
        })
    }

    fn depth(&self, n: usize) -> Vec<PriceLevel> {
        let mut out = self.full_depth(Side::Buy);
        out.reverse(); // full_depth is worst-to-best (index order); best-to-worst wanted
        out.truncate(n);
        let mut asks = self.full_depth(Side::Sell);
        asks.truncate(n);
        out.extend(asks);
        out
    }

    fn qty_at_price(&self, side: Side, price: Price) -> Qty {
        match self.idx_of_checked(price.0) {
            Some(idx) => Qty(self.levels(side)[idx].qty),
            None => Qty(0),
        }
    }

    fn state(&self) -> BookState {
        self.state
    }
}

impl MboBook for MboBookImpl {
    fn queue_position(&self, handle: OrderHandle) -> Option<i64> {
        if handle.instrument != self.instrument {
            return None;
        }
        let idx = self.idx_of_checked(handle.price.0)?;
        let lvl = &self.levels(handle.side)[idx];
        let mut ahead = 0i64;
        for slot in &lvl.orders {
            if slot.priority_ts == handle.priority_ts {
                return Some(ahead);
            }
            ahead += slot.qty;
        }
        None
    }
}

/// `pub(crate)`: shared with this component's own FR-B11 validation
/// harness (`validate.rs`), which needs the same wire-`Side` ->
/// `types::Side` conversion when assembling ground-truth orders from
/// `SnapshotOrder` (13602) records.
pub(crate) fn conv_side(s: decoder::Side) -> Option<Side> {
    match s {
        decoder::Side::Buy => Some(Side::Buy),
        decoder::Side::Sell => Some(Side::Sell),
        decoder::Side::Unknown(_) => None,
    }
}

// ---------------------------------------------------------------------
// BookBuilder -- ARCHITECTURE.md §4.8, the multi-instrument owner.
// ---------------------------------------------------------------------

/// Native MCX `SecurityID` tokens for the two instruments this
/// component's own FR-B11 validation harness targets. No longer
/// load-bearing for tick size or price band (both are now generic --
/// see `BookBuilder` below); kept only as named, readable constants for
/// call sites (`validate.rs`, `cache.rs`, `dummy_strategy.rs`) that still
/// want to refer to "the CRUDEOIL/NATURALGAS token" by name rather than a
/// bare integer literal.
pub const CRUDEOIL_ID: InstrumentId = InstrumentId(467_013);
pub const NATURALGAS_ID: InstrumentId = InstrumentId(465_849);

const RUPEE_RAW: i64 = 100_000_000; // decoder's MCX_PRICE_MULTIPLIER

/// A `13603` (`InstrumentInfo`) band is trusted only if it's internally
/// sane: `lower < upper`, and neither bound sits anywhere near `i64::MIN`
/// -- this project's own real, empirically-found sentinel for "not yet
/// computed" (`decoder::InstrumentInfo`'s doc comment: a genuine
/// End-of-Day rebroadcast artifact found at the tail of both real
/// `19_01_2026` increment capture files, with `PrevClosePrice`/
/// `UpperDailyPriceLimit`/`LowerDailyPriceLimit` all exactly `i64::MIN`).
/// Never trusted blindly, matching this project's established "check
/// real bytes, don't assume the wire value is sane" discipline.
fn plausible_band(lower_raw: i64, upper_raw: i64) -> bool {
    lower_raw > i64::MIN / 2 && upper_raw > i64::MIN / 2 && lower_raw < upper_raw
}

fn security_id_of(event: &decoder::DecodedMessage) -> Option<i64> {
    use decoder::DecodedMessage as D;
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

/// One instrument's construction state inside `BookBuilder`. A book's
/// dense array can't be sized (`MboBookImpl::new` needs both a tick size
/// and a price band) until both are known -- `refdata` gives the tick
/// size generically and up front (`types::Instrument.tick_size`, correct
/// for every instrument now, see refdata_user_doc.md), but the band only
/// arrives as a real `InstrumentInfo` (13603) message *in the same feed*
/// `book` already consumes, not as static reference data available at
/// construction time. `Pending` is that real, expected initial state, not
/// an error condition -- see `BookBuilder::apply`'s `InstrumentInfo`
/// handling for how (and when) it becomes `Ready`.
enum BookSlot {
    Pending { tick_raw: i64 },
    Ready(MboBookImpl),
}

/// Owns one `MboBookImpl` per instrument it was constructed with (once
/// each one's band is learned -- see `BookSlot`). `cache` (T05) and
/// `simulator` (T06) hold or drive this rather than owning per-instrument
/// book storage themselves (ARCHITECTURE.md §4.8).
///
/// **Fully generic now -- any FUTCOM instrument `refdata` knows about,
/// not a two-instrument allowlist.** Tick size and price band used to be
/// a hardcoded per-id match (`band_config`, removed) covering exactly
/// CRUDEOIL/NATURALGAS; both are now real, per-instrument facts learned
/// the same way for every instrument: tick size from the caller (sourced
/// from `refdata`), band from a real `InstrumentInfo` (13603) observed in
/// the applied stream, or from `seed_band` for a caller whose own feed
/// doesn't carry one. See book_user_doc.md's "generic price band and
/// tick size" section for the real 19_01_2026-session evidence this was
/// checked against (CRUDEOIL/NATURALGAS reproduce the exact old
/// `band_config` numbers; ALUMINIUM, never previously supported, now
/// builds and runs against real order flow).
///
/// **Instrument scope is still caller-declared, not learned:** there is
/// still no strategy-declared filter inside this component (that's
/// M5/T05's `on_start` predicate, D32) -- `BookBuilder` only ever tracks
/// the instruments it was constructed with; an event for anything else is
/// silently ignored (FR-B16 filtering intent), same as before.
pub struct BookBuilder {
    slots: HashMap<InstrumentId, BookSlot>,
}

impl BookBuilder {
    /// `instruments`: `(id, tick_raw)` pairs, `tick_raw` in the same raw
    /// wire-price scale everything else in this module uses (e.g. from
    /// `types::Instrument.tick_size.0`). The price band is deliberately
    /// *not* a constructor parameter -- unlike tick size, MCXScrips.bcp
    /// can't give it generically (its own DPR columns are an
    /// unconvertible percentage with no reference price, see
    /// refdata_user_doc.md), so every instrument starts `Pending` and is
    /// finalized by `apply`/`seed_band` once a real band is known.
    pub fn new(instruments: &[(InstrumentId, i64)]) -> Self {
        let mut slots = HashMap::new();
        for &(id, tick_raw) in instruments {
            assert!(
                tick_raw > 0,
                "book: instrument {id:?} constructed with a non-positive tick size ({tick_raw}) -- caller passed a bad value"
            );
            slots.insert(id, BookSlot::Pending { tick_raw });
        }
        BookBuilder { slots }
    }

    /// Explicitly supplies a real, already-known band for `id`, without
    /// waiting for an `InstrumentInfo` to arrive through `apply`.
    ///
    /// **A real, not hypothetical, gap this exists for.** Checked against
    /// real bytes: MCX's real `19_01_2026` increment capture for CRUDEOIL
    /// (467013) never carries a *valid* 13603 during the trading session
    /// at all (only a corrupted End-of-Day one, see
    /// `decoder::InstrumentInfo`'s doc comment) -- its DPR never changed
    /// that day, so nothing re-published it on the increment channel
    /// (only the snapshot channel repeats it every cycle -- see
    /// book_user_doc.md), and this capture began recording after the
    /// one-time Start-of-Day broadcast that would have carried it. A
    /// caller whose only real feed *is* that increment stream (this
    /// crate's `cache`/`dummy_strategy` binaries, both increment-file-only
    /// consumers) has no way to learn CRUDEOIL's band from `apply` alone
    /// -- exactly the scenario `apply`'s own doc comment says to fail
    /// loud on, unless the caller supplies the band another real way.
    /// This is that other way: the caller passes real,
    /// independently-verified numbers (this codebase's callers source
    /// theirs from the paired snapshot capture's own 13603 stream -- the
    /// same real values a live consumer with snapshot-channel access
    /// would have learned on its own). Same idiom as
    /// `MboBookImpl::bootstrap` (explicit seeding from real out-of-band
    /// data, not the normal streaming path) -- not a fallback to a guess.
    ///
    /// No-op if `id` isn't a tracked (constructed) instrument, or if the
    /// given bounds don't pass `plausible_band`. Widens (never narrows)
    /// if the slot is already `Ready` -- same union semantics as an
    /// in-stream `InstrumentInfo` (see `MboBookImpl::widen_band_if_needed`).
    pub fn seed_band(&mut self, id: InstrumentId, band_min_raw: i64, band_max_raw: i64) {
        self.learn_band(id, band_min_raw, band_max_raw);
    }

    fn learn_band(&mut self, id: InstrumentId, lower_raw: i64, upper_raw: i64) {
        if !plausible_band(lower_raw, upper_raw) {
            return; // e.g. the real End-of-Day sentinel record -- see decoder::InstrumentInfo's doc comment
        }
        let Some(slot) = self.slots.get_mut(&id) else { return }; // not a tracked instrument -- FR-B16 filtering intent
        match slot {
            BookSlot::Pending { tick_raw } => {
                *slot = BookSlot::Ready(MboBookImpl::new(id, *tick_raw, lower_raw, upper_raw));
            }
            BookSlot::Ready(book) => book.widen_band_if_needed(lower_raw, upper_raw),
        }
    }

    /// Routes one decoded event to the book for its instrument, by
    /// native `SecurityID`. An event for an instrument this builder
    /// wasn't constructed with is silently ignored -- the FR-B16
    /// filtering intent, even without the real strategy-driven predicate
    /// (that's M5).
    ///
    /// A real `InstrumentInfo` (13603) for a tracked instrument finalizes
    /// (or widens) its band -- see `learn_band`.
    ///
    /// **A real order-mutating event for a tracked instrument whose band
    /// is still `Pending` panics loudly**, per this component's existing
    /// "fail on a wrong assumption, never silently guess" discipline
    /// (`MboBookImpl::idx_of`'s own doc comment carries the same
    /// philosophy). Silently skipping it, or guessing a band, would be
    /// exactly the "silent wrong book" failure mode this whole component
    /// exists to catch. Per the EOBI spec's own product-state table
    /// (`InstrumentInfo`/setup-type messages are published during
    /// Start-Of-Day/Pre-Trading, before Trading state's real order flow
    /// begins), this should never fire against a well-formed real feed
    /// that starts recording from Start-of-Day -- see `seed_band`'s doc
    /// comment for the real, verified exception (a feed that starts mid-
    /// session and never sees a DPR revision) and book_user_doc.md for
    /// the full account.
    pub fn apply(&mut self, event: &decoder::DecodedMessage) {
        use decoder::DecodedMessage as D;
        if let D::InstrumentInfo(info) = event {
            self.learn_band(
                InstrumentId(info.security_id as u32),
                info.lower_daily_price_limit.0,
                info.upper_daily_price_limit.0,
            );
            return;
        }
        let Some(sid) = security_id_of(event) else { return };
        let id = InstrumentId(sid as u32);
        match self.slots.get_mut(&id) {
            None => {} // not a tracked instrument -- FR-B16 filtering intent
            Some(BookSlot::Ready(book)) => book.apply(event),
            Some(BookSlot::Pending { .. }) => panic!(
                "book[{id:?}]: real order-mutating event arrived before this instrument's price band was known -- \
                 no InstrumentInfo (13603) has been seen for it yet, and BookBuilder::seed_band was never called. \
                 event={event:?}. See book_user_doc.md's \"generic price band\" section."
            ),
        }
    }

    pub fn get(&self, id: InstrumentId) -> Option<&dyn Book> {
        match self.slots.get(&id) {
            Some(BookSlot::Ready(b)) => Some(b as &dyn Book),
            _ => None,
        }
    }

    /// Non-trait-object accessor for the FR-B11 harness and tests, which
    /// need `full_depth`/`resting_orders` -- not part of `Book`. Returns
    /// `None` for a tracked-but-still-`Pending` instrument, same as for
    /// an untracked one -- callers needing to distinguish should not need
    /// to for any real, well-formed replay (see `apply`'s doc comment).
    pub fn get_impl(&self, id: InstrumentId) -> Option<&MboBookImpl> {
        match self.slots.get(&id) {
            Some(BookSlot::Ready(b)) => Some(b),
            _ => None,
        }
    }

    /// Mutable counterpart, used only by the FR-B11 harness to call
    /// `MboBookImpl::bootstrap` once per instrument before replaying any
    /// increments -- see `bootstrap`'s doc comment.
    pub fn get_impl_mut(&mut self, id: InstrumentId) -> Option<&mut MboBookImpl> {
        match self.slots.get_mut(&id) {
            Some(BookSlot::Ready(b)) => Some(b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{DecodedMessage, OrderAdd, OrderDelete, OrderMassDelete, OrderModify, OrderModifySamePriority, Price as DPrice, Qty as DQty, Side as DSide, Trade};

    fn test_book() -> MboBookImpl {
        // Small band for fast, readable tests: ticks of 1, band [100,200].
        MboBookImpl::new(InstrumentId(1), 1, 100, 200)
    }

    fn add(id: i64, side: DSide, price: i64, qty: i64, prio: u64) -> DecodedMessage {
        DecodedMessage::OrderAdd(OrderAdd {
            seq: 0,
            security_id: id,
            side,
            price: DPrice(price),
            qty: DQty(qty),
            priority_ts: prio,
            event_time: 0, // not used by book logic -- see decoder::OrderAdd::event_time
        })
    }

    #[test]
    fn add_then_best_bid_and_ask() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Buy, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 160, 5, 2));
        assert_eq!(b.best_bid().unwrap().price, Price(150));
        assert_eq!(b.best_bid().unwrap().qty, Qty(10));
        assert_eq!(b.best_ask().unwrap().price, Price(160));
        assert_eq!(b.state(), BookState::Ok);
    }

    #[test]
    fn crossed_book_does_not_panic() {
        // FR-B09: an aggressive order publishes before the trade it
        // causes -- a bid at or above an existing ask is legal, transient
        // state, not a bug.
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 5, 1));
        b.apply(&add(1, DSide::Buy, 155, 5, 2)); // crosses the ask at 150
        assert_eq!(b.best_bid().unwrap().price, Price(155));
        assert_eq!(b.best_ask().unwrap().price, Price(150));
    }

    #[test]
    fn modify_loses_priority_moves_to_back() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Buy, 150, 10, 1));
        b.apply(&add(1, DSide::Buy, 150, 20, 2));
        // order 1 modifies (price unchanged, qty increased) -- priority LOST,
        // goes to the back behind order 2 even though it arrived first.
        b.apply(&DecodedMessage::OrderModify(OrderModify {
            seq: 0,
            security_id: 1,
            side: DSide::Buy,
            prev_price: DPrice(150),
            prev_qty: DQty(10),
            price: DPrice(150),
            qty: DQty(15),
            prev_priority_ts: 1,
            priority_ts: 3,
            event_time: 0,
        }));
        let orders = b.resting_orders(Side::Buy);
        let mut by_prio: Vec<_> = orders.iter().map(|(_, p, _)| *p).collect();
        by_prio.sort();
        assert_eq!(by_prio, vec![2, 3]);
        assert_eq!(b.qty_at_price(Side::Buy, Price(150)), Qty(35)); // 20 + 15
        let h = OrderHandle { instrument: InstrumentId(1), side: Side::Buy, price: Price(150), priority_ts: 3 };
        assert_eq!(b.queue_position(h), Some(20)); // order 2's 20 ahead of it
    }

    #[test]
    fn modify_same_priority_keeps_fifo_slot() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Buy, 150, 10, 1));
        b.apply(&add(1, DSide::Buy, 150, 20, 2));
        b.apply(&DecodedMessage::OrderModifySamePriority(OrderModifySamePriority {
            seq: 0,
            security_id: 1,
            side: DSide::Buy,
            prev_qty: DQty(10),
            qty: DQty(4),
            price: DPrice(150),
            priority_ts: 1,
            event_time: 0,
        }));
        assert_eq!(b.qty_at_price(Side::Buy, Price(150)), Qty(24)); // 4 + 20
        let h = OrderHandle { instrument: InstrumentId(1), side: Side::Buy, price: Price(150), priority_ts: 1 };
        assert_eq!(b.queue_position(h), Some(0)); // still at the front
    }

    #[test]
    fn delete_removes_exact_order() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Buy, 150, 10, 1));
        b.apply(&add(1, DSide::Buy, 150, 20, 2));
        b.apply(&DecodedMessage::OrderDelete(OrderDelete {
            seq: 0,
            security_id: 1,
            side: DSide::Buy,
            price: DPrice(150),
            qty: DQty(10),
            priority_ts: 1,
            event_time: 0,
        }));
        assert_eq!(b.qty_at_price(Side::Buy, Price(150)), Qty(20));
        assert_eq!(b.resting_orders(Side::Buy).len(), 1);
    }

    #[test]
    fn mass_delete_clears_both_sides() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Buy, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 160, 5, 2));
        b.apply(&DecodedMessage::OrderMassDelete(OrderMassDelete { seq: 0, security_id: 1, event_time: 0 }));
        assert!(b.best_bid().is_none());
        assert!(b.best_ask().is_none());
    }

    #[test]
    fn partial_trade_reduces_front_order_only() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 150, 20, 2));
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: false,
            security_id: 1,
            aggressor_side: DSide::Sell, // resting side hit, per business-rule finding
            price: DPrice(150),
            qty: DQty(4),
            event_time: 0,
        }));
        let mut orders = b.resting_orders(Side::Sell);
        orders.sort_by_key(|(_, p, _)| *p);
        assert_eq!(orders, vec![(150, 1, 6), (150, 2, 20)]);
        assert_eq!(b.qty_at_price(Side::Sell, Price(150)), Qty(26));
    }

    #[test]
    fn full_trade_removes_front_order() {
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 150, 20, 2));
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: true,
            security_id: 1,
            aggressor_side: DSide::Sell,
            price: DPrice(150),
            qty: DQty(10),
            event_time: 0,
        }));
        let orders = b.resting_orders(Side::Sell);
        assert_eq!(orders, vec![(150, 2, 20)]);
    }

    #[test]
    fn trade_larger_than_front_order_cascades_to_next() {
        // Regression test for a real divergence found in FR-B11
        // validation (see book_user_doc.md): a trade quantity larger
        // than the front order's remaining size must fully consume that
        // order and carry the remainder into the next one, never leave
        // a negative-quantity zombie sitting at the front.
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 150, 20, 2));
        b.apply(&add(1, DSide::Sell, 150, 30, 3));
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: false, // wire flag deliberately says "partial" -- magnitude still governs, see apply_trade
            security_id: 1,
            aggressor_side: DSide::Sell,
            price: DPrice(150),
            qty: DQty(25), // more than order 1's 10 -- must finish order 1 and eat 15 from order 2
            event_time: 0,
        }));
        let orders = b.resting_orders(Side::Sell);
        assert_eq!(orders, vec![(150, 2, 5), (150, 3, 30)]);
        assert!(orders.iter().all(|(_, _, qty)| *qty >= 0), "no negative-quantity zombie orders");
    }

    #[test]
    fn trade_targets_specific_order_by_event_time_not_fifo_front() {
        // Regression test for the real root cause of every NATURALGAS
        // FR-B11 miss and divergence (see book_user_doc.md §5.7): a real
        // Trade's `event_time` field carries the *matched resting
        // order's own* `priority_ts`, not a wall-clock time -- MCX
        // targets a specific resting order directly, not necessarily the
        // FIFO front. Three orders rest at one busy level; a trade
        // targets the *middle* one specifically. Only that order may
        // move -- the front order (which arrived first but wasn't the
        // one actually hit) must be completely untouched.
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1)); // front -- must stay untouched
        b.apply(&add(1, DSide::Sell, 150, 50, 2)); // the real target
        b.apply(&add(1, DSide::Sell, 150, 30, 3)); // behind the target -- must also stay untouched
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: false,
            security_id: 1,
            aggressor_side: DSide::Sell,
            price: DPrice(150),
            qty: DQty(20), // partial fill of order 2 specifically
            event_time: 2, // matches order 2's priority_ts, not order 1's (the FIFO front)
        }));
        let orders = b.resting_orders(Side::Sell);
        assert_eq!(orders, vec![(150, 1, 10), (150, 2, 30), (150, 3, 30)]);
    }

    #[test]
    fn trade_cascades_from_targeted_order_not_from_front() {
        // Same real-data pattern as above, but the trade quantity exceeds
        // what the *targeted* order has left -- the remainder must
        // cascade into the next order *after the target* in arrival
        // order (order 3), never touching the untargeted front order
        // (order 1), which the original FIFO-front-cascade design would
        // have consumed from instead.
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1)); // front -- must stay untouched
        b.apply(&add(1, DSide::Sell, 150, 50, 2)); // the real target, fully consumed
        b.apply(&add(1, DSide::Sell, 150, 30, 3)); // must absorb the 10-unit remainder
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: true,
            security_id: 1,
            aggressor_side: DSide::Sell,
            price: DPrice(150),
            qty: DQty(60), // more than order 2's 50 -- finishes order 2, spills 10 into order 3
            event_time: 2,
        }));
        let orders = b.resting_orders(Side::Sell);
        assert_eq!(orders, vec![(150, 1, 10), (150, 3, 20)]);
    }

    #[test]
    fn trade_falls_back_to_fifo_front_when_target_not_resting() {
        // If the matched `priority_ts` isn't in this book at all (e.g. a
        // genuine race, or an order that predates this replay's
        // bootstrap window), the trade must still affect the book rather
        // than being silently dropped -- falling back to the old
        // FIFO-front policy, not a no-op.
        let mut b = test_book();
        b.apply(&add(1, DSide::Sell, 150, 10, 1));
        b.apply(&add(1, DSide::Sell, 150, 20, 2));
        b.apply(&DecodedMessage::Trade(Trade {
            seq: 0,
            full: true,
            security_id: 1,
            aggressor_side: DSide::Sell,
            price: DPrice(150),
            qty: DQty(10),
            event_time: 999_999, // matches no resting order
        }));
        let orders = b.resting_orders(Side::Sell);
        assert_eq!(orders, vec![(150, 2, 20)]); // order 1 (the FIFO front) was consumed instead
    }

    #[test]
    fn bootstrap_reconstructs_fifo_order_from_priority_ts() {
        // Simulates seeding from a snapshot that already contains orders
        // this book's own increment replay never saw an ADD for (e.g.
        // multi-day-resident orders) -- D14's real initialization path.
        let mut b = test_book();
        let mut orders = vec![(Side::Buy, 150, 5, 10), (Side::Buy, 150, 2, 20), (Side::Buy, 150, 9, 5)];
        b.bootstrap(&mut orders);
        assert_eq!(b.qty_at_price(Side::Buy, Price(150)), Qty(35));
        // FIFO order must follow priority_ts (2, 5, 9), not insertion order.
        let h_first = OrderHandle { instrument: InstrumentId(1), side: Side::Buy, price: Price(150), priority_ts: 2 };
        let h_middle = OrderHandle { instrument: InstrumentId(1), side: Side::Buy, price: Price(150), priority_ts: 5 };
        assert_eq!(b.queue_position(h_first), Some(0));
        assert_eq!(b.queue_position(h_middle), Some(20)); // order with priority_ts=2 (qty 20) ahead of it
    }

    #[test]
    fn book_builder_routes_by_instrument_and_ignores_unfiltered() {
        let mut bb = BookBuilder::new(&[(InstrumentId(467_013), RUPEE_RAW)]);
        bb.seed_band(InstrumentId(467_013), 3_000 * RUPEE_RAW, 9_000 * RUPEE_RAW);
        bb.apply(&add(467_013, DSide::Buy, 5_400 * RUPEE_RAW, 1, 1));
        bb.apply(&add(999_999, DSide::Buy, 100, 1, 2)); // not in the filter set
        assert!(bb.get(InstrumentId(467_013)).unwrap().best_bid().is_some());
        assert!(bb.get(InstrumentId(999_999)).is_none());
    }

    // ---- Generic price band / tick size mechanism (real-data-driven) -------

    fn instrument_info(id: i64, lower_raw: i64, upper_raw: i64) -> DecodedMessage {
        DecodedMessage::InstrumentInfo(decoder::InstrumentInfo {
            seq: 0,
            security_id: id,
            close_price: DPrice(0),
            prev_close_price: DPrice(0),
            upper_daily_price_limit: DPrice(upper_raw),
            lower_daily_price_limit: DPrice(lower_raw),
        })
    }

    #[test]
    #[should_panic(expected = "price band was known")]
    fn order_before_any_band_known_panics_loudly() {
        // Per BookBuilder::apply's doc comment: a real order-mutating
        // event for a tracked instrument whose band is still Pending is a
        // genuine problem, not something to silently guess at.
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.apply(&add(1, DSide::Buy, 150, 10, 1));
    }

    #[test]
    fn instrument_info_finalizes_a_pending_book() {
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        assert!(bb.get(InstrumentId(1)).is_none(), "still Pending -- no InstrumentInfo seen yet");
        bb.apply(&instrument_info(1, 100, 200));
        assert!(bb.get(InstrumentId(1)).is_some(), "InstrumentInfo finalized the band -- book is now Ready");
        bb.apply(&add(1, DSide::Buy, 150, 10, 1));
        assert_eq!(bb.get(InstrumentId(1)).unwrap().best_bid().unwrap().price, Price(150));
    }

    #[test]
    fn instrument_info_for_an_untracked_instrument_is_ignored() {
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.apply(&instrument_info(999_999, 100, 200)); // not constructed with this id
        assert!(bb.get(InstrumentId(999_999)).is_none());
    }

    #[test]
    fn instrument_info_sentinel_garbage_is_rejected_not_trusted() {
        // Real, not hypothetical: a genuine End-of-Day rebroadcast found
        // at the tail of both real 19_01_2026 increment capture files
        // carries i64::MIN for the band fields -- see
        // decoder::InstrumentInfo's doc comment. Must never be trusted as
        // a real band.
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.apply(&instrument_info(1, i64::MIN, i64::MIN));
        assert!(bb.get(InstrumentId(1)).is_none(), "sentinel garbage must not finalize the book");
    }

    #[test]
    fn later_wider_instrument_info_widens_the_book_preserving_orders() {
        // Real, not hypothetical: NATURALGAS's real DPR widened six times
        // over the 19_01_2026 session (see book_user_doc.md). A book
        // sized only from the first band it saw must still accept a real
        // order that only a later, wider band makes representable --
        // without losing any order already resting from the narrower
        // band.
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.apply(&instrument_info(1, 100, 200));
        bb.apply(&add(1, DSide::Buy, 150, 10, 1));
        bb.apply(&add(1, DSide::Sell, 190, 5, 2));
        // Widen both edges outward, as a real DPR revision did for NATURALGAS.
        bb.apply(&instrument_info(1, 50, 250));
        // Pre-existing orders (from the narrower band) must still be there.
        assert_eq!(bb.get(InstrumentId(1)).unwrap().best_bid().unwrap().price, Price(150));
        assert_eq!(bb.get(InstrumentId(1)).unwrap().best_ask().unwrap().price, Price(190));
        // A price only representable under the new, wider band must now work.
        bb.apply(&add(1, DSide::Buy, 60, 3, 3));
        bb.apply(&add(1, DSide::Sell, 240, 2, 4));
        let book = bb.get_impl(InstrumentId(1)).unwrap();
        assert_eq!(book.qty_at_price(Side::Buy, Price(60)), Qty(3));
        assert_eq!(book.qty_at_price(Side::Sell, Price(240)), Qty(2));
        // Original resting orders are untouched by the resize.
        assert_eq!(book.qty_at_price(Side::Buy, Price(150)), Qty(10));
        assert_eq!(book.qty_at_price(Side::Sell, Price(190)), Qty(5));
    }

    #[test]
    fn narrower_later_instrument_info_never_shrinks_the_band() {
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.apply(&instrument_info(1, 100, 200));
        bb.apply(&add(1, DSide::Buy, 100, 10, 1)); // at the original lower edge
        bb.apply(&instrument_info(1, 150, 180)); // narrower -- must be a no-op
        // The order at the old edge must still be there and still reachable.
        assert_eq!(bb.get_impl(InstrumentId(1)).unwrap().qty_at_price(Side::Buy, Price(100)), Qty(10));
    }

    #[test]
    fn seed_band_finalizes_a_book_without_any_stream_instrument_info() {
        // The real gap `seed_band` exists for: a caller whose own feed
        // (e.g. an increment-only capture) never carries a valid
        // InstrumentInfo for this instrument during the session at all
        // (real, confirmed case: CRUDEOIL on 19_01_2026 -- see
        // BookBuilder::seed_band's doc comment).
        let mut bb = BookBuilder::new(&[(InstrumentId(1), 1)]);
        bb.seed_band(InstrumentId(1), 100, 200);
        bb.apply(&add(1, DSide::Buy, 150, 10, 1));
        assert_eq!(bb.get(InstrumentId(1)).unwrap().best_bid().unwrap().price, Price(150));
    }
}

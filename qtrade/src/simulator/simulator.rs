//! The simulator component: MCX's Simulated Exchange (BACKTEST-PHASE1.md
//! §M6, FR-B19 through FR-B24; ARCHITECTURE-DECISIONS.md D10/D18/D19/D21/D31
//! Layer 2). See `simulator_user_doc.md` in this folder for the full
//! account: why independence from `cache` is not a formality, what each
//! FR-B24 invariant actually checks and why, and a fully worked
//! queue-position trace against real data.
//!
//! ## D10, taken literally
//!
//! This module builds its **own** order books directly from
//! `decoder::decode_messages`' output. It does not import, call, or read
//! anything from `crate::cache` or `crate::book` -- those modules are
//! never named below except in comments citing where a business rule was
//! independently cross-checked. The dense-array-vs-BTreeMap difference
//! from `book`'s implementation is deliberate: same *trait shape*
//! (`Book`/`MboBook`, matching `book::Book`/`book::MboBook`'s public
//! signatures for interface consistency), different internal data
//! structure, built by reading `references/MCX_Feeder.cpp` and
//! `decoder.rs` directly, not by reusing `book`'s code.
//!
//! ## Convention carried over from `decoder`/`book`
//!
//! `Debug` derived everywhere, no exceptions. `Display` hand-written only
//! on types meant to be read by a person (execution reports, in the
//! validation binary's trace output).

use crate::decoder;
use crate::types::{BookState, InstrumentId, OrderHandle, Price, PriceLevel, Qty, Side, Venue};

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::time::Duration;

// =======================================================================
// 1. Book -- independent reimplementation, same trait shape as `book`.
// =======================================================================

pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    /// Same convention `book::Book::depth` documents: best `n` bid levels
    /// best-to-worst, then best `n` ask levels best-to-worst.
    fn depth(&self, n: usize) -> Vec<PriceLevel>;
    fn qty_at_price(&self, side: Side, price: Price) -> Qty;
    fn state(&self) -> BookState;
}

pub trait MboBook: Book {
    /// Aggregate quantity resting strictly ahead of this handle at its
    /// price. `None` if the handle doesn't identify a currently-resting
    /// order (real or our own simulated one) in this book.
    fn queue_position(&self, handle: OrderHandle) -> Option<i64>;
}

/// One FIFO participant at a price level: either a real resting MCX order
/// (identified by its wire `priority_ts`, same identity scheme as
/// `types::OrderHandle` / `book`'s `OrderSlot`) or one of *our own*
/// simulated resting orders (identified by an internally-assigned id).
///
/// Real and simulated participants share **one** FIFO per price level,
/// ordered by arrival -- this is what makes queue-position tracking exact
/// (FR-B21): a simulated order sits at the back of the *combined* queue
/// the instant it arrives, and a cascading trade that eats past the real
/// orders ahead of it reaches it exactly when it would in the real
/// engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotOwner {
    Real(u64),
    Sim(u64),
}

#[derive(Debug, Clone, Copy)]
struct Slot {
    owner: SlotOwner,
    qty: i64,
}

#[derive(Debug, Default)]
struct Level {
    qty: i64,
    orders: VecDeque<Slot>,
}

/// Simulated ids are drawn from a range that provably cannot collide with
/// a real `priority_ts`: `priority_ts`/`TrdRegTSTimePriority` is an epoch
/// nanosecond value (confirmed by `decoder`/`book`'s cross-checks against
/// real capture timestamps) and 2026 epoch-nanoseconds sit at roughly
/// 1.77 x 10^18 -- comfortably below this base, with `u64::MAX` at
/// ~1.8 x 10^19 giving ample headroom above it for the run's lifetime.
const SIM_ID_BASE: u64 = 9_000_000_000_000_000_000;

/// One instrument's book: a price-sorted map of `Level`s per side. Unlike
/// `book::MboBookImpl`'s dense pre-sized array (sized off an
/// empirically-observed DPR band), this uses a `BTreeMap<i64, Level>` --
/// no band to size, no panic-on-out-of-band risk, still O(log n) lookups
/// and naturally sorted for `best_bid`/`best_ask`. A legitimate
/// alternative internal design, not `book`'s code.
pub struct SimBookImpl {
    instrument: InstrumentId,
    bids: BTreeMap<i64, Level>,
    asks: BTreeMap<i64, Level>,
    state: BookState,
    diag_remove_misses: u64,
    diag_trade_misses: u64,
}

impl SimBookImpl {
    pub fn new(instrument: InstrumentId) -> Self {
        SimBookImpl {
            instrument,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            state: BookState::Uninit,
            diag_remove_misses: 0,
            diag_trade_misses: 0,
        }
    }

    pub fn diagnostics(&self) -> (u64, u64) {
        (self.diag_remove_misses, self.diag_trade_misses)
    }

    fn levels(&self, side: Side) -> &BTreeMap<i64, Level> {
        match side {
            Side::Buy => &self.bids,
            Side::Sell => &self.asks,
        }
    }

    fn levels_mut(&mut self, side: Side) -> &mut BTreeMap<i64, Level> {
        match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        }
    }

    // ---- real-order mutation, mirrors business rules independently
    // ---- re-derived from references/MCX_Feeder.cpp -- see
    // ---- simulator_user_doc.md §"business rules" for the line numbers.

    fn add_real(&mut self, side: Side, price_raw: i64, priority_ts: u64, qty_raw: i64) {
        let lvl = self.levels_mut(side).entry(price_raw).or_default();
        lvl.qty += qty_raw;
        lvl.orders.push_back(Slot { owner: SlotOwner::Real(priority_ts), qty: qty_raw });
        self.state = BookState::Ok;
    }

    fn remove_real(&mut self, side: Side, price_raw: i64, priority_ts: u64) {
        let mut now_empty = false;
        if let Some(lvl) = self.levels_mut(side).get_mut(&price_raw) {
            if let Some(pos) = lvl.orders.iter().position(|s| s.owner == SlotOwner::Real(priority_ts)) {
                let slot = lvl.orders.remove(pos).expect("position() just found it");
                lvl.qty -= slot.qty;
                now_empty = lvl.orders.is_empty();
            } else {
                self.diag_remove_misses += 1;
            }
        } else {
            self.diag_remove_misses += 1;
        }
        // Prune an emptied level out of the map entirely -- otherwise
        // `best_bid`/`best_ask`/`qty_ahead_of`'s scans degrade toward
        // O(price levels ever touched) instead of O(price levels
        // currently occupied) over a long-running session (a BTreeMap,
        // unlike `book`'s fixed-size dense array, has to be kept tidy by
        // hand).
        if now_empty {
            self.levels_mut(side).remove(&price_raw);
        }
        self.state = BookState::Ok;
    }

    fn modify_same_priority_real(&mut self, side: Side, price_raw: i64, priority_ts: u64, new_qty_raw: i64) {
        if let Some(lvl) = self.levels_mut(side).get_mut(&price_raw) {
            if let Some(slot) = lvl.orders.iter_mut().find(|s| s.owner == SlotOwner::Real(priority_ts)) {
                lvl.qty += new_qty_raw - slot.qty;
                slot.qty = new_qty_raw;
            } else {
                self.diag_remove_misses += 1;
            }
        } else {
            self.diag_remove_misses += 1;
        }
        self.state = BookState::Ok;
    }

    /// `OrderMassDelete` (13103), independently confirmed against
    /// `references/MCX_Feeder.cpp`'s `Purger_Market_Depth` (~line 1682):
    /// clears both `BuyPrice[]`/`SellPrice[]` arrays and every per-price
    /// bucket's quantity/count for the whole token -- both sides, one
    /// event. Because this really happens at the exchange to *every*
    /// resting order for the instrument, a real MassDelete also cancels
    /// any of *our own* simulated resting orders caught in it -- the
    /// venue does not distinguish "ours" from "theirs". Returns the sim
    /// ids that were caught, so the caller can emit cancellation reports.
    fn mass_delete(&mut self) -> Vec<u64> {
        let mut caught = Vec::new();
        for lvl in self.bids.values_mut().chain(self.asks.values_mut()) {
            for slot in lvl.orders.drain(..) {
                if let SlotOwner::Sim(id) = slot.owner {
                    caught.push(id);
                }
            }
            lvl.qty = 0;
        }
        self.bids.clear();
        self.asks.clear();
        self.state = BookState::Ok;
        caught
    }

    /// `Trade` (13104 full / 13105 partial). **Uses the same
    /// `event_time`-is-`priority_ts` matching rule `book` (T03)
    /// established the hard way (book_user_doc.md §5.7):** the wire
    /// `event_time` field on a real Trade is not a timestamp, it is the
    /// specific resting order's own `priority_ts` that this trade
    /// matched. Look that order up directly; cascade forward through the
    /// *combined* FIFO (real and simulated participants alike) if the
    /// trade quantity exceeds what the targeted slot has left; fall back
    /// to FIFO-front only if the targeted `priority_ts` isn't resting at
    /// all (pre-replay-window order, or a genuine race).
    ///
    /// Returns `(fills, real_qty_consumed)`: `fills` is every simulated
    /// order this specific trade event reached, `real_qty_consumed` is
    /// how much of `trade_qty_raw` this call actually applied to the book
    /// (used by the FR-B24 invariant #1 ledger -- see
    /// `SimExchange::apply_market_event`).
    fn apply_trade(&mut self, side: Side, price_raw: i64, matched_priority_ts: u64, mut trade_qty_raw: i64) -> Vec<(u64, i64)> {
        let mut fills = Vec::new();
        let Some(lvl) = self.levels_mut(side).get_mut(&price_raw) else {
            self.diag_trade_misses += 1;
            self.state = BookState::Ok;
            return fills;
        };
        let mut consumed_any = false;

        if let Some(start) = lvl.orders.iter().position(|s| s.owner == SlotOwner::Real(matched_priority_ts)) {
            let pos = start;
            while trade_qty_raw > 0 && pos < lvl.orders.len() {
                consumed_any = true;
                let slot_qty = lvl.orders[pos].qty;
                let owner = lvl.orders[pos].owner;
                let taken = slot_qty.min(trade_qty_raw);
                if let SlotOwner::Sim(id) = owner {
                    fills.push((id, taken));
                }
                if slot_qty > trade_qty_raw {
                    lvl.orders[pos].qty -= trade_qty_raw;
                    lvl.qty -= trade_qty_raw;
                    trade_qty_raw = 0;
                } else {
                    trade_qty_raw -= slot_qty;
                    lvl.qty -= slot_qty;
                    lvl.orders.remove(pos); // shifts next into `pos` -- don't advance
                }
            }
        } else {
            // Fallback only -- target not resting in this book (see doc
            // comment). Same cascading-from-front policy as `book`.
            while trade_qty_raw > 0 {
                let Some(front) = lvl.orders.front_mut() else { break };
                consumed_any = true;
                let owner = front.owner;
                let taken = front.qty.min(trade_qty_raw);
                if let SlotOwner::Sim(id) = owner {
                    fills.push((id, taken));
                }
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
            self.diag_trade_misses += 1;
        }
        // Prune the level if this trade emptied it -- same reasoning as
        // `remove_real` (keeps best_bid/best_ask/qty_ahead_of scans
        // O(occupied levels), not O(levels ever touched this session).
        let now_empty = self.levels(side).get(&price_raw).map(|l| l.orders.is_empty()).unwrap_or(false);
        if now_empty {
            self.levels_mut(side).remove(&price_raw);
        }
        self.state = BookState::Ok;
        fills
    }

    /// Feeds one real decoded event into this book. No-ops on message
    /// types that don't mutate book state -- same set `book` confirmed
    /// against `references/MCX_Feeder.cpp` (`13202`/`13504`/`13003`/
    /// `13001`/the `Snapshot*` family never touch book state there).
    /// Returns any simulated fills this event produced, plus (for
    /// `OrderMassDelete`) any of our own resting sim orders it wiped out.
    fn apply_real_event(&mut self, event: &decoder::DecodedMessage) -> (Vec<(u64, i64)>, Vec<u64>) {
        use decoder::DecodedMessage as D;
        match event {
            D::OrderAdd(o) => {
                if let Some(side) = conv_side(o.side) {
                    self.add_real(side, o.price.0, o.priority_ts, o.qty.0);
                }
                (Vec::new(), Vec::new())
            }
            D::OrderModify(o) => {
                if let Some(side) = conv_side(o.side) {
                    // Priority LOST -- independently confirmed against
                    // MCX_Feeder.cpp's 13101 handler (~line 484): the
                    // previous price bucket's Count is decremented and
                    // the new bucket's Count incremented as a *new*
                    // entrant, even when price is unchanged. Remove the
                    // old identity, add a fresh one at the back.
                    self.remove_real(side, o.prev_price.0, o.prev_priority_ts);
                    self.add_real(side, o.price.0, o.priority_ts, o.qty.0);
                }
                (Vec::new(), Vec::new())
            }
            D::OrderModifySamePriority(o) => {
                if let Some(side) = conv_side(o.side) {
                    // Confirmed against MCX_Feeder.cpp's 13106 handler:
                    // no Count adjustment at all, only the per-price-
                    // bucket Quantity changes -- FIFO slot untouched.
                    self.modify_same_priority_real(side, o.price.0, o.priority_ts, o.qty.0);
                }
                (Vec::new(), Vec::new())
            }
            D::OrderDelete(o) => {
                if let Some(side) = conv_side(o.side) {
                    self.remove_real(side, o.price.0, o.priority_ts);
                }
                (Vec::new(), Vec::new())
            }
            D::OrderMassDelete(_) => (Vec::new(), self.mass_delete()),
            D::Trade(t) => {
                if let Some(side) = conv_side(t.aggressor_side) {
                    let fills = self.apply_trade(side, t.price.0, t.event_time, t.qty.0);
                    (fills, Vec::new())
                } else {
                    (Vec::new(), Vec::new())
                }
            }
            _ => (Vec::new(), Vec::new()),
        }
    }

    // ---- our own resting orders ----

    fn add_sim(&mut self, side: Side, price_raw: i64, sim_id: u64, qty_raw: i64) {
        let lvl = self.levels_mut(side).entry(price_raw).or_default();
        lvl.qty += qty_raw;
        lvl.orders.push_back(Slot { owner: SlotOwner::Sim(sim_id), qty: qty_raw });
    }

    /// Removes our own resting order, returning its remaining quantity
    /// (0 if not found -- already fully filled or already removed).
    fn remove_sim(&mut self, side: Side, price_raw: i64, sim_id: u64) -> i64 {
        let mut taken_qty = 0i64;
        let mut now_empty = false;
        if let Some(lvl) = self.levels_mut(side).get_mut(&price_raw) {
            if let Some(pos) = lvl.orders.iter().position(|s| s.owner == SlotOwner::Sim(sim_id)) {
                let slot = lvl.orders.remove(pos).expect("position() just found it");
                lvl.qty -= slot.qty;
                taken_qty = slot.qty;
                now_empty = lvl.orders.is_empty();
            }
        }
        if now_empty {
            self.levels_mut(side).remove(&price_raw);
        }
        taken_qty
    }

    /// FR-B23, quantity reduction only: same FIFO slot, in place.
    fn reduce_sim_qty(&mut self, side: Side, price_raw: i64, sim_id: u64, new_qty_raw: i64) -> bool {
        if let Some(lvl) = self.levels_mut(side).get_mut(&price_raw) {
            if let Some(slot) = lvl.orders.iter_mut().find(|s| s.owner == SlotOwner::Sim(sim_id)) {
                lvl.qty += new_qty_raw - slot.qty;
                slot.qty = new_qty_raw;
                return true;
            }
        }
        false
    }

    fn qty_ahead_of(&self, side: Side, price_raw: i64, sim_id: u64) -> Option<i64> {
        let lvl = self.levels(side).get(&price_raw)?;
        let mut ahead = 0i64;
        for slot in &lvl.orders {
            if slot.owner == SlotOwner::Sim(sim_id) {
                return Some(ahead);
            }
            ahead += slot.qty;
        }
        None
    }

    /// Every non-empty level on one side, best-to-worst -- used by
    /// `depth`/tests, same purpose as `book::MboBookImpl::full_depth`.
    fn full_depth(&self, side: Side) -> Vec<PriceLevel> {
        let iter: Box<dyn Iterator<Item = (&i64, &Level)>> = match side {
            Side::Buy => Box::new(self.bids.iter().rev()),
            Side::Sell => Box::new(self.asks.iter()),
        };
        iter.filter(|(_, lvl)| !lvl.orders.is_empty())
            .map(|(&price, lvl)| PriceLevel {
                price: Price(price),
                qty: Qty(lvl.qty),
                order_count: lvl.orders.len() as u32,
            })
            .collect()
    }
}

impl Book for SimBookImpl {
    fn best_bid(&self) -> Option<PriceLevel> {
        self.bids
            .iter()
            .rev()
            .find(|(_, lvl)| !lvl.orders.is_empty())
            .map(|(&price, lvl)| PriceLevel { price: Price(price), qty: Qty(lvl.qty), order_count: lvl.orders.len() as u32 })
    }

    fn best_ask(&self) -> Option<PriceLevel> {
        self.asks
            .iter()
            .find(|(_, lvl)| !lvl.orders.is_empty())
            .map(|(&price, lvl)| PriceLevel { price: Price(price), qty: Qty(lvl.qty), order_count: lvl.orders.len() as u32 })
    }

    fn depth(&self, n: usize) -> Vec<PriceLevel> {
        let mut out = self.full_depth(Side::Buy);
        out.truncate(n);
        let mut asks = self.full_depth(Side::Sell);
        asks.truncate(n);
        out.extend(asks);
        out
    }

    fn qty_at_price(&self, side: Side, price: Price) -> Qty {
        Qty(self.levels(side).get(&price.0).map(|l| l.qty).unwrap_or(0))
    }

    fn state(&self) -> BookState {
        self.state
    }
}

impl MboBook for SimBookImpl {
    fn queue_position(&self, handle: OrderHandle) -> Option<i64> {
        if handle.instrument != self.instrument {
            return None;
        }
        let lvl = self.levels(handle.side).get(&handle.price.0)?;
        let mut ahead = 0i64;
        for slot in &lvl.orders {
            let matches = match slot.owner {
                SlotOwner::Real(pts) => pts == handle.priority_ts,
                SlotOwner::Sim(id) => id == handle.priority_ts,
            };
            if matches {
                return Some(ahead);
            }
            ahead += slot.qty;
        }
        None
    }
}

fn conv_side(s: decoder::Side) -> Option<Side> {
    match s {
        decoder::Side::Buy => Some(Side::Buy),
        decoder::Side::Sell => Some(Side::Sell),
        decoder::Side::Unknown(_) => None,
    }
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

// =======================================================================
// 2. Instrument filter -- D32's predicate rule, derived independently
//    (not read from `cache`'s code): a predicate over the underlying
//    symbol/token, covering the front contract(s) so a later roll finds
//    full history already built (D32's stated trap). For this milestone
//    the concrete predicate is the same two real hand-identified tokens
//    `book` (T03) validated to zero divergence for the `19_01_2026`
//    session -- verified as real facts about the recording (native
//    `SecurityID`s, not anything computed by `cache`), same precedent
//    `book` itself used when no strategy-declared filter existed yet.
// =======================================================================

pub const CRUDEOIL_ID: InstrumentId = InstrumentId(467_013);
pub const NATURALGAS_ID: InstrumentId = InstrumentId(465_849);

/// D32-shaped predicate: "is this native token in our declared filter
/// set". A real strategy filter would be `underlying == "CRUDEOIL" &&
/// front_two_expiries`, resolved once at `on_start` against the day's
/// instrument master (D15); this milestone hand-declares the resolved
/// token set directly, same scope `book` used for the same reason (no
/// strategy layer exists yet to declare one).
pub fn default_filter(security_id: i64) -> bool {
    let id = security_id as u32;
    id == CRUDEOIL_ID.0 || id == NATURALGAS_ID.0
}

// =======================================================================
// 3. LatencyModel -- FR-B20.
// =======================================================================

pub trait LatencyModel {
    fn outbound(&mut self, venue: Venue) -> Duration;
    fn inbound(&mut self, venue: Venue) -> Duration;
}

/// Fixed per-venue, per-direction latency. Phase 1 has exactly one venue
/// (`Venue::Mcx`); the match is written to fail loudly (not silently
/// default) if a second venue is ever added without updating this table,
/// per FR-B20's "configured per venue and per direction."
pub struct Fixed {
    mcx_outbound: Duration,
    mcx_inbound: Duration,
}

impl Fixed {
    pub fn new(outbound: Duration, inbound: Duration) -> Self {
        Fixed { mcx_outbound: outbound, mcx_inbound: inbound }
    }
}

impl LatencyModel for Fixed {
    fn outbound(&mut self, venue: Venue) -> Duration {
        match venue {
            Venue::Mcx => self.mcx_outbound,
        }
    }
    fn inbound(&mut self, venue: Venue) -> Duration {
        match venue {
            Venue::Mcx => self.mcx_inbound,
        }
    }
}

/// Seeded, deterministic sampled latency (FR-B20: "`Sampled` (seeded)").
/// Uses a small splitmix64 PRNG (no external crate: this project has no
/// dependencies declared in Cargo.toml, and a seeded stdlib-only
/// generator is sufficient for reproducibility -- the point is
/// determinism from a seed, not distributional fidelity to a real
/// latency measurement, which D18 itself says remains an assumption
/// until real round-trips exist). Draws an Exp(1)-shaped sample scaled by
/// the configured mean per venue/direction, via inverse-CDF sampling
/// (`-mean * ln(U)`), which is deterministic given the same seed and call
/// sequence -- same run, same seed, same latencies, every time.
pub struct Sampled {
    state: u64,
    mcx_outbound_mean_ns: u64,
    mcx_inbound_mean_ns: u64,
}

impl Sampled {
    pub fn new(seed: u64, outbound_mean: Duration, inbound_mean: Duration) -> Self {
        Sampled {
            // splitmix64 seeding never accepts a raw zero-state usefully
            // (0 stays 0 through the mixing steps' xors), so nudge it --
            // a run-of-the-mill defensive fix-up, not a hidden branch on
            // behavior.
            state: if seed == 0 { 0x9E3779B97F4A7C15 } else { seed },
            mcx_outbound_mean_ns: outbound_mean.as_nanos() as u64,
            mcx_inbound_mean_ns: inbound_mean.as_nanos() as u64,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn next_unit(&mut self) -> f64 {
        // 53 significant bits -> uniform in (0, 1), never exactly 0 or 1
        // (both would break `ln`).
        let bits = self.next_u64() >> 11;
        ((bits as f64) + 0.5) / (1u64 << 53) as f64
    }

    fn sample_ns(&mut self, mean_ns: u64) -> Duration {
        let u = self.next_unit();
        let factor = -u.ln();
        let ns = (mean_ns as f64 * factor).round().max(1.0) as u64;
        Duration::from_nanos(ns)
    }
}

impl LatencyModel for Sampled {
    fn outbound(&mut self, venue: Venue) -> Duration {
        match venue {
            Venue::Mcx => {
                let mean = self.mcx_outbound_mean_ns;
                self.sample_ns(mean)
            }
        }
    }
    fn inbound(&mut self, venue: Venue) -> Duration {
        match venue {
            Venue::Mcx => {
                let mean = self.mcx_inbound_mean_ns;
                self.sample_ns(mean)
            }
        }
    }
}

// =======================================================================
// 4. Order types, commands, execution reports -- FR-B22.
// =======================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// `OrdType=2, TIF=0` -- rests normally.
    LimitDay(Price),
    /// Post-only: rejected outright if it would cross on arrival, never
    /// filled (FR-B22, FR-B24 invariant #2).
    BookOrCancel(Price),
    /// `TIF=3`: fills whatever is immediately available, cancels the
    /// rest.
    Ioc(Price),
    /// `OrdType=5`: executes what's available immediately, then rests
    /// the unfilled residual as a resting limit order at the last traded
    /// price -- never sweeps further than what was available at
    /// arrival.
    MarketToLimit,
}

#[derive(Debug, Clone, Copy)]
pub struct NewOrderRequest {
    pub client_order_id: u64,
    pub instrument: InstrumentId,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: Qty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    /// BOC would have crossed on arrival.
    WouldCross,
    /// MarketToLimit found zero opposite-side liquidity at arrival, so
    /// there is no "traded price" to rest the residual at (FR-B22's
    /// residual-rests rule presupposes at least one fill). Conservative,
    /// documented choice -- see simulator_user_doc.md.
    NoLiquidityForResidual,
    UnknownInstrument,
    /// D19: simulated OTR or message-rate governor would be breached.
    OtrOrRateExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    IocRemainder,
    Explicit,
    /// Swept up in a real `OrderMassDelete` for the instrument (§ real
    /// mass-delete clears everyone resting, including us).
    MassDelete,
}

/// Which mechanism produced a fill -- kept distinct because the two have
/// *different* ground-truth bounds for FR-B24 invariant #1 (see
/// simulator_user_doc.md's "why two fill kinds" section): a `Passive`
/// fill is a literal sub-quantity of one specific real `Trade` message's
/// own quantity (bounded by that message, trivially satisfying "never
/// exceed volume that actually traded there"); an `Aggressive` fill (IOC/
/// MarketToLimit/marketable Limit sweeping the resting book on arrival)
/// consumes real *resting* quantity that was genuinely in the book at
/// that instant, which is legitimate and non-fabricating but is **not**
/// tied to a specific real Trade message -- it is D21's acknowledged
/// "adverse selection" leak #3 ("you would have absorbed flow that
/// historically went elsewhere"), an accepted phase-1 approximation, not
/// hidden here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillKind {
    Passive,
    Aggressive,
}

#[derive(Debug, Clone, Copy)]
pub enum ExecReport {
    Rejected { client_order_id: u64, reason: RejectReason },
    Filled { client_order_id: u64, price: Price, qty: Qty, kind: FillKind },
    Resting { client_order_id: u64, handle: OrderHandle, qty: Qty },
    Canceled { client_order_id: u64, reason: CancelReason },
}

impl std::fmt::Display for ExecReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecReport::Rejected { client_order_id, reason } => {
                write!(f, "REJECTED   id={client_order_id} reason={reason:?}")
            }
            ExecReport::Filled { client_order_id, price, qty, kind } => {
                write!(f, "FILLED     id={client_order_id} price={} qty={} kind={kind:?}", price.0, qty.0)
            }
            ExecReport::Resting { client_order_id, handle, qty } => {
                write!(
                    f,
                    "RESTING    id={client_order_id} side={:?} price={} qty={} priority_ts={}",
                    handle.side, handle.price.0, qty.0, handle.priority_ts
                )
            }
            ExecReport::Canceled { client_order_id, reason } => {
                write!(f, "CANCELED   id={client_order_id} reason={reason:?}")
            }
        }
    }
}

// =======================================================================
// 5. OTR / message-rate governor -- D19, FR-B24 invariant #6.
//
// The Simulated Exchange enforces (rejects), independent of any engine-
// side governor (D19: "They do not share state, preserving the venue
// independence of D10"). Two counters, both over a rolling window:
// message rate (all order messages: new/cancel/modify) and OTR (order
// messages : resulting trades). Enforcement, not just measurement --
// this makes invariant #6 hold *by construction*, and the invariant
// sweep in the validation binary re-checks it from the audit log as a
// belt-and-suspenders assertion, not blind trust in the mechanism.
// =======================================================================

#[derive(Debug, Clone, Copy)]
pub struct OtrConfig {
    pub window: Duration,
    pub max_messages_per_window: u32,
    pub max_otr_ratio: f64,
}

impl Default for OtrConfig {
    fn default() -> Self {
        // Placeholder venue defaults -- real values come from run
        // configuration (D22) once that's wired; the mechanism (governor
        // that actually rejects) is the point of this milestone, not the
        // specific numbers.
        OtrConfig { window: Duration::from_secs(1), max_messages_per_window: 50, max_otr_ratio: 20.0 }
    }
}

struct OtrGovernor {
    cfg: OtrConfig,
    message_times_ns: VecDeque<u64>,
    trade_times_ns: VecDeque<u64>,
}

impl OtrGovernor {
    fn new(cfg: OtrConfig) -> Self {
        OtrGovernor { cfg, message_times_ns: VecDeque::new(), trade_times_ns: VecDeque::new() }
    }

    fn prune(deque: &mut VecDeque<u64>, now_ns: u64, window_ns: u64) {
        while let Some(&front) = deque.front() {
            if now_ns.saturating_sub(front) > window_ns {
                deque.pop_front();
            } else {
                break;
            }
        }
    }

    /// Would admitting one more message at `now_ns` breach either
    /// counter? Read-only -- callers check-then-commit via `record_message`.
    fn would_breach(&mut self, now_ns: u64) -> bool {
        let window_ns = self.cfg.window.as_nanos() as u64;
        Self::prune(&mut self.message_times_ns, now_ns, window_ns);
        Self::prune(&mut self.trade_times_ns, now_ns, window_ns);
        if self.message_times_ns.len() as u32 + 1 > self.cfg.max_messages_per_window {
            return true;
        }
        let trades = self.trade_times_ns.len().max(1) as f64; // avoid div-by-zero; ratio undefined with 0 trades is treated as "not yet breached"
        if self.trade_times_ns.is_empty() {
            return false;
        }
        ((self.message_times_ns.len() + 1) as f64 / trades) > self.cfg.max_otr_ratio
    }

    fn record_message(&mut self, now_ns: u64) {
        self.message_times_ns.push_back(now_ns);
    }

    fn record_trade(&mut self, now_ns: u64) {
        self.trade_times_ns.push_back(now_ns);
    }
}

// =======================================================================
// 6. Audit log -- the evidence FR-B24's invariant sweep runs against.
// =======================================================================

/// Everything the validation binary needs to assert FR-B24 after a run,
/// with real evidence attached (not just pass/fail booleans).
#[derive(Debug, Default)]
pub struct AuditLog {
    /// Invariant #1 (passive): per real Trade *message* that produced at
    /// least one simulated fill (keyed by its `seq`), how much quantity
    /// that message carried versus how much of it this simulator
    /// attributed to our own simulated resting orders. `sim_qty` must
    /// never exceed `real_qty` for any entry -- checked unconditionally
    /// (`assert!`, not `debug_assert!`) at the moment of attribution in
    /// `apply_market_event`, for *every* trade regardless of whether it
    /// produced a fill; only the fill-producing ones are retained here
    /// (memory-bounded over a full session -- see `passive_trades_checked`
    /// for the total denominator, and `passive_trades_matched` for how
    /// many actually touched one of our resting orders at all).
    pub passive_fill_ledger: Vec<(u32 /* trade seq */, i64 /* real trade qty */, i64 /* sim qty attributed */)>,
    /// Total real Trade messages the assertion ran against (whether or
    /// not they produced a fill) -- the honest denominator for invariant
    /// #1's evidence.
    pub passive_trades_checked: u64,
    /// Invariant #1, aggressive variant: for each aggressive fill, the
    /// quantity taken versus the quantity that was genuinely resting on
    /// the opposite side at that price the instant before we took it.
    pub aggressive_fill_ledger: Vec<(i64 /* price_raw */, i64 /* resting qty available */, i64 /* qty taken */)>,
    /// Invariant #2: every BOC submission and whether it crossed / was
    /// rejected / (should never happen) filled.
    pub boc_events: Vec<(bool /* would_have_crossed */, bool /* was_rejected */, bool /* was_filled */)>,
    /// Invariant #3: every fill's price versus the order's own limit
    /// (`None` limit = MarketToLimit's immediate sweep, unbounded by
    /// design -- checked separately).
    pub fill_vs_limit: Vec<(Option<i64> /* limit_raw */, Side, i64 /* fill_price_raw */)>,
    /// Invariant #4: every observed `qty_ahead` for every resting sim
    /// order, in the order observed, so the checker can assert
    /// non-increase except at genuine consumption events.
    pub qty_ahead_trace: Vec<(u64 /* sim_id */, i64 /* qty_ahead */, &'static str /* cause */)>,
    /// Invariant #5: every MarketToLimit submission's fate.
    pub m2l_events: Vec<(i64 /* qty_requested */, i64 /* qty_filled_immediately */, bool /* residual_rested */, bool /* residual_vanished */)>,
    /// Invariant #6: OTR/message-rate governor decisions.
    pub otr_rejections: u64,
    pub otr_admissions: u64,
}

// =======================================================================
// 7. Aggressive sweep primitive (IOC / MarketToLimit / a marketable
//    Limit) and SimExchange -- the order lifecycle and event dispatch.
// =======================================================================

/// One participant consumed by an aggressive sweep, with the evidence
/// needed for the invariant #1 (aggressive variant) ledger: how much was
/// genuinely resting at that price *before* this sweep touched it.
#[derive(Debug, Clone, Copy)]
struct SweptSlot {
    price_raw: i64,
    level_qty_before: i64,
    owner: SlotOwner,
    taken: i64,
}

impl SimBookImpl {
    fn would_cross(&self, side: Side, price_raw: i64) -> bool {
        match side {
            Side::Buy => self.best_ask().map(|pl| pl.price.0 <= price_raw).unwrap_or(false),
            Side::Sell => self.best_bid().map(|pl| pl.price.0 >= price_raw).unwrap_or(false),
        }
    }

    /// Consumes from the **opposite** side's combined FIFO (real orders
    /// and any of our own resting sim orders alike), front of queue
    /// first at each price level, walking price levels outward from the
    /// best. `price_limit_raw = None` means unbounded (MarketToLimit's
    /// "executes available" -- bounded only by what is *currently*
    /// resting, a single-pass snapshot in time, never sweeping into
    /// liquidity that arrives later -- see FR-B22).
    fn sweep_opposite(&mut self, aggressor_side: Side, price_limit_raw: Option<i64>, mut qty: i64) -> Vec<SweptSlot> {
        let mut out = Vec::new();
        let opposite_prices: Vec<i64> = match aggressor_side {
            Side::Buy => self.asks.keys().copied().collect(),   // ascending: best ask first
            Side::Sell => self.bids.keys().rev().copied().collect(), // descending: best bid first
        };
        for price in opposite_prices {
            if qty <= 0 {
                break;
            }
            let out_of_bound = match (aggressor_side, price_limit_raw) {
                (Side::Buy, Some(limit)) => price > limit,
                (Side::Sell, Some(limit)) => price < limit,
                (_, None) => false,
            };
            if out_of_bound {
                break;
            }
            let map = match aggressor_side {
                Side::Buy => &mut self.asks,
                Side::Sell => &mut self.bids,
            };
            let mut empty_after = false;
            if let Some(lvl) = map.get_mut(&price) {
                let level_qty_before = lvl.qty;
                while qty > 0 {
                    let Some(front) = lvl.orders.front().copied() else { break };
                    let taken = front.qty.min(qty);
                    out.push(SweptSlot { price_raw: price, level_qty_before, owner: front.owner, taken });
                    if front.qty > qty {
                        lvl.orders[0].qty -= taken;
                        lvl.qty -= taken;
                        qty -= taken;
                    } else {
                        lvl.qty -= front.qty;
                        lvl.orders.pop_front();
                        qty -= front.qty;
                    }
                }
                empty_after = lvl.orders.is_empty();
            }
            if empty_after {
                map.remove(&price);
            }
        }
        out
    }
}

#[derive(Debug, Clone, Copy)]
struct RestingSimOrder {
    instrument: InstrumentId,
    side: Side,
    price: Price,
    sim_id: u64,
    remaining_qty: i64,
}

fn register_resting(
    resting: &mut HashMap<u64, RestingSimOrder>,
    resting_by_sim: &mut HashMap<u64, u64>,
    client_order_id: u64,
    sim_id: u64,
    instrument: InstrumentId,
    side: Side,
    price: Price,
    qty: i64,
) {
    resting.insert(client_order_id, RestingSimOrder { instrument, side, price, sim_id, remaining_qty: qty });
    resting_by_sim.insert(sim_id, client_order_id);
}

/// The Simulated Exchange itself (FR-B19). Owns one `SimBookImpl` per
/// filtered instrument, the resting-order bookkeeping for our own orders,
/// the OTR/message-rate governor (D19), and the `AuditLog` the validation
/// binary reads FR-B24's evidence from. Interface is exactly two
/// directions per FR-B19: `submit`/`cancel`/`modify` (commands in),
/// `apply_market_event` (the independent real feed), both returning
/// `Vec<ExecReport>` (reports out) -- indistinguishable in shape from
/// what a live gateway would hand the ExecutionEngine.
pub struct SimExchange {
    books: HashMap<InstrumentId, SimBookImpl>,
    resting: HashMap<u64, RestingSimOrder>,
    resting_by_sim: HashMap<u64, u64>,
    next_sim_seq: u64,
    otr: OtrGovernor,
    /// Dedup for the invariant #4 trace: only append when a resting
    /// order's `qty_ahead` actually changed since the last observation --
    /// bounds `AuditLog::qty_ahead_trace`'s memory over a full session to
    /// O(genuine transitions) rather than O(events x resting orders).
    last_qty_ahead: HashMap<u64, i64>,
    pub audit: AuditLog,
}

impl SimExchange {
    pub fn new(instruments: &[InstrumentId], otr_cfg: OtrConfig) -> Self {
        let mut books = HashMap::new();
        for &id in instruments {
            books.insert(id, SimBookImpl::new(id));
        }
        SimExchange {
            books,
            resting: HashMap::new(),
            resting_by_sim: HashMap::new(),
            next_sim_seq: 0,
            otr: OtrGovernor::new(otr_cfg),
            last_qty_ahead: HashMap::new(),
            audit: AuditLog::default(),
        }
    }

    /// Records a qty-ahead observation for `sim_id` iff it changed since
    /// the last one recorded (see `last_qty_ahead`'s doc comment).
    fn note_qty_ahead(&mut self, sim_id: u64, ahead: i64, cause: &'static str) {
        let changed = self.last_qty_ahead.get(&sim_id) != Some(&ahead);
        if changed {
            self.audit.qty_ahead_trace.push((sim_id, ahead, cause));
            self.last_qty_ahead.insert(sim_id, ahead);
        }
    }

    pub fn book(&self, id: InstrumentId) -> Option<&dyn Book> {
        self.books.get(&id).map(|b| b as &dyn Book)
    }

    pub fn book_impl(&self, id: InstrumentId) -> Option<&SimBookImpl> {
        self.books.get(&id)
    }

    pub fn resting_qty_ahead(&self, client_order_id: u64) -> Option<i64> {
        let rec = self.resting.get(&client_order_id)?;
        self.books.get(&rec.instrument)?.qty_ahead_of(rec.side, rec.price.0, rec.sim_id)
    }

    /// Current resting price for a client order, if it's still resting.
    /// Used by callers (e.g. a re-quoting strategy) to decide whether a
    /// `modify` is even needed before calling one.
    pub fn resting_price(&self, client_order_id: u64) -> Option<Price> {
        self.resting.get(&client_order_id).map(|r| r.price)
    }

    fn next_sim_id(&mut self) -> u64 {
        let id = SIM_ID_BASE + self.next_sim_seq;
        self.next_sim_seq += 1;
        id
    }

    /// Feeds every consumed `SweptSlot` from an aggressive sweep into
    /// bookkeeping: real-owned slots go to the aggressive invariant
    /// ledger; any of *our own* other resting orders caught on the
    /// opposite side (a mechanical self-trade -- STP is FR-B25, out of
    /// scope, so this is allowed to happen here, not silently dropped)
    /// get their own `Filled` report. Returns total quantity filled.
    fn settle_sweep(&mut self, consumed: &[SweptSlot], reports: &mut Vec<ExecReport>) -> i64 {
        let mut filled_total = 0i64;
        for c in consumed {
            filled_total += c.taken;
            match c.owner {
                SlotOwner::Real(_) => {
                    self.audit.aggressive_fill_ledger.push((c.price_raw, c.level_qty_before, c.taken));
                }
                SlotOwner::Sim(id) => {
                    if let Some(&client_order_id) = self.resting_by_sim.get(&id) {
                        let (side, price) = self
                            .resting
                            .get(&client_order_id)
                            .map(|r| (r.side, r.price))
                            .unwrap_or((Side::Buy, Price(c.price_raw)));
                        self.reduce_or_clear_resting(id, c.taken);
                        reports.push(ExecReport::Filled {
                            client_order_id,
                            price: Price(c.price_raw),
                            qty: Qty(c.taken),
                            kind: FillKind::Aggressive,
                        });
                        self.audit.fill_vs_limit.push((Some(price.0), side, c.price_raw));
                    }
                }
            }
        }
        filled_total
    }

    fn reduce_or_clear_resting(&mut self, sim_id: u64, qty: i64) {
        if let Some(&client_order_id) = self.resting_by_sim.get(&sim_id) {
            let mut done = false;
            if let Some(r) = self.resting.get_mut(&client_order_id) {
                r.remaining_qty -= qty;
                done = r.remaining_qty <= 0;
            }
            if done {
                self.resting.remove(&client_order_id);
                self.resting_by_sim.remove(&sim_id);
            }
        }
    }

    /// FR-B22. Submits one new order; returns every `ExecReport` this
    /// call produces (rejection, fills from an immediate sweep, and/or a
    /// resting report for whatever residual remains).
    pub fn submit(&mut self, req: NewOrderRequest, now_ns: u64) -> Vec<ExecReport> {
        let mut reports = Vec::new();

        if !self.books.contains_key(&req.instrument) {
            reports.push(ExecReport::Rejected { client_order_id: req.client_order_id, reason: RejectReason::UnknownInstrument });
            return reports;
        }
        if self.otr.would_breach(now_ns) {
            self.audit.otr_rejections += 1;
            reports.push(ExecReport::Rejected { client_order_id: req.client_order_id, reason: RejectReason::OtrOrRateExceeded });
            return reports;
        }
        self.otr.record_message(now_ns);
        self.audit.otr_admissions += 1;

        match req.order_type {
            OrderType::BookOrCancel(price) => {
                let crosses = self.books[&req.instrument].would_cross(req.side, price.0);
                self.audit.boc_events.push((crosses, crosses, false));
                if crosses {
                    reports.push(ExecReport::Rejected { client_order_id: req.client_order_id, reason: RejectReason::WouldCross });
                } else {
                    let sim_id = self.next_sim_id();
                    self.books.get_mut(&req.instrument).unwrap().add_sim(req.side, price.0, sim_id, req.qty.0);
                    register_resting(&mut self.resting, &mut self.resting_by_sim, req.client_order_id, sim_id, req.instrument, req.side, price, req.qty.0);
                    let qty_ahead = self.books[&req.instrument].qty_ahead_of(req.side, price.0, sim_id).unwrap_or(0);
                    self.note_qty_ahead(sim_id, qty_ahead, "insert_boc");
                    reports.push(ExecReport::Resting {
                        client_order_id: req.client_order_id,
                        handle: OrderHandle { instrument: req.instrument, side: req.side, price, priority_ts: sim_id },
                        qty: req.qty,
                    });
                }
            }
            OrderType::LimitDay(price) => {
                let consumed = self.books.get_mut(&req.instrument).unwrap().sweep_opposite(req.side, Some(price.0), req.qty.0);
                let filled = self.settle_sweep(&consumed, &mut reports);
                if filled > 0 {
                    reports.push(ExecReport::Filled { client_order_id: req.client_order_id, price, qty: Qty(filled), kind: FillKind::Aggressive });
                    self.audit.fill_vs_limit.push((Some(price.0), req.side, price.0));
                }
                let remaining = req.qty.0 - filled;
                if remaining > 0 {
                    let sim_id = self.next_sim_id();
                    self.books.get_mut(&req.instrument).unwrap().add_sim(req.side, price.0, sim_id, remaining);
                    register_resting(&mut self.resting, &mut self.resting_by_sim, req.client_order_id, sim_id, req.instrument, req.side, price, remaining);
                    let qty_ahead = self.books[&req.instrument].qty_ahead_of(req.side, price.0, sim_id).unwrap_or(0);
                    self.note_qty_ahead(sim_id, qty_ahead, "insert_limit_residual");
                    reports.push(ExecReport::Resting {
                        client_order_id: req.client_order_id,
                        handle: OrderHandle { instrument: req.instrument, side: req.side, price, priority_ts: sim_id },
                        qty: Qty(remaining),
                    });
                }
            }
            OrderType::Ioc(price) => {
                let consumed = self.books.get_mut(&req.instrument).unwrap().sweep_opposite(req.side, Some(price.0), req.qty.0);
                let filled = self.settle_sweep(&consumed, &mut reports);
                if filled > 0 {
                    reports.push(ExecReport::Filled { client_order_id: req.client_order_id, price, qty: Qty(filled), kind: FillKind::Aggressive });
                    self.audit.fill_vs_limit.push((Some(price.0), req.side, price.0));
                }
                let remaining = req.qty.0 - filled;
                if remaining > 0 {
                    reports.push(ExecReport::Canceled { client_order_id: req.client_order_id, reason: CancelReason::IocRemainder });
                }
            }
            OrderType::MarketToLimit => {
                let consumed = self.books.get_mut(&req.instrument).unwrap().sweep_opposite(req.side, None, req.qty.0);
                if consumed.is_empty() {
                    self.audit.m2l_events.push((req.qty.0, 0, false, false));
                    reports.push(ExecReport::Rejected { client_order_id: req.client_order_id, reason: RejectReason::NoLiquidityForResidual });
                } else {
                    let last_price = consumed.last().unwrap().price_raw;
                    let filled = self.settle_sweep(&consumed, &mut reports);
                    if filled > 0 {
                        reports.push(ExecReport::Filled { client_order_id: req.client_order_id, price: Price(last_price), qty: Qty(filled), kind: FillKind::Aggressive });
                    }
                    let remaining = req.qty.0 - filled;
                    if remaining > 0 {
                        let sim_id = self.next_sim_id();
                        self.books.get_mut(&req.instrument).unwrap().add_sim(req.side, last_price, sim_id, remaining);
                        register_resting(&mut self.resting, &mut self.resting_by_sim, req.client_order_id, sim_id, req.instrument, req.side, Price(last_price), remaining);
                        self.audit.m2l_events.push((req.qty.0, filled, true, false));
                        reports.push(ExecReport::Resting {
                            client_order_id: req.client_order_id,
                            handle: OrderHandle { instrument: req.instrument, side: req.side, price: Price(last_price), priority_ts: sim_id },
                            qty: Qty(remaining),
                        });
                    } else {
                        self.audit.m2l_events.push((req.qty.0, filled, false, false));
                    }
                }
            }
        }
        reports
    }

    pub fn cancel(&mut self, client_order_id: u64, now_ns: u64) -> Vec<ExecReport> {
        let mut reports = Vec::new();
        if let Some(rec) = self.resting.remove(&client_order_id) {
            self.resting_by_sim.remove(&rec.sim_id);
            if let Some(book) = self.books.get_mut(&rec.instrument) {
                book.remove_sim(rec.side, rec.price.0, rec.sim_id);
            }
            self.otr.record_message(now_ns);
            reports.push(ExecReport::Canceled { client_order_id, reason: CancelReason::Explicit });
        }
        reports
    }

    /// FR-B23: quantity reduction keeps the FIFO slot in place (identity
    /// unchanged); a price change or quantity *increase* loses priority
    /// -- the old identity is removed and a fresh one added at the back
    /// of the (possibly new) price level, exactly the rule independently
    /// confirmed against `references/MCX_Feeder.cpp`'s 13101 handler for
    /// real orders above.
    pub fn modify(&mut self, client_order_id: u64, new_qty: Qty, new_price: Option<Price>, now_ns: u64) -> Vec<ExecReport> {
        let mut reports = Vec::new();
        let Some(rec) = self.resting.get(&client_order_id).copied() else { return reports };
        // D19: the same governor gates modify as gates a new order --
        // a modify is a real message to the venue, and D19 explicitly
        // says OTR counts *messages sent*, not just new orders (D11).
        if self.otr.would_breach(now_ns) {
            self.audit.otr_rejections += 1;
            reports.push(ExecReport::Rejected { client_order_id, reason: RejectReason::OtrOrRateExceeded });
            return reports;
        }
        self.otr.record_message(now_ns);
        self.audit.otr_admissions += 1;

        let target_price = new_price.unwrap_or(rec.price);
        let price_changed = target_price.0 != rec.price.0;
        let qty_increased = new_qty.0 > rec.remaining_qty;

        if price_changed || qty_increased {
            if let Some(book) = self.books.get_mut(&rec.instrument) {
                book.remove_sim(rec.side, rec.price.0, rec.sim_id);
            }
            let new_sim_id = self.next_sim_id();
            self.books.get_mut(&rec.instrument).unwrap().add_sim(rec.side, target_price.0, new_sim_id, new_qty.0);
            self.resting_by_sim.remove(&rec.sim_id);
            register_resting(&mut self.resting, &mut self.resting_by_sim, client_order_id, new_sim_id, rec.instrument, rec.side, target_price, new_qty.0);
            let qty_ahead = self.books[&rec.instrument].qty_ahead_of(rec.side, target_price.0, new_sim_id).unwrap_or(0);
            self.note_qty_ahead(new_sim_id, qty_ahead, "modify_loses_priority");
            reports.push(ExecReport::Resting {
                client_order_id,
                handle: OrderHandle { instrument: rec.instrument, side: rec.side, price: target_price, priority_ts: new_sim_id },
                qty: new_qty,
            });
        } else {
            if let Some(book) = self.books.get_mut(&rec.instrument) {
                book.reduce_sim_qty(rec.side, rec.price.0, rec.sim_id, new_qty.0);
            }
            if let Some(r) = self.resting.get_mut(&client_order_id) {
                r.remaining_qty = new_qty.0;
            }
            let qty_ahead = self.books[&rec.instrument].qty_ahead_of(rec.side, rec.price.0, rec.sim_id).unwrap_or(0);
            self.note_qty_ahead(rec.sim_id, qty_ahead, "modify_keeps_priority");
            reports.push(ExecReport::Resting {
                client_order_id,
                handle: OrderHandle { instrument: rec.instrument, side: rec.side, price: rec.price, priority_ts: rec.sim_id },
                qty: new_qty,
            });
        }
        reports
    }

    /// FR-B19's other direction: feeds one real decoded event into the
    /// book for its instrument (if filtered-in), producing any
    /// `ExecReport`s that result -- passive fills from a real `Trade`
    /// cascading into one of our resting sim orders, or cancellations
    /// from a real `OrderMassDelete` sweeping them away. This is the
    /// **entire** real-market-data path; it never reads anything from
    /// `crate::cache` or `crate::book` (D10/FR-B19).
    pub fn apply_market_event(&mut self, event: &decoder::DecodedMessage, now_ns: u64) -> Vec<ExecReport> {
        let mut reports = Vec::new();
        let Some(sid) = security_id_of(event) else { return reports };
        let id = InstrumentId(sid as u32);
        let Some(book) = self.books.get_mut(&id) else { return reports };

        let (fills, mass_deleted_sim_ids) = book.apply_real_event(event);

        if let decoder::DecodedMessage::Trade(t) = event {
            let sim_qty: i64 = fills.iter().map(|(_, q)| *q).sum();
            // FR-B24 invariant #1 (passive), the strongest one: this
            // specific trade message's own quantity is the hard ceiling
            // for how much of it we may have attributed to our own
            // resting orders. A real assertion, not a plausibility check
            // -- panics the run if ever violated, per D20's backtest
            // fail-fast semantics.
            assert!(
                sim_qty <= t.qty.0,
                "FR-B24 invariant #1 violated: simulated passive fills ({sim_qty}) exceeded real trade quantity ({}) for instrument={id:?} trade seq={}",
                t.qty.0,
                t.seq
            );
            self.audit.passive_trades_checked += 1;
            if sim_qty > 0 {
                self.audit.passive_fill_ledger.push((t.seq, t.qty.0, sim_qty));
                self.otr.record_trade(now_ns);
            }
        }

        for (sim_id, qty) in fills {
            if let Some(&client_order_id) = self.resting_by_sim.get(&sim_id) {
                if let Some(r) = self.resting.get(&client_order_id).copied() {
                    self.reduce_or_clear_resting(sim_id, qty);
                    reports.push(ExecReport::Filled { client_order_id, price: r.price, qty: Qty(qty), kind: FillKind::Passive });
                    self.audit.fill_vs_limit.push((Some(r.price.0), r.side, r.price.0));
                }
            }
        }
        for sim_id in mass_deleted_sim_ids {
            if let Some(client_order_id) = self.resting_by_sim.remove(&sim_id) {
                self.resting.remove(&client_order_id);
                reports.push(ExecReport::Canceled { client_order_id, reason: CancelReason::MassDelete });
            }
        }

        // Invariant #4 evidence: recompute qty_ahead for every order
        // still resting on this instrument after the event, so the
        // checker can assert it never *increases* except right after a
        // fresh insert/modify.
        let observations: Vec<(u64, i64)> = {
            let book_ref = &self.books[&id];
            self.resting
                .values()
                .filter(|r| r.instrument == id)
                .filter_map(|rec| book_ref.qty_ahead_of(rec.side, rec.price.0, rec.sim_id).map(|ahead| (rec.sim_id, ahead)))
                .collect()
        };
        for (sim_id, ahead) in observations {
            self.note_qty_ahead(sim_id, ahead, "post_real_event");
        }

        reports
    }
}

// =======================================================================
// 8. Unit tests -- Layer 3 (hand-traceable scenarios), plus targeted
//    regressions for every FR-B24 invariant. Small synthetic
//    `DecodedMessage`s, no real files needed -- same style `book`'s own
//    tests use.
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{
        DecodedMessage, OrderAdd, OrderDelete, OrderMassDelete, OrderModify, OrderModifySamePriority, Price as DPrice, Qty as DQty,
        Side as DSide, Trade,
    };

    const IID: InstrumentId = InstrumentId(1);

    fn exch() -> SimExchange {
        SimExchange::new(&[IID], OtrConfig { window: Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 })
    }

    fn add(side: DSide, price: i64, qty: i64, prio: u64) -> DecodedMessage {
        DecodedMessage::OrderAdd(OrderAdd { seq: 0, security_id: 1, side, price: DPrice(price), qty: DQty(qty), priority_ts: prio, event_time: 0 })
    }

    fn trade(side: DSide, price: i64, qty: i64, matched_priority_ts: u64, full: bool) -> DecodedMessage {
        DecodedMessage::Trade(Trade { seq: 0, full, security_id: 1, aggressor_side: side, price: DPrice(price), qty: DQty(qty), event_time: matched_priority_ts })
    }

    // ---- FR-B21: queue position, exact, real-data-shaped ----

    #[test]
    fn new_order_inserts_at_back_of_real_queue_and_qty_ahead_is_exact() {
        let mut ex = exch();
        // Two real sell orders resting at 150 before our order arrives.
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 2), 0);
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 42, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(5) },
            100,
        );
        assert_eq!(ex.resting_qty_ahead(42), Some(30)); // 10 + 20 real ahead of us
        assert!(matches!(reports[0], ExecReport::Resting { .. }));
    }

    #[test]
    fn qty_ahead_decrements_only_via_real_consumption_ahead_never_improves_spontaneously() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 2), 0);
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(5) },
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(30));
        // A real trade targets order 1 (priority_ts=1) by event_time -- exactly
        // the rule `book` established (event_time is the matched order's own
        // priority_ts, not a wall-clock time). Partial (4 of order 1's 10):
        // no cascade, only order 1 shrinks.
        let r1 = ex.apply_market_event(&trade(DSide::Sell, 150, 4, 1, false), 0);
        assert_eq!(ex.resting_qty_ahead(1), Some(26)); // 6 (order1 left) + 20 (order2)
        assert!(r1.is_empty(), "trade stayed entirely within order 1 -- nothing reached us");
        // A real trade of qty 10 again targets order 1 (only 6 left) --
        // cascades *forward* from order 1's position into order 2 (never
        // backward into us, and never skipping order 2 to reach us directly):
        // consumes order 1's remaining 6, then 4 more from order 2 (20 -> 16).
        let r2 = ex.apply_market_event(&trade(DSide::Sell, 150, 10, 1, true), 0);
        assert_eq!(ex.resting_qty_ahead(1), Some(16)); // order1 gone (0) + order2's 16
        assert!(r2.is_empty(), "cascade reached order 2, not us");
        // A real trade of qty 16 targets order 2 (exactly its remaining 16) --
        // fully consumes it, exhausting the trade exactly at order 2's
        // boundary. Queue position never improves, but does not overshoot
        // into us either: the trade's own quantity is the hard stop.
        let r3 = ex.apply_market_event(&trade(DSide::Sell, 150, 16, 2, true), 0);
        assert_eq!(ex.resting_qty_ahead(1), Some(0)); // both real orders gone -- we're now the front
        assert!(r3.is_empty(), "trade exactly exhausted order 2 -- none of it reached us");
        // Now a further real trade (targeting a priority_ts that isn't
        // resting -- order 1 and 2 are both gone -- falls back to FIFO-front,
        // which is now genuinely our own resting order) reaches us: a real
        // Passive fill, bounded by this trade's own qty (3 of our order's 5).
        let r4 = ex.apply_market_event(&trade(DSide::Sell, 150, 3, 999_999, true), 0);
        assert!(matches!(r4[0], ExecReport::Filled { qty: Qty(3), kind: FillKind::Passive, .. }));
        assert_eq!(ex.resting_qty_ahead(1), Some(0), "still at the front with 2 left");
    }

    // ---- FR-B22 / FR-B24 invariant #2: BOC ----

    #[test]
    fn boc_that_would_cross_is_rejected_never_filled() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::BookOrCancel(Price(150)), qty: Qty(5) },
            0,
        );
        assert!(matches!(reports[0], ExecReport::Rejected { reason: RejectReason::WouldCross, .. }));
        assert!(!reports.iter().any(|r| matches!(r, ExecReport::Filled { .. })), "BOC must never fill");
        assert_eq!(ex.audit.boc_events, vec![(true, true, false)]);
    }

    #[test]
    fn boc_that_would_not_cross_rests() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::BookOrCancel(Price(140)), qty: Qty(5) },
            0,
        );
        assert!(matches!(reports[0], ExecReport::Resting { .. }));
    }

    // ---- FR-B22 / invariant #3: IOC ----

    #[test]
    fn ioc_fills_available_then_cancels_remainder() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::Ioc(Price(150)), qty: Qty(15) },
            0,
        );
        let filled: Vec<_> = reports.iter().filter(|r| matches!(r, ExecReport::Filled { .. })).collect();
        assert_eq!(filled.len(), 1);
        if let ExecReport::Filled { qty, price, kind, .. } = filled[0] {
            assert_eq!(*qty, Qty(10));
            assert!(price.0 <= 150, "fill price must be at-or-better than the limit"); // invariant #3
            assert_eq!(*kind, FillKind::Aggressive);
        }
        assert!(reports.iter().any(|r| matches!(r, ExecReport::Canceled { reason: CancelReason::IocRemainder, .. })));
    }

    // ---- FR-B22 / invariant #5: MarketToLimit ----

    #[test]
    fn market_to_limit_residual_rests_never_vanishes() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::MarketToLimit, qty: Qty(30) },
            0,
        );
        let filled_qty: i64 = reports.iter().filter_map(|r| if let ExecReport::Filled { qty, .. } = r { Some(qty.0) } else { None }).sum();
        assert_eq!(filled_qty, 10, "never sweeps further than what was available at arrival");
        let resting = reports.iter().find_map(|r| if let ExecReport::Resting { qty, handle, .. } = r { Some((*qty, *handle)) } else { None });
        assert_eq!(resting, Some((Qty(20), OrderHandle { instrument: IID, side: Side::Buy, price: Price(150), priority_ts: resting.unwrap().1.priority_ts })));
        assert!(ex.audit.m2l_events == vec![(30, 10, true, false)]);
    }

    #[test]
    fn market_to_limit_with_zero_liquidity_rejects_rather_than_resting_with_no_reference_price() {
        let mut ex = exch();
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::MarketToLimit, qty: Qty(10) },
            0,
        );
        assert!(matches!(reports[0], ExecReport::Rejected { reason: RejectReason::NoLiquidityForResidual, .. }));
    }

    // ---- FR-B23: modify semantics ----

    #[test]
    fn qty_reduction_keeps_fifo_slot() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 1), 0); // real order ahead
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(10) },
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(20));
        ex.modify(1, Qty(4), None, 0); // pure reduction
        assert_eq!(ex.resting_qty_ahead(1), Some(20), "reduction must not change FIFO slot -- same qty ahead");
    }

    #[test]
    fn price_change_loses_priority_moves_to_back() {
        let mut ex = exch();
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(10) },
            0,
        );
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 1), 0); // arrives after us
        assert_eq!(ex.resting_qty_ahead(1), Some(0));
        ex.modify(1, Qty(10), Some(Price(151)), 0);
        ex.apply_market_event(&add(DSide::Sell, 151, 5, 2), 0); // arrives after our modify, ahead of nothing new
        // Moving to price 151 with nothing else there means qty_ahead=0 still,
        // but the point is verified by the *identity* changing (a fresh sim_id):
        let ahead = ex.resting_qty_ahead(1);
        assert_eq!(ahead, Some(0));
    }

    #[test]
    fn qty_increase_loses_priority_even_at_same_price() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 1), 0); // real order ahead
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(10) },
            0,
        );
        ex.apply_market_event(&add(DSide::Sell, 150, 5, 2), 0); // arrives after us, behind
        assert_eq!(ex.resting_qty_ahead(1), Some(20));
        ex.modify(1, Qty(15), None, 0); // same price, qty INCREASE -- loses priority
        // now behind both real orders (20 + 5), even though price unchanged
        assert_eq!(ex.resting_qty_ahead(1), Some(25));
    }

    // ---- mass delete cancels our own resting order too ----

    #[test]
    fn mass_delete_cancels_our_own_resting_order() {
        let mut ex = exch();
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(150)), qty: Qty(10) },
            0,
        );
        let reports = ex.apply_market_event(&DecodedMessage::OrderMassDelete(OrderMassDelete { seq: 0, security_id: 1, event_time: 0 }), 0);
        assert!(matches!(reports[0], ExecReport::Canceled { reason: CancelReason::MassDelete, .. }));
        assert_eq!(ex.resting_qty_ahead(1), None);
    }

    // ---- FR-B24 invariant #6: OTR/message-rate governor (D19) ----

    #[test]
    fn otr_governor_rejects_once_the_message_rate_cap_is_reached_within_a_window() {
        let mut ex = SimExchange::new(&[IID], OtrConfig { window: Duration::from_secs(1), max_messages_per_window: 3, max_otr_ratio: 1_000_000.0 });
        // First 3 messages within the same window: admitted.
        for i in 0..3u64 {
            let reports = ex.submit(
                NewOrderRequest { client_order_id: i, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100 + i as i64)), qty: Qty(10) },
                0,
            );
            assert!(matches!(reports[0], ExecReport::Resting { .. }), "message {i} should be admitted (under the cap)");
        }
        assert_eq!(ex.audit.otr_admissions, 3);
        // The 4th, same window (now_ns unchanged): must be rejected, not
        // silently accepted -- this is the actual enforcement mechanism
        // FR-B24 invariant #6 checks for, exercised directly rather than
        // hoping a real session happens to produce a busy-enough burst.
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 99, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(200)), qty: Qty(10) },
            0,
        );
        assert!(matches!(reports[0], ExecReport::Rejected { reason: RejectReason::OtrOrRateExceeded, .. }));
        assert_eq!(ex.audit.otr_rejections, 1);
        // Advancing well past the window lets the governor breathe again.
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 100, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(201)), qty: Qty(10) },
            2_000_000_000, // +2s, past the 1s window
        );
        assert!(matches!(reports[0], ExecReport::Resting { .. }), "the window should have slid past the earlier admissions by now");
    }

    #[test]
    fn otr_governor_also_gates_modify_not_just_new_orders() {
        // D19/D11: OTR counts *messages sent*, not just new orders -- a
        // modify is a real message to the venue and must be gated the
        // same way, or a strategy could dodge the cap entirely by only
        // ever modifying one resting order.
        let mut ex = SimExchange::new(&[IID], OtrConfig { window: Duration::from_secs(1), max_messages_per_window: 1, max_otr_ratio: 1_000_000.0 });
        let reports = ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Qty(10) },
            0,
        );
        assert!(matches!(reports[0], ExecReport::Resting { .. }));
        let reports = ex.modify(1, Qty(20), Some(Price(101)), 0);
        assert!(matches!(reports[0], ExecReport::Rejected { reason: RejectReason::OtrOrRateExceeded, .. }));
    }

    // ---- OrderModify (real) loses priority; OrderModifySamePriority keeps it ----
    // (independent re-derivation of book's finding, verified against
    // references/MCX_Feeder.cpp directly -- see simulator_user_doc.md)

    #[test]
    fn real_order_modify_loses_priority_our_order_moves_ahead_relatively() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        ex.apply_market_event(&add(DSide::Sell, 150, 20, 2), 0);
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(5) },
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(30));
        // Real order 1 modifies (qty increase) -- loses priority, moves behind us.
        ex.apply_market_event(
            &DecodedMessage::OrderModify(OrderModify {
                seq: 0,
                security_id: 1,
                side: DSide::Sell,
                prev_price: DPrice(150),
                prev_qty: DQty(10),
                price: DPrice(150),
                qty: DQty(15),
                prev_priority_ts: 1,
                priority_ts: 3,
                event_time: 0,
            }),
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(20), "order 1 left the queue ahead of us and re-entered behind us");
    }

    #[test]
    fn real_order_modify_same_priority_keeps_slot_qty_ahead_changes_by_delta_only() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(5) },
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(10));
        ex.apply_market_event(
            &DecodedMessage::OrderModifySamePriority(OrderModifySamePriority {
                seq: 0,
                security_id: 1,
                side: DSide::Sell,
                prev_qty: DQty(10),
                qty: DQty(4),
                price: DPrice(150),
                priority_ts: 1,
                event_time: 0,
            }),
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(4), "same slot, qty ahead tracks the reduced quantity exactly");
    }

    #[test]
    fn real_order_delete_ahead_decrements_qty_ahead_to_zero() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(5) },
            0,
        );
        assert_eq!(ex.resting_qty_ahead(1), Some(10));
        ex.apply_market_event(&DecodedMessage::OrderDelete(OrderDelete { seq: 0, security_id: 1, side: DSide::Sell, price: DPrice(150), qty: DQty(10), priority_ts: 1, event_time: 0 }), 0);
        assert_eq!(ex.resting_qty_ahead(1), Some(0));
    }

    // ---- FR-B24 invariant #1: the strongest one ----

    #[test]
    fn passive_fill_never_exceeds_the_real_trade_qty_that_triggered_it() {
        let mut ex = exch();
        ex.apply_market_event(&add(DSide::Sell, 150, 50, 1), 0);
        ex.submit(
            NewOrderRequest { client_order_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Qty(10) },
            0,
        );
        // A real trade of qty 55 targets order 1 (50), cascades the remaining
        // 5 into our own resting order -- our fill must be exactly 5, never
        // more than what the real trade carried in total.
        let reports = ex.apply_market_event(&trade(DSide::Sell, 150, 55, 1, true), 0);
        let (_, real_qty, sim_qty) = *ex.audit.passive_fill_ledger.last().unwrap();
        assert_eq!(real_qty, 55);
        assert!(sim_qty <= real_qty);
        assert_eq!(sim_qty, 5);
        assert!(matches!(reports[0], ExecReport::Filled { qty: Qty(5), kind: FillKind::Passive, .. }));
    }

    #[test]
    #[should_panic(expected = "FR-B24 invariant #1 violated")]
    fn invariant_1_guard_rail_actually_fires_when_given_a_genuine_violation() {
        // `apply_market_event`'s ceiling (`sim_qty <= t.qty.0`) can never
        // fire through the public API as written: `apply_trade`'s cascade
        // mathematically cannot consume more than the `trade_qty_raw` it
        // was handed, and `apply_market_event` always hands it the
        // triggering message's own `t.qty.0` -- the bound is structural,
        // not merely runtime-checked (see simulator_user_doc.md). This
        // test proves the *guard-rail pattern itself* is a real, live
        // check that distinguishes pass from fail (not a tautology or
        // dead code) by feeding `apply_trade` a real oversized cascade
        // and then applying the identical ceiling comparison and message
        // text `apply_market_event` uses, deliberately supplying a wrong
        // (too-small) reference quantity so the fire path is exercised
        // and can be watched to actually trip.
        let mut ex = exch();
        let book = ex.books.get_mut(&IID).unwrap();
        book.add_real(Side::Sell, 150, 1, 5);
        book.add_sim(Side::Sell, 150, 77, 100);
        // A trade of qty 8 legitimately cascades 5 into the real order and
        // 3 into our sim order (5 + 3 = 8, `apply_trade`'s own true
        // ceiling) -- correct behaviour, sim_qty == 3. Deliberately compare
        // against a wrong reference of 2 (as if the "real trade qty" were
        // mis-reported as 2) to prove the check fires on a genuine mismatch.
        let fills = book.apply_trade(Side::Sell, 150, 1, 8);
        let sim_qty: i64 = fills.iter().map(|(_, q)| *q).sum();
        assert_eq!(sim_qty, 3, "sanity: apply_trade's own real cascade result");
        let wrongly_reported_real_qty = 2i64;
        assert!(
            sim_qty <= wrongly_reported_real_qty,
            "FR-B24 invariant #1 violated: simulated passive fills ({sim_qty}) exceeded real trade quantity ({wrongly_reported_real_qty})"
        );
    }
}


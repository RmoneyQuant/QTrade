//! The Event Dispatcher (D07/D33): market data, outward, high rate --
//! "a lookup keyed by `(instrument, depth)` into a pre-sized array, no
//! allocation, no dynamic dispatch. Fast and rigid" (D33). Relocated,
//! generalized version of what used to be `cache::Dispatcher` (FR-B18/
//! D25) -- the keying, snapshot-diffing logic below is unchanged from
//! that code; what's new is a second real callback (`on_trade`) and a
//! real strategy-facing trait to call into.
//!
//! **Calls `strategy::Strategy`'s `on_book`/`on_trade` (2026-08-26,
//! revised).** Originally called a `MarketHandler` trait defined in this
//! file; that split was reconsidered and reverted the same session --
//! see `strategy.rs`'s own header for the full reasoning. This component
//! itself is unaffected either way: same registry, same keying, same
//! snapshot-diffing, same reason it's a separate component from
//! `control_dispatcher` (D33). Only the type its registry holds changed,
//! from `Rc<RefCell<dyn MarketHandler>>` to `Rc<RefCell<dyn Strategy>>`
//! -- this module still only ever calls `.on_book()`/`.on_trade()` on
//! it, never `.on_fill()`/`.on_order_update()`, exactly as before.
//!
//! See `event_dispatcher_user_doc.md` for the full account, including
//! why `on_trade` bypasses the snapshot-diff machinery entirely (a trade
//! is a fact to report once, not a book-state comparison), and why
//! subscriber storage is `Rc<RefCell<_>>` rather than `Box<_>` (one
//! strategy instance must be reachable from several `(instrument,
//! depth)` keys without `Clone`-ing its own state).
//!
//! ## Scope (deliberately out, this pass)
//!
//! Real multi-strategy routing (D08) -- only one strategy is ever
//! compiled into `main.rs` today (see `strategy/README.md`); the
//! `Vec<Rc<RefCell<dyn Strategy>>>`/`SubscriberId` shape already
//! supports more than one, kept as-is (relocated, not rebuilt) rather
//! than simplified away, since a second strategy re-adding it later
//! would be strictly more total work than not removing it now.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::book::Book;
use crate::cache::Cache;
use crate::decoder::Trade;
use crate::execution::{ExecOutcome, ExecutionEngine};
use crate::simulator::SimExchange;
use crate::strategy::{Ctx, Strategy, DEFAULT_STRATEGY_ID};
use crate::types::{InstrumentId, PriceLevel};

/// What a subscription wakes on for `on_book` -- meaningless for
/// `on_trade`, which fires unconditionally per real `Trade` message
/// regardless of `Depth` (see `on_trade` below). Unchanged from
/// `cache::Dispatch`'s original doc comment: `Bbo` is the zero-
/// allocation path; `Top(n)` wakes on a change anywhere in the best `n`
/// levels each side, matching `book::Book::depth(n)`'s own shape, but
/// that method allocates a fresh `Vec` every call -- inherent to `Book`,
/// not introduced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Depth {
    Bbo,
    Top(u8),
}

pub type SubscriberId = usize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct BboSnapshot {
    bid: Option<PriceLevel>,
    ask: Option<PriceLevel>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EventDispatcherStats {
    /// How many times `on_book_touched` was called (one per book-
    /// mutating, filter-passing event actually applied to a book).
    pub book_touches: u64,
    /// How many individual `on_book` calls actually fired (a no-change
    /// touch fires zero).
    pub wakes_fired: u64,
    /// How many `on_trade` calls actually fired -- unconditional per
    /// real `Trade` message on a subscribed instrument, so this is
    /// exactly the count of such messages seen, not a "did it change"
    /// count.
    pub trades_fired: u64,
}

/// Subscriber storage keyed by `(instrument, depth)` (FR-B18/D25,
/// unchanged) plus a plain `by_instrument` map (any depth) that
/// `on_trade` reuses, since a trade has no depth of its own. Owns the
/// "last observed snapshot" per key needed to detect a real value
/// change for `on_book` -- `on_trade` does not consult these at all.
pub struct EventDispatcher {
    subs_by_key: HashMap<(InstrumentId, Depth), Vec<SubscriberId>>,
    by_instrument: HashMap<InstrumentId, Vec<Depth>>,
    handlers: Vec<Rc<RefCell<dyn Strategy>>>,
    bbo_snapshots: HashMap<(InstrumentId, Depth), BboSnapshot>,
    depth_snapshots: HashMap<(InstrumentId, Depth), Vec<PriceLevel>>,
    pub stats: EventDispatcherStats,
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventDispatcher {
    pub fn new() -> Self {
        EventDispatcher {
            subs_by_key: HashMap::new(),
            by_instrument: HashMap::new(),
            handlers: Vec::new(),
            bbo_snapshots: HashMap::new(),
            depth_snapshots: HashMap::new(),
            stats: EventDispatcherStats::default(),
        }
    }

    /// Stores `handler` and returns its id -- no key declared yet. Split
    /// from `subscribe` below because registration happens once, at
    /// `main.rs`'s construction time, while key declarations happen
    /// later, inside the strategy's own `on_start` (D33: subscribing is
    /// the strategy's job, not the orchestrator's).
    pub fn register(&mut self, handler: Rc<RefCell<dyn Strategy>>) -> SubscriberId {
        let id = self.handlers.len();
        self.handlers.push(handler);
        id
    }

    /// Declares that subscriber `id` wants to be woken at `(instrument,
    /// depth)`. Setup work, not steady-state -- allocation here is
    /// expected and fine, same reasoning `cache::Dispatch::subscribe`'s
    /// own doc comment gave. Pre-populates this key's snapshot slot so
    /// the hot path's `get_mut` in `on_book_touched` always finds an
    /// existing entry.
    pub fn subscribe(&mut self, id: SubscriberId, instrument: InstrumentId, depth: Depth) {
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
    }

    /// Call once per event that actually touched `instrument`'s book
    /// (i.e. after `Cache::apply`). Checks every depth this instrument
    /// has subscribers at; calls `on_book` only for the ones whose
    /// subscribed slice of the book actually changed value -- identical
    /// diff logic to the original `cache::Dispatch::on_book_touched`,
    /// just calling a real strategy method instead of `on_wake`.
    ///
    /// **Phase C:** `on_book` can now submit/cancel/modify through
    /// `Ctx` (`engine` is why this takes `&mut ExecutionEngine`) --
    /// whatever it produces is accumulated into the returned
    /// `ExecOutcome`, which the caller (`main.rs`) forwards to
    /// `control_dispatcher::ControlDispatcher::dispatch` itself; this
    /// method never talks to `ControlDispatcher` directly (same
    /// independence D07 already asks of the two dispatchers).
    /// `recorder_ts` (renamed from `packet_transact_time_ns`, dual-clock
    /// replay, 2026-08-27): this is the canonical clock now (decision #2
    /// of the planning session) -- "when qtrade found out", not the
    /// exchange's own send time. `venue`: `Ctx` needs `&mut SimExchange`
    /// directly if the strategy submits/cancels/modifies from this
    /// callback -- the venue call is synchronous now (2026-08-27,
    /// reversing the dual-clock pass's own scheduled-delivery design;
    /// see `strategy.rs`'s own header).
    pub fn on_book_touched(&mut self, book: &dyn Book, instrument: InstrumentId, cache: &Cache, engine: &mut ExecutionEngine, venue: &mut SimExchange, seq: u64, recorder_ts: u64) -> ExecOutcome {
        self.stats.book_touches += 1;
        let mut merged = ExecOutcome::default();
        let Some(depths) = self.by_instrument.get(&instrument) else {
            return merged;
        };
        let depths = depths.clone(); // short, setup-sized list; avoids holding an immutable borrow of `self` across the mutable calls below
        for depth in depths {
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
                    let cur = book.depth(n as usize); // allocates -- inherent to `Book`, not introduced here
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
                        // Reborrow, not move -- `venue` is used again on
                        // the next loop iteration (and by `on_book`'s
                        // caller, further down for other instruments).
                        let mut ctx = Ctx::new(cache, engine, &mut *venue, recorder_ts, DEFAULT_STRATEGY_ID, true);
                        self.handlers[id].borrow_mut().on_book(&mut ctx, instrument, seq, recorder_ts);
                        let produced = ctx.take_outcome();
                        merged.fills.extend(produced.fills);
                        merged.order_events.extend(produced.order_events);
                    }
                }
            }
        }
        merged
    }

    /// Call once per real `Trade` message on a filtered instrument.
    /// Deliberately unconditional -- no snapshot to compare, since the
    /// trade itself (not its effect on the book) is the fact being
    /// reported. Reuses `by_instrument` (any depth counts as "this
    /// instrument has a registered handler") since a trade has no depth
    /// of its own to key on. Same Phase C `ExecOutcome`-returning shape
    /// as `on_book_touched`, same reasoning.
    /// `recorder_ts`/`venue`: same reasoning as `on_book_touched`.
    pub fn on_trade(&mut self, cache: &Cache, engine: &mut ExecutionEngine, venue: &mut SimExchange, instrument: InstrumentId, trade: &Trade, seq: u64, recorder_ts: u64) -> ExecOutcome {
        let mut merged = ExecOutcome::default();
        let Some(depths) = self.by_instrument.get(&instrument) else {
            return merged;
        };
        let mut ids: Vec<SubscriberId> = depths
            .iter()
            .filter_map(|d| self.subs_by_key.get(&(instrument, *d)))
            .flatten()
            .copied()
            .collect();
        ids.sort_unstable();
        ids.dedup();
        for id in ids {
            self.stats.trades_fired += 1;
            let mut ctx = Ctx::new(cache, engine, &mut *venue, recorder_ts, DEFAULT_STRATEGY_ID, true);
            self.handlers[id].borrow_mut().on_trade(&mut ctx, instrument, trade, seq, recorder_ts);
            let produced = ctx.take_outcome();
            merged.fills.extend(produced.fills);
            merged.order_events.extend(produced.order_events);
        }
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{Cache, InstrumentFilter};
    use crate::decoder::{DecodedMessage, OrderAdd, OrderModify, Price as DPrice, Qty as DQty, Side as DSide};
    use crate::execution::{AlwaysAllowRms, CostConfig, LocalOtrConfig, OtrConfigSummary, RunConfig};
    use crate::refdata::InstrumentMaster;
    use crate::simulator::{OrderType, OtrConfig, SimExchange};
    use crate::strategy::StartCtx;
    use crate::types::{Currency, Date, InstrumentKind, Instrument, Lots, Price, Settlement, Side, Venue, YearMonth};

    const RUPEE_RAW: i64 = 100_000_000;
    const TEST_BAND: (i64, i64) = (0, 10_000 * RUPEE_RAW);

    /// A synthetic `ExecutionEngine` with no instruments registered --
    /// enough for tests that only need `Ctx::new` to compile and never
    /// call `submit`/`order`/`position`/etc. See `engine_with` below for
    /// tests that actually exercise the execution path. Returns
    /// `(ExecutionEngine, SimExchange)` now -- `SimExchange` moved out to
    /// `main.rs`'s own ownership in the real code (dual-clock replay,
    /// 2026-08-27).
    fn engine() -> (ExecutionEngine, SimExchange) {
        engine_with(vec![])
    }

    fn engine_with(instruments: Vec<Instrument>) -> (ExecutionEngine, SimExchange) {
        let ids: Vec<InstrumentId> = instruments.iter().map(|i| i.id).collect();
        let run_config = RunConfig {
            session_id: 1,
            cost_config: CostConfig::default(),
            local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
            venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
            markout_horizons_ns: vec![],
        };
        let venue_otr = OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
        let venue = SimExchange::new(&ids, venue_otr);
        let eng = ExecutionEngine::new(run_config, instruments, Box::new(AlwaysAllowRms), CostConfig::default(), vec![], true);
        (eng, venue)
    }

    fn add(id: i64, side: DSide, price: i64, qty: i64, prio: u64) -> DecodedMessage {
        DecodedMessage::OrderAdd(OrderAdd { seq: 0, security_id: id, side, price: DPrice(price), qty: DQty(qty), priority_ts: prio, event_time: 0 })
    }

    fn future_instrument(native_id: i64, underlying: &str) -> Instrument {
        Instrument {
            id: InstrumentId(native_id as u32),
            venue: Venue::Mcx,
            native_id,
            kind: InstrumentKind::Future { underlying: underlying.to_string(), expiry: Date(0), contract_month: YearMonth { year: 2026, month: 1 }, settlement: Settlement::Cash },
            tick_size: Price(RUPEE_RAW),
            lot_size: 1,
            multiplier: 1,
            freeze_qty: 0,
            price_band: None,
            currency: Currency::Inr,
        }
    }

    fn trade(id: i64, side: DSide, price: i64, qty: i64) -> Trade {
        Trade { seq: 0, full: true, security_id: id, aggressor_side: side, price: DPrice(price), qty: DQty(qty), event_time: 0 }
    }

    /// Records every `on_book`/`on_trade` call it receives -- enough to
    /// prove both paths actually fire, without needing a real strategy.
    struct RecordingHandler {
        book_calls: Rc<RefCell<u32>>,
        trade_calls: Rc<RefCell<u32>>,
    }
    impl Strategy for RecordingHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_book(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
            *self.book_calls.borrow_mut() += 1;
        }
        fn on_trade(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _trade: &Trade, _seq: u64, _packet_transact_time_ns: u64) {
            *self.trade_calls.borrow_mut() += 1;
        }
    }

    #[test]
    fn on_book_wakes_on_bbo_change_not_on_deeper_level() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);

        let book_calls = Rc::new(RefCell::new(0u32));
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: book_calls.clone(), trade_calls: Rc::new(RefCell::new(0)) })));
        let instrument = InstrumentId(467_013);
        dispatcher.subscribe(id, instrument, Depth::Bbo);

        cache.apply(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1));
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 1, 0);
        assert_eq!(*book_calls.borrow(), 1);

        // Deeper-level-only change: must not fire.
        cache.apply(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 50, 2));
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 2, 0);
        assert_eq!(*book_calls.borrow(), 1, "a deeper-level-only change must not fire on_book for a BBO subscriber");

        cache.apply(&add(467_013, DSide::Buy, 5_010 * RUPEE_RAW, 1, 3));
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 3, 0);
        assert_eq!(*book_calls.borrow(), 2);
    }

    #[test]
    fn full_book_reachable_regardless_of_subscribed_depth() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let instrument = InstrumentId(467_013);
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: Rc::new(RefCell::new(0)), trade_calls: Rc::new(RefCell::new(0)) })));
        dispatcher.subscribe(id, instrument, Depth::Bbo);

        cache.apply(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1));
        cache.apply(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 5, 2));
        cache.apply(&add(467_013, DSide::Buy, 4_980 * RUPEE_RAW, 3, 3));

        let book = cache.book(instrument).unwrap();
        let depth = book.depth(10);
        assert!(depth.len() >= 3, "expected at least 3 levels reachable on demand, got {depth:?}");
    }

    #[test]
    fn modify_moving_bbo_wakes_exactly_once() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let instrument = InstrumentId(467_013);
        let book_calls = Rc::new(RefCell::new(0u32));
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: book_calls.clone(), trade_calls: Rc::new(RefCell::new(0)) })));
        dispatcher.subscribe(id, instrument, Depth::Bbo);

        cache.apply(&add(467_013, DSide::Sell, 5_200 * RUPEE_RAW, 10, 1));
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 1, 0);
        assert_eq!(*book_calls.borrow(), 1);

        cache.apply(&DecodedMessage::OrderModify(OrderModify {
            seq: 0,
            security_id: 467_013,
            side: DSide::Sell,
            prev_price: DPrice(5_200 * RUPEE_RAW),
            prev_qty: DQty(10),
            price: DPrice(5_190 * RUPEE_RAW),
            qty: DQty(10),
            prev_priority_ts: 1,
            priority_ts: 2,
            event_time: 0,
        }));
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 2, 0);
        assert_eq!(*book_calls.borrow(), 2);
    }

    #[test]
    fn dispatch_stats_count_touches_and_wakes_separately() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        cache.seed_book_band(InstrumentId(467_013), TEST_BAND.0, TEST_BAND.1);
        let instrument = InstrumentId(467_013);
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: Rc::new(RefCell::new(0)), trade_calls: Rc::new(RefCell::new(0)) })));
        dispatcher.subscribe(id, instrument, Depth::Bbo);

        cache.apply(&add(467_013, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1)); // touch + wake (None->Some)
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 1, 0);
        cache.apply(&add(467_013, DSide::Buy, 4_990 * RUPEE_RAW, 5, 2)); // touch, no wake (deeper level)
        dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 2, 0);

        assert_eq!(dispatcher.stats.book_touches, 2);
        assert_eq!(dispatcher.stats.wakes_fired, 1);
    }

    #[test]
    fn on_trade_fires_unconditionally_regardless_of_book_state_change() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        let instrument = InstrumentId(467_013);
        let trade_calls = Rc::new(RefCell::new(0u32));
        let book_calls = Rc::new(RefCell::new(0u32));
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: book_calls.clone(), trade_calls: trade_calls.clone() })));
        dispatcher.subscribe(id, instrument, Depth::Top(5));

        let t = trade(467_013, DSide::Buy, 305 * RUPEE_RAW, 5);
        dispatcher.on_trade(&cache, &mut engine, &mut venue, instrument, &t, 1, 0);
        dispatcher.on_trade(&cache, &mut engine, &mut venue, instrument, &t, 2, 0);
        assert_eq!(*trade_calls.borrow(), 2, "on_trade must fire once per real Trade message, unconditionally");
        assert_eq!(*book_calls.borrow(), 0, "on_trade must never call on_book");
        assert_eq!(dispatcher.stats.trades_fired, 2);
    }

    #[test]
    fn on_trade_does_not_fire_for_an_unsubscribed_instrument() {
        let filter = InstrumentFilter::from_native_ids([467_013]);
        let master = InstrumentMaster::new(vec![future_instrument(467_013, "CRUDEOIL")]);
        let cache = Cache::new(master, filter);
        let (mut engine, mut venue) = engine();
        let mut dispatcher = EventDispatcher::new();
        // No subscribe() call at all -- by_instrument has no entry.
        let t = trade(467_013, DSide::Buy, 305 * RUPEE_RAW, 5);
        dispatcher.on_trade(&cache, &mut engine, &mut venue, InstrumentId(467_013), &t, 1, 0); // must not panic, must not fire anything
        assert_eq!(dispatcher.stats.trades_fired, 0);
    }

    // ---- Phase C: on_book/on_trade can submit and read through Ctx ----

    fn engine_instrument(native_id: i64) -> Instrument {
        Instrument {
            id: InstrumentId(native_id as u32),
            venue: Venue::Mcx,
            native_id,
            kind: InstrumentKind::Future { underlying: "CRUDEOIL".to_string(), expiry: Date(0), contract_month: YearMonth { year: 2026, month: 1 }, settlement: Settlement::Cash },
            tick_size: Price(RUPEE_RAW),
            lot_size: 1,
            multiplier: 1,
            freeze_qty: 1000,
            price_band: None,
            currency: Currency::Inr,
        }
    }

    /// Submits exactly once (on the first `on_book` it ever sees), then
    /// records its own position on every call after -- enough to prove
    /// `ctx.submit`/`ctx.position` both work from `on_book`, without
    /// needing a real trading strategy.
    struct SubmittingHandler {
        last_order_id: Rc<RefCell<Option<u64>>>,
        last_position: Rc<RefCell<i64>>,
    }
    impl Strategy for SubmittingHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
            if self.last_order_id.borrow().is_none() {
                let id = ctx.submit(instrument, Side::Buy, OrderType::LimitDay(Price(100 * RUPEE_RAW)), Lots(5)).expect("submit must be allowed from on_book");
                *self.last_order_id.borrow_mut() = Some(id);
            }
            *self.last_position.borrow_mut() = ctx.position(instrument);
        }
    }

    #[test]
    fn on_book_can_submit_through_ctx_and_the_order_event_comes_back_in_exec_outcome() {
        let native_id = 467_013i64;
        let instrument = InstrumentId(native_id as u32);
        let filter = InstrumentFilter::from_native_ids([native_id]);
        let master = InstrumentMaster::new(vec![future_instrument(native_id, "CRUDEOIL")]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(instrument, TEST_BAND.0, TEST_BAND.1);
        let (mut engine, mut venue) = engine_with(vec![engine_instrument(native_id)]);

        let last_order_id = Rc::new(RefCell::new(None));
        let last_position = Rc::new(RefCell::new(0i64));
        let mut dispatcher = EventDispatcher::new();
        let id = dispatcher.register(Rc::new(RefCell::new(SubmittingHandler { last_order_id: last_order_id.clone(), last_position: last_position.clone() })));
        dispatcher.subscribe(id, instrument, Depth::Bbo);

        cache.apply(&add(native_id, DSide::Buy, 5_000 * RUPEE_RAW, 10, 1));
        let outcome = dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut engine, &mut venue, 1, 0);

        assert!(last_order_id.borrow().is_some(), "ctx.submit() from on_book must succeed");
        // Two real order-events (2026-08-27, "send as fast as possible"
        // reversal of the dual-clock pass's own scheduled-delivery
        // design): `ctx.submit()` now calls all the way through to the
        // venue in the same instant its local gates pass -- "submit:
        // gates passed" (Submitted) immediately followed by "resting"
        // (Accepted), since `venue`'s own book here is empty (nothing to
        // sweep against). There is no separate scheduled delivery step
        // left to drive by hand any more; `SimExchange::with_order_latency`
        // (unused here -- this `venue` has the default zero latency) is
        // the only remaining seam that could make this call *not*
        // resolve synchronously.
        assert_eq!(outcome.order_events.len(), 2);
        assert_eq!(*last_position.borrow(), 0, "no fill has happened yet -- ctx.position() must still read flat");
    }
}

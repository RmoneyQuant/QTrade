//! The Control Dispatcher (D07/D33): commands inward, reports outward,
//! low rate -- "typed messages and observers added by wiring, so
//! Reporting can observe execution reports without the ExecutionEngine
//! knowing Reporting exists" (D33).
//!
//! **Two real jobs (2026-08-25, two passes; calls `strategy::Strategy`
//! directly as of 2026-08-26 -- see that file's header for why the
//! separate `ControlHandler` trait this used to call was merged away):**
//!
//! 1. **Subscribing** (Phase A). D33's own diagram is `Strategy ->
//!    subscribe() -> Control Dispatcher -> Data Engine, which then
//!    updates the filter and the Event Dispatcher's routing table` --
//!    today, that is a one-line forward into
//!    `event_dispatcher::EventDispatcher::subscribe`, since `main.rs`
//!    already builds a complete `InstrumentFilter` before any strategy
//!    code runs (D32: the filter must admit every expiry of a subscribed
//!    underlying up front, to defeat the roll trap -- see
//!    `cache_user_doc.md`), so there is no *filter* mutation left for a
//!    runtime `subscribe()` call to do in this phase; only the routing-
//!    table half of D33's sentence is live machinery.
//! 2. **Delivering fills/order-updates** (Phase B). `Strategy::on_fill`/
//!    `on_order_update`, called once per real event with whatever
//!    `execution::ExecutionEngine`'s own methods just returned
//!    (`ExecOutcome` -- see that type's doc comment in `execution.rs`
//!    for how it's computed). Kept as "one destination"
//!    (`Option<Rc<RefCell<dyn Strategy>>>`, not a `strategy_id`-keyed
//!    registry) since only one strategy is ever compiled into `main.rs`
//!    today -- building real multi-strategy routing now, with nothing
//!    to exercise it, would be speculation.
//!
//! See `control_dispatcher_user_doc.md` for the full account, including
//! two disclosed gaps: live delivery is coupled to `tier2_enabled`, and
//! `OrderEventRecord` doesn't carry a structured `CancelReason` the way
//! `Order.cancel_reason` does.

use crate::cache::Cache;
use crate::event_dispatcher::{Depth, EventDispatcher, SubscriberId};
use crate::execution::{ExecOutcome, ExecutionEngine};
use crate::logging;
use crate::simulator::SimExchange;
use crate::strategy::{Ctx, Strategy, DEFAULT_STRATEGY_ID};
use crate::types::InstrumentId;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct ControlDispatcher {
    handler: Option<Rc<RefCell<dyn Strategy>>>,
}

impl ControlDispatcher {
    pub fn new() -> Self {
        ControlDispatcher { handler: None }
    }

    /// One destination -- see this file's header for why that's
    /// deliberate, not an oversight. Registering a second handler
    /// silently replaces the first; there is no real multi-strategy
    /// config (D08) yet for "replace" vs. "reject" to be a meaningful
    /// choice between.
    pub fn register(&mut self, handler: Rc<RefCell<dyn Strategy>>) {
        self.handler = Some(handler);
    }

    /// D33: `Strategy -> subscribe() -> Control Dispatcher -> Data
    /// Engine`. `event_dispatcher` is taken as a parameter rather than
    /// stored -- `ControlDispatcher` only ever needs it for the instant
    /// of this call (inside a strategy's `on_start`), and holding a
    /// persistent `&mut EventDispatcher` here would conflict with
    /// `main.rs`'s own later need for one in the replay loop.
    pub fn subscribe(&mut self, event_dispatcher: &mut EventDispatcher, id: SubscriberId, instrument: InstrumentId, depth: Depth) {
        event_dispatcher.subscribe(id, instrument, depth);
        // Pre-replay (`on_start`, `now_ns: None`), same honest rendering
        // as a strategy's own `on_start` line -- there is no sim
        // timestamp yet for this to report. "Each part of qtrade sends
        // its own confirmation" (2026-08-27): this is the system's own
        // confirmation that the subscription was actually registered,
        // independent of whatever a strategy chooses to print about it.
        tracing::info!("{}", logging::line("ControlDispatcher", None, "SUBSCRIBE_OK", &format!("subscriber_id={id} instrument={instrument:?} depth={depth:?}")));
    }

    /// Delivers one call's worth of real fills/order-events to the
    /// registered handler, in order -- fills first, then order-events,
    /// matching the order `execution.rs`'s own `handle_exec_reports`
    /// produces them in for a `Filled` report (`on_fill` runs before the
    /// resulting `log_event` call in `ExecutionEngine::on_fill`). A
    /// no-op if nothing is registered (e.g. `LimitOrderBookGenerator`,
    /// whose `Strategy` impl leaves `on_fill`/`on_order_update` at their
    /// default) or if `outcome` is empty.
    ///
    /// **Phase C:** takes `&mut ExecutionEngine` too, purely so it can
    /// build the same `Ctx` type `EventDispatcher` does -- `on_fill`/
    /// `on_order_update` can read `ctx.order()`/`position()`/`pnl()`/
    /// `cost()` like any other callback. `Ctx` here is always
    /// constructed with `can_submit: false` (Q3): a strategy calling
    /// `ctx.submit()`/`cancel()`/`modify()` from inside `on_fill`/
    /// `on_order_update` gets `Err(CtxError::SubmitNotAllowedHere)`
    /// immediately, at its own call site (Q4) -- this method never
    /// needs to notice or forward anything from that failed attempt,
    /// since a failed write never touches `Ctx`'s `pending` accumulator.
    /// `venue`: `Ctx` needs `&mut SimExchange` regardless of `can_submit`
    /// -- unused in practice here, since `can_submit: false` means
    /// `submit`/`cancel`/`modify` fail at their own call site before ever
    /// touching it, but `Ctx::new` still needs one to construct.
    pub fn dispatch(&mut self, cache: &Cache, engine: &mut ExecutionEngine, venue: &mut SimExchange, outcome: &ExecOutcome) {
        let Some(handler) = &self.handler else { return };
        for fill in &outcome.fills {
            // One `DISPATCH` line right before the handoff -- the moment
            // the strategy actually learns this fill happened, distinct
            // from whatever it then does in `on_fill` itself (2026-08-27
            // component-level event logging pass).
            tracing::info!("{}", logging::line("ControlDispatcher", Some(fill.timestamp_ns), "DISPATCH", &format!("on_fill(client_order_id={} price={} qty={})", fill.client_order_id, fill.price, fill.qty)));
            let mut ctx = Ctx::new(cache, engine, &mut *venue, fill.timestamp_ns, DEFAULT_STRATEGY_ID, false);
            handler.borrow_mut().on_fill(&mut ctx, fill);
        }
        for event in &outcome.order_events {
            tracing::info!("{}", logging::line("ControlDispatcher", Some(event.timestamp_ns), "DISPATCH", &format!("on_order_update(client_order_id={} state={:?})", event.client_order_id, event.resulting_state)));
            let mut ctx = Ctx::new(cache, engine, &mut *venue, event.timestamp_ns, DEFAULT_STRATEGY_ID, false);
            handler.borrow_mut().on_order_update(&mut ctx, event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::{Cache, InstrumentFilter};
    use crate::decoder::{DecodedMessage, OrderAdd, Price as DPrice, Qty as DQty, Side as DSide, Trade};
    use crate::execution::{AlwaysAllowRms, CostConfig, ExecutionEngine, FillRecord, GateOutcome, NewOrderIntent, LocalOtrConfig, OrderEventRecord, OtrConfigSummary, RunConfig};
    use crate::refdata::InstrumentMaster;
    use crate::simulator::{OrderType, OtrConfig};
    use crate::strategy::{Ctx, StartCtx, Strategy};
    use crate::types::{Currency, Date, Instrument, InstrumentKind, Lots, Price, Settlement, Side, Venue, YearMonth};
    use std::cell::RefCell;
    use std::rc::Rc;

    struct RecordingHandler {
        book_calls: Rc<RefCell<u32>>,
    }
    impl Strategy for RecordingHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_book(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
            *self.book_calls.borrow_mut() += 1;
        }
    }

    #[test]
    fn subscribe_forwards_to_event_dispatcher_and_a_real_event_reaches_the_handler() {
        const RUPEE_RAW: i64 = 100_000_000;
        let native_id = 467_013i64;
        let instrument = InstrumentId(native_id as u32);
        let master = InstrumentMaster::new(vec![Instrument {
            id: instrument,
            venue: Venue::Mcx,
            native_id,
            kind: InstrumentKind::Future { underlying: "CRUDEOIL".to_string(), expiry: Date(0), contract_month: YearMonth { year: 2026, month: 1 }, settlement: Settlement::Cash },
            tick_size: Price(RUPEE_RAW),
            lot_size: 1,
            multiplier: 1,
            max_single_order_qty: 0,
            price_band: None,
            currency: Currency::Inr,
        }]);
        let filter = InstrumentFilter::from_native_ids([native_id]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(instrument, 0, 10_000 * RUPEE_RAW);
        let (mut eng, mut venue) = engine(vec![]);

        let mut event_dispatcher = EventDispatcher::new();
        let book_calls = Rc::new(RefCell::new(0u32));
        let id = event_dispatcher.register(Rc::new(RefCell::new(RecordingHandler { book_calls: book_calls.clone() })));

        // The call under test: ControlDispatcher, not EventDispatcher
        // directly, is what a strategy's `on_start` actually calls (D33).
        let mut control_dispatcher = ControlDispatcher::new();
        control_dispatcher.subscribe(&mut event_dispatcher, id, instrument, Depth::Top(5));

        let event = DecodedMessage::OrderAdd(OrderAdd { seq: 0, security_id: native_id, side: DSide::Buy, price: DPrice(5_000 * RUPEE_RAW), qty: DQty(10), priority_ts: 1, event_time: 0 });
        cache.apply(&event);
        event_dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut eng, &mut venue, 1, 0);

        assert_eq!(*book_calls.borrow(), 1, "a real event must reach the handler through ControlDispatcher::subscribe's forward -- proves the forward isn't a silent no-op");
    }

    // ---- Phase B: real fill/order-update delivery ----

    struct RecordingControlHandler {
        fills: Rc<RefCell<Vec<FillRecord>>>,
        order_updates: Rc<RefCell<Vec<OrderEventRecord>>>,
    }
    impl Strategy for RecordingControlHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_fill(&mut self, _ctx: &mut Ctx, fill: &FillRecord) {
            self.fills.borrow_mut().push(fill.clone());
        }
        fn on_order_update(&mut self, _ctx: &mut Ctx, update: &OrderEventRecord) {
            self.order_updates.borrow_mut().push(update.clone());
        }
    }

    fn engine_instrument(native_id: i64) -> crate::types::Instrument {
        crate::types::Instrument {
            id: InstrumentId(native_id as u32),
            venue: crate::types::Venue::Mcx,
            native_id,
            kind: crate::types::InstrumentKind::Future {
                underlying: "CRUDEOIL".to_string(),
                expiry: crate::types::Date(0),
                contract_month: crate::types::YearMonth { year: 2026, month: 1 },
                settlement: crate::types::Settlement::Cash,
            },
            tick_size: Price(1),
            lot_size: 1,
            multiplier: 1,
            max_single_order_qty: 1000,
            price_band: None,
            currency: crate::types::Currency::Inr,
        }
    }

    /// Returns `(ExecutionEngine, SimExchange)` now -- `SimExchange`
    /// moved out to `main.rs`'s own ownership in the real code (dual-
    /// clock replay, 2026-08-27).
    fn engine(instruments: Vec<crate::types::Instrument>) -> (ExecutionEngine, crate::simulator::SimExchange) {
        let ids: Vec<InstrumentId> = instruments.iter().map(|i| i.id).collect();
        let run_config = RunConfig {
            session_id: 1,
            cost_config: CostConfig::default(),
            local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
            venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
            markout_horizons_ns: vec![],
        };
        let venue_otr = OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
        let venue = crate::simulator::SimExchange::new(&ids, venue_otr);
        let eng = ExecutionEngine::new(run_config, instruments, Box::new(AlwaysAllowRms), CostConfig::default(), vec![], true);
        (eng, venue)
    }

    /// Test convenience for tests that call `submit_order_local`/
    /// `deliver_order` directly rather than through `Ctx::submit` --
    /// recombines them into the one synchronous call `Ctx::submit` itself
    /// now makes in real code too (2026-08-27: the venue call is
    /// synchronous, not a separate scheduled phase -- see `strategy.rs`'s
    /// own header).
    fn submit_order_sync(eng: &mut ExecutionEngine, venue: &mut crate::simulator::SimExchange, intent: NewOrderIntent, now_ns: u64) -> (GateOutcome, ExecOutcome) {
        let (outcome, req, mut merged) = eng.submit_order_local(intent, now_ns, venue);
        if let Some(req) = req {
            let delivered = eng.deliver_order(req, now_ns, venue);
            merged.fills.extend(delivered.fills);
            merged.order_events.extend(delivered.order_events);
        }
        (outcome, merged)
    }

    /// Test-only convenience, same reasoning: recombines
    /// `prepare_for_market_event` + `venue.apply_market_event` +
    /// `apply_venue_reports` into one synchronous call.
    fn on_market_event_sync(eng: &mut ExecutionEngine, venue: &mut crate::simulator::SimExchange, event: &DecodedMessage, now_ns: u64) -> ExecOutcome {
        eng.prepare_for_market_event(venue);
        let reports = venue.apply_market_event(event, now_ns);
        eng.apply_venue_reports(reports, now_ns)
    }

    /// The delivery mechanism this pass exists to build, proven with
    /// synthetic data -- `LimitOrderBookGenerator` (the one real
    /// strategy) submits no orders, so the real `19_08_2026` run's own
    /// `ExecOutcome` is always empty; this is where the real proof lives
    /// instead.
    #[test]
    fn a_real_fill_reaches_on_fill_through_control_dispatcher() {
        let native_id = 467_013i64;
        let instrument = InstrumentId(native_id as u32);
        let master = InstrumentMaster::new(vec![engine_instrument(native_id)]);
        let filter = InstrumentFilter::from_native_ids([native_id]);
        let cache = Cache::new(master, filter); // no book activity needed for this test -- Ctx::book is unused by RecordingControlHandler

        let (mut eng, mut venue) = engine(vec![engine_instrument(native_id)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(5) };
        let (submit_outcome, _) = submit_order_sync(&mut eng, &mut venue, intent, 0);
        let GateOutcome::Submitted { client_order_id } = submit_outcome else { panic!("expected Submitted") };

        let mut control_dispatcher = ControlDispatcher::new();
        let fills = Rc::new(RefCell::new(Vec::new()));
        let order_updates = Rc::new(RefCell::new(Vec::new()));
        control_dispatcher.register(Rc::new(RefCell::new(RecordingControlHandler { fills: fills.clone(), order_updates: order_updates.clone() })));

        // A real trade that fully fills our resting Sell order -- same
        // recipe execution.rs's own tests use (fallback-to-FIFO-front
        // rule: nothing else is resting ahead of us at this price).
        let trade = DecodedMessage::Trade(Trade { seq: 0, full: true, security_id: native_id, aggressor_side: DSide::Sell, price: DPrice(150), qty: DQty(intent_raw_qty()), event_time: 999 });
        let outcome = on_market_event_sync(&mut eng, &mut venue, &trade, 10);
        assert_eq!(outcome.fills.len(), 1, "the fill must actually be in what on_market_event returned");

        control_dispatcher.dispatch(&cache, &mut eng, &mut venue, &outcome);

        assert_eq!(fills.borrow().len(), 1, "on_fill must have fired exactly once, via ControlDispatcher::dispatch");
        assert_eq!(fills.borrow()[0].client_order_id, client_order_id);
        assert_eq!(eng.order(client_order_id).unwrap().state, crate::execution::OrderState::Filled);
    }

    /// `Lots(5)` converted the same way `NewOrderIntent`/`submit_order`
    /// convert it internally (`Lots::to_raw_qty`), kept as its own
    /// function only so the test above reads as "the real fill quantity"
    /// rather than a bare magic number.
    fn intent_raw_qty() -> i64 {
        Lots(5).to_raw_qty().0 as i64
    }

    // ---- Phase C: submit from on_book -> real fill -> delivered via on_fill ----

    /// Submits once, from `on_book`, then leaves `on_fill`/
    /// `on_order_update` to `RecordingControlHandler` -- proves the two
    /// halves (submitting through `EventDispatcher`, delivering through
    /// `ControlDispatcher`) are actually the same pipeline end to end.
    struct SubmittingMarketHandler {
        instrument: InstrumentId,
        submitted: Rc<RefCell<Option<u64>>>,
    }
    impl Strategy for SubmittingMarketHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_book(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
            if self.submitted.borrow().is_none() {
                let id = ctx.submit(self.instrument, Side::Sell, OrderType::LimitDay(Price(150)), Lots(5)).expect("submit must be allowed from on_book");
                *self.submitted.borrow_mut() = Some(id);
            }
        }
    }

    #[test]
    fn a_strategy_submitted_order_is_filled_and_delivered_through_on_fill() {
        let native_id = 467_013i64;
        let instrument = InstrumentId(native_id as u32);
        let master = InstrumentMaster::new(vec![engine_instrument(native_id)]);
        let filter = InstrumentFilter::from_native_ids([native_id]);
        let mut cache = Cache::new(master, filter);
        cache.seed_book_band(instrument, 0, 10_000); // band irrelevant here -- book state isn't read by either handler
        let (mut eng, mut venue) = engine(vec![engine_instrument(native_id)]);

        let mut event_dispatcher = EventDispatcher::new();
        let submitted = Rc::new(RefCell::new(None));
        let market_id = event_dispatcher.register(Rc::new(RefCell::new(SubmittingMarketHandler { instrument, submitted: submitted.clone() })));
        event_dispatcher.subscribe(market_id, instrument, Depth::Bbo);

        let mut control_dispatcher = ControlDispatcher::new();
        let fills = Rc::new(RefCell::new(Vec::new()));
        let order_updates = Rc::new(RefCell::new(Vec::new()));
        control_dispatcher.register(Rc::new(RefCell::new(RecordingControlHandler { fills: fills.clone(), order_updates: order_updates.clone() })));

        // 1. A real book-touching event fires on_book, which now submits
        // *and* delivers to the venue in the same synchronous call
        // (2026-08-27, "send as fast as possible" -- reversing the
        // dual-clock pass's own scheduled-delivery design; see
        // `strategy.rs`'s own header). No Scheduler/pending-ops table
        // needed here any more -- there's no second phase to drive by
        // hand.
        cache.apply(&DecodedMessage::OrderAdd(OrderAdd { seq: 0, security_id: native_id, side: DSide::Buy, price: DPrice(100), qty: DQty(10), priority_ts: 1, event_time: 0 }));
        let submit_outcome = event_dispatcher.on_book_touched(cache.book(instrument).unwrap(), instrument, &cache, &mut eng, &mut venue, 1, 0);
        control_dispatcher.dispatch(&cache, &mut eng, &mut venue, &submit_outcome);
        let client_order_id = submitted.borrow().expect("on_book must have submitted");
        // "submit: gates passed" *and* "resting" arrive together now --
        // the venue call happens the instant the local gates pass, not
        // on some later scheduled event.
        assert_eq!(order_updates.borrow().len(), 2, "submit + resting both arrive from the one synchronous ctx.submit() call");

        // 2. A real trade fully fills the resting Sell order.
        let trade = DecodedMessage::Trade(Trade { seq: 0, full: true, security_id: native_id, aggressor_side: DSide::Sell, price: DPrice(150), qty: DQty(intent_raw_qty()), event_time: 999 });
        let fill_outcome = on_market_event_sync(&mut eng, &mut venue, &trade, 10);
        control_dispatcher.dispatch(&cache, &mut eng, &mut venue, &fill_outcome);

        assert_eq!(fills.borrow().len(), 1, "the fill must have reached on_fill, not been returned directly from ctx.submit() (Q2)");
        assert_eq!(fills.borrow()[0].client_order_id, client_order_id);
    }

    // ---- Phase C / Q4: submit from on_fill/on_order_update fails loudly ----

    struct SubmitAttemptingControlHandler {
        instrument: InstrumentId,
        result: Rc<RefCell<Option<Result<u64, crate::strategy::CtxError>>>>,
    }
    impl Strategy for SubmitAttemptingControlHandler {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
        fn on_fill(&mut self, ctx: &mut Ctx, _fill: &FillRecord) {
            *self.result.borrow_mut() = Some(ctx.submit(self.instrument, Side::Buy, OrderType::LimitDay(Price(150)), Lots(1)));
        }
    }

    #[test]
    fn submit_from_on_fill_fails_loudly_instead_of_silently_dropping() {
        let native_id = 467_013i64;
        let instrument = InstrumentId(native_id as u32);
        let master = InstrumentMaster::new(vec![engine_instrument(native_id)]);
        let filter = InstrumentFilter::from_native_ids([native_id]);
        let cache = Cache::new(master, filter);
        let (mut eng, mut venue) = engine(vec![engine_instrument(native_id)]);

        let intent = NewOrderIntent { strategy_id: 1, instrument, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(5) };
        submit_order_sync(&mut eng, &mut venue, intent, 0);

        let mut control_dispatcher = ControlDispatcher::new();
        let result = Rc::new(RefCell::new(None));
        control_dispatcher.register(Rc::new(RefCell::new(SubmitAttemptingControlHandler { instrument, result: result.clone() })));

        let trade = DecodedMessage::Trade(Trade { seq: 0, full: true, security_id: native_id, aggressor_side: DSide::Sell, price: DPrice(150), qty: DQty(intent_raw_qty()), event_time: 999 });
        let outcome = on_market_event_sync(&mut eng, &mut venue, &trade, 10);
        assert_eq!(outcome.fills.len(), 1);
        control_dispatcher.dispatch(&cache, &mut eng, &mut venue, &outcome);

        assert_eq!(*result.borrow(), Some(Err(crate::strategy::CtxError::SubmitNotAllowedHere)), "ctx.submit() from on_fill must fail loudly (Q4), not silently no-op or panic");
    }
}

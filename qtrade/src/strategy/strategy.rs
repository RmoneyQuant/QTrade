//! `Strategy`/`Ctx`/`StartCtx` -- the strategy-facing types named in
//! `STRATEGY-GUIDE.md` §2/§4. Lives directly in `src/strategy/`, not in
//! any one strategy's own subfolder (see `strategy/README.md`): this is
//! shared infrastructure every strategy uses, not a strategy itself.
//!
//! **Why this file exists, rather than putting these types inside
//! `event_dispatcher.rs`:** `StartCtx` has to reach *both*
//! `event_dispatcher::EventDispatcher` and
//! `control_dispatcher::ControlDispatcher` (D33: `subscribe()` routes
//! through the Control Dispatcher, which then reaches into the Event
//! Dispatcher's routing table). Defining `Strategy`/`Ctx`/`StartCtx`
//! inside either dispatcher's own module would make that dispatcher
//! depend on the other's module just to declare its own calls -- exactly
//! the coupling neither dispatcher is supposed to have. A neutral third
//! home avoids it: both dispatcher modules depend on this one for the
//! trait they call into and the type it's written in terms of; this
//! module depends on both of them for the dispatcher types `StartCtx`
//! wraps. No cycle between the two dispatchers themselves.
//!
//! ## `Ctx` can act now (2026-08-25, Phase C)
//!
//! `ctx.book`/`refdata`/`now` are always available. `ctx.order`/
//! `position`/`pnl`/`cost` are read-only and safe from every callback.
//! `ctx.submit`/`cancel`/`modify` are gated by `can_submit` -- `true`
//! only when `event_dispatcher::EventDispatcher` constructs this `Ctx`
//! for `on_book`/`on_trade`; `false` when `control_dispatcher`
//! constructs it for `on_fill`/`on_order_update`. Calling a write method
//! when `can_submit` is `false` returns `Err(CtxError::SubmitNotAllowedHere)`
//! -- a deliberate, discussed choice: submitting from a delivery callback
//! would need recursive delivery (a reaction's own fill needing to fire
//! *this same event*, with real risk of never terminating), which
//! nothing here needs yet, so it fails loudly rather than silently
//! discarding the order.
//!
//! `ctx.submit`/`cancel`/`modify` return only an acknowledgment (a
//! client order id, or nothing) -- never fill data directly, even for an
//! instant fill that happens synchronously inside the call. Every fill
//! or state change reaches a strategy exactly one way: through
//! `on_fill`/`on_order_update`, via the `ExecOutcome` this `Ctx`
//! accumulates (`pending`) and hands back to whichever dispatcher
//! constructed it (`take_outcome`), for that dispatcher to forward to
//! `control_dispatcher::ControlDispatcher::dispatch` itself.
//!
//! ## Scope (deliberately out, still)
//!
//! Submitting from `on_fill`/`on_order_update` (see above). `ctx.cancel_all`/
//! `cancel_all_mine` (`STRATEGY-GUIDE.md` §6) -- not needed until a real
//! strategy asks for them. `ctx.set_timer`/`offload`/`rng`/`log`/`publish`
//! -- need scheduler/session-state work nothing here touches.
//!
//! ## `Strategy` -- one real trait (2026-08-26, revised)
//!
//! Built first as two traits split across `event_dispatcher.rs`/
//! `control_dispatcher.rs`, on the reasoning that D33's "these were
//! never going to be one thing" (about the two *dispatchers*) extended
//! to the trait each one calls into. On review, that extension didn't
//! hold: D33's argument is about the dispatch *mechanism* (different
//! lookup key, different cardinality, different delivery guarantee),
//! not the callback *interface* -- two dispatcher components can each
//! call a subset of methods on one shared trait just as well as two.
//! `STRATEGY-GUIDE.md` §2 always specified one `Strategy` trait; this is
//! now that trait, for real, matching the guide's own words: *"A
//! strategy is a struct... implementing the `Strategy` trait."*
//!
//! `EventDispatcher`/`ControlDispatcher` themselves are unchanged by
//! this -- same registries, same keying, same `ExecOutcome`
//! accumulation, same reason they're two components (see each one's own
//! doc comment). Only the type they hold changes, from two separate
//! trait objects to `Rc<RefCell<dyn Strategy>>` each -- `EventDispatcher`
//! calls `.on_book()`/`.on_trade()` on it, `ControlDispatcher` calls
//! `.on_fill()`/`.on_order_update()`, and neither needs to know the
//! other exists, exactly as before.
//!
//! **All ten of the guide's callbacks are declared now**, not just the
//! five with real machinery behind them. `on_start` has no default (the
//! one thing every strategy must declare); every other method defaults
//! to `{}` -- "implement only what you use" is the guide's own stated
//! design (§2), not an approximation. Five of the ten are honestly
//! unbacked and documented as such at each method: `on_warmup_complete`
//! (no warmup lifecycle), `on_timer` (no scheduler wiring), `on_session_change`
//! (no session-state tracking), `on_book_state_change` (nothing detects
//! STALE/RECOVERING). `on_stop` is the one exception -- real, wired: see
//! `main_user_doc.md`, `main.rs` calls it once, right after the replay
//! loop ends.

use crate::book::Book;
use crate::cache::Cache;
use crate::control_dispatcher::ControlDispatcher;
use crate::decoder::Trade;
use crate::event_dispatcher::{Depth, EventDispatcher, SubscriberId};
use crate::execution::{Cost, ExecOutcome, ExecutionEngine, FillRecord, GateOutcome, NewOrderIntent, Order, OrderEventRecord, StrategyId};
use crate::refdata::InstrumentMaster;
use crate::simulator::OrderType;
use crate::types::{BookState, InstrumentId, Lots, Price, Qty, Side, Venue};

/// Only one strategy is ever compiled into `main.rs` today (see
/// `strategy/README.md`) -- `Ctx` supplies this fixed id itself, so a
/// strategy never needs to know or pass its own `StrategyId`. Real
/// multi-strategy assignment (D08) is later work.
pub const DEFAULT_STRATEGY_ID: StrategyId = 1;

/// Why a write can fail: today, exactly one reason -- see this file's
/// header. More reasons may join this enum later; it deliberately isn't
/// just a `bool`/`&str`, so a caller can match on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtxError {
    /// `ctx.submit`/`cancel`/`modify` called from `on_fill`/
    /// `on_order_update`, where it isn't supported this pass.
    SubmitNotAllowedHere,
}

/// A strategy's own net position and P&L, read from its `SubAccount` --
/// `ctx.pnl()`'s return type. Deliberately a named struct, not a bare
/// tuple, so `gross`/`net` can't be swapped by accident at a call site.
#[derive(Debug, Clone, Copy, Default)]
pub struct Pnl {
    pub gross: f64,
    pub net: f64,
}

/// Handed to every callback except `on_start` (`on_book`, `on_trade`,
/// `on_fill`, `on_order_update` all take the same `Ctx` type --
/// `STRATEGY-GUIDE.md` §2 defines one shared context for every non-start
/// callback, not one per callback).
pub struct Ctx<'a> {
    cache: &'a Cache,
    engine: &'a mut ExecutionEngine,
    now_ns: u64,
    strategy_id: StrategyId,
    can_submit: bool,
    /// What `submit`/`cancel`/`modify` have produced during this one
    /// call so far -- drained by `take_outcome` after the strategy's
    /// callback returns, so whichever dispatcher constructed this `Ctx`
    /// can forward it to `ControlDispatcher::dispatch` for real
    /// delivery. Never read by the strategy itself (see this file's
    /// header: writes return only an acknowledgment).
    pending: ExecOutcome,
}

impl<'a> Ctx<'a> {
    pub fn new(cache: &'a Cache, engine: &'a mut ExecutionEngine, now_ns: u64, strategy_id: StrategyId, can_submit: bool) -> Self {
        Ctx { cache, engine, now_ns, strategy_id, can_submit, pending: ExecOutcome::default() }
    }

    /// Full book access, exactly as `Cache::book` already provides --
    /// not gated by subscription depth (D25: subscription governs
    /// waking, not access).
    pub fn book(&self, instrument: InstrumentId) -> Option<&'a dyn Book> {
        self.cache.book(instrument)
    }

    pub fn refdata(&self) -> &'a InstrumentMaster {
        self.cache.refdata()
    }

    /// The current simulated time -- `STRATEGY-GUIDE.md` §11's own rule
    /// (`ctx.now()`, never `SystemTime::now()`/`Instant::now()`), real
    /// now that there's a reason to read it (`submit`/`cancel`/`modify`
    /// use this same value, so a strategy that logs `ctx.now()` sees
    /// exactly the timestamp its own order was submitted at).
    pub fn now(&self) -> u64 {
        self.now_ns
    }

    // ---- read-only: safe from every callback, no `can_submit` gate ----

    pub fn order(&self, client_order_id: u64) -> Option<&Order> {
        self.engine.order(client_order_id)
    }

    /// Net position, in lots -- `0` if this strategy has never traded
    /// `instrument` at all. Real `SubAccount::net_position`, already
    /// tested (D08's own two-level accounting).
    pub fn position(&self, instrument: InstrumentId) -> i64 {
        self.engine.portfolio().sub_account(self.strategy_id).map(|s| s.net_position(instrument)).unwrap_or(0)
    }

    /// This strategy's own gross/net P&L -- **never the firm's**
    /// (`ctx.firm_position`/`firm_pnl` would be the firm view;
    /// deliberately not added yet, since nothing here needs it and D08's
    /// own warning is "skew on your own, read the firm view to degrade
    /// gracefully" -- a distinction worth keeping real, not blurred).
    pub fn pnl(&self) -> Pnl {
        match self.engine.portfolio().sub_account(self.strategy_id) {
            Some(sub) => Pnl { gross: sub.gross_pnl(), net: sub.net_pnl() },
            None => Pnl::default(),
        }
    }

    /// What a fill of `qty` lots of `instrument` at `price` on `side`
    /// would cost -- the same `CostModel::round_trip` a real fill is
    /// later charged through (D23: "the same function serves both
    /// callers", so a pre-trade query and realised accounting can never
    /// quietly disagree). `None` only if `instrument` isn't in this
    /// engine's registry at all.
    pub fn cost(&self, instrument: InstrumentId, qty: Lots, price: Price, side: Side) -> Option<Cost> {
        self.engine.instrument(instrument).map(|i| self.engine.cost_model().round_trip(i, qty, price, side))
    }

    // ---- writes: gated by `can_submit` (Q3/Q4) ----

    /// Submits a new order. Returns only the client order id -- never
    /// fill data, even for an instant fill (see this file's header): any
    /// fills/order-events this produces are accumulated into `pending`
    /// and delivered later, through `on_fill`/`on_order_update`.
    pub fn submit(&mut self, instrument: InstrumentId, side: Side, order_type: OrderType, qty: Lots) -> Result<u64, CtxError> {
        if !self.can_submit {
            return Err(CtxError::SubmitNotAllowedHere);
        }
        let intent = NewOrderIntent { strategy_id: self.strategy_id, instrument, side, order_type, qty };
        let (outcome, exec) = self.engine.submit_order(intent, self.now_ns);
        self.pending.fills.extend(exec.fills);
        self.pending.order_events.extend(exec.order_events);
        Ok(match outcome {
            GateOutcome::Submitted { client_order_id } => client_order_id,
            GateOutcome::Denied { client_order_id, .. } => client_order_id,
        })
    }

    /// Cancels an order -- both real phases (`request_cancel` then
    /// `deliver_cancel_to_venue`) run immediately, one after the other:
    /// no latency model exists anywhere in this codebase yet (a
    /// deliberate, project-wide deferral -- "our setup is a money
    /// printer at this phase"), so there is no real delay for a second
    /// phase to wait through.
    pub fn cancel(&mut self, client_order_id: u64) -> Result<(), CtxError> {
        if !self.can_submit {
            return Err(CtxError::SubmitNotAllowedHere);
        }
        let (_, e1) = self.engine.request_cancel(client_order_id, self.now_ns);
        let e2 = self.engine.deliver_cancel_to_venue(client_order_id, self.now_ns);
        self.pending.fills.extend(e1.fills);
        self.pending.order_events.extend(e1.order_events);
        self.pending.fills.extend(e2.fills);
        self.pending.order_events.extend(e2.order_events);
        Ok(())
    }

    /// Modifies an order -- same two-phase-but-immediate reasoning as
    /// `cancel`. "So `ctx.modify(id, new_qty)` to shrink a quote keeps
    /// your place in the queue" (`STRATEGY-GUIDE.md` §6) -- real once
    /// this pass lands, not just documented intent.
    pub fn modify(&mut self, client_order_id: u64, new_qty: Qty, new_price: Option<Price>) -> Result<(), CtxError> {
        if !self.can_submit {
            return Err(CtxError::SubmitNotAllowedHere);
        }
        let (_, e1) = self.engine.request_modify(client_order_id, self.now_ns);
        let e2 = self.engine.deliver_modify_to_venue(client_order_id, new_qty, new_price, self.now_ns);
        self.pending.fills.extend(e1.fills);
        self.pending.order_events.extend(e1.order_events);
        self.pending.fills.extend(e2.fills);
        self.pending.order_events.extend(e2.order_events);
        Ok(())
    }

    /// Drains whatever `submit`/`cancel`/`modify` produced during this
    /// call -- called by whichever dispatcher constructed this `Ctx`,
    /// after the strategy's own callback returns, never by the strategy
    /// itself.
    pub(crate) fn take_outcome(&mut self) -> ExecOutcome {
        std::mem::take(&mut self.pending)
    }
}

/// Handed only to `on_start` -- "the only place you can declare
/// instruments, dependencies and time series. Market data has not
/// started" (`STRATEGY-GUIDE.md` §3).
pub struct StartCtx<'a> {
    resolver: &'a dyn Fn(&str) -> Option<InstrumentId>,
    event_dispatcher: &'a mut EventDispatcher,
    control_dispatcher: &'a mut ControlDispatcher,
    my_id: SubscriberId,
}

impl<'a> StartCtx<'a> {
    pub fn new(resolver: &'a dyn Fn(&str) -> Option<InstrumentId>, event_dispatcher: &'a mut EventDispatcher, control_dispatcher: &'a mut ControlDispatcher, my_id: SubscriberId) -> Self {
        StartCtx { resolver, event_dispatcher, control_dispatcher, my_id }
    }

    /// Same call, same signature, in both Backtest Mode and Live Mode
    /// (`STRATEGY-GUIDE.md`'s own opening guarantee) -- only what's
    /// *behind* it differs: `main.rs` supplies a closure over whatever
    /// this mode's real instrument lookup is (backtest: results already
    /// resolved via `feed_replay::resolve_front_month`; live: a
    /// different mechanism, not built yet).
    pub fn resolve(&self, name: &str) -> Option<InstrumentId> {
        (self.resolver)(name)
    }

    /// D33: `Strategy -> subscribe() -> Control Dispatcher -> Data
    /// Engine`.
    pub fn subscribe(&mut self, instrument: InstrumentId, depth: Depth) {
        self.control_dispatcher.subscribe(self.event_dispatcher, self.my_id, instrument, depth);
    }
}

/// A timer or alarm identifier -- `on_timer`'s own parameter shape in
/// `STRATEGY-GUIDE.md` §10. Placeholder: no scheduler wiring produces
/// one anywhere in this codebase yet (`scheduler.rs` exists, has real
/// tests, but has zero real callers -- see `main_user_doc.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerId(pub u64);

/// A venue's trading phase -- `on_session_change`'s own parameter shape
/// in the guide. Placeholder: nothing in this codebase tracks session
/// state (D16 is written, not built) to ever produce one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionPhase {
    PreOpen,
    Continuous,
    Closed,
}

/// `STRATEGY-GUIDE.md` §2's real trait: "a struct holding your own
/// state, implementing the `Strategy` trait, reacting to events and
/// submitting orders." Every method but `on_start` defaults to `{}` --
/// implement only what you use, per the guide's own words, not a promise
/// this file makes on its own. See this file's header for which five of
/// the ten are real today and which five are declared, unbacked
/// placeholders.
pub trait Strategy {
    /// Called once, before any market data. Declare what you need
    /// (`ctx.resolve`/`ctx.subscribe`) -- the only method without a
    /// default, since a strategy that declares nothing has nothing to
    /// do.
    fn on_start(&mut self, ctx: &mut StartCtx);

    /// A subscribed book changed. Real -- called by
    /// `event_dispatcher::EventDispatcher::on_book_touched`.
    fn on_book(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {}

    /// A trade printed on a subscribed instrument. Real -- called by
    /// `event_dispatcher::EventDispatcher::on_trade`.
    fn on_trade(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _trade: &Trade, _seq: u64, _packet_transact_time_ns: u64) {}

    /// One of your orders filled, fully or partially. Real -- called by
    /// `control_dispatcher::ControlDispatcher::dispatch`.
    fn on_fill(&mut self, _ctx: &mut Ctx, _fill: &FillRecord) {}

    /// One of your orders changed state. Real -- called by
    /// `control_dispatcher::ControlDispatcher::dispatch`.
    fn on_order_update(&mut self, _ctx: &mut Ctx, _update: &OrderEventRecord) {}

    /// Warmup is over, you may now quote. **Unbacked**: no bootstrap/
    /// warmup lifecycle exists (`STRATEGY-GUIDE.md` §3) -- nothing ever
    /// calls this.
    fn on_warmup_complete(&mut self, _ctx: &mut Ctx) {}

    /// A timer or alarm you scheduled has fired. **Unbacked**: no
    /// scheduler is wired into the event loop.
    fn on_timer(&mut self, _ctx: &mut Ctx, _timer: TimerId) {}

    /// A venue opened, closed, halted, or entered an auction.
    /// **Unbacked**: no session-state tracking exists.
    fn on_session_change(&mut self, _ctx: &mut Ctx, _venue: Venue, _phase: SessionPhase) {}

    /// A book became STALE, started RECOVERING, or returned to OK.
    /// **Unbacked**: nothing detects this transition today.
    fn on_book_state_change(&mut self, _ctx: &mut Ctx, _instrument: InstrumentId, _state: BookState) {}

    /// Shutting down. Last chance to clean up. **Real** -- `main.rs`
    /// calls this once, right after the replay loop ends, before
    /// writing reports.
    fn on_stop(&mut self, _ctx: &mut Ctx) {}
}

#[cfg(test)]
mod strategy_trait_tests {
    use super::*;

    /// A minimal, local strategy -- proves `on_start` alone is enough to
    /// satisfy `Strategy` (every other method's default applies), the
    /// same "implement only what you use" guarantee the guide states.
    /// Local rather than naming `LimitOrderBookGenerator`/
    /// `NaturalGasBracket` directly: this file compiles into more than
    /// one `[[bin]]` target, and not every one declares every real
    /// strategy's module.
    struct MinimalStrategy;
    impl Strategy for MinimalStrategy {
        fn on_start(&mut self, _ctx: &mut StartCtx) {}
    }

    fn assert_is_strategy<T: Strategy>() {}

    #[test]
    fn implementing_only_on_start_satisfies_strategy() {
        assert_is_strategy::<MinimalStrategy>();
    }
}

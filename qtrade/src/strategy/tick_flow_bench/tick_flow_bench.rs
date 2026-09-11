//! Tick-flow benchmark strategy -- not a trading strategy, makes no
//! attempt at P&L. Its only job is a simple, understood, repeating
//! order-submission pattern, so a full-day backtest run against it gives
//! a known, honest reference point for "how long should a backtest like
//! this take" -- see this module's own `main.rs` for the timing report
//! it prints.
//!
//! The pattern, exactly as specified: every 10 ticks, submit one BUY
//! LimitDay order at the current best bid (1 lot). If it hasn't filled
//! within 5 ticks of being submitted, cancel it. Only one order is ever
//! in flight at a time -- a new one is submitted only once the previous
//! one has resolved (filled, canceled, or otherwise left the book).
//!
//! "Tick" here means one `on_book` callback -- one BBO-changing market
//! event for the subscribed instrument, not a wall-clock interval.

use qtrade::logging;
use qtrade::{Ctx, Depth, FillRecord, InstrumentId, Lots, OrderEventRecord, OrderState, OrderType, Price, Side, StartCtx, Strategy};

pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

const SUBMIT_EVERY_N_TICKS: u64 = 10;
const CANCEL_AFTER_N_TICKS: u64 = 5;

struct OpenOrder {
    client_order_id: u64,
    submitted_at_tick: u64,
    cancel_requested: bool,
}

pub struct TickFlowBench {
    instrument: Option<InstrumentId>,
    ticks: u64,
    open: Option<OpenOrder>,
    submitted: u64,
    filled: u64,
    timeout_cancels: u64,
}

impl TickFlowBench {
    pub fn new() -> Self {
        Self { instrument: None, ticks: 0, open: None, submitted: 0, filled: 0, timeout_cancels: 0 }
    }

    pub fn stats(&self) -> (u64, u64, u64, u64) {
        (self.ticks, self.submitted, self.filled, self.timeout_cancels)
    }

    fn on_tick(&mut self, ctx: &mut Ctx) {
        self.ticks += 1;
        let Some(instrument) = self.instrument else { return };

        // Cancel first, if the current order has overstayed its welcome --
        // checked every tick, independent of the 10-tick submit cadence.
        //
        // **A real bug, found by measuring this benchmark, not a
        // hypothetical:** this used to clear `self.open` unconditionally
        // right after calling `ctx.cancel`, regardless of whether the
        // cancel actually took effect. `ctx.cancel` is a no-op if the
        // order isn't `is_open()` yet -- in particular, if it's still
        // `Submitted` (outbound latency hasn't landed it as `Accepted`
        // at the venue yet), which can genuinely happen within 5 ticks
        // on a fast-moving book. Forgetting the order there orphans it:
        // it goes on to rest at the venue forever, never cancelled,
        // never referenced again -- a real, permanent leak into
        // `ExecutionEngine`'s open-order tracking. The fix: only clear
        // `self.open` once `on_fill`/`on_order_update` *confirms* a
        // terminal state. Left un-cleared here, the next tick's
        // `self.ticks - open.submitted_at_tick >= CANCEL_AFTER_N_TICKS`
        // stays true and retries the cancel -- self-healing once the
        // order actually reaches `Accepted`.
        if let Some(open) = &mut self.open {
            if self.ticks - open.submitted_at_tick >= CANCEL_AFTER_N_TICKS {
                let id = open.client_order_id;
                let first_attempt = !open.cancel_requested;
                open.cancel_requested = true;
                tracing::info!("{}", logging::line("TickFlowBench", Some(ctx.now()), "TIMEOUT_CANCEL", &format!("client_order_id={id} unfilled after {CANCEL_AFTER_N_TICKS} ticks -- requesting cancel{}", if first_attempt { "" } else { " (retry)" })));
                let _ = ctx.cancel(id);
                if first_attempt {
                    self.timeout_cancels += 1;
                }
            }
        }

        if self.open.is_some() || self.ticks % SUBMIT_EVERY_N_TICKS != 0 {
            return;
        }

        let Some(book) = ctx.book(instrument) else { return };
        let Some(bid) = book.best_bid() else { return };
        match ctx.submit(instrument, Side::Buy, OrderType::LimitDay(Price(bid.price.0)), Lots(1)) {
            Ok(id) => {
                tracing::info!("{}", logging::line("TickFlowBench", Some(ctx.now()), "SUBMIT", &format!("tick={} client_order_id={id} BUY 1 lot LimitDay @ Rs {:.2}", self.ticks, bid.price.0 as f64 / 100_000_000.0)));
                self.open = Some(OpenOrder { client_order_id: id, submitted_at_tick: self.ticks, cancel_requested: false });
                self.submitted += 1;
            }
            Err(e) => {
                tracing::info!("{}", logging::line("TickFlowBench", Some(ctx.now()), "SUBMIT_REFUSED", &format!("tick={} {e:?}", self.ticks)));
            }
        }
    }
}

impl Strategy for TickFlowBench {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        for name in UNDERLYINGS {
            // `ctx.resolve` panics loudly on its own if `name` doesn't
            // resolve -- see `StartCtx::resolve`'s own doc comment.
            let id = ctx.resolve(name);
            self.instrument = Some(id);
            ctx.subscribe(id, Depth::Bbo);
            tracing::info!("{}", logging::line("TickFlowBench", None, "SUBSCRIBE", &format!("{name} -- native/MCX token id={}, depth=Bbo", id.0)));
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
        self.on_tick(ctx);
    }

    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {
        if self.open.as_ref().map(|o| o.client_order_id) == Some(fill.client_order_id) {
            self.filled += 1;
            self.open = None;
        }
        tracing::info!(
            "{}",
            logging::line(
                "TickFlowBench",
                Some(ctx.now()),
                "FILL",
                &format!("client_order_id={} price=Rs {:.2} qty={} lots", fill.client_order_id, fill.price.0 as f64 / 100_000_000.0, fill.qty.0 / qtrade::RAW_QTY_PER_LOT)
            )
        );
    }

    fn on_order_update(&mut self, _ctx: &mut Ctx, update: &OrderEventRecord) {
        // A denial/rejection/cancel confirmation on our tracked order frees
        // the slot even if `on_fill` never fires for it.
        if self.open.as_ref().map(|o| o.client_order_id) == Some(update.client_order_id)
            && matches!(update.resulting_state, OrderState::Denied | OrderState::Rejected | OrderState::Canceled)
        {
            self.open = None;
        }
    }
}

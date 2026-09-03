//! Order-lifecycle demo strategy.
//!
//! Not a trading strategy -- it makes no attempt at P&L and leaves
//! inventory un-flat. Its only job is to deliberately walk an order
//! through **every order state qtrade can actually reach**, one scripted
//! action at a time on NATURALGAS, so a single `events.log` shows the
//! full `OrderState` machine instead of just the `Submitted -> Filled`
//! path a pure-taker strategy exercises.
//!
//! `OrderState` has 11 variants (`execution.rs`); this script hits 8 of
//! them:
//!
//! | State            | Step here                                        |
//! |------------------|-------------------------------------------------|
//! | `Submitted`      | every `ctx.submit` -- qtrade's own "gates passed" |
//! | `Denied`         | step 1: qty over the instrument's max single order qty (local gate) |
//! | `Rejected`       | step 2: `BookOrCancel` priced through the ask -> venue `WouldCross` |
//! | `Accepted`       | step 3: deep passive `LimitDay` that cannot fill |
//! | `PendingUpdate`  | step 4: `ctx.modify` of the resting order        |
//! | `PendingCancel`  | step 5: `ctx.cancel` of the resting order        |
//! | `Canceled`       | step 5 (venue confirms the cancel)               |
//! | `PartiallyFilled`| step 6: large crossing `LimitDay` -- sweeps what's there, rests the rest |
//! | `Filled`         | step 8: `MarketToLimit` -- fills on arrival      |
//!
//! Not reachable from a strategy today, and why:
//! - `Initialized` -- internal pre-gate state, never dispatched as an
//!   `on_order_update` (it becomes `Submitted` or `Denied` before any
//!   event is emitted).
//! - `Expired` -- `ExecutionEngine::mark_expired` exists and is tested
//!   but has no caller (no GTD / end-of-day-Lean expiry wired into the
//!   replay loop yet).
//!
//! Every `on_order_update` / `on_fill` is logged verbatim so the state
//! transitions are visible in order in `events.log`.

use crate::book::Book;
use crate::decoder::Trade;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord};
use crate::logging;
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{InstrumentId, Lots, Price, Qty, Side};

pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

/// First scripted action fires at or after this sim timestamp -- 10:05
/// IST on 21_08_2026 (market has been open 5 min, the book has real
/// depth). Derived the same way `naturalgas_bracket`'s own `ENTRY_NS`
/// was: 10:00 IST + 300s.
const START_NS: u64 = 1_787_286_600_000_000_000 + 300 * 1_000_000_000;
/// Sim-time gap between one scripted step and the next -- wide enough
/// that each step's own venue round trip (outbound + inbound latency)
/// resolves and gets logged before the next step starts.
const STEP_GAP_NS: u64 = 20 * 1_000_000_000;

const RUPEE_RAW: f64 = 100_000_000.0;
/// Fallback NATURALGAS tick (Rs 0.10 in wire units) if refdata lookup
/// somehow fails.
const NATURALGAS_TICK_FALLBACK: i64 = 10_000_000;
const MAX_ORDER_QTY_FALLBACK_LOTS: i64 = 1_000;
/// How many ticks step 6 will wait for a touch thin enough to partial
/// against before giving up and submitting anyway -- see that step.
const PARTIAL_MAX_ATTEMPTS: u32 = 500;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Step {
    WaitStart,
    Denied,
    Rejected,
    Accept,
    Modify,
    Cancel,
    PartialFill,
    CancelRemainder,
    FullFill,
    Done,
}

pub struct OrderLifecycleDemo {
    instrument: Option<InstrumentId>,
    step: Step,
    next_at_ns: u64,
    resting_id: Option<u64>,
    partial_id: Option<u64>,
    partial_attempts: u32,
}

impl OrderLifecycleDemo {
    pub fn new() -> Self {
        Self { instrument: None, step: Step::WaitStart, next_at_ns: START_NS, resting_id: None, partial_id: None, partial_attempts: 0 }
    }

    /// `main.rs`'s summary loop calls this on every strategy; this one
    /// has no round trips to report.
    pub fn round_trips(&self) -> &[(&'static str, i64, i64, &'static str)] {
        &[]
    }

    fn tick_raw(ctx: &Ctx, instrument: InstrumentId) -> i64 {
        ctx.refdata().get(instrument).map(|i| i.tick_size.0).filter(|t| *t > 0).unwrap_or(NATURALGAS_TICK_FALLBACK)
    }

    fn max_order_lots(ctx: &Ctx, instrument: InstrumentId) -> i64 {
        let f = ctx.refdata().get(instrument).map(|i| i.max_single_order_qty).unwrap_or(MAX_ORDER_QTY_FALLBACK_LOTS);
        if f > 0 { f } else { MAX_ORDER_QTY_FALLBACK_LOTS }
    }

    /// The whole script -- one step per call, only once `now_ns` has
    /// reached `next_at_ns`. Reads the live book each time; if either
    /// side is missing it just waits for the next tick without advancing.
    fn run_script(&mut self, ctx: &mut Ctx, now_ns: u64) {
        if now_ns < self.next_at_ns || self.step == Step::Done {
            return;
        }
        let Some(instrument) = self.instrument else { return };
        let Some(book) = ctx.book(instrument) else { return };
        let (Some(bid), Some(ask)) = (book.best_bid(), book.best_ask()) else { return };
        let tick = Self::tick_raw(ctx, instrument);
        let bid_raw = bid.price.0;
        let ask_raw = ask.price.0;

        macro_rules! log {
            ($tag:expr, $($arg:tt)*) => {{
                tracing::info!("{}", logging::line("OrderLifecycleDemo", Some(now_ns), $tag, &format!($($arg)*)))
            }};
        }

        let advance = |s: &mut Self, next: Step| {
            s.step = next;
            s.next_at_ns = now_ns + STEP_GAP_NS;
        };

        match self.step {
            Step::WaitStart => {
                log!("SCRIPT", "starting -- NATURALGAS bid Rs {:.2} / ask Rs {:.2}, tick Rs {:.2}", bid_raw as f64 / RUPEE_RAW, ask_raw as f64 / RUPEE_RAW, tick as f64 / RUPEE_RAW);
                advance(self, Step::Denied);
            }

            // 1. DENIED -- qty over the instrument's max single order quantity
            //    (MCX: "maximum single transaction quantity") trips
            //    the local Validation gate; the order never reaches the venue.
            Step::Denied => {
                let over = Lots(Self::max_order_lots(ctx, instrument) + 100);
                let px = Price(bid_raw - 20 * tick); // deep, harmless price
                log!("STEP 1/8 DENIED", "submit BUY {} lots (over max single order qty) LimitDay @ Rs {:.2} -- expect state=Denied (local gate)", over.0, px.0 as f64 / RUPEE_RAW);
                let _ = ctx.submit(instrument, Side::Buy, OrderType::LimitDay(px), over);
                advance(self, Step::Rejected);
            }

            // 2. REJECTED -- BookOrCancel is post-only; priced at the ask
            //    it would cross on arrival, so the venue rejects it outright.
            Step::Rejected => {
                let px = Price(ask_raw); // at the ask -> a buy BOC would cross
                log!("STEP 2/8 REJECTED", "submit BUY 1 lot BookOrCancel @ Rs {:.2} (would cross) -- expect state=Rejected (venue: WouldCross)", px.0 as f64 / RUPEE_RAW);
                let _ = ctx.submit(instrument, Side::Buy, OrderType::BookOrCancel(px), Lots(1));
                advance(self, Step::Accept);
            }

            // 3. ACCEPTED -- a deep passive LimitDay that cannot fill.
            Step::Accept => {
                let px = Price(bid_raw - 15 * tick); // deep passive -- will not fill
                log!("STEP 3/8 ACCEPTED", "submit BUY 1 lot LimitDay @ Rs {:.2} (deep passive) -- expect state=Submitted then Accepted (resting)", px.0 as f64 / RUPEE_RAW);
                match ctx.submit(instrument, Side::Buy, OrderType::LimitDay(px), Lots(1)) {
                    Ok(id) => {
                        self.resting_id = Some(id);
                        log!("STEP 3/8 ACCEPTED", "resting order client_order_id={id}");
                    }
                    Err(e) => log!("STEP 3/8 ACCEPTED", "submit refused: {e:?}"),
                }
                advance(self, Step::Modify);
            }

            // 4. PENDING_UPDATE -> ACCEPTED -- move the resting order to a
            //    new (still passive) price. Price change loses queue
            //    priority on MCX, but state-wise it goes PendingUpdate then
            //    back to Accepted.
            Step::Modify => {
                if let Some(id) = self.resting_id {
                    let new_px = Price(bid_raw - 8 * tick);
                    log!("STEP 4/8 PENDING_UPDATE", "modify client_order_id={id} -> Rs {:.2}, qty 1 lot -- expect state=PendingUpdate then Accepted", new_px.0 as f64 / RUPEE_RAW);
                    let _ = ctx.modify(id, Lots(1).to_raw_qty(), Some(new_px));
                } else {
                    log!("STEP 4/8 PENDING_UPDATE", "no resting order to modify -- skipped");
                }
                advance(self, Step::Cancel);
            }

            // 5. PENDING_CANCEL -> CANCELED -- pull the resting order.
            Step::Cancel => {
                if let Some(id) = self.resting_id.take() {
                    log!("STEP 5/8 PENDING_CANCEL", "cancel client_order_id={id} -- expect state=PendingCancel then Canceled");
                    let _ = ctx.cancel(id);
                } else {
                    log!("STEP 5/8 PENDING_CANCEL", "no resting order to cancel -- skipped");
                }
                advance(self, Step::PartialFill);
            }

            // 6. PARTIALLY_FILLED -- a BUY LimitDay at the touch, sized at
            //    this instrument's real max single order quantity (its maximum order
            //    size). Reworked 2026-09-03: `max_single_order_qty` is now the real
            //    per-day value from the contract file (48 lots for
            //    NATURALGAS), not the old blanket 1,000-lot demo override,
            //    so the previous trick -- dwarf the touch with a huge
            //    order -- is no longer possible: nothing may exceed the
            //    cap, and the cap is smaller than the touch often is.
            //
            //    Instead this waits for a tick where the best-ask level is
            //    itself thinner than the cap, then takes all of it and
            //    rests the remainder -- a genuine partial. If the book
            //    stays thicker than the cap for `PARTIAL_MAX_ATTEMPTS`
            //    ticks it submits anyway; a full fill still exercises the
            //    path, it just doesn't demonstrate `PartiallyFilled`, and
            //    the log says which happened.
            Step::PartialFill => {
                let cap_lots = Self::max_order_lots(ctx, instrument);
                let touch_lots = ask.qty.0 / crate::types::RAW_QTY_PER_LOT;
                if touch_lots >= cap_lots && self.partial_attempts < PARTIAL_MAX_ATTEMPTS {
                    // Touch is at least as deep as the most we may send --
                    // a full fill, not a partial. Wait for a thinner one
                    // (`next_at_ns` is untouched, so the next tick retries).
                    self.partial_attempts += 1;
                    return;
                }
                let px = Price(ask_raw); // touch-only: matches just the best ask
                let qty = Lots(cap_lots);
                log!("STEP 6/8 PARTIAL", "submit BUY {} lots (= max single order qty) LimitDay @ Rs {:.2} against a {}-lot touch after {} wait(s) -- expect {}", qty.0, px.0 as f64 / RUPEE_RAW, touch_lots, self.partial_attempts, if touch_lots < cap_lots { "a partial Filled then state=PartiallyFilled for the working remainder" } else { "a full Filled (gave up waiting for a thin touch)" });
                match ctx.submit(instrument, Side::Buy, OrderType::LimitDay(px), qty) {
                    Ok(id) => {
                        self.partial_id = Some(id);
                        log!("STEP 6/8 PARTIAL", "order client_order_id={id}");
                    }
                    Err(e) => log!("STEP 6/8 PARTIAL", "submit refused: {e:?}"),
                }
                advance(self, Step::CancelRemainder);
            }

            // 7. cancel the working remainder from step 6 -> PendingCancel/Canceled again,
            //    this time from a PartiallyFilled order.
            Step::CancelRemainder => {
                if let Some(id) = self.partial_id.take() {
                    log!("STEP 7/8 CANCEL REMAINDER", "cancel the partially-filled order client_order_id={id} -- expect PendingCancel then Canceled");
                    let _ = ctx.cancel(id);
                } else {
                    log!("STEP 7/8 CANCEL REMAINDER", "no partial order to cancel -- skipped");
                }
                advance(self, Step::FullFill);
            }

            // 8. FILLED -- a plain MarketToLimit that fills on arrival,
            //    the same Submitted -> Filled path a pure taker shows.
            Step::FullFill => {
                log!("STEP 8/8 FILLED", "submit SELL 1 lot MarketToLimit -- expect state=Submitted then Filled (no Accepted, matches MCX 10103 Immediate Execution Response)");
                let _ = ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1));
                advance(self, Step::Done);
                log!("SCRIPT", "done -- see the order-state transitions above");
            }

            Step::Done => {}
        }
    }
}

impl Strategy for OrderLifecycleDemo {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        for name in UNDERLYINGS {
            if let Some(id) = ctx.resolve(name) {
                self.instrument = Some(id);
                ctx.subscribe(id, Depth::Bbo);
                tracing::info!("{}", logging::line("OrderLifecycleDemo", None, "SUBSCRIBE", &format!("{name} -- native/MCX token id={}, depth=Bbo", id.0)));
            } else {
                tracing::info!("{}", logging::line("OrderLifecycleDemo", None, "SUBSCRIBE", &format!("{name} -- NOT resolved in this day's refdata")));
            }
        }
        tracing::info!("{}", logging::line("OrderLifecycleDemo", None, "START", "order-lifecycle demo armed -- first scripted action at 10:05 IST"));
    }

    fn on_book(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, packet_transact_time_ns: u64) {
        self.run_script(ctx, packet_transact_time_ns);
    }

    fn on_trade(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _trade: &Trade, _seq: u64, packet_transact_time_ns: u64) {
        self.run_script(ctx, packet_transact_time_ns);
    }

    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {
        tracing::info!(
            "{}",
            logging::line(
                "OrderLifecycleDemo",
                Some(ctx.now()),
                "FILL",
                &format!(
                    "client_order_id={} side={:?} price=Rs {:.2} qty={} lots kind={:?} queue_pos_at_fill={}",
                    fill.client_order_id,
                    fill.side,
                    fill.price.0 as f64 / RUPEE_RAW,
                    fill.qty.0 / crate::types::RAW_QTY_PER_LOT,
                    fill.kind,
                    fill.queue_position_at_fill.map(|q| q.to_string()).unwrap_or_else(|| "--".into()),
                )
            )
        );
    }

    fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord) {
        tracing::info!(
            "{}",
            logging::line(
                "OrderLifecycleDemo",
                Some(ctx.now()),
                "ORDER_UPDATE",
                &format!("client_order_id={} state={:?} -- {}", update.client_order_id, update.resulting_state, update.description)
            )
        );
    }
}

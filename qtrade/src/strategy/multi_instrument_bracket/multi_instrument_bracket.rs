//! A deliberately more complete real strategy than `naturalgas_bracket`
//! (2026-08-28) -- written specifically to exercise the parts of this
//! project's own machinery that no real strategy has touched yet:
//!
//! - **Two instruments at once** (`NATURALGAS`, `CRUDEOIL`), each with
//!   fully independent state -- proves the per-instrument dispatch
//!   (`EventDispatcher`'s own keying) actually works for more than one
//!   subscriber target, not just the one `naturalgas_bracket` used.
//! - **Fully aggressive entries and exits** (`OrderType::MarketToLimit`
//!   both ways) -- a real, evidence-based reversal (2026-08-28) from an
//!   earlier passive-resting-entry design that produced zero fills over a
//!   full real session (queue-priority churn from re-pricing). Aggressive
//!   orders trade for real, at the cost of not controlling the exact
//!   entry/exit price.
//! - **A time-boxed exit**: if neither the take-profit nor the stop-loss
//!   threshold is crossed within `EXIT_TIMEOUT_NS` of the entry fill, the
//!   position is closed at market anyway -- no position is ever held
//!   indefinitely.
//! - **Continuous re-entry** all session long (`CoolingDown` ->
//!   `BeforeEntry`), not a single trade -- this is meant to actually
//!   generate a real trade log, not just prove a bracket completes once.
//!
//! Not written to make money -- TP/SL are a tight, real, evidence-based
//! ±0.5%/1% band (see `SL_PER_MILLE`/`TP_PER_MILLE`).

use crate::decoder::Trade;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord, OrderState};
use crate::logging;
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{InstrumentId, Lots, Side};

pub const UNDERLYINGS: &[&str] = &["NATURALGAS", "CRUDEOIL"];

/// 10:00:00 IST, 2026-08-21 -- same verified-safe day and entry instant
/// `naturalgas_bracket` already established (see that strategy's own doc
/// comment for why this specific day, not any earlier one). Only gates
/// the *first* entry of the session -- every re-entry after that goes
/// straight from `CoolingDown` back to `BeforeEntry`, which submits
/// immediately since `now_ns` is already well past this threshold.
const ENTRY_NS: u64 = 1_787_286_600_000_000_000;
/// 23:30:00 IST, 2026-08-21 -- the real session end. **Found via a real
/// run (2026-08-28): the exchange itself broadcasts a `MassDelete` right
/// at session close (real MCX behavior, already exercised by
/// `simulator.rs`'s own `mass_delete_cancels_our_own_resting_order`
/// test), purging every resting order an instant before any force-close
/// attempt keyed to this exact timestamp can win the race.**
/// `EOD_EXIT_NS` below, not this constant, is what actually gates the
/// force-close.
const EOD_NS: u64 = 1_787_335_200_000_000_000;
/// Trigger the force-close this far before the real session end
/// (`EOD_NS`), so it has a real chance to execute against a still-live
/// book instead of racing the exchange's own end-of-session `MassDelete`.
const EOD_EXIT_BUFFER_NS: u64 = 5 * 60 * 1_000_000_000;
const EOD_EXIT_NS: u64 = EOD_NS - EOD_EXIT_BUFFER_NS;
const REENTRY_GAP_NS: u64 = 5 * 60 * 1_000_000_000;
/// If neither TP nor SL has fired this long after the entry fill, close
/// the position at market anyway.
const EXIT_TIMEOUT_NS: u64 = 176 * 1_000_000_000;
/// 1% -- how far the bid can fall below the entry price before this
/// strategy exits at a loss rather than waiting out the full
/// `EXIT_TIMEOUT_NS` window.
const SL_PER_MILLE: i64 = 990;
/// 0.5% -- the target. `1.005x` the entry price.
const TP_PER_MILLE: i64 = 1_005;
const RUPEE_RAW: f64 = 100_000_000.0;

fn rupees(raw: i64) -> f64 {
    raw as f64 / RUPEE_RAW
}

fn log(now_ns: u64, component: &str, tag: &str, msg: &str) {
    tracing::info!("{}", logging::line(component, Some(now_ns), tag, msg));
}

/// Reads and logs this strategy's own current position/PnL through `ctx`
/// -- called after every real fill, so "portfolio updated" is an
/// observed fact (what `ctx.position`/`ctx.pnl` actually report
/// immediately after `Portfolio::apply_fill` ran inside
/// `ExecutionEngine`), not an assumption the strategy computes itself.
/// A free function, not a method: every call site already holds a
/// mutable borrow of one `InstrumentState` inside `self.states`, and a
/// `&self` method here would conflict with that borrow for no reason --
/// this needs `ctx`, nothing from `self` at all.
fn log_portfolio(ctx: &Ctx, name: &str, instrument: InstrumentId, now_ns: u64) {
    let position = ctx.position(instrument);
    let pnl = ctx.pnl();
    log(now_ns, "MultiInstrumentBracket", "PORTFOLIO", &format!("{name}: position={position} lot(s) (firm), gross_pnl=Rs {:.4}, net_pnl=Rs {:.4}", pnl.gross, pnl.net));
}

enum Phase {
    BeforeEntry,
    /// Aggressive entry submitted, not yet confirmed -- resolved by
    /// `on_fill` (success) or `on_order_update` (a real `Denied`/
    /// `Rejected`, which sends this back to `BeforeEntry` to retry).
    WaitingForEntryFill { client_order_id: u64 },
    /// Filled and open, being monitored every tick against TP, SL, and
    /// `EXIT_TIMEOUT_NS` -- no resting orders at all, just a book read.
    Open { entry_price_raw: i64, entry_ns: u64 },
    WaitingForExitFill { client_order_id: u64, entry_price_raw: i64, reason: &'static str, forced_eod: bool },
    CoolingDown { reenter_at_ns: u64 },
    Done,
}

struct InstrumentState {
    instrument: InstrumentId,
    name: &'static str,
    phase: Phase,
}

pub struct MultiInstrumentBracket {
    states: Vec<InstrumentState>,
    /// (name, entry_raw, exit_raw, reason) -- instrument name kept
    /// alongside the prices since this strategy, unlike
    /// `naturalgas_bracket`, trades more than one.
    round_trips: Vec<(&'static str, i64, i64, &'static str)>,
}

impl MultiInstrumentBracket {
    pub fn new() -> Self {
        MultiInstrumentBracket { states: Vec::new(), round_trips: Vec::new() }
    }

    pub fn round_trips(&self) -> &[(&'static str, i64, i64, &'static str)] {
        &self.round_trips
    }

    /// The one real per-tick decision loop, run once per subscribed
    /// instrument per real event -- shared by `on_book`/`on_trade`, same
    /// convention `naturalgas_bracket` uses.
    fn tick_state(ctx: &mut Ctx, state: &mut InstrumentState, now_ns: u64) {
        let instrument = state.instrument;
        let name = state.name;

        // End of day: force-close whatever's open, then stop acting for
        // the rest of the run.
        if now_ns >= EOD_EXIT_NS {
            match state.phase {
                Phase::Open { entry_price_raw, .. } => {
                    log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: EOD reached with a position still open -- exiting at market"));
                    match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            log(now_ns, "MultiInstrumentBracket", "PLACING_ORDER", &format!("{name}: SELL 1 lot MarketToLimit (EOD force-close), client_order_id={client_order_id}, entry was Rs {:.2}", rupees(entry_price_raw)));
                            state.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason: "EOD", forced_eod: true };
                        }
                        Err(e) => log(now_ns, "MultiInstrumentBracket", "ERROR", &format!("{name}: EOD force-close submit failed: {e:?}")),
                    }
                }
                Phase::WaitingForEntryFill { .. } => {
                    log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: EOD reached with the entry order still in flight -- not re-entering"));
                    state.phase = Phase::Done;
                }
                Phase::BeforeEntry | Phase::CoolingDown { .. } => state.phase = Phase::Done,
                Phase::WaitingForExitFill { .. } | Phase::Done => {}
            }
            return;
        }

        match &state.phase {
            Phase::BeforeEntry => {
                if now_ns >= ENTRY_NS {
                    log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: entering aggressively"));
                    match ctx.submit(instrument, Side::Buy, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            log(now_ns, "MultiInstrumentBracket", "PLACING_ORDER", &format!("{name}: BUY 1 lot MarketToLimit (aggressive entry), client_order_id={client_order_id}"));
                            state.phase = Phase::WaitingForEntryFill { client_order_id };
                        }
                        Err(e) => log(now_ns, "MultiInstrumentBracket", "ERROR", &format!("{name}: entry submit failed: {e:?}")),
                    }
                }
            }
            Phase::WaitingForEntryFill { .. } => {
                // Nothing to do here -- resolved by on_fill (success) or
                // on_order_update (Denied/Rejected sends this back to
                // BeforeEntry).
            }
            Phase::Open { entry_price_raw, entry_ns } => {
                let entry_price_raw = *entry_price_raw;
                let entry_ns = *entry_ns;
                let Some(book) = ctx.book(instrument) else { return };
                let Some(bid) = book.best_bid() else { return };
                let tp = entry_price_raw * TP_PER_MILLE / 1000;
                let sl = entry_price_raw * SL_PER_MILLE / 1000;
                let reason = if bid.price.0 >= tp {
                    Some("TP")
                } else if bid.price.0 <= sl {
                    Some("SL")
                } else if now_ns >= entry_ns + EXIT_TIMEOUT_NS {
                    Some("TIMEOUT")
                } else {
                    None
                };
                if let Some(reason) = reason {
                    log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: {reason} @ bid Rs {:.2} (entry Rs {:.2}, tp=Rs {:.2}, sl=Rs {:.2}) -- exiting at market", rupees(bid.price.0), rupees(entry_price_raw), rupees(tp), rupees(sl)));
                    match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            log(now_ns, "MultiInstrumentBracket", "PLACING_ORDER", &format!("{name}: SELL 1 lot MarketToLimit ({reason} exit), client_order_id={client_order_id}"));
                            state.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod: false };
                        }
                        Err(e) => log(now_ns, "MultiInstrumentBracket", "ERROR", &format!("{name}: {reason} exit submit failed: {e:?}")),
                    }
                }
            }
            Phase::CoolingDown { reenter_at_ns } => {
                if now_ns >= *reenter_at_ns {
                    log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: re-entry cooldown elapsed -- returning to BeforeEntry"));
                    state.phase = Phase::BeforeEntry;
                }
            }
            Phase::WaitingForExitFill { .. } | Phase::Done => {}
        }
    }
}

impl Strategy for MultiInstrumentBracket {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        tracing::info!("{}", logging::line("MultiInstrumentBracket", None, "START", "fired (pre-replay -- no sim timestamp exists yet)"));
        for name in UNDERLYINGS {
            match ctx.resolve(name) {
                Some(id) => {
                    ctx.subscribe(id, Depth::Bbo);
                    tracing::info!("{}", logging::line("MultiInstrumentBracket", None, "SUBSCRIBE", &format!("{name} -- native/MCX token id={}, depth=Bbo", id.0)));
                    self.states.push(InstrumentState { instrument: id, name, phase: Phase::BeforeEntry });
                }
                None => tracing::info!("{}", logging::line("MultiInstrumentBracket", None, "SUBSCRIBE_FAILED", &format!("{name} did not resolve to a real front-month future today -- not subscribed"))),
            }
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, packet_transact_time_ns: u64) {
        if let Some(state) = self.states.iter_mut().find(|s| s.instrument == instrument) {
            Self::tick_state(ctx, state, packet_transact_time_ns);
        }
    }

    fn on_trade(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _trade: &Trade, _seq: u64, packet_transact_time_ns: u64) {
        if let Some(state) = self.states.iter_mut().find(|s| s.instrument == instrument) {
            Self::tick_state(ctx, state, packet_transact_time_ns);
        }
    }

    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {
        let now_ns = ctx.now();
        let Some(state) = self.states.iter_mut().find(|s| s.instrument == fill.instrument) else { return };
        let name = state.name;
        match state.phase {
            Phase::WaitingForEntryFill { client_order_id } if client_order_id == fill.client_order_id => {
                log(now_ns, "MultiInstrumentBracket", "FILL", &format!("{name}: entry BUY filled @ Rs {:.2} (client_order_id={client_order_id})", rupees(fill.price.0)));
                state.phase = Phase::Open { entry_price_raw: fill.price.0, entry_ns: now_ns };
                log_portfolio(ctx, name, fill.instrument, now_ns);
            }
            Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod } if client_order_id == fill.client_order_id => {
                let pnl_rupees = rupees(fill.price.0) - rupees(entry_price_raw);
                log(now_ns, "MultiInstrumentBracket", "FILL", &format!("{name}: {reason} SELL filled @ Rs {:.2} (client_order_id={client_order_id}) -- round trip: entry Rs {:.2} -> exit Rs {:.2}, {:+.2} Rs/lot before costs", rupees(fill.price.0), rupees(entry_price_raw), rupees(fill.price.0), pnl_rupees));
                self.round_trips.push((name, entry_price_raw, fill.price.0, reason));
                log_portfolio(ctx, name, fill.instrument, now_ns);
                state.phase = if forced_eod { Phase::Done } else { Phase::CoolingDown { reenter_at_ns: fill.timestamp_ns + REENTRY_GAP_NS } };
            }
            _ => {}
        }
    }

    fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord) {
        let now_ns = ctx.now();
        log(now_ns, "MultiInstrumentBracket", "ORDER_UPDATE", &format!("client_order_id={} state={:?} -- {}", update.client_order_id, update.resulting_state, update.description));

        // Aggressive orders can still be Denied (a local gate) or
        // Rejected (the venue) -- e.g. NoLiquidityForResidual on a
        // MarketToLimit with nothing to sweep. Retry rather than getting
        // stuck: an entry failure goes back to BeforeEntry (resubmits
        // next tick); an exit failure goes back to Open with entry_ns
        // reset to 0, so tick_state's timeout condition is immediately
        // true and the very next tick retries the exit.
        if !matches!(update.resulting_state, OrderState::Denied | OrderState::Rejected) {
            return;
        }
        let Some(state) = self.states.iter_mut().find(|s| match &s.phase {
            Phase::WaitingForEntryFill { client_order_id } => *client_order_id == update.client_order_id,
            Phase::WaitingForExitFill { client_order_id, .. } => *client_order_id == update.client_order_id,
            _ => false,
        }) else { return };
        let name = state.name;
        match state.phase {
            Phase::WaitingForEntryFill { .. } => {
                log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: entry order failed ({:?}) -- retrying on the next tick", update.resulting_state));
                state.phase = Phase::BeforeEntry;
            }
            Phase::WaitingForExitFill { entry_price_raw, .. } => {
                log(now_ns, "MultiInstrumentBracket", "ALARM", &format!("{name}: exit order failed ({:?}) -- retrying on the next tick", update.resulting_state));
                state.phase = Phase::Open { entry_price_raw, entry_ns: 0 };
            }
            _ => {}
        }
    }
}

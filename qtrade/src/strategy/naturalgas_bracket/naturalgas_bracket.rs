//! A real trading strategy: buy NATURALGAS once, at a fixed time of day,
//! bracket the fill with a stop-loss and a take-profit, and repeat for
//! the rest of the session. The first strategy in this project to
//! actually place an order -- everything before this (`event_dispatcher`/
//! `control_dispatcher`/`strategy::Ctx`'s `submit`/`cancel`/`order`/
//! `position`/`pnl`, built earlier the same day) existed to make this
//! possible, but nothing had exercised it for real yet.
//!
//! **The rules, as given, with the real gaps this codebase has today:**
//!
//! - **Entry**: buy 1 lot at the first market event at or after
//!   `ENTRY_NS` (10:00:00 IST, 2026-08-21). No timer exists anywhere in
//!   this codebase (`ctx.set_timer()` isn't built -- see
//!   `strategy/strategy.rs`'s own scope notes) -- this is a threshold
//!   check on `ctx.now()`, evaluated every `on_book`/`on_trade` tick, not
//!   a real wall-clock alarm. Accurate to the nearest tick of market
//!   activity, not to the second. Logged as an "ALARM" line regardless
//!   (2026-08-27 logging pass) -- it *behaves* like one from the
//!   strategy's point of view (a threshold crossing that triggers an
//!   action), even though the mechanism underneath is tick-polling, not
//!   a scheduled timer. See `strategy/README.md` for the honest
//!   distinction.
//! - **Bracket**: stop-loss at `SL_PER_MILLE`/1000, take-profit at
//!   `TP_PER_MILLE`/1000 of the fill price -- tightened again on
//!   2026-08-27 (from 98%/102% to 99.5%/100.5%) specifically because the
//!   real `21_08_2026` run showed **zero** triggers all session at
//!   ±2% -- NATURALGAS's real intraday range that day never moved that
//!   far from the 10:00 entry price. ±0.5% is still a real, considered
//!   band, not an arbitrary tightening for its own sake: tight enough to
//!   actually observe round trips, wide enough not to be pure noise
//!   trading on tick-to-tick jitter. `simulator::OrderType` has no
//!   stop-order variant at all -- this strategy *is* the stop, watching
//!   `ctx.book`'s best bid on every tick and submitting a real market
//!   sell the instant it's crossed. Not a resting order sitting at the
//!   exchange; a reaction, one tick after the crossing print.
//! - **Order type, both sides**: `MarketToLimit` -- guarantees the
//!   entry happens at ~10:00 and the exit happens once triggered, at
//!   the cost of not controlling the exact fill price.
//! - **Re-entry**: 5 minutes after the exit fill, repeating for the
//!   rest of the session.
//! - **End of day**: force-closes any still-open position at `EOD_NS`
//!   (23:30:00 IST) -- real, checked-against-the-capture-file cutoff,
//!   not a guess (see `naturalgas_bracket.md`).
//! - **No `Denied`/`Rejected` handling.** If a submit is ever locally
//!   denied or venue-rejected, this strategy has no retry logic -- it
//!   would simply wait forever for a fill that never comes (visible now,
//!   since 2026-08-27's logging pass, as a printed `on_order_update`
//!   line showing the real rejection, rather than silence). Accepted for
//!   this first version; a real strategy would need to.
//!
//! **Logging (2026-08-27, revised same day for low latency):** every
//! line below goes through `crate::logging::line` + `tracing::info!` --
//! the same shared, off-hot-path mechanism every other component uses
//! (see `logging.rs`'s own header). This file used to keep a private
//! `log`/`fmt_ist`/`civil_from_days` -- deleted in favor of the one
//! shared copy, not a fourth. `now_ns` is always `ctx.now()`, a genuinely
//! physical/simulated timestamp: the recorder's own receipt time for
//! whatever tick triggered an `on_book`/`on_trade`-adjacent decision, or
//! the scheduled delivery time for `on_fill`/`on_order_update` -- never
//! wall-clock-at-log-time. A strategy's own log is exactly as
//! trustworthy a timeline as `orders.log`/`fills.log`.

use crate::book::Book;
use crate::decoder::Trade;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord};
use crate::logging;
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{InstrumentId, Lots, Side};

pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

/// 10:00:00 IST, 2026-08-21 -- switched from 2026-08-19 for the
/// dual-clock replay verification run (2026-08-27): `19_08_2026` predates
/// the recording rig's NTP sync (~2026-08-20) between its two servers,
/// and a real negative feed-latency delta was found partway through that
/// file's session (Q1's fail-fast, working as designed) -- `21_08_2026`
/// is the first full day after sync, verified clean (zero negative
/// deltas across a 60M-record real scan). **Pick a day at or after this
/// one** for any future run against real data -- see
/// `naturalgas_bracket.md` and `main_user_doc.md` for the full account of
/// why `19_08_2026` (and, by the same reasoning, anything before
/// `20_08_2026`) is not safe input for the dual-clock model. Verified for
/// real against this file: the first real `PacketHeader.TransactTime`
/// decodes to `2026-08-21 08:30:23.67 IST`, a plausible pre-session time.
const ENTRY_NS: u64 = 1_787_286_600_000_000_000;
/// 23:30:00 IST, 2026-08-21 -- same day switch as `ENTRY_NS` above.
const EOD_NS: u64 = 1_787_335_200_000_000_000;
const REENTRY_GAP_NS: u64 = 5 * 60 * 1_000_000_000;
/// Per-mille, not percent (2026-08-27) -- `SL_NUM`/`TP_NUM` at 98/102
/// (±2%) never triggered once across the entire `21_08_2026` session;
/// this codebase's own real run is the evidence. ±0.5% (`995`/`1005` of
/// 1000) is the tightened replacement -- still a real, chosen band, not
/// noise-trading tick jitter, but tight enough to actually produce
/// visible round trips against NATURALGAS's real intraday range.
const SL_PER_MILLE: i64 = 995;
const TP_PER_MILLE: i64 = 1_005;
const RUPEE_RAW: f64 = 100_000_000.0;

fn rupees(raw: i64) -> f64 {
    raw as f64 / RUPEE_RAW
}

/// One line, every log call in this file goes through this --
/// `crate::logging::line` builds the shared `t=<raw> (<IST>) [component]
/// tag: msg` format, `tracing::info!` delivers it off the hot path (see
/// `logging.rs`'s own header). Component name fixed to this strategy's
/// own, so call sites only ever supply `now_ns`/`tag`/`msg`.
fn log(now_ns: u64, tag: &str, msg: &str) {
    tracing::info!("{}", logging::line("NaturalGasBracket", Some(now_ns), tag, msg));
}

enum Phase {
    BeforeEntry,
    WaitingForEntryFill { client_order_id: u64 },
    Open { entry_price_raw: i64 },
    WaitingForExitFill { client_order_id: u64, entry_price_raw: i64, reason: &'static str, forced_eod: bool },
    CoolingDown { reenter_at_ns: u64 },
    Done,
}

pub struct NaturalGasBracket {
    instrument: Option<InstrumentId>,
    phase: Phase,
    round_trips: Vec<(i64, i64, &'static str)>, // (entry_raw, exit_raw, reason)
}

impl NaturalGasBracket {
    pub fn new() -> Self {
        NaturalGasBracket { instrument: None, phase: Phase::BeforeEntry, round_trips: Vec::new() }
    }

    pub fn round_trips(&self) -> &[(i64, i64, &'static str)] {
        &self.round_trips
    }

    /// Reads and logs this strategy's own current position/PnL through
    /// `ctx` -- called after every real fill, so "portfolio updated" is
    /// an observed fact (what `ctx.position`/`ctx.pnl` actually report
    /// immediately after `Portfolio::apply_fill` ran inside
    /// `ExecutionEngine`), not an assumption the strategy computes itself.
    fn log_portfolio(&self, ctx: &Ctx, instrument: InstrumentId, now_ns: u64) {
        let position = ctx.position(instrument);
        let pnl = ctx.pnl();
        log(now_ns, "PORTFOLIO", &format!("position={position} lot(s), gross_pnl=Rs {:.4}, net_pnl=Rs {:.4}", pnl.gross, pnl.net));
    }

    /// The one real per-tick decision loop -- shared by `on_book` and
    /// `on_trade`, since both just mean "something happened, check the
    /// clock and the book again." Does nothing if `on_start` never
    /// resolved a real instrument (shouldn't happen in practice, but
    /// this strategy has no business guessing one).
    fn tick(&mut self, ctx: &mut Ctx, now_ns: u64) {
        let Some(instrument) = self.instrument else { return };

        // End of day: force-close whatever's open, then stop acting for
        // the rest of the run -- checked before the normal phase match so
        // it pre-empts a fresh entry this same tick.
        if now_ns >= EOD_NS {
            if let Phase::Open { entry_price_raw } = self.phase {
                log(now_ns, "ALARM", "EOD threshold reached -- firing force-close handler");
                match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                    Ok(client_order_id) => {
                        log(now_ns, "PLACING_ORDER", &format!("SELL 1 lot MarketToLimit (EOD force-close), instrument={instrument:?}, client_order_id={client_order_id}, entry was Rs {:.2}", rupees(entry_price_raw)));
                        self.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason: "EOD", forced_eod: true };
                    }
                    Err(e) => log(now_ns, "ERROR", &format!("EOD force-close submit failed: {e:?}")),
                }
                return;
            }
            if matches!(self.phase, Phase::BeforeEntry | Phase::CoolingDown { .. }) {
                self.phase = Phase::Done;
            }
            return;
        }

        match self.phase {
            Phase::BeforeEntry => {
                if now_ns >= ENTRY_NS {
                    log(now_ns, "ALARM", "entry threshold reached -- firing entry handler");
                    match ctx.submit(instrument, Side::Buy, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            log(now_ns, "PLACING_ORDER", &format!("BUY 1 lot MarketToLimit, instrument={instrument:?}, client_order_id={client_order_id}"));
                            self.phase = Phase::WaitingForEntryFill { client_order_id };
                        }
                        Err(e) => log(now_ns, "ERROR", &format!("entry submit failed: {e:?}")),
                    }
                }
            }
            Phase::Open { entry_price_raw } => {
                let Some(book) = ctx.book(instrument) else { return };
                let Some(bid) = book.best_bid() else { return };
                let sl = entry_price_raw * SL_PER_MILLE / 1000;
                let tp = entry_price_raw * TP_PER_MILLE / 1000;
                let reason = if bid.price.0 <= sl {
                    Some("SL")
                } else if bid.price.0 >= tp {
                    Some("TP")
                } else {
                    None
                };
                if let Some(reason) = reason {
                    log(now_ns, "ALARM", &format!("{reason} threshold crossed @ bid Rs {:.2} (entry Rs {:.2}, sl=Rs {:.2}, tp=Rs {:.2}) -- firing exit handler", rupees(bid.price.0), rupees(entry_price_raw), rupees(sl), rupees(tp)));
                    match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            log(now_ns, "PLACING_ORDER", &format!("SELL 1 lot MarketToLimit ({reason} exit), instrument={instrument:?}, client_order_id={client_order_id}"));
                            self.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod: false };
                        }
                        Err(e) => log(now_ns, "ERROR", &format!("exit submit failed: {e:?}")),
                    }
                }
            }
            Phase::CoolingDown { reenter_at_ns } => {
                if now_ns >= reenter_at_ns {
                    log(now_ns, "ALARM", "re-entry cooldown elapsed -- returning to BeforeEntry (next tick fires the buy)");
                    // Next tick's `BeforeEntry` branch fires the buy --
                    // `ENTRY_NS` is already long past by now.
                    self.phase = Phase::BeforeEntry;
                }
            }
            Phase::WaitingForEntryFill { .. } | Phase::WaitingForExitFill { .. } | Phase::Done => {}
        }
    }
}

impl Strategy for NaturalGasBracket {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        // No `ctx.now()` at this point -- `on_start` runs before any
        // event has been scheduled or dispatched (main.rs's own wiring,
        // before the replay loop begins), so there is genuinely no sim
        // timestamp yet to print. `logging::line`'s own `None` branch
        // renders this honestly ("pre-replay"), rather than faking one.
        tracing::info!("{}", logging::line("NaturalGasBracket", None, "START", "fired (pre-replay -- no sim timestamp exists yet)"));
        for name in UNDERLYINGS {
            match ctx.resolve(name) {
                Some(id) => {
                    self.instrument = Some(id);
                    ctx.subscribe(id, Depth::Bbo);
                    tracing::info!("{}", logging::line("NaturalGasBracket", None, "SUBSCRIBE", &format!("{name} -- native/MCX token id={}, depth=Bbo", id.0)));
                }
                None => tracing::info!("{}", logging::line("NaturalGasBracket", None, "SUBSCRIBE_FAILED", &format!("{name} did not resolve to a real front-month future today -- not subscribed"))),
            }
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, packet_transact_time_ns: u64) {
        self.tick(ctx, packet_transact_time_ns);
    }

    fn on_trade(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _trade: &Trade, _seq: u64, packet_transact_time_ns: u64) {
        self.tick(ctx, packet_transact_time_ns);
    }

    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {
        let now_ns = ctx.now();
        match self.phase {
            Phase::WaitingForEntryFill { client_order_id } if client_order_id == fill.client_order_id => {
                log(now_ns, "FILL", &format!("BUY @ Rs {:.2} (client_order_id={client_order_id}, qty={:.1} lot)", rupees(fill.price.0), fill.qty.0 as f64 / 10_000.0));
                self.phase = Phase::Open { entry_price_raw: fill.price.0 };
                self.log_portfolio(ctx, fill.instrument, now_ns);
            }
            Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod } if client_order_id == fill.client_order_id => {
                let pnl_rupees = rupees(fill.price.0) - rupees(entry_price_raw);
                log(
                    now_ns,
                    "FILL",
                    &format!(
                        "SELL @ Rs {:.2} (client_order_id={client_order_id}, reason={reason}) -- round trip: entry Rs {:.2} -> exit Rs {:.2}, {:+.2} Rs/lot before costs",
                        rupees(fill.price.0),
                        rupees(entry_price_raw),
                        rupees(fill.price.0),
                        pnl_rupees
                    ),
                );
                self.round_trips.push((entry_price_raw, fill.price.0, reason));
                self.log_portfolio(ctx, fill.instrument, now_ns);
                self.phase = if forced_eod { Phase::Done } else { Phase::CoolingDown { reenter_at_ns: fill.timestamp_ns + REENTRY_GAP_NS } };
            }
            _ => {}
        }
    }

    fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord) {
        log(ctx.now(), "ORDER_UPDATE", &format!("client_order_id={} state={:?} -- {}", update.client_order_id, update.resulting_state, update.description));
    }
}

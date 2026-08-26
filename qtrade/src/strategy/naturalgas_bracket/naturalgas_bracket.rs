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
//!   `ENTRY_NS` (10:00:00 IST, 2026-08-19). No timer exists anywhere in
//!   this codebase (`ctx.set_timer()` isn't built -- see
//!   `strategy/strategy.rs`'s own scope notes) -- this is a threshold
//!   check on `packet_transact_time_ns`, evaluated every `on_book`/
//!   `on_trade` tick, not a real wall-clock alarm. Accurate to the
//!   nearest tick of market activity, not to the second.
//! - **Bracket**: stop-loss at 98% of the fill price, take-profit at
//!   102% -- tightened from the original 95%/105% ask (too wide to ever
//!   realistically trigger on NATURALGAS's real intraday range).
//!   `simulator::OrderType` has no stop-order variant at all -- this
//!   strategy *is* the stop, watching `ctx.book`'s best bid on every
//!   tick and submitting a real market sell the instant it's crossed.
//!   Not a resting order sitting at the exchange; a reaction, one tick
//!   after the crossing print.
//! - **Order type, both sides**: `MarketToLimit` -- guarantees the
//!   entry happens at ~10:00 and the exit happens once triggered, at
//!   the cost of not controlling the exact fill price.
//! - **Re-entry**: 5 minutes after the exit fill, repeating for the
//!   rest of the session.
//! - **End of day**: force-closes any still-open position at `EOD_NS`
//!   (23:30:00 IST) -- real, checked-against-the-capture-file cutoff,
//!   not a guess (see `naturalgas_bracket.md`).
//! - **No `Denied`/`Rejected` handling.** If a submit is ever locally
//!   denied or venue-rejected, this strategy has no `on_order_update`
//!   logic to notice and retry -- it would simply wait forever for a
//!   fill that never comes. Accepted for this first version; a real
//!   strategy would need to.

use crate::book::Book;
use crate::decoder::Trade;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord};
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{InstrumentId, Lots, Side};

pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

/// 10:00:00 IST, 2026-08-19 -- verified for real against the capture
/// file: a real `PacketHeader.TransactTime` sample decodes to
/// `2026-08-19 09:00:00.185 IST`, confirming this field is a genuine
/// Unix-epoch nanosecond timestamp, not an arbitrary clock reading.
const ENTRY_NS: u64 = 1_787_113_800_000_000_000;
/// 23:30:00 IST, 2026-08-19 -- verified for real against the tail of
/// the same capture file: real activity runs to approximately this
/// time (see `naturalgas_bracket.md` for the scan).
const EOD_NS: u64 = 1_787_162_400_000_000_000;
const REENTRY_GAP_NS: u64 = 5 * 60 * 1_000_000_000;
const SL_NUM: i64 = 98;
const TP_NUM: i64 = 102;
const RUPEE_RAW: f64 = 100_000_000.0;

fn rupees(raw: i64) -> f64 {
    raw as f64 / RUPEE_RAW
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
                match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                    Ok(client_order_id) => {
                        println!("[naturalgas_bracket] EOD force-close: SELL submitted client_order_id={client_order_id} (entry was Rs {:.2})", rupees(entry_price_raw));
                        self.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason: "EOD", forced_eod: true };
                    }
                    Err(e) => println!("[naturalgas_bracket] EOD force-close submit failed: {e:?}"),
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
                    match ctx.submit(instrument, Side::Buy, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            println!("[naturalgas_bracket] t={now_ns} BUY submitted, client_order_id={client_order_id}");
                            self.phase = Phase::WaitingForEntryFill { client_order_id };
                        }
                        Err(e) => println!("[naturalgas_bracket] entry submit failed: {e:?}"),
                    }
                }
            }
            Phase::Open { entry_price_raw } => {
                let Some(book) = ctx.book(instrument) else { return };
                let Some(bid) = book.best_bid() else { return };
                let sl = entry_price_raw * SL_NUM / 100;
                let tp = entry_price_raw * TP_NUM / 100;
                let reason = if bid.price.0 <= sl {
                    Some("SL")
                } else if bid.price.0 >= tp {
                    Some("TP")
                } else {
                    None
                };
                if let Some(reason) = reason {
                    match ctx.submit(instrument, Side::Sell, OrderType::MarketToLimit, Lots(1)) {
                        Ok(client_order_id) => {
                            println!("[naturalgas_bracket] t={now_ns} {reason} triggered @ bid Rs {:.2} (entry Rs {:.2}) -- SELL submitted, client_order_id={client_order_id}", rupees(bid.price.0), rupees(entry_price_raw));
                            self.phase = Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod: false };
                        }
                        Err(e) => println!("[naturalgas_bracket] exit submit failed: {e:?}"),
                    }
                }
            }
            Phase::CoolingDown { reenter_at_ns } => {
                if now_ns >= reenter_at_ns {
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
        for name in UNDERLYINGS {
            if let Some(id) = ctx.resolve(name) {
                self.instrument = Some(id);
                ctx.subscribe(id, Depth::Bbo);
            }
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _seq: u64, packet_transact_time_ns: u64) {
        self.tick(ctx, packet_transact_time_ns);
    }

    fn on_trade(&mut self, ctx: &mut Ctx, _instrument: InstrumentId, _trade: &Trade, _seq: u64, packet_transact_time_ns: u64) {
        self.tick(ctx, packet_transact_time_ns);
    }

    fn on_fill(&mut self, _ctx: &mut Ctx, fill: &FillRecord) {
        match self.phase {
            Phase::WaitingForEntryFill { client_order_id } if client_order_id == fill.client_order_id => {
                println!("[naturalgas_bracket] BUY filled @ Rs {:.2} (client_order_id={client_order_id})", rupees(fill.price.0));
                self.phase = Phase::Open { entry_price_raw: fill.price.0 };
            }
            Phase::WaitingForExitFill { client_order_id, entry_price_raw, reason, forced_eod } if client_order_id == fill.client_order_id => {
                let pnl_rupees = rupees(fill.price.0) - rupees(entry_price_raw);
                println!(
                    "[naturalgas_bracket] SELL filled @ Rs {:.2} (client_order_id={client_order_id}, reason={reason}) -- round trip: entry Rs {:.2} -> exit Rs {:.2}, {:+.2} Rs/lot before costs",
                    rupees(fill.price.0),
                    rupees(entry_price_raw),
                    rupees(fill.price.0),
                    pnl_rupees
                );
                self.round_trips.push((entry_price_raw, fill.price.0, reason));
                self.phase = if forced_eod { Phase::Done } else { Phase::CoolingDown { reenter_at_ns: fill.timestamp_ns + REENTRY_GAP_NS } };
            }
            _ => {}
        }
    }

    fn on_order_update(&mut self, _ctx: &mut Ctx, _update: &OrderEventRecord) {}
}

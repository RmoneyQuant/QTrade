//! Existence proof for the 2026-09-09 instrument-selection work: a
//! single strategy subscribing to a commodity Future, a commodity
//! Option, an index Future (`FUTIDX`), and an index Option (`OPTIDX`)
//! all in the same `on_start` -- using `ctx.instruments()` for anything
//! beyond "front-month Future by name" (no `&str` could ever have
//! carried a strike/right, and `FUTIDX`/`OPTIDX` share the exact same
//! `Future`/`Option` shape as their commodity counterparts, just with an
//! index for `underlying`). Not a trading strategy -- logs what it
//! found, including each instrument's real `settlement`/`pricing_model`
//! (read back from `InstrumentMaster`, proving those are genuinely
//! derived from the real BCP row, not defaulted), and stops.
//!
//! Every resolution here goes through `.one()` (or `ctx.resolve`, which
//! is `.one()` underneath) -- qtrade's rule, stated directly (2026-09-09):
//! a strategy's declared intent either names a real instrument or the
//! run panics loudly, right there, at the point the mistake was made.
//! Nothing in this file checks for a missing instrument, because there
//! is nothing to check -- a miss never returns quietly.

use qtrade::logging;
use qtrade::{Ctx, Depth, InstrumentId, InstrumentKind, Right, StartCtx, Strategy};
use std::collections::HashSet;

pub struct MixedInstrumentsDemo {
    seen: HashSet<InstrumentId>,
}

impl MixedInstrumentsDemo {
    pub fn new() -> Self {
        Self { seen: HashSet::new() }
    }
}

impl Strategy for MixedInstrumentsDemo {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        let fut = ctx.resolve("NATURALGAS");
        ctx.subscribe(fut, Depth::Bbo);
        tracing::info!("{}", logging::line("MixedInstrumentsDemo", None, "SUBSCRIBE", &format!("NATURALGAS front-month future -- native id={}", fut.0)));

        let call = ctx.instruments().underlying("NATURALGAS").kind_is_option().right(Right::Call).front_n_expiries(1).one();
        ctx.subscribe(call, Depth::Bbo);
        tracing::info!("{}", logging::line("MixedInstrumentsDemo", None, "SUBSCRIBE", &format!("NATURALGAS nearest-expiry Call -- native id={}", call.0)));

        let idx_fut = ctx.instruments().underlying("MCXBULLDEX").kind_is_future().front_n_expiries(1).one();
        ctx.subscribe(idx_fut, Depth::Bbo);
        tracing::info!("{}", logging::line("MixedInstrumentsDemo", None, "SUBSCRIBE", &format!("MCXBULLDEX (index) nearest-expiry Future -- native id={}", idx_fut.0)));

        let idx_put = ctx.instruments().underlying("MCXBULLDEX").kind_is_option().right(Right::Put).front_n_expiries(1).one();
        ctx.subscribe(idx_put, Depth::Bbo);
        tracing::info!("{}", logging::line("MixedInstrumentsDemo", None, "SUBSCRIBE", &format!("MCXBULLDEX (index) nearest-expiry Put -- native id={}", idx_put.0)));

        // The "loop over a set of strikes instead of writing each one by
        // hand" pattern: `.one()` per iteration means a strike that
        // doesn't exist today panics right there, naming exactly which
        // one, rather than silently subscribing to fewer strikes than
        // asked for.
        for strike_rupees in [200.0, 250.0, 300.0] {
            let strike = qtrade::Price((strike_rupees * 100_000_000.0) as i64);
            let id = ctx.instruments().underlying("NATURALGAS").kind_is_option().right(Right::Call).strike(strike).front_n_expiries(1).one();
            ctx.subscribe(id, Depth::Bbo);
            tracing::info!("{}", logging::line("MixedInstrumentsDemo", None, "SUBSCRIBE", &format!("NATURALGAS Call strike=Rs {strike_rupees:.2} -- native id={}", id.0)));
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
        if self.seen.insert(instrument) {
            if let Some(i) = ctx.refdata().get(instrument) {
                let detail = match &i.kind {
                    InstrumentKind::Future { underlying, settlement, .. } => format!("Future underlying={underlying} settlement={settlement:?}"),
                    InstrumentKind::Option { underlying, right, strike, settlement, pricing_model, .. } => {
                        format!("Option underlying={underlying} right={right:?} strike={strike:?} settlement={settlement:?} pricing_model={pricing_model:?}")
                    }
                    other => format!("{other:?}"),
                };
                tracing::info!("{}", logging::line("MixedInstrumentsDemo", Some(ctx.now()), "FIRST_BOOK", &format!("id={} {detail}", instrument.0)));
            }
        }
    }
}

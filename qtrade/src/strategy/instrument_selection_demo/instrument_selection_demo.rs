//! The whole new workflow, start to finish, in one small strategy:
//! **define** which instruments you want (by real attributes -- kind,
//! underlying, strike, right, expiry), **filter** the day's real catalog
//! down to exactly those (`ctx.instruments()...one()`, panicking loudly
//! if any of them isn't real today -- see that method's own doc comment),
//! **subscribe**, then place a couple of real trades, and stop. Nothing
//! more -- this is the reference shape for "how do I write a strategy
//! against qtrade now," not a trading strategy with an edge.

use qtrade::logging;
use qtrade::{Ctx, Depth, InstrumentId, Lots, OrderType, Price, Right, Side, StartCtx, Strategy};
use std::collections::HashSet;

pub struct InstrumentSelectionDemo {
    traded: HashSet<InstrumentId>,
    want: HashSet<InstrumentId>,
}

impl InstrumentSelectionDemo {
    pub fn new() -> Self {
        Self { traded: HashSet::new(), want: HashSet::new() }
    }
}

impl Strategy for InstrumentSelectionDemo {
    fn on_start(&mut self, ctx: &mut StartCtx) {
        // 1. DEFINE -- state the real attributes of what you want to
        //    trade today. A Future needs only underlying + "which
        //    expiry"; an Option also needs strike + right.
        let future = ctx.resolve("NATURALGAS"); // shorthand for underlying+kind_is_future+front_n_expiries(1)
        let call = ctx
            .instruments()
            .underlying("NATURALGAS")
            .kind_is_option()
            .right(Right::Call)
            .strike(Price(250 * 100_000_000)) // Rs 250.00 strike, qtrade's wire-raw scale
            .front_n_expiries(1)
            .one(); // 2. FILTER -- resolves against today's real refdata, panics loudly if it isn't real

        // 3. SUBSCRIBE -- only now does either instrument actually start
        //    flowing market data to this strategy.
        for id in [future, call] {
            ctx.subscribe(id, Depth::Bbo);
            self.want.insert(id);
            tracing::info!("{}", logging::line("InstrumentSelectionDemo", None, "SUBSCRIBE", &format!("native id={}", id.0)));
        }
    }

    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
        // 4. TRADE -- once, per instrument, the first time its book has
        //    a real two-sided market. That's it; no follow-up, no exit,
        //    no lifecycle management -- this strategy's only job is
        //    proving the instruments it defined are real and tradable.
        if !self.want.contains(&instrument) || self.traded.contains(&instrument) {
            return;
        }
        let Some(book) = ctx.book(instrument) else { return };
        let (Some(bid), Some(_ask)) = (book.best_bid(), book.best_ask()) else { return };

        match ctx.submit(instrument, Side::Buy, OrderType::LimitDay(bid.price), Lots(1)) {
            Ok(client_order_id) => {
                self.traded.insert(instrument);
                tracing::info!("{}", logging::line("InstrumentSelectionDemo", Some(ctx.now()), "TRADE", &format!("instrument={} client_order_id={client_order_id} BUY 1 lot @ Rs {:.2}", instrument.0, bid.price.0 as f64 / 100_000_000.0)));
            }
            Err(e) => tracing::info!("{}", logging::line("InstrumentSelectionDemo", Some(ctx.now()), "TRADE_REFUSED", &format!("instrument={} {e:?}", instrument.0))),
        }
    }
}

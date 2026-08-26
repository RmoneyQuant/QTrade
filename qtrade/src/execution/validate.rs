//! T07 (`execution`)'s own acceptance harness -- **not** part of the
//! component's public API, not wired into `main.rs` (that file is
//! explicitly off-limits this round, same reason every other component's
//! validation binary exists as a second `[[bin]]` target -- see
//! `execution_user_doc.md` for the fuller account, and
//! `book_user_doc.md`/`cache_user_doc.md`/`simulator_user_doc.md` for the
//! same pattern established by earlier components).
//!
//! Unlike `book-validate`/`cache-validate`/`simulator-validate`, this
//! harness does not stream a real capture file. Every scenario BACKTEST-
//! PHASE1.md §M7's acceptance bar names (the `PendingCancel -> Filled`
//! race, local `Denied` never reaching the venue, the cost model's
//! direction asymmetry, a Tier 1 report with a real run identity) is
//! about `execution`'s own gate/state-machine/accounting logic, not about
//! anything that depends on which real session is replayed -- so a
//! small, deterministic, hand-checkable synthetic scenario is *more*
//! convincing evidence than pointing the same logic at an arbitrary real
//! file, not less (same reasoning `simulator-validate`'s own hand-trace
//! mode already uses for its Layer 3 evidence). This binary exists so
//! these scenarios run under real `cargo run`/`cargo test` against
//! `execution`'s actual dependency graph, not just the standalone
//! `rustc --test` check used mid-build.
//!
//! Run: `cargo run --bin execution-validate`

#[allow(dead_code)]
#[path = "../types/types.rs"]
mod types;
#[path = "../decoder/decoder.rs"]
mod decoder;
#[path = "../simulator/simulator.rs"]
mod simulator;
#[path = "execution.rs"]
mod execution;

use decoder::{DecodedMessage, OrderAdd, Price as DPrice, Qty as DQty, Side as DSide, Trade};
use execution::{AlwaysAllowRms, CostConfig, CostModel, ExecutionEngine, GateOutcome, LocalOtrConfig, NewOrderIntent, OrderState, OtrConfigSummary, RunConfig};
use simulator::{OrderType, OtrConfig, RejectReason};
use std::process::ExitCode;
use std::time::Duration;
use types::{Currency, Date, Instrument, InstrumentId, InstrumentKind, Lots, Price, Settlement, Side, Venue, YearMonth};

const IID: InstrumentId = InstrumentId(1);

fn future_instrument(id: u32, tick_size: i64, freeze_qty: i64) -> Instrument {
    Instrument {
        id: InstrumentId(id),
        venue: Venue::Mcx,
        native_id: id as i64,
        kind: InstrumentKind::Future { underlying: "CRUDEOIL".to_string(), expiry: Date(0), contract_month: YearMonth { year: 2026, month: 1 }, settlement: Settlement::Cash },
        tick_size: Price(tick_size),
        lot_size: 1,
        multiplier: 1,
        freeze_qty,
        price_band: None,
        currency: Currency::Inr,
    }
}

fn engine(session_id: u32, instrument: Instrument) -> ExecutionEngine {
    let run_config = RunConfig {
        session_id,
        cost_config: CostConfig::default(),
        local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
        venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
        markout_horizons_ns: vec![1_000_000, 5_000_000],
    };
    let venue_otr = OtrConfig { window: Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
    ExecutionEngine::new(run_config, vec![instrument], Box::new(AlwaysAllowRms), CostConfig::default(), venue_otr, vec![1_000_000, 5_000_000], true)
}

fn add(side: DSide, price: i64, qty: i64, prio: u64) -> DecodedMessage {
    DecodedMessage::OrderAdd(OrderAdd { seq: 0, security_id: 1, side, price: DPrice(price), qty: DQty(qty), priority_ts: prio, event_time: 0 })
}

fn trade(side: DSide, price: i64, qty: i64, full: bool, event_time: u64) -> DecodedMessage {
    DecodedMessage::Trade(Trade { seq: 0, full, security_id: 1, aggressor_side: side, price: DPrice(price), qty: DQty(qty), event_time })
}

fn main() -> ExitCode {
    let mut failures = 0u32;

    println!("=== T07 execution -- acceptance scenario 1: PendingCancel -> Filled race ===");
    {
        let mut eng = engine(7, future_instrument(1, 1, 1000));
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!("expected Submitted") };
        println!("  order {client_order_id} submitted, state={:?}", eng.order(client_order_id).unwrap().state);

        assert!(eng.request_cancel(client_order_id, 100).0);
        println!("  cancel requested (in flight, not yet delivered), state={:?}", eng.order(client_order_id).unwrap().state);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::PendingCancel);

        // Real trade quantity is in wire-raw units (simulator's native
        // scale) -- `intent.qty.to_raw_qty()` (10 lots) so this real
        // trade genuinely fully consumes the resting order.
        eng.on_market_event(&trade(DSide::Sell, 150, intent.qty.to_raw_qty().0, true, 999_999), 150);
        let state_after_fill = eng.order(client_order_id).unwrap().state;
        println!("  real trade arrives before our cancel does -- state={state_after_fill:?}, filled_qty={}", eng.order(client_order_id).unwrap().filled_qty.0);
        if state_after_fill != OrderState::Filled {
            eprintln!("  FAIL: expected Filled, the fill did not win the race");
            failures += 1;
        }

        eng.deliver_cancel_to_venue(client_order_id, 200);
        let final_state = eng.order(client_order_id).unwrap().state;
        println!("  moot cancel now delivered late -- state stays {final_state:?}, fills recorded={}", eng.fills().len());
        if final_state != OrderState::Filled || eng.fills().len() != 1 {
            eprintln!("  FAIL: late cancel regressed or double-counted the fill");
            failures += 1;
        } else {
            println!("  PASS: fill won the race, not silently dropped, not double counted");
        }
    }

    println!("\n=== T07 execution -- acceptance scenario 2: Denied never reaches the venue ===");
    {
        let mut eng = engine(7, future_instrument(1, 10, 100));
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(105)), qty: Lots(5) };
        let (outcome, _) = eng.submit_order(intent, 0);
        println!("  tick-size violation (price=105, tick_size=10): outcome={outcome:?}, venue_submit_calls={}", eng.venue_submit_calls());
        let ok_tick = matches!(outcome, GateOutcome::Denied { .. }) && eng.venue_submit_calls() == 0;

        let mut eng2 = engine(7, future_instrument(1, 10, 50));
        let intent2 = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(51) };
        let (outcome2, _) = eng2.submit_order(intent2, 0);
        println!("  freeze-qty violation (qty=51, freeze_qty=50): outcome={outcome2:?}, venue_submit_calls={}", eng2.venue_submit_calls());
        let ok_freeze = matches!(outcome2, GateOutcome::Denied { .. }) && eng2.venue_submit_calls() == 0;

        // Contrast: a BOC that would cross passes every local gate and
        // genuinely reaches the venue, which itself refuses it --
        // Rejected, a different terminal state from Denied.
        let mut eng3 = engine(7, future_instrument(1, 1, 1000));
        eng3.on_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let intent3 = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::BookOrCancel(Price(150)), qty: Lots(5) };
        let (outcome3, _) = eng3.submit_order(intent3, 0);
        let GateOutcome::Submitted { client_order_id: c3 } = outcome3 else { panic!("BOC should reach the venue") };
        let order3 = eng3.order(c3).unwrap();
        println!(
            "  BOC that would cross: reaches the venue (venue_submit_calls={}), venue refuses -- state={:?}, reject_reason={:?}, deny_reason={:?}",
            eng3.venue_submit_calls(),
            order3.state,
            order3.reject_reason,
            order3.deny_reason
        );
        let ok_reject = eng3.venue_submit_calls() == 1
            && order3.state == OrderState::Rejected
            && matches!(order3.reject_reason, Some(RejectReason::WouldCross))
            && order3.deny_reason.is_none();

        if ok_tick && ok_freeze && ok_reject {
            println!("  PASS: local Denied never reaches the venue; venue Rejected is a genuinely different, later, path");
        } else {
            eprintln!("  FAIL: local/venue rejection paths not distinct as required");
            failures += 1;
        }
    }

    println!("\n=== T07 execution -- acceptance scenario 3: cost model direction asymmetry ===");
    {
        let instrument = future_instrument(1, 1, 1000);
        let model = CostModel::new(CostConfig::default());
        let price = Price(500_000_00_00); // Rs 500.00
        let qty = Lots(10);
        let buy = model.round_trip(&instrument, qty, price, Side::Buy);
        let sell = model.round_trip(&instrument, qty, price, Side::Sell);
        println!("  buy  round-trip: total=Rs {:.4}  stamp_duty=Rs {:.4}  ctt=Rs {:.4}", buy.total_rupees, buy.stamp_duty, buy.ctt);
        println!("  sell round-trip: total=Rs {:.4}  stamp_duty=Rs {:.4}  ctt=Rs {:.4}", sell.total_rupees, sell.stamp_duty, sell.ctt);
        let ok = buy.stamp_duty > 0.0 && buy.ctt == 0.0 && sell.ctt > 0.0 && sell.stamp_duty == 0.0 && sell.total_rupees > buy.total_rupees;
        if ok {
            println!("  PASS: buy pays stamp duty / zero CTT, sell pays CTT / zero stamp duty, totals concretely differ");
        } else {
            eprintln!("  FAIL: cost model is not genuinely direction-asymmetric");
            failures += 1;
        }

        // Same function, pre-trade and post-fill -- can't disagree.
        let mut eng = engine(7, instrument.clone());
        let pre_trade_cost = eng.cost_model().round_trip(&instrument, qty, price, Side::Sell);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(price), qty };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        // Real trade quantity in wire-raw units, matching `qty` (Lots)
        // converted via `to_raw_qty()` so this trade genuinely fully
        // fills the resting order -- not the pre-fix bug where a lot
        // count was handed to the venue as if it were already raw.
        eng.on_market_event(&trade(DSide::Sell, price.0, qty.to_raw_qty().0, true, 999_999), 10);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Filled);
        let realised = &eng.fills()[0];
        println!("  pre-trade query: Rs {:.4}   realised fill's cost: Rs {:.4}", pre_trade_cost.total_rupees, realised.cost.total_rupees);
        if (pre_trade_cost.total_rupees - realised.cost.total_rupees).abs() > 1e-9 {
            eprintln!("  FAIL: pre-trade cost query and realised fill cost disagree");
            failures += 1;
        } else {
            println!("  PASS: pre-trade query and realised fill use the identical CostModel::round_trip call");
        }
    }

    println!("\n=== T07 execution -- acceptance scenario 4: Tier 1 report with run identity ===");
    {
        let eng = engine(7, future_instrument(1, 1, 1000));
        let report = eng.tier1_report();
        let printed = format!("{report}");
        println!("{printed}");
        let ok = printed.contains("run identity") && printed.contains(execution::BUILD_HASH) && printed.contains(&format!("{:#018x}", report.run_identity.config_hash));
        if ok {
            println!("  PASS: Tier 1 report embeds and prints (config_hash, build_hash)");
        } else {
            eprintln!("  FAIL: Tier 1 report missing run identity in its printed form");
            failures += 1;
        }
    }

    println!("\n=== T07 execution -- acceptance scenario 5: queue position at fill is genuine, not fabricated ===");
    {
        let mut eng = engine(7, future_instrument(1, 1, 1000));
        eng.on_market_event(&add(DSide::Sell, 150, 10, 1), 0);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(5) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        // First real trade consumes the 10 (raw) resting ahead of us --
        // does not fill us yet.
        eng.on_market_event(&trade(DSide::Sell, 150, 10, true, 1), 5);
        // Second, separate real trade actually fills us -- wire-raw qty
        // equal to our own order's requested raw quantity (5 lots).
        eng.on_market_event(&trade(DSide::Sell, 150, intent.qty.to_raw_qty().0, true, 999_999), 6);

        let fill = &eng.fills()[0];
        println!("  queue_position_at_fill = {:?} (expected Some(10) -- the position established when the order joined the queue, not the 0 left by the time our own fill's event ran)", fill.queue_position_at_fill);
        if fill.queue_position_at_fill == Some(10) {
            println!("  PASS: genuine pre-fill queue position captured");
        } else {
            eprintln!("  FAIL: queue position at fill is fabricated/stale");
            failures += 1;
        }
    }

    println!("\n=== summary: {} scenario(s) failed ===", failures);
    if failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

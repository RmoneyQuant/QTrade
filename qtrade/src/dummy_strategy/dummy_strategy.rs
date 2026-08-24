//! A minimal, deliberately dumb strategy. Two things, both real, both
//! against real MCX data:
//!
//! 1. **Read side**: subscribes to CRUDEOIL and NATURALGAS at BBO via
//!    `cache` and prints every real best-bid/best-ask change -- proves
//!    `decoder -> cache` (filter -> `book` -> dispatch) works end to end.
//! 2. **Trade side**: every so often, fires one aggressive 1-lot IOC
//!    order (alternating buy/sell) through `execution::ExecutionEngine`
//!    against its own independent `simulator::SimExchange` venue (D10 --
//!    the same real market data feeds both `cache`'s view and the
//!    venue's, but they never talk to each other), just to generate real
//!    orders/fills/positions worth reporting.
//!
//! No real trading logic, no quoting, no risk awareness -- "dummy" is
//! not modesty, it's the actual design. Its only purpose is to prove the
//! whole engine (read side + trade side) works end to end against real
//! data, and to produce real report files before any actual `Strategy`
//! trait exists. See `dummy_strategy.md` in this folder for what it is,
//! how to run it, and where the reports land.
//!
//! Not wired into `main.rs` (reserved for the decoder CLI); its own
//! `[[bin]]` target (`dummy-strategy`) in Cargo.toml, same convention
//! every other component's own validation binary already uses.

#[path = "../types/types.rs"]
mod types;
#[path = "../decoder/decoder.rs"]
mod decoder;
#[path = "../refdata/refdata.rs"]
mod refdata;
#[path = "../book/book.rs"]
mod book;
#[path = "../cache/cache.rs"]
mod cache;
#[path = "../simulator/simulator.rs"]
mod simulator;
#[path = "../execution/execution.rs"]
mod execution;

use std::cell::RefCell;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;

use cache::{Cache, Depth, InstrumentFilter, Subscriber};
use execution::{CostConfig, ExecutionEngine, LocalOtrConfig, NewOrderIntent, OtrConfigSummary, RunConfig};
use types::{InstrumentId, Lots, Side};

/// Where every report file this run produces actually lands. Answers
/// "where is the log" literally.
const LOG_DIR: &str = "logs/dummy_strategy";

/// The read-side subscriber. Holds no state beyond a shared handle back
/// to `main`'s loop -- `Subscriber::on_wake` deliberately doesn't get a
/// `&Cache` (a real strategy shouldn't be able to reach back into
/// `Cache` and mutate it mid-dispatch), so this just records *that* a
/// wake happened; `main` does the actual reading and, per the trade-side
/// logic below, the actual deciding.
struct DummyStrategy {
    woke: Rc<RefCell<Vec<InstrumentId>>>,
}

impl Subscriber for DummyStrategy {
    fn on_wake(&mut self, instrument: InstrumentId, _depth: Depth) {
        self.woke.borrow_mut().push(instrument);
    }
}

/// Raw wire units -> human units, same conversion `decoder`'s own
/// `Price`/`Qty` `Display` impls use (see decoder.rs's
/// `MCX_PRICE_MULTIPLIER`/`MCX_QTY_DIVISOR`) -- duplicated here rather
/// than imported since `decoder`'s `Price`/`Qty` are private to that
/// module (they model the wire, not the shared vocabulary).
const RUPEE_RAW: f64 = 100_000_000.0;
const LOT_RAW: f64 = 10_000.0;

fn fmt_level(lvl: Option<types::PriceLevel>) -> String {
    match lvl {
        Some(l) => format!("Rs {:.2} x {:.1}", l.price.0 as f64 / RUPEE_RAW, l.qty.0 as f64 / LOT_RAW),
        None => "--".to_string(),
    }
}

fn print_bbo(cache: &Cache, instrument: InstrumentId, label: &str, seq: u64) {
    let Some(book) = cache.book(instrument) else { return };
    let bid = book.best_bid();
    let ask = book.best_ask();
    let spread = match (bid, ask) {
        (Some(b), Some(a)) => format!("Rs {:.2}", (a.price.0 - b.price.0) as f64 / RUPEE_RAW),
        _ => "--".to_string(),
    };
    println!("[{seq:>6}] {label:<10} bid={:<18} ask={:<18} spread={spread}", fmt_level(bid), fmt_level(ask));
}

/// Per-instrument trade-demo state: how many times we've woken for it,
/// and which side to fire next (alternating, so a buy is always followed
/// by a sell -- flattening the position back down rather than just
/// accumulating one direction, so the final report has something to say
/// about both a fill *and* a realized P&L).
#[derive(Default)]
struct TradeState {
    wakes_seen: u32,
    next_side: Option<Side>,
    orders_sent: u32,
}

/// Every `WAKE_PERIOD`-th wake, fire one order, up to `MAX_ORDERS_PER_INSTRUMENT`.
/// 50 was picked empirically against the default 20MB slice (a few
/// hundred real BBO-changing wakes for CRUDEOIL in that window) so the
/// demo reliably produces several orders without needing a much larger
/// (slower) slice -- not a realistic trading frequency, just a knob to
/// make sure this demo actually demonstrates something.
const WAKE_PERIOD: u32 = 50;
const MAX_ORDERS_PER_INSTRUMENT: u32 = 6;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let capture_path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin");
    let max_bytes: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20_000_000);
    let max_bbo_prints: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);

    let refdata_path = Path::new("/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp");
    let master = match refdata::InstrumentMaster::load_mcx(refdata_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("failed to load refdata: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Deliberately narrowed to exactly the two native tokens `book`'s
    // `band_config` has real validated price bands for (`CRUDEOIL_ID`/
    // `NATURALGAS_ID`) -- a broader "any NATURALGAS-family contract"
    // predicate admits sibling expiries/NATGASMINI that hit `book`'s
    // generic stub band and panic on real data (see dummy_strategy.md
    // for the exact panic this hit during development). Same real
    // limitation `cache_user_doc.md` §2.1 already found, same fix.
    let filter = InstrumentFilter::from_native_ids([book::CRUDEOIL_ID.0 as i64, book::NATURALGAS_ID.0 as i64]);

    // The two real Instrument records `execution::ExecutionEngine` needs
    // for tick-size/freeze-qty validation and cost-model lookups.
    //
    // **Tick-size fix, verified against real data:** `refdata`'s
    // `TickSize` column (`parts[21]`) is denominated in *paise*
    // (confirmed empirically -- see `refdata_user_doc.md`'s "TickSize
    // units" section), not qtrade's internal wire-raw `Price` scale.
    // `refdata::load_mcx_instruments` now converts via
    // `types::ContractFilePaise::to_wire_price()` before populating
    // `Instrument.tick_size`, so the value it hands back for CRUDEOIL
    // (467013) is exactly `Price(100_000_000)` = Rs 1.00 and for
    // NATURALGAS (465849) exactly `Price(10_000_000)` = Rs 0.10 --
    // matching `book.rs`'s own independently-validated `band_config`
    // values exactly. No override needed here any more; the demo-only
    // workaround this comment used to describe is gone because the real
    // bug it was working around is fixed.
    //
    // `freeze_qty` is still overridden below -- a separate, still-open
    // gap: `refdata` has no source column for freeze quantity (T01's own
    // documented scope note) and defaults every instrument's
    // `freeze_qty` to `0`, which would deny every order regardless of
    // the (now-fixed) tick-size units question above. This override is
    // demo-only headroom over this demo's 1-lot orders, in `Lots` (the
    // same unit `execution::validate()` now compares `freeze_qty`
    // against), not a claim about MCX's real freeze quantity -- see
    // `dummy_strategy.md` for the full account.
    //
    // No `.id` remap needed any more: `refdata::load_mcx_instruments`
    // now assigns `Instrument.id` as the native token directly, the same
    // convention `book`/`cache`/`simulator`/`execution` already used --
    // there is exactly one `InstrumentId` space in qtrade now, not two.
    // (There used to be a `.map(|mut i| { i.id = InstrumentId(i.native_id
    // as u32); ... })` line right here, purely to paper over the gap --
    // see `refdata_user_doc.md`'s "on `InstrumentId` unification" section
    // for the real fix that made it unnecessary.)
    const DEMO_FREEZE_QTY_LOTS: i64 = 1_000; // generous headroom over this demo's 1-lot orders
    let trade_instruments: Vec<types::Instrument> = master
        .all()
        .iter()
        .filter(|i| i.native_id == 467_013 || i.native_id == 465_849)
        .cloned()
        .map(|mut i| {
            i.freeze_qty = DEMO_FREEZE_QTY_LOTS;
            i
        })
        .collect();

    println!(
        "refdata: {} instruments loaded, filter admits {} native ids (narrowed to book's validated bands), \
         {} of them resolved for order entry",
        master.all().len(),
        filter.len(),
        trade_instruments.len()
    );

    let mut cache = Cache::new(master, filter);

    // Same real, increment-only-feed gap `cache-validate` hits (see its
    // own comment at its `Cache::new` call site, and book_user_doc.md's
    // "generic price band" section): this demo only ever reads one
    // `Increment_capture` file, which for CRUDEOIL never carries a valid
    // `InstrumentInfo` (13603) during the session at all. Seeded here
    // with the same real, snapshot-verified band `book-validate`'s own
    // harness learned from the paired snapshot file's 13603 stream --
    // not a guess, and not the old hardcoded `band_config` (removed).
    cache.seed_book_band(book::CRUDEOIL_ID, 523_200_000_000, 566_600_000_000); // Rs 5,232.00 - Rs 5,666.00
    cache.seed_book_band(book::NATURALGAS_ID, 22_160_000_000, 33_920_000_000); // Rs 221.60 - Rs 339.20 (full-session union)

    let run_config = RunConfig {
        session_id: 1,
        cost_config: CostConfig::default(),
        local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
        venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
        markout_horizons_ns: vec![1_000_000, 5_000_000],
    };
    let venue_otr = simulator::OtrConfig {
        window: std::time::Duration::from_secs(1),
        max_messages_per_window: 10_000,
        max_otr_ratio: 1_000_000.0,
    };
    let mut engine = ExecutionEngine::new(
        run_config,
        trade_instruments,
        Box::new(execution::AlwaysAllowRms),
        CostConfig::default(),
        venue_otr,
        vec![1_000_000, 5_000_000],
        true,
    );

    let woke = Rc::new(RefCell::new(Vec::new()));
    cache.subscribe(book::CRUDEOIL_ID, Depth::Bbo, Box::new(DummyStrategy { woke: woke.clone() }));
    cache.subscribe(book::NATURALGAS_ID, Depth::Bbo, Box::new(DummyStrategy { woke: woke.clone() }));

    let mut file = match File::open(capture_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open {capture_path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut data = vec![0u8; max_bytes];
    let read = match file.read(&mut data) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed to read {capture_path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    data.truncate(read);
    println!(
        "streaming the first {read} bytes of {capture_path} (a bounded prefix, not a full session -- \
         this is a demo, not a correctness gate; `book`/`cache`/`simulator` already proved full-session \
         correctness elsewhere)\n"
    );

    // `now_ns` is a synthetic, strictly-increasing counter, not a real
    // captured timestamp -- decode_messages's per-message stream doesn't
    // carry the outer packet's real TransactTime at this level (see
    // dummy_strategy.md). Every consumer here (the OTR governors' window
    // pruning, markout-horizon bookkeeping) only needs *a* monotonic
    // clock, not the *real* one -- unlike `scheduler`'s `SimClock`, which
    // this demo does not use at all (no event loop, no scheduling, just
    // a straight-line replay).
    let mut now_ns: u64 = 0;
    let mut events = 0u64;
    let mut bbo_printed = 0usize;
    let mut trade_state: std::collections::HashMap<InstrumentId, TradeState> = std::collections::HashMap::new();

    for event in decoder::decode_messages(&data) {
        now_ns += 1_000; // 1 microsecond per message, arbitrary but monotonic
        cache.on_message(&event);
        engine.on_market_event(&event, now_ns);
        events += 1;

        for instrument in woke.borrow_mut().drain(..) {
            let label = if instrument == book::CRUDEOIL_ID { "CRUDEOIL" } else { "NATURALGAS" };
            if bbo_printed < max_bbo_prints {
                print_bbo(&cache, instrument, label, events);
                bbo_printed += 1;
            }

            let state = trade_state.entry(instrument).or_default();
            state.wakes_seen += 1;
            if state.orders_sent >= MAX_ORDERS_PER_INSTRUMENT || state.wakes_seen % WAKE_PERIOD != 0 {
                continue;
            }
            let side = *state.next_side.get_or_insert(Side::Buy);
            let Some(book_ref) = cache.book(instrument) else { continue };
            // Aggressive IOC crossing the current touch, guaranteeing a
            // real fill against real resting liquidity if any exists at
            // that moment -- deliberately simple, this is a demo, not a
            // quoting strategy.
            let cross_price = match side {
                Side::Buy => book_ref.best_ask().map(|l| l.price),
                Side::Sell => book_ref.best_bid().map(|l| l.price),
            };
            let Some(price) = cross_price else { continue };
            let intent = NewOrderIntent {
                strategy_id: 1,
                instrument,
                side,
                order_type: simulator::OrderType::Ioc(price),
                qty: Lots(1),
            };
            let outcome = engine.submit_order(intent, now_ns);
            println!("  >> {label} order #{}: {side:?} 1.0 lot @ Rs {:.2} IOC -> {outcome:?}", state.orders_sent + 1, price.0 as f64 / RUPEE_RAW);
            state.orders_sent += 1;
            state.next_side = Some(if side == Side::Buy { Side::Sell } else { Side::Buy });
        }
    }

    println!("\n--- summary ---");
    println!("events processed: {events}");
    println!("BBO lines printed: {bbo_printed} (capped at {max_bbo_prints})");
    for (id, label) in [(book::CRUDEOIL_ID, "CRUDEOIL"), (book::NATURALGAS_ID, "NATURALGAS")] {
        if let Some(b) = cache.book(id) {
            println!("final {label} (as seen by cache): bid={} ask={} state={:?}", fmt_level(b.best_bid()), fmt_level(b.best_ask()), b.state());
        }
    }

    // --- Write the actual report files. This is the answer to "where is
    // the log": three real files under LOG_DIR, plus everything already
    // printed above to stdout.
    if let Err(e) = fs::create_dir_all(LOG_DIR) {
        eprintln!("failed to create {LOG_DIR}: {e}");
        return ExitCode::FAILURE;
    }

    let orders_path = format!("{LOG_DIR}/orders.log");
    let mut orders_file = File::create(&orders_path).expect("create orders.log");
    writeln!(orders_file, "# order report -- every order-state transition this run produced").unwrap();
    for ev in engine.order_events() {
        writeln!(orders_file, "t={:>10} client_order_id={:<20} state={:<14} {}", ev.timestamp_ns, ev.client_order_id, format!("{:?}", ev.resulting_state), ev.description).unwrap();
    }

    let fills_path = format!("{LOG_DIR}/fills.log");
    let mut fills_file = File::create(&fills_path).expect("create fills.log");
    writeln!(fills_file, "# fills / trade report -- every real fill this run produced").unwrap();
    for f in engine.fills() {
        writeln!(
            fills_file,
            "fill_id={:<6} client_order_id={:<20} instrument={:<12} side={:<5} price=Rs {:<10.2} qty={:<6.1} kind={:<10} queue_pos_at_fill={:<8} cost=Rs {:.4}",
            f.fill_id,
            f.client_order_id,
            format!("{:?}", f.instrument),
            format!("{:?}", f.side),
            f.price.0 as f64 / RUPEE_RAW,
            f.qty.0 as f64 / LOT_RAW,
            format!("{:?}", f.kind),
            f.queue_position_at_fill.map(|q| q.to_string()).unwrap_or_else(|| "--".to_string()),
            f.cost.total_rupees
        )
        .unwrap();
    }

    let report_path = format!("{LOG_DIR}/report.txt");
    let tier1 = engine.tier1_report();
    fs::write(&report_path, format!("{tier1}")).expect("write report.txt");

    println!("\n--- report (Tier 1) ---\n{tier1}");
    println!("logs written:");
    println!("  {orders_path}  ({} order events)", engine.order_events().len());
    println!("  {fills_path}  ({} fills)", engine.fills().len());
    println!("  {report_path}");

    ExitCode::SUCCESS
}

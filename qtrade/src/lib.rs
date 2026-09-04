//! qtrade -- the strategy-facing library.
//!
//! This is the same engine `main.rs` (the `qtrade` demo binary) drives --
//! moved here, unchanged, so an external crate can depend on `qtrade`,
//! `impl Strategy` for its own type, and call `run_backtest` instead of
//! forking this source tree. See `main.rs` for the thin CLI wrapper this
//! library is built for, and `strategy/strategy.rs`'s own header /
//! `STRATEGY-GUIDE.md` for how to write a `Strategy`.
//!
//! ## What's public
//!
//! Deliberately narrow, not "everything `pub`": only the types that
//! actually appear in `Strategy`'s own methods or `Ctx`/`StartCtx`'s
//! public methods are re-exported below. Everything else --
//! `SimExchange`, `Cache`, `ExecutionEngine`, `BookBuilder`, the whole
//! matching/ledger machinery -- stays internal. A strategy author gets
//! exactly what `Ctx`'s own methods hand them, nothing more (same D25
//! "subscription governs waking, not access" narrowness, now applied to
//! the crate boundary itself).
//!
//! ## What moved here from `main.rs`, and what didn't
//!
//! Every module declaration, every helper function (`dispatch_event`,
//! `sync_venue_alarms`, `drain_cache_injections`, `civil_from_days`,
//! `run_timestamp_ist`, `fmt_level`), and the entire body of the old
//! `main()` moved here verbatim, into `run_backtest`. Two real, small
//! changes were needed to make it generic over `impl Strategy` rather
//! than the one hardcoded demo strategy -- both noted at `run_backtest`'s
//! own doc comment. Nothing about *how a backtest runs* changed: no
//! matching logic, no book logic, no scheduler logic, no timing logic
//! was touched.

#[path = "config/config.rs"]
mod config;
#[path = "types/types.rs"]
mod types;
#[path = "decoder/decoder.rs"]
mod decoder;
#[path = "refdata/refdata.rs"]
mod refdata;
// Real, wired caller as of the dual-clock replay pass (2026-08-27) --
// `run_backtest`'s own loop drives `Scheduler` directly (see
// `main_user_doc.md`). `#[allow(dead_code)]` stays: `SessionTransition`/
// `StalenessTimeout`/`WatchdogExpiry`/`OffloadCompletion`/`StrategyTimer`
// remain real, unbacked placeholders -- nothing schedules them yet.
#[allow(dead_code)]
#[path = "scheduler/scheduler.rs"]
mod scheduler;
#[path = "book/book.rs"]
mod book;
#[path = "cache/cache.rs"]
mod cache;
#[path = "simulator/simulator.rs"]
mod simulator;
#[path = "execution/execution.rs"]
mod execution;
// `pub`, unlike every other module here -- a small, self-contained
// structured-log-line formatter (`logging::line`) any strategy author
// can use to match `events.log`'s own formatting, plus what
// `run_backtest` itself uses internally (`LogLevel`, `init`).
#[path = "logging/logging.rs"]
pub mod logging;
#[path = "feed_replay/feed_replay.rs"]
mod feed_replay;
#[path = "event_dispatcher/event_dispatcher.rs"]
mod event_dispatcher;
#[path = "control_dispatcher/control_dispatcher.rs"]
mod control_dispatcher;
// Shared `Ctx`/`StartCtx` -- distinct from any one strategy's own crate
// (see strategy/README.md and strategy.rs's own header). No concrete
// strategy module is declared here -- that's the whole point: a
// strategy lives in the *consuming* crate now, not in this one.
#[path = "strategy/strategy.rs"]
mod strategy;

// =======================================================================
// Public surface -- see this file's own header for why it's this list
// and not `pub mod` everything.
// =======================================================================

pub use book::Book;
pub use decoder::Trade;
pub use event_dispatcher::Depth;
pub use execution::{CancelReason, Cost, DenyReason, FillRecord, Order, OrderEventRecord, OrderState, StrategyId};
pub use refdata::InstrumentMaster;
pub use simulator::{FillKind, OrderType, RejectReason};
pub use strategy::{Ctx, CtxError, Pnl, StartCtx, Strategy};
pub use types::{BookState, Currency, Date, Instrument, InstrumentId, InstrumentKind, Lots, OrderHandle, Price, PriceLevel, Qty, RAW_QTY_PER_LOT, Settlement, Side, Venue, YearMonth};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use cache::{Cache, InstrumentFilter};
use control_dispatcher::ControlDispatcher;
use event_dispatcher::EventDispatcher;
use execution::{CostConfig, ExecutionEngine, LocalOtrConfig, OtrConfigSummary, RunConfig};
use scheduler::{EventClass, EventPayload, Scheduler, Target};
use simulator::SimExchange;

/// Howard Hinnant's `civil_from_days` (public domain, proleptic
/// Gregorian) -- days-since-epoch -> (year, month, day). Moved verbatim
/// from `main.rs`; see that file's own former doc comment for why no
/// external date/time crate is used.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// The current wall-clock instant, IST, formatted `YYYYMMDD_HHMMSS` -- a
/// per-run output folder name, so two runs never overwrite each other's
/// logs. Moved verbatim from `main.rs`.
fn run_timestamp_ist() -> String {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let ist_secs = now.as_secs() as i64 + 5 * 3600 + 30 * 60;
    let days = ist_secs.div_euclid(86_400);
    let secs_of_day = ist_secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let (hh, mm, ss) = (secs_of_day / 3600, (secs_of_day / 60) % 60, secs_of_day % 60);
    format!("{y:04}{m:02}{d:02}_{hh:02}{mm:02}{ss:02}")
}

const RUPEE_RAW: f64 = 100_000_000.0;
const LOT_RAW: f64 = 10_000.0;

fn fmt_level(lvl: Option<types::PriceLevel>) -> String {
    match lvl {
        Some(l) => format!("Rs {:.2} x {:.1}", l.price.0 as f64 / RUPEE_RAW, l.qty.0 as f64 / LOT_RAW),
        None => "--".to_string(),
    }
}

/// Pattern-matches one popped `scheduler::Event` and calls whatever it
/// means. Moved verbatim from `main.rs` -- see that file's former doc
/// comment (still accurate) for the full account of the venue-poll-alarm
/// design this implements.
fn dispatch_event(event: scheduler::Event, cache: &mut Cache, engine: &mut ExecutionEngine, venue: &mut SimExchange, event_dispatcher: &mut EventDispatcher, control_dispatcher: &mut ControlDispatcher) {
    let now = event.timestamp as u64;
    match event.payload {
        EventPayload::MarketData { target: Target::SimExchange, message, .. } => {
            engine.prepare_for_market_event(venue);
            let reports = venue.apply_market_event(&message, now);
            let outcome = engine.apply_venue_reports(reports, now);
            control_dispatcher.dispatch(cache, engine, venue, &outcome);
        }
        EventPayload::MarketData { target: Target::Cache, message, seq_no, .. } => {
            if let Some(instrument) = cache.apply(&message) {
                if let Some(book) = cache.book(instrument) {
                    let local_outcome = event_dispatcher.on_book_touched(book, instrument, cache, engine, venue, seq_no, now);
                    control_dispatcher.dispatch(cache, engine, venue, &local_outcome);
                }
            }
            if let decoder::DecodedMessage::Trade(t) = &message {
                if cache.filter().passes(t.security_id) {
                    let local_outcome = event_dispatcher.on_trade(cache, engine, venue, InstrumentId(t.security_id as u32), t, seq_no, now);
                    control_dispatcher.dispatch(cache, engine, venue, &local_outcome);
                }
            }
        }
        EventPayload::OrderArrival { .. } | EventPayload::ReportDelivery { .. } => {
            let reports = venue.poll(now);
            if !reports.is_empty() {
                let outcome = engine.apply_venue_reports(reports, now);
                control_dispatcher.dispatch(cache, engine, venue, &outcome);
            }
        }
        _ => {}
    }
}

/// Keeps the Scheduler's own alarms in sync with whatever `venue` itself
/// says it next needs to be checked at. Moved verbatim from `main.rs`.
fn sync_venue_alarms(venue: &SimExchange, sched: &mut Scheduler, next_arrival_alarm: &mut Option<i64>, next_visibility_alarm: &mut Option<i64>) {
    if !venue.has_pending() && next_arrival_alarm.is_none() && next_visibility_alarm.is_none() {
        return;
    }
    let want_arrival = venue.next_arrival_due_ns();
    if want_arrival != *next_arrival_alarm {
        if let Some(t) = want_arrival {
            sched.schedule(t, EventClass::OrderArrival, EventPayload::OrderArrival { op_id: 0 });
        }
        *next_arrival_alarm = want_arrival;
    }
    let want_visibility = venue.next_visibility_due_ns();
    if want_visibility != *next_visibility_alarm {
        if let Some(t) = want_visibility {
            sched.schedule(t, EventClass::ReportDelivery, EventPayload::ReportDelivery { op_id: 0 });
        }
        *next_visibility_alarm = want_visibility;
    }
}

/// Own-order injection (2026-09-03). Moved verbatim from `main.rs`.
fn drain_cache_injections(engine: &mut ExecutionEngine, sched: &mut Scheduler) {
    for (now_ns, message) in engine.take_pending_cache_injections() {
        let now = now_ns as i64;
        sched.schedule(now, EventClass::MarketData, EventPayload::MarketData { target: Target::Cache, message, seq_no: 0, exchange_ts: now_ns, recorder_ts: now });
    }
}

/// Runs one full backtest: load `config_path`, replay its capture
/// file(s) against `strategy`, write `orders.log`/`fills.log`/
/// `report.txt` into a fresh timestamped folder under `[run].report_dir`,
/// and return the strategy handle so the caller can read whatever
/// strategy-specific state/instrumentation it wants afterward (this
/// engine only ever calls `Strategy`'s own trait methods on it -- it has
/// no idea `OrderLifecycleDemo::round_trips()` or any other concrete
/// strategy's own methods exist).
///
/// This is `main()`'s entire former body, unchanged in what it does --
/// two things had to change to make it generic over `impl Strategy`
/// instead of one hardcoded module, and both are new, not modified
/// behavior:
///
/// 1. **`underlyings` is now an explicit parameter**, not
///    `order_lifecycle_demo::UNDERLYINGS` (a module-level const specific
///    to that one strategy -- `Strategy` itself has no such const, and
///    adding one to the trait felt like the wrong place to put "which
///    instruments does this specific run track", since two different
///    callers might legitimately want to run the identical strategy
///    over different underlyings). The caller passes it directly, the
///    same names that used to be hardcoded.
/// 2. **Errors return as `Result<_, String>`** instead of `eprintln!` +
///    `ExitCode::FAILURE` -- a library can't call `std::process::exit`
///    on the caller's behalf. The caller (`main.rs`) does the
///    `eprintln!`/exit-code translation now, once, at the top level.
///    Message text is unchanged at every site.
///
/// **Inherited as-is, not fixed here**: a implausible feed-latency delta
/// (D20 fail-fast) still calls `std::process::exit(1)` directly from
/// inside the replay callback, because `feed_replay::replay`'s own
/// closure signature has no way to propagate an early error out through
/// it -- same hard-exit behavior the CLI binary always had, just also
/// inherited by anything that calls this function as a library now.
/// Worth a real fix later, not part of this move.
pub fn run_backtest<S: Strategy + 'static>(config_path: &Path, underlyings: &[&str], strategy: S) -> Result<Rc<RefCell<S>>, String> {
    let cfg = config::load(config_path).map_err(|e| e.to_string())?;

    if cfg.run.mode != "backtest" {
        return Err(format!("mode {:?} not implemented yet -- only \"backtest\" is supported (no live feed source exists in this codebase yet)", cfg.run.mode));
    }

    let capture_paths: &[String] = &cfg.run.recording_paths;
    let primary_path = capture_paths[0].as_str();
    let max_outer_records = cfg.run.max_outer_records;

    let master = feed_replay::load_refdata(primary_path).map_err(|e| e.to_string())?;

    let resolved: Vec<(&str, Option<InstrumentId>)> = underlyings.iter().map(|name| (*name, feed_replay::resolve_front_month(&master, name))).collect();
    let tracked_ids: Vec<InstrumentId> = resolved.iter().filter_map(|(_, id)| *id).collect();
    if tracked_ids.is_empty() {
        return Err(format!("none of {underlyings:?} resolved to a real front-month future in this day's refdata"));
    }
    let names_by_id: HashMap<InstrumentId, &str> = resolved.iter().filter_map(|(name, id)| id.map(|i| (i, *name))).collect();
    let label_of = |id: InstrumentId| -> &str { names_by_id.get(&id).copied().unwrap_or("UNKNOWN") };
    let name_to_id: HashMap<&str, InstrumentId> = resolved.iter().filter_map(|(name, id)| id.map(|i| (*name, i))).collect();

    let filter = InstrumentFilter::from_native_ids(tracked_ids.iter().map(|id| id.0 as i64));

    let trade_instruments: Vec<types::Instrument> = master.all().iter().filter(|i| tracked_ids.contains(&i.id)).cloned().collect();

    println!("refdata: {} instruments loaded, filter admits {} native ids, {} of them resolved for order entry", master.all().len(), filter.len(), trade_instruments.len());
    for id in &tracked_ids {
        println!("  resolved {}: native id {}", label_of(*id), id.0);
    }

    let mut cache = Cache::new(master, filter);

    let mut seeded: HashSet<InstrumentId> = HashSet::new();
    for path in capture_paths {
        let Some(snapshot_path) = feed_replay::snapshot_path_for(path) else {
            println!("{path} doesn't look like an Increment_capture file -- no paired snapshot file to auto-seed from");
            continue;
        };
        println!("scanning the paired snapshot file for real price bands: {snapshot_path}");
        match feed_replay::scan_snapshot_for_bands(&snapshot_path, &tracked_ids) {
            Ok(bands) => {
                for (id, (lower, upper, count)) in &bands {
                    println!("  {} ({}): real band [Rs {:.2}, Rs {:.2}], full-session union of {count} InstrumentInfo (13603) records", label_of(*id), id.0, *lower as f64 / RUPEE_RAW, *upper as f64 / RUPEE_RAW);
                    cache.seed_book_band(*id, *lower, *upper);
                    seeded.insert(*id);
                }
            }
            Err(e) => println!("  could not scan {snapshot_path}: {e} -- proceeding without it"),
        }
    }
    for id in &tracked_ids {
        if !seeded.contains(id) {
            println!("  {} ({}): no InstrumentInfo found in any scanned snapshot file -- not seeded, `book` will panic if a real order arrives before it learns one", label_of(*id), id.0);
        }
    }
    println!();

    let run_config = RunConfig {
        session_id: cfg.run.session_id,
        cost_config: CostConfig::default(),
        local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
        venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
        markout_horizons_ns: vec![1_000_000, 5_000_000],
    };
    let venue_otr = simulator::OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
    let mut sim_venue = SimExchange::new(&tracked_ids, venue_otr).with_order_latency(cfg.run.order_outbound_latency_ns, cfg.run.order_inbound_latency_ns);
    let mut engine = ExecutionEngine::new(run_config, trade_instruments, Box::new(execution::AlwaysAllowRms), CostConfig::default(), vec![1_000_000, 5_000_000], true);

    let run_dir = format!("{}/{}", cfg.run.report_dir, run_timestamp_ist());
    fs::create_dir_all(&run_dir).map_err(|e| format!("failed to create {run_dir}: {e}"))?;
    println!("run output folder: {run_dir}\n");

    let events_log_path = format!("{run_dir}/events.log");
    let log_level = logging::LogLevel::parse(&cfg.run.log_level);
    let _log_guards = logging::init(log_level, Path::new(&events_log_path)).map_err(|e| format!("failed to create {events_log_path}: {e}"))?;

    let strategy = Rc::new(RefCell::new(strategy));

    let mut event_dispatcher = EventDispatcher::new();
    let mut control_dispatcher = ControlDispatcher::new();
    let my_id = event_dispatcher.register(strategy.clone() as Rc<RefCell<dyn Strategy>>);
    control_dispatcher.register(strategy.clone() as Rc<RefCell<dyn Strategy>>);
    {
        let resolver = |name: &str| name_to_id.get(name).copied();
        let mut start_ctx = strategy::StartCtx::new(&resolver, &mut event_dispatcher, &mut control_dispatcher, my_id);
        strategy.borrow_mut().on_start(&mut start_ctx);
    }

    let limit_desc = if max_outer_records == 0 { "no limit -- full file, start to end".to_string() } else { format!("capped at {max_outer_records} outer records") };
    if capture_paths.len() == 1 {
        println!("streaming {primary_path} record-by-record ({limit_desc})\n");
    } else {
        println!("k-way merging {} streams on exchange_ts, record-by-record ({limit_desc}):", capture_paths.len());
        for (i, p) in capture_paths.iter().enumerate() {
            println!("  [{i}] {p}");
        }
        println!();
    }

    let mut sched = Scheduler::new();
    let max_feed_delta_ns = cfg.run.max_feed_delta_ns as i64;
    let mut next_arrival_alarm: Option<i64> = None;
    let mut next_visibility_alarm: Option<i64> = None;

    let stats = feed_replay::replay(capture_paths, max_outer_records, |ev| {
        let exchange_ts = ev.exchange_ts as i64;
        let recorder_ts = ev.recorder_ts;
        let delta = recorder_ts - exchange_ts;
        if delta < 0 || delta > max_feed_delta_ns {
            eprintln!("FATAL: implausible feed-latency delta at seq={}: recorder_ts={recorder_ts} exchange_ts={exchange_ts} delta={delta}ns (ceiling {max_feed_delta_ns}ns)", ev.seq_no);
            std::process::exit(1);
        }

        while let Some(peek_ts) = sched.peek_earliest_timestamp() {
            if peek_ts >= exchange_ts {
                break;
            }
            let event = sched.pop_earliest().expect("just peeked Some");
            dispatch_event(event, &mut cache, &mut engine, &mut sim_venue, &mut event_dispatcher, &mut control_dispatcher);
            sync_venue_alarms(&sim_venue, &mut sched, &mut next_arrival_alarm, &mut next_visibility_alarm);
            drain_cache_injections(&mut engine, &mut sched);
        }

        sched.schedule(exchange_ts, EventClass::MarketData, EventPayload::MarketData { target: Target::SimExchange, message: *ev.event, seq_no: ev.seq_no, exchange_ts: ev.exchange_ts, recorder_ts: ev.recorder_ts });
        sched.schedule(recorder_ts, EventClass::MarketData, EventPayload::MarketData { target: Target::Cache, message: *ev.event, seq_no: ev.seq_no, exchange_ts: ev.exchange_ts, recorder_ts: ev.recorder_ts });
    })
    .map_err(|e| format!("failed to open capture stream(s) {capture_paths:?}: {e}"))?;

    while let Some(event) = sched.pop_earliest() {
        dispatch_event(event, &mut cache, &mut engine, &mut sim_venue, &mut event_dispatcher, &mut control_dispatcher);
        sync_venue_alarms(&sim_venue, &mut sched, &mut next_arrival_alarm, &mut next_visibility_alarm);
        drain_cache_injections(&mut engine, &mut sched);
    }

    {
        let mut stop_ctx = strategy::Ctx::new(&cache, &mut engine, &mut sim_venue, 0, strategy::DEFAULT_STRATEGY_ID, false);
        strategy.borrow_mut().on_stop(&mut stop_ctx);
    }

    println!("\n--- summary ---");
    println!("outer records processed: {}", stats.outer_records);
    println!("events (decoded messages) processed: {}", stats.events);
    println!("elapsed: {:.2}s ({:.0} records/s, {:.0} messages/s)", stats.elapsed.as_secs_f64(), stats.outer_records as f64 / stats.elapsed.as_secs_f64(), stats.events as f64 / stats.elapsed.as_secs_f64());
    for id in &tracked_ids {
        if let Some(b) = cache.book(*id) {
            println!("final {} (as seen by cache): bid={} ask={} state={:?}", label_of(*id), fmt_level(b.best_bid()), fmt_level(b.best_ask()), b.state());
        }
    }

    let orders_path = format!("{run_dir}/orders.log");
    let mut orders_file = File::create(&orders_path).expect("create orders.log");
    writeln!(orders_file, "# order report -- every order-state transition this run produced").unwrap();
    for ev in engine.order_events() {
        writeln!(orders_file, "t={:>10} client_order_id={:<20} state={:<14} {}", ev.timestamp_ns, ev.client_order_id, format!("{:?}", ev.resulting_state), ev.description).unwrap();
    }

    let fills_path = format!("{run_dir}/fills.log");
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

    let report_path = format!("{run_dir}/report.txt");
    let tier1 = engine.tier1_report(&sim_venue);
    fs::write(&report_path, format!("{tier1}")).expect("write report.txt");

    println!("\n--- report (Tier 1) ---\n{tier1}");
    println!("logs written:");
    println!("  {events_log_path}  (component-level event trail, level={:?})", log_level);
    println!("  {orders_path}  ({} order events)", engine.order_events().len());
    println!("  {fills_path}  ({} fills)", engine.fills().len());
    println!("  {report_path}");

    Ok(strategy)
}

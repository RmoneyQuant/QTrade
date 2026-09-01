//! qtrade -- the one real entry point. `[[bin]] qtrade`.
//!
//! Owns everything that isn't strategy decision code or backtest-only
//! feed mechanics: reading the run's config file, mode selection,
//! constructing the shared engines (`Cache`, `ExecutionEngine`), and
//! writing the run's report files. See `main_user_doc.md` for the full
//! account, including why this file used to be decode-only and why
//! `backtester.rs` used to be a separate binary.
//!
//! Invocation: `qtrade <config-file>` -- one positional argument. Mode
//! (`backtest`, eventually `live`) is a *field inside that file*
//! (`[run] mode = "..."`), per D22/D39/BACKTEST-PHASE1.md §2.3's own
//! already-written spec, not a CLI subcommand or flag.
//!
//! **Does not itself decide what a strategy sees or does beyond the
//! wake.** `feed.csv` generation used to live here as "generic
//! instrumentation" -- discovered and corrected the same day it was
//! noticed: that made it unconditional for *any* strategy plugged in,
//! not a strategy's own choice, which is backwards. It now lives inside
//! `limit_order_book_generator.rs`, the one strategy that wants it; a
//! different strategy dropped in here simply wouldn't produce one.

#[path = "config/config.rs"]
mod config;
#[path = "types/types.rs"]
mod types;
#[path = "decoder/decoder.rs"]
mod decoder;
#[path = "refdata/refdata.rs"]
mod refdata;
// Real, wired caller as of the dual-clock replay pass (2026-08-27) --
// `main.rs`'s own loop now drives `Scheduler`/`SimClock` directly (see
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
#[path = "logging/logging.rs"]
mod logging;
#[path = "feed_replay/feed_replay.rs"]
mod feed_replay;
#[path = "event_dispatcher/event_dispatcher.rs"]
mod event_dispatcher;
#[path = "control_dispatcher/control_dispatcher.rs"]
mod control_dispatcher;
// Shared `Ctx`/`StartCtx` -- distinct from any one strategy's own
// subfolder (see strategy/README.md and strategy.rs's own header).
#[path = "strategy/strategy.rs"]
mod strategy;
// Only one strategy is compiled into `main.rs` at a time (see
// `strategy/README.md`) -- swapping strategies means pointing this
// declaration (and the `use`/construction lines below) at a different
// subfolder, not compiling more than one in. Currently:
// `multi_instrument_bracket` (2026-08-28) -- a real strategy trading two
// instruments at once, with real resting LIMIT orders, `ctx.modify()`,
// and `ctx.cancel()`, none of which `naturalgas_bracket` (still present,
// just not compiled in) ever exercised. `limit_order_book_generator/
// limit_order_book_generator.rs` is the pure-observer alternative.
#[path = "strategy/order_lifecycle_demo/order_lifecycle_demo.rs"]
mod order_lifecycle_demo;

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::rc::Rc;

use cache::{Cache, InstrumentFilter};
use control_dispatcher::ControlDispatcher;
use event_dispatcher::EventDispatcher;
use execution::{CostConfig, ExecutionEngine, LocalOtrConfig, OtrConfigSummary, RunConfig};
use order_lifecycle_demo::OrderLifecycleDemo;
use scheduler::{EventClass, EventPayload, Scheduler, Target};
use simulator::SimExchange;
use strategy::Strategy;
use types::InstrumentId;

/// Howard Hinnant's `civil_from_days` (public domain, proleptic
/// Gregorian) -- days-since-epoch -> (year, month, day). Same algorithm
/// `refdata.rs`'s own `year_month_from_days` already uses (that one
/// discards the day; this one keeps it, for a run-folder timestamp). No
/// external date/time crate, same "not worth it for one conversion"
/// call already made there -- same reasoning that kept `config.rs`
/// dependency-free too.
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

/// The current wall-clock instant, IST (matching every other timestamp
/// this project reports in, e.g. the real feed timestamps in
/// `feed.csv`), formatted `YYYYMMDD_HHMMSS` -- a per-run output folder
/// name, so two runs never overwrite each other's logs. Report-folder
/// naming is orchestrator config, not strategy or feed-replay concern.
fn run_timestamp_ist() -> String {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let ist_secs = now.as_secs() as i64 + 5 * 3600 + 30 * 60;
    let days = ist_secs.div_euclid(86_400);
    let secs_of_day = ist_secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    let (hh, mm, ss) = (secs_of_day / 3600, (secs_of_day / 60) % 60, secs_of_day % 60);
    format!("{y:04}{m:02}{d:02}_{hh:02}{mm:02}{ss:02}")
}

/// Raw wire units -> human units, same conversion `decoder`'s own
/// `Price`/`Qty` `Display` impls use. Kept as its own copy here (not
/// imported from the strategy) since it's only used by this file's own
/// summary printline, a different concern than whatever a strategy
/// prints.
const RUPEE_RAW: f64 = 100_000_000.0;
const LOT_RAW: f64 = 10_000.0;

fn fmt_level(lvl: Option<types::PriceLevel>) -> String {
    match lvl {
        Some(l) => format!("Rs {:.2} x {:.1}", l.price.0 as f64 / RUPEE_RAW, l.qty.0 as f64 / LOT_RAW),
        None => "--".to_string(),
    }
}

/// Pattern-matches one popped `scheduler::Event` and calls whatever it
/// means -- the dispatch pattern-matching D07 says belongs in startup
/// wiring, not inside the Scheduler itself (dual-clock replay,
/// 2026-08-27; see `scheduler.rs`'s own header for the one dependency it
/// *does* take on, `decoder::DecodedMessage`, and why that's as far as
/// it goes).
///
/// **The venue call is synchronous, but `OrderArrival`/`ReportDelivery`
/// are real again -- as venue-poll alarms, not strategy-scheduled
/// deliveries (2026-08-27, third pass).** `ExecutionEngine::deliver_order`/
/// `SimExchange::submit` are called directly from `strategy::Ctx::submit`/
/// `cancel`/`modify` the instant local gates pass -- see `strategy.rs`'s
/// own header. The real ~500μs MCX round trip lives entirely inside
/// `SimExchange` itself (`with_order_latency`, split into outbound/inbound
/// legs), invisible from here -- **except** that a real exchange answers
/// at its own processing time regardless of whether anything else happens
/// to occur then, and `SimExchange`'s own `drain_due` only runs when
/// *something* hands it a fresh `now_ns`. `main.rs` closes that gap: after
/// every dispatched event, `sync_venue_alarms` checks whether the venue
/// itself has something pending and keeps exactly one `OrderArrival`
/// alarm (outbound leg) and one `ReportDelivery` alarm (inbound leg) in
/// sync with that, reusing the two event classes' own original outbound/
/// inbound doc comments (`scheduler.rs`) for real, rather than the
/// strategy-scheduling role they briefly had. When either fires here, it
/// calls `venue.poll(now)` -- not a specific order's own delivery, just
/// "check what's due" -- and forwards anything that surfaces exactly like
/// a real market event would.
fn dispatch_event(event: scheduler::Event, cache: &mut Cache, engine: &mut ExecutionEngine, venue: &mut SimExchange, event_dispatcher: &mut EventDispatcher, control_dispatcher: &mut ControlDispatcher) {
    let now = event.timestamp as u64; // always non-negative in this domain -- validated at the point each timestamp entered the Scheduler
    match event.payload {
        EventPayload::MarketData { target: Target::SimExchange, message, .. } => {
            // No gate here, same as before this pass -- a real market
            // event always reaches the venue, unconditionally (it never
            // had a live counterpart to gate against in the first place;
            // see `ExecutionEngine::prepare_for_market_event`'s own doc).
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
            // A venue-poll alarm firing -- not a specific order's own
            // delivery (there is no `op_id` to look up any more; `venue`
            // itself already knows what's due). `poll` runs the exact
            // same two-stage drain `apply_market_event`/`submit` already
            // run at their own top, just with no new message attached.
            let reports = venue.poll(now);
            if !reports.is_empty() {
                let outcome = engine.apply_venue_reports(reports, now);
                control_dispatcher.dispatch(cache, engine, venue, &outcome);
            }
        }
        // StrategyTimer/SessionTransition/StalenessTimeout/WatchdogExpiry/
        // OffloadCompletion: real, unbacked placeholders here -- nothing
        // schedules them yet (same honest status `strategy/README.md`
        // already gives `on_timer`/`on_session_change`/etc).
        _ => {}
    }
}

/// Keeps the Scheduler's own alarms in sync with whatever `venue` itself
/// says it next needs to be checked at (2026-08-27) -- called after every
/// `dispatch_event`, from both the lookahead-drain loop and the final
/// drain. Reschedules only when the due time actually *changes*
/// (`next_arrival_alarm`/`next_visibility_alarm` remember what's already
/// scheduled) -- an actively-ticking instrument would otherwise get one
/// redundant alarm pushed onto the Scheduler per real market event while
/// an order sits pending, for no benefit (`poll`'s own drain is already
/// idempotent against a stale duplicate).
fn sync_venue_alarms(venue: &SimExchange, sched: &mut Scheduler, next_arrival_alarm: &mut Option<i64>, next_visibility_alarm: &mut Option<i64>) {
    // Fast path: nothing pending, and no alarm was already scheduled --
    // by far the common case (hundreds of millions of market-data calls
    // per real order). Skips both `next_*_due_ns` scans below entirely.
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

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(config_path) = args.get(1) else {
        eprintln!("usage: {} <config-file>", args.first().map(String::as_str).unwrap_or("qtrade"));
        return ExitCode::FAILURE;
    };
    let cfg = match config::load(Path::new(config_path)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // Mode selection -- the CLI asked for this explicitly, rather than
    // silently assuming backtest; now it's a field in the config file
    // being asked, per D22/D39. No live feed source exists anywhere in
    // this codebase yet, so "live" fails cleanly instead of pretending
    // to work; building unused live scaffolding now would be speculative.
    if cfg.run.mode != "backtest" {
        eprintln!("mode {:?} not implemented yet -- only \"backtest\" is supported (no live feed source exists in this codebase yet)", cfg.run.mode);
        return ExitCode::FAILURE;
    }

    // One capture file is the common case; more than one is a k-way merge
    // for a strategy whose instruments live on different MCX stream files
    // the same day (`feed_replay::replay` does the merge -- see
    // `feed_replay_user_doc.md` §2b). `primary_path` (`recording_paths[0]`)
    // is what derives this day's real `MCXScrips.bcp` -- every stream of
    // one day shares one contract file, so any one resolves it.
    let capture_paths: &[String] = &cfg.run.recording_paths;
    let primary_path = capture_paths[0].as_str();
    let max_outer_records = cfg.run.max_outer_records;
    // `cfg.run.max_feed_stdout_lines` is `limit_order_book_generator`'s
    // own config knob -- unused while `multi_instrument_bracket` is
    // compiled in, since it doesn't produce a `feed.csv`.

    // Derived from `capture_path`'s own filename by `feed_replay`, so
    // pointing this at any real day's capture file finds that same
    // day's real reference data automatically -- no second date
    // argument to keep in sync with the first, and no hardcoded date.
    let master = match feed_replay::load_refdata(primary_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // The strategy declares *names* (`order_lifecycle_demo::UNDERLYINGS`);
    // this orchestrator resolves each to *this day's* real front-month
    // token via `feed_replay`. The strategy never sees a hardcoded
    // token from a different day.
    let resolved: Vec<(&str, Option<InstrumentId>)> = order_lifecycle_demo::UNDERLYINGS.iter().map(|name| (*name, feed_replay::resolve_front_month(&master, name))).collect();
    let tracked_ids: Vec<InstrumentId> = resolved.iter().filter_map(|(_, id)| *id).collect();
    if tracked_ids.is_empty() {
        eprintln!("none of {:?} resolved to a real front-month future in this day's refdata", order_lifecycle_demo::UNDERLYINGS);
        return ExitCode::FAILURE;
    }
    let names_by_id: HashMap<InstrumentId, &str> = resolved.iter().filter_map(|(name, id)| id.map(|i| (i, *name))).collect();
    let label_of = |id: InstrumentId| -> &str { names_by_id.get(&id).copied().unwrap_or("UNKNOWN") };
    // The reverse direction, for `StartCtx::resolve` -- same `resolved`
    // data `main.rs` already computed for its own filter/engine
    // construction, just keyed the other way round for a strategy's
    // `ctx.resolve(name)` to look up.
    let name_to_id: HashMap<&str, InstrumentId> = resolved.iter().filter_map(|(name, id)| id.map(|i| (*name, i))).collect();

    let filter = InstrumentFilter::from_native_ids(tracked_ids.iter().map(|id| id.0 as i64));

    // The real `Instrument` record(s) `execution::ExecutionEngine` needs
    // for tick-size/freeze-qty validation and cost-model lookups -- kept
    // ready regardless of whether the currently plugged-in strategy ever
    // actually submits an order (some don't -- `LimitOrderBookGenerator`
    // never does, and `orders.log`/`fills.log`/`report.txt` legitimately
    // come out empty for it; `multi_instrument_bracket`, compiled in now, does).
    //
    // `freeze_qty` is overridden below -- a separate, still-open gap:
    // `refdata` has no source column for freeze quantity and defaults
    // every instrument's `freeze_qty` to `0`, which would deny every
    // order regardless of tick-size units. This override is demo-only
    // headroom for whatever strategy is plugged in, in `Lots`, not a
    // claim about MCX's real freeze quantity.
    const DEMO_FREEZE_QTY_LOTS: i64 = 1_000;
    let trade_instruments: Vec<types::Instrument> = master
        .all()
        .iter()
        .filter(|i| tracked_ids.contains(&i.id))
        .cloned()
        .map(|mut i| {
            i.freeze_qty = DEMO_FREEZE_QTY_LOTS;
            i
        })
        .collect();

    println!(
        "refdata: {} instruments loaded, filter admits {} native ids, {} of them resolved for order entry",
        master.all().len(),
        filter.len(),
        trade_instruments.len()
    );
    for id in &tracked_ids {
        println!("  resolved {}: native id {}", label_of(*id), id.0);
    }

    let mut cache = Cache::new(master, filter);

    // **No hardcoded per-day band values here.** Both `19_01_2026`'s own
    // CRUDEOIL increment file and every other real day checked so far
    // start recording after the one real Start-of-Day broadcast that
    // would have carried each instrument's `InstrumentInfo` (13603) --
    // so `book` correctly panics rather than guess, exactly as designed,
    // the moment the increment-only stream is played alone.
    // `feed_replay::scan_snapshot_for_bands` -- the structurally
    // backtest-only piece of this pipeline -- pre-scans the paired
    // snapshot file for the real, full-session union of each tracked
    // instrument's band and seeds it before the main replay starts.
    // Works for any day whose paired snapshot file exists, automatically.
    // One paired snapshot file per capture stream. A tracked instrument's
    // real band comes from whichever stream's snapshot actually carries
    // it (e.g. CRUDEOIL on stream 2, NATURALGAS on stream 4 the same
    // day), so scan every stream and union the results before the replay
    // starts.
    let mut seeded: std::collections::HashSet<InstrumentId> = std::collections::HashSet::new();
    for path in capture_paths {
        let Some(snapshot_path) = feed_replay::snapshot_path_for(path) else {
            println!("{path} doesn't look like an Increment_capture file -- no paired snapshot file to auto-seed from");
            continue;
        };
        println!("scanning the paired snapshot file for real price bands: {snapshot_path}");
        match feed_replay::scan_snapshot_for_bands(&snapshot_path, &tracked_ids) {
            Ok(bands) => {
                for (id, (lower, upper, count)) in &bands {
                    println!(
                        "  {} ({}): real band [Rs {:.2}, Rs {:.2}], full-session union of {count} InstrumentInfo (13603) records",
                        label_of(*id),
                        id.0,
                        *lower as f64 / RUPEE_RAW,
                        *upper as f64 / RUPEE_RAW,
                    );
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
    let venue_otr = simulator::OtrConfig {
        window: std::time::Duration::from_secs(1),
        max_messages_per_window: 10_000,
        max_otr_ratio: 1_000_000.0,
    };
    // `SimExchange` built here now, not inside `ExecutionEngine::new` --
    // it's a sibling `main.rs` owns directly (dual-clock replay,
    // 2026-08-27), reachable from two places: the Scheduler's direct
    // market-event feed, and `ExecutionEngine`'s own order-delivery
    // methods, which now take it as a borrowed parameter (same pattern
    // `ControlDispatcher::subscribe` already uses for `EventDispatcher`).
    // `with_order_latency` (2026-08-27, split into outbound/inbound legs
    // the same day): the real ~500μs MCX round trip now lives entirely
    // inside the venue itself -- `ExecutionEngine`/`Ctx` call
    // `submit`/`cancel`/`modify` immediately and have no idea whether the
    // venue answers now or later. See
    // `simulator::SimExchange::with_order_latency`'s own doc comment.
    let mut sim_venue = SimExchange::new(&tracked_ids, venue_otr).with_order_latency(cfg.run.order_outbound_latency_ns, cfg.run.order_inbound_latency_ns);
    let mut engine = ExecutionEngine::new(run_config, trade_instruments, Box::new(execution::AlwaysAllowRms), CostConfig::default(), vec![1_000_000, 5_000_000], true);

    // A fresh, timestamped folder per run -- report generation location
    // is orchestrator config, not strategy or feed-replay concern -- so
    // two runs never overwrite each other's `feed.csv`/`orders.log`/
    // `fills.log`/`report.txt`.
    let run_dir = format!("{}/{}", cfg.run.report_dir, run_timestamp_ist());
    if let Err(e) = fs::create_dir_all(&run_dir) {
        eprintln!("failed to create {run_dir}: {e}");
        return ExitCode::FAILURE;
    }
    println!("run output folder: {run_dir}\n");

    // Low-latency, off the hot path (2026-08-27): every `tracing::info!`/
    // `debug!` call from here on is a cheap, non-blocking channel push --
    // the real file/stdout write happens on `tracing-appender`'s own
    // background worker thread, never on whatever thread is driving the
    // replay loop. `_log_guards` must live for the rest of `main()` --
    // dropping either guard early stops its worker and silently drops
    // whatever's still queued. See `logging.rs`'s own header for the
    // full account (including why this is qtrade's first-ever external
    // dependency).
    let events_log_path = format!("{run_dir}/events.log");
    let log_level = logging::LogLevel::parse(&cfg.run.log_level);
    let _log_guards = match logging::init(log_level, Path::new(&events_log_path)) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("failed to create {events_log_path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let strategy = OrderLifecycleDemo::new();
    // `Rc<RefCell<_>>`, not `Box`: a clone (coerced to `dyn Strategy`) is
    // what actually moves into `event_dispatcher`'s/`control_dispatcher`'s
    // registries below -- this concrete handle stays here so the final
    // summary section can still call `round_trips()`, which isn't part
    // of `Strategy` (it's this strategy's own instrumentation, not
    // something every strategy has).
    let strategy = Rc::new(RefCell::new(strategy));

    let mut event_dispatcher = EventDispatcher::new();
    let mut control_dispatcher = ControlDispatcher::new();
    let my_id = event_dispatcher.register(strategy.clone() as Rc<RefCell<dyn Strategy>>);
    // Same strategy instance, registered a second time -- `EventDispatcher`
    // will only ever call `.on_book()`/`.on_trade()` on it,
    // `ControlDispatcher` only ever `.on_fill()`/`.on_order_update()`;
    // `multi_instrument_bracket` is the first strategy where the latter
    // drives real state (`WaitingForEntryFill`/`WaitingForExitFill` onward)
    // across more than one instrument at once.
    control_dispatcher.register(strategy.clone() as Rc<RefCell<dyn Strategy>>);
    {
        // Same resolution `main.rs` already did above for its own
        // filter/engine construction, exposed to the strategy under a
        // mode-agnostic signature (`ctx.resolve`) -- what's behind it is
        // what differs per mode, not the call the strategy makes.
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

    // Dual-clock replay (2026-08-27): `SimExchange` runs on the exchange's
    // own clock (`exchange_ts`), `Cache`/`EventDispatcher`/`Strategy` run
    // on the recorder's (`recorder_ts` = `exchange_ts` + real, measured
    // feed latency) -- see `feed_replay::ReplayEvent`'s own doc comment
    // and `main_user_doc.md` for the full account, including the real
    // check against `19_08_2026` that grounded this. `Scheduler`
    // (`scheduler.rs`, built but never called until the dual-clock pass)
    // is what makes a single real message deliverable at two different
    // scheduled times without two separate passes over the file --
    // that's still real; only the *order-latency* scheduling this same
    // loop used to also drive (`OrderArrival`/`ReportDelivery`) moved
    // into `SimExchange` itself the same day (see `strategy.rs`'s header).
    let mut sched = Scheduler::new();
    let max_feed_delta_ns = cfg.run.max_feed_delta_ns as i64;
    // `sync_venue_alarms`'s own memory of what's already scheduled
    // (2026-08-27) -- see that function's doc comment: a real exchange
    // answers at its own processing time even in a quiet market, so
    // `sim_venue` itself has to be checked after every dispatched event,
    // not just when a real market tick happens to nudge it.
    let mut next_arrival_alarm: Option<i64> = None;
    let mut next_visibility_alarm: Option<i64> = None;

    let stats = match feed_replay::replay(capture_paths, max_outer_records, |ev| {
        let exchange_ts = ev.exchange_ts as i64;
        let recorder_ts = ev.recorder_ts;
        let delta = recorder_ts - exchange_ts;
        // D20 fail-fast, decision #1 of the 2026-08-27 planning session --
        // never clamped. A negative delta or one past the ceiling means
        // the input data itself isn't trustworthy; better to know that
        // now than to model physics off a fabricated number.
        if delta < 0 || delta > max_feed_delta_ns {
            eprintln!(
                "FATAL: implausible feed-latency delta at seq={}: recorder_ts={recorder_ts} exchange_ts={exchange_ts} delta={delta}ns (ceiling {max_feed_delta_ns}ns)",
                ev.seq_no
            );
            std::process::exit(1);
        }

        // Safe lookahead drain: nothing still unread in the file can ever
        // produce a timestamp earlier than *this* message's own
        // `exchange_ts` (exchange_ts is monotonic non-decreasing across
        // one venue's stream). Draining strictly before it -- never at or
        // past it -- so this message's own not-yet-scheduled SimExchange
        // delivery, or an exact tie already queued, is never popped
        // early. See `main_user_doc.md` for the full reasoning and why
        // this doesn't need a full priority-queue merge against the file
        // read itself.
        while let Some(peek_ts) = sched.peek_earliest_timestamp() {
            if peek_ts >= exchange_ts {
                break;
            }
            let event = sched.pop_earliest().expect("just peeked Some");
            dispatch_event(event, &mut cache, &mut engine, &mut sim_venue, &mut event_dispatcher, &mut control_dispatcher);
            sync_venue_alarms(&sim_venue, &mut sched, &mut next_arrival_alarm, &mut next_visibility_alarm);
        }

        sched.schedule(exchange_ts, EventClass::MarketData, EventPayload::MarketData { target: Target::SimExchange, message: *ev.event, seq_no: ev.seq_no, exchange_ts: ev.exchange_ts, recorder_ts: ev.recorder_ts });
        sched.schedule(recorder_ts, EventClass::MarketData, EventPayload::MarketData { target: Target::Cache, message: *ev.event, seq_no: ev.seq_no, exchange_ts: ev.exchange_ts, recorder_ts: ev.recorder_ts });
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to open capture stream(s) {capture_paths:?}: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Final drain -- nothing more can arrive once the file is exhausted.
    // Venue alarms still fire correctly here too: any order still
    // in-flight when the file ends gets its own outbound/inbound wake-ups
    // popped in the same loop, same as during real replay.
    while let Some(event) = sched.pop_earliest() {
        dispatch_event(event, &mut cache, &mut engine, &mut sim_venue, &mut event_dispatcher, &mut control_dispatcher);
        sync_venue_alarms(&sim_venue, &mut sched, &mut next_arrival_alarm, &mut next_visibility_alarm);
    }

    // `Strategy::on_stop` -- real, wired (Q3 of the 2026-08-25 design
    // session): "shutting down, last chance to clean up." `can_submit:
    // false` -- unlike `on_book`/`on_trade`, nothing forwards whatever a
    // submit here might produce (the replay loop has already ended,
    // there is no more `control_dispatcher.dispatch` call coming), so a
    // write must fail loudly rather than be silently lost. `now_ns: 0` --
    // there is no real "current event" once the run is over for this to
    // report.
    {
        let mut stop_ctx = strategy::Ctx::new(&cache, &mut engine, &mut sim_venue, 0, strategy::DEFAULT_STRATEGY_ID, false);
        strategy.borrow_mut().on_stop(&mut stop_ctx);
    }

    println!("\n--- summary ---");
    println!("outer records processed: {}", stats.outer_records);
    println!("events (decoded messages) processed: {}", stats.events);
    println!(
        "elapsed: {:.2}s ({:.0} records/s, {:.0} messages/s)",
        stats.elapsed.as_secs_f64(),
        stats.outer_records as f64 / stats.elapsed.as_secs_f64(),
        stats.events as f64 / stats.elapsed.as_secs_f64()
    );
    println!("round trips: {}", strategy.borrow().round_trips().len());
    for (i, (name, entry_raw, exit_raw, reason)) in strategy.borrow().round_trips().iter().enumerate() {
        println!(
            "  #{}: {name}: entry Rs {:.2} -> exit Rs {:.2} ({reason}), {:+.2} Rs/lot before costs",
            i + 1,
            *entry_raw as f64 / RUPEE_RAW,
            *exit_raw as f64 / RUPEE_RAW,
            (*exit_raw - *entry_raw) as f64 / RUPEE_RAW
        );
    }
    for id in &tracked_ids {
        if let Some(b) = cache.book(*id) {
            println!("final {} (as seen by cache): bid={} ask={} state={:?}", label_of(*id), fmt_level(b.best_bid()), fmt_level(b.best_ask()), b.state());
        }
    }

    // --- Write the remaining report files (orders/fills/Tier 1) -- PnL
    // calculation itself already lives in `execution.rs`'s
    // `tier1_report`/`order_events`/`fills`, unchanged by this merge;
    // only the output directory (`run_dir`, already created above) is
    // this orchestrator's job. Legitimately empty for a strategy (like
    // this one) that never calls `submit_order`.
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

    ExitCode::SUCCESS
}

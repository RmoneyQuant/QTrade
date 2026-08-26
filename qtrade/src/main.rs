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
// Not used anywhere yet (see main_user_doc.md's honest account of this
// gap) -- declared here purely so its own real tests (priority queue,
// tie-break ordering) keep compiling and running as part of this crate's
// one real binary, the same reason the old decode-only main.rs declared
// it. Dropping this declaration during the backtester/main.rs merge
// would have silently removed scheduler's test coverage from `cargo
// test` entirely -- caught and fixed before it shipped.
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
// `naturalgas_bracket` (a real, order-placing strategy); swap back to
// `limit_order_book_generator/limit_order_book_generator.rs` for the
// pure-observer one.
#[path = "strategy/naturalgas_bracket/naturalgas_bracket.rs"]
mod naturalgas_bracket;

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
use naturalgas_bracket::NaturalGasBracket;
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

    let capture_path = cfg.run.recording_path.as_str();
    let max_outer_records = cfg.run.max_outer_records;
    // `cfg.run.max_feed_stdout_lines` is `limit_order_book_generator`'s
    // own config knob -- unused while `naturalgas_bracket` is compiled
    // in, since it doesn't produce a `feed.csv`.

    // Derived from `capture_path`'s own filename by `feed_replay`, so
    // pointing this at any real day's capture file finds that same
    // day's real reference data automatically -- no second date
    // argument to keep in sync with the first, and no hardcoded date.
    let master = match feed_replay::load_refdata(capture_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // The strategy declares *names* (`naturalgas_bracket::UNDERLYINGS`);
    // this orchestrator resolves each to *this day's* real front-month
    // token via `feed_replay`. The strategy never sees a hardcoded
    // token from a different day.
    let resolved: Vec<(&str, Option<InstrumentId>)> = naturalgas_bracket::UNDERLYINGS.iter().map(|name| (*name, feed_replay::resolve_front_month(&master, name))).collect();
    let tracked_ids: Vec<InstrumentId> = resolved.iter().filter_map(|(_, id)| *id).collect();
    if tracked_ids.is_empty() {
        eprintln!("none of {:?} resolved to a real front-month future in this day's refdata", naturalgas_bracket::UNDERLYINGS);
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
    // come out empty for it; `naturalgas_bracket`, compiled in now, does).
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
    if let Some(snapshot_path) = feed_replay::snapshot_path_for(capture_path) {
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
                }
                for id in &tracked_ids {
                    if !bands.contains_key(id) {
                        println!("  {} ({}): no InstrumentInfo found in the snapshot file -- not seeded, `book` will panic if a real order arrives before it learns one", label_of(*id), id.0);
                    }
                }
            }
            Err(e) => println!("  could not scan {snapshot_path}: {e} -- proceeding unseeded"),
        }
        println!();
    } else {
        println!("capture path doesn't look like an Increment_capture file -- no paired snapshot file to auto-seed from\n");
    }

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
    let mut engine = ExecutionEngine::new(
        run_config,
        trade_instruments,
        Box::new(execution::AlwaysAllowRms),
        CostConfig::default(),
        venue_otr,
        vec![1_000_000, 5_000_000],
        true,
    );

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

    let strategy = NaturalGasBracket::new();
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
    // `naturalgas_bracket` is the first strategy where the latter drives
    // real state (`WaitingForEntryFill`/`WaitingForExitFill` onward).
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
    println!("streaming {capture_path} record-by-record ({limit_desc})\n");

    let stats = match feed_replay::replay(capture_path, max_outer_records, |ev| {
        // Phase C, Q1: the venue applies this event *before* the strategy
        // is ever asked to react to it -- so by the time on_book/on_trade
        // fire, Cache and SimExchange already agree on this exact event.
        // A strategy that submits from on_book/on_trade would otherwise
        // be acting against a one-event-stale venue book.
        let outcome = engine.on_market_event(ev.event, ev.now_ns);
        control_dispatcher.dispatch(&cache, &mut engine, &outcome);

        if let Some(instrument) = cache.apply(ev.event) {
            if let Some(book) = cache.book(instrument) {
                let strategy_outcome = event_dispatcher.on_book_touched(book, instrument, &cache, &mut engine, ev.seq_no, ev.packet_transact_time_ns);
                control_dispatcher.dispatch(&cache, &mut engine, &strategy_outcome);
            }
        }
        if let decoder::DecodedMessage::Trade(t) = ev.event {
            if cache.filter().passes(t.security_id) {
                let strategy_outcome = event_dispatcher.on_trade(&cache, &mut engine, InstrumentId(t.security_id as u32), t, ev.seq_no, ev.packet_transact_time_ns);
                control_dispatcher.dispatch(&cache, &mut engine, &strategy_outcome);
            }
        }
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to open {capture_path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    // `Strategy::on_stop` -- real, wired (Q3 of today's design session):
    // "shutting down, last chance to clean up." `can_submit: false` --
    // unlike `on_book`/`on_trade`, nothing forwards whatever a submit
    // here might produce (the replay loop has already ended, there is no
    // more `control_dispatcher.dispatch` call coming), so a write must
    // fail loudly rather than be silently lost. `now_ns: 0` -- there is
    // no real "current event" once the run is over for this to report.
    {
        let mut stop_ctx = strategy::Ctx::new(&cache, &mut engine, 0, strategy::DEFAULT_STRATEGY_ID, false);
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
    for (i, (entry_raw, exit_raw, reason)) in strategy.borrow().round_trips().iter().enumerate() {
        println!(
            "  #{}: entry Rs {:.2} -> exit Rs {:.2} ({reason}), {:+.2} Rs/lot before costs",
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
    let tier1 = engine.tier1_report();
    fs::write(&report_path, format!("{tier1}")).expect("write report.txt");

    println!("\n--- report (Tier 1) ---\n{tier1}");
    println!("logs written:");
    println!("  {orders_path}  ({} order events)", engine.order_events().len());
    println!("  {fills_path}  ({} fills)", engine.fills().len());
    println!("  {report_path}");

    ExitCode::SUCCESS
}

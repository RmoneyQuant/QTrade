//! qtrade -- the demo CLI. `[[bin]] qtrade`.
//!
//! **This file is now a thin wrapper, not the engine (2026-09-04).** The
//! whole backtest engine -- config loading, mode selection, constructing
//! `Cache`/`ExecutionEngine`/`SimExchange`, the dual-clock Scheduler
//! loop, writing `orders.log`/`fills.log`/`report.txt` -- moved to
//! `lib.rs`'s `run_backtest`, so an external crate can depend on
//! `qtrade`, `impl qtrade::Strategy` for its own type, and call
//! `run_backtest` itself instead of forking this source tree. This
//! binary is just one example caller of that library: it compiles in
//! `order_lifecycle_demo` (see `strategy/README.md` for why only one
//! strategy is ever compiled into any one binary) and prints that
//! strategy's own `round_trips()` summary after the run -- something
//! `run_backtest` itself can't do, since it only ever calls `Strategy`'s
//! trait methods, not one concrete strategy's own extra methods.
//!
//! Invocation unchanged: `qtrade <config-file>` -- mode (`backtest`,
//! eventually `live`) is still a field inside that file (`[run] mode =
//! "..."`), per D22/D39/BACKTEST-PHASE1.md §2.3.
//!
//! See `main_user_doc.md` for the fuller history (including why this
//! file used to be decode-only, and why `backtester.rs` used to be a
//! separate binary) and `lib.rs`'s own header for what moved and why.

// This strategy module is declared here, in the *binary*, not in
// `lib.rs` -- a strategy lives in the consuming crate, and this binary
// is (for demo purposes) that consuming crate. Swapping strategies means
// pointing this declaration (and the two lines that construct/use it
// below) at a different subfolder, not compiling more than one in.
#[path = "strategy/order_lifecycle_demo/order_lifecycle_demo.rs"]
mod order_lifecycle_demo;

use std::env;
use std::path::Path;
use std::process::ExitCode;

use order_lifecycle_demo::OrderLifecycleDemo;

const RUPEE_RAW: f64 = 100_000_000.0;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(config_path) = args.get(1) else {
        eprintln!("usage: {} <config-file>", args.first().map(String::as_str).unwrap_or("qtrade"));
        return ExitCode::FAILURE;
    };

    let strategy = match qtrade::run_backtest(Path::new(config_path), order_lifecycle_demo::UNDERLYINGS, OrderLifecycleDemo::new()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    // `round_trips()` is `OrderLifecycleDemo`'s own instrumentation, not
    // part of `Strategy` -- `run_backtest` has no idea it exists, which
    // is exactly why it hands the strategy handle back rather than
    // printing this itself. Any other strategy compiled in here would
    // print whatever *it* wants at this same point instead.
    let strategy = strategy.borrow();
    println!("round trips: {}", strategy.round_trips().len());
    for (i, (name, entry_raw, exit_raw, reason)) in strategy.round_trips().iter().enumerate() {
        println!(
            "  #{}: {name}: entry Rs {:.2} -> exit Rs {:.2} ({reason}), {:+.2} Rs/lot before costs",
            i + 1,
            *entry_raw as f64 / RUPEE_RAW,
            *exit_raw as f64 / RUPEE_RAW,
            (*exit_raw - *entry_raw) as f64 / RUPEE_RAW
        );
    }

    ExitCode::SUCCESS
}

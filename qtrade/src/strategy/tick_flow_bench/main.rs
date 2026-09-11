#[path = "tick_flow_bench.rs"]
mod tick_flow_bench;

use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;
use tick_flow_bench::TickFlowBench;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(config_path) = args.get(1) else {
        eprintln!("usage: tick-flow-bench <config-file>");
        return ExitCode::FAILURE;
    };

    let start = Instant::now();
    let strategy = match qtrade::run_backtest(Path::new(config_path), TickFlowBench::new()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let elapsed = start.elapsed();

    let strategy = strategy.borrow();
    let (ticks, submitted, filled, timeout_cancels) = strategy.stats();
    println!("--- tick-flow-bench summary ---");
    println!("wall clock: {:.2}s", elapsed.as_secs_f64());
    println!("ticks (on_book calls): {ticks}");
    println!("orders submitted: {submitted}");
    println!("orders filled: {filled}");
    println!("orders cancelled on 5-tick timeout: {timeout_cancels}");

    ExitCode::SUCCESS
}

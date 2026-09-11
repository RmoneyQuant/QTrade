#[path = "instrument_selection_demo.rs"]
mod instrument_selection_demo;

use instrument_selection_demo::InstrumentSelectionDemo;
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(config_path) = args.get(1) else {
        eprintln!("usage: instrument-selection-demo <config-file>");
        return ExitCode::FAILURE;
    };
    match qtrade::run_backtest(Path::new(config_path), InstrumentSelectionDemo::new()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

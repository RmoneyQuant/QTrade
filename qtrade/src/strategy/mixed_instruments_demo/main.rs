#[path = "mixed_instruments_demo.rs"]
mod mixed_instruments_demo;

use mixed_instruments_demo::MixedInstrumentsDemo;
use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(config_path) = args.get(1) else {
        eprintln!("usage: mixed-instruments-demo <config-file>");
        return ExitCode::FAILURE;
    };
    match qtrade::run_backtest(Path::new(config_path), MixedInstrumentsDemo::new()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

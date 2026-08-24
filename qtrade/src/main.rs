//! CLI entry point. All decoding logic lives in the `decoder` component
//! (src/decoder/decoder.rs, docs in src/decoder/user_doc.md) — this file
//! only parses arguments and prints the final summary.

#[path = "decoder/decoder.rs"]
mod decoder;

#[allow(dead_code)]
#[path = "types/types.rs"]
mod types;

#[allow(dead_code)]
#[path = "scheduler/scheduler.rs"]
mod scheduler;

#[allow(dead_code)]
#[path = "refdata/refdata.rs"]
mod refdata;

#[allow(dead_code)]
#[path = "book/book.rs"]
mod book;

#[allow(dead_code)]
#[path = "cache/cache.rs"]
mod cache;

#[allow(dead_code)]
#[path = "simulator/simulator.rs"]
mod simulator;

#[allow(dead_code)]
#[path = "execution/execution.rs"]
mod execution;

use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    let debug = if let Some(i) = args.iter().position(|a| a == "--debug") {
        args.remove(i);
        true
    } else {
        false
    };

    if args.len() < 2 {
        eprintln!(
            "usage: {} <capture-file> [max-records-to-print] [skip-records] [--debug]",
            args[0]
        );
        eprintln!("  --debug prints each message's full field dump ({{:?}}) instead of the one-line summary ({{}})");
        return ExitCode::FAILURE;
    }
    let path = &args[1];
    let max_print: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20);
    let skip: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);

    let data = match fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            return ExitCode::FAILURE;
        }
    };

    let summary = decoder::decode_file(&data, skip, max_print, debug);

    println!("\n--- summary ---");
    println!("file: {path}");
    println!("records decoded: {}", summary.records);
    let exact = summary.bytes_consumed == data.len();
    println!(
        "bytes consumed: {} / {} ({})",
        summary.bytes_consumed,
        data.len(),
        if exact { "exact match" } else { "MISMATCH -- investigate" }
    );
    println!("message counts by template id:");
    for (tid, count) in &summary.template_counts {
        println!("  {tid:>6} : {count}");
    }

    ExitCode::SUCCESS
}

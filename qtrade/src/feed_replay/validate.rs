//! `feed_replay`'s own regression check: real, known-good price bands
//! for three real (instrument, day) pairs, re-derived from real snapshot
//! files via `scan_snapshot_for_bands` and compared against the exact
//! values this project already hand-verified this session. Not part of
//! `feed_replay`'s public API, same reason every other component's own
//! `validate.rs` exists (no `[lib]` target, `main.rs` intentionally
//! untouched) -- see `feed_replay_user_doc.md` for how these numbers
//! were first derived.

#[path = "../types/types.rs"]
mod types;
#[path = "../decoder/decoder.rs"]
mod decoder;
#[path = "../refdata/refdata.rs"]
mod refdata;
#[path = "feed_replay.rs"]
mod feed_replay;

use types::InstrumentId;

const RUPEE_RAW: f64 = 100_000_000.0;

struct Case {
    label: &'static str,
    snapshot_path: &'static str,
    id: InstrumentId,
    expect_lower: f64,
    expect_upper: f64,
    expect_count: u32,
}

fn main() {
    let cases = [
        // Expected values below were independently re-verified 2026-08-24
        // via a from-scratch Python byte parser reading these exact real
        // files (not carried over from an earlier session's memory --
        // an earlier recollection of the CRUDEOIL/NATURALGAS counts here
        // conflated a different metric, "snapshot cycles", with the real
        // `InstrumentInfo` record count checked here, and was off by one;
        // the 15_06_2026 CRUDEOIL figures were substantially wrong in the
        // same stale recollection. Both this code's own output and the
        // independent Python parser agree exactly on the values below.
        Case {
            label: "CRUDEOIL (467013), 19_01_2026, stream 4",
            snapshot_path: "/mnt/MCX_Recording_Files/mcx_feeder_snapshot_capture_19_01_2026_1_4.bin",
            id: InstrumentId(467_013),
            expect_lower: 5232.00,
            expect_upper: 5666.00,
            expect_count: 8025,
        },
        Case {
            label: "NATURALGAS (465849), 19_01_2026, stream 5",
            snapshot_path: "/mnt/MCX_Recording_Files/mcx_feeder_snapshot_capture_19_01_2026_1_5.bin",
            id: InstrumentId(465_849),
            expect_lower: 221.60,
            expect_upper: 339.20,
            expect_count: 1271,
        },
        Case {
            label: "CRUDEOIL (499095), 15_06_2026, stream 2",
            snapshot_path: "/mnt/MCX_Recording_Files/mcx_feeder_snapshot_capture_15_06_2026_1_2.bin",
            id: InstrumentId(499_095),
            expect_lower: 7347.00,
            expect_upper: 8799.00,
            expect_count: 6584,
        },
    ];

    let mut failures = 0;
    for case in &cases {
        print!("{}: ", case.label);
        match feed_replay::scan_snapshot_for_bands(case.snapshot_path, &[case.id]) {
            Ok(bands) => match bands.get(&case.id) {
                Some(&(lower_raw, upper_raw, count)) => {
                    let lower = lower_raw as f64 / RUPEE_RAW;
                    let upper = upper_raw as f64 / RUPEE_RAW;
                    let ok = (lower - case.expect_lower).abs() < 0.005 && (upper - case.expect_upper).abs() < 0.005 && count == case.expect_count;
                    if ok {
                        println!("OK -- band [{lower:.2}, {upper:.2}], {count} InstrumentInfo records");
                    } else {
                        println!(
                            "MISMATCH -- got band [{lower:.2}, {upper:.2}], {count} records; expected [{:.2}, {:.2}], {} records",
                            case.expect_lower, case.expect_upper, case.expect_count
                        );
                        failures += 1;
                    }
                }
                None => {
                    println!("MISMATCH -- no InstrumentInfo found for {} in {}", case.id.0, case.snapshot_path);
                    failures += 1;
                }
            },
            Err(e) => {
                println!("ERROR -- could not scan {}: {e}", case.snapshot_path);
                failures += 1;
            }
        }
    }

    if failures == 0 {
        println!("\nall {} real band checks passed", cases.len());
    } else {
        println!("\n{failures} of {} real band checks FAILED", cases.len());
        std::process::exit(1);
    }
}

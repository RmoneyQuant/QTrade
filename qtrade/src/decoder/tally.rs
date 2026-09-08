//! Coverage-audit tool, not part of the library. Streams one or more real
//! MCX increment capture files start to finish, decodes every message,
//! and tallies counts per `DecodedMessage` variant -- specifically to
//! answer one question: after adding decoders for the 5 previously-
//! unparsed template IDs (13201, 13300, 13301, 13302, 13604), does a
//! real, multi-day corpus still produce any `Unknown` messages?
//!
//! Standalone `[[bin]]`, same pattern as `book/validate.rs` -- this crate
//! has a `[lib]` target now, but this tool predates needing it and the
//! `#[path]` pull-in is simpler than wiring a temporary pub re-export.
//!
//! Usage: `decoder-tally <file1.bin> [file2.bin ...]`

#[path = "decoder.rs"]
mod decoder;

use decoder::DecodedMessage;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read};

struct RecordSource {
    reader: BufReader<File>,
}

impl RecordSource {
    fn open(path: &str) -> io::Result<Self> {
        Ok(RecordSource { reader: BufReader::with_capacity(1 << 20, File::open(path)?) })
    }

    fn next_record(&mut self, payload: &mut Vec<u8>) -> io::Result<bool> {
        let mut hdr = [0u8; 16];
        match self.reader.read_exact(&mut hdr) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
            Err(e) => return Err(e),
        }
        let length = u64::from_le_bytes(hdr[0..8].try_into().unwrap()) as usize;
        if length < 8 {
            return Ok(false);
        }
        let payload_len = length - 8;
        payload.resize(payload_len, 0);
        self.reader.read_exact(payload)?;
        Ok(true)
    }
}

fn variant_name(m: &DecodedMessage) -> &'static str {
    match m {
        DecodedMessage::PacketHeader(_) => "PacketHeader",
        DecodedMessage::Heartbeat(_) => "Heartbeat",
        DecodedMessage::OrderAdd(_) => "OrderAdd",
        DecodedMessage::OrderModify(_) => "OrderModify",
        DecodedMessage::OrderModifySamePriority(_) => "OrderModifySamePriority",
        DecodedMessage::OrderDelete(_) => "OrderDelete",
        DecodedMessage::OrderMassDelete(_) => "OrderMassDelete",
        DecodedMessage::Trade(_) => "Trade",
        DecodedMessage::ExecutionSummary(_) => "ExecutionSummary",
        DecodedMessage::TopOfBook(_) => "TopOfBook",
        DecodedMessage::SnapshotProductSummary(_) => "SnapshotProductSummary",
        DecodedMessage::SnapshotInstrumentSummary(_) => "SnapshotInstrumentSummary",
        DecodedMessage::SnapshotOrder(_) => "SnapshotOrder",
        DecodedMessage::InstrumentInfo(_) => "InstrumentInfo",
        DecodedMessage::TradeReport(_) => "TradeReport",
        DecodedMessage::ProductStateChange(_) => "ProductStateChange",
        DecodedMessage::InstrumentStateChange(_) => "InstrumentStateChange",
        DecodedMessage::MassInstrumentStateChange(_) => "MassInstrumentStateChange",
        DecodedMessage::IndexInfo(_) => "IndexInfo",
        DecodedMessage::Unknown(_) => "Unknown",
    }
}

fn main() -> io::Result<()> {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("usage: decoder-tally <file1.bin> [file2.bin ...]");
        std::process::exit(1);
    }

    let mut totals: BTreeMap<&'static str, u64> = BTreeMap::new();
    let mut unknown_by_template: BTreeMap<u16, u64> = BTreeMap::new();
    let mut band_seen: BTreeMap<i64, (i64, i64, u32)> = BTreeMap::new(); // security_id -> (lower, upper, count of InstrumentInfo)

    for path in &paths {
        eprintln!("=== {path} ===");
        let mut source = RecordSource::open(path)?;
        let mut payload = Vec::new();
        let mut outer_records: u64 = 0;
        let mut inner_messages: u64 = 0;

        while source.next_record(&mut payload)? {
            outer_records += 1;
            let mut off = 0usize;
            while off + 8 <= payload.len() {
                let body_len = u16::from_le_bytes([payload[off], payload[off + 1]]) as usize;
                let template_id = u16::from_le_bytes([payload[off + 2], payload[off + 3]]);
                let seq = u32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap());
                if body_len == 0 || off + body_len > payload.len() {
                    break;
                }
                let msg = decoder::decode_message(template_id, seq, &payload[off..off + body_len]);
                inner_messages += 1;
                *totals.entry(variant_name(&msg)).or_insert(0) += 1;
                match &msg {
                    DecodedMessage::Unknown(u) => {
                        *unknown_by_template.entry(u.template_id).or_insert(0) += 1;
                    }
                    DecodedMessage::InstrumentInfo(info) => {
                        let lower = info.lower_daily_price_limit.0;
                        let upper = info.upper_daily_price_limit.0;
                        let entry = band_seen.entry(info.security_id).or_insert((0, 0, 0));
                        entry.2 += 1;
                        // last plausible band wins; widening/union logic lives in `book`, not here -- this
                        // tool only reports what was observed, per instrument, across the whole file
                        if lower != i64::MIN && upper != i64::MIN && lower < upper {
                            entry.0 = lower;
                            entry.1 = upper;
                        }
                    }
                    _ => {}
                }
                off += body_len;
            }
        }

        eprintln!("  outer records: {outer_records}");
        eprintln!("  inner messages: {inner_messages}");
    }

    println!("\n=== message-type totals across all files ===");
    for (name, count) in &totals {
        println!("{name:<28} {count}");
    }

    if unknown_by_template.is_empty() {
        println!("\nzero Unknown messages -- 100% of template IDs seen were decoded");
    } else {
        println!("\n=== Unknown messages by template ID ===");
        for (tid, count) in &unknown_by_template {
            println!("  template {tid:<8} count={count}");
        }
    }

    println!("\n=== InstrumentInfo (13603) bands observed, by SecurityID ===");
    for (id, (lower, upper, count)) in &band_seen {
        println!(
            "  SecurityID={id:<10} lower={:<12} upper={:<12} (~{:.2}..{:.2}) occurrences={count}",
            lower,
            upper,
            *lower as f64 / 100_000_000.0,
            *upper as f64 / 100_000_000.0
        );
    }

    Ok(())
}

//! A faithful Rust port of `references/CME_Data_Check.cpp` -- reads the
//! CME feeder's **decrypted output** file (`cme_feeder_<DD_MM_YYYY>.bin`)
//! and prints one block per record in exactly the C++ reader's format.
//!
//! # Which file this reads
//!
//! The naming convention matters, and getting it wrong cost a whole
//! session (2026-09-01):
//!
//! | Name | What it is | Readable here? |
//! |---|---|---|
//! | `cme_feeder_capture_<Tier>_<date>.bin` | raw `CME_Recorder` output, the feeder's **input** | **no** -- encrypted |
//! | `cme_feeder_<date>.bin` | the feeder's **output** | **yes** -- this file |
//!
//! Settled from `Multicast_Feeder_CME.cpp` itself: line 244 `fopen`s the
//! `_capture_` name `"r"`, line 371 `fopen`s `cme_feeder_%s.bin` `"wb"`.
//!
//! # Record framing
//!
//! The feeder writes three values per record (`Multicast_Feeder_CME.cpp`
//! lines 337-340):
//!
//! ```text
//! nTot = sizeof(FEED_DETAILS) + sizeof(uint64_t)   // 216 + 8 = 224, constant
//! fwrite(&nTot,       8);
//! fwrite(&RcTime,     8);
//! fwrite(&SymbolData, 216);
//! ```
//!
//! so every record is a fixed **232 bytes** and a valid file satisfies
//! `size % 232 == 0`. The reader consumes `nTot` as a length prefix and
//! then reads `nTot` bytes, of which the first 8 are `RcTime` -- which is
//! why the C++ prints `sz:224 payloadSize:216`.
//!
//! # Struct layout -- NOT packed
//!
//! `FEED_DETAILS` is declared at line 102 of `CME_Data_Check.cpp`, after
//! the `#pragma pack(pop)` at line 85. It therefore uses **natural
//! alignment**, not `#[repr(packed)]`. Offsets below are compiler-
//! verified, not assumed -- note the 2 bytes of padding after the 50-byte
//! `SymbolCode` (to 4-align `Index`) and the 6 bytes after the bitfield
//! (to 8-align `SrcTimestamp`). Reading this as packed silently yields
//! garbage from `Index` onward.
//!
//! # Timestamp units
//!
//! `RcTime` is **nanoseconds** (`clock_gettime(CLOCK_REALTIME)`, taken in
//! `CME_Recorder` immediately after `recvfrom` -- confirmed from that
//! binary's disassembly). `SrcTimestamp` / `DS_rcv_timestamp` /
//! `DS_snd_timestamp` come from the QED library in **microseconds**. Any
//! consumer differencing the two must scale the microsecond value by 1000
//! first; this reader prints all four raw, exactly as the C++ does.
//!
//! # Output fidelity
//!
//! Output is byte-identical to `CME_Data_Check`'s. That is the point of
//! this binary: it exists to prove the Rust struct/framing logic matches
//! the reference C++ before the decode is wired into qtrade proper.
//! Prices are printed raw (integers scaled by 100 -- `9621` is $96.21);
//! no scaling, no conversion, no filtering.
//!
//! # Usage
//!
//! ```text
//! cme-decoder <path-to-cme_feeder_<date>.bin> [max-records]
//! ```
//!
//! `max-records` is the one intentional addition over the C++ (which
//! always prints the whole file -- 103 million lines for a full trading
//! day). Omit it to match the C++ exactly.

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::ExitCode;

const FEED_LEVEL_DEPTH: usize = 5;

/// `sizeof(FEED_DETAILS)` under natural alignment.
const FEED_DETAILS_SIZE: usize = 216;

/// The C++ reads into `unsigned char Buffer[8192]`; a record claiming
/// more than that would overrun it there. Rust won't, so the limit is
/// kept only to reject an absurd length prefix from a corrupt file.
const MAX_RECORD: usize = 8192;

// Compiler-verified field offsets within FEED_DETAILS.
const OFF_SYMBOL_CODE: usize = 0;
const OFF_INDEX: usize = 52;
const OFF_EXPIRY_DATE: usize = 56;
const OFF_BUY_PRICE: usize = 60;
const OFF_BUY_QTY: usize = 80;
const OFF_SELL_PRICE: usize = 100;
const OFF_SELL_QTY: usize = 120;
const OFF_NO_OF_BUY_ORDS: usize = 140;
const OFF_NO_OF_SELL_ORDS: usize = 160;
const OFF_LAST_TRADED_PRICE: usize = 180;
const OFF_BITFIELD: usize = 184;
const OFF_SRC_TIMESTAMP: usize = 192;
const OFF_DS_RCV_TIMESTAMP: usize = 200;
const OFF_DS_SND_TIMESTAMP: usize = 208;

const SYMBOL_CODE_LEN: usize = 50;

fn i32_le(buf: &[u8], off: usize) -> i32 {
    i32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn u32_le(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn u64_le(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
}

fn levels_i32(buf: &[u8], off: usize) -> [i32; FEED_LEVEL_DEPTH] {
    std::array::from_fn(|i| i32_le(buf, off + i * 4))
}

fn levels_u32(buf: &[u8], off: usize) -> [u32; FEED_LEVEL_DEPTH] {
    std::array::from_fn(|i| u32_le(buf, off + i * 4))
}

/// Null-terminated fixed-size char array -> `&str`, matching how the C++
/// passes `ct->SymbolCode` to `printf("%s")` (stops at the first `\0`).
fn c_string(buf: &[u8], off: usize, len: usize) -> String {
    let raw = &buf[off..off + len];
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    String::from_utf8_lossy(&raw[..end]).into_owned()
}

/// One `FEED_DETAILS` record. Every field is parsed even though
/// `print_record` surfaces only the subset the C++ prints -- the whole
/// struct is what downstream qtrade code will consume, and `NoOfBuyOrds`/
/// `NoOfSellOrds`/`LastTradedPrice`/`LastTradedQty`/`FeedEventAggressor`
/// are real data the reference reader simply never displays.
#[allow(dead_code)]
#[derive(Debug)]
struct FeedDetails {
    symbol_code: String,
    index: i32,
    expiry_date: i32,
    buy_price: [i32; FEED_LEVEL_DEPTH],
    buy_qty: [u32; FEED_LEVEL_DEPTH],
    sell_price: [i32; FEED_LEVEL_DEPTH],
    sell_qty: [u32; FEED_LEVEL_DEPTH],
    /// `65535` (`0xFFFF`) is the feed's "not published" sentinel -- CME
    /// supplies order counts only for the top two levels.
    no_of_buy_ords: [i32; FEED_LEVEL_DEPTH],
    no_of_sell_ords: [i32; FEED_LEVEL_DEPTH],
    last_traded_price: i32,
    /// Bits 0-11 of the `uint16_t` bitfield at offset 184.
    last_traded_qty: u16,
    /// Bits 12-13 of the same bitfield.
    feed_event_aggressor: u8,
    /// Microseconds -- see this file's "Timestamp units" note.
    src_timestamp: u64,
    /// Microseconds.
    ds_rcv_timestamp: u64,
    /// Microseconds.
    ds_snd_timestamp: u64,
}

impl FeedDetails {
    /// `buf` must be at least `FEED_DETAILS_SIZE` bytes -- checked by the
    /// caller before this is reached.
    fn read_from(buf: &[u8]) -> Self {
        let bitfield = u16::from_le_bytes(
            buf[OFF_BITFIELD..OFF_BITFIELD + 2].try_into().unwrap(),
        );
        FeedDetails {
            symbol_code: c_string(buf, OFF_SYMBOL_CODE, SYMBOL_CODE_LEN),
            index: i32_le(buf, OFF_INDEX),
            expiry_date: i32_le(buf, OFF_EXPIRY_DATE),
            buy_price: levels_i32(buf, OFF_BUY_PRICE),
            buy_qty: levels_u32(buf, OFF_BUY_QTY),
            sell_price: levels_i32(buf, OFF_SELL_PRICE),
            sell_qty: levels_u32(buf, OFF_SELL_QTY),
            no_of_buy_ords: levels_i32(buf, OFF_NO_OF_BUY_ORDS),
            no_of_sell_ords: levels_i32(buf, OFF_NO_OF_SELL_ORDS),
            last_traded_price: i32_le(buf, OFF_LAST_TRADED_PRICE),
            last_traded_qty: bitfield & 0x0FFF,
            feed_event_aggressor: ((bitfield >> 12) & 0x3) as u8,
            src_timestamp: u64_le(buf, OFF_SRC_TIMESTAMP),
            ds_rcv_timestamp: u64_le(buf, OFF_DS_RCV_TIMESTAMP),
            ds_snd_timestamp: u64_le(buf, OFF_DS_SND_TIMESTAMP),
        }
    }
}

/// Mirrors `CME_Data_Check.cpp` lines 150-161 exactly, including the
/// `----`/`------` dash counts and the `qty-price|price-qty` column order
/// (note the asymmetry: buy side prints qty first, sell side prints price
/// first -- that is what the C++ `printf` does).
fn print_record<W: Write>(
    out: &mut W,
    rc_time: u64,
    n_read: i64,
    payload_size: i64,
    fd: &FeedDetails,
) -> std::io::Result<()> {
    writeln!(out, "RcTime:{rc_time}")?;
    writeln!(out, "sz:{n_read} payloadSize:{payload_size}")?;
    writeln!(out, "----{}------", fd.symbol_code)?;
    writeln!(
        out,
        "{}:{}:{}",
        fd.src_timestamp, fd.ds_rcv_timestamp, fd.ds_snd_timestamp
    )?;
    for k in 0..FEED_LEVEL_DEPTH {
        writeln!(
            out,
            "{}-{}|{}-{}",
            fd.buy_qty[k], fd.buy_price[k], fd.sell_price[k], fd.sell_qty[k]
        )?;
    }
    Ok(())
}

fn run(path: &str, max_records: Option<u64>) -> std::io::Result<()> {
    let mut f = BufReader::with_capacity(1 << 20, File::open(path)?);
    let stdout = std::io::stdout();
    let mut out = BufWriter::with_capacity(1 << 20, stdout.lock());

    let mut packet_count: u64 = 0;
    // `ApplSeqNum` and `Counter` are declared and printed by the C++ but
    // never assigned, so both are always 0. Kept for output fidelity.
    let appl_seq_num: i64 = 0;
    let missed: i64 = 0;

    let mut buf = vec![0u8; MAX_RECORD];

    loop {
        if let Some(limit) = max_records {
            if packet_count >= limit {
                break;
            }
        }

        let mut len_buf = [0u8; 8];
        match f.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                writeln!(out, "EOF reached ApplSeqNum: {appl_seq_num}")?;
                writeln!(out, "Total Packet Count:{packet_count} Missed:{missed}")?;
                break;
            }
            Err(e) => return Err(e),
        }

        let n_read = i64::from_le_bytes(len_buf);
        if n_read < 8 {
            writeln!(out, "Corrupted record found. Exiting.")?;
            break;
        }
        if n_read as usize > MAX_RECORD {
            writeln!(
                out,
                "Corrupted record found (nRead={n_read} exceeds {MAX_RECORD}-byte buffer). Exiting."
            )?;
            break;
        }
        let payload_size = n_read - 8;
        let n_read_usize = n_read as usize;

        match f.read_exact(&mut buf[..n_read_usize]) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // The C++ skips its print block on a short fread and then
                // falls out through the length-prefix read next iteration.
                writeln!(out, "EOF reached ApplSeqNum: {appl_seq_num}")?;
                writeln!(out, "Total Packet Count:{packet_count} Missed:{missed}")?;
                break;
            }
            Err(e) => return Err(e),
        }

        let rc_time = u64_le(&buf, 0);
        let rest = &buf[8..n_read_usize];
        packet_count += 1;

        if rest.len() < FEED_DETAILS_SIZE {
            writeln!(
                out,
                "RcTime:{rc_time}\nsz:{n_read} payloadSize:{payload_size}\n\
                 -- payload too short for FEED_DETAILS: need {FEED_DETAILS_SIZE} bytes, got {}",
                rest.len()
            )?;
            continue;
        }

        let fd = FeedDetails::read_from(rest);
        print_record(&mut out, rc_time, n_read, payload_size, &fd)?;
    }

    out.flush()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: cme-decoder <path-to-cme_feeder_<date>.bin> [max-records]");
        return ExitCode::FAILURE;
    };
    let max_records = match args.get(2).map(|s| s.parse::<u64>()) {
        None => None,
        Some(Ok(n)) => Some(n),
        Some(Err(_)) => {
            eprintln!("error: max-records must be a non-negative integer");
            return ExitCode::FAILURE;
        }
    };

    match run(path, max_records) {
        Ok(()) => ExitCode::SUCCESS,
        // A closed pipe (`| head`) is the normal way to read a few
        // records out of a multi-gigabyte file, not an error.
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error reading {path}: {e}");
            ExitCode::FAILURE
        }
    }
}

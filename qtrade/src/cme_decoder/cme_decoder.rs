//! A faithful Rust port of `references/CME_parsefeed.cpp` -- reads a CME
//! capture file's own record framing and, when the payload is large
//! enough, interprets it as the `CONTRACT_DETAILS`/`GenericContract`
//! struct the original C++ casts directly onto raw bytes.
//!
//! **This is a structural port, not a working decoder.** Two things were
//! confirmed empirically (2026-09-01, this session) before this port was
//! written, and both matter for reading its output honestly:
//!
//! 1. **Real record sizes don't match the struct at all.** Real messages
//!    in `/mnt/CME_Recording_Files/cme_feeder_capture_Tier_1_*.bin` are
//!    only 226 or 25 bytes after their 8-byte `RcTime`. A genuine
//!    `CONTRACT_DETAILS` needs 292 bytes -- confirmed by compiling
//!    `CME_parsefeed.cpp`'s own struct definitions with `g++ -std=c++17`
//!    and reading `sizeof`/`offsetof` directly (the accompanying PDF's
//!    "~282B"/"158B" claims were both slightly wrong; this file's
//!    constants below are the real, compiler-confirmed numbers, not the
//!    document's). No amount of correct field-offset math fixes a
//!    payload that's structurally too short to hold the struct.
//! 2. **The payload itself looks encrypted.** Comparing several real
//!    226-byte messages side by side, almost every byte past the first
//!    ~30 differs between messages with no discernible pattern -- the
//!    signature of encrypted, not decoded, data. This matches
//!    `CME_Feeder.cpp`'s model (`QEDC_is_encrypted_packet` /
//!    `QEDC_decrypt_packet` before any real decode), not this file's
//!    "just read it directly" model.
//!
//! So: running this against real `/mnt/CME_Recording_Files/` data will
//! print "too short to decode" or garbage-looking field values for every
//! record. That is expected, not a bug in this port -- it's the same
//! finding `CME_parsefeed.cpp` itself would produce, just reported
//! honestly instead of silently reading past the end of a buffer (which
//! is exactly what the original C++ does with its raw pointer cast; Rust
//! won't let this port do that by construction). This binary exists to
//! exercise the file-framing logic and stand as a scaffold for the real
//! decrypt-then-decode step ("the feeder part") that comes next.

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

const FEED_LEVEL_DEPTH: usize = 5;

/// Real, compiler-confirmed size of `GenericContract` (packed) --
/// verified via `g++`'s own `sizeof(GenericContract)`. The PDF claimed
/// 158; the real number is 165 (its own per-field offset table actually
/// implied 165 too -- only its summary sentence said 158).
const GENERIC_CONTRACT_SIZE: usize = 165;

/// Real, compiler-confirmed size of `CONTRACT_DETAILS` (unpacked outer
/// struct, packed `GenericContract` nested inside) -- verified via
/// `g++`'s own `sizeof(CONTRACT_DETAILS)`. 126 bytes of levels/last-trade/
/// bitfield, + 165-byte `Contract`, + 1 trailing alignment byte the
/// compiler adds so the struct's size is a multiple of its own 4-byte
/// alignment. The PDF claimed "~282B"; the real number is 292.
const CONTRACT_DETAILS_SIZE: usize = 292;
const CONTRACT_OFFSET: usize = 126;

fn i32_le(buf: &[u8], off: usize) -> i32 {
    i32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn u32_le(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn levels_i32(buf: &[u8], off: usize) -> [i32; FEED_LEVEL_DEPTH] {
    std::array::from_fn(|i| i32_le(buf, off + i * 4))
}

fn levels_u32(buf: &[u8], off: usize) -> [u32; FEED_LEVEL_DEPTH] {
    std::array::from_fn(|i| u32_le(buf, off + i * 4))
}

/// Null-terminated fixed-size char array -> `String`, matching how the
/// original prints `ct->Contract.SymbolCode` via `<<` (stops at the
/// first `\0`, same as a C string).
fn c_string(buf: &[u8], off: usize, len: usize) -> String {
    let raw = &buf[off..off + len];
    let end = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
    String::from_utf8_lossy(&raw[..end]).into_owned()
}

/// `Exchange_Type`/`Instrument_Type`/`Option_Type` are all `int8_t`
/// enums in the C++ -- decoded here as plain `i8` plus a name lookup
/// (`describe_*`), rather than a Rust `enum`, since an out-of-range
/// value (expected constantly, given point 1/2 in this file's own doc
/// comment) must never be a panic -- this is exploratory reading, not a
/// place, that assumes the input is well-formed.
fn describe_exchange(v: i8) -> &'static str {
    match v {
        0 => "EXCHG_CME",
        1 => "EXCHG_NONE",
        _ => "?",
    }
}

fn describe_instrument_type(v: i8) -> &'static str {
    match v {
        0 => "EQUITY", 1 => "INDEX", 2 => "FUTIDX", 3 => "FUTSTK", 4 => "FUTCOM",
        5 => "FUTCUR", 6 => "FUTINT", 7 => "FUTIVX", 8 => "FUTIRC", 9 => "FUTIRD",
        10 => "FUTIRT", 11 => "FUTENR", 12 => "FUTBLN", 13 => "FUTBAS", 14 => "COM",
        15 => "OPTIDX", 16 => "OPTSTK", 17 => "OPTFUT", 18 => "OPTCOM", 19 => "OPTCUR",
        20 => "OPTINT", 21 => "OPTIVX", 22 => "OPTIRC", 23 => "OPTIRD", 24 => "OPTENR",
        25 => "OPTBLN", 26 => "OPTBAS", 27 => "UNDINT", 28 => "UNDCUR", 29 => "UNDIRC",
        30 => "UNDIRT", 31 => "INST_ERROR",
        _ => "?",
    }
}

fn describe_option_type(v: i8) -> &'static str {
    match v {
        0 => "CA", 1 => "PA", 2 => "CE", 3 => "PE", 4 => "OPTION_NONE",
        _ => "?",
    }
}

/// Parsed in full for structural fidelity with the real struct, even
/// though `print_record` below only surfaces a subset -- `#[allow]`
/// rather than deleting fields nothing prints yet, since this is meant
/// as a base for the real decoder, not a finished diagnostic tool.
#[allow(dead_code)]
#[derive(Debug)]
struct GenericContract {
    exchange: i8,
    index: i32,
    token: i32,
    expiry_date: i32,
    strike_price: i32,
    lot_size: i32,
    tick_size: i32,
    low_price_range: i32,
    high_price_range: i32,
    base_price: i32,
    multiplier: i32,
    initial_margin: u32,
    additional_margin: u32,
    regulatory_margin: u32,
    long_margin: u32,
    short_margin: u32,
    symbol: String,
    symbol_code: String,
    instrument_type: i8,
    option_type: i8,
    price_exponent: u8,
    price_exponent_display: u8,
}

impl GenericContract {
    /// `buf` must be at least `GENERIC_CONTRACT_SIZE` bytes -- checked by
    /// every caller in this file before this is reached.
    fn read_from(buf: &[u8]) -> Self {
        GenericContract {
            exchange: buf[0] as i8,
            index: i32_le(buf, 1),
            token: i32_le(buf, 5),
            expiry_date: i32_le(buf, 9),
            strike_price: i32_le(buf, 13),
            lot_size: i32_le(buf, 17),
            tick_size: i32_le(buf, 21),
            low_price_range: i32_le(buf, 25),
            high_price_range: i32_le(buf, 29),
            base_price: i32_le(buf, 33),
            multiplier: i32_le(buf, 37),
            initial_margin: u32_le(buf, 41),
            additional_margin: u32_le(buf, 45),
            regulatory_margin: u32_le(buf, 49),
            long_margin: u32_le(buf, 53),
            short_margin: u32_le(buf, 57),
            symbol: c_string(buf, 61, 50),
            symbol_code: c_string(buf, 111, 50),
            instrument_type: buf[161] as i8,
            option_type: buf[162] as i8,
            price_exponent: buf[163],
            price_exponent_display: buf[164],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct ContractDetails {
    buy_price: [i32; FEED_LEVEL_DEPTH],
    buy_qty: [u32; FEED_LEVEL_DEPTH],
    sell_price: [i32; FEED_LEVEL_DEPTH],
    sell_qty: [u32; FEED_LEVEL_DEPTH],
    no_of_buy_ords: [i32; FEED_LEVEL_DEPTH],
    no_of_sell_ords: [i32; FEED_LEVEL_DEPTH],
    last_traded_price: i32,
    /// Bits 0-11 of the packed bitfield at offset 124-125 -- empirically
    /// confirmed (2026-09-01) against a `g++`-compiled probe of the
    /// original two-bitfield struct, not assumed from a convention.
    last_traded_qty: u16,
    /// Bits 12-13 of the same packed bitfield -- see `last_traded_qty`.
    feed_event_aggressor: u8,
    contract: GenericContract,
}

impl ContractDetails {
    /// `buf` must be at least `CONTRACT_DETAILS_SIZE` bytes -- callers
    /// check this first (real records almost never satisfy it -- see
    /// this file's own module doc comment).
    fn read_from(buf: &[u8]) -> Self {
        let bitfield = u16::from_le_bytes(buf[124..126].try_into().unwrap());
        ContractDetails {
            buy_price: levels_i32(buf, 0),
            buy_qty: levels_u32(buf, 20),
            sell_price: levels_i32(buf, 40),
            sell_qty: levels_u32(buf, 60),
            no_of_buy_ords: levels_i32(buf, 80),
            no_of_sell_ords: levels_i32(buf, 100),
            last_traded_price: i32_le(buf, 120),
            last_traded_qty: bitfield & 0x0FFF,
            feed_event_aggressor: ((bitfield >> 12) & 0x3) as u8,
            contract: GenericContract::read_from(&buf[CONTRACT_OFFSET..CONTRACT_OFFSET + GENERIC_CONTRACT_SIZE]),
        }
    }
}

fn print_record(idx: u64, rc_time: u64, cd: &ContractDetails) {
    println!("---------- record #{idx} (RcTime={rc_time}) ----------");
    println!(
        "symbol_code={:?} symbol={:?} exchange={} instrument_type={} option_type={}",
        cd.contract.symbol_code,
        cd.contract.symbol,
        describe_exchange(cd.contract.exchange),
        describe_instrument_type(cd.contract.instrument_type),
        describe_option_type(cd.contract.option_type),
    );
    println!(
        "token={} expiry_date={} tick_size={} lot_size={}",
        cd.contract.token, cd.contract.expiry_date, cd.contract.tick_size, cd.contract.lot_size
    );
    for k in 0..FEED_LEVEL_DEPTH {
        println!(
            "  {} - {} | {} - {}",
            cd.buy_qty[k], cd.buy_price[k], cd.sell_price[k], cd.sell_qty[k]
        );
    }
    println!(
        "last_traded_price={} last_traded_qty={} feed_event_aggressor={}",
        cd.last_traded_price, cd.last_traded_qty, cd.feed_event_aggressor
    );
}

fn run(path: &str) -> std::io::Result<()> {
    let mut f = File::open(path)?;
    let mut record_count: u64 = 0;
    let mut decoded_count: u64 = 0;
    let mut too_short_count: u64 = 0;

    loop {
        let mut len_buf = [0u8; 8];
        match f.read_exact(&mut len_buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("\nEOF reached.");
                break;
            }
            Err(e) => return Err(e),
        }
        let n_read = i64::from_le_bytes(len_buf);
        if n_read < 8 {
            println!("Corrupted record found (nRead={n_read} < 8). Exiting.");
            break;
        }
        let n_read = n_read as usize;

        let mut payload = vec![0u8; n_read];
        match f.read_exact(&mut payload) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("\nEOF mid-record (wanted {n_read} bytes). Exiting.");
                break;
            }
            Err(e) => return Err(e),
        }

        record_count += 1;
        let rc_time = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let rest = &payload[8..];

        if rest.len() >= CONTRACT_DETAILS_SIZE {
            let cd = ContractDetails::read_from(rest);
            decoded_count += 1;
            if record_count <= 20 || record_count % 10_000 == 0 {
                print_record(record_count, rc_time, &cd);
            }
        } else {
            too_short_count += 1;
            if too_short_count <= 5 {
                println!(
                    "record #{record_count} (RcTime={rc_time}): too short to decode as CONTRACT_DETAILS -- need {CONTRACT_DETAILS_SIZE} bytes, got {} -- first bytes: {}",
                    rest.len(),
                    rest.iter().take(16).map(|b| format!("{b:02x}")).collect::<Vec<_>>().join("")
                );
            }
        }
    }

    println!("\n--- summary ---");
    println!("total records read: {record_count}");
    println!("decoded as CONTRACT_DETAILS (payload >= {CONTRACT_DETAILS_SIZE}B): {decoded_count}");
    println!("too short to decode: {too_short_count}");
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: cme-decoder <path-to-cme-capture-file>");
        return ExitCode::FAILURE;
    };
    match run(path) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error reading {path}: {e}");
            ExitCode::FAILURE
        }
    }
}

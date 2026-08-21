//! The decoder component: everything needed to turn a raw recorded MCX
//! T7 EOBI capture file into readable messages. Wire structs, decode
//! logic, and the framing parser all live here, together, on purpose —
//! see README.md at the project root for what this validated and why.
//!
//! ## Convention established here, for the rest of qtrade
//!
//! Every decoded message is a real typed value (a struct or enum), not a
//! pre-formatted string. Two traits, two different jobs:
//!
//! - `Debug` (`{:?}`) — derived on every type, no exceptions. A full,
//!   mechanical field dump. This is what you want when something looks
//!   wrong and you need to see every raw value, not a summary of them.
//! - `Display` (`{}`) — hand-written, only on types meant to be read by a
//!   person. The one-line human-readable form (`ORDER_ADD Token=... Side=...`).
//!   Never built ad hoc with `format!` scattered through calling code —
//!   the type itself owns its own readable representation.

use std::collections::BTreeMap;
use std::fmt;

/// Corrected after cross-checking against the real contract file: the
/// legacy MCX_Feeder.cpp code divides by 1_000_000, which puts real
/// CRUDEOILM (token 467014) orders at ~540,000 -- implausible (Crude Oil
/// Mini trades in the few-thousand-rupee range). The EOBI spec's own
/// claim of "integer including 8 decimals" checks out empirically
/// instead: dividing by 10^8 puts the same real orders at ~5,400, which
/// matches. Trust the cross-checked number over the legacy code here.
const MCX_PRICE_MULTIPLIER: f64 = 100_000_000.0;
const MCX_QTY_DIVISOR: f64 = 10_000.0;

fn u16_le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32_le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64_le(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}
fn i64_le(b: &[u8], o: usize) -> i64 {
    i64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Buy,
    Sell,
    Unknown(u8),
}

impl Side {
    fn from_raw(raw: u8) -> Self {
        match raw {
            1 => Side::Buy,
            2 => Side::Sell,
            other => Side::Unknown(other),
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
            Side::Unknown(raw) => write!(f, "?({raw})"),
        }
    }
}

/// A price, kept as the raw wire integer -- `Display` is where the
/// human-readable rupee conversion happens, once, rather than being
/// recomputed by every caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Price(i64);

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (~{:.2})", self.0, self.0 as f64 / MCX_PRICE_MULTIPLIER)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Qty(i64);

impl fmt::Display for Qty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (~{:.2})", self.0, self.0 as f64 / MCX_QTY_DIVISOR)
    }
}

// ---------------------------------------------------------------------
// One type per EOBI message we understand. Field offsets in the parsing
// code below are byte offsets from the start of the message (including
// its 8-byte header), taken directly from the #pragma pack(1) struct
// layouts in references/MCX_Feeder.h.
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct PacketHeader {
    seq: u32,
    market_segment_id: u32,
    transact_time: u64,
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} PACKET_HEADER      MarketSegmentID={} TransactTime={}",
            self.seq, self.market_segment_id, self.transact_time
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Heartbeat {
    seq: u32,
    last_seq_no: u64,
}

impl fmt::Display for Heartbeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq={:<10} HEARTBEAT          LastSeqNo={}", self.seq, self.last_seq_no)
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderAdd {
    seq: u32,
    security_id: i64,
    side: Side,
    price: Price,
    qty: Qty,
}

impl fmt::Display for OrderAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} ORDER_ADD          Token={:<10} Side={:<4} Price={} Qty={}",
            self.seq, self.security_id, self.side, self.price, self.qty
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderModify {
    seq: u32,
    security_id: i64,
    side: Side,
    prev_price: Price,
    prev_qty: Qty,
    price: Price,
    qty: Qty,
}

impl fmt::Display for OrderModify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} ORDER_MODIFY       Token={:<10} Side={:<4} Prev[{} x {}] -> New[{} x {}]  [priority LOST]",
            self.seq, self.security_id, self.side, self.prev_price, self.prev_qty, self.price, self.qty
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderModifySamePriority {
    seq: u32,
    security_id: i64,
    side: Side,
    prev_qty: Qty,
    qty: Qty,
    price: Price,
}

impl fmt::Display for OrderModifySamePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} ORDER_MODIFY_SAME  Token={:<10} Side={:<4} PrevQty={} -> NewQty={}  Price={}  [priority KEPT]",
            self.seq, self.security_id, self.side, self.prev_qty, self.qty, self.price
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderDelete {
    seq: u32,
    security_id: i64,
    side: Side,
    price: Price,
    qty: Qty,
}

impl fmt::Display for OrderDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} ORDER_DELETE       Token={:<10} Side={:<4} Price={} Qty={}",
            self.seq, self.security_id, self.side, self.price, self.qty
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderMassDelete {
    seq: u32,
    security_id: i64,
}

impl fmt::Display for OrderMassDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq={:<10} ORDER_MASS_DELETE  Token={}", self.seq, self.security_id)
    }
}

#[derive(Debug, Clone, Copy)]
struct Trade {
    seq: u32,
    full: bool,
    security_id: i64,
    aggressor_side: Side,
    price: Price,
    qty: Qty,
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.full { "TRADE_FULL" } else { "TRADE_PARTIAL" };
        write!(
            f,
            "seq={:<10} {name:<18} Token={:<10} Aggr.Side={:<4} Price={} Qty={}",
            self.seq, self.security_id, self.aggressor_side, self.price, self.qty
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct ExecutionSummary {
    seq: u32,
    security_id: i64,
    aggressor_side: Side,
    price: Price,
    qty: Qty,
}

impl fmt::Display for ExecutionSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} EXECUTION_SUMMARY  Token={:<10} Aggr.Side={:<4} Price={} Qty={}",
            self.seq, self.security_id, self.aggressor_side, self.price, self.qty
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct TopOfBook {
    seq: u32,
    security_id: i64,
    bid_price: Price,
    bid_qty: Qty,
    ask_price: Price,
    ask_qty: Qty,
}

impl fmt::Display for TopOfBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} TOP_OF_BOOK        Token={:<10} Bid={} x {}  Ask={} x {}",
            self.seq, self.security_id, self.bid_price, self.bid_qty, self.ask_price, self.ask_qty
        )
    }
}

/// A template id we saw but have no verified struct layout for (e.g.
/// 13300/13301 -- not defined in references/MCX_Feeder.h). Skipped
/// safely using `body_len`, never guessed at.
#[derive(Debug, Clone, Copy)]
struct UnknownMessage {
    seq: u32,
    template_id: u16,
    body_len: u16,
}

impl fmt::Display for UnknownMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} UNKNOWN template={} ({}B) -- skipped",
            self.seq, self.template_id, self.body_len
        )
    }
}

/// One decoded message, tagged by which type it turned out to be.
/// `Debug` is derived (delegates to each variant's own derived `Debug`);
/// `Display` is hand-written below and delegates to each variant's own
/// hand-written `Display`. Neither impl duplicates the other's job.
#[derive(Debug, Clone, Copy)]
enum DecodedMessage {
    PacketHeader(PacketHeader),
    Heartbeat(Heartbeat),
    OrderAdd(OrderAdd),
    OrderModify(OrderModify),
    OrderModifySamePriority(OrderModifySamePriority),
    OrderDelete(OrderDelete),
    OrderMassDelete(OrderMassDelete),
    Trade(Trade),
    ExecutionSummary(ExecutionSummary),
    TopOfBook(TopOfBook),
    Unknown(UnknownMessage),
}

impl fmt::Display for DecodedMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodedMessage::PacketHeader(m) => write!(f, "{m}"),
            DecodedMessage::Heartbeat(m) => write!(f, "{m}"),
            DecodedMessage::OrderAdd(m) => write!(f, "{m}"),
            DecodedMessage::OrderModify(m) => write!(f, "{m}"),
            DecodedMessage::OrderModifySamePriority(m) => write!(f, "{m}"),
            DecodedMessage::OrderDelete(m) => write!(f, "{m}"),
            DecodedMessage::OrderMassDelete(m) => write!(f, "{m}"),
            DecodedMessage::Trade(m) => write!(f, "{m}"),
            DecodedMessage::ExecutionSummary(m) => write!(f, "{m}"),
            DecodedMessage::TopOfBook(m) => write!(f, "{m}"),
            DecodedMessage::Unknown(m) => write!(f, "{m}"),
        }
    }
}

/// This is the template-id dispatch table. Field offsets are commented
/// with their source; see references/MCX_Feeder.h for the byte-exact
/// struct definitions these were transcribed from.
fn decode_message(template_id: u16, seq: u32, m: &[u8]) -> DecodedMessage {
    let len = m.len();
    match template_id {
        13003 if len >= 32 => DecodedMessage::PacketHeader(PacketHeader {
            seq,
            market_segment_id: u32_le(m, 12),
            transact_time: u64_le(m, 24),
        }),
        13001 => DecodedMessage::Heartbeat(Heartbeat {
            seq,
            last_seq_no: if len >= 16 { u64_le(m, 8) } else { 0 },
        }),
        13100 if len >= 56 => DecodedMessage::OrderAdd(OrderAdd {
            seq,
            security_id: i64_le(m, 16),
            side: Side::from_raw(m[40]),
            price: Price(i64_le(m, 48)),
            qty: Qty(i64_le(m, 32)),
        }),
        13101 if len >= 80 => DecodedMessage::OrderModify(OrderModify {
            seq,
            security_id: i64_le(m, 40),
            side: Side::from_raw(m[64]),
            prev_price: Price(i64_le(m, 24)),
            prev_qty: Qty(i64_le(m, 32)),
            price: Price(i64_le(m, 72)),
            qty: Qty(i64_le(m, 56)),
        }),
        13106 if len >= 72 => DecodedMessage::OrderModifySamePriority(OrderModifySamePriority {
            seq,
            security_id: i64_le(m, 32),
            side: Side::from_raw(m[56]),
            prev_qty: Qty(i64_le(m, 24)),
            qty: Qty(i64_le(m, 48)),
            price: Price(i64_le(m, 64)),
        }),
        13102 if len >= 64 => DecodedMessage::OrderDelete(OrderDelete {
            seq,
            security_id: i64_le(m, 24),
            side: Side::from_raw(m[48]),
            price: Price(i64_le(m, 56)),
            qty: Qty(i64_le(m, 40)),
        }),
        13103 if len >= 24 => DecodedMessage::OrderMassDelete(OrderMassDelete {
            seq,
            security_id: i64_le(m, 8),
        }),
        13104 | 13105 if len >= 56 => DecodedMessage::Trade(Trade {
            seq,
            full: template_id == 13104,
            security_id: i64_le(m, 32),
            aggressor_side: Side::from_raw(m[8]),
            price: Price(i64_le(m, 48)),
            qty: Qty(i64_le(m, 40)),
        }),
        13202 if len >= 80 => DecodedMessage::ExecutionSummary(ExecutionSummary {
            seq,
            security_id: i64_le(m, 8),
            aggressor_side: Side::from_raw(m[48]),
            price: Price(i64_le(m, 56)),
            qty: Qty(i64_le(m, 40)),
        }),
        13504 if len >= 64 => DecodedMessage::TopOfBook(TopOfBook {
            seq,
            security_id: i64_le(m, 16),
            bid_price: Price(i64_le(m, 24)),
            bid_qty: Qty(i64_le(m, 40)),
            ask_price: Price(i64_le(m, 32)),
            ask_qty: Qty(i64_le(m, 48)),
        }),
        _ => DecodedMessage::Unknown(UnknownMessage {
            seq,
            template_id,
            body_len: len as u16,
        }),
    }
}

/// The outer per-record framing, verified empirically against a real file
/// (see README.md for why this needed checking -- the C++ reference
/// code's literal description didn't match what these files actually
/// contain):
///
///   [8 bytes]  u64 LE: length of what follows = 8 (timestamp) + payload_len
///   [8 bytes]  i64 LE: local capture timestamp (monotonic-looking, NOT a
///              wall-clock epoch value -- don't try to print it as a date)
///   [payload_len bytes]: one or more EOBI messages back to back, each
///              starting with a MessageHeader and advancing by body_len
pub struct Summary {
    pub records: usize,
    pub bytes_consumed: usize,
    pub template_counts: BTreeMap<u16, u64>,
}

pub fn decode_file(data: &[u8], skip: usize, max_print: usize, debug: bool) -> Summary {
    let mut pos = 0usize;
    let mut record_no = 0usize;
    let mut template_counts: BTreeMap<u16, u64> = BTreeMap::new();

    while pos + 16 <= data.len() {
        let length = u64_le(data, pos) as usize; // 8 (timestamp) + payload_len
        let ts = i64_le(data, pos + 8);
        if length < 8 {
            eprintln!("bad record at offset {pos}: length field {length} < 8, stopping");
            break;
        }
        let payload_len = length - 8;
        let payload_start = pos + 16;
        let payload_end = payload_start + payload_len;
        if payload_end > data.len() {
            eprintln!(
                "truncated record at offset {pos}: need {payload_len} bytes, only {} available",
                data.len() - payload_start
            );
            break;
        }
        let payload = &data[payload_start..payload_end];
        let printing = record_no >= skip && record_no < skip + max_print;

        if printing {
            println!("[record {record_no}] offset={pos} capture_ts={ts} payload_len={payload_len}");
        }

        let mut off = 0usize;
        while off + 8 <= payload.len() {
            let body_len = u16_le(payload, off) as usize;
            let template_id = u16_le(payload, off + 2);
            let seq = u32_le(payload, off + 4);
            if body_len == 0 || off + body_len > payload.len() {
                if printing {
                    println!("    [off {off}] body_len={body_len} -- stopping (malformed or end)");
                }
                break;
            }
            let msg = &payload[off..off + body_len];
            *template_counts.entry(template_id).or_insert(0) += 1;
            if printing {
                let decoded = decode_message(template_id, seq, msg);
                if debug {
                    println!("    {decoded:?}");
                } else {
                    println!("    {decoded}");
                }
            }
            off += body_len;
        }

        pos = payload_end;
        record_no += 1;
    }

    Summary {
        records: record_no,
        bytes_consumed: pos,
        template_counts,
    }
}

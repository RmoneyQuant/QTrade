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

/// `pub` — `book` (T03) matches on this directly when routing decoded
/// events onto its own `types::Side`. Everything else about this type is
/// unchanged from the original pilot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
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
///
/// `pub` field: `book` needs the untouched raw integer (it does its own
/// tick-band arithmetic on it), not the rupee-scaled `f64` `Display`
/// produces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Price(pub i64);

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (~{:.2})", self.0, self.0 as f64 / MCX_PRICE_MULTIPLIER)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qty(pub i64);

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
pub struct PacketHeader {
    pub seq: u32,
    pub market_segment_id: u32,
    pub transact_time: u64,
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
pub struct Heartbeat {
    pub seq: u32,
    pub last_seq_no: u64,
}

impl fmt::Display for Heartbeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq={:<10} HEARTBEAT          LastSeqNo={}", self.seq, self.last_seq_no)
    }
}

/// `priority_ts` (`TrdRegTSTimePriority`, offset 24 in `references/MCX_Feeder.h`)
/// was added for `book` (T03): MCX has no broadcast order id (FR-B05), so
/// `book` identifies a resting order by `(instrument, side, price,
/// priority_ts)` -- see `types::OrderHandle`. Without this field there is
/// no way to tell apart two orders resting at the same price. Not printed
/// in `Display` (kept as the original one-line summary); visible via
/// `--debug`.
///
/// `event_time` (`TrdRegTSTimeIn`, offset 8) was added later, also for
/// `book`: its FR-B11 validation harness originally aligned the increment
/// and snapshot streams by `PacketHeader.TransactTime` (one timestamp per
/// *packet*), which turned out too coarse -- multiple business events for
/// the same instrument can share one packet's timestamp, and the exact
/// boundary order at a snapshot cutoff was sometimes on the wrong side of
/// it. `event_time` gives each individual event its own real timestamp,
/// precise enough to match against `SnapshotInstrumentSummary.last_update_time`
/// one event at a time. See `book_user_doc.md`.
#[derive(Debug, Clone, Copy)]
pub struct OrderAdd {
    pub seq: u32,
    pub security_id: i64,
    pub side: Side,
    pub price: Price,
    pub qty: Qty,
    pub priority_ts: u64,
    pub event_time: u64,
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

/// `prev_priority_ts`/`priority_ts` added for `book` (T03), same reason
/// as `OrderAdd::priority_ts`: this message replaces the resting order at
/// `(side, prev_price, prev_priority_ts)` with a new one at `(side,
/// price, priority_ts)` -- priority is lost, which is why the identifying
/// timestamp changes too.
#[derive(Debug, Clone, Copy)]
pub struct OrderModify {
    pub seq: u32,
    pub security_id: i64,
    pub side: Side,
    pub prev_price: Price,
    pub prev_qty: Qty,
    pub price: Price,
    pub qty: Qty,
    pub prev_priority_ts: u64,
    pub priority_ts: u64,
    /// `TrdRegTSTimeIn`, offset 8 -- see `OrderAdd::event_time`.
    pub event_time: u64,
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

/// `priority_ts` added for `book` (T03): priority is *kept* here, so this
/// is the same value the resting order was already keyed on -- used to
/// find it in the FIFO without disturbing its position.
#[derive(Debug, Clone, Copy)]
pub struct OrderModifySamePriority {
    pub seq: u32,
    pub security_id: i64,
    pub side: Side,
    pub prev_qty: Qty,
    pub qty: Qty,
    pub price: Price,
    pub priority_ts: u64,
    /// `TrdRegTSTimeIn`, offset 8 -- see `OrderAdd::event_time`.
    pub event_time: u64,
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

/// `priority_ts` added for `book` (T03): identifies exactly which resting
/// order at `(side, price)` this delete removes.
#[derive(Debug, Clone, Copy)]
pub struct OrderDelete {
    pub seq: u32,
    pub security_id: i64,
    pub side: Side,
    pub price: Price,
    pub qty: Qty,
    pub priority_ts: u64,
    /// `TrdRegTSTimeIn`, offset 8 -- see `OrderAdd::event_time`.
    pub event_time: u64,
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
pub struct OrderMassDelete {
    pub seq: u32,
    pub security_id: i64,
    /// `TransactTime`, offset 16 -- see `OrderAdd::event_time`.
    pub event_time: u64,
}

impl fmt::Display for OrderMassDelete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "seq={:<10} ORDER_MASS_DELETE  Token={}", self.seq, self.security_id)
    }
}

/// `book` (T03) finding, checked against `references/MCX_Feeder.cpp`'s
/// own book-mutation logic (~line 1328 on), not assumed from the field
/// name: despite being called `aggressor_side` here (the pilot's original
/// guess), this `Side` identifies the **resting** order's side -- the
/// side of the book a trade removes/reduces quantity from -- not the
/// side of the incoming aggressor. The legacy C++ decrements
/// `BUY_ORDER_PAISA_QUANTITY_INFO` when this field is `1` (Buy) and
/// `SELL_ORDER_PAISA_QUANTITY_INFO` when it is `2` (Sell); if it were
/// really the aggressor's side, a buy aggressor hitting resting sells
/// would need the *sell* side decremented, which is not what the
/// reference code does. `book` treats `13104`/`13105` as book-mutating
/// events for this reason -- they are not purely informational the way
/// `13202` (`ExecutionSummary`, confirmed unused for book state in the
/// same reference file) is. Field name kept as-is (not renamed) to avoid
/// restructuring beyond what T03 asked for; this comment is the
/// correction of record.
#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub seq: u32,
    pub full: bool,
    pub security_id: i64,
    pub aggressor_side: Side,
    pub price: Price,
    pub qty: Qty,
    /// `TransactTime`, offset 24 -- see `OrderAdd::event_time`.
    pub event_time: u64,
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
pub struct ExecutionSummary {
    pub seq: u32,
    pub security_id: i64,
    pub aggressor_side: Side,
    pub price: Price,
    pub qty: Qty,
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
pub struct TopOfBook {
    pub seq: u32,
    pub security_id: i64,
    pub bid_price: Price,
    pub bid_qty: Qty,
    pub ask_price: Price,
    pub ask_qty: Qty,
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

// ---------------------------------------------------------------------
// Snapshot-cycle messages (13600/13601/13602), added for `book` (T03).
// `decoder` previously only counted these template ids without typing
// their fields (see T02_decoder.md); `book`'s FR-B11 validation needs
// them typed to compare against the incrementally-built book. Byte
// offsets are from `references/MCX_Feeder.h`, same convention as every
// other message above. Empirically confirmed against a real capture
// (`mcx_feeder_snapshot_capture_19_01_2026_1_4.bin`): one
// `SnapshotProductSummary` (13600) precedes a run of per-instrument
// blocks; each block is one `SnapshotInstrumentSummary` (13601) followed
// immediately by a `13603` (`InstrumentInfo`, decoded for real below --
// see its own doc comment for the 13203/13603 template-id correction and
// the real DPR/circuit-limit values it carries, now load-bearing for
// `book`'s generic price-band mechanism, not just reference data) and
// then exactly `TotNoOrders` `SnapshotOrder` (13602) records, one per
// currently resting order for that instrument. Verified over 1,060
// consecutive cycles for token 467013 with zero count mismatches between
// `TotNoOrders` and the number of `13602` records actually present.
// ---------------------------------------------------------------------

/// TemplateID 13600. Precedes a run of per-instrument snapshot blocks.
///
/// `last_msg_seq_num_processed` is `LastMsgSeqNumProcessed` in
/// `references/MCX_Feeder.h`. **Investigated, not assumed usable as a
/// cross-stream cursor**: despite the name, its values (observed: 2, 339,
/// ... climbing slowly) do not correspond to the increment stream's own
/// `ApplSeqNum` for the same `MarketSegmentID` at the same wall-clock
/// time (observed in the thousands over the same interval) -- the two
/// are not the same counter. `book`'s snapshot-cycle validation harness
/// aligns the two streams by wall-clock time (`PacketHeader.TransactTime`
/// / `SnapshotInstrumentSummary.last_update_time`, both real epoch
/// nanoseconds) instead. See `book_user_doc.md` for the full account.
#[derive(Debug, Clone, Copy)]
pub struct SnapshotProductSummary {
    pub seq: u32,
    pub last_msg_seq_num_processed: u32,
}

impl fmt::Display for SnapshotProductSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} SNAPSHOT_PRODUCT   LastMsgSeqNumProcessed={}",
            self.seq, self.last_msg_seq_num_processed
        )
    }
}

/// TemplateID 13601. One per instrument per snapshot cycle; opens a block
/// of exactly `tot_no_orders` following `SnapshotOrder` (13602) records.
/// Only the fields `book` actually needs are kept (routing key and the
/// order-count sanity check) -- the rest of the real struct is OHLC/DPR/
/// open-interest reference data (`MDInstrumentEntryGrp`, a repeating
/// group inline in the same message), not order-book state, and isn't
/// touched here to keep this addition minimal.
#[derive(Debug, Clone, Copy)]
pub struct SnapshotInstrumentSummary {
    pub seq: u32,
    pub security_id: i64,
    pub tot_no_orders: u16,
    /// `LastUpdateTime`, offset 16 -- a real epoch-nanosecond timestamp
    /// (confirmed: consistently a few tens to a few hundred ms *before*
    /// the enclosing packet's own `PacketHeader.TransactTime`, exactly
    /// what "book last touched at T, snapshot broadcast shortly after"
    /// would look like). Added for `book`'s FR-B11 harness: the precise
    /// cutoff to align against increment-stream events' own
    /// `event_time` -- see `OrderAdd::event_time` and book_user_doc.md.
    pub last_update_time: u64,
}

impl fmt::Display for SnapshotInstrumentSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} SNAPSHOT_INSTRUMENT Token={:<10} TotNoOrders={}",
            self.seq, self.security_id, self.tot_no_orders
        )
    }
}

/// TemplateID 13602. One resting order line within the current
/// `SnapshotInstrumentSummary` block. **No `security_id` field on the
/// wire** (confirmed against `references/MCX_Feeder.h`'s `SnapshotOrder`
/// struct and against the real capture) -- unlike every other per-order
/// message type, this one inherits its instrument identity from context:
/// whichever `SnapshotInstrumentSummary` most recently preceded it in the
/// stream. Callers that need to know which instrument a `SnapshotOrder`
/// belongs to must track that themselves (`book`'s validation harness
/// does this).
#[derive(Debug, Clone, Copy)]
pub struct SnapshotOrder {
    pub seq: u32,
    pub priority_ts: u64,
    pub qty: Qty,
    pub side: Side,
    pub price: Price,
}

impl fmt::Display for SnapshotOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} SNAPSHOT_ORDER     Side={:<4} Price={} Qty={} PriorityTs={}",
            self.seq, self.side, self.price, self.qty, self.priority_ts
        )
    }
}

/// TemplateID **13603** (real, on-the-wire, verified against actual bytes
/// -- see the correction below). Published whenever an instrument's daily
/// price range (circuit limit) is set or revised; `book` (T03) uses this
/// to size its dense per-instrument array generically, in place of a
/// hardcoded per-id band -- see `book_user_doc.md`.
///
/// **A real discrepancy in MCX's own reference material, resolved by
/// checking actual bytes rather than trusting either source blindly:**
/// `references/MCX_Feeder.h` defines this exact struct (`SecurityID`,
/// `ClosePrice`, `PrevClosePrice`, `UpperCktLimit`, `LowerCktLimit`,
/// field-for-field identical to MCX's own EOBI spec document, circular
/// MCX/CTCL/057/2024 §4.19) but labels it `//TemplateID : 13203` --
/// while the EOBI spec's own §4.19 section heading and its master
/// message-type table both say **13603**, and only that section's own
/// field-table row contradicts itself with a leftover "Value: 13203
/// (MarketDataTrade, MsgType = U22)" description line that doesn't even
/// match this message's own name. Checked against real bytes rather than
/// guessed: a template-id scan of the real `19_01_2026` CRUDEOIL snapshot
/// capture (`mcx_feeder_snapshot_capture_19_01_2026_1_4.bin`, 3,140,083
/// outer records, streamed in full) found **zero** messages tagged
/// `13203` anywhere in the file, and 2,848,887 messages tagged `13603`
/// with exactly this struct's 48-byte shape -- decoding, for CRUDEOIL
/// (467013), to `UpperDailyPriceLimit`=5,666.00 / `LowerDailyPriceLimit`=
/// 5,232.00, matching `book_user_doc.md`'s already-documented, independen
/// -tly-derived real DPR bounds for this instrument exactly. **13603 is
/// the real wire id; `MCX_Feeder.h`'s own `13203` comment and the spec's
/// own field-table description line are both wrong for this message.**
///
/// Field layout (offsets from the start of the message, i.e. including
/// the 8-byte `MessageHeader`, same convention as every other message in
/// this file): `SecurityID` i64 @8, `ClosePrice` i64 @16, `PrevClosePrice`
/// i64 @24, `UpperDailyPriceLimit` (`UpperCktLimit`) i64 @32,
/// `LowerDailyPriceLimit` (`LowerCktLimit`) i64 @40. Total body length 48
/// bytes -- confirmed exactly (no `SnapshotOrder`-style surprise here;
/// every real occurrence observed carries `body_len == 48`).
///
/// **A real, empirically-found corruption to guard against, not a
/// hypothetical:** this message also appears once, right at the very tail
/// of both real increment capture files (CRUDEOIL: record 56,600,528 of
/// 56,602,508; NATURALGAS: record 242,298,182 of 242,321,672 -- the last
/// handful of records in each multi-gigabyte file), evidently a batch
/// End-of-Day rebroadcast for the *next* trading day's reference prices:
/// `ClosePrice` is a real number but `PrevClosePrice`/`UpperDailyPriceLimit`
/// /`LowerDailyPriceLimit` are all exactly `i64::MIN`
/// (`-9223372036854775808`) -- a sentinel for "not yet computed," not a
/// real price. `book`'s consumer of this type validates
/// `lower < upper` (and rejects `i64::MIN`) before trusting a band from
/// it, exactly this project's established "never blindly trust a wire
/// value" discipline (see decoder's own price-multiplier and framing
/// corrections, and `book`'s own trade-matching and snapshot-size fixes).
#[derive(Debug, Clone, Copy)]
pub struct InstrumentInfo {
    pub seq: u32,
    pub security_id: i64,
    pub close_price: Price,
    pub prev_close_price: Price,
    pub upper_daily_price_limit: Price,
    pub lower_daily_price_limit: Price,
}

impl fmt::Display for InstrumentInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "seq={:<10} INSTRUMENT_INFO    Token={:<10} Upper={} Lower={} Close={} PrevClose={}",
            self.seq, self.security_id, self.upper_daily_price_limit, self.lower_daily_price_limit, self.close_price, self.prev_close_price
        )
    }
}

/// A template id we saw but have no verified struct layout for (e.g.
/// 13300/13301 -- not defined in references/MCX_Feeder.h). Skipped
/// safely using `body_len`, never guessed at.
#[derive(Debug, Clone, Copy)]
pub struct UnknownMessage {
    pub seq: u32,
    pub template_id: u16,
    pub body_len: u16,
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
///
/// `pub` (enum and every variant): `book` (T03) matches on this directly
/// to route decoded events into its per-instrument books -- see
/// `T02_decoder.md`'s follow-up note.
#[derive(Debug, Clone, Copy)]
pub enum DecodedMessage {
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
    SnapshotProductSummary(SnapshotProductSummary),
    SnapshotInstrumentSummary(SnapshotInstrumentSummary),
    SnapshotOrder(SnapshotOrder),
    InstrumentInfo(InstrumentInfo),
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
            DecodedMessage::SnapshotProductSummary(m) => write!(f, "{m}"),
            DecodedMessage::SnapshotInstrumentSummary(m) => write!(f, "{m}"),
            DecodedMessage::SnapshotOrder(m) => write!(f, "{m}"),
            DecodedMessage::InstrumentInfo(m) => write!(f, "{m}"),
            DecodedMessage::Unknown(m) => write!(f, "{m}"),
        }
    }
}

/// This is the template-id dispatch table. Field offsets are commented
/// with their source; see references/MCX_Feeder.h for the byte-exact
/// struct definitions these were transcribed from.
/// `pub`: every real consumer (`book`'s streaming validation harness,
/// `feed_replay::replay`, `decode_messages` above) reads outer/inner
/// framing itself and calls this per-message dispatch directly once it
/// has sliced out one message's bytes -- this is the one real decoding
/// entry point, shared by every mode.
pub fn decode_message(template_id: u16, seq: u32, m: &[u8]) -> DecodedMessage {
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
            priority_ts: u64_le(m, 24),
            event_time: u64_le(m, 8),
        }),
        13101 if len >= 80 => DecodedMessage::OrderModify(OrderModify {
            seq,
            security_id: i64_le(m, 40),
            side: Side::from_raw(m[64]),
            prev_price: Price(i64_le(m, 24)),
            prev_qty: Qty(i64_le(m, 32)),
            price: Price(i64_le(m, 72)),
            qty: Qty(i64_le(m, 56)),
            prev_priority_ts: u64_le(m, 16),
            priority_ts: u64_le(m, 48),
            event_time: u64_le(m, 8),
        }),
        13106 if len >= 72 => DecodedMessage::OrderModifySamePriority(OrderModifySamePriority {
            seq,
            security_id: i64_le(m, 32),
            side: Side::from_raw(m[56]),
            prev_qty: Qty(i64_le(m, 24)),
            qty: Qty(i64_le(m, 48)),
            price: Price(i64_le(m, 64)),
            priority_ts: u64_le(m, 40),
            event_time: u64_le(m, 8),
        }),
        13102 if len >= 64 => DecodedMessage::OrderDelete(OrderDelete {
            seq,
            security_id: i64_le(m, 24),
            side: Side::from_raw(m[48]),
            price: Price(i64_le(m, 56)),
            qty: Qty(i64_le(m, 40)),
            priority_ts: u64_le(m, 32),
            event_time: u64_le(m, 8),
        }),
        13103 if len >= 24 => DecodedMessage::OrderMassDelete(OrderMassDelete {
            seq,
            security_id: i64_le(m, 8),
            event_time: u64_le(m, 16),
        }),
        13104 | 13105 if len >= 56 => DecodedMessage::Trade(Trade {
            seq,
            full: template_id == 13104,
            security_id: i64_le(m, 32),
            aggressor_side: Side::from_raw(m[8]),
            price: Price(i64_le(m, 48)),
            qty: Qty(i64_le(m, 40)),
            event_time: u64_le(m, 24),
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
        13600 if len >= 24 => DecodedMessage::SnapshotProductSummary(SnapshotProductSummary {
            seq,
            last_msg_seq_num_processed: u32_le(m, 8),
        }),
        13601 if len >= 48 => {
            DecodedMessage::SnapshotInstrumentSummary(SnapshotInstrumentSummary {
                seq,
                security_id: i64_le(m, 8),
                tot_no_orders: u16_le(m, 32),
                last_update_time: u64_le(m, 16),
            })
        }
        13602 if len >= 40 => DecodedMessage::SnapshotOrder(SnapshotOrder {
            seq,
            priority_ts: u64_le(m, 8),
            qty: Qty(i64_le(m, 16)),
            side: Side::from_raw(m[24]),
            price: Price(i64_le(m, 32)),
        }),
        13603 if len >= 48 => DecodedMessage::InstrumentInfo(InstrumentInfo {
            seq,
            security_id: i64_le(m, 8),
            close_price: Price(i64_le(m, 16)),
            prev_close_price: Price(i64_le(m, 24)),
            upper_daily_price_limit: Price(i64_le(m, 32)),
            lower_daily_price_limit: Price(i64_le(m, 40)),
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
///
/// The in-memory streaming API T02's follow-up asked for: decode every
/// message in an already-loaded buffer, lazily, without printing. This
/// is the entry point `book` (T03) builds its books from when it has (or
/// wants) the whole buffer in memory; `book`'s own FR-B11 validation
/// harness, working against files far bigger than comfortably fits in
/// RAM, instead streams records directly off disk and calls
/// `decode_message` per message itself -- see `book_user_doc.md`.
pub fn decode_messages(data: &[u8]) -> impl Iterator<Item = DecodedMessage> + '_ {
    let mut pos = 0usize;
    // Outer iterator: one item per outer (packet) record, yielding the
    // byte range of its payload and advancing `pos` past it, per the
    // framing described above.
    std::iter::from_fn(move || {
        if pos + 16 > data.len() {
            return None;
        }
        let length = u64_le(data, pos) as usize;
        if length < 8 {
            return None;
        }
        let payload_len = length - 8;
        let payload_start = pos + 16;
        let payload_end = payload_start + payload_len;
        if payload_end > data.len() {
            return None;
        }
        pos = payload_end;
        Some(&data[payload_start..payload_end])
    })
    // Inner iterator: one item per EOBI message inside that payload.
    .flat_map(|payload: &[u8]| {
        let mut off = 0usize;
        std::iter::from_fn(move || {
            if off + 8 > payload.len() {
                return None;
            }
            let body_len = u16_le(payload, off) as usize;
            let template_id = u16_le(payload, off + 2);
            let seq = u32_le(payload, off + 4);
            if body_len == 0 || off + body_len > payload.len() {
                return None;
            }
            let msg = &payload[off..off + body_len];
            off += body_len;
            Some(decode_message(template_id, seq, msg))
        })
    })
}


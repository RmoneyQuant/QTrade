//! Shared vocabulary: value types more than one component needs, defined
//! once here instead of each component inventing its own. See
//! `types_user_doc.md` for what each type is for and which component
//! introduced the need for it.

use std::fmt;

/// A price, in ticks -- never `f64`. See STRATEGY-GUIDE.md §11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(pub i64);

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Raw wire price / this = rupees -- the same wire-raw scale `decoder`,
/// `book`, `simulator`, and `execution` all mean by `Price`. Defined here
/// (not just in `execution.rs`, which has its own copy as
/// `RAW_PRICE_SCALE`) because `ContractFilePaise::to_wire_price` below
/// needs it too, and duplicating a business-meaning-free wire constant
/// across two files is harmless (unlike `decoder`'s own copy, which is
/// out of scope to touch -- see `RAW_QTY_PER_LOT`'s doc comment).
pub const WIRE_PRICE_PER_RUPEE: i64 = 100_000_000;

/// A price-like value exactly as it appears in MCXScrips.bcp's own
/// columns (e.g. `TickSize`, `parts[21]`) -- confirmed empirically to be
/// denominated in **paise** (hundredths of a rupee), NOT qtrade's
/// internal wire-raw `Price` scale. See `refdata_user_doc.md`'s "TickSize
/// units" section for the evidence: MCXScrips.bcp's raw `TickSize` column
/// reads `100` for every CRUDEOIL-family future and `10` for every
/// NATURALGAS-family future, and separately `100` for GOLD/SILVER and `5`
/// for COPPER/ALUMINIUM/ZINC/LEAD -- each of which, read as paise (/100
/// -> rupees), reproduces that commodity's real, publicly documented MCX
/// tick size (Rs 1.00, Rs 1.00, Rs 1.00, Rs 0.05 respectively) exactly.
///
/// This type exists so `refdata` cannot hand out a bare `Price` built
/// directly from the column's raw integer without going through
/// `to_wire_price()` first -- the bug this type exists to prevent already
/// happened once: `refdata::load_mcx_instruments` used to put the raw
/// column value straight into `Instrument.tick_size: Price`, six orders
/// of magnitude off `book.rs`'s own already-validated real value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractFilePaise(pub i64);

impl ContractFilePaise {
    /// paise -> rupees is `/100`; rupees -> wire-raw is `*
    /// WIRE_PRICE_PER_RUPEE` (`*10^8`). Combined: paise -> wire-raw is
    /// `* (WIRE_PRICE_PER_RUPEE / 100)` = `* 1_000_000`. Written as one
    /// multiply (not a divide-then-multiply) since `WIRE_PRICE_PER_RUPEE`
    /// is exactly divisible by 100, so this is exact integer arithmetic,
    /// not a lossy round-trip through a fraction.
    pub fn to_wire_price(self) -> Price {
        Price(self.0 * (WIRE_PRICE_PER_RUPEE / 100))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Qty(pub i64);

impl fmt::Display for Qty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Raw wire quantity units per lot. Matches `decoder`'s own private
/// `MCX_QTY_DIVISOR` constant (`decoder/user_doc.md`'s "qty fields"
/// section) and is independently confirmed by `simulator/validate.rs`'s
/// own real-data test, which submits `Qty(10_000)` to mean exactly one
/// lot against a real CRUDEOIL book. `decoder.rs` is out of scope for
/// this change (see the task that introduced this constant), so its own
/// constant is intentionally left as an independently-maintained literal
/// that happens to agree -- this one exists for the two new types below,
/// `Lots`/`Qty`'s own conversion, not to unify with `decoder`'s.
pub const RAW_QTY_PER_LOT: i64 = 10_000;

/// A quantity in plain lot count -- what a strategy or a human means by
/// "1 lot," and what `execution`'s cost model and reporting need. Never
/// used for order-book matching directly; see `Qty` (this module) for
/// the wire-raw scale `decoder`/`book`/`simulator` actually match orders
/// in. Exists because `execution::NewOrderIntent.qty` and
/// `execution::CostModel::round_trip`'s `qty` parameter were once both
/// typed as plain `Qty` and silently disagreed by exactly
/// `RAW_QTY_PER_LOT` on which scale that `Qty` was in -- see
/// `execution_user_doc.md` and `dummy_strategy.md` for the real
/// inflated-cost output that bug produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lots(pub i64);

impl fmt::Display for Lots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Qty {
    /// Wire-raw units -> plain lot count. Integer division: a `Qty` that
    /// isn't an exact multiple of `RAW_QTY_PER_LOT` truncates -- every
    /// real quantity this codebase produces (real order sizes, real
    /// trade quantities) is in fact an exact multiple, so this is not
    /// expected to lose information in practice, only to be simple.
    pub fn to_lots(self) -> Lots {
        Lots(self.0 / RAW_QTY_PER_LOT)
    }
}

impl Lots {
    /// Plain lot count -> wire-raw units, the scale `decoder`/`book`/
    /// `simulator` actually match orders in.
    pub fn to_raw_qty(self) -> Qty {
        Qty(self.0 * RAW_QTY_PER_LOT)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "BUY"),
            Side::Sell => write!(f, "SELL"),
        }
    }
}

/// The venue's own native token (MCX `SecurityID`), cast directly. FR-B02
/// originally called for a separately-interned dense counter instead
/// (see git history / `refdata_user_doc.md`'s "on `InstrumentId`
/// unification" section) -- that was tried, and produced a real bug: two
/// different `InstrumentId` numbering schemes existed side by side
/// (`refdata`'s own dense counter vs. every other component's "raw token,
/// cast directly"), requiring a manual translation wherever both were
/// needed together. Unified on the raw-token convention since it was
/// already what `book`/`cache`/`simulator`/`execution` all independently
/// settled on. Not stable across trading days (FR-16) -- MCX reassigns
/// tokens daily, so an `InstrumentId` from one day's `refdata` load means
/// nothing against a different day's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstrumentId(pub u32);

/// More venues later, not now -- keep this open at the call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Venue {
    Mcx,
}

impl fmt::Display for Venue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Venue::Mcx => write!(f, "MCX"),
        }
    }
}

// Supporting types InstrumentKind/Instrument reference below. Kept
// deliberately simple -- these are data carriers, not behavior.

/// Days-since-epoch. Widen only if a real need appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    pub year: i32,
    pub month: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Settlement {
    Cash,
    Physical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Right {
    Call,
    Put,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exercise {
    European,
    American,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Inr,
}

/// D37 -- `Future` implemented, the rest are stubs until a real need
/// arrives for them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentKind {
    Future {
        underlying: String,
        expiry: Date,
        contract_month: YearMonth,
        settlement: Settlement,
    },
    Option {
        underlying: String,
        expiry: Date,
        strike: Price,
        right: Right,
        exercise: Exercise,
        settlement: Settlement,
    },
    Equity {
        series: String,
    },
    Spread {
        leg1: InstrumentId,
        leg2: InstrumentId,
    },
}

/// FR-B01, verbatim shape from BACKTEST-PHASE1.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    pub id: InstrumentId,
    pub venue: Venue,
    pub native_id: i64,
    pub kind: InstrumentKind,
    pub tick_size: Price,
    pub lot_size: i64,
    pub multiplier: i64,
    pub freeze_qty: i64,
    pub price_band: Option<(Price, Price)>,
    pub currency: Currency,
}

/// FR-B10, verbatim. Only `Uninit`/`Ok` are reachable before a live
/// Transport exists to drive `Recovering`/`Stale`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookState {
    Uninit,
    Recovering,
    Ok,
    Stale,
}

/// FR-B05 -- MCX has no broadcast order id, so a resting order is
/// identified by where it sits, not by an exchange-assigned integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderHandle {
    pub instrument: InstrumentId,
    pub side: Side,
    pub price: Price,
    pub priority_ts: u64,
}

/// Used by `book`'s `Book` trait (best_bid/best_ask/depth). BACKTEST-PHASE1.md's
/// own FR-B08 code uses this type but never defines it -- defined here
/// since nothing else does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceLevel {
    pub price: Price,
    pub qty: Qty,
    pub order_count: u32,
}

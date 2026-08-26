//! The execution component: order lifecycle, two-level accounting,
//! transaction costs, and reporting -- BACKTEST-PHASE1.md §M7
//! (FR-B26 through FR-B31), ARCHITECTURE-DECISIONS.md D08/D19/D23/D34/
//! D36/D40, STRATEGY-GUIDE.md §7a. See `execution_user_doc.md` in this
//! folder for the full account: why the two rejection paths are
//! genuinely different mechanisms, why the client order ID scheme can't
//! use wall-clock, the two accounting levels, the cost model's
//! direction-asymmetry, and the report tiers.
//!
//! ## What this component owns
//!
//! - The eleven-state order machine (§1) and its transition rules,
//!   including `PendingCancel -> Filled` (a real race, not an edge case)
//!   and `Denied` (local, never left qtrade) vs `Rejected` (the venue's
//!   own refusal) as genuinely distinct terminal states.
//! - Three gates -- Validation, RMS, a local OTR/rate governor -- run in
//!   that order before anything reaches the venue (§3). Local rejections
//!   return synchronously; a venue response only ever arrives as the
//!   `Vec<ExecReport>` `SimExchange` itself hands back.
//! - `ClOrdIdGen` (§2): `(session_id, counter)`, session id injected,
//!   counter monotonic -- never a wall-clock read (D40).
//! - `Portfolio` (§5): per-strategy sub-accounts plus a firm aggregate
//!   computed by netting them, never independently mutable (D08).
//! - `CostModel` (§4): direction-asymmetric transaction costs, queryable
//!   before a strategy quotes and applied identically to real fills
//!   (D23).
//! - Reporting (§7): a Tier 1 summary (always on) and Tier 2 per-fill/
//!   per-event detail (switchable), with queue position and markout
//!   horizons present on every fill record from the moment it is created
//!   (D26). Tier 3 (strategy-published series) is out of scope -- no
//!   `Strategy` trait exists yet to publish from.
//!
//! ## Scope (deliberately out, per the task brief)
//!
//! Margin and cash (a later real RMS's own work, D34's own deferral). A
//! full `Strategy` trait / strategy-authoring API. D22's full config-file
//! infrastructure -- the run identity hash here is a placeholder scheme,
//! explicitly permitted by this milestone's acceptance bar.
//!
//! ## Convention carried over from `simulator`/`cache`/`scheduler`
//!
//! `Debug` derived everywhere. `Display` hand-written on the types a
//! person actually reads: `OrderState`, `DenyReason`, `CancelReason`,
//! and the Tier 1 report itself.

use crate::decoder::DecodedMessage;
use crate::simulator::{self, CancelReason as VenueCancelReason, ExecReport, FillKind, NewOrderRequest, OrderType, OtrConfig, RejectReason, SimExchange};
use crate::types::{Instrument, InstrumentId, Lots, Price, Qty, Side};

use std::collections::{HashMap, VecDeque};
use std::fmt;

/// Raw wire price / this = rupees. The same constant `decoder`'s own
/// documentation establishes (`decoder/user_doc.md` §"price fields":
/// `MCX_PRICE_MULTIPLIER = 100_000_000.0`) -- a fact about how MCX's ETI
/// wire format encodes price, not a business rate, so unlike the cost
/// stack's rates (D23: config, not literals) this is a decoding constant
/// in the same spirit as `simulator`'s own `SIM_ID_BASE`.
pub const RAW_PRICE_SCALE: f64 = 100_000_000.0;

fn rupees(price: Price) -> f64 {
    price.0 as f64 / RAW_PRICE_SCALE
}

// =======================================================================
// 1. Order state machine -- FR-B26, STRATEGY-GUIDE.md §7a, eleven states.
// =======================================================================

/// The eleven states, verbatim from STRATEGY-GUIDE.md §7a. Nautilus's
/// fifteen minus `EMULATED`/`RELEASED` (no order emulator), `TRIGGERED`
/// (no stop orders, D12) and `VOIDED` (no contingent orders) -- the same
/// omissions the guide itself documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    /// Record created, gates not yet run.
    Initialized,
    /// A local gate rejected it. It never left qtrade. **Terminal.**
    Denied,
    /// Passed the gates, in flight to the venue.
    Submitted,
    /// Venue acknowledged, resting in the book.
    Accepted,
    /// Venue refused it. **Terminal.**
    Rejected,
    /// Some quantity filled, remainder working.
    PartiallyFilled,
    /// Fully filled. **Terminal.**
    Filled,
    /// Modify sent, awaiting venue response.
    PendingUpdate,
    /// Cancel sent, awaiting venue response. Still exposed -- `is_open()`
    /// includes this state on purpose (STRATEGY-GUIDE.md §7a).
    PendingCancel,
    /// Removed from the book. **Terminal.**
    Canceled,
    /// Removed by time -- end of day for Lean, or GTD reached. **Terminal.**
    Expired,
}

impl OrderState {
    /// "Do I still have a quote in the market?" Includes `PendingCancel`
    /// deliberately: until the venue confirms, you are still exposed --
    /// exactly the reasoning behind the `PendingCancel -> Filled` race.
    pub fn is_open(self) -> bool {
        matches!(
            self,
            OrderState::Accepted | OrderState::PartiallyFilled | OrderState::PendingUpdate | OrderState::PendingCancel
        )
    }

    /// A message is out, awaiting a venue response.
    pub fn is_inflight(self) -> bool {
        matches!(self, OrderState::Submitted | OrderState::PendingUpdate | OrderState::PendingCancel)
    }

    /// Nothing further can happen to this order.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            OrderState::Denied | OrderState::Rejected | OrderState::Filled | OrderState::Canceled | OrderState::Expired
        )
    }
}

impl fmt::Display for OrderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderState::Initialized => "INITIALIZED",
            OrderState::Denied => "DENIED",
            OrderState::Submitted => "SUBMITTED",
            OrderState::Accepted => "ACCEPTED",
            OrderState::Rejected => "REJECTED",
            OrderState::PartiallyFilled => "PARTIALLY_FILLED",
            OrderState::Filled => "FILLED",
            OrderState::PendingUpdate => "PENDING_UPDATE",
            OrderState::PendingCancel => "PENDING_CANCEL",
            OrderState::Canceled => "CANCELED",
            OrderState::Expired => "EXPIRED",
        };
        write!(f, "{s}")
    }
}

/// Why a **local** gate refused an order (FR-B27). Genuinely distinct
/// from `simulator::RejectReason` -- that one is the venue's own refusal
/// and arrives via `ExecReport`; this one is decided in-process, in
/// nanoseconds, before anything is sent (D36).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyReason {
    /// Order Validation (D17): price is not a multiple of the
    /// instrument's tick size.
    TickSize,
    /// Order Validation (D17): quantity exceeds the instrument's freeze
    /// quantity (or is non-positive).
    FreezeQty,
    /// RMS (D34) said no. Phase 1's RMS always says yes, so this variant
    /// is unreachable today but must exist so a real RMS slots in later
    /// without changing this enum's shape.
    RmsRejected,
    /// The **engine's own** OTR/message-rate governor, independent of
    /// whatever `SimExchange`'s internal governor decides (D19: "they do
    /// not share state, preserving the venue independence of D10"). Not
    /// to be confused with `simulator::RejectReason::OtrOrRateExceeded`,
    /// which is a venue rejection, not a local one.
    LocalOtrOrRate,
    /// The instrument isn't in this engine's registry -- a fact knowable
    /// locally, with no need to pay a round trip to learn it.
    UnknownInstrument,
}

impl fmt::Display for DenyReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DenyReason::TickSize => "TICK_SIZE",
            DenyReason::FreezeQty => "FREEZE_QTY",
            DenyReason::RmsRejected => "RMS_REJECTED",
            DenyReason::LocalOtrOrRate => "LOCAL_OTR_OR_RATE",
            DenyReason::UnknownInstrument => "UNKNOWN_INSTRUMENT",
        };
        write!(f, "{s}")
    }
}

/// `Canceled` is one state with several causes (STRATEGY-GUIDE.md §7a).
/// A superset of what `simulator::CancelReason` can actually produce
/// today (`Explicit`/`IocRemainder`/`MassDelete`, mapped in via
/// `map_cancel_reason` below) plus the reasons the guide documents that
/// nothing in this codebase drives yet -- `Watchdog` (D28, no
/// declared-dependency staleness detector built), `Mmp` (no
/// market-maker-protection model in `SimExchange`), `SessionLoss`/
/// `EndOfDay` (no session-state component yet), `Risk` (no real RMS that
/// pulls orders, D34's own deferral). Kept in the enum now, unreachable
/// today, for the same "call site exists, later work slots in" reason
/// D34 gives for the RMS trait itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    Strategy,
    IocRemainder,
    MassCancel,
    Watchdog,
    Mmp,
    SessionLoss,
    EndOfDay,
    Risk,
}

fn map_cancel_reason(venue: VenueCancelReason) -> CancelReason {
    match venue {
        VenueCancelReason::Explicit => CancelReason::Strategy,
        VenueCancelReason::IocRemainder => CancelReason::IocRemainder,
        VenueCancelReason::MassDelete => CancelReason::MassCancel,
    }
}

impl fmt::Display for CancelReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub type StrategyId = u32;

/// Everything reporting and accounting need about one order, across its
/// whole life. Fully `pub` fields, same transparency convention
/// `simulator::AuditLog` uses -- this is a record meant to be inspected,
/// not an encapsulated object.
#[derive(Debug, Clone)]
pub struct Order {
    pub client_order_id: u64,
    pub strategy_id: StrategyId,
    pub instrument: InstrumentId,
    pub side: Side,
    pub order_type: OrderType,
    pub requested_qty: Qty,
    pub state: OrderState,
    pub filled_qty: Qty,
    pub leaves_qty: Qty,
    pub working_price: Option<Price>,
    pub deny_reason: Option<DenyReason>,
    pub reject_reason: Option<RejectReason>,
    pub cancel_reason: Option<CancelReason>,
    /// Set once, when the order first rests: did it price better than
    /// the then-prevailing best on its own side (tightening the market)
    /// rather than joining/crossing it? A phase-1 heuristic -- see
    /// execution_user_doc.md's reporting section.
    pub spread_improving: bool,
    pre_submit_best_same_side: Option<Price>,
}

// =======================================================================
// 2. Client order ID -- FR-B28, D40.
// =======================================================================

/// `ClOrdId = (session_id, counter)`. `session_id` is injected at
/// construction -- a deterministic value from `[run]` config in a
/// backtest, process-start time in live (D40) -- **never read from a
/// wall clock or any OS-provided randomness here**: doing so would mean
/// two runs of the identical backtest produce different IDs, breaking
/// FR-12's byte-identical-output requirement. `counter` is monotonic and
/// is why two orders submitted inside the same scheduler callback
/// (identical `SimClock::now()`, since the clock does not advance within
/// one dispatch) still get distinct IDs.
///
/// Packed into one `u64`: `session_id` in the upper 24 bits, `counter` in
/// the lower 40 -- the split D40 itself proposes pending verification of
/// MCX ETI's real `ClOrdID` field width, which is not needed to satisfy
/// this milestone's own gate.
#[derive(Debug, Clone, Copy)]
pub struct ClOrdIdGen {
    session_id: u64,
    counter: u64,
}

const COUNTER_BITS: u32 = 40;
const COUNTER_MASK: u64 = (1u64 << COUNTER_BITS) - 1;

impl ClOrdIdGen {
    /// `session_id` is masked to 24 bits (matching the split above) --
    /// silently, not by panicking, since a session id derived from e.g. a
    /// run-config hash is expected to be wider than 24 bits and only the
    /// low bits are needed for uniqueness within one process's lifetime.
    pub fn new(session_id: u32) -> Self {
        ClOrdIdGen { session_id: (session_id as u64) & 0xFF_FFFF, counter: 0 }
    }

    /// Assigns and returns the next id. Never consults a clock -- this is
    /// the entire reason `counter` exists rather than deriving uniqueness
    /// from `now_ns` (D40).
    pub fn next(&mut self) -> u64 {
        let id = (self.session_id << COUNTER_BITS) | (self.counter & COUNTER_MASK);
        self.counter += 1;
        id
    }
}

// =======================================================================
// 3. Gates -- FR-B27, D36: Validation -> RMS -> local OTR governor.
// =======================================================================

/// What a strategy asks the engine to do. Deliberately thin: the engine,
/// not the caller, is responsible for assigning the client order id and
/// creating the record (FR-B27: "before the gates").
///
/// `qty` is `Lots` -- plain lot count, what a strategy actually means by
/// "1 lot" -- **not** `simulator`/`decoder`/`book`'s wire-raw `Qty`. This
/// used to be a bare `Qty`, which silently meant two different scales
/// depending which of its two consumers you asked: `simulator` (which
/// needs wire-raw for real order matching) and `CostModel::round_trip`
/// (which was written and tested assuming a small literal lot count).
/// `submit_order` converts to wire-raw exactly once, explicitly, via
/// `Lots::to_raw_qty()`, at the point `simulator::NewOrderRequest` is
/// built -- see `execution_user_doc.md`'s "Lots vs Qty" section.
#[derive(Debug, Clone, Copy)]
pub struct NewOrderIntent {
    pub strategy_id: StrategyId,
    pub instrument: InstrumentId,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: Lots,
}

fn order_type_price(ot: OrderType) -> Option<Price> {
    match ot {
        OrderType::LimitDay(p) | OrderType::BookOrCancel(p) | OrderType::Ioc(p) => Some(p),
        OrderType::MarketToLimit => None,
    }
}

/// Order Validation (D17): stateless reference-data checking -- tick size
/// and freeze quantity, exactly what FR-B27 names as in scope for phase
/// 1. Distinct from RMS (stateful policy) and from the venue's own
/// checks (D36).
///
/// `instrument.freeze_qty` is compared directly against `intent.qty`
/// (`Lots`) -- a real trading-limits concept like freeze quantity is
/// naturally expressed in lots, not wire-raw units, so now that
/// `NewOrderIntent.qty` is itself `Lots` this comparison is finally in
/// the right unit space. `freeze_qty` is still always `0` from
/// `refdata`'s own documented stub (no source column exists yet) -- a
/// separate, pre-existing data-completeness gap, not a units bug; see
/// `refdata_user_doc.md` §4.
fn validate(instrument: &Instrument, intent: &NewOrderIntent) -> Result<(), DenyReason> {
    if intent.qty.0 <= 0 || intent.qty.0 > instrument.freeze_qty {
        return Err(DenyReason::FreezeQty);
    }
    if let Some(price) = order_type_price(intent.order_type) {
        if instrument.tick_size.0 > 0 && price.0 % instrument.tick_size.0 != 0 {
            return Err(DenyReason::TickSize);
        }
    }
    Ok(())
}

/// RMS (D34): a trait, so a real implementation slots in later without
/// touching `ExecutionEngine`. The phase-1 answer is always yes -- no
/// throttling, no limits, matching the same swappable-trait pattern
/// `simulator::LatencyModel` already established for T06.
pub trait Rms {
    /// *Should this order go to the exchange?* -- as opposed to Order
    /// Validation's *will the exchange accept this order*. `true` means
    /// allow.
    fn check(&mut self, intent: &NewOrderIntent, portfolio: &Portfolio) -> bool;
}

/// Phase 1's RMS: always yes. Margin and cash checks are a later RMS
/// implementation's job (D34's own deferral) -- this component cannot
/// show a strategy would have been margin-called.
#[derive(Debug, Default)]
pub struct AlwaysAllowRms;

impl Rms for AlwaysAllowRms {
    fn check(&mut self, _intent: &NewOrderIntent, _portfolio: &Portfolio) -> bool {
        true
    }
}

/// Config for the engine's own local OTR/message-rate governor. Kept as
/// its own type (not reusing `simulator::OtrConfig` directly, though the
/// shape mirrors it) so it is unmistakably clear at every call site which
/// governor is being configured -- D19's "they do not share state" is
/// easy to violate by accident if the same config value is threaded
/// through both constructors without a second thought.
#[derive(Debug, Clone, Copy)]
pub struct LocalOtrConfig {
    pub window_ns: u64,
    pub max_messages_per_window: u32,
}

/// The engine-side third gate (FR-B27's "OTR governor", before the
/// venue). **Deliberately a separate implementation with separate state
/// from `simulator`'s own internal governor** -- D19 states this in
/// exactly these words: the two "do not share state, preserving the
/// venue independence of D10." A real exchange's own risk systems enforce
/// OTR at the exchange regardless of what a member's own pre-trade
/// controls do; modelling both, independently, is the honest shape.
struct LocalOtrGovernor {
    cfg: LocalOtrConfig,
    message_times_ns: VecDeque<u64>,
    admissions: u64,
    rejections: u64,
}

impl LocalOtrGovernor {
    fn new(cfg: LocalOtrConfig) -> Self {
        LocalOtrGovernor { cfg, message_times_ns: VecDeque::new(), admissions: 0, rejections: 0 }
    }

    fn prune(&mut self, now_ns: u64) {
        while let Some(&front) = self.message_times_ns.front() {
            if now_ns.saturating_sub(front) > self.cfg.window_ns {
                self.message_times_ns.pop_front();
            } else {
                break;
            }
        }
    }

    fn would_breach(&mut self, now_ns: u64) -> bool {
        self.prune(now_ns);
        if self.message_times_ns.len() as u32 + 1 > self.cfg.max_messages_per_window {
            self.rejections += 1;
            return true;
        }
        false
    }

    fn record(&mut self, now_ns: u64) {
        self.message_times_ns.push_back(now_ns);
        self.admissions += 1;
    }
}

// =======================================================================
// 4. Cost model -- FR-B30, D23.
// =======================================================================

/// Config-driven rates -- **not hardcoded literals inside the formula**
/// (D23). `Default` below supplies representative MCX-shaped placeholder
/// values for tests and demonstrations; real values belong in run
/// configuration (D22) once wired, and BACKTEST-PHASE1.md §7 ("do this
/// first") calls for checking them against real circulars before trusting
/// a quotable-spread computation built on them.
#[derive(Debug, Clone, Copy)]
pub struct CostConfig {
    /// Exchange transaction charge, fraction of turnover, both sides.
    pub exchange_txn_rate: f64,
    /// SEBI turnover fee, fraction of turnover, both sides.
    pub sebi_fee_rate: f64,
    /// Commodity Transaction Tax -- **sell side only** (D23).
    pub ctt_rate: f64,
    /// Stamp duty -- **buy side only** (D23).
    pub stamp_duty_rate: f64,
    /// GST, applied to (exchange transaction charge + brokerage), both
    /// sides.
    pub gst_rate: f64,
    /// Clearing/brokerage, flat rupees per lot, both sides.
    pub brokerage_per_lot: f64,
}

impl Default for CostConfig {
    fn default() -> Self {
        // Placeholder, representative-of-MCX rates -- not verified
        // against a live rate circular (BACKTEST-PHASE1.md §7's own
        // "compute the minimum quotable spread" task, out of scope for
        // this milestone's gate). The point being demonstrated is the
        // *mechanism* -- config-driven, direction-asymmetric -- not these
        // specific numbers.
        CostConfig {
            exchange_txn_rate: 0.0000002,
            sebi_fee_rate: 0.0000001,
            ctt_rate: 0.0001,
            stamp_duty_rate: 0.00002,
            gst_rate: 0.18,
            brokerage_per_lot: 20.0,
        }
    }
}

/// The full cost-stack breakdown for one leg (one fill), rupees.
/// Components broken out individually (not just a `total`) so Tier 2
/// reporting can show *why* a fill cost what it cost, per D26's
/// "realised cost from D23" requirement on fill records.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cost {
    pub exchange_txn_charge: f64,
    pub sebi_fee: f64,
    /// Zero unless this leg was a sell.
    pub ctt: f64,
    /// Zero unless this leg was a buy.
    pub stamp_duty: f64,
    pub gst: f64,
    pub brokerage: f64,
    pub total_rupees: f64,
}

impl Cost {
    /// Converts the rupee total into ticks for this instrument -- the
    /// exact comparison STRATEGY-GUIDE.md §9 shows
    /// (`edge_ticks <= cost.in_ticks(instrument)`).
    pub fn in_ticks(&self, instrument: &Instrument) -> f64 {
        let tick_rupees = rupees(instrument.tick_size) * instrument.multiplier as f64;
        if tick_rupees <= 0.0 {
            return f64::INFINITY;
        }
        self.total_rupees / tick_rupees
    }
}

/// A `CostModel` component (D23): config-driven, applied identically pre-
/// trade and post-trade -- **the same function serves both callers**, so
/// a strategy's quoting assumption and the realised accounting can never
/// quietly disagree.
#[derive(Debug, Clone, Copy)]
pub struct CostModel {
    cfg: CostConfig,
}

impl CostModel {
    pub fn new(cfg: CostConfig) -> Self {
        CostModel { cfg }
    }

    /// Cost of one leg -- `qty` of `instrument` trading at `price` on
    /// `side`. Named `round_trip` to match the call shape
    /// STRATEGY-GUIDE.md §9 and BACKTEST-PHASE1.md's FR-B30 show
    /// (`ctx.cost().round_trip(instrument, qty, side)`); **see
    /// execution_user_doc.md's cost-model section for why this is
    /// deliberately a per-leg function** -- a genuine two-legged round
    /// trip always pays exactly one CTT leg and one stamp-duty leg
    /// regardless of which comes first, which makes a *combined* round
    /// trip total unavoidably side-symmetric by arithmetic. The
    /// asymmetry that matters -- and that this task's acceptance bar
    /// checks -- is at the leg level: a buy fill and a sell fill of
    /// identical qty/price/instrument cost genuinely different totals,
    /// because CTT and stamp duty are set at different rates by
    /// regulation. Applied to a real fill, `side` is simply that fill's
    /// own side, so pre-trade query and post-trade accounting can never
    /// disagree about which component applied.
    ///
    /// `qty` is `Lots` -- deliberately, not `simulator`'s wire-raw `Qty`.
    /// `brokerage_per_lot * qty.0` and the turnover formula below both
    /// need a genuine lot count; this used to take a plain `Qty` and be
    /// called with `simulator`'s wire-raw fill quantity directly, which
    /// inflated every cost/turnover figure by `RAW_QTY_PER_LOT` (10,000x)
    /// -- see `execution_user_doc.md` and `dummy_strategy.md` for the
    /// real, previously-inflated output this produced. A caller holding a
    /// real fill's wire-raw `Qty` converts once, explicitly, via
    /// `Qty::to_lots()`.
    pub fn round_trip(&self, instrument: &Instrument, qty: Lots, price: Price, side: Side) -> Cost {
        let turnover = rupees(price) * qty.0 as f64 * instrument.multiplier as f64;
        let exchange_txn_charge = turnover * self.cfg.exchange_txn_rate;
        let sebi_fee = turnover * self.cfg.sebi_fee_rate;
        let ctt = if side == Side::Sell { turnover * self.cfg.ctt_rate } else { 0.0 };
        let stamp_duty = if side == Side::Buy { turnover * self.cfg.stamp_duty_rate } else { 0.0 };
        let brokerage = self.cfg.brokerage_per_lot * qty.0 as f64;
        let gst = (exchange_txn_charge + brokerage) * self.cfg.gst_rate;
        let total_rupees = exchange_txn_charge + sebi_fee + ctt + stamp_duty + gst + brokerage;
        Cost { exchange_txn_charge, sebi_fee, ctt, stamp_duty, gst, brokerage, total_rupees }
    }
}

// =======================================================================
// 5. Two-level accounting -- FR-B29, D08.
// =======================================================================

/// One strategy's own view: position, average entry price (for realised
/// P&L attribution), realised P&L, unrealised P&L (mark-to-market,
/// updated via `Portfolio::mark_to_market`), and total transaction cost
/// paid. **A strategy skews on this, never the firm's** (D08) -- if two
/// market makers shared one inventory number, each would see the other's
/// fills as its own.
///
/// **`position` is in plain lot count (`Lots`, signed), never
/// `simulator`'s wire-raw `Qty` scale** -- kept as a plain `i64` rather
/// than `types::Lots` itself only because `Lots` has no arithmetic
/// operators defined on it in `types.rs` (deliberately conservative scope
/// for this fix: see `execution_user_doc.md` §11 for why extending
/// `types.rs` was judged not worth it here) and this field needs to add,
/// negate, and compare signs. This is the third Lots-vs-`Qty` bug found
/// this session: `apply_fill` used to insert the fill's raw wire quantity
/// straight into this map and then multiply straight into `realized_pnl`
/// with no `instrument.multiplier` applied at all -- see
/// `execution_user_doc.md` §11's before/after numbers.
#[derive(Debug, Clone, Default)]
pub struct SubAccount {
    pub strategy_id: StrategyId,
    /// Signed position, in **lots** (`Qty::to_lots()` applied at the one
    /// place a fill's wire-raw quantity enters accounting, `apply_fill`).
    pub position: HashMap<InstrumentId, i64>,
    pub avg_price_rupees: HashMap<InstrumentId, f64>,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_cost: f64,
}

impl SubAccount {
    /// Signed position in **lots**, not wire-raw units -- see
    /// `SubAccount::position`'s own doc comment.
    pub fn net_position(&self, instrument: InstrumentId) -> i64 {
        self.position.get(&instrument).copied().unwrap_or(0)
    }

    pub fn gross_pnl(&self) -> f64 {
        self.realized_pnl + self.unrealized_pnl
    }

    pub fn net_pnl(&self) -> f64 {
        self.gross_pnl() - self.total_cost
    }
}

/// The firm-level view: every sub-account's position and P&L netted
/// together. **Always computed, never independently stored or
/// mutated** -- there is no setter anywhere in this module, which is what
/// actually enforces D08's "strategies can read both, but a strategy
/// skews on its own inventory, never the firm's": nothing exposes a way
/// to write into this from outside `Portfolio::firm()`'s own aggregation.
#[derive(Debug, Clone, Default)]
pub struct FirmAccount {
    /// Signed position, in **lots** -- netted straight from every
    /// sub-account's own `SubAccount::position`, itself in lots (see that
    /// field's doc comment). Summing lots across strategies for the same
    /// instrument is always valid (the per-lot contract size is a
    /// property of the instrument, not the strategy).
    pub position: HashMap<InstrumentId, i64>,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_cost: f64,
}

impl FirmAccount {
    pub fn gross_pnl(&self) -> f64 {
        self.realized_pnl + self.unrealized_pnl
    }

    pub fn net_pnl(&self) -> f64 {
        self.gross_pnl() - self.total_cost
    }
}

/// Owns every strategy's `SubAccount`. The firm view is derived on
/// demand (`firm()`), never stored -- see `FirmAccount`'s doc comment for
/// why that is what actually makes "read-only firm view" a real
/// guarantee rather than a naming convention.
#[derive(Debug, Default)]
pub struct Portfolio {
    sub_accounts: HashMap<StrategyId, SubAccount>,
}

impl Portfolio {
    pub fn sub_account(&self, strategy_id: StrategyId) -> Option<&SubAccount> {
        self.sub_accounts.get(&strategy_id)
    }

    pub fn strategies(&self) -> impl Iterator<Item = &StrategyId> {
        self.sub_accounts.keys()
    }

    /// Nets every sub-account together -- exposure across the whole firm,
    /// which is what STP and (later) margin must be checked against,
    /// since the exchange sees one member and one session, not one
    /// strategy at a time (D08).
    pub fn firm(&self) -> FirmAccount {
        let mut firm = FirmAccount::default();
        for sub in self.sub_accounts.values() {
            for (&instrument, &qty) in &sub.position {
                *firm.position.entry(instrument).or_insert(0) += qty;
            }
            firm.realized_pnl += sub.realized_pnl;
            firm.unrealized_pnl += sub.unrealized_pnl;
            firm.total_cost += sub.total_cost;
        }
        firm
    }

    /// Weighted-average-price position/P&L update for one fill. Extends
    /// or opens a position when the fill is same-direction as (or there
    /// is no) existing position; realises P&L on the portion that
    /// reduces or flips it. Standard average-cost accounting -- not a
    /// FIFO lot-matching engine, which phase 1 does not need.
    ///
    /// `qty` is the fill's real, `simulator`-native wire-raw `Qty` --
    /// converted to a plain lot count (`Qty::to_lots()`) at the very top,
    /// the one place a real fill quantity enters accounting, mirroring
    /// `on_fill`'s own conversion before calling `CostModel::round_trip`.
    /// `multiplier` is the instrument's real per-lot contract size
    /// (`Instrument.multiplier`, e.g. 100 barrels/lot for CRUDEOIL) --
    /// **the fix for the third Lots-vs-Qty bug found this session**: this
    /// function used to fold `qty.0` (wire-raw, e.g. `10_000` for "1
    /// lot") directly into `realized_pnl` with no `multiplier` applied at
    /// all, inflating every realised P&L figure by exactly
    /// `RAW_QTY_PER_LOT / multiplier` (100x for CRUDEOIL's multiplier of
    /// 100) -- see `execution_user_doc.md` §11 for the real, hand-checked
    /// before/after numbers.
    fn apply_fill(&mut self, strategy_id: StrategyId, instrument: InstrumentId, side: Side, qty: Qty, price: Price, cost_rupees: f64, multiplier: i64) {
        let sub = self.sub_accounts.entry(strategy_id).or_insert_with(|| SubAccount { strategy_id, ..Default::default() });
        let lots = qty.to_lots().0;
        let signed = match side {
            Side::Buy => lots,
            Side::Sell => -lots,
        };
        let price_rupees = rupees(price);
        let pos = *sub.position.get(&instrument).unwrap_or(&0);
        let avg = *sub.avg_price_rupees.get(&instrument).unwrap_or(&0.0);

        if pos == 0 || pos.signum() == signed.signum() {
            // Opening or extending in the same direction: fold the new
            // leg into the weighted average price (rupees per lot --
            // `multiplier` does not enter here, it only converts a price
            // *difference* into real rupee P&L, realised below).
            let new_pos = pos + signed;
            let new_avg = ((avg * pos.unsigned_abs() as f64) + price_rupees * signed.unsigned_abs() as f64) / new_pos.unsigned_abs() as f64;
            sub.position.insert(instrument, new_pos);
            sub.avg_price_rupees.insert(instrument, new_avg);
        } else {
            // Reducing or flipping through zero: realise P&L on the
            // closed portion at the existing average price -- real
            // rupees = price-diff-per-lot * lots-closed * real contract
            // size per lot.
            let closing_qty_lots = signed.unsigned_abs().min(pos.unsigned_abs());
            let pnl_per_lot = if pos > 0 { price_rupees - avg } else { avg - price_rupees };
            sub.realized_pnl += pnl_per_lot * closing_qty_lots as f64 * multiplier as f64;
            let new_pos = pos + signed;
            sub.position.insert(instrument, new_pos);
            if new_pos == 0 {
                sub.avg_price_rupees.insert(instrument, 0.0);
            } else if new_pos.signum() != pos.signum() {
                // Flipped through zero: whatever's left opens a fresh
                // position at this fill's price.
                sub.avg_price_rupees.insert(instrument, price_rupees);
            }
        }
        sub.total_cost += cost_rupees;
    }

    /// Mark-to-market: recomputes unrealised P&L for `instrument` in one
    /// strategy's sub-account against `mark_price`. Called by whatever
    /// drives the run (e.g. on every book update) -- this component does
    /// not read a book itself (D10-style independence: accounting reacts
    /// to prices it's handed, it does not go looking for them).
    ///
    /// Takes the full `&Instrument`, not just its id -- `multiplier` is
    /// needed to convert `(mark - avg)` (a rupees-per-lot price
    /// difference) into real rupee unrealised P&L, the same fix
    /// `apply_fill` needed for realised P&L (see that function's doc
    /// comment and `execution_user_doc.md` §11).
    pub fn mark_to_market(&mut self, strategy_id: StrategyId, instrument: &Instrument, mark_price: Price) {
        if let Some(sub) = self.sub_accounts.get_mut(&strategy_id) {
            let pos_lots = *sub.position.get(&instrument.id).unwrap_or(&0);
            let avg = *sub.avg_price_rupees.get(&instrument.id).unwrap_or(&0.0);
            let mark = rupees(mark_price);
            sub.unrealized_pnl = (mark - avg) * pos_lots as f64 * instrument.multiplier as f64;
        }
    }
}

// =======================================================================
// 6. Reporting -- FR-B31, D26.
// =======================================================================

/// The (config hash, build hash) run identity every output embeds
/// (FR-B31, D22). A placeholder scheme is explicitly acceptable for this
/// milestone's gate -- D22's full config-file infrastructure is not
/// required here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunIdentity {
    pub config_hash: u64,
    pub build_hash: &'static str,
}

/// A hardcoded literal, explicitly permitted by this milestone's
/// acceptance bar ("even a placeholder hash scheme ... a hardcoded build
/// string is acceptable"). A real build hash (git commit + compiler
/// flags) is D22 infrastructure, out of scope here.
pub const BUILD_HASH: &str = "phase1-execution-v0";

/// What actually determines a run's results, for hashing purposes --
/// deliberately mirrors D39's own `[run]`/`[deployment]` split ("hash
/// what changes results; do not hash what does not") even though the
/// real config-file infrastructure (D22) doesn't exist yet: `session_id`
/// affects client order ids, the cost/OTR configs affect fills and
/// accounting, the markout horizons affect what Tier 2 records. None of
/// this is deployment-only detail.
#[derive(Debug, Clone)]
pub struct RunConfig {
    pub session_id: u32,
    pub cost_config: CostConfig,
    pub local_otr: LocalOtrConfig,
    pub venue_otr: OtrConfigSummary,
    pub markout_horizons_ns: Vec<u64>,
}

/// A `Debug`/`Hash`-friendly summary of `simulator::OtrConfig` (which
/// itself holds a `Duration`, fine for `Debug` but this keeps
/// `RunConfig`'s hash input simple and independent of `Duration`'s own
/// `Debug` formatting stability).
#[derive(Debug, Clone, Copy)]
pub struct OtrConfigSummary {
    pub window_ns: u64,
    pub max_messages_per_window: u32,
    pub max_otr_ratio_bits: u64,
}

impl RunConfig {
    /// A deterministic hash of everything above via `Debug` formatting --
    /// stable for a given `RunConfig` value, run to run, which is the
    /// only property FR-12/FR-B31 actually need from a placeholder
    /// scheme. Not cryptographic, not meant to be.
    pub fn hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        format!("{self:?}").hash(&mut hasher);
        hasher.finish()
    }
}

/// One fill, carrying **queue position at fill and markout at fixed
/// horizons from the moment this record is created** (D26: "not optional
/// and not deferrable" -- retrofitting means re-running everything
/// already trusted). `markouts` is pre-populated with every configured
/// horizon mapped to `None`; `ExecutionEngine::observe_markout` fills
/// each in as that much simulated time actually elapses. The *field*
/// exists from fill time even though the *value* necessarily cannot.
#[derive(Debug, Clone)]
pub struct FillRecord {
    pub fill_id: u64,
    pub client_order_id: u64,
    pub strategy_id: StrategyId,
    pub instrument: InstrumentId,
    pub side: Side,
    pub price: Price,
    pub qty: Qty,
    pub kind: FillKind,
    pub timestamp_ns: u64,
    /// Aggregate quantity that was genuinely ahead of this order in the
    /// combined FIFO the instant before this fill -- `None` for an
    /// aggressive fill (immediate execution, nothing was queued to be
    /// "ahead" of).
    pub queue_position_at_fill: Option<i64>,
    pub spread_improving: bool,
    pub cost: Cost,
    /// `(horizon_ns, signed_markout_in_raw_price_units)` -- positive
    /// means the market moved in this fill's favour by that horizon.
    pub markouts: Vec<(u64, Option<i64>)>,
}

/// Tier 2's other stream: every order command and response, with
/// rejection reasons distinguished (own limit / firm limit / venue
/// rejection -- D08 requires strategies to tell these apart, D26 requires
/// reporting to as well).
#[derive(Debug, Clone)]
pub struct OrderEventRecord {
    pub client_order_id: u64,
    pub timestamp_ns: u64,
    pub description: String,
    pub resulting_state: OrderState,
}

/// What one call to a mutating `ExecutionEngine` method just produced --
/// the real slice of `self.fills`/`self.order_events` that call itself
/// added, not the whole running history. This is what makes live
/// delivery possible: `control_dispatcher::ControlDispatcher::dispatch`
/// takes one of these and calls a strategy's `on_fill`/`on_order_update`
/// once per new record, the moment they're produced, rather than a
/// strategy only ever seeing them via `ExecutionEngine::fills()`/
/// `order_events()` after the whole run ends. See each wrapping method's
/// own doc comment (search `_inner` in this file) for how this is
/// computed -- a before/after length snapshot around the unchanged
/// original body, not a new accounting path.
#[derive(Debug, Clone, Default)]
pub struct ExecOutcome {
    pub fills: Vec<FillRecord>,
    pub order_events: Vec<OrderEventRecord>,
}

#[derive(Debug, Clone, Default)]
pub struct MessageCounts {
    pub new_order_attempts: u64,
    pub denied: u64,
    pub submitted_to_venue: u64,
    pub cancel_requests: u64,
    pub modify_requests: u64,
    pub market_events_applied: u64,
}

#[derive(Debug, Clone, Default)]
pub struct StrategySummary {
    pub strategy_id: StrategyId,
    pub gross_pnl: f64,
    pub net_pnl: f64,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_cost: f64,
    /// Read straight from `SubAccount::position` -- signed **lots**, not
    /// wire-raw units (see that field's doc comment). A 1-lot position
    /// now reports as `1`, not `10000`.
    pub positions: Vec<(InstrumentId, i64)>,
}

/// A simple mean-and-count summary per configured horizon, over whatever
/// markout observations have actually been recorded so far.
#[derive(Debug, Clone, Default)]
pub struct MarkoutSummary {
    /// `(horizon_ns, observation_count, mean_raw_price_units)`.
    pub per_horizon: Vec<(u64, u64, f64)>,
}

/// Tier 1 (FR-B31): the always-on, compact per-run summary. P&L gross and
/// net at both accounting levels, inventory, markout distribution, OTR
/// consumed (both governors, kept distinct per D19), message counts.
/// Columnar in spirit -- every field here is a plain number or a small
/// vector, so hundreds of sweep runs can aggregate this trivially.
#[derive(Debug, Clone)]
pub struct Tier1Summary {
    pub run_identity: RunIdentity,
    pub firm_gross_pnl: f64,
    pub firm_net_pnl: f64,
    pub firm_realized_pnl: f64,
    pub firm_unrealized_pnl: f64,
    pub firm_total_cost: f64,
    /// Read straight from `FirmAccount::position` -- signed **lots**, not
    /// wire-raw units (see that field's doc comment).
    pub firm_inventory: Vec<(InstrumentId, i64)>,
    pub per_strategy: Vec<StrategySummary>,
    pub markout_distribution: MarkoutSummary,
    pub local_otr_admissions: u64,
    pub local_otr_rejections: u64,
    pub venue_otr_admissions: u64,
    pub venue_otr_rejections: u64,
    pub message_counts: MessageCounts,
    /// Terminal-state counts across every order this engine has ever
    /// seen -- `Denied`/`Rejected`/`Filled`/`Canceled`/`Expired` counted
    /// separately, exactly the D08/D26 "tell these apart" requirement
    /// applied to the summary level too.
    pub denied_count: u64,
    pub rejected_count: u64,
    pub filled_count: u64,
    pub canceled_count: u64,
    pub expired_count: u64,
}

impl fmt::Display for Tier1Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== qtrade run report (Tier 1) ===")?;
        writeln!(f, "run identity: config_hash={:#018x} build_hash={}", self.run_identity.config_hash, self.run_identity.build_hash)?;
        writeln!(f, "--- firm level ---")?;
        writeln!(
            f,
            "gross_pnl={:.4} net_pnl={:.4} realized={:.4} unrealized={:.4} total_cost={:.4}",
            self.firm_gross_pnl, self.firm_net_pnl, self.firm_realized_pnl, self.firm_unrealized_pnl, self.firm_total_cost
        )?;
        write!(f, "inventory: ")?;
        for (i, (id, qty)) in self.firm_inventory.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{id:?}={qty}")?;
        }
        writeln!(f)?;
        writeln!(f, "--- per-strategy ---")?;
        for s in &self.per_strategy {
            writeln!(
                f,
                "strategy={} gross_pnl={:.4} net_pnl={:.4} realized={:.4} unrealized={:.4} cost={:.4}",
                s.strategy_id, s.gross_pnl, s.net_pnl, s.realized_pnl, s.unrealized_pnl, s.total_cost
            )?;
        }
        writeln!(f, "--- OTR ---")?;
        writeln!(
            f,
            "local: admitted={} rejected={} | venue: admitted={} rejected={}",
            self.local_otr_admissions, self.local_otr_rejections, self.venue_otr_admissions, self.venue_otr_rejections
        )?;
        writeln!(f, "--- messages ---")?;
        writeln!(
            f,
            "new_order_attempts={} denied={} submitted_to_venue={} cancel_requests={} modify_requests={} market_events_applied={}",
            self.message_counts.new_order_attempts,
            self.message_counts.denied,
            self.message_counts.submitted_to_venue,
            self.message_counts.cancel_requests,
            self.message_counts.modify_requests,
            self.message_counts.market_events_applied
        )?;
        writeln!(f, "--- terminal state counts ---")?;
        writeln!(
            f,
            "denied={} rejected={} filled={} canceled={} expired={}",
            self.denied_count, self.rejected_count, self.filled_count, self.canceled_count, self.expired_count
        )?;
        writeln!(f, "--- markout ---")?;
        for (horizon_ns, n, mean) in &self.markout_distribution.per_horizon {
            writeln!(f, "horizon_ns={horizon_ns} observations={n} mean_raw_price_units={mean:.4}")?;
        }
        Ok(())
    }
}

// =======================================================================
// 7. ExecutionEngine -- ties gates, client order ids, the venue,
//    accounting, cost, and reporting together.
// =======================================================================

/// The outcome of `ExecutionEngine::submit_order`. `Denied` returns
/// synchronously with a reason and the order never touched the venue
/// (FR-B27, D36); `Submitted` means all three gates passed and the venue
/// has already been called (its own `Vec<ExecReport>` was processed
/// immediately -- see `execution_user_doc.md` for why this single call
/// still honestly represents "venue responses arrive as events": the
/// *timing* of when this method itself gets invoked, at the
/// latency-adjusted simulated timestamp, is what the scheduler-driven
/// caller controls, per FR-B14/D36).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateOutcome {
    Denied { client_order_id: u64, reason: DenyReason },
    Submitted { client_order_id: u64 },
}

pub struct ExecutionEngine {
    clord: ClOrdIdGen,
    instruments: HashMap<InstrumentId, Instrument>,
    venue: SimExchange,
    rms: Box<dyn Rms>,
    local_otr: LocalOtrGovernor,
    cost_model: CostModel,
    portfolio: Portfolio,
    orders: HashMap<u64, Order>,
    /// The genuine queue position each currently-open order had ahead of
    /// it the first time it was observed resting -- captured once, then
    /// held fixed (never overwritten by a later, smaller reading) until
    /// the order terminates. Price/time priority means qty-ahead can
    /// only ever be consumed (shrink), never grow, once an order is
    /// resting, so "first observed while open" *is* the position
    /// established when the order joined the queue -- the number that
    /// answers "how much size did this order have to wait through before
    /// its fill", which is what "queue position at fill" means (see
    /// `on_market_event` and the bug this fixed in
    /// execution_user_doc.md).
    pre_event_qty_ahead: HashMap<u64, i64>,
    venue_submit_calls: u64,
    venue_cancel_calls: u64,
    venue_modify_calls: u64,
    message_counts: MessageCounts,
    next_fill_id: u64,
    tier2_enabled: bool,
    fills: Vec<FillRecord>,
    order_events: Vec<OrderEventRecord>,
    markout_horizons_ns: Vec<u64>,
    run_config: RunConfig,
}

impl ExecutionEngine {
    pub fn new(
        run_config: RunConfig,
        instruments: Vec<Instrument>,
        rms: Box<dyn Rms>,
        cost_config: CostConfig,
        venue_otr_cfg: OtrConfig,
        markout_horizons_ns: Vec<u64>,
        tier2_enabled: bool,
    ) -> Self {
        let ids: Vec<InstrumentId> = instruments.iter().map(|i| i.id).collect();
        let venue = SimExchange::new(&ids, venue_otr_cfg);
        let clord = ClOrdIdGen::new(run_config.session_id);
        let local_otr = LocalOtrGovernor::new(run_config.local_otr);
        let mut map = HashMap::new();
        for i in instruments {
            map.insert(i.id, i);
        }
        ExecutionEngine {
            clord,
            instruments: map,
            venue,
            rms,
            local_otr,
            cost_model: CostModel::new(cost_config),
            portfolio: Portfolio::default(),
            orders: HashMap::new(),
            pre_event_qty_ahead: HashMap::new(),
            venue_submit_calls: 0,
            venue_cancel_calls: 0,
            venue_modify_calls: 0,
            message_counts: MessageCounts::default(),
            next_fill_id: 0,
            tier2_enabled,
            fills: Vec::new(),
            order_events: Vec::new(),
            markout_horizons_ns,
            run_config,
        }
    }

    // ---- introspection used by callers and by the test suite ----

    pub fn order(&self, client_order_id: u64) -> Option<&Order> {
        self.orders.get(&client_order_id)
    }

    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }

    pub fn portfolio_mut(&mut self) -> &mut Portfolio {
        &mut self.portfolio
    }

    /// How many times this engine has actually called
    /// `SimExchange::submit` -- the call counter the acceptance bar asks
    /// for to verify a `Denied` order never reaches the venue.
    pub fn venue_submit_calls(&self) -> u64 {
        self.venue_submit_calls
    }

    pub fn venue_cancel_calls(&self) -> u64 {
        self.venue_cancel_calls
    }

    pub fn venue_modify_calls(&self) -> u64 {
        self.venue_modify_calls
    }

    pub fn fills(&self) -> &[FillRecord] {
        &self.fills
    }

    pub fn order_events(&self) -> &[OrderEventRecord] {
        &self.order_events
    }

    pub fn cost_model(&self) -> &CostModel {
        &self.cost_model
    }

    pub fn instrument(&self, id: InstrumentId) -> Option<&Instrument> {
        self.instruments.get(&id)
    }

    fn log_event(&mut self, client_order_id: u64, description: &str, resulting_state: OrderState, now_ns: u64) {
        if self.tier2_enabled {
            self.order_events.push(OrderEventRecord {
                client_order_id,
                timestamp_ns: now_ns,
                description: description.to_string(),
                resulting_state,
            });
        }
    }

    // ---- gates + submission (FR-B27, D36) ----

    /// Runs Validation -> RMS -> local OTR governor, then (only if all
    /// three pass) forwards to the venue, and hands back whatever
    /// fills/order-events this one call produced -- see `ExecOutcome`'s
    /// own doc comment. A before/after snapshot of `self.fills`/
    /// `self.order_events`' lengths around the unchanged original body
    /// (`submit_order_inner`), not a new accounting path.
    pub fn submit_order(&mut self, intent: NewOrderIntent, now_ns: u64) -> (GateOutcome, ExecOutcome) {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        let outcome = self.submit_order_inner(intent, now_ns);
        (outcome, ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() })
    }

    /// Runs Validation -> RMS -> local OTR governor, in that order, then
    /// (only if all three pass) forwards to the venue. The order record
    /// is created **before** the gates run, per FR-B27, so a
    /// locally-denied order still exists, still shows up in `order()`
    /// and in reporting, and still counts toward the local OTR governor
    /// if it reached that gate (and only if it reached that gate --
    /// Validation/RMS failures never touch `local_otr` at all).
    fn submit_order_inner(&mut self, intent: NewOrderIntent, now_ns: u64) -> GateOutcome {
        let client_order_id = self.clord.next();
        self.message_counts.new_order_attempts += 1;

        let Some(instrument) = self.instruments.get(&intent.instrument).cloned() else {
            self.deny(client_order_id, &intent, DenyReason::UnknownInstrument, now_ns);
            return GateOutcome::Denied { client_order_id, reason: DenyReason::UnknownInstrument };
        };

        if let Err(reason) = validate(&instrument, &intent) {
            self.deny(client_order_id, &intent, reason, now_ns);
            return GateOutcome::Denied { client_order_id, reason };
        }
        if !self.rms.check(&intent, &self.portfolio) {
            self.deny(client_order_id, &intent, DenyReason::RmsRejected, now_ns);
            return GateOutcome::Denied { client_order_id, reason: DenyReason::RmsRejected };
        }
        if self.local_otr.would_breach(now_ns) {
            self.deny(client_order_id, &intent, DenyReason::LocalOtrOrRate, now_ns);
            return GateOutcome::Denied { client_order_id, reason: DenyReason::LocalOtrOrRate };
        }
        self.local_otr.record(now_ns);

        let (pre_bid, pre_ask) = self
            .venue
            .book(intent.instrument)
            .map(|b| (b.best_bid().map(|l| l.price), b.best_ask().map(|l| l.price)))
            .unwrap_or((None, None));
        let pre_submit_best_same_side = match intent.side {
            Side::Buy => pre_bid,
            Side::Sell => pre_ask,
        };

        // `Order`'s own fill-tracking fields (`requested_qty`/
        // `filled_qty`/`leaves_qty`) stay in `Qty` -- unchanged type,
        // unchanged meaning -- since they interact with real fills from
        // `simulator`, which is itself unchanged and native to that
        // scale. `intent.qty` (`Lots`) is converted exactly once, here,
        // via `to_raw_qty()`.
        let requested_qty = intent.qty.to_raw_qty();
        let order = Order {
            client_order_id,
            strategy_id: intent.strategy_id,
            instrument: intent.instrument,
            side: intent.side,
            order_type: intent.order_type,
            requested_qty,
            state: OrderState::Submitted,
            filled_qty: Qty(0),
            leaves_qty: requested_qty,
            working_price: order_type_price(intent.order_type),
            deny_reason: None,
            reject_reason: None,
            cancel_reason: None,
            spread_improving: false,
            pre_submit_best_same_side,
        };
        self.orders.insert(client_order_id, order);
        self.log_event(client_order_id, "submit: gates passed, forwarding to venue", OrderState::Submitted, now_ns);

        let req = NewOrderRequest { client_order_id, instrument: intent.instrument, side: intent.side, order_type: intent.order_type, qty: requested_qty };
        self.venue_submit_calls += 1;
        self.message_counts.submitted_to_venue += 1;
        let reports = self.venue.submit(req, now_ns);
        self.handle_exec_reports(reports, now_ns);
        GateOutcome::Submitted { client_order_id }
    }

    fn deny(&mut self, client_order_id: u64, intent: &NewOrderIntent, reason: DenyReason, now_ns: u64) {
        let order = Order {
            client_order_id,
            strategy_id: intent.strategy_id,
            instrument: intent.instrument,
            side: intent.side,
            order_type: intent.order_type,
            requested_qty: intent.qty.to_raw_qty(),
            state: OrderState::Denied,
            filled_qty: Qty(0),
            leaves_qty: Qty(0),
            working_price: order_type_price(intent.order_type),
            deny_reason: Some(reason),
            reject_reason: None,
            cancel_reason: None,
            spread_improving: false,
            pre_submit_best_same_side: None,
        };
        self.orders.insert(client_order_id, order);
        self.message_counts.denied += 1;
        self.log_event(client_order_id, &format!("denied: {reason}"), OrderState::Denied, now_ns);
        // Deliberately nothing else happens here: no venue call of any
        // kind. This is what the acceptance test's call-counter check
        // confirms from outside.
    }

    // ---- cancel (two-phase, so the PendingCancel -> Filled race is
    // ---- actually constructible, not merely asserted) ----

    /// Same `ExecOutcome`-returning wrapper as `submit_order`, around
    /// the unchanged `request_cancel_inner`.
    pub fn request_cancel(&mut self, client_order_id: u64, now_ns: u64) -> (bool, ExecOutcome) {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        let ok = self.request_cancel_inner(client_order_id, now_ns);
        (ok, ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() })
    }

    /// Phase 1 of a cancel: marks the order `PendingCancel` -- "cancel
    /// sent, awaiting venue response" (STRATEGY-GUIDE.md §7a). Does
    /// **not** itself call the venue; in production that call happens
    /// later, when the scheduler-driven caller's `OrderArrival`-class
    /// event for this cancel message actually fires (T04/D36). Returns
    /// `false` if the order isn't currently open (nothing to cancel).
    fn request_cancel_inner(&mut self, client_order_id: u64, now_ns: u64) -> bool {
        self.message_counts.cancel_requests += 1;
        let Some(order) = self.orders.get_mut(&client_order_id) else { return false };
        if !order.state.is_open() {
            return false;
        }
        order.state = OrderState::PendingCancel;
        self.log_event(client_order_id, "cancel requested", OrderState::PendingCancel, now_ns);
        true
    }

    /// Same `ExecOutcome`-returning wrapper, around the unchanged
    /// `deliver_cancel_to_venue_inner`.
    pub fn deliver_cancel_to_venue(&mut self, client_order_id: u64, now_ns: u64) -> ExecOutcome {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        self.deliver_cancel_to_venue_inner(client_order_id, now_ns);
        ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() }
    }

    /// Phase 2: the cancel message actually reaches the venue. Between
    /// `request_cancel` and this call, `on_market_event` may have already
    /// filled the order at the venue -- `SimExchange` has no notion of
    /// "pending cancel," it just still had the order resting until
    /// something removed it. If that happened, `self.venue.cancel(...)`
    /// finds nothing resting and returns an empty `Vec`; `handle_exec_reports`
    /// then has nothing to apply, and the order is left exactly as the
    /// fill already left it: `Filled`. This is the race, made real rather
    /// than asserted by inspection.
    fn deliver_cancel_to_venue_inner(&mut self, client_order_id: u64, now_ns: u64) {
        self.venue_cancel_calls += 1;
        let reports = self.venue.cancel(client_order_id, now_ns);
        self.handle_exec_reports(reports, now_ns);
    }

    // ---- modify (two-phase, same shape as cancel) ----

    /// Same `ExecOutcome`-returning wrapper as `request_cancel`.
    pub fn request_modify(&mut self, client_order_id: u64, now_ns: u64) -> (bool, ExecOutcome) {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        let ok = self.request_modify_inner(client_order_id, now_ns);
        (ok, ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() })
    }

    fn request_modify_inner(&mut self, client_order_id: u64, now_ns: u64) -> bool {
        self.message_counts.modify_requests += 1;
        let Some(order) = self.orders.get_mut(&client_order_id) else { return false };
        if !matches!(order.state, OrderState::Accepted | OrderState::PartiallyFilled) {
            return false;
        }
        order.state = OrderState::PendingUpdate;
        self.log_event(client_order_id, "modify requested", OrderState::PendingUpdate, now_ns);
        true
    }

    /// Same `ExecOutcome`-returning wrapper as `deliver_cancel_to_venue`.
    pub fn deliver_modify_to_venue(&mut self, client_order_id: u64, new_qty: Qty, new_price: Option<Price>, now_ns: u64) -> ExecOutcome {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        self.deliver_modify_to_venue_inner(client_order_id, new_qty, new_price, now_ns);
        ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() }
    }

    fn deliver_modify_to_venue_inner(&mut self, client_order_id: u64, new_qty: Qty, new_price: Option<Price>, now_ns: u64) {
        self.venue_modify_calls += 1;
        let reports = self.venue.modify(client_order_id, new_qty, new_price, now_ns);
        self.handle_exec_reports(reports, now_ns);
    }

    // ---- unsolicited: end of day / GTD (no live driver yet; the
    // ---- transition exists so later session-state work can call it) ----

    /// Same `ExecOutcome`-returning wrapper as `request_cancel`.
    pub fn mark_expired(&mut self, client_order_id: u64, now_ns: u64) -> (bool, ExecOutcome) {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        let ok = self.mark_expired_inner(client_order_id, now_ns);
        (ok, ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() })
    }

    fn mark_expired_inner(&mut self, client_order_id: u64, now_ns: u64) -> bool {
        let Some(order) = self.orders.get_mut(&client_order_id) else { return false };
        if !order.state.is_open() {
            return false;
        }
        order.state = OrderState::Expired;
        self.pre_event_qty_ahead.remove(&client_order_id);
        self.log_event(client_order_id, "expired", OrderState::Expired, now_ns);
        true
    }

    // ---- market data -> the venue's independent book (D10) ----

    /// Feeds one real decoded event to the venue and processes whatever
    /// `ExecReport`s it produces. Before applying the event, records each
    /// currently-open order's queue position *only if not already
    /// recorded* -- the first reading taken while an order is open is its
    /// genuine queue position, and price/time priority guarantees it can
    /// only shrink from there while resting, never grow. Recomputing (and
    /// overwriting) this on every subsequent market event would instead
    /// capture whatever's left just before the *specific* event that
    /// happens to trigger the fill -- which is frequently already 0 once
    /// earlier events have consumed the real quantity ahead, silently
    /// reporting "no one was ahead of you" for an order that in fact
    /// queued behind real size. See execution_user_doc.md for the test
    /// that caught this and the fix.
    /// Same `ExecOutcome`-returning wrapper as `submit_order`, around the
    /// unchanged `on_market_event_inner` -- this is the one of the seven
    /// wrapped methods `main.rs`'s real replay loop actually calls every
    /// event, and therefore the one real path `control_dispatcher`
    /// currently ever receives anything through.
    pub fn on_market_event(&mut self, event: &DecodedMessage, now_ns: u64) -> ExecOutcome {
        let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
        self.on_market_event_inner(event, now_ns);
        ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() }
    }

    fn on_market_event_inner(&mut self, event: &DecodedMessage, now_ns: u64) {
        for (&id, order) in self.orders.iter() {
            if order.state.is_open() {
                if let Some(ahead) = self.venue.resting_qty_ahead(id) {
                    self.pre_event_qty_ahead.entry(id).or_insert(ahead);
                }
            }
        }
        self.message_counts.market_events_applied += 1;
        let reports = self.venue.apply_market_event(event, now_ns);
        self.handle_exec_reports(reports, now_ns);
    }

    /// Records a markout observation for one already-recorded fill, at
    /// one of its pre-configured horizons.
    pub fn observe_markout(&mut self, fill_id: u64, horizon_ns: u64, mid_price: Price) {
        if let Some(fill) = self.fills.iter_mut().find(|f| f.fill_id == fill_id) {
            if let Some(slot) = fill.markouts.iter_mut().find(|(h, _)| *h == horizon_ns) {
                let signed = match fill.side {
                    Side::Buy => mid_price.0 - fill.price.0,
                    Side::Sell => fill.price.0 - mid_price.0,
                };
                slot.1 = Some(signed);
            }
        }
    }

    // ---- exec report handling -- the single place all of Rejected/
    // ---- Resting/Filled/Canceled are turned into state transitions,
    // ---- accounting, and fill records ----

    fn handle_exec_reports(&mut self, reports: Vec<ExecReport>, now_ns: u64) {
        for report in reports {
            match report {
                ExecReport::Rejected { client_order_id, reason } => {
                    if let Some(order) = self.orders.get_mut(&client_order_id) {
                        match order.state {
                            OrderState::Submitted => {
                                order.state = OrderState::Rejected;
                                order.reject_reason = Some(reason);
                                self.pre_event_qty_ahead.remove(&client_order_id);
                            }
                            OrderState::PendingUpdate => {
                                // "Modify accepted or rejected -- either
                                // way the order is working again"
                                // (STRATEGY-GUIDE.md §7a): a rejected
                                // modify does not terminally reject the
                                // order itself.
                                order.state = if order.filled_qty.0 > 0 { OrderState::PartiallyFilled } else { OrderState::Accepted };
                                order.reject_reason = Some(reason);
                            }
                            _ => {} // already terminal or otherwise not awaiting this -- ignore
                        }
                    }
                    self.log_event(client_order_id, &format!("venue rejected: {reason:?}"), OrderState::Rejected, now_ns);
                }
                ExecReport::Resting { client_order_id, handle, qty } => {
                    if let Some(order) = self.orders.get_mut(&client_order_id) {
                        if !order.state.is_terminal() {
                            order.leaves_qty = qty;
                            order.working_price = Some(handle.price);
                            let pre = order.pre_submit_best_same_side;
                            order.spread_improving = match (order.side, pre) {
                                (Side::Buy, Some(p)) => handle.price.0 > p.0,
                                (Side::Sell, Some(p)) => handle.price.0 < p.0,
                                (_, None) => true, // no prior same-side quote to improve on -- treat as improving
                            };
                            order.state = if order.filled_qty.0 > 0 { OrderState::PartiallyFilled } else { OrderState::Accepted };
                        }
                    }
                    self.log_event(client_order_id, "resting", OrderState::Accepted, now_ns);
                }
                ExecReport::Filled { client_order_id, price, qty, kind } => {
                    self.on_fill(client_order_id, price, qty, kind, now_ns);
                }
                ExecReport::Canceled { client_order_id, reason } => {
                    if let Some(order) = self.orders.get_mut(&client_order_id) {
                        // The race: never regress an order that is
                        // already terminal (in particular `Filled`, via
                        // `PendingCancel -> Filled`) back to `Canceled`
                        // just because a cancel response arrived after
                        // the fact.
                        if !order.state.is_terminal() {
                            order.state = OrderState::Canceled;
                            order.cancel_reason = Some(map_cancel_reason(reason));
                            self.pre_event_qty_ahead.remove(&client_order_id);
                        }
                    }
                    self.log_event(client_order_id, &format!("canceled: {reason:?}"), OrderState::Canceled, now_ns);
                }
            }
        }
    }

    fn on_fill(&mut self, client_order_id: u64, price: Price, qty: Qty, kind: FillKind, now_ns: u64) {
        let Some(order) = self.orders.get(&client_order_id) else { return };
        let Some(instrument) = self.instruments.get(&order.instrument).cloned() else { return };
        let queue_position_at_fill = if kind == FillKind::Passive { self.pre_event_qty_ahead.get(&client_order_id).copied() } else { None };
        let side = order.side;
        let strategy_id = order.strategy_id;
        let spread_improving = order.spread_improving;
        let instrument_id = order.instrument;

        // `qty` here is `simulator`'s own wire-raw fill quantity --
        // convert to `Lots` at this call site, the one place a real fill
        // quantity meets the cost model (see `round_trip`'s own doc
        // comment for why it now takes `Lots`, not `Qty`).
        let cost = self.cost_model.round_trip(&instrument, qty.to_lots(), price, side);
        self.portfolio.apply_fill(strategy_id, instrument_id, side, qty, price, cost.total_rupees, instrument.multiplier);

        if let Some(order) = self.orders.get_mut(&client_order_id) {
            order.filled_qty = Qty(order.filled_qty.0 + qty.0);
            order.leaves_qty = Qty((order.requested_qty.0 - order.filled_qty.0).max(0));
            // The race, made concrete: this assignment happens
            // unconditionally, overriding `PendingCancel` (or any other
            // non-terminal state) alike -- a fill always wins.
            order.state = if order.leaves_qty.0 <= 0 { OrderState::Filled } else { OrderState::PartiallyFilled };
            if order.state == OrderState::Filled {
                // Order is done -- drop its sticky queue-position entry
                // now rather than let it accumulate for the life of the
                // run (client_order_ids are never reused, so a stale
                // entry would otherwise sit unread forever).
                self.pre_event_qty_ahead.remove(&client_order_id);
            }
        }

        let fill_id = self.next_fill_id;
        self.next_fill_id += 1;
        if self.tier2_enabled {
            self.fills.push(FillRecord {
                fill_id,
                client_order_id,
                strategy_id,
                instrument: instrument_id,
                side,
                price,
                qty,
                kind,
                timestamp_ns: now_ns,
                queue_position_at_fill,
                spread_improving,
                cost,
                markouts: self.markout_horizons_ns.iter().map(|h| (*h, None)).collect(),
            });
        }
        self.log_event(client_order_id, &format!("filled qty={} kind={kind:?}", qty.0), OrderState::Filled, now_ns);
    }

    // ---- reporting (FR-B31, D26) ----

    pub fn run_identity(&self) -> RunIdentity {
        RunIdentity { config_hash: self.run_config.hash(), build_hash: BUILD_HASH }
    }

    pub fn tier1_report(&self) -> Tier1Summary {
        let firm = self.portfolio.firm();
        let mut per_strategy = Vec::new();
        let mut strategy_ids: Vec<StrategyId> = self.portfolio.strategies().copied().collect();
        strategy_ids.sort_unstable();
        for sid in strategy_ids {
            if let Some(sub) = self.portfolio.sub_account(sid) {
                let mut positions: Vec<(InstrumentId, i64)> = sub.position.iter().map(|(&i, &q)| (i, q)).collect();
                positions.sort_by_key(|(i, _)| i.0);
                per_strategy.push(StrategySummary {
                    strategy_id: sid,
                    gross_pnl: sub.gross_pnl(),
                    net_pnl: sub.net_pnl(),
                    realized_pnl: sub.realized_pnl,
                    unrealized_pnl: sub.unrealized_pnl,
                    total_cost: sub.total_cost,
                    positions,
                });
            }
        }
        let mut firm_inventory: Vec<(InstrumentId, i64)> = firm.position.iter().map(|(&i, &q)| (i, q)).collect();
        firm_inventory.sort_by_key(|(i, _)| i.0);

        let mut denied_count = 0u64;
        let mut rejected_count = 0u64;
        let mut filled_count = 0u64;
        let mut canceled_count = 0u64;
        let mut expired_count = 0u64;
        for order in self.orders.values() {
            match order.state {
                OrderState::Denied => denied_count += 1,
                OrderState::Rejected => rejected_count += 1,
                OrderState::Filled => filled_count += 1,
                OrderState::Canceled => canceled_count += 1,
                OrderState::Expired => expired_count += 1,
                _ => {}
            }
        }

        let mut per_horizon = Vec::new();
        for &h in &self.markout_horizons_ns {
            let observed: Vec<i64> = self.fills.iter().filter_map(|f| f.markouts.iter().find(|(hh, _)| *hh == h).and_then(|(_, v)| *v)).collect();
            let n = observed.len() as u64;
            let mean = if n > 0 { observed.iter().sum::<i64>() as f64 / n as f64 } else { 0.0 };
            per_horizon.push((h, n, mean));
        }

        Tier1Summary {
            run_identity: self.run_identity(),
            firm_gross_pnl: firm.gross_pnl(),
            firm_net_pnl: firm.net_pnl(),
            firm_realized_pnl: firm.realized_pnl,
            firm_unrealized_pnl: firm.unrealized_pnl,
            firm_total_cost: firm.total_cost,
            firm_inventory,
            per_strategy,
            markout_distribution: MarkoutSummary { per_horizon },
            local_otr_admissions: self.local_otr.admissions,
            local_otr_rejections: self.local_otr.rejections,
            venue_otr_admissions: self.venue.audit.otr_admissions,
            venue_otr_rejections: self.venue.audit.otr_rejections,
            message_counts: self.message_counts.clone(),
            denied_count,
            rejected_count,
            filled_count,
            canceled_count,
            expired_count,
        }
    }
}

// =======================================================================
// 8. Tests -- the deterministic, synthetic acceptance harness this
//    milestone's own brief permits in place of a full real-data replay
//    gate. Run with `cargo test --bin execution-validate` (see
//    validate.rs in this folder for why a second bin target exists,
//    same reason `book`/`cache`/`simulator` each added one -- main.rs is
//    intentionally untouched this round).
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Currency, InstrumentKind, Settlement, Venue};

    fn future_instrument(id: u32, tick_size: i64, freeze_qty: i64, multiplier: i64) -> Instrument {
        Instrument {
            id: InstrumentId(id),
            venue: Venue::Mcx,
            native_id: id as i64,
            kind: InstrumentKind::Future {
                underlying: "CRUDEOIL".to_string(),
                expiry: crate::types::Date(0),
                contract_month: crate::types::YearMonth { year: 2026, month: 1 },
                settlement: Settlement::Cash,
            },
            tick_size: Price(tick_size),
            lot_size: 1,
            multiplier,
            freeze_qty,
            price_band: None,
            currency: Currency::Inr,
        }
    }

    fn engine(instruments: Vec<Instrument>) -> ExecutionEngine {
        let run_config = RunConfig {
            session_id: 7,
            cost_config: CostConfig::default(),
            local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
            venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
            markout_horizons_ns: vec![1_000_000, 5_000_000],
        };
        let venue_otr = OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
        ExecutionEngine::new(run_config, instruments, Box::new(AlwaysAllowRms), CostConfig::default(), venue_otr, vec![1_000_000, 5_000_000], true)
    }

    const IID: InstrumentId = InstrumentId(1);

    // ---- §1: eleven-state machine, groupings ----

    #[test]
    fn groupings_match_strategy_guide_7a() {
        use OrderState::*;
        assert!(Accepted.is_open() && PartiallyFilled.is_open() && PendingUpdate.is_open() && PendingCancel.is_open());
        assert!(!Initialized.is_open() && !Submitted.is_open() && !Denied.is_open() && !Rejected.is_open() && !Filled.is_open() && !Canceled.is_open() && !Expired.is_open());

        assert!(Submitted.is_inflight() && PendingUpdate.is_inflight() && PendingCancel.is_inflight());
        assert!(!Initialized.is_inflight() && !Accepted.is_inflight() && !PartiallyFilled.is_inflight());

        assert!(Denied.is_terminal() && Rejected.is_terminal() && Filled.is_terminal() && Canceled.is_terminal() && Expired.is_terminal());
        assert!(!Initialized.is_terminal() && !Submitted.is_terminal() && !Accepted.is_terminal() && !PartiallyFilled.is_terminal() && !PendingUpdate.is_terminal() && !PendingCancel.is_terminal());
    }

    // ---- §2: client order id -- never wall-clock, distinct within one
    // ---- identical timestamp (D40) ----

    #[test]
    fn client_order_ids_are_distinct_within_the_same_simulated_instant() {
        let mut gen = ClOrdIdGen::new(7);
        let a = gen.next();
        let b = gen.next();
        assert_ne!(a, b, "two orders in the same callback (same now_ns) must still get distinct ids");
        // Session id recoverable from the upper bits, same for both.
        assert_eq!(a >> COUNTER_BITS, b >> COUNTER_BITS);
        assert_eq!(a >> COUNTER_BITS, 7);
    }

    #[test]
    fn two_identical_backtest_runs_produce_identical_client_order_ids() {
        // Simulates "same run twice": a fresh generator seeded from the
        // same injected session_id produces the exact same sequence --
        // nothing here reads a clock or any OS randomness.
        let mut run_a = ClOrdIdGen::new(42);
        let mut run_b = ClOrdIdGen::new(42);
        let ids_a: Vec<u64> = (0..5).map(|_| run_a.next()).collect();
        let ids_b: Vec<u64> = (0..5).map(|_| run_b.next()).collect();
        assert_eq!(ids_a, ids_b);
    }

    // ---- §3: three gates, Denied never reaches the venue ----

    #[test]
    fn tick_size_violation_is_denied_locally_and_never_reaches_the_venue() {
        let mut eng = engine(vec![future_instrument(1, 10, 100, 1)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(105)), qty: Lots(5) };
        let (outcome, _) = eng.submit_order(intent, 0);
        assert!(matches!(outcome, GateOutcome::Denied { reason: DenyReason::TickSize, .. }));
        assert_eq!(eng.venue_submit_calls(), 0, "a locally-denied order must never call SimExchange::submit");
        let GateOutcome::Denied { client_order_id, .. } = outcome else { unreachable!() };
        let order = eng.order(client_order_id).expect("denied order still exists in the record, per FR-B27");
        assert_eq!(order.state, OrderState::Denied);
        assert!(order.state.is_terminal());
    }

    #[test]
    fn freeze_qty_violation_is_denied_locally_and_never_reaches_the_venue() {
        let mut eng = engine(vec![future_instrument(1, 10, 50, 1)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(51) };
        let (outcome, _) = eng.submit_order(intent, 0);
        assert!(matches!(outcome, GateOutcome::Denied { reason: DenyReason::FreezeQty, .. }));
        assert_eq!(eng.venue_submit_calls(), 0);
    }

    #[test]
    fn local_otr_gate_denies_without_touching_the_venue_and_is_independent_of_the_venues_own_governor() {
        let run_config = RunConfig {
            session_id: 1,
            cost_config: CostConfig::default(),
            local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 2 },
            venue_otr: OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
            markout_horizons_ns: vec![],
        };
        let venue_otr = OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
        let mut eng = ExecutionEngine::new(run_config, vec![future_instrument(1, 1, 1000, 1)], Box::new(AlwaysAllowRms), CostConfig::default(), venue_otr, vec![], true);

        for i in 0..2u64 {
            let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100 + i as i64)), qty: Lots(1) };
            let (outcome, _) = eng.submit_order(intent, 0);
            assert!(matches!(outcome, GateOutcome::Submitted { .. }), "first two messages within the window should be admitted");
        }
        assert_eq!(eng.venue_submit_calls(), 2);

        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(103)), qty: Lots(1) };
        let (outcome, _) = eng.submit_order(intent, 0);
        assert!(matches!(outcome, GateOutcome::Denied { reason: DenyReason::LocalOtrOrRate, .. }));
        assert_eq!(eng.venue_submit_calls(), 2, "the local governor's rejection must not call the venue at all -- the venue's own OTR governor (D19) is a wholly separate mechanism");
    }

    #[test]
    fn venue_rejection_is_a_genuinely_different_terminal_state_from_denied() {
        // BOC that would cross: passes every local gate, reaches the
        // venue, and *the venue* refuses it -- Rejected, not Denied.
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        // Rest a real sell order first via a market event so the BOC buy would cross.
        eng.on_market_event(
            &DecodedMessage::OrderAdd(crate::decoder::OrderAdd { seq: 0, security_id: 1, side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(10), priority_ts: 1, event_time: 0 }),
            0,
        );
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::BookOrCancel(Price(150)), qty: Lots(5) };
        let (outcome, _) = eng.submit_order(intent, 0);
        assert!(matches!(outcome, GateOutcome::Submitted { .. }), "BOC passed every local gate -- it must reach the venue");
        assert_eq!(eng.venue_submit_calls(), 1, "unlike Denied, a venue-bound order DOES call submit");
        let GateOutcome::Submitted { client_order_id } = outcome else { unreachable!() };
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Rejected, "the venue refused it -- Rejected, not Denied");
        assert!(matches!(order.reject_reason, Some(RejectReason::WouldCross)));
        assert!(order.deny_reason.is_none(), "Rejected and Denied are different fields entirely -- this order was never Denied");
    }

    // ---- rejects and partial fills drive the state machine, not just
    // ---- the happy path ----

    #[test]
    fn partial_fill_then_remainder_fill_reaches_filled_via_partially_filled() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(10) };
        let (outcome, _) = eng.submit_order(intent, 0);
        let GateOutcome::Submitted { client_order_id } = outcome else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        // A real trade partially fills us (fallback-to-FIFO-front rule,
        // since nothing real is resting ahead of us at this price). Trade
        // quantities are wire-raw (simulator's native scale), scaled by
        // RAW_QTY_PER_LOT from the original 4/6 split against the
        // order's 10-lot (100,000 raw) requested quantity.
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 0, full: false, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(40_000), event_time: 999 }), 10);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::PartiallyFilled);
        assert_eq!(order.filled_qty, Qty(40_000));
        assert_eq!(order.leaves_qty, Qty(60_000));

        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 1, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(60_000), event_time: 999 }), 20);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Filled);
        assert_eq!(order.leaves_qty, Qty(0));
        assert!(order.state.is_terminal());
        assert_eq!(eng.fills().len(), 2, "two separate fill records, queue position/markout fields present on both from creation");
        for fill in eng.fills() {
            assert_eq!(fill.markouts.len(), 2, "markout horizons pre-populated at fill time, not added later");
            assert!(fill.markouts.iter().all(|(_, v)| v.is_none()), "values genuinely unknown yet -- only the slots exist so far");
        }
    }

    #[test]
    fn cancel_confirmation_reaches_canceled_through_pending_cancel() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        assert!(eng.request_cancel(client_order_id, 5).0);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::PendingCancel);
        assert!(eng.order(client_order_id).unwrap().state.is_open(), "still exposed while the cancel is in flight");

        eng.deliver_cancel_to_venue(client_order_id, 10);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Canceled);
        assert_eq!(order.cancel_reason, Some(CancelReason::Strategy));
    }

    // ---- THE race: PendingCancel -> Filled ----

    #[test]
    fn pending_cancel_to_filled_race_the_fill_wins_not_silently_dropped_or_double_counted() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        // 1. Submit a resting order.
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        // 2. Strategy decides to cancel -- the cancel message is "sent"
        // (state moves to PendingCancel) but has NOT yet reached the
        // venue (deliver_cancel_to_venue not called yet -- modelling the
        // outbound leg still in flight).
        assert!(eng.request_cancel(client_order_id, 100).0);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::PendingCancel);
        assert_eq!(eng.venue_cancel_calls(), 0, "the cancel has not actually reached the venue yet");

        // 3. Before our cancel arrives, a real trade fills the order at
        // the venue (it is still genuinely resting there -- SimExchange
        // has no notion of "pending cancel", only what's actually in its
        // book). This is the adversely-selected fill the race describes.
        // Trade quantity is wire-raw (simulator's native scale) -- 10
        // lots' worth (RAW_QTY_PER_LOT-scaled), fully consuming the
        // resting order in one shot.
        eng.on_market_event(
            &DecodedMessage::Trade(crate::decoder::Trade { seq: 0, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(100_000), event_time: 999_999 }),
            150,
        );
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Filled, "the fill must win over the in-flight cancel intent");
        assert_eq!(order.filled_qty, Qty(100_000));
        assert_eq!(eng.fills().len(), 1, "exactly one fill record -- not double counted");
        let strategy_position = eng.portfolio().sub_account(1).unwrap().net_position(IID);
        // `SubAccount::position` is in lots, not the fill's wire-raw
        // qty (100,000 raw = 10 lots sold, i.e. -10).
        assert_eq!(strategy_position, -10, "accounting reflects the fill (sold 10 lots) -- not silently dropped");

        // 4. Our cancel message now actually reaches the venue. Nothing
        // is resting any more (it was fully filled), so SimExchange
        // returns an empty Vec<ExecReport> for this cancel -- and the
        // order must stay exactly Filled: not regressed to Canceled,
        // not double counted, not lost.
        eng.deliver_cancel_to_venue(client_order_id, 200);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Filled, "a late, moot cancel response must not overwrite a real fill");
        assert_eq!(order.filled_qty, Qty(100_000), "still exactly one fill's worth -- the moot cancel changed nothing");
        assert_eq!(eng.fills().len(), 1, "still exactly one fill record after the moot cancel arrives");
        assert_eq!(eng.venue_cancel_calls(), 1, "the cancel message genuinely was delivered -- it just found nothing to cancel");
    }

    // ---- modify: rejected modify keeps the order working, not terminal ----

    #[test]
    fn rejected_modify_returns_to_accepted_not_terminal() {
        let run_config = RunConfig {
            session_id: 1,
            cost_config: CostConfig::default(),
            local_otr: LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
            venue_otr: OtrConfigSummary { window_ns: 1, max_messages_per_window: 1, max_otr_ratio_bits: 0 },
            markout_horizons_ns: vec![],
        };
        // Venue OTR window with a cap of exactly 1 message per (tiny)
        // window -- the new order consumes it, so the very next modify
        // message the venue sees within the same instant is rejected by
        // *its own* governor (D19), giving us a real venue-rejected
        // modify to test against, not a fabricated one.
        let venue_otr = OtrConfig { window: std::time::Duration::from_nanos(1), max_messages_per_window: 1, max_otr_ratio: 1_000_000.0 };
        let mut eng = ExecutionEngine::new(run_config, vec![future_instrument(1, 1, 1000, 1)], Box::new(AlwaysAllowRms), CostConfig::default(), venue_otr, vec![], true);

        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        assert!(eng.request_modify(client_order_id, 0).0);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::PendingUpdate);

        eng.deliver_modify_to_venue(client_order_id, Qty(20), Some(Price(101)), 0);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Accepted, "rejected modify -- order keeps working, per STRATEGY-GUIDE.md §7a");
        assert!(!order.state.is_terminal());
    }

    // ---- expiry ----

    #[test]
    fn expire_transitions_an_open_order_to_a_terminal_state() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert!(eng.mark_expired(client_order_id, 100).0);
        let order = eng.order(client_order_id).unwrap();
        assert_eq!(order.state, OrderState::Expired);
        assert!(order.state.is_terminal());
    }

    // ---- §5: two-level accounting -- sub-account skew vs firm netting ----

    #[test]
    fn firm_account_nets_across_strategies_sub_accounts_stay_independent() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        // Strategy 1 buys 10 lots (100,000 raw units).
        let i1 = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(Price(100)), qty: Lots(10) };
        let (GateOutcome::Submitted { client_order_id: c1 }, _) = eng.submit_order(i1, 0) else { panic!() };
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 0, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Buy, price: crate::decoder::Price(100), qty: crate::decoder::Qty(100_000), event_time: 999_999 }), 10);
        assert_eq!(eng.order(c1).unwrap().state, OrderState::Filled);

        // Strategy 2 sells 4 lots (40,000 raw units) of the SAME
        // instrument -- must not appear in strategy 1's own position.
        let i2 = NewOrderIntent { strategy_id: 2, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(101)), qty: Lots(4) };
        let (GateOutcome::Submitted { client_order_id: c2 }, _) = eng.submit_order(i2, 20) else { panic!() };
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 1, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(101), qty: crate::decoder::Qty(40_000), event_time: 999_998 }), 30);
        assert_eq!(eng.order(c2).unwrap().state, OrderState::Filled);

        // Position tracking (`Portfolio`/`SubAccount`) is in **lots**, not
        // the fill's wire-raw qty -- see `SubAccount::position`'s own doc
        // comment (the third Lots-vs-Qty bug found this session: this
        // used to read `100_000`/`-40_000`/`60_000`, the raw wire units,
        // here instead of the real lot counts below).
        assert_eq!(eng.portfolio().sub_account(1).unwrap().net_position(IID), 10, "strategy 1 sees only its own fill (10 lots bought)");
        assert_eq!(eng.portfolio().sub_account(2).unwrap().net_position(IID), -4, "strategy 2 sees only its own fill (4 lots sold), never strategy 1's");

        let firm = eng.portfolio().firm();
        assert_eq!(firm.position.get(&IID).copied().unwrap_or(0), 6, "firm nets both strategies together: 10 - 4 = 6 lots");
    }

    // ---- realised P&L: lots * instrument.multiplier, not raw wire qty
    // ---- (the third Lots-vs-Qty bug, found and fixed this session) ----

    #[test]
    fn realized_pnl_uses_lots_and_instrument_multiplier_not_raw_wire_qty() {
        // Reproduces the exact real-data numbers that exposed the bug
        // (see execution_user_doc.md §11 / dummy_strategy.md): 1 lot of a
        // CRUDEOIL-shaped instrument (multiplier=100, e.g. barrels/lot)
        // bought at Rs 5424.00 and sold at Rs 5421.00. Real P&L = price
        // diff * lots * multiplier = (5421 - 5424) * 1 * 100 = **-300**.
        // The pre-fix bug instead computed (5421 - 5424) * 10_000 (the
        // fill's raw wire qty for "1 lot", with no multiplier at all) =
        // -30,000 -- neither the right magnitude nor the right factor,
        // which is exactly why this needs a real instrument multiplier
        // in the test, not just qty=1 with multiplier=1.
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 100)]);
        let price_of = |rupees: f64| Price((rupees * RAW_PRICE_SCALE) as i64);

        let buy = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Buy, order_type: OrderType::LimitDay(price_of(5424.0)), qty: Lots(1) };
        let (GateOutcome::Submitted { client_order_id: c1 }, _) = eng.submit_order(buy, 0) else { panic!() };
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 0, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Buy, price: crate::decoder::Price(price_of(5424.0).0), qty: crate::decoder::Qty(10_000), event_time: 1 }), 1);
        assert_eq!(eng.order(c1).unwrap().state, OrderState::Filled);
        assert_eq!(eng.portfolio().sub_account(1).unwrap().net_position(IID), 1, "1 lot bought");

        let sell = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(price_of(5421.0)), qty: Lots(1) };
        let (GateOutcome::Submitted { client_order_id: c2 }, _) = eng.submit_order(sell, 2) else { panic!() };
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 1, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(price_of(5421.0).0), qty: crate::decoder::Qty(10_000), event_time: 3 }), 3);
        assert_eq!(eng.order(c2).unwrap().state, OrderState::Filled);

        let sub = eng.portfolio().sub_account(1).unwrap();
        assert_eq!(sub.net_position(IID), 0, "round trip flattens the position");
        assert!(
            (sub.realized_pnl - (-300.0)).abs() < 1e-6,
            "real P&L = (5421 - 5424) rupees/lot * 1 lot * 100 multiplier = -300, got {}",
            sub.realized_pnl
        );
    }

    // ---- §4: cost model asymmetry -- concrete numbers, not just a Side
    // ---- parameter that compiles ----

    #[test]
    fn cost_model_buy_and_sell_round_trip_costs_concretely_differ() {
        let instrument = future_instrument(1, 1, 1000, 1);
        let cfg = CostConfig::default();
        let model = CostModel::new(cfg);
        let price = Price(500_000_00_00); // Rs 500.00 in raw wire units (RAW_PRICE_SCALE)
        let qty = Lots(10);

        let buy = model.round_trip(&instrument, qty, price, Side::Buy);
        let sell = model.round_trip(&instrument, qty, price, Side::Sell);

        // Buy pays stamp duty, zero CTT; sell pays CTT, zero stamp duty.
        assert!(buy.stamp_duty > 0.0);
        assert_eq!(buy.ctt, 0.0);
        assert!(sell.ctt > 0.0);
        assert_eq!(sell.stamp_duty, 0.0);

        // The concrete totals genuinely differ (ctt_rate != stamp_duty_rate
        // in the placeholder config -- 0.01% vs 0.002%).
        assert_ne!(buy.total_rupees, sell.total_rupees, "buy-side and sell-side round-trip costs must concretely differ, not merely accept a Side parameter");
        println!("buy round-trip cost:  Rs {:.4} (stamp_duty=Rs {:.4}, ctt=Rs {:.4})", buy.total_rupees, buy.stamp_duty, buy.ctt);
        println!("sell round-trip cost: Rs {:.4} (stamp_duty=Rs {:.4}, ctt=Rs {:.4})", sell.total_rupees, sell.stamp_duty, sell.ctt);

        // With this placeholder config (ctt_rate 0.0001 > stamp_duty_rate
        // 0.00002), a sell round trip costs strictly more than a buy one
        // for identical qty/price -- pin the direction, not just "differ".
        assert!(sell.total_rupees > buy.total_rupees);
    }

    #[test]
    fn cost_is_queryable_pretrade_and_the_same_function_is_applied_to_the_realised_fill() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let instrument = eng.instrument(IID).unwrap().clone();
        let price = Price(500_000_00_00);
        let qty = Lots(10);

        // Pre-trade query, exactly the STRATEGY-GUIDE.md §9 shape.
        let pre_trade_cost = eng.cost_model().round_trip(&instrument, qty, price, Side::Sell);

        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(price), qty };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        // Real trade quantity in wire-raw units (`qty.to_raw_qty()`) so
        // this trade genuinely fully fills the resting order; `on_fill`
        // converts it back to `Lots` via `Qty::to_lots()` before calling
        // `round_trip` again, so the realised cost below must equal
        // `pre_trade_cost` exactly.
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 0, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(price.0), qty: crate::decoder::Qty(qty.to_raw_qty().0), event_time: 999_999 }), 10);
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Filled);

        let realised_fill = &eng.fills()[0];
        assert_eq!(realised_fill.cost.total_rupees, pre_trade_cost.total_rupees, "same CostModel::round_trip call, same qty/price/side -- pre-trade assumption and realised accounting cannot disagree");
    }

    // ---- §7: reporting -- run identity + queue position / markout
    // ---- presence on fill records ----

    #[test]
    fn tier1_report_embeds_a_run_identity_and_prints_it() {
        let eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let report = eng.tier1_report();
        assert_eq!(report.run_identity.build_hash, BUILD_HASH);
        let printed = format!("{report}");
        assert!(printed.contains("run identity"));
        assert!(printed.contains(BUILD_HASH));
        assert!(printed.contains(&format!("{:#018x}", report.run_identity.config_hash)));
        println!("{printed}");
    }

    #[test]
    fn two_engines_built_from_the_identical_run_config_hash_identically() {
        let eng_a = engine(vec![future_instrument(1, 1, 1000, 1)]);
        let eng_b = engine(vec![future_instrument(1, 1, 1000, 1)]);
        assert_eq!(eng_a.run_identity().config_hash, eng_b.run_identity().config_hash, "same [run]-shaped config must hash identically -- FR-12's determinism requirement");
    }

    #[test]
    fn queue_position_and_markout_fields_exist_on_every_fill_from_creation() {
        let mut eng = engine(vec![future_instrument(1, 1, 1000, 1)]);
        // Rest behind two real resting sell orders so this fill is
        // genuinely passive with a non-trivial queue position.
        eng.on_market_event(&DecodedMessage::OrderAdd(crate::decoder::OrderAdd { seq: 0, security_id: 1, side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(10), priority_ts: 1, event_time: 0 }), 0);
        let intent = NewOrderIntent { strategy_id: 1, instrument: IID, side: Side::Sell, order_type: OrderType::LimitDay(Price(150)), qty: Lots(5) };
        let (GateOutcome::Submitted { client_order_id }, _) = eng.submit_order(intent, 0) else { panic!() };
        assert_eq!(eng.order(client_order_id).unwrap().state, OrderState::Accepted);

        // Real trade consumes the resting order ahead of us (raw qty 10,
        // unrelated to our own order's lot scale), then reaches us: a
        // Passive fill with a genuine pre-fill queue position of 10. The
        // second trade's quantity is wire-raw, equal to our own order's
        // requested raw quantity (5 lots), so it fully fills us.
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 1, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(10), event_time: 1 }), 5);
        eng.on_market_event(&DecodedMessage::Trade(crate::decoder::Trade { seq: 2, full: true, security_id: 1, aggressor_side: crate::decoder::Side::Sell, price: crate::decoder::Price(150), qty: crate::decoder::Qty(intent.qty.to_raw_qty().0), event_time: 999_999 }), 6);

        assert_eq!(eng.fills().len(), 1);
        let fill = &eng.fills()[0];
        assert_eq!(fill.queue_position_at_fill, Some(10), "genuine pre-fill queue position captured, not fabricated");
        assert_eq!(fill.markouts.len(), 2, "both configured horizons present on the record from the moment it was created");
        assert!(fill.markouts.iter().all(|(_, v)| v.is_none()));
        let fill_id = fill.fill_id;

        // Now observe both horizons.
        eng.observe_markout(fill_id, 1_000_000, Price(151));
        eng.observe_markout(fill_id, 5_000_000, Price(148));
        let fill = &eng.fills()[0];
        // Sold at 150; mid moved to 151 (against us: -1) then 148 (favourable: +2).
        assert_eq!(fill.markouts[0], (1_000_000, Some(-1)));
        assert_eq!(fill.markouts[1], (5_000_000, Some(2)));
    }
}

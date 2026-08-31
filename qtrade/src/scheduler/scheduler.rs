//! The scheduler component: the priority-queue event loop and `SimClock`
//! that make a backtest reproducible. Everything here follows
//! BACKTEST-PHASE1.md §M4 (FR-B12 - FR-B15), ARCHITECTURE.md §4.6/§5.1,
//! and ARCHITECTURE-DECISIONS.md D04/D30. See `scheduler_user_doc.md` in
//! this folder for the full explanation; this file's comments cover the
//! "why" behind specific lines, not the whole design.
//!
//! ## Scope (deliberately out)
//!
//! No Cache, no dispatch-to-strategy logic (T05), no latency model
//! (T06). This is the loop, the clock, and the priority queue -- nothing
//! else. Dispatch (matching a popped `Event` against what it means and
//! calling the right thing) lives entirely in `main.rs`'s own loop, not
//! here -- same D07 principle EventDispatcher/ControlDispatcher already
//! follow ("routing knowledge lives in startup wiring, never inside
//! either dispatcher").
//!
//! **One deliberate exception, added 2026-08-27** (dual-clock replay):
//! `EventPayload::MarketData` carries a real `decoder::DecodedMessage` so
//! it's actually useful to `main.rs`'s dispatch -- the one dependency
//! this module takes on, still well below `cache`/`execution`/
//! `simulator` in the stack. `OrderArrival`/`ReportDelivery` stay opaque
//! (`op_id: u64`) precisely to avoid needing `execution`/`simulator`
//! types here too -- `main.rs` interprets the id against its own pending-
//! ops table; this module never does.
//!
//! ## Convention carried over from `decoder`
//!
//! `Debug` (`{:?}`) is derived everywhere, no exceptions. `Display`
//! (`{}`) is hand-written only on the types a person actually reads --
//! here, that's every public type, since the whole point of this
//! module's tests is to print a human-readable dispatch log and diff it
//! between two runs.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fmt;

use crate::decoder::DecodedMessage;

// ---------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------

/// Simulation time, nanoseconds on a monotonic axis -- the same kind of
/// value `decoder`'s `capture_ts` carries (see decoder/user_doc.md §3.1).
///
/// **This is never a wall-clock/epoch value by itself** (D30). It only
/// means "earlier/later than another `Timestamp` produced in the same
/// run". Two different runs, or a run and a live session, do not agree
/// on what wall-clock instant a given `Timestamp` corresponds to unless
/// something else (a per-session wall-clock anchor, per D30) says so --
/// and nothing in this module does that mapping. Anyone wanting "what
/// time did this happen in the real world" needs a component that isn't
/// this one.
pub type Timestamp = i64;

/// Owns simulated time. `now()` returns the timestamp of whichever event
/// is currently being dispatched; nothing else may change it (FR-B12,
/// FR-B15). Never sleeps, never performs a real-time wait -- backtest
/// time is free, and wall-clock runtime spent running the loop is
/// throughput, not a quantity this clock reports (D30, ARCHITECTURE.md
/// §5.4: "In Backtest Mode time is free").
#[derive(Debug, Default)]
pub struct SimClock {
    now: Option<Timestamp>,
}

impl SimClock {
    /// A clock that has not yet observed an event. Calling `now()` before
    /// the first `set()` is a programming error (see `now()` below), not
    /// a silently-defaulted zero -- a silent default would be exactly
    /// the kind of unannounced time-travel D30's invariant forbids.
    pub fn new() -> Self {
        Self { now: None }
    }

    /// Advance the clock to `t`. This is the **only** place simulated
    /// time is allowed to change (FR-B12's `clock.set(event.timestamp)`).
    ///
    /// Panics if `t` would move time backward. This is not defensive
    /// paranoia: ARCHITECTURE.md §4.6 states the invariant in exactly
    /// these words ("Time only moves forward"), and a scheduler that let
    /// it slide would corrupt every downstream equality assertion this
    /// milestone exists to make possible (FR-B15).
    pub fn set(&mut self, t: Timestamp) {
        if let Some(prev) = self.now {
            assert!(
                t >= prev,
                "SimClock invariant violated: time only moves forward \
                 (attempted to set {t} after {prev})"
            );
        }
        self.now = Some(t);
    }

    /// The current event's timestamp.
    ///
    /// Panics if called before the loop has dispatched a first event --
    /// there is no meaningful "current time" yet, and returning 0 would
    /// silently misrepresent the start of time as a real event.
    pub fn now(&self) -> Timestamp {
        self.now
            .expect("SimClock::now() called before any event was dispatched")
    }

    /// Whether any event has been dispatched yet.
    pub fn is_started(&self) -> bool {
        self.now.is_some()
    }
}

// ---------------------------------------------------------------------
// Ordering key: (timestamp, event_class, monotonic_seq) -- FR-B13
// ---------------------------------------------------------------------

/// The second component of the ordering key (FR-B13). Ranks *why* an
/// event exists, so that when two events land on the identical
/// timestamp -- FR-B13 is explicit that this is routine, not a corner
/// case -- there is still a defined winner.
///
/// **Rank, ascending = dispatched first, and the rule behind it:**
/// exogenous facts about the world (what a venue published, or the
/// venue's own declared state, or the system noticing an *absence* of
/// data) outrank endogenous facts (something qtrade itself scheduled to
/// happen later). A strategy must never see its own pending action as
/// if it had landed before real market activity at the same instant --
/// this is the literal scenario ARCHITECTURE.md §5.4 walks through: real
/// participants arriving in the latency window are processed *before*
/// the self-generated `OrderArrival` they raced against. That worked
/// trace is the reason for this variant order; it is not arbitrary.
///
/// Declared in increasing discriminant order on purpose -- `derive(Ord)`
/// on a fieldless enum compares by declaration order, and pinning
/// explicit discriminants here means a future reordering of the `enum`
/// body for readability can't silently change the tie-break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EventClass {
    /// A message from the venue via the Sequencer (FR-B14). The ground
    /// truth everything else reacts to; must land before anything
    /// reacting to it at the same timestamp.
    MarketData = 0,
    /// Venue-declared session state (pre-open/continuous/auction/close,
    /// ARCHITECTURE.md §5.2) -- exogenous, same tier as market data.
    SessionTransition = 1,
    /// A fill already happened at the venue; this is qtrade learning
    /// about it at `fill_time + inbound latency` (§5.4). Exogenous fact,
    /// ranked ahead of qtrade's own pending actions.
    ReportDelivery = 2,
    /// Fires when *nothing* arrives (FR-B14) -- an absence is still a
    /// fact about the world, not something qtrade scheduled for itself.
    StalenessOrHeartbeatTimeout = 3,
    /// Declared-dependency staleness detection (D28) -- same tier as
    /// other "the system noticed a fact" sources.
    WatchdogExpiry = 4,
    /// qtrade's own order becoming visible to the venue at
    /// `submit_time + outbound latency` (§5.4). Endogenous: this only
    /// exists because qtrade decided to submit something.
    OrderArrival = 5,
    /// A strategy's own `set_timer`/`set_alarm` firing. Endogenous.
    StrategyTimer = 6,
    /// An expensive strategy computation's result returning as a
    /// scheduled event (D04's offload mechanism) rather than a blocking
    /// call. Endogenous, and last: it is qtrade's own background work
    /// completing, not anything the market or the venue did.
    OffloadCompletion = 7,
}

impl fmt::Display for EventClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EventClass::MarketData => "MARKET_DATA",
            EventClass::SessionTransition => "SESSION_TRANSITION",
            EventClass::ReportDelivery => "REPORT_DELIVERY",
            EventClass::StalenessOrHeartbeatTimeout => "STALENESS_TIMEOUT",
            EventClass::WatchdogExpiry => "WATCHDOG_EXPIRY",
            EventClass::OrderArrival => "ORDER_ARRIVAL",
            EventClass::StrategyTimer => "STRATEGY_TIMER",
            EventClass::OffloadCompletion => "OFFLOAD_COMPLETION",
        };
        write!(f, "{s}")
    }
}

// ---------------------------------------------------------------------
// Payload -- deliberately thin (out of scope: dispatch-to-strategy logic,
// latency model). This phase only needs to synthesize market-data-like
// and timer-like test events, and to prove the event *shape* can carry
// "fires at T + latency" generically for sources nothing feeds yet.
// ---------------------------------------------------------------------

/// Which independent consumer a `MarketData` delivery is for -- the dual-
/// clock replay's own split (2026-08-27): the same real message reaches
/// `SimExchange` and `Cache` at two different scheduled times (`exchange_ts`
/// and `recorder_ts` respectively), never through a read path between
/// them (D10 stays intact). One `EventClass::MarketData` covers both --
/// they essentially never land on the identical timestamp in practice
/// (verified against real data: the delta is never zero), so they don't
/// need a tie-break relationship with each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    SimExchange,
    Cache,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::SimExchange => write!(f, "SimExchange"),
            Target::Cache => write!(f, "Cache"),
        }
    }
}

/// What an event actually carries. `MarketData` and `OrderArrival`/
/// `ReportDelivery` are real, wired producers (dual-clock replay,
/// 2026-08-27) driven by `main.rs`. `StrategyTimer` is still a
/// synthesized stand-in for a strategy's `set_timer`/`set_alarm` --
/// nothing calls it yet. `SessionTransition`/`StalenessOrHeartbeatTimeout`/
/// `WatchdogExpiry`/`OffloadCompletion` remain shape-only placeholders,
/// waiting on components not yet built.
#[derive(Debug, Clone)]
pub enum EventPayload {
    /// One decoded real message, destined for one of the two independent
    /// consumers. `seq_no`/`exchange_ts`/`recorder_ts` are carried
    /// alongside `message` purely for logging/debugging -- the event's
    /// own `timestamp` (whichever of the two this delivery was scheduled
    /// at) is what actually drives dispatch.
    MarketData { target: Target, message: DecodedMessage, seq_no: u64, exchange_ts: u64, recorder_ts: i64 },
    /// A synthesized stand-in for a strategy's `set_timer`/`set_alarm`.
    StrategyTimer { label: &'static str },
    /// "This order (or cancel/modify delivery) becomes visible to the
    /// venue at T + latency." `op_id` is deliberately opaque -- it is
    /// never a `client_order_id` directly (a cancel/modify delivery isn't
    /// "a new order"), it's a key into `main.rs`'s own pending-ops table,
    /// which this module never reads.
    OrderArrival { op_id: u64 },
    /// "A fill/order-update is learned at T + latency." Same opaque-`op_id`
    /// convention as `OrderArrival` -- `main.rs` looks up what was
    /// actually produced and hands it to `ControlDispatcher::dispatch`.
    ReportDelivery { op_id: u64 },
    /// Shape-only: venue session-state change.
    SessionTransition { session: &'static str },
    /// Shape-only: silence past threshold (FR-B14, FR-02).
    StalenessTimeout,
    /// Shape-only: declared-dependency staleness (D28).
    WatchdogExpiry,
    /// Shape-only: offload mechanism result (D04).
    OffloadCompletion,
}

impl fmt::Display for EventPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventPayload::MarketData { target, seq_no, exchange_ts, recorder_ts, .. } => {
                write!(f, "MarketData(target={target}, seq={seq_no}, exchange_ts={exchange_ts}, recorder_ts={recorder_ts})")
            }
            EventPayload::StrategyTimer { label } => write!(f, "StrategyTimer({label})"),
            EventPayload::OrderArrival { op_id } => write!(f, "OrderArrival(op_id={op_id})"),
            EventPayload::ReportDelivery { op_id } => write!(f, "ReportDelivery(op_id={op_id})"),
            EventPayload::SessionTransition { session } => write!(f, "SessionTransition({session})"),
            EventPayload::StalenessTimeout => write!(f, "StalenessTimeout"),
            EventPayload::WatchdogExpiry => write!(f, "WatchdogExpiry"),
            EventPayload::OffloadCompletion => write!(f, "OffloadCompletion"),
        }
    }
}

/// One entry in the queue. `seq` is assigned once, at enqueue time
/// (`Scheduler::schedule`), and never changes afterward -- it is the
/// third and final component of FR-B13's ordering key, and the reason
/// two events sharing `(timestamp, event_class)` still have a defined
/// winner: whichever was enqueued first, wins. Since qtrade is
/// single-threaded (D04), "enqueued first" is itself a deterministic,
/// run-to-run-identical fact, not a race.
#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp: Timestamp,
    pub event_class: EventClass,
    pub seq: u64,
    pub payload: EventPayload,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "t={:<20} class={:<19} seq={:<6} {}",
            self.timestamp, self.event_class, self.seq, self.payload
        )
    }
}

/// Ordering compares only the three-part key `(timestamp, event_class,
/// seq)` -- FR-B13, verbatim. `payload` deliberately does not
/// participate: it carries data, not ordering information, and two
/// events that are identical in every ordering-relevant respect but
/// differ in payload should still compare equal for queue purposes
/// (this never actually happens in practice, since `seq` is unique per
/// enqueue, but the impl only compares what the ordering key says to
/// compare).
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key() == other.sort_key()
    }
}
impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl Event {
    fn sort_key(&self) -> (Timestamp, EventClass, u64) {
        (self.timestamp, self.event_class, self.seq)
    }
}

// ---------------------------------------------------------------------
// The priority queue itself
// ---------------------------------------------------------------------

/// A priority queue of `Event`s ordered by `(timestamp, event_class,
/// seq)` -- earliest first. Backed by `std::collections::BinaryHeap`,
/// which is a max-heap; events are pushed wrapped in `Reverse` so the
/// heap's max (by `Reverse`'s inverted `Ord`) is the queue's *earliest*
/// event, giving `pop_earliest()` its name honestly.
#[derive(Debug, Default)]
pub struct Scheduler {
    heap: BinaryHeap<Reverse<Event>>,
    next_seq: u64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            next_seq: 0,
        }
    }

    /// Enqueue `payload`, to fire at `timestamp`, classified as
    /// `event_class`. Assigns and returns the `seq` this event was given
    /// -- the third component of the ordering key, fixed for the life of
    /// the event.
    ///
    /// `timestamp` may be at, or after, the current clock reading --
    /// this is deliberately how "fires at T + latency" is represented
    /// (FR-B14): the caller (eventually `simulator`, T06) computes
    /// `T + latency` and schedules there. Nothing about this method
    /// distinguishes "fires now" from "fires later"; the queue and the
    /// clock make that distinction automatically, by ordering.
    pub fn schedule(&mut self, timestamp: Timestamp, event_class: EventClass, payload: EventPayload) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.heap.push(Reverse(Event {
            timestamp,
            event_class,
            seq,
            payload,
        }));
        seq
    }

    /// Remove and return the earliest event by `(timestamp, event_class,
    /// seq)`, or `None` if the queue is empty (FR-B12's loop-exit
    /// condition).
    pub fn pop_earliest(&mut self) -> Option<Event> {
        self.heap.pop().map(|Reverse(e)| e)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// The earliest queued event's timestamp, without removing it --
    /// added for `main.rs`'s own interleaved read-then-drain loop (dual-
    /// clock replay, 2026-08-27): safe to drain everything strictly
    /// before the next unread file record's `exchange_ts` (exchange_ts is
    /// monotonic non-decreasing across one venue's stream, D05), but only
    /// once that next record has actually been read and its own
    /// `exchange_ts` is known -- this is what lets the caller check
    /// "is it safe yet" without popping and needing to push back.
    pub fn peek_earliest_timestamp(&self) -> Option<Timestamp> {
        self.heap.peek().map(|Reverse(e)| e.timestamp)
    }

    /// FR-B12's loop, verbatim: pop earliest, advance the clock to it
    /// (the *only* place time moves), dispatch. `dispatch` receives
    /// `&mut Scheduler` and `&SimClock` so a handler can enqueue further
    /// events (a timer rearming itself, a market event triggering an
    /// order that arrives later) -- exactly "handlers may enqueue more"
    /// from the loop shape in the task brief and in ARCHITECTURE.md
    /// §5.1.
    ///
    /// This is intentionally the full extent of "dispatch" this
    /// component does: `dispatch` is a caller-supplied closure, not a
    /// strategy-facing dispatcher (that's T05's Cache/dispatch
    /// component, explicitly out of scope here).
    pub fn run(&mut self, clock: &mut SimClock, mut dispatch: impl FnMut(&mut Scheduler, &SimClock, &Event)) {
        loop {
            let Some(event) = self.pop_earliest() else { break };
            clock.set(event.timestamp);
            dispatch(self, clock, &event);
        }
    }
}

// ---------------------------------------------------------------------
// Tests -- FR-B15's acceptance bar: replay the same input twice, assert
// byte-identical dispatch output. Run with:
//   rustc --edition 2021 --test scheduler.rs -o /tmp/scheduler_test
//   ./tmp/scheduler_test --nocapture
// (or, once wired into main.rs, `cargo test`).
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::{DecodedMessage, UnknownMessage};

    /// A lightweight, distinguishable `DecodedMessage` for tests that
    /// only care about scheduling/ordering, not decode content --
    /// `Unknown` is the decoder's own "seen, no verified layout" variant,
    /// perfect for this since these tests never inspect the message
    /// itself, only which event carried it and when.
    fn dummy_message(tag: u16) -> DecodedMessage {
        DecodedMessage::Unknown(UnknownMessage { seq: tag as u32, template_id: tag, body_len: 0 })
    }

    fn dummy_market_data(target: Target, tag: u16) -> EventPayload {
        EventPayload::MarketData { target, message: dummy_message(tag), seq_no: tag as u64, exchange_ts: 0, recorder_ts: 0 }
    }

    /// Schedules a fixed, hand-written sequence of events -- market data
    /// and strategy timers, the two sources this phase synthesizes test
    /// events for (task brief) -- then runs the FR-B12 loop, recording
    /// one formatted line per dispatched event. One `MarketData` dispatch
    /// (`sequence == 2`) also schedules a follow-up `OrderArrival` 250
    /// (simulated) nanoseconds later, purely to prove the event shape
    /// can carry "fires at T + latency" generically (task requirement
    /// 4) and that a handler enqueuing more events during dispatch is
    /// itself deterministic. No latency model computes the 250ns offset
    /// -- it is a literal in the test, standing in for what `simulator`
    /// will compute for real in T06.
    ///
    /// Returns the full log as one `String`, plus the final clock
    /// reading, so callers can compare both across runs.
    fn run_fixed_sequence() -> (String, Timestamp) {
        let mut scheduler = Scheduler::new();
        let mut clock = SimClock::new();
        let mut log = String::new();

        // Enqueued in this exact order, every run.
        scheduler.schedule(1_000, EventClass::MarketData, dummy_market_data(Target::Cache, 1));
        scheduler.schedule(1_000, EventClass::StrategyTimer, EventPayload::StrategyTimer { label: "quote_refresh" });
        scheduler.schedule(1_500, EventClass::MarketData, dummy_market_data(Target::Cache, 2));
        scheduler.schedule(2_000, EventClass::StrategyTimer, EventPayload::StrategyTimer { label: "risk_check" });
        scheduler.schedule(2_000, EventClass::MarketData, dummy_market_data(Target::Cache, 3));

        scheduler.run(&mut clock, |sched, clk, event| {
            log.push_str(&format!("now={:<20} dispatch: {}\n", clk.now(), event));
            if let EventPayload::MarketData { seq_no: 2, .. } = event.payload {
                // Handler enqueues a follow-up event at T + 250 -- the
                // "fires at T + latency" shape, exercised for real.
                sched.schedule(event.timestamp + 250, EventClass::OrderArrival, EventPayload::OrderArrival { op_id: 42 });
            }
        });

        (log, clock.now())
    }

    #[test]
    fn determinism_two_runs_are_byte_identical() {
        let (log_a, clock_a) = run_fixed_sequence();
        let (log_b, clock_b) = run_fixed_sequence();

        println!("--- run A ---\n{log_a}");
        println!("--- run B ---\n{log_b}");
        println!("final clock A={clock_a} B={clock_b}");

        assert_eq!(log_a, log_b, "two runs of the same input diverged");
        assert_eq!(clock_a, clock_b);

        // Pin the exact expected dispatch order too, not just A==B -- so
        // a change that breaks the documented tie-break (but happens to
        // still be internally consistent between two runs of *this*
        // build) still fails the test. Note the t=1750 `OrderArrival`,
        // enqueued *during* the t=1500 dispatch (250ns of "latency"
        // ahead), pops before the t=2000 events that were enqueued
        // earlier but fire later -- the priority queue doing its job,
        // not insertion order.
        let dispatched_order: Vec<&str> = log_a.lines().collect();
        assert_eq!(dispatched_order.len(), 6);
        assert!(dispatched_order[0].contains("seq=0") && dispatched_order[0].contains("MARKET_DATA"));
        assert!(dispatched_order[1].contains("seq=1") && dispatched_order[1].contains("STRATEGY_TIMER"));
        assert!(dispatched_order[2].contains("seq=2") && dispatched_order[2].contains("MARKET_DATA"));
        assert!(dispatched_order[3].contains("now=1750") && dispatched_order[3].contains("ORDER_ARRIVAL"));
        assert!(dispatched_order[4].contains("now=2000") && dispatched_order[4].contains("MARKET_DATA") && dispatched_order[4].contains("seq=4"));
        assert!(dispatched_order[5].contains("now=2000") && dispatched_order[5].contains("STRATEGY_TIMER") && dispatched_order[5].contains("seq=3"));
    }

    /// FR-B13: "ties are guaranteed, not hypothetical". Constructs two
    /// events sharing the exact same `(timestamp, event_class)` and
    /// checks the documented tie-break -- enqueue order via `seq` --
    /// decides the winner, every run. Also constructs a same-timestamp,
    /// *different*-class tie to check the class ranking (exogenous
    /// before endogenous) wins over enqueue order, per `EventClass`'s
    /// doc comment.
    fn run_tie_break_case() -> String {
        let mut scheduler = Scheduler::new();
        let mut clock = SimClock::new();
        let mut log = String::new();

        // Same timestamp, same class (MarketData), enqueued B before A.
        // seq alone must decide: B (seq=0) dispatches before A (seq=1).
        scheduler.schedule(5_000, EventClass::MarketData, dummy_market_data(Target::Cache, 100)); // "B", seq=0
        scheduler.schedule(5_000, EventClass::MarketData, dummy_market_data(Target::Cache, 101)); // "A", seq=1

        // Same timestamp, OrderArrival enqueued *before* MarketData.
        // Enqueue order alone would fire OrderArrival first (seq=2 <
        // seq=3) -- the class rank must override that: MarketData (an
        // exogenous fact) still dispatches first, even though it was
        // enqueued second.
        scheduler.schedule(6_000, EventClass::OrderArrival, EventPayload::OrderArrival { op_id: 7 }); // seq=2
        scheduler.schedule(6_000, EventClass::MarketData, dummy_market_data(Target::Cache, 200)); // seq=3

        scheduler.run(&mut clock, |_sched, clk, event| {
            log.push_str(&format!("now={} seq={} class={} {}\n", clk.now(), event.seq, event.event_class, event.payload));
        });

        log
    }

    #[test]
    fn ties_resolve_deterministically_and_repeatably() {
        let run_1 = run_tie_break_case();
        let run_2 = run_tie_break_case();

        println!("--- tie-break run 1 ---\n{run_1}");
        println!("--- tie-break run 2 ---\n{run_2}");

        assert_eq!(run_1, run_2, "tie-break diverged between two runs of the same input");

        let lines: Vec<&str> = run_1.lines().collect();
        assert_eq!(lines.len(), 4);
        // t=5000: same class -> seq order (seq=0 before seq=1).
        assert!(lines[0].contains("seq=0"), "expected lower seq to win an in-class tie, got: {}", lines[0]);
        assert!(lines[1].contains("seq=1"));
        // t=6000: MarketData (seq=3) must beat OrderArrival (seq=2)
        // despite being enqueued second -- class rank overrides seq.
        assert!(
            lines[2].contains("seq=3") && lines[2].contains("MARKET_DATA"),
            "expected MarketData to win the cross-class tie despite later enqueue, got: {}",
            lines[2]
        );
        assert!(lines[3].contains("seq=2") && lines[3].contains("ORDER_ARRIVAL"));
    }

    #[test]
    #[should_panic(expected = "SimClock invariant violated")]
    fn clock_rejects_moving_backward() {
        let mut clock = SimClock::new();
        clock.set(100);
        clock.set(99); // must panic -- time only moves forward (D30, §4.6)
    }

    #[test]
    #[should_panic(expected = "called before any event was dispatched")]
    fn clock_now_before_start_panics_instead_of_defaulting() {
        let clock = SimClock::new();
        let _ = clock.now();
    }

    #[test]
    fn pop_earliest_on_empty_queue_is_none_and_ends_the_loop() {
        let mut scheduler = Scheduler::new();
        assert!(scheduler.pop_earliest().is_none());
        assert!(scheduler.is_empty());
    }
}

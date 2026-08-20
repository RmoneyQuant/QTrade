# Trading Engine — Architecture

**Status:** Design complete for phase 1. No code written.
**Last updated:** 2026-08-19

**Companion documents**
- [CONTEXT.md](CONTEXT.md) — glossary. Terms below are used as defined there.
- [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md) — the 40 decisions and their reasoning. Referenced here as **D01**–**D40**.
- [OPEN-QUESTIONS.md](OPEN-QUESTIONS.md) — what remains unresolved.
- [BACKTEST-PHASE1.md](BACKTEST-PHASE1.md) — implementation spec for the first backtest.
- [STRATEGY-GUIDE.md](STRATEGY-GUIDE.md) — how to write a strategy against the `Strategy` trait.

---

## 1. Purpose and scope

### 1.1 What this system is

An engine that runs **the same compiled code** in Backtest Mode and Live Mode. Only two thin edges swap: where market data bytes come from, and where orders go. Everything between — decoding, book building, strategy logic, execution, accounting — is one artefact, exercised identically in both modes.

qtrade is **multi-venue and exchange-agnostic**. All venue-specific knowledge lives in Feed Adapters at the edge.

### 1.2 Phase 1

**Market making in MCX Crude Oil and Natural Gas futures**, priced from CME and a USD/INR rate.

| Venue | Role | Book | Depth | Adapter |
|---|---|---|---|---|
| **MCX** | Trading venue — we quote here | **MBO** | full | MCX T7 EOBI (direct multicast) |
| **CME** | Price signal | MBP | 5 levels | Quincy QED |
| **DGCX / GME** | USD/INR signal | MBP | 1 (BBO) | Quincy QED |

Two Feed Adapters, three venues — a Feed Adapter is defined by provider and protocol, not by venue (**D32**, **D27**).

### 1.3 Out of scope for phase 1

NSE (deferred, protocol facts already recorded) · options market making (deferred; the architecture accepts pricing and volatility plug-ins) · the generic risk framework (**D17**) · NUMA and CPU-core pinning · kernel bypass (both committed to a later phase, **D02**) · stop and auction order types · iceberg orders · quote/mass-quote interfaces (not offered by MCX ETI).

---

## 2. Requirements

### 2.1 Functional requirements

Each requirement below states **why it exists**, **what must be true**, and **how to verify it**. The *Refs* line points at the decision that produced it.

---

#### FR-01 — Heartbeat sequence reconciliation

**Context.** MCX T7 EOBI emits a `Heartbeat (13001)` when a stream has been idle, and it carries the **last sequence number sent on that stream**. This is the only mechanism that detects a gap at the *tail* of a burst. If the final messages before a quiet period are lost, no subsequent data message reveals it — qtrade would hold a silently incomplete book, possibly for minutes, and quote against it.

**Requirement.** On every heartbeat, compare the heartbeat's last-sequence field against the highest sequence received on that stream. Any discrepancy is a detected gap.

**Behaviour.**
1. Compute `missing = heartbeat.last_seq − highest_seq_received`.
2. If `missing > 0`: mark every book fed by that stream `STALE`, emit a gap alert on the Control Dispatcher, and initiate recovery (snapshot channel, or retransmission).
3. Books remain `STALE` until recovery completes and convergence is confirmed.
4. If `missing < 0` — the heartbeat reports an *older* sequence than received — treat as a DR switchover or feed reset, not a gap; log and re-evaluate stream identity.

**Acceptance.** Replay a recording with a synthetic tail gap immediately preceding a heartbeat. Assert: gap detected at the heartbeat rather than at the next data message; affected books `STALE`; recovery initiated; quoting blocked throughout (FR-15).

**Refs.** D05, D16 · MCX EOBI §3.4

---

#### FR-02 — Silence detection

**Context.** The dangerous failure is not a gap — it is **receiving nothing at all** while believing the market is quiet. A dead multicast subscription, a failed NIC, or a switch misconfiguration all present identically to a genuinely idle instrument. Because EOBI heartbeats arrive during idle periods, *the absence of both data and heartbeat is diagnostic*.

**Requirement.** For each subscribed stream, if neither a data message nor a heartbeat arrives within a configurable threshold, raise a **critical** alert and mark affected books `STALE`.

**Behaviour.**
1. Each stream carries a per-stream silence timer, reset on any received message including heartbeats.
2. Threshold is configured per stream in `[deployment]`, defaulting to a small multiple of the venue's heartbeat interval.
3. On expiry: **critical** alert (distinct from FR-03's warning), books `STALE`, quoting blocked.
4. The timer is a Scheduler event, so it fires identically in both modes — this is precisely why the Scheduler owns time rather than the data stream.

**Acceptance.** Replay a recording truncated mid-session for one stream while others continue. Assert the critical alert fires at threshold, not at end of file, and that books for that stream alone go `STALE`.

**Refs.** D28, D30

---

#### FR-03 — Unexpected-heartbeat warning

**Context.** A heartbeat means the venue had nothing to send. On an illiquid instrument that is routine; on a liquid one during continuous trading it is an anomaly worth surfacing. Alerting on *every* heartbeat would train operators to ignore the alarm, so the signal must be scoped by expectation.

**Requirement.** A heartbeat received on a stream that the calendar marks **expected-active** for the current session phase raises a **warning**.

**Behaviour.**
1. "Expected active" is derived from the venue calendar and current session phase — this is the calendar's anomaly-detection role, distinct from its run-planning role.
2. Configurable per stream; streams carrying only illiquid instruments may disable it.
3. Warning severity, not critical — it indicates *possible* trouble, unlike FR-02.

**Acceptance.** Replay a session containing a heartbeat during continuous trading on a liquid stream. Assert a warning is emitted and that the same heartbeat during a scheduled break emits nothing.

**Refs.** D16

---

#### FR-04 — Capture timestamps

**Context.** The Sequencer merges streams on capture timestamp (D05), and the Scheduler requires time to move only forward (D30). Both properties are only as sound as the timestamp. A wall-clock timestamp subject to NTP correction can move backwards mid-session, silently reordering the merge and producing negative measured latencies.

**Requirement.** Every message carries a capture timestamp taken from a **single monotonic clock**, stamped on receipt, plus one **wall-clock anchor per session** to place the run in absolute time.

**Behaviour.**
1. Monotonic value used for **all ordering, sequencing and interval measurement**.
2. Wall clock used only for display, reporting, and correlation against venue timestamps.
3. Stamped as early as possible — **NIC hardware timestamping where the card supports it**, since a software stamp includes OS scheduling jitter, which is precisely the variable being measured around.
4. One clock source across all streams; timestamps from different sources are not comparable.

**Acceptance.** Assert capture timestamps are monotonically non-decreasing across an entire session per source. Inject a simulated NTP step and assert ordering is unaffected.

**Refs.** D05, D30 · depends on PTP availability, see NFR notes

---

#### FR-05 — Capture at the point of consumption

**Context.** Modelled latency is only honest if the recording reflects what qtrade actually saw. A capture taken at a mirror port, upstream device, or different geography observes packets at a different instant — and for CME arriving in Mumbai, that difference is the entire signal delay the strategy must respect. Replaying a Chicago-sourced CME capture in a Mumbai-context backtest hands the strategy information it can never have live.

**Requirement.** Recording occurs **inside the Transport threads**, from the same buffers qtrade consumes. Not a separate `tcpdump` process, not a mirror port, not an upstream appliance.

**Behaviour.**
1. The Recorder captures raw pre-decode bytes at the point of receipt in the Transport.
2. Timestamps applied per FR-04 are the same values the Sequencer uses.
3. If a separate capture exists for network diagnostics, it is **not** a valid backtest input.

**Acceptance.** Compare capture timestamps against decode timestamps for the same messages; the delta must be bounded by in-process handoff, not by network or scheduling.

**Refs.** D05

---

#### FR-06 — Recordings preserve state messages

**Context.** D16 requires session state to come from the **same source in both modes**. On MCX that source is `Product State Change (13300)` and `Instrument State Change (13301)`, which arrive on the same multicast feed as order messages. A capture that filters to order and trade messages for size would make those messages unavailable in Backtest Mode, forcing a calendar fallback in backtest and exchange messages in live — a mode-specific branch inside qtrade, which the architecture forbids.

**Requirement.** Recordings retain **all** message types, including product state, instrument state, mass state change, and heartbeats. Filtering for size is prohibited.

**Behaviour.**
1. The Recorder writes raw bytes without inspection or filtering.
2. Instrument filtering (D32) happens **after** decode, in qtrade — never in the Recorder.

**Acceptance.** Assert every template ID present in a live session is present in its recording. Assert a replayed session produces the same sequence of session-phase transitions as the live session did.

**Refs.** D16, D32

---

#### FR-07 — Post-merge journal

**Context.** FR-12 requires replaying a live session and comparing decisions. That is only possible if the *exact order qtrade consumed events* is recoverable. It cannot be derived from per-stream captures, because live consumes in **arrival** order while a timestamp merge may legitimately produce a different interleaving.

**Requirement.** Live Mode writes a Journal recording the post-merge stream **in qtrade consumption order**, together with every outbound command and inbound report.

**Behaviour.**
1. Written at the qtrade boundary, after the Sequencer, before dispatch.
2. Contains: merged market events in consumption order; every order command with its client order ID; every execution report; every clock read that crossed into a strategy.
3. Written through a bounded ring by a separate low-priority writer. **On overflow: drop and alarm.** A gap in a recording is bad; a stalled engine is worse.
4. Segmented with checksums so corruption is detectable rather than discovered during a backtest.

**Acceptance.** Replay a journal and assert the event sequence delivered to strategies is identical to the live sequence, byte for byte.

**Refs.** D05, FR-12

---

#### FR-08 — Order Validation on both sides

**Context.** Tick size and freeze quantity are venue acceptance rules. The Simulated Exchange must enforce them because the real venue does — otherwise a backtest fills orders production would reject. qtrade must *also* pre-check them, because sending an order that cannot succeed wastes a round trip and consumes OTR budget for nothing.

**Requirement.** Tick-size and freeze-quantity validation is implemented **independently on both sides**: as a pre-trade gate in qtrade, and as an acceptance check in the Simulated Exchange. The two must not share state.

**Behaviour.**
1. qtrade side: runs before RMS and the OTR governor; rejection is **synchronous** (D36) with reason `Validation`.
2. Simulated Exchange side: rejects on arrival, producing an `OrderUpdate` event with state `Rejected` — the asynchronous path, matching live.
3. Both read the same instrument reference data but hold separate code paths, preserving venue independence (D10).
4. Likely additions beyond phase 1: lot-size multiple, price band, maximum order value.

**Acceptance.** Submit an order at an invalid tick with the qtrade gate disabled; assert the Simulated Exchange rejects it. Re-enable the gate; assert it never reaches the venue.

**Refs.** D17, D10, D36

---

#### FR-09 — OTR counted on both sides

**Context.** MCX penalises high order-to-trade ratios, and a market maker is the archetypal high-OTR participant. The venue enforces; qtrade must govern, so the strategy is throttled *before* sending rather than penalised after. Sharing state between them would breach venue independence (D10) — each counts its own flow.

**Requirement.** OTR is tracked by **two independent counters** — an enforcement counter in the Simulated Exchange and a governor in qtrade — each with its **own enable switch**.

**Behaviour.**
1. Both count **messages sent**, not orders created, so quote traffic slots in unchanged if the quote interface is ever adopted (D11).
2. Independent switches permit "enforcement off, governor on" — useful for exploring unconstrained strategy behaviour.
3. Governor rejection is synchronous with reason `OtrBudget`; enforcement rejection arrives as an event.
4. Thresholds are configuration; MCX revises them by circular.

**Acceptance.** Run a strategy that exceeds the configured ratio. Assert the governor throttles first, and with the governor disabled, that the Simulated Exchange rejects at the venue threshold.

**Refs.** D19, D11

---

#### FR-10 — `cancel_all_for_strategy`

**Context.** When a strategy is halted — by panic isolation (D20), by the watchdog (D28), or by an operator — its resting orders remain in the market. Lean orders protect against a *session* drop, but a strategy halt is not a session drop: the session stays up and those orders keep working, managed by code that has stopped running.

**Requirement.** A reliable `cancel_all_for_strategy` operation exists, behaves identically in both modes, and is honoured identically by the Simulated Exchange.

**Behaviour.**
1. Cancels every order owned by that strategy across all instruments and venues.
2. Invoked automatically on: strategy panic under isolate policy, watchdog trip, operator halt, session-phase exit.
3. Must complete even if the strategy is unresponsive — it is engine machinery, not strategy code.
4. Resulting cancellations carry the reason code that triggered them, so reporting can distinguish them.

**Acceptance.** Panic a strategy mid-session under isolate policy. Assert all its orders reach `Canceled` with the correct reason, that other strategies continue unaffected, and that the Simulated Exchange reflects the cancellations in its own books.

**Refs.** D20, D28

---

#### FR-11 — Bootstrap equivalence

**Context.** Live Mode must support joining a session already in progress — a process restarting at 3pm has to rebuild the book from the snapshot channel. That path is intricate and runs under pressure with money at risk. If Backtest Mode always replayed from start of day, the bootstrap path would never execute outside production.

**Requirement.**

> Book state after **snapshot-start at T** then replay to **T′** must equal book state after **full replay from start-of-day to T′**.

**Behaviour.**
1. Parameterised over **many values of T**, deliberately including points *inside* a snapshot cycle rather than only at cycle boundaries.
2. Follows MCX's documented procedure: subscribe to snapshot, continue processing incremental concurrently, apply incrementals whose sequence exceeds `LastMsgSeqNumProcessed` once the cycle completes.
3. Comparison covers full depth, not just BBO — a book can be correct at the top and wrong at level three.

**Why the parameterisation matters.** The synchronisation window is where bootstrap defects live: off-by-one on the watermark, an incremental applied twice, or one dropped. A single well-chosen start time would miss all three.

**Acceptance.** Automated test sweeping T across a full session at fine granularity, asserting full-depth book equality at multiple T′ per start point.

**Refs.** D14 · MCX EOBI §3.2.1

---

#### FR-12 — Live replay parity

**Context.** This is the guarantee the entire architecture exists to provide. Sharing code between modes makes divergence unlikely; it does not make it impossible. Any path by which a strategy reaches ambient state — a wall clock, unseeded randomness, direct I/O — breaks the guarantee silently.

**Requirement.** A recorded Live session, replayed through qtrade, produces a **byte-identical decision stream**.

**Behaviour.**
1. "Decision stream" means every order command in sequence, with identical instrument, side, price, quantity, type and timing relative to the event stream.
2. Replay uses the post-merge Journal (FR-07), so consumption order is exactly reproduced.
3. Any divergence is a **defect**, not a tolerance — it means the strategy reached state it should not have been able to touch.
4. Exposed as a first-class command (`qtrade replay`) so it is run routinely rather than aspirationally.

**Acceptance.** Run a live or paper session, replay its journal, diff the command streams. Zero differences.

**Refs.** D05, D22, D30 · the determinism rules in §6

---

#### FR-13 — Auction bootstrap produces no book

**Context.** The MCX EOBI specification states that during auctions, snapshot messages carry **Auction Best Bid-Offer or Auction Clearing Price messages instead of order messages** — visible orders are not published via snapshot during an auction. A process starting mid-auction therefore *cannot* construct a book from the snapshot channel, however correct its bootstrap logic.

**Requirement.** Bootstrap during an auction leaves the book `UNINIT`. It converges only once Continuous Trading resumes, at which point the venue republishes all visible orders on the incremental channel.

**Behaviour.**
1. Detect auction phase from `Instrument State Change (13301)`.
2. While in auction: no book construction attempted, state `UNINIT`, quoting blocked (FR-15).
3. On transition to Continuous: consume the full republished book and converge.
4. This is **correct behaviour, not a failure** — it must not raise a critical alert.

**Acceptance.** Start a run mid-auction. Assert: book stays `UNINIT`; no quoting occurs; no critical alert; on resumption the book converges to full-replay state.

**Refs.** D14 · MCX EOBI §3.4.3

---

#### FR-14 — Gap recovery convergence

**Context.** Distinct from cold start. Here qtrade already holds a book, detects a sequence gap, and must resynchronise — a different code path with different failure modes, notably applying recovered messages that were already processed.

**Requirement.** Book state after **gap-and-recovery at T** equals book state after **uninterrupted replay** to the same point.

**Behaviour.**
1. On gap: mark `STALE`, block quoting, initiate recovery.
2. Recovery via retransmission for small gaps, snapshot channel for large ones.
3. Duplicate suppression during recovery — a recovered message already applied must not be applied twice.
4. State returns to `OK` only after convergence is confirmed, not on receipt of the last recovered message.

**Acceptance.** Inject synthetic gaps of varying size at varying points; assert full-depth book equality against uninterrupted replay for each.

**Refs.** D05, FR-01

---

#### FR-15 — An uncertain book blocks quoting

**Context.** A book that is `UNINIT` or `RECOVERING` does not describe the market. Quoting against it means placing orders priced on state you do not have. This must be enforced by the engine rather than by strategy convention, because the moment it matters most is the moment a strategy is malfunctioning.

**Requirement.** Any order submission for an instrument whose book is `UNINIT` or `RECOVERING` is rejected. `STALE` behaviour is governed by the strategy's declared staleness tolerance (D28).

**Behaviour.**
1. Enforced at the qtrade gate — synchronous rejection with reason `NotQuotable`.
2. Also covers warmup incomplete and position reconciliation incomplete (D38).
3. Exposed to strategies as `ctx.can_quote()` so they can check rather than be rejected.
4. Independent of the watchdog: this blocks *new* orders; the watchdog cancels *existing* ones.

**Acceptance.** Submit during warmup, during bootstrap, and during recovery. Assert rejection with the correct reason in each case, and that `can_quote()` returns false.

**Refs.** D28, D38, D36

---

#### FR-16 — Daily contract file archived with each recording

**Context.** MCX publishes a contract file each trading day defining every tradable instrument and its `SecurityID`. **Instrument identifiers are not stable across days.** Replaying a past session against a *current* contract file maps events to the wrong instruments — silently, with no error, producing a backtest that traded something other than what you believe.

**Requirement.** The daily contract file is **archived alongside that day's recording** and loaded from that archive in Backtest Mode. The recording unit is *capture + contract file + calendar version*, versioned together.

**Behaviour.**
1. The Recorder archives the contract file with the session it belongs to.
2. Backtest Mode loads the contract file from the archive for the day being replayed — never the current one.
3. The master version is pinned in the hashed `[run]` config (D22), so a run using different reference data is a distinguishable run.
4. A missing or mismatched contract file is a **hard failure at startup**, never a warning.
5. Multi-day runs reload per trading day.

**Acceptance.** Attempt a backtest of a past date with only the current contract file present. Assert startup fails with a clear diagnostic rather than proceeding.

**Refs.** D15, D22, D37

---

### 2.2 Non-functional requirements

#### NFR-01 — Determinism

**This is a correctness property, not a convenience.** FR-11 through FR-14 are all equality assertions between runs; if runs are not reproducible, none of them can be evaluated at all.

**Requirement.** Identical input and identical `[run]` configuration produce **byte-identical output**, including the order command stream, all fills, and all reported figures.

**Guaranteed by:** single-threaded qtrade (D04) · total event ordering on `(timestamp, class, seq)` with every tie decided (D30) · Sequencer merging by data rather than thread timing (D05) · monotonic clock for ordering so an NTP step cannot reorder (D30) · seeded randomness including the latency model (D18) · no ambient state reachable from strategies (D03).

**Verified by:** running any backtest twice and diffing outputs. Belongs in CI, not in someone's memory.

---

#### NFR-02 — Mode parity

**Requirement.** **No component below the Normalizer may branch on run mode.** Mode differences live at the edges — Transport, Clock, venue — or in configuration.

**Why it is stated as a prohibition.** Every mode branch is a code path that runs in one mode and not the other, which is by definition untested where it matters. The known, deliberate mode differences are enumerated in §3.3; anything else is a defect.

**Verified by:** FR-12. A recorded live session replayed through qtrade must produce an identical decision stream — a mode branch would show up as divergence.

---

#### NFR-03 — Live latency

**Requirement.** Sub-millisecond tick-to-trade, colocated, ordinary sockets, for phase 1.

**Constraint this places on the design.** Kernel bypass is a **committed later phase** (D02), so the `LiveTransport` boundary must stay clean enough that bypass replaces that component alone. No socket-specific assumption may leak into the Decoder or qtrade.

**Deferred with it:** NUMA placement and CPU-core pinning return in the same phase.

**Noted tension.** Quincy microwave data buys a latency edge measured in microseconds while ordinary kernel sockets spend a meaningful part of it. Acceptable as build order — correctness first — but it makes the bypass phase more urgent than "eventually."

---

#### NFR-04 — Backtest throughput

**Requirement.** A full trading day completes in minutes, not hours.

**Parallelism model.** Across **processes** — separate days, separate parameter sets — never threads within a run. This is what makes NFR-01 affordable: determinism costs nothing when scaling happens at the process boundary.

**Consequence.** This assumption holds only for `independent` session mode (D29). A `continuous` multi-day run serialises by construction, and gives up cross-day parallelism deliberately.

---

#### NFR-05 — Hot path discipline

**Requirement.** **No allocation** in market-data dispatch or book maintenance.

**Where this binds.** Every `OrderAdd`, `OrderModify`, `OrderDelete` and execution touches the book; an allocation there is multiplied by message count. Dispatch is keyed by `(instrument, depth)` into a pre-sized structure — no string keys, no hashmap lookup, no dynamic type dispatch on the market-data path.

**Where it does not bind.** The Control Dispatcher, reporting, and anything behind `ctx.offload()`. Trading flexibility for speed is correct there and wrong on the data path — which is why they are separate mechanisms (D33).

---

#### NFR-06 — Extensibility

**Requirement.** Adding a venue requires **no change below the Normalizer**.

**How it is tested.** Stage 10 of the build order adds the Quincy adapter. If that touches any file inside qtrade, the exchange abstraction has leaked — and it is far cheaper to discover that with two venues than three.

**Also required to plug in without restructuring:** option pricing and volatility models (D09 boundary), margin models inside RMS (D34), latency models (D18), and bar aggregation when it arrives in phase 2 (D35).

## 3. System structure

### 3.1 Three layers

```
┌─────────────────────────────────────────────────────────────┐
│ LAYER 1 — TRANSPORT            per venue AND per mode       │
│   LiveTransport  |  ReplayTransport                         │
│   produces framed messages + capture timestamps             │
└─────────────────────────────────────────────────────────────┘
                            ↓ framed messages
┌─────────────────────────────────────────────────────────────┐
│ LAYER 2 — DECODE + NORMALIZE   per venue, SHARED by modes   │
│   Decoder → Normalizer → instrument filter                  │
│   ** this is where backtest/live parity is enforced **      │
└─────────────────────────────────────────────────────────────┘
                            ↓ internal events
┌─────────────────────────────────────────────────────────────┐
│ LAYER 3 — CORE                 exchange- and mode-agnostic  │
│   Scheduler · Data Engine · BookBuilder · Cache             │
│   Strategies · ExecutionEngine · Order Validation           │
└─────────────────────────────────────────────────────────────┘
                            ↓ order commands    ↑ execution reports
┌─────────────────────────────────────────────────────────────┐
│ VENUE EDGE                     per mode                     │
│   Simulated Exchange  |  Exchange Gateway                   │
└─────────────────────────────────────────────────────────────┘
```

**Layer 2 is where parity is enforced.** The MCX Decoder is one implementation driving both the live path and the replay path, so a decoder defect reproduces in backtest instead of waiting for production.

### 3.2 What swaps between modes

| Component | Backtest Mode | Live Mode |
|---|---|---|
| Transport | `ReplayTransport` — recorded messages | `LiveTransport` — multicast, A/B arbitration, gap detection, recovery |
| Clock | `SimClock` — advanced by the Scheduler | `LiveClock` — real time |
| Latency | `Fixed` or `Sampled` model (**D18**) | Real network |
| Venue | Simulated Exchange | Exchange Gateway |
| Failure policy | fail-fast (default) | isolate (default) |

**Everything else is identical.** Decoder, Normalizer, Scheduler, Data Engine, BookBuilder, Cache, Strategy, ExecutionEngine, Order Validation, Cost Model, Reporting.

### 3.3 Threading

- **One qtrade thread.** All strategies run inline on it, one after another. This is what makes runs reproducible (**D04**).
- **One thread per Feed Adapter**, doing receive → decode → normalise → filter, pushing internal events to a bounded queue.
- **One writer thread** for the Recorder and Reporting, fed from bounded rings, never blocking.
- **Offload workers** for expensive strategy computation, whose results return as *scheduled events* rather than blocking calls.

Backtest parallelism is achieved by running many processes across days and parameter sets (**D22**).

---

## 4. Components

### 4.1 Transport

**Responsibility.** Produce framed messages with capture timestamps. Own all session and recovery mechanics so nothing leaks upward.

| | |
|---|---|
| **Input** | UDP multicast (live) or recorded file (replay) |
| **Output** | Framed messages, each carrying a monotonic capture timestamp (**FR-04**, **D30**) |
| **Live only** | A/B "Live-Live" arbitration, per-channel gap detection, snapshot channel subscription, retransmission |
| **Invariant** | No socket- or file-specific concept appears above this layer. Kernel bypass later replaces this component alone (**D02**). |

### 4.2 Decoder

**Responsibility.** Translate one venue protocol into structured messages.

| | |
|---|---|
| **Input** | Framed messages |
| **Output** | Structured messages in that protocol's own vocabulary |
| **Invariant** | One implementation per protocol, shared by live and replay. Never duplicated for a transcode path. |

### 4.3 Normalizer

**Responsibility.** Convert protocol messages into internal events, absorbing every venue-specific representation.

| Conversion | Detail |
|---|---|
| Time | Venue epoch → `i64` nanoseconds, Unix epoch. MCX is already Unix/UTC; NSE is 1980-based. |
| Price | Venue scale → `i64` ticks. MCX carries 8 decimals; NSE CM/FO is ÷10². |
| Instrument | Native id → interned `InstrumentId` carrying its Venue |
| Order identity | An abstract handle. MCX has **no broadcast order ID** — identity is composite, including priority timestamp. NSE has an explicit ID (a wire `DOUBLE`, converted to `u64` at decode and never compared as a float). |
| Priority | An explicit `priority_retained` flag. **MCX publishes this directly** (`13101` vs `13106`); the NSE decoder must infer it from matching rules. |

**Invariant.** Nothing venue-shaped survives this boundary. If a decoder cannot express something in the internal vocabulary, the vocabulary is wrong — widening it with optional fields is not the fix (**D32** rationale).

### 4.4 Instrument filter

**Responsibility.** Discard events for instruments no strategy wants, as early as possible.

| | |
|---|---|
| **Position** | Immediately after decode, keyed on the venue's native instrument id, before normalisation and before any book work |
| **Source** | The strategy declares its filter **programmatically** at `on_start`, resolved against the day's instrument master via the Data Engine's query interface (**D32**, **D15**) |
| **Invariant** | The predicate covers contracts the strategy will roll into, so a mid-run subscription always finds a book with full history |
| **Scope** | The same filtered set is used by both qtrade and the Simulated Exchange |

A recording contains all of MCX. Without this filter, building books for every contract would dominate backtest runtime.

### 4.5 Sequencer

**Responsibility.** Merge several ordered streams into one totally-ordered stream.

| | |
|---|---|
| **Ordering key** | `(capture_timestamp, source_id, sequence_number)` |
| **Why capture time** | Venue clocks are not comparable across exchanges, and exchange time would give strategies a cleaner cross-venue view than they can ever have live |
| **Backtest** | Blocks until every ring has an event or signals EOF |
| **Live** | Takes what is available; never waits on a quiet venue |
| **Watermarks** | Heartbeats advance a quiet stream's low-water mark so the merge can proceed without data |

### 4.6 Scheduler and Clock

**Responsibility.** Own time. Nothing else advances it.

| | |
|---|---|
| **Structure** | Priority queue ordered by `(timestamp, event_class, monotonic_seq)`, the sequence assigned at enqueue |
| **Loop** | Pop earliest → set clock → dispatch → handler may enqueue further events |
| **Backtest** | `SimClock` set to each popped event's timestamp |
| **Live** | `LiveClock` reads real time; timers are real |
| **Invariant** | Time only moves forward. Every tie has a defined winner — an undefined tie is a non-reproducible run. |

**A clock advanced only by market data cannot detect the absence of market data.** A five-second feed silence would jump the clock five seconds and never fire the staleness timer. This is why the Scheduler, not the data stream, owns time.

### 4.7 Data Engine

**Responsibility.** Subscriptions, session phase, book state, reference data.

| | |
|---|---|
| **Subscriptions** | Dynamic — strategies subscribe and unsubscribe mid-run as they roll (**D15**) |
| **Session state** | Sourced **per venue, identically across modes** (**D16**). MCX publishes it (`13300`, `13301`), and recordings preserve it, so backtest reads the same messages. Venues publishing none fall back to calendar plus data-presence inference, in both modes. |
| **Reference data** | Instrument metadata and query interface: venue, underlying, contract month, expiry, tick size, lot size, multiplier, freeze quantity |
| **Calendar** | Used for run planning and **anomaly detection**, never as the state source where the venue publishes it |

### 4.8 BookBuilder

**Responsibility.** Maintain order books. A pure function of the event stream.

| | |
|---|---|
| **Instances** | One per filtered instrument, shared by all strategies (**D06**) |
| **Types** | `MboBook` and `MbpBook` behind a common `Book` trait |
| **Common interface** | `best_bid` · `best_ask` · `depth(n)` · `qty_at_price` |
| **MBO only** | `queue_position(order_handle)` — absent from `MbpBook` by type, not by runtime null |
| **Invariant** | Tolerates a crossed book. On MTBT, aggressive orders publish before the resulting trade, so `best_bid >= best_ask` is a normal transient state. |

**Purity matters:** because a book depends on nothing but the event stream, instance count is a wiring decision. Should measurement later favour per-strategy or per-group books, that is configuration rather than restructuring.

### 4.9 Cache

**Responsibility.** The shared read model. Read-only to strategies.

Holds: order books per filtered instrument · **Book State** (`UNINIT` / `RECOVERING` / `OK` / `STALE`) · **Session State** per venue and instrument · own orders · Sub-account positions and P&L · Firm Aggregate · reference data for the trading day.

**Book State and Session State are separate and both visible.** A CME book during the maintenance break is perfectly `OK` and completely frozen. A market maker unable to distinguish these will either quote off an hour-old signal or stand down whenever a book goes quiet.

### 4.10 Dispatchers

There is **no message bus** (**D07**). Two named mechanisms, both direct calls on qtrade thread.

| | Event Dispatcher | Control Dispatcher |
|---|---|---|
| **Carries** | Market data | Commands, execution reports, session changes, alerts |
| **Keying** | `(instrument, depth)` subscriber lists | Typed handler lists |
| **Properties** | Static dispatch, no allocation | Observers added by wiring, not by editing publishers |

**Depth-scoped dispatch** (**D25**): a strategy subscribing at BBO depth is woken when the best bid or offer changes and not otherwise. The full book is still maintained — subscription governs *waking*, not *access*.

Routing knowledge lives in startup wiring, never inside either dispatcher.

### 4.11 Strategy

**Responsibility.** Trading logic. Identical code in both modes.

**Callbacks:** `on_start` · `on_book` · `on_trade` · `on_fill` · `on_order_update` · `on_timer` · `on_session_change` · `on_book_state_change` · `on_warmup_complete` · `on_stop`

**Context handle provides:** clock reads · Cache access (books, both position levels, session and book state) · Cost Model queries · order submit / cancel / modify · dynamic subscribe / unsubscribe · timers and alarms · instrument reference queries · seeded RNG · deterministic logging · time-series publication.

**Strategies own:** fair value (**D09**), skew, roll policy (**D15**), the instrument filter (**D32**), and their declared signal dependencies (**D28**).

**Strategies never reach ambient state.** No system clock, no unseeded randomness, no direct I/O. Dispatch is `dyn Strategy`, so strategy sets are loadable from run configuration.

### 4.12 ExecutionEngine

**Responsibility.** Order lifecycle and reconciliation.

Assigns client order IDs · maintains the order state machine (pending → acked → partial → filled / cancelled / rejected) · updates Sub-account and Firm Aggregate on fills · provides `cancel_all_for_strategy` (**FR-10**) · distinguishes rejection reasons: own limit, firm limit, venue rejection.

**Invariant.** It cannot tell the Simulated Exchange from the Exchange Gateway. Both present the same interface: commands in, reports out.

### 4.13 Order Validation

**Responsibility.** Stateless per-order checks against reference data — whether the venue would accept the order at all.

Phase 1: **tick size** and **freeze quantity**. Likely additions: lot-size multiple, price bands, maximum order value.

**Runs on both sides** (**D17**): the Simulated Exchange enforces because the real venue does; the Engine pre-checks to avoid burning a round trip and an OTR message on an order that cannot succeed. Independent implementations, identical rules.

Distinct from **Risk Limits** — stateful, portfolio-scoped, deferred (**D17**).

### 4.14 Cost Model

**Responsibility.** Transaction costs, queryable before quoting and applied to fills.

| | |
|---|---|
| **Components** | Exchange transaction charges, SEBI turnover fees, CTT, GST, stamp duty, clearing and brokerage. Rates are configuration. |
| **Asymmetry** | **CTT falls on the sell side, stamp duty on the buy side.** Cost is not a flat per-lot figure. Several components are turnover-percentage based and therefore scale with price. |
| **Pre-trade** | `round_trip_cost(instrument, qty, side)` available synchronously on the hot path |
| **Post-trade** | The same model applied to realised fills, so the quoting assumption and the accounting cannot disagree |
| **Not in the venue** | The real exchange does not report CTT or stamp duty in an execution report — those arrive in contract notes. A simulator producing them would generate information the live path never does. |

**No market-maker concession applies** (**D23**). Round-trip cost is therefore a hard floor on the quotable spread.

### 4.15 Simulated Exchange

**Responsibility.** Stand in for the venue in Backtest Mode.

| | |
|---|---|
| **Own books** | Built directly from the normalized event stream, for the whole filtered set (**D32**) |
| **No Cache access** | No read path into qtrade. Only order commands in, execution reports out. |
| **Models** | Latency (**D18**) · queue position · price-time priority · modify-vs-cancel-replace semantics · BOC rejection · Market-to-Limit residual · OTR enforcement · STP · MMP cancellations · watchdog cancellations (**D28**) · Lean order cancellation on session loss, market reset and end of day |

**Why independence matters.** In production the venue is not inside your process. A feed-derived book can go `STALE` after a packet gap, but **a real exchange does not become uncertain because your receiver dropped packets.** A simulator reading your Cache would inherit your corruption and fill you against a book that never existed.

### 4.16 Recorder

**Responsibility.** Capture during Live runs.

| | |
|---|---|
| **Two recording points** | **Per-stream raw capture** at the Transport, preserving native order and per-channel sequence numbers, feeding development replay. **Post-merge Journal** at qtrade boundary, in actual consumption order, feeding **FR-12** parity verification. |
| **Why both** | The Journal cannot be derived from per-stream captures, because live consumes in *arrival* order and a timestamp merge may legitimately produce a different interleaving. |
| **Where** | In the Transport threads, not a separate capture process — **FR-05** requires capturing where you consume. |
| **Never blocks** | Bounded ring, separate low-priority writer. On overflow: **drop and alarm**. A gap in a recording is bad; a stalled engine is worse. |
| **Also** | Stamps per **D30** · preserves state messages (**FR-06**) · segments with checksums so corruption is detectable |

### 4.17 Reporting

**Responsibility.** Emit run results. Registered as an observer on the Control Dispatcher — nothing publishes *to* it.

| Tier | Content |
|---|---|
| **1 — always** | Compact structured summary: P&L gross and net at both accounting levels, inventory over time, markout distribution, OTR consumed, message counts, invariant violations. Columnar, so sweeps aggregate. |
| **2 — switchable** | Full per-event detail: every command and response with rejection reasons distinguished; per-fill records carrying **queue position at fill**, **markout at fixed horizons**, a **spread-improving flag**, and realised cost. |
| **3 — strategy series** | Named scalar series published by strategies at clock time — fair value, skew, quote width, basis. **Fills explain outcomes; these explain reasoning.** Pre-registered handles, write-only, non-behavioural, identical API in both modes. |

Every output embeds the full run specification and both identity hashes (**D22**).

---

## 5. Flow of events

### 5.1 qtrade loop

One thread, one loop. Everything else follows from what is in the queue.

```
loop {
    event = scheduler.pop_earliest()      // priority queue
    if event.is_none() { break }
    clock.set(event.timestamp)            // time only moves here
    dispatch(event)                       // handler may enqueue more events
}
```

### 5.2 Event sources

Market data is **one source among several**:

| Source | Notes |
|---|---|
| Market data | From the Sequencer |
| Strategy timers and alarms | `set_timer` / `set_alarm` |
| **Order arrival** | Submitted at T, arrives at `T + outbound latency` |
| **Report delivery** | Filled at T, learned at `T + inbound latency` |
| Session transitions | Pre-open, continuous, auction, close |
| **Staleness and heartbeat timeouts** | Fire when *nothing* arrives |
| Offload completion | Expensive strategy computation returning as a scheduled event |
| Watchdog expiry | Declared-dependency staleness (**D28**) |

### 5.3 Market data path — one event

1. Packet arrives on a multicast group. **Transport** stamps a monotonic capture timestamp and pushes to its ring. In Live Mode the **Recorder** captures the raw bytes here.
2. The **Feed Adapter thread** decodes and normalises, applies the **instrument filter**, and pushes internal events.
3. The **Sequencer** merges across streams by `(capture_timestamp, source_id, sequence_number)` and enqueues into the **Scheduler**.
4. qtrade loop pops it, sets the clock, and dispatches to the **Data Engine**.
5. The **BookBuilder** applies it to the book; the **Cache** now reflects the new state.
6. The **Event Dispatcher** wakes only strategies whose subscribed *depth* was affected.
7. Independently, the **Simulated Exchange** receives the same event and updates **its own** book.

### 5.4 Order path — a worked trace

Outbound latency modelled at 250µs.

| Time | Event |
|---|---|
| `09:20:00.000000000` | `OrderAdd` on MCX Crude pops. Clock set. Book updated. `Strategy::on_book()` called. |
| — | Strategy calls `ctx.submit_order(...)`. **This does not reach the venue.** ExecutionEngine assigns a client order ID; Order Validation checks tick size and freeze quantity; OTR governor checks budget; an `OrderArrival` event is enqueued at `09:20:00.000250000`. |
| `.000000` → `.000250` | The loop keeps popping. **Real market events in that window are processed first** — other participants arriving at your price level, ahead of you. |
| `09:20:00.000250000` | `OrderArrival` pops. The Simulated Exchange inserts the order into **its own** book, at the back of the queue at that price, behind everyone who arrived during those 250µs. |
| later | Aggressing flow arrives. The simulator determines the fill and enqueues an `ExecutionReport` at `fill_time + inbound latency`. **It does not call back directly.** |
| later still | The report pops. ExecutionEngine advances the state machine, updates Sub-account and Firm Aggregate, and calls `Strategy::on_fill()`. **Only now does skew logic see the new inventory.** |

**Two properties to notice.** Every delay is a timestamp in the queue, never a sleep — nothing blocks. And the strategy's view is always the delayed one, exactly as in production.

**In Backtest Mode time is free.** The clock jumps event to event; a day completes in however long the CPU takes. The sub-millisecond target appears only as modelled offsets, never as run time.

### 5.5 Startup and warmup

1. Load run configuration; resolve the instrument master for the trading day.
2. Construct components and wire them. Wiring — not the dispatchers — holds all routing knowledge.
3. `Strategy::on_start()` — the strategy declares its instrument filter, its signal dependencies, and its time series.
4. Bootstrap books: full-day replay (default) or snapshot-start (**D14**). Books are `UNINIT` until complete.
5. **Warmup**: strategies receive events and update state but **may not quote**.
6. `on_warmup_complete()` — quoting permitted.

**During an auction, snapshot bootstrap cannot build a book at all** — the EOBI snapshot channel carries Auction BBO or Clearing Price messages instead of orders. The book stays `UNINIT` until Continuous Trading resumes, at which point all visible orders are republished on the incremental channel (**FR-13**).

### 5.6 Failure paths

| Condition | Response |
|---|---|
| Sequence gap | Mark books `STALE`, trigger recovery, quoting blocked (**FR-15**) |
| No data and no heartbeat past threshold | Critical alert, books `STALE` (**FR-02**) |
| Declared dependency stale | Watchdog **cancels** the strategy's orders (**D28**) |
| Strategy panic | Live: isolate — halt strategy, cancel its orders, others continue. Backtest: fail-fast. Both are configuration (**D20**). |
| Malformed packet or book inconsistency | Mark `STALE`, recover. **Never process termination.** |
| Recorder ring overflow | Drop and alarm. Never block qtrade. |

---

## 6. Determinism and reproducibility

Determinism is a **correctness property** here, since **FR-11** through **FR-14** are all equality assertions between runs.

**What guarantees it**

1. **Single-threaded qtrade** — no scheduling nondeterminism (**D04**).
2. **Total event ordering** — `(timestamp, event_class, monotonic_seq)`, every tie decided (**D30**).
3. **Deterministic merge** — the Sequencer orders by data, not by thread timing (**D05**).
4. **Monotonic clock for ordering** — an NTP step cannot reorder events (**D30**).
5. **Seeded randomness** — including the `Sampled` latency model, seeded from run configuration (**D18**).
6. **No ambient state in strategies** — no system clock, no unseeded RNG, no direct I/O.

**Run identity** is `(config hash, build/commit hash)` (**D22**). A config hash alone captures no strategy code; two runs on identical configuration and different commits will differ, and config-only identity leads to concluding a parameter mattered when it was a code change.

**Pinned in run configuration:** recordings and their content hashes · **the full strategy set** and every parameter · latency model type, parameters and seed · simulator switches · warmup and bootstrap mode · calendar and instrument master versions · session-boundary mode (**D29**).

**Deliberately excluded — deployment configuration** (**D27**): multicast addresses, ports, interfaces, session credentials, environment selection. These affect where the process connects, not what it produces. Including them would make a colocation rack change appear to invalidate previous results.

---

## 7. Validation

### 7.1 Book correctness

**Primary: the snapshot channel.** Compare the incrementally-built book against each arriving snapshot cycle — full depth, continuously through the session. Assertions in Backtest Mode, metrics in Live Mode. A BBO-only check would miss a book correct at the top and wrong at depth three.

**Secondary: `Top Of Book (13504)`** as an end-of-session checkpoint only — the spec restricts its dissemination to post-trading through end of day, so it cannot validate during continuous trading.

### 7.2 Simulator invariants

Checked automatically on every backtest run, requiring no additional data. Strongest first:

> **Simulated fills at a given price and time must not exceed the volume that actually traded at that price in the recording.**

Filling 100 lots where 20 traded means the simulator fabricated liquidity.

Supporting: a **Book-or-Cancel** order that would cross is **rejected, never filled** · fill price at or better than limit · queue position never improves except through consumption ahead · **Market-to-Limit** residual becomes a resting limit rather than vanishing · simulated OTR and message-rate counters never exceed configured venue limits.

### 7.3 Scenario and parity tests

**Hand-traceable scenarios** — synthetic flow small enough to verify on paper, kept as regression tests. Feasible because phase 1 is a scalar-inventory futures contract.

**Bootstrap equivalence** — **FR-11**, **FR-13**, **FR-14**.

**Live replay parity** — **FR-12**. A recorded Live run replayed through qtrade must produce an identical decision stream. Divergence means a strategy reached ambient state it should not have been able to touch.

**Shadow comparison, once live** — run the Simulated Exchange in parallel against real trading, same data and same orders, and diff simulated fills against real fills. **The only test measuring the simulator against reality rather than against our own assumptions.**

---

## 8. Build order

Each stage validates the one before it. Performance work — pinning, NUMA, kernel bypass — is deliberately absent until a throughput budget is measured against something that already works.

| # | Stage | Done when |
|---|---|---|
| 1 | **Reference data** — instrument master, interned `InstrumentId`, metadata query | The same contract resolves to the same identity across days |
| 2 | **MCX T7 EOBI decoder** — all template IDs, no book yet | Byte counts reconcile; unknown templates skip cleanly |
| 3 | **BookBuilder** — MBO, crossed-tolerant, gap-aware | Book matches every snapshot cycle, full depth, all session |
| 4 | **Scheduler** — priority queue, total ordering, timers, session phases | Same input replayed twice is byte-identical |
| 5 | **Cache, Data Engine, dispatch, filter** | A no-op strategy runs a full day; throughput measured |
| 6 | **Cost Model** | **Minimum quotable spread computed against historical spreads** — this either validates the phase-1 premise or redirects it |
| 7 | **Simulated Exchange** — independent books, latency, queue position, invariants | A strategy with analytically predictable behaviour fills as predicted |
| 8 | **ExecutionEngine, Order Validation, accounting** | Rejects and partial fills drive the state machine correctly |
| 9 | **Recorder, Journal, parity diff** | A recorded session replays to an identical decision stream |
| 10 | **Quincy QED adapter** — second decoder, MBP books | qtrade unchanged; no qtrade file modified |
| 11 | **Live transport, MCX ETI gateway** | Paper session diffs clean against its own replay |
| 12 | **Risk framework** | **Required before live trading with real money** |

**Stage 10 is the real test of this architecture.** If adding the second adapter requires touching anything below the Normalizer, the exchange abstraction has leaked and is worth fixing before a third venue arrives.

**Stage 6 is worth doing early** — it needs no engine, and given no market-maker concession applies, it answers whether one-tick market making is viable at all before significant build effort.

**Stage 12 is not optional.** Deferring the risk framework is reasonable for backtesting; going live with market making and no max-net-position and no daily-loss halt is not. A quoting strategy with a bug accumulates position quickly, and Lean orders protect against a disconnect but not against a strategy working exactly as coded and wrong.

# Trading Engine — Architecture

**Status:** Design complete for phase 1. No code written.
**Last updated:** 2026-08-19

**Companion documents**
- [CONTEXT.md](CONTEXT.md) — glossary. Terms below are used as defined there.
- [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md) — the 32 decisions and their reasoning. Referenced here as **D01**–**D32**.
- [OPEN-QUESTIONS.md](OPEN-QUESTIONS.md) — what remains unresolved.

---

## 1. Purpose and scope

### 1.1 What this system is

An engine that runs **the same compiled code** in Backtest Mode and Live Mode. Only two thin edges swap: where market data bytes come from, and where orders go. Everything between — decoding, book building, strategy logic, execution, accounting — is one artefact, exercised identically in both modes.

The engine is **multi-venue and exchange-agnostic at its Core**. All venue-specific knowledge lives in Feed Adapters at the edge.

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

### 2.1 Functional

| # | Requirement |
|---|---|
| **FR-01** | On heartbeat receipt, compare its last-sequence-number against the last sequence received on that stream. Any discrepancy triggers gap recovery and marks affected books `STALE`. |
| **FR-02** | Absence of **both** data and heartbeat on a subscribed stream beyond a configurable threshold raises a **critical** alert and marks affected books `STALE`. |
| **FR-03** | A heartbeat on a stream expected to be active for the current session phase raises a **warning**, configurable per stream. |
| **FR-04** | Every message carries a capture timestamp from a single monotonic clock, stamped on receipt. |
| **FR-05** | Recording occurs at the point of consumption, never at a mirror port or upstream location. |
| **FR-06** | Recordings preserve product and instrument state messages. |
| **FR-07** | Live Mode writes a post-merge Journal in Core consumption order. |
| **FR-08** | Order Validation enforces tick size and freeze quantity, independently on the Engine and Simulated Exchange sides. |
| **FR-09** | OTR is counted independently on both sides, each with its own enable switch. |
| **FR-10** | `cancel_all_for_strategy` exists, is reliable in both modes, and is honoured identically by the Simulated Exchange. |
| **FR-11** | Book state after snapshot-start at T then replay to T′ equals book state after full replay to T′. Parameterised over many values of T, including points inside a snapshot cycle. |
| **FR-12** | A recorded Live run replayed through the Core produces an identical decision stream. |
| **FR-13** | Bootstrap during an auction produces no book. The book stays `UNINIT` until Continuous Trading resumes, then converges to full-replay state. |
| **FR-14** | Book state after mid-session gap recovery equals book state after uninterrupted replay to the same point. |
| **FR-15** | A book that is `UNINIT` or `RECOVERING` prevents quoting. |

### 2.2 Non-functional

| Property | Requirement |
|---|---|
| **Determinism** | Identical input and configuration produce byte-identical output. This is a correctness property, not a convenience. |
| **Mode parity** | No component below the Normalizer may branch on run mode. Mode differences live at the edges or in configuration. |
| **Live latency** | Sub-millisecond tick-to-trade, colocated, ordinary sockets. Kernel bypass in a later phase without Core changes. |
| **Backtest throughput** | A full trading day completes in minutes. Parallelism comes from concurrent processes, never from threads within a run. |
| **Hot path** | No allocation in market data dispatch or book maintenance. |
| **Extensibility** | New venues require no change below the Normalizer. Option pricing and volatility arrive as plug-ins. |

---

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

- **One Core thread.** All strategies run inline on it, one after another. This is what makes runs reproducible (**D04**).
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
| **Scope** | The same filtered set is used by both the Core and the Simulated Exchange |

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

There is **no message bus** (**D07**). Two named mechanisms, both direct calls on the Core thread.

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
| **No Cache access** | No read path into the Core. Only order commands in, execution reports out. |
| **Models** | Latency (**D18**) · queue position · price-time priority · modify-vs-cancel-replace semantics · BOC rejection · Market-to-Limit residual · OTR enforcement · STP · MMP cancellations · watchdog cancellations (**D28**) · Lean order cancellation on session loss, market reset and end of day |

**Why independence matters.** In production the venue is not inside your process. A feed-derived book can go `STALE` after a packet gap, but **a real exchange does not become uncertain because your receiver dropped packets.** A simulator reading your Cache would inherit your corruption and fill you against a book that never existed.

### 4.16 Recorder

**Responsibility.** Capture during Live runs.

| | |
|---|---|
| **Two recording points** | **Per-stream raw capture** at the Transport, preserving native order and per-channel sequence numbers, feeding development replay. **Post-merge Journal** at the Core boundary, in actual consumption order, feeding **FR-12** parity verification. |
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

### 5.1 The Core loop

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
4. The Core loop pops it, sets the clock, and dispatches to the **Data Engine**.
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
| Recorder ring overflow | Drop and alarm. Never block the Core. |

---

## 6. Determinism and reproducibility

Determinism is a **correctness property** here, since **FR-11** through **FR-14** are all equality assertions between runs.

**What guarantees it**

1. **Single-threaded Core** — no scheduling nondeterminism (**D04**).
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

**Live replay parity** — **FR-12**. A recorded Live run replayed through the Core must produce an identical decision stream. Divergence means a strategy reached ambient state it should not have been able to touch.

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
| 10 | **Quincy QED adapter** — second decoder, MBP books | Core unchanged; no Core file modified |
| 11 | **Live transport, MCX ETI gateway** | Paper session diffs clean against its own replay |
| 12 | **Risk framework** | **Required before live trading with real money** |

**Stage 10 is the real test of this architecture.** If adding the second adapter requires touching anything below the Normalizer, the exchange abstraction has leaked and is worth fixing before a third venue arrives.

**Stage 6 is worth doing early** — it needs no engine, and given no market-maker concession applies, it answers whether one-tick market making is viable at all before significant build effort.

**Stage 12 is not optional.** Deferring the risk framework is reasonable for backtesting; going live with market making and no max-net-position and no daily-loss halt is not. A quoting strategy with a bug accumulates position quickly, and Lean orders protect against a disconnect but not against a strategy working exactly as coded and wrong.

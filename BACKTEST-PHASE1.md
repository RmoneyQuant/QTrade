# Phase 1 — Backtest Implementation Spec

**Purpose:** an implementation-ready specification for the first working backtest. Written so an engineer or coding agent can start without re-deriving context.

**Prerequisites:** [CONTEXT.md](CONTEXT.md) for vocabulary · [ARCHITECTURE.md](ARCHITECTURE.md) for the full design · [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md) for reasoning (referenced as **Dnn**) · [STRATEGY-GUIDE.md](STRATEGY-GUIDE.md) for the strategy API.

**Source specifications:** `Trading Binary Interface - MCX Enhanced Trading Interface (ETI) API v1.4.2 circular_536_2024.pdf` (in repo) and MCX EOBI API v1.2, circular MCX/CTCL/502/2023.

---

## 1. Scope

### 1.1 What this phase delivers

**A backtest that reads recorded MCX T7 EOBI data, rebuilds order books, runs a strategy, simulates fills, and produces a verifiable result.**

Concretely, at the end of this phase you can:
- Replay a full MCX trading day from a recording
- Prove the rebuilt book matches the venue's own snapshot cycles, at full depth, all session
- Run a strategy that quotes, gets filled, and accumulates position
- Get a report with P&L net of costs, queue position at each fill, and markout
- Run the same backtest twice and get byte-identical output

### 1.2 What is deliberately **not** in this phase

| Excluded | Why |
|---|---|
| Live Mode entirely — sockets, A/B arbitration, TCP recovery, ETI gateway | Backtest first. All of it swaps in at the Transport and venue edges without touching qtrade. |
| Quincy QED adapter | Blocked on the vendor specification (O5–O7). MCX-only until it arrives. |
| Journal and Recorder | Live-mode components. |
| RMS beyond pass-through | D34 — trait exists, implementation always returns yes. |
| Margin and cash | Later RMS implementation. |
| Bar aggregation | Phase 2, D35. |
| Options, equities, spread books | `InstrumentKind` taxonomy defined, only `Future` implemented (D37). |
| NUMA, CPU pinning, kernel bypass | D02, later phase. |

### 1.3 Consequence of excluding Quincy

The real strategy prices MCX Crude from CME and USD/INR. **Without the Quincy adapter that strategy cannot run.** This phase therefore validates the machinery using a **reference strategy that quotes off MCX's own book** — not a strategy you would deploy, but one that exercises every path: quoting, queue position, fills, inventory, skew, cancellation, cost.

When Quincy arrives, the real strategy is a new `Strategy` implementation. **No qtrade change.** That is the test of NFR-06.

---

## 2. Inputs

### 2.1 Recording

**Format:** raw MCX T7 EOBI messages as received, each preceded by a capture record.

```
┌────────────────────────────────────────────┐
│ capture_ts_mono   u64  monotonic ns        │
│ capture_ts_wall   u64  ns since Unix epoch │   (session anchor, FR-04)
│ source_id         u16  which stream        │
│ length            u16  payload bytes       │
│ payload           [u8] raw EOBI message    │
└────────────────────────────────────────────┘
```

**Requirements on the recording:**
- **All message types present.** Filtering to order and trade messages breaks D16 (FR-06).
- **Capture order preserved.** Do not sort by exchange timestamp (D05).
- **Per-stream sequence numbers intact** — EOBI numbers per channel, so gap detection is per channel.

### 2.2 Contract file

The MCX daily contract file defining every tradable instrument for that session.

**FR-16 is binding here:** the contract file **for the day being replayed**, archived with the recording. Instrument identifiers are not stable across days. Loading a current contract file to replay a past day maps events to the wrong instruments **silently** — no error, no crash, a backtest that traded something else.

**Hard failure at startup** if missing or if its date does not match the recording.

### 2.3 Run configuration

Single TOML file, two sections (D39). For backtest only `[run]` is populated.

```toml
[run]
mode            = "backtest"
start_date      = "2026-08-19"
end_date        = "2026-08-19"
session_mode    = "independent"        # or "continuous" (D29)
recording_dir   = "/data/mcx/2026-08-19"
contract_file   = "/data/mcx/2026-08-19/contracts.csv"
calendar_version = "2026-08"
bootstrap       = "full_replay"        # or "snapshot"
warmup          = "300s"

[run.latency]
model    = "fixed"                     # or "sampled"
seed     = 42
mcx_out  = "250us"
mcx_in   = "250us"

[run.simulator]
enforce_otr        = true
enforce_validation = true

[[run.strategy]]
name         = "reference_maker"
underlying   = "CRUDEOIL"
expiries     = 2
quote_size   = 1
base_spread  = 2                       # ticks
max_position = 10
skew_per_lot = 0.5
```

**Run identity is `(hash of [run], build/commit hash)`** (D22). Print it as the first line of every run.

---

## 3. Milestones

Each milestone is independently testable and gates the next.

---

### M1 — Reference data and instrument taxonomy

**Deliverable.** Load the daily contract file; expose instruments by a stable internal identity with a query interface.

#### FR-B01 — Instrument taxonomy

Define the full `InstrumentKind` enum now; implement `Future` only (D37).

```rust
pub enum InstrumentKind {
    Future  { underlying: Symbol, expiry: Date,
              contract_month: YearMonth, settlement: Settlement },
    Option  { underlying: Symbol, expiry: Date, strike: Price,
              right: Right, exercise: Exercise, settlement: Settlement },
    Equity  { series: Series },
    Spread  { leg1: InstrumentId, leg2: InstrumentId },
}

pub struct Instrument {
    pub id:            InstrumentId,      // interned, dense u32
    pub venue:         Venue,
    pub native_id:     i64,               // MCX SecurityID
    pub kind:          InstrumentKind,
    pub tick_size:     Price,             // i64 ticks
    pub lot_size:      i64,
    pub multiplier:    i64,
    pub freeze_qty:    i64,
    pub price_band:    Option<(Price, Price)>,
    pub currency:      Currency,
}
```

**Why the taxonomy now and not when options arrive:** CTT is levied on **turnover** for futures and on **premium** for options. The Cost Model needs `InstrumentKind` in phase 1, and it sits on the hot path (D23). A flat instrument record cannot answer it.

**Acceptance.** The same contract resolves to the same `InstrumentId` across a multi-day run despite `SecurityID` changing.

#### FR-B02 — Interned identity

`InstrumentId` is a dense `u32` index assigned at load, so downstream structures are arrays rather than maps. `native_id → InstrumentId` resolution happens once, in the Normalizer.

**Acceptance.** No hash lookup by `SecurityID` occurs after the Normalizer.

#### FR-B03 — Query interface

Strategies declare instrument filters as predicates (D32), so the query must support:

```rust
ctx.instruments()
   .venue(Venue::MCX)
   .underlying("CRUDEOIL")
   .kind_is_future()
   .front_n_expiries(2)
   .collect()
```

**Acceptance.** `front_n_expiries(2)` on a day with four live Crude contracts returns exactly the two nearest by expiry, ordered.

---

### M2 — MCX T7 EOBI decoder

**Deliverable.** Raw bytes → internal events. No book yet.

#### FR-B04 — Message dispatch

Every EOBI message carries a header: `BodyLen (u16, offset 0)`, `TemplateID (u16, offset 2)`, `MsgSeqNum (u32, offset 4)`. **The `TemplateID` determines the fixed message size** — dispatch on it, then decode the known layout.

**Verified facts from the spec — build against these:**
- **Little-endian**, fixed-length layouts, **no compression**, message padding for byte alignment
- Sequence numbers increment **per channel only**; `MarketSegmentID` appears in the packet header only
- Timestamps: **nanoseconds since Unix epoch, UTC**
- Prices: **integer including 8 decimals**
- `SecurityID`: **8-byte signed int**

**Template IDs to implement:**

| Template | ID | Notes |
|---|---|---|
| Order Add | 13100 | |
| Order Modify | 13101 | **priority lost** |
| Order Modify Same Priority | 13106 | **priority retained** |
| Order Delete | 13102 | publishes only `SecurityID`, `Side`, `Price`, `TransactTime` |
| Order Mass Delete | 13103 | |
| Partial Order Execution | 13105 | per-order fill |
| Full Order Execution | 13104 | per-order fill |
| Execution Summary | 13202 | aggregate match event |
| Product State Change | 13300 | session state (D16) |
| Instrument State Change | 13301 | session state (D16) |
| Mass Instrument State Change | 13302 | |
| Snapshot Order | 13602 | bootstrap |
| Heartbeat | 13001 | FR-01, FR-02 |
| Top Of Book | 13504 | **post-trading only** — not continuous |
| Product / Instrument Summary | 13600 / 13601 | |

Unknown template IDs must be **skipped safely** using `BodyLen`, never treated as an error.

**Acceptance.** Decode a full session; assert byte counts reconcile and no message is misparsed. Assert unknown templates skip cleanly.

#### FR-B05 — Order identity

**T7 has no broadcast order ID.** An order is identified by the composite `(SecurityID, Side, Price, priority_timestamp)`. This differs fundamentally from NSE MTBT, which carries an explicit day-unique order ID.

The internal event vocabulary therefore uses an **abstract order handle**, not an `order_id` field. Do not model it as an integer ID.

#### FR-B06 — Priority semantics carried explicitly

The internal event carries a `priority_retained: bool`.

**On MCX the decoder reads it** — template `13101` means priority lost, `13106` means retained. The spec is explicit: modifying to another price, or **increasing** quantity, changes time priority; **reducing** quantity does not.

*(When NSE is added later its decoder must infer this from matching rules, since MTBT does not publish it. The internal event is identical either way — this is the textbook justification for per-venue normalisation.)*

**Acceptance.** Assert `13106` produces `priority_retained = true` and `13101` produces `false`.

#### FR-B07 — Normalizer

Converts decoded messages to internal events:

| Field | Conversion |
|---|---|
| Time | ns since Unix epoch → `i64` ns (MCX already Unix/UTC; no conversion, unlike NSE's 1980 epoch) |
| Price | 8-decimal integer → `i64` ticks using the instrument's tick size |
| Instrument | `SecurityID` → interned `InstrumentId` |
| Side | `1 = Buy`, `2 = Sell` → `Side` |

**Never `f64` for price or time, anywhere, at any point.**

---

### M3 — Book builder

**Deliverable.** An `MboBook` per instrument, provably correct.

#### FR-B08 — MBO book construction

```rust
pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    fn depth(&self, n: usize) -> &[PriceLevel];
    fn qty_at_price(&self, side: Side, price: Price) -> i64;
}

pub trait MboBook: Book {
    fn queue_position(&self, handle: OrderHandle) -> Option<i64>;
}
```

`queue_position` exists **only** on `MboBook`. When MBP venues arrive, asking for it must fail to compile rather than return an estimate (D37, Figure 4).

**Structure guidance.** Price levels as a dense array indexed by tick offset over the day's price band — MCX publishes circuit limits, so the range is bounded, and this is far faster than a map. Each level holds a FIFO of order slots plus aggregate quantity and count. Orders in a slab allocated per instrument.

#### FR-B09 — Crossed books are legal

**`best_bid >= best_ask` is a normal transient state**, not a bug. On order-by-order feeds an aggressive order is published before the trade it causes. **Do not assert against it.** A book that panics on a crossed state will fail within minutes of real data.

#### FR-B10 — Book state machine

```rust
pub enum BookState { Uninit, Recovering, Ok, Stale }
```

Transitions and their triggers are FR-01, FR-02, FR-13, FR-14 in [ARCHITECTURE.md](ARCHITECTURE.md). In backtest, a gap in the recording is a **data fact**, not a recoverable condition — mark `STALE` and surface it rather than attempting recovery.

#### FR-B11 — Snapshot-cycle validation ← **the gate for this milestone**

**Compare the incrementally-built book against every arriving snapshot cycle, at full depth.**

This is the only real ground truth available, and it is free — the snapshot channel is required for bootstrap anyway. **`Top Of Book (13504)` cannot substitute**: the spec restricts it to post-trading through end of day, so it is an end-of-session checkpoint only.

A BBO-only comparison would miss a book that is correct at the top and wrong at level three.

**Acceptance — and this milestone is not done until it passes:**
> Replay a full session. At every snapshot cycle, the rebuilt book equals the snapshot at full depth, for every subscribed instrument. Zero divergences.

---

### M4 — Scheduler and clock

**Deliverable.** Deterministic event loop.

#### FR-B12 — The loop

```rust
loop {
    let Some(event) = scheduler.pop_earliest() else { break };
    clock.set(event.timestamp);        // time moves ONLY here
    dispatch(event);                   // handlers may enqueue more
}
```

#### FR-B13 — Total ordering

Events ordered by **`(timestamp, event_class, monotonic_seq)`**, sequence assigned at enqueue.

**Ties are guaranteed**, not hypothetical — a strategy timer landing on the same nanosecond as a market event is routine. An undefined tie makes the run non-reproducible, which invalidates every equality assertion in the test plan.

#### FR-B14 — Event sources

Market data is **one source among several**. All enqueue into the same queue:

| Source | Present in phase 1 |
|---|---|
| Market data from the Sequencer | yes |
| Strategy timers and alarms | yes |
| **Order arrival** at `T + outbound latency` | yes |
| **Report delivery** at `T + inbound latency` | yes |
| Session transitions | yes |
| Staleness and heartbeat timeouts | yes |
| Watchdog expiry | yes |
| Offload completion | scaffold only |

**A clock advanced only by market data cannot detect the absence of market data.** A five-second silence would jump the clock and never fire the staleness timer. This is why the Scheduler owns time.

#### FR-B15 — SimClock

`clock.now()` returns the current event's timestamp. Backtest time is free — the clock jumps event to event, never sleeps. Wall-clock runtime is throughput, unrelated to modelled latency.

**Acceptance.** Replay the same recording twice; assert byte-identical output.

---

### M5 — Cache, filter and dispatch

**Deliverable.** A no-op strategy runs a full day; throughput measured.

#### FR-B16 — Instrument filter

Applied **immediately after decode, keyed on `SecurityID`**, before normalisation and before any book work. An event for an unfiltered instrument costs one comparison.

A recording contains all of MCX. Building books for every contract while quoting two would dominate runtime.

**Declared programmatically by the strategy** at `on_start` as a predicate, resolved against the day's master (D32). The predicate must cover contracts the strategy will roll into — otherwise a mid-run subscription finds an empty book in a market that has traded for hours.

#### FR-B17 — Cache contents

| Holds | Notes |
|---|---|
| Books per filtered instrument | one instance, shared (D06) |
| Book state | per instrument |
| Session state | per venue and instrument (D16) |
| Own orders | the OMS state |
| Sub-account positions and P&L | per strategy (D08) |
| Firm aggregate | netted (D08) |
| Reference data | for the trading day |

**Read-only to strategies.** The ExecutionEngine and BookBuilder mutate; strategies read.

#### FR-B18 — Depth-scoped dispatch

Subscriber lists keyed by **`(instrument, depth)`**. A strategy subscribed at BBO is woken when the best bid or offer changes and **not otherwise**.

The full book is still maintained — **subscription governs waking, not access**. Deeper levels remain reachable on demand through the Cache.

Static dispatch, **no allocation** on this path (NFR-05).

**Acceptance.** No-op strategy replays a full session. Record messages/second. Assert zero allocations in the dispatch and book paths under a profiler.

---

### M6 — Simulated Exchange ← **the highest-risk component**

**Deliverable.** Fills that are defensible.

#### FR-B19 — Independence

The Simulated Exchange builds **its own books** directly from the normalized event stream, for the whole filtered set (D32). It has **no read path into the Cache**.

**Why this is not negotiable:** in production the venue is not inside your process. Your feed-derived book can go `STALE` after a gap — **a real exchange does not become uncertain because your receiver dropped packets**. A simulator reading your Cache inherits your corruption and fills you against a book that never existed.

Interface is exactly two directions: **order commands in, execution reports out** — identical to what the live gateway will present, so the ExecutionEngine cannot tell them apart.

#### FR-B20 — Latency model

```rust
pub trait LatencyModel {
    fn outbound(&mut self, venue: Venue) -> Duration;
    fn inbound(&mut self, venue: Venue) -> Duration;
}
```

Two implementations: `Fixed` and `Sampled` (seeded from `[run].latency.seed`). Configured **per venue and per direction** — feed-in and order-out are different paths.

**An order submitted at T does not reach the venue until `T + outbound`.** Every market event in that window is processed first. This is what makes queue position honest, and it is expressible only because the Scheduler owns time.

**Set outbound pessimistically.** Feed latency is measurable from capture-versus-exchange timestamps; **outbound latency is not observable from market data at all** and remains an assumption until real round-trips exist.

#### FR-B21 — Queue position

On order arrival, insert at the **back of the queue at that price** — behind everything that arrived during the flight time.

Track `qty_ahead` per resting order. Decrement on executions and deletions ahead of it. **Order-by-order data makes this exact**, which is the entire reason for using MBO on the venue we quote.

#### FR-B22 — Order types

| Type | Behaviour |
|---|---|
| `Limit` + `Day` | rests |
| **`BookOrCancel`** | **rejected outright if it would cross — never filled** |
| `IOC` | fills what it can, remainder cancelled |
| `MarketToLimit` | executes available, **residual rests as a limit at the traded price** — does not sweep |

**BOC and Market-to-Limit are the two that silently inflate results if modelled wrongly.** Treating BOC as an ordinary limit books aggressive fills production would have rejected, and those cluster in fast markets where the estimate matters most.

#### FR-B23 — Modify semantics

Reducing quantity **retains** queue priority. Price change or quantity increase **loses** it — the order goes to the back of the queue at the new level.

For a market maker this is most of the P&L, so it must be exact.

#### FR-B24 — Simulator invariants ← **run these on every backtest**

Automatic assertions requiring no additional data. Strongest first:

> **Simulated fills at a given price and time must not exceed the volume that actually traded at that price in the recording.**

Filling 100 lots where 20 traded means the simulator **fabricated liquidity**, and every result built on it is worthless.

Supporting invariants:
- A `BookOrCancel` order that would cross is rejected, never filled
- Fill price is at or better than the limit price
- Queue position never improves except through consumption ahead
- `MarketToLimit` residual rests rather than vanishing
- Simulated OTR and message-rate counters never exceed configured venue limits

**Acceptance.** Run a strategy with analytically predictable behaviour and verify fills by hand. Then assert all invariants hold across a full session.

#### FR-B25 — Also modelled

STP across strategies · OTR enforcement (D19) · MMP cancellations · watchdog cancellations (D28) · Lean order cancellation at end of day.

---

### M7 — Execution, accounting and reporting

**Deliverable.** A complete, interpretable run result.

#### FR-B26 — Order state machine

Eleven states with the transitions in [STRATEGY-GUIDE.md](STRATEGY-GUIDE.md) §7a.

**Two that are easy to get wrong:**
- **`PendingCancel → Filled` is a real path**, not an edge case. Your cancel is in flight; an aggressor takes the order you were pulling. Those fills are adversely selected by construction.
- **`Denied` is terminal and the order never left qtrade** — distinct from `Rejected`, which means the venue refused it.

#### FR-B27 — Three gates, two rejection paths

Order flow: **Validation → RMS → OTR governor → venue.**

- **Local gate rejections return synchronously** from `submit()`. No time passed; the book is unchanged; correcting and resubmitting immediately is valid.
- **Venue rejections arrive as events.** A full round trip elapsed; the market has moved.

The ExecutionEngine assigns a client order ID and creates the record **before** the gates, so a locally-rejected order still exists, still transitions to `Denied`, still appears in reporting, and still counts toward OTR if it reached that gate.

**Phase 1: Validation enforces tick size and freeze quantity. RMS is a trait that always returns yes (D34).**

#### FR-B28 — Client order ID

```
ClOrdId = (session_id, counter)
```

`session_id` is **injected** — a deterministic value from `[run]` config in backtest, process start time in live. **Reading a wall clock here would break FR-12**, since the same backtest run twice would produce different IDs.

`counter` is required because **the SimClock does not advance within a callback** — a market maker submitting bid and ask in one `on_book()` gets the identical timestamp for both.

*Verify MCX ETI's `ClOrdID` field type and width before fixing the encoding.*

#### FR-B29 — Two-level accounting

**Per-strategy sub-accounts** for position, inventory, P&L attribution — a strategy skews on **its own** inventory, never the firm's.

**Firm aggregate** netted across strategies, for exposure and (later) margin.

Strategies can read both. Reading the firm view lets a strategy widen or stand down gracefully rather than discovering a limit through rejection.

#### FR-B30 — Cost model

```rust
ctx.cost().round_trip(instrument, qty, side) -> Cost
```

Queryable **pre-quote on the hot path**, and applied to realised fills so the quoting assumption and the accounting cannot disagree.

**Direction-asymmetric by construction:** CTT falls on the **sell** side, stamp duty on the **buy** side. Several components are turnover-percentage based and scale with price. Cost is not a flat per-lot number.

**No market-maker concession applies** (D23) — full retail rates on both legs. **Round-trip cost is a hard floor on the quotable spread.**

#### FR-B31 — Reporting

**Tier 1, always:** P&L gross and net at both levels, inventory over time, markout distribution, OTR consumed, message counts, invariant violation counts. Columnar, so sweeps aggregate.

**Tier 2, switchable:** every order command and response with rejection reasons distinguished; per-fill records carrying **queue position at fill**, **markout at fixed horizons**, a **spread-improving flag**, and realised cost.

**Tier 3, strategy series:** named scalars published by the strategy — fair value, skew, quote width.

**Queue position and markout on fill records are not optional and not deferrable.** Retrofitting them means re-running everything you have already trusted.

Every output embeds the full `[run]` spec and both identity hashes (D22).

---

## 4. Reference strategy

A deliberately simple strategy to exercise the machinery. **Not for deployment.**

**Behaviour:** quote both sides around the MCX mid at a configured spread; skew on own inventory; stop quoting a side at the position limit; cancel all on session change or book-state change.

**What it exercises:** subscription and filtering · depth-scoped waking · order submission through all three gates · queue position · fills and partial fills · inventory and skew · cost queries · cancellation paths · reporting including markout.

**What it deliberately does not have:** any alpha. Its P&L is expected to be negative after costs. **That is the correct result** — a market maker quoting a fixed spread around the mid with no signal loses to adverse selection, and seeing that in the report is evidence the simulator is honest.

> **A positive result from this strategy is a bug**, not a discovery. Investigate the simulator before believing it.

---

## 5. Test plan

### 5.1 Milestone gates

| Milestone | Gate |
|---|---|
| M1 | Same contract → same `InstrumentId` across days |
| M2 | Full session decodes; byte counts reconcile; unknown templates skip |
| M3 | **Book equals every snapshot cycle at full depth, all session, zero divergences** |
| M4 | Same recording replayed twice → byte-identical output |
| M5 | No-op strategy runs a full day; zero allocations on the hot path |
| M6 | Hand-verified fills match; all invariants hold |
| M7 | Report produced; run identity printed and embedded |

### 5.2 Standing assertions

Run on every backtest, not just in CI:
- All FR-B24 simulator invariants
- Book equals snapshot cycle at full depth
- Book state never `Ok` while a gap is outstanding
- No order submitted while `can_quote()` is false

### 5.3 Determinism

Two runs, same input, same `[run]` config → **byte-identical** output. In CI. This is the foundation every other assertion rests on.

---

## 6. Definition of done

- [ ] Full MCX session replays end to end
- [ ] Book matches every snapshot cycle at full depth, zero divergences
- [ ] Same run twice → byte-identical output
- [ ] Reference strategy quotes, fills, accumulates inventory, skews
- [ ] All simulator invariants pass across a full session
- [ ] Report shows P&L net of costs, queue position per fill, markout distribution
- [ ] Run identity `(config hash, build hash)` printed and embedded in output
- [ ] Zero allocations in dispatch and book paths under a profiler
- [ ] Contract-file mismatch fails at startup with a clear diagnostic
- [ ] No wall-clock read, unseeded randomness, or direct I/O anywhere in qtrade or the reference strategy

---

## 7. Do this first — before M1

**Compute the minimum quotable spread.** Take the cost model components — exchange transaction charges, SEBI fees, CTT on the sell leg, GST, stamp duty on the buy leg — and compare the round-trip cost against the historical distribution of MCX Crude bid-ask spreads.

**This needs no engine.** It is a spreadsheet exercise on historical data.

**Why before M1:** with no market-maker fee concession, round-trip cost is a hard floor on the spread you can profitably quote. If one tick does not clear it, one-tick market making on Crude is loss-making by construction, and the phase-1 premise needs revisiting **while revisiting is still cheap**.

Either answer is valuable. Getting it during M6 instead of before M1 is not.

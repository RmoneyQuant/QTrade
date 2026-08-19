# Trading Engine — Architecture Decisions

**Status:** Design complete for phase 1. All 32 decisions settled.
**Last updated:** 2026-08-19

> **This is the decision log — the *why*.** It records what was decided, what was rejected, and the reasoning, so a future reader does not have to reconstruct it.

**Document set**

| Document | Answers |
|---|---|
| [CONTEXT.md](CONTEXT.md) | **What things are called** — the glossary |
| [ARCHITECTURE.md](ARCHITECTURE.md) | **What the system is** — requirements, components, event flow, build order |
| **ARCHITECTURE-DECISIONS.md** *(this file)* | **Why it is that way** — decisions D01–D32 |
| [OPEN-QUESTIONS.md](OPEN-QUESTIONS.md) | **What is still unresolved** — with question sets for Quincy and MCX |

---

## 1. What we are building

A single engine that runs **the same compiled code** in backtesting and in live trading. Only two thin edges swap: where market data bytes come from, and where orders go.

The engine is **multi-venue and exchange-agnostic at its core**. Venue-specific knowledge lives entirely in per-venue adapters at the edge.

### Phase 1 objective

**Market making in MCX Crude Oil and Natural Gas futures**, priced off CME and a USD/INR rate.

| Venue | Role | Book type | Depth | Source |
|---|---|---|---|---|
| **MCX** | Trading venue — we quote here | **MBO** (order-by-order) | full | Direct: T7 EOBI multicast |
| **CME** | Price signal | MBP | **5 levels** | Quincy QED |
| **DGCX / GME** | USD/INR rate signal | MBP | **1 level (BBO)** | Quincy QED |

**NSE** (Cash Market and F&O) is deferred to a later phase. Currency Derivatives and Commodity segments of NSE are out of scope entirely.

**Options market making** is deferred. The architecture must accept option pricing and volatility modelling as **plug-in modules** later, without restructuring.

---

## 2. Canonical terminology

> **Moved to [CONTEXT.md](CONTEXT.md)**, which is the single source of truth for vocabulary.

Terms retired during design, deliberately unused across this document set:

| Retired | Use instead | Why |
|---|---|---|
| "message bus" | **Event Dispatcher** / **Control Dispatcher** | The design has no queueing, async delivery or thread-crossing dispatch (D07) |
| "data engine" meaning a file reader | **Transport** | Collided with **Data Engine**, which owns subscriptions and book state |
| "LOB" | **Order Book** | |
| "simulated fill engine" | **Simulated Exchange** | It is a venue, not a calculator (D10) |
| "core" (lowercase) | **Core** or **CPU core** | The two senses collided and caused real confusion mid-design |
| "backtester" | **Engine** (system) / **Backtest Mode** (mode) | Meant both the system and one of its modes |

---

## 3. Decision log

### D01 — Implementation language: Rust
**Decision:** Rust for the Core, adapters and Simulated Exchange. 

**Why:** Python is eliminated because order-book construction is a sequential state machine — data-dependent pointer chasing with no vectorisation available, so numpy/numba offer no escape. Between Rust and C++, the deciding factors were that MTBT-style packed structs read from network buffers are exactly where memory bugs live, and that **a corrupted book does not crash — it produces plausible wrong fills** that could be trusted for weeks. Greenfield repo, so no legacy C++ pull.

**Trade-off accepted:** C++ has deeper hiring pools in Indian trading infrastructure and first-class vendor support for kernel-bypass libraries.

---

### D02 — Latency regime: sub-millisecond now, kernel bypass later
**Decision:** Colocated sub-millisecond userspace with ordinary sockets for phase 1. Kernel bypass is a committed later phase, not a possibility.

**Consequences:**
- The `LiveTransport` boundary must stay genuinely clean — no socket-specific assumption may leak into the Decoder or Core, or bypass becomes a rewrite instead of a swap.
- NUMA placement and CPU-core pinning are **deferred, not cancelled**. They return in the same phase as bypass.

**Noted risk:** Quincy microwave data buys a latency edge measured in microseconds; ordinary kernel sockets spend a meaningful part of it. Acceptable as build-order (correctness first), but it makes the bypass phase more urgent than "eventually."

---

### D03 — Strategies are written in Rust
**Decision:** Single tier, Rust, same code in backtest and live.

**Why:** Market making requotes on every relevant book change; an interpreter in that loop is not viable. More importantly, a Python research tier would mean every promotion to production is a **rewrite** — precisely the unvalidated language crossing this architecture exists to prevent.

**Mitigation:** The `Strategy` trait is designed so authors never write an explicit lifetime or generic bound. Python stays for orchestration, parameter sweeps and post-run analysis — around the engine, never inside it.

**Rejected:** A two-tier system with "research-only" Python strategies.

---

### D04 — Single-threaded Core, separate ingestion threads
**Decision:** The Core runs on one thread. Feed Adapters run on their own threads doing receive → decode → normalise, pushing internal events to the Core through bounded queues.

**Why:** A single-threaded Core makes backtests reproducible by construction. Adapter threads keep decode work off the Core thread without affecting ordering.

**Also decided:** An explicit **offload mechanism** for expensive work (e.g. future volatility surface recalibration) — the result returns as a *scheduled event*, not a blocking call, so determinism holds and the quoting loop is never stalled.

**Backtest parallelism** comes from running many processes (different days, parameter sets), never from threading inside one run.

---

### D05 — Deterministic ordering: two mechanisms
**Problem:** Multiple adapter threads feeding one queue makes cross-venue interleaving depend on thread timing, destroying reproducibility.

**Decision — both mechanisms, for different jobs:**

1. **Post-merge journal (live)** — the journal is written *after* merge, in the exact order the Core consumed events. Replaying it needs no merge at all. **Used only for live-vs-backtest parity verification.**
2. **Deterministic k-way merge** — each adapter has its own ring; the Sequencer pops the earliest by `(capture_timestamp, source_id, sequence_number)`. **Used for strategy development**, and required for vendor-supplied per-venue files.

**Merge on capture timestamp, never exchange timestamp** — venue clocks are not comparable, and exchange time would hand strategies a cleaner cross-venue view than they can ever have live.

**Requirements this creates:**
- Every packet must carry a **capture timestamp from a single host clock**, stamped on receipt.
- **Capture where you consume.** Recording CME in Chicago and replaying it in a Mumbai-context backtest would hand the strategy information it can never have live.
- Quiet sources must not stall the merge — EOF markers in backtest, heartbeats as watermarks in live.
- Backpressure differs by mode: a full ring in backtest just waits; in live it means dropped data and must alarm and mark books `STALE`.

**Accepted asymmetry:** live takes what is available on arrival and cannot wait for a quiet venue. Mechanism 1 is what stops this from being an untested gap.

---

### D06 — One order book per subscribed instrument, shared
**Decision:** One book instance per subscribed instrument, owned by the BookBuilder, exposed read-only through the Cache. Strategies maintain their own **derived** state (microprice, imbalance, fair value) on top.

**Why:** The original case for per-strategy books rested on avoiding lock contention and preserving CPU-core locality — both arguments about *threads*. With a single-threaded Core (D04) neither applies. Duplicating books would multiply the system's most expensive operation by strategy count, on the one thread that also runs all strategy logic, and would introduce N books that are supposed to be identical but might not be.

---

### D07 — No message bus
**Decision:** Two named dispatch mechanisms, both direct calls on the single thread:
- **Event Dispatcher** — market data, statically dispatched to per-instrument subscriber lists, zero allocation.
- **Control Dispatcher** — commands, execution reports, session changes, alerts, via typed handler lists so observers are added by wiring rather than by editing publishers.

**Routing knowledge lives in startup wiring, never inside either dispatcher.**

**Why:** A message bus buys decoupling across threads — locks, queues, scheduled delivery. With a single-threaded Core none of that machinery has work to do. The term itself is retired because it implies queueing and async semantics this design deliberately does not have.

---

### D08 — Two-level portfolio model
**Decision:**
- **Per-strategy sub-accounts** — position, inventory, skew basis and P&L attribution.
- **Firm-level aggregate** — exposure limits, self-trade prevention, margin.
- **Strategies can read both.**

**Why per-strategy:** if two market makers shared one inventory number, each would see the other's fills as its own and skew against flow it never traded.

**Why firm-level is mandatory:** five strategies each inside a 100-lot limit can put the firm 500 lots long; STP must be checked *across* strategies because the exchange sees one member and one session; and margin is computed on the net firm position.

**Consequence accepted:** a strategy's behaviour depends on what else is running, so **backtest runs must declare the full strategy set**, and single-strategy results are not predictive of multi-strategy live behaviour.

---

### D09 — Cross-venue fair value lives in the strategy
**Decision:** Fair value — MCX Crude priced from CME WTI and the DGCX USD/INR rate — is computed **inside the strategy**, not in an engine component.

**Why:** Domain pricing models in the engine erode the genericness the platform exists to provide.

**Escape hatch if several strategies need the same model:** share it as a **library crate** that strategies link — consistency without engine coupling.

**Same principle applies to:** skew (strategy), roll policy (strategy), and later option pricing and volatility surfaces (plug-in modules, not engine internals).

---

### D10 — Simulated Exchange is fully independent
**Decision:** The Simulated Exchange builds **its own order books** directly from the normalized event stream. It has **no read path into the Cache** and no shared state with the Core. The only interface is order commands in, execution reports out — identical to the live gateway.

**Why:** In production the venue is not inside your process. If the simulator read your Cache you would create a coupling that does not exist live. Sharper still: your feed-derived book can go `STALE` after a packet gap, but **a real exchange does not become uncertain because your receiver dropped packets** — a simulator reading your Cache would inherit your corruption and fill you against a book that never existed.

**Cost is smaller than it appears:** the simulator only needs books for instruments you actually *trade*, not everything you subscribe to.

**Also gained:** the simulator becomes unit-testable against synthetic order flow with zero Core dependency.

---

### D11 — Order mechanism: plain orders
**Decision:** Plain individual orders. Verified from MCX ETI v1.4.2 that **no mass-quote or quote-entry interface exists** — this is not a simplification, it is the only mechanism MCX offers.

**Extensibility built in now** so a quote entity can be added later without restructuring:
- The venue command interface is an enum that can gain a `Quote` variant without touching existing paths.
- **OTR counters count *messages sent*, not *orders sent*.**
- STP operates on "my live price per side per instrument," which is already quote-shaped.

---

### D12 — Order types in scope
**In scope for phase 1:**

| Type | Purpose |
|---|---|
| **Limit, Day** (`OrdType=2`, `TIF=0`) | The quoting order |
| **Book-or-Cancel** | Post-only. Rejected if it would execute immediately |
| **IOC** (`TIF=3`) | Deliberate liquidity taking when hedging or flattening |
| **Market-to-Limit** (`OrdType=5`) | Emergency flatten only |

**Out of scope, recorded as a decision not an oversight:** Stop Market (`3`), Stop Limit (`4`), Auction Buy In / Sell Out (`6`/`7`), iceberg.

**Two behaviours the Simulated Exchange must model exactly:**
- **BOC that would cross is rejected, not filled.** Treating it as an ordinary limit silently books aggressive fills that production would reject — and those cluster in fast markets, where the P&L estimate matters most.
- **Market-to-Limit converts unfilled remainder to a limit at the traded price.** It does not sweep the book. Modelling it as a plain market order would show a clean sweep where production leaves you resting.

---

### D13 — Lean orders for quoting
**Decision:** Lean orders for quoting; Standard orders where an order must survive a reconnect. The order model carries an explicit `Standard | Lean` attribute from day one.

**Why:** Lean orders carry less broadcast and bookkeeping overhead, which is the point when requoting constantly. They are **always non-persistent**, so a session drop pulls your quotes automatically — a dead-man's switch a market maker wants rather than a cost.

**Constraints this imposes:** Lean orders can only be Day, IOC or Session validity — **GTC and GTD are Standard-only**. Lean orders are cancelled at end of business day and after a market reset.

**The Simulated Exchange must reproduce** quote disappearance on session loss, market reset and end of day.

---

### D14 — Bootstrap: both paths, replay by default
**Decision:**
- **Full-day replay is the default** for backtest runs.
- **Snapshot bootstrap is built and exercised in Backtest Mode** as a first-class option.
- **Explicit warmup window**, configured per run, identical semantics in both modes: the strategy receives events and updates state but may not quote.

**Why build snapshot bootstrap even though replay is the default:** live *must* support it — a process restarting at 3pm has to join a market in progress. That path is intricate and runs under pressure with money at risk. If backtest always replayed from start of day, **the bootstrap path would never execute outside production**.

**Standing invariant to test:**
> Book state after snapshot-start at T then replay to T′ **must equal** book state after full replay from start-of-day to T′.

---

### D15 — Instruments: strategies name specific contracts
**Decision:** Strategies subscribe to **specific contracts**, not rolling concepts like "front month." The engine supplies instrument **metadata** — venue, underlying, contract month, expiry, tick size, lot size, multiplier, freeze quantity — and a **query interface** (e.g. all live MCX Crude contracts ordered by expiry). **Roll policy lives in strategy code.**

**Why:** front-month resolution is domain policy. Worse, since fair value spans venues with different contract calendars, an engine that resolved "front" would be making an implicit claim about which CME contract corresponds to which MCX contract — a claim belonging to whoever owns the pricing model (D09).

**Consequence accepted:** **subscriptions are dynamic.** A strategy can subscribe and unsubscribe mid-run as it rolls, so the Cache must handle books appearing and disappearing during a session. This is true in live regardless.

---

### D16 — Session state: per venue, identical across modes
**Decision:** For any given venue, session state comes from **the same source in both modes**:
- **MCX** — exchange messages (`Product State Change 13300`, `Instrument State Change 13301`), in live *and* in backtest, because the recording contains them.
- **CME / DGCX via Quincy** — whatever Quincy provides; if it publishes no state, then calendar plus data-presence inference, **in both modes**.

**Rejected:** calendar in backtest, exchange messages in live. That would put a **mode-specific branch inside the Core**, and three things follow: unscheduled halts (`110 Volatility Interruption`, `105 Product State Halt`) would never appear in a backtest; open/close timing would differ from real jitter; and it repeats the D14 mistake of leaving the least-tested code on the most critical path.

**The calendar's actual jobs, in both modes:** run planning (which trading days exist) and **anomaly detection** (the expectation against which reality is checked — this is what makes FR-04 meaningful).

**Two states are separate and both live in the Cache:**
- **Book state** — `UNINIT / RECOVERING / OK / STALE` — is my view trustworthy?
- **Venue session state** — open, closed, halted, auction, maintenance break — is the market live?

A CME book during the maintenance break is perfectly `OK` and completely frozen. A market maker that cannot tell these apart will either quote off an hour-old signal or stand down every time a book goes quiet.

**Hard requirement:** recordings must preserve product and instrument state messages. If capture filters to order and trade messages for size, this approach collapses.

---

### D17 — Two gates: Order Validation now, Risk Limits later
**Decision:** These are categorically different and must not be conflated.

**Order Validation** — stateless, per-order, reference-data driven. Answers *"will the exchange accept this order at all?"*
- **Phase 1: tick size and order freeze quantity.**
- Likely additions: lot-size multiple, price band / circuit limits, maximum order value.
- **Both sides**, same pattern as OTR (D19): the Simulated Exchange enforces because the real exchange does; the engine pre-checks to avoid burning a round trip and an OTR message.

**Risk Limits** — stateful, portfolio-scoped, policy-driven (position, P&L, exposure, OTR budget). **Deferred, not rejected.** When built, it should be a **generic `(metric, scope, threshold, action)` framework** with metrics as registered providers, so option Greeks limits later mean registering a new metric rather than editing engine internals.

**Flagged, unresolved:** going live with market making and **no max-net-position and no daily-loss kill switch** is materially different from deferring them in backtest. A quoting strategy with a bug accumulates position quickly, and Lean orders protect against a disconnect but not against a strategy working exactly as coded and wrong.

---

### D18 — Latency model is swappable
**Decision:** A `LatencyModel` trait with two phase-1 implementations — **`Fixed`** and **`Sampled`** (drawn from a distribution). Configured **per venue and per direction**, since feed-in and order-out are different paths. The sampled variant is **seeded from run configuration** so runs stay reproducible. Trace replay can be added later as a third implementation without touching callers.

**Asymmetry to respect:** feed latency is measurable now from capture versus exchange timestamps (with a clock-offset caveat — trust the distribution shape, not the absolute mean). **Order latency is not observable from market data at all** and remains a parameter until real round-trips exist. Set it pessimistically.

---

### D19 — OTR accounted on both sides, independently switchable
**Decision:** Order-to-trade ratio is tracked in **two independent counters**:
- **Simulated Exchange** — enforcement. Rejects or penalises as the venue would.
- **Engine** — governance. Throttles the strategy before it sends.

They **do not share state**, preserving the venue independence of D10. Each counts its own flow.

**Binary switch per side**, independently — "enforcement off, governor on" is a useful configuration for exploring unconstrained strategy behaviour.

---

### D20 — Failure semantics
**Decision:**
- **Live: isolate.** Halt the failing strategy, cancel all its orders, let others continue.
- **Backtest: fail-fast.** Stop the run loudly rather than produce results that might be trusted.
- **Both are configuration, not code paths** — different defaults per mode. A live deployment can be configured fail-fast during initial rollout.

**Why isolation is safe:** strategies **cannot corrupt engine state**. They hold shared references into the Cache and submit commands; they never mutate books, positions or the scheduler.

**Obligation:** `cancel_all_for_strategy` must exist, be reliable in both modes, and be honoured identically by the Simulated Exchange. Lean orders cover session loss, but a **strategy halt is not a session loss** — those orders keep resting unless something explicitly pulls them.

**Engine data errors are a separate category:** a malformed packet or a detected book inconsistency marks the instrument `STALE` and triggers recovery — the same path as a sequence gap. Never process termination.

---

### D32 — Instrument filtering is declared programmatically by the strategy
**Decision:** The strategy declares its instrument filter **in code**, resolved against the day's instrument master via D15's metadata query at `on_start`. The filter is applied **immediately after decode, keyed on `SecurityID`**, before normalisation and before any book work. Both the Core **and the Simulated Exchange** build books for that same set.

**Why it belongs in the strategy:** D15 already puts roll policy there, so the strategy is the component that knows which contracts it will want — *including ones it has not rolled into yet*. A predicate such as "CRUDEOIL, front two expiries" naturally includes next month's contract, so **when the strategy rolls, the book is already built with full history**. No universe declaration duplicated between run config and strategy code.

**The trap this avoids:** with a naive filter, a strategy subscribing mid-run to a previously-filtered contract would receive an empty book in a market that had been trading for hours. Declaring the filter up-front as a predicate covering future contracts removes the case entirely.

**Why it matters for throughput:** a recording contains all of MCX — every product, every contract. Building books for everything while quoting two contracts would dominate backtest runtime. An event for an unfiltered instrument should cost one comparison.

**Amends D10:** the Simulated Exchange builds books for the **whole filtered set**, not only instruments actually quoted. A lazy "build on first order" approach would hit the same trap — inserting an order into a book that missed every prior event. Since the filter is already narrow, the waste is small and bounded, and avoiding a second declaration that can silently disagree with the first is worth more than the saved updates.

---

### D31 — Simulated Exchange validation: four layers
**Decision:** All four layers, with **Layers 1 and 2 built before the first strategy**.

**Why this component gets dedicated treatment:** the Simulated Exchange is the part of the system that can lie *silently*. A wrong book crashes or visibly diverges; a wrong fill model produces plausible numbers that get trusted for months and then bet on. There is no natural ground truth, because no recording exists of what would have happened had your orders been present.

**Layer 1 — book correctness.** Compare the incrementally-built book against each arriving **snapshot cycle** — full depth, continuously through the session. Assertions in Backtest Mode (divergence fails the run), metrics in Live Mode (divergence alerts). `Top Of Book (13504)` serves as an end-of-session BBO checkpoint only, since it is not disseminated during continuous trading (§4).

*A BBO-only check would miss a book that is correct at the top and wrong at depth three. The snapshot channel is already required for bootstrap (D14), so this adds comparison logic rather than infrastructure.*

**Layer 2 — automatic simulator invariants**, checked on every backtest run with no additional data. Strongest first:

> **Simulated fills at a given price and time must not exceed the volume that actually traded at that price in the recording.**

Filling 100 lots where 20 traded means the simulator fabricated liquidity. Supporting invariants: a **Book-or-Cancel** order that would cross must be **rejected, never filled** (D12); fill price **at or better than** limit; **queue position never improves** except through consumption ahead; **Market-to-Limit** residual becomes a resting limit rather than vanishing (D12); simulated **OTR** and **message-rate** counters never exceed configured venue limits (D19).

**Layer 3 — hand-traceable scenarios.** Synthetic order flow small enough to verify on paper, kept as regression tests. Feasible precisely because phase 1 is a **scalar-inventory futures contract** — this would not work against an option chain.

**Layer 4 — shadow comparison, once live.** Run the Simulated Exchange in parallel during live trading, fed the same market data and the same orders actually sent, and diff simulated fills against real fills. **The only layer measuring against reality rather than against our own assumptions** — the venue-level counterpart to FR-12.

**Why Layers 1 and 2 come first:** they are cheap, need no live access, and a simulator that fabricates liquidity is worse than no backtest, because its results carry false confidence and get acted on.

---

### D30 — Clock discipline: monotonic for ordering, wall clock for meaning
**Decision:** **Two clocks with different jobs.**
- **Monotonic** — never steps, never goes backward. Used for **all ordering, sequencing and interval measurement**.
- **Wall clock (UTC)** — used for display, correlation against exchange timestamps, and reporting.

A capture timestamp carries a **monotonic value for ordering**, plus a **single wall-clock anchor per session** to place it in absolute time.

**The hazard this prevents:** NTP corrections step the system clock, including *backward*. With wall-clock capture timestamps, a mid-session correction would silently break D05's merge ordering (events captured later carrying earlier timestamps), violate the Scheduler's invariant that time only moves forward, and produce negative latency measurements.

**Accuracy requirement, which bears directly on D18:** NTP achieves millisecond-scale accuracy. The target is sub-millisecond, and feed latency must be *measured* to calibrate the latency model — **NTP's error would exceed the quantity being measured**, making calibration data mostly clock error. **PTP** is required for the `Sampled` latency model to be grounded in anything real; it reaches sub-microsecond and is commonly available in colocation, often from the exchange.

**NIC hardware timestamping** is the target for FR-04 where the card supports it — stamping on arrival excludes OS scheduling jitter, which is precisely the variable component being measured around.

**If PTP proves unavailable** in the MCX colocation, the architecture is unchanged but D18's latency calibration stays bounded by clock error, and the latency model remains a parameter rather than a measurement for longer.

---

### D29 — Multi-day runs: `independent` or `continuous`, declared per run
**Decision:** A run declares its session-boundary semantics in configuration. **Default for phase 1 is `independent`.**

**`independent`** — a run is one trading day; start flat, end flat. Preserves D04's parallelism: a month runs as thirty processes, deterministic, scaling linearly. Warmup occurs each morning.

**`continuous`** — one process spans the date range and position, P&L and strategy state carry across days. Models overnight inventory and long-horizon strategy state (volatility estimates, basis models). **Gives up cross-day parallelism**, since day fifteen depends on day fourteen. Warmup occurs once, which is more realistic but means a warmup bug on day one silently contaminates the whole range.

**Tension made explicit:** D04 assumed days were independent when it made multi-process sweeps the parallelism model. That assumption holds only for `independent` runs. Naming it here rather than discovering it during the first month-long backtest.

**Always resets at a session boundary regardless of mode:** order books (per-session, rebuilt from scratch), Lean orders (cancelled at end of business day by MCX itself, D13), instrument masters (reloaded per trading day, D15).

**Daily settlement is modelled in both modes.** MCX marks futures to the daily settlement price and settles mark-to-market in **cash**. A position held across days has its P&L **realised daily**, with real cash and margin consequences. Computing P&L only at final close gets the timing wrong, understates margin usage, and could miss a margin breach entirely. This applies equally to a flat-by-EOD market maker, whose intraday MTM and margin still move.

---

### D28 — Declared-dependency watchdog for stale signals
**Decision:** Strategies **declare their signal dependencies** at subscribe time — which inputs their quoting depends on, and a staleness tolerance for each. The engine runs a **generic watchdog** over declared dependencies and **cancels** the strategy's orders on breach, independent of whether the strategy notices.

**The scenario, which is routine rather than exceptional:** MCX Crude open and being quoted at 21:30 IST when CME hits its maintenance break, or the Quincy microwave link degrades (microwave is weather-sensitive — that is the trade for the latency), or DGCX closes and USD/INR stops updating. Fair value is now computed from a price that has stopped moving, while you quote into a market still trading. This is how a market maker gets picked off systematically.

**Why not leave it entirely to the strategy** (which D09 and D16 would suggest): the scenario where quotes most need pulling is the scenario where the strategy is misbehaving. A strategy that is stuck, looping, or has a bug in its own staleness check keeps quoting *because* it is broken. Depending on the possibly-broken component to notice it is broken is the wrong direction for a safety behaviour.

**Why this keeps the platform generic:** policy stays in the strategy — *which* dependencies, *what* tolerance. The enforcement mechanism is engine code that knows nothing about Crude, WTI or exchange rates; it watches declared inputs against declared thresholds. Same separation as D17's validation mechanism versus configured limits.

**Two requirements for it to be honest:**
- The watchdog must distinguish **stale** from **legitimately quiet**. CME's maintenance break is scheduled, so the D16 calendar suppresses the watchdog rather than having it fire predictably every night.
- **The Simulated Exchange must reproduce watchdog cancellations exactly**, or backtests keep quoting through link degradations that production would have pulled you out of — the same class of error as failing to model MMP (D21).

**Cancellation, not a warning.** A market maker with stale inputs should be out of the market, not quoting with a flag set.

**Declared dependencies include the trading venue's own book**, not only external signals. A stale MCX book is the most dangerous dependency of all — quoting into a market whose state you have lost is worse than quoting off a stale reference price.

---

### D27 — Deployment configuration is separate from run configuration
**Decision:** Network endpoints and session details are **deployment configuration** — changeable at will, never compiled in. Critically, this is a **separate file from the run configuration of D22**.

**Why the separation matters, and it is subtle:** D22 makes run identity `(config hash, build hash)` so results are comparable. If multicast addresses lived in the run config, **moving to a different colocation rack would change your run hash and make previous results appear incomparable** — which is wrong, because the endpoint has no effect on outcomes. Conversely, Backtest Mode has no multicast endpoints at all, so putting them in the run config would leave a mandatory field permanently empty.

**Rule: anything that affects results goes in run config and is hashed. Anything that affects only where the process connects goes in deployment config and is excluded from run identity.**

**Deployment configuration contains:** per-channel multicast group address, port, interface and source (for source-specific multicast); A/B service pairs; incremental and snapshot channel addressing separately (D-§4 confirms these are different address/port combinations); recovery and retransmission endpoints; ETI session credentials and endpoints; Quincy endpoint details once known.

**Environment-aware.** MCX operates a member simulation environment for certification with its own endpoints, so deployment config selects among production, UAT and simulation rather than being edited between them.

**Validated at startup** — can the group be joined, is the interface up, are credentials accepted — so a misconfiguration fails immediately and loudly rather than presenting as a silent absence of market data.

**Not hot-reloadable.** Endpoints change between sessions, not during one. A restart is the correct mechanism and avoids an entire class of mid-session reconfiguration bugs.

---

### D26 — Reporting: two tiers plus strategy-published series
**Decision:** A run emits three things, all through an **observer registered on the Control Dispatcher** (D07) — nothing publishes *to* reporting, so it can be added, changed or removed by wiring alone.

**Tier 1 — compact structured per-run summary. Always emitted.**
Net and gross P&L after costs at both levels (D08), inventory over time, markout distribution, OTR consumed against budget, message counts, invariant violation counts. Columnar and machine-readable so hundreds of sweep runs aggregate trivially.

**Tier 2 — full per-event detail. Switchable per run.**
Every order command and response with **rejection reasons distinguished** — own limit, firm limit, exchange rejection (D08 requires strategies to tell these apart, so reporting must too). Per-fill records carrying price, quantity, side, timestamp, **queue position at fill**, **mid-price at fixed horizons afterwards**, a **spread-improving flag**, and realised cost from D23.

*Why two tiers:* summary-only leaves results unexplainable; detail-only makes a sweep produce gigabytes that cannot be summarised. Sweeps run lean, investigations run verbose.

**Tier 3 — strategy-published time series.**
Strategies can publish named scalar series at the current clock time — fair value, skew applied, quote width, basis, signal inputs, any intermediate model output. **Fills explain outcomes; these explain reasoning.**

Design constraints:
- **Pre-registered series handles** obtained at `on_start`, so there is no string lookup on the hot path
- **Write-only and non-behavioural** — publishing must never influence strategy decisions
- **Clock-timestamped**, so series align exactly with fills and events on one timeline
- **Cheap when the tier is disabled**, and never blocking in Live Mode
- **Identical API in both modes** — the same calls feed backtest analysis and live monitoring, differing only in sink

**Per D22:** every output embeds the full run specification and both hashes, so any result is self-describing.

**Non-negotiable from day one:** queue position and markout horizons on fill records. Retrofitting them means re-running everything already trusted.

---

### D25 — Depth-scoped subscription and dispatch
**Decision:** Subscriptions declare a **depth of interest**. The Event Dispatcher wakes a strategy only when the book changes *within that depth*. Subscriber lists are keyed by `(instrument, depth)`.

**The full book is still maintained** (D06 unchanged) — subscription governs **waking, not access**. A strategy can reach any depth on demand through the Cache.

**Why not fire on every change:** order-by-order data means a callback for an order added at the eighth price level, which no market maker cares about. The strategy's first line becomes "has the BBO changed? no? return" — the engine woke it to be told to go back to sleep, on the one Core thread.

**Why not coalesce over time:** batching would be cheapest, and parity would hold since both modes would coalesce identically, but **it directly costs reaction time**. Coalescing to save CPU while paying for Quincy microwave data works against the whole point.

**Also matches the data:** DGCX provides one level, CME five — depth-scoped subscription is already the natural shape at the signal venues.

---

### D24 — Strategies are written by engineers; the trait stays thin
**Decision:** Engineering writes strategies. The `Strategy` trait is thin and direct:

`on_start` · `on_book` · `on_trade` · `on_fill` · `on_order_update` · `on_timer` · `on_session_change` · `on_book_state_change` · `on_warmup_complete` · `on_stop`

**Dispatch is `dyn Strategy`** — a vtable call costs a couple of nanoseconds, and runtime-loadable strategy sets are what D22's config-driven runs require.

**Context handle provides:** clock reads, Cache access (books, sub-account and firm positions, session and book state), cost queries (D23), order submit/cancel/modify, dynamic subscribe/unsubscribe (D15), timers and alarms, instrument reference queries, seeded RNG and deterministic logging. **Strategies never reach ambient state** — no system clock, no unseeded randomness, no direct I/O.

**Rejected alternative:** had a separate research group without Rust been writing strategies, the requote loop, OTR budget, STP and order lifecycle would have moved into engineered Rust with a declarative plug-in point for pricing and skew. That is a materially larger build and is not needed.

---

### D23 — Transaction costs: a `CostModel`, queryable pre-trade
**Decision:** A `CostModel` component with config-driven rates, applied **identically in both modes**, **queryable by strategies at decision time** and applied to fills for net P&L. It is **not** in the Simulated Exchange.

**Why it matters more here than usual:** for a market maker earning a spread measured in ticks and paying costs on every round trip, transaction costs are frequently the difference between a profitable and unprofitable strategy.

**Cost stack (MCX, rates are circular-driven config):** exchange transaction charges, SEBI turnover fees, CTT, GST on brokerage and exchange charges, stamp duty, clearing/brokerage.

**Direction-asymmetric by construction:** **CTT falls on the sell side, stamp duty on the buy side.** Cost cannot be modelled as a flat per-lot figure. Several components are turnover-percentage based, so they scale with price.

**Why not in the Simulated Exchange, despite the intuitive fit:** the real exchange does **not** report CTT or stamp duty in an execution report — those arrive later in contract notes and settlement. A simulator producing them would generate information the live gateway never generates, and the ExecutionEngine would consume a field present in one mode and absent in the other.

**Why queryable pre-trade, not reporting-only:** a market maker must price costs into its quotes. Deciding whether a one-tick spread is worth quoting requires round-trip cost *before* sending. One model serves both the quoting assumption and the realised accounting, so they cannot disagree.

**Fee arrangement: none.** Confirmed — no market-maker concession applies. **Standard retail rates on both legs of every round trip.**

**Consequence, and it constrains strategy design rather than the engine:** with no fee concession, the round-trip cost is a hard floor on the spread you can profitably quote. If one tick does not cover the round trip on MCX Crude, one-tick markets are loss-making by construction and the strategy must quote wider — which changes fill rate, inventory turnover and the entire risk profile. **This makes D23's pre-trade cost queryability essential rather than convenient**, and it is the first thing a phase-1 backtest should quantify, before any alpha work.

---

### D22 — Run specification is a declarative file
**Decision:** A run is defined by a **declarative config file**, hashed. **Run identity is `(config hash, build/commit hash)`.** The full specification is embedded in every result artifact so results are self-describing.

**Everything that influences output and is not already inside the recording gets pinned:**
- Data — which recordings, date range, and content hashes
- **The full strategy set** and every parameter (per D08, a strategy's behaviour depends on what ran alongside it — one-strategy and three-strategy runs are not comparable results)
- Latency model type, parameters and **seed** (D18)
- Simulated Exchange configuration — OTR enforcement and governor switches, validation switches (D17, D19)
- Warmup window and bootstrap mode (D14)
- Calendar version and instrument master version (D15, D16)

**Why the build hash matters:** a config hash captures no strategy code. Two runs with identical configuration on different commits will differ, and config-only identity leads to concluding a parameter mattered when it was a code change.

**Why declarative rather than programmatic:** hashable, diffable, version-controllable and **machine-generatable** — parameter sweeps across many processes are the entire parallelism model (D04).

---

### D21 — Market-making mechanics that must be modelled
Agreed as structural, not optional features:

- **Modify vs cancel-replace priority.** Quantity reduction retains priority; price change or quantity increase loses it. For a market maker this is most of the P&L. On MCX this is **published explicitly** (see §4); on NSE it must be inferred from matching rules.
- **Self-trade prevention** across strategies, at the firm level, before an order leaves.
- **Market Maker Protection** — MCX can cancel your orders (reason `3`). If the simulator does not model it, backtests keep quoting through exactly the adverse runs where production would have pulled you out.
- **Markout as a first-class output** — mid-price at fixed horizons after every fill, plus queue position at fill, plus a flag on every fill that came from a spread-improving quote. A market-making backtest showing consistently *positive* markout indicates a simulator bug or an unmodelled counterfactual, not alpha.

**On adverse selection:** price-time priority alone does **not** address it. Price-time priority answers *did I get filled* — the largest source of fake fills, and worth modelling correctly. Adverse selection is about *what the fill was worth*. Historical replay captures much of it for free because the aggressor and the subsequent price move are real. It leaks in three places: your order was never in the real feed so nobody reacted to it; improving the price makes all downstream flow counterfactual; and you would have absorbed flow that historically went elsewhere. **The estimate degrades as size grows and degrades faster when quoting inside the spread.**

---

## 4. Verified protocol facts

### MCX T7 EOBI — market data
Source: circular **MCX/CTCL/502/2023**, *MCX_EOBI_API v1.2*, 5 July 2023.

| Property | Value |
|---|---|
| Timestamps | nanoseconds past **Unix epoch, UTC** |
| Price | integer including **8 decimals** |
| Instrument id | `SecurityID`, 8-byte signed int |
| Message id | numeric `templateID`, also fixes message size |
| Transport | UDP multicast, A/B ("Live-Live") services |
| Recovery | **separate multicast snapshot channel** |

**Template IDs:**

| Message | ID | Message | ID |
|---|---|---|---|
| Order Add | 13100 | Execution Summary | 13202 |
| Order Modify | 13101 | Top Of Book | 13504 |
| Order Modify Same Priority | **13106** | Product State Change | 13300 |
| Order Delete | 13102 | Instrument State Change | 13301 |
| Order Mass Delete | 13103 | Product Summary | 13600 |
| **Full Order Execution** | **13104** | Instrument Summary | 13601 |
| **Partial Order Execution** | **13105** | Snapshot Order | 13602 |
| Heartbeat | 13001 | Instrument Info / Index Info | 13603 / 13604 |

**Channel architecture (resolves most of O8):**
- **Two channel types on different multicast address/port combinations** — incremental and snapshot. Both differ from the netted market-data broadcast channel.
- **"Live - Live" multicast** — the A/B redundancy assumed in D05.
- **Packet sequence numbers increment per channel only.** `MarketSegmentID` appears in the packet header only.
- **Fixed-length message layouts, no compression**, message padding for byte alignment, little-endian.
- Push-based publishing, out-of-band distribution mode.

**Snapshot bootstrap procedure — explicitly specified (refines D14):**
1. Subscribe to the snapshot channel; messages are grouped **by product**.
2. **Keep processing the incremental channel concurrently** while the snapshot arrives.
3. Snapshot messages carry **`LastMsgSeqNumProcessed`** as the synchronisation watermark.
4. After the full snapshot is processed, apply any incremental messages whose sequence number exceeds that watermark.

A snapshot cycle contains Product State, Instrument State, **Trade Statistics per instrument**, and all visible orders — sequenced by product id, then instrument id, then price level, best to worst.

**`Top Of Book (13504)` has restricted dissemination.** The spec states it is published "starting from **post trading state until end of day trading state** to provide the BBO instrument's information." **It is not disseminated during continuous trading**, so it cannot serve as continuous book validation — only as an end-of-session BBO checkpoint. Its `NumberOfBuyOrders` / `NumberOfSellOrders` fields are marked *not used*.

**Continuous book validation therefore comes from the snapshot channel**, which publishes all visible orders cycle by cycle throughout the session — full-depth rather than BBO-only, and already required for bootstrap (D14). A book can carry a correct BBO while being wrong at depth, which a BBO comparison would never catch.

**Two operational consequences:**
- **Trade statistics are not on the incremental channel at all.** They are omitted deliberately to keep messages small, and appear only on the snapshot channel for recovery. **The engine must derive them from order execution messages** (`13104`/`13105`) rather than expecting them live. Any strategy wanting day volume, VWAP or OHLC gets them from our own accumulation, not from the feed.
- **On transition into Continuous Trading, all visible orders are immediately republished** on the incremental channel — relevant to both session handling (D16) and bootstrap.

**Two findings that shaped the design:**

1. **No broadcast order ID.** Order Delete publishes only `SecurityID`, `Side`, `Price`, `TransactTime` (§3.2.4). Orders are identified by a composite key including priority timestamp. NSE by contrast gives an explicit day-unique Order ID (as an IEEE754 `DOUBLE`). **The internal event vocabulary needs an abstract order handle, not an `order_id` field.**

2. **Priority semantics are published.** §3.2.3: modification to another price or a quantity *increase* changes time priority → `Order Modify (13101)`. No priority loss (e.g. quantity reduction) → `Order Modify Same Priority (13106)`. **The MCX decoder reads this; the NSE decoder must infer it.** The internal event carries a `priority_retained` flag either way — a textbook justification for per-venue normalisation.

3. **Queue consumption is directly observable.** `Partial Order Execution (13105)` and `Full Order Execution (13104)` report per-order fills alongside the aggregate `Execution Summary (13202)`. *(This corrects an earlier assumption that consumption would have to be inferred.)*

### MCX T7 ETI — order entry
Source: *Trading Binary Interface — MCX ETI API v1.4.2*, circular 536/2024.

**`OrdType` (40):** `2` Limit · `3` Stop Market · `4` Stop Limit · **`5` Market To Limit** · `6` Auction Buy In · `7` Auction Sell Out.
**There is no plain market order.** Modification rules confirm: a Limit order "may only be modified to a Market Order (OrdType = 5)."

**`TimeInForce` (59):** `0` Day · `1` GTC *(Standard only)* · `3` IOC · `6` GTD *(Standard only)* · `7` Session (EOS).

**`ExecInst` (18):** bit-encoded. `1` Persistent (H) · `2` Non-Persistent (Q). Persistent/non-persistent can be modified in both directions.

**Book-or-Cancel exists** — execution report reason `212 "Book or Cancel Order accepted"`, in both order-add and order-modify responses.

**Market Maker Protection exists** — unsolicited cancellation reason `3 "Market Maker protection"`, alongside `7` Duplicate Session Login, `105` Product State Halt, `106` Product State Holiday, `110` Volatility Interruption, `111` Product temporarily not tradeable.

**Standard vs Lean orders:** Standard orders are persistent, fully broadcast, recoverable via retransmission. **Lean orders are always non-persistent**, visible only to the submitting session, with execution notifications recoverable only on that session's data channel, and are automatically cancelled at end of business day or after a market reset.

**No mass-quote or quote-entry interface exists in the specification.**

### NSE MTBT 7.0 — deferred phase, recorded for later
Source: *MTBT API Specification v7.0*, Aug 2026. One unified document covers CM, FO, CD and CO.

Message types: `N`/`M`/`X` order add/modify/cancel · `T` trade · `G`/`H`/`J` spread order add/modify/cancel · `K` spread trade · `C` trade cancel · `Z` heartbeat · `R`/`Y`, `O`/`B` recovery control.

Key properties: little-endian, `#pragma pack(1)` · **Order ID is a `DOUBLE`** (convert to `u64` at decode; never compare or hash as float) · timestamps in **nanoseconds from 01-Jan-1980** · price ÷10² for CM/FO · sequence resets to 1 on DR switchover · two multicast sources active-active, one lagging · TCP recovery capped at 300k messages per request.

Behavioural rules from the spec: **the book is legitimately crossed at times** (aggressive orders publish as `N` before the resulting `T`) · **stop-loss orders are never disseminated**, so a cancel for an unknown order ID must be *ignored* and a modify for an unknown order ID treated as a *new order* · market orders are not published, so trades may reference unseen order IDs · modify and cancel carry no old price or quantity.

### Quincy QED — specification not yet obtained
Integrated, normalised, low-latency feed carrying **select** data from multiple exchanges over microwave. One protocol, one adapter, two venues (CME + DGCX/GME).

**Known:** CME depth 5 levels; DGCX depth 1 (BBO).
**Unknown, needed:** wire format; whether it provides a snapshot or book-refresh mechanism; whether it passes venue session state through.

> **Note:** DGCX has rebranded to **Gulf Mercantile Exchange (GME)**. Documentation appears under both names.

---

## 5. Event flow

### The Core loop
One thread, one loop. Everything else is a consequence of what is in the queue.

```
loop {
    event = scheduler.pop_earliest()      // priority queue
    if event.is_none() { break }
    clock.set(event.timestamp)            // time only moves here
    dispatch(event)                       // handler may enqueue more events
}
```

### Event sources
Market data is **one source among several**. All of these enqueue into the same ordered queue:

| Source | Notes |
|---|---|
| Market data | replay iterator or live socket |
| Strategy timers and alarms | `set_timer` / `set_alarm` |
| **Order arrival** | submitted at T, arrives at `T + outbound latency` |
| **Report delivery** | filled at T, learned at `T + inbound latency` |
| Session transitions | pre-open, open, break, close |
| **Staleness and heartbeat timeouts** | fire when *nothing* arrives |

**A clock advanced only by market data cannot detect the absence of market data.** If a feed goes silent for five seconds, a data-driven clock jumps five seconds instantly and never fires the staleness timer. That is why the Scheduler, not the data stream, owns time.

**Ordering key:** `(timestamp, event_class, monotonic_seq)`, sequence assigned at enqueue. Ties are guaranteed at nanosecond resolution; an undefined tie is a non-reproducible backtest.

### Worked trace — one quote, one fill
Modelled outbound latency 250µs.

1. `09:20:00.000000000` — market event pops (`OrderAdd`, MCX Crude). Clock set. Dispatched to Data Engine → BookBuilder → Cache → Event Dispatcher → `Strategy::on_book()`.
2. Strategy calls `ctx.submit_order(...)`. **This does not reach the Simulated Exchange.** ExecutionEngine assigns a client order ID; Order Validation checks tick size and freeze quantity; an `OrderArrival` event is enqueued at `09:20:00.000250000`.
3. `.000000` → `.000250` — the loop keeps popping. Real market events in that window are processed first — **other participants arriving at your price level, ahead of you.**
4. `09:20:00.000250000` — `OrderArrival` pops. The Simulated Exchange inserts the order into **its own** book, at the back of the queue at that price, behind everyone who arrived during those 250µs.
5. Aggressing flow arrives. The simulator determines the fill and enqueues an `ExecutionReport` at `fill_time + inbound latency`. **It does not call back directly.**
6. The report pops → ExecutionEngine advances the order state machine → updates sub-account and firm aggregate in the Cache → `Strategy::on_fill()`. **Only now does skew logic see the new inventory.**

**Two properties to notice:** every delay is a timestamp in the queue, never a sleep — nothing blocks. And the strategy's view is always the delayed one, exactly as it will be in production.

**In Backtest Mode time is free.** The clock jumps event to event; a trading day completes in however long the CPU takes. The sub-millisecond target affects only the modelled offsets, never run time.

---

## 6. Functional requirements captured

### Heartbeat and liveness

**FR-01** — On heartbeat receipt, compare its last-sequence-number field against the last sequence received on that stream. Any discrepancy triggers gap recovery and marks affected books `STALE`.

**FR-02** — Absence of **both** data and heartbeat on a subscribed stream beyond a configurable threshold raises a **critical** alert and marks affected books `STALE`. *(This is the connection-dead case — the genuinely dangerous one.)*

**FR-03** — Heartbeat received on a stream **expected to be active** for the current session phase raises a **warning**, configurable per stream. *(Depends on the calendar, per D16.)*

### Recording and capture

**FR-04** — Every packet carries a capture timestamp from a single host clock, stamped on receipt.

**FR-05** — Recording occurs at the point of consumption. *(Capture-where-you-consume, per D05.)*

**FR-06** — Recordings preserve product and instrument state messages. *(Per D16.)*

**FR-07** — Live writes a post-merge journal in Core consumption order, reserved for parity verification.

### Validation and control

**FR-08** — Order Validation enforces tick size and freeze quantity, independently on both engine and Simulated Exchange sides.

**FR-09** — OTR is counted independently on both sides, each with its own binary enable switch.

**FR-10** — `cancel_all_for_strategy` exists, is reliable in both modes, and is honoured identically by the Simulated Exchange.

### Testing invariants

**FR-11** — Book state after snapshot-start at T then replay to T′ equals book state after full replay to T′. **Parameterised over many values of T**, deliberately including points inside a snapshot cycle rather than only at cycle boundaries, since the synchronisation window is where bootstrap bugs live: off-by-one on the `LastMsgSeqNumProcessed` watermark, an incremental applied twice, or one dropped.

**FR-13** — **Bootstrap during an auction must not produce a book.** The EOBI spec states that during auctions, snapshot messages carry Auction Best Bid-Offer or Auction Clearing Price messages **instead of order messages — visible orders are not published via snapshot during an auction.** A process starting mid-auction therefore *cannot* build a book from the snapshot channel. The book must remain `UNINIT` until Continuous Trading resumes, at which point the spec guarantees all visible orders are immediately republished on the incremental channel. Test: start during an auction, assert `UNINIT`, assert no quoting occurs, then assert convergence to full-replay state once continuous trading begins.

**FR-14** — **Mid-session gap recovery converges.** Distinct from cold start: the process already holds a book, detects a sequence gap, and must resynchronise. Book state after gap-and-recovery at T must equal book state after an uninterrupted replay to the same point.

**FR-15** — **A book that is `UNINIT` or `RECOVERING` prevents quoting.** Enforced via D28's watchdog, since the trading venue's own book is a declared dependency. A strategy must never quote into a market whose state it has not yet established.

**FR-12** — A recorded live session replayed through the Core produces an identical decision stream. Any divergence is a defect.

---

## 7. Open questions

| # | Question | Blocking |
|---|---|---|
> **Detail lives in [OPEN-QUESTIONS.md](OPEN-QUESTIONS.md)** — including ready-to-send question sets for Quincy and MCX. Summary only here.

| # | Question | Status |
|---|---|---|
| **O5** | Quincy QED wire format | **Open** — blocks the second decoder |
| **O6** | Whether QED provides snapshot / book-refresh | **Open** — blocks bootstrap for signal venues |
| **O7** | Whether QED passes venue session state through | **Open** — determines D16 fallback for CME/DGCX |
| ~~O9~~ | Simulated Exchange validation approach | Closed → **D31** (four layers) |
| ~~O1~~ | Who writes strategies | Closed → **D24** (engineering) |
| ~~O2~~ | Run configuration | Closed → **D22** (declarative file) |
| ~~O3~~ | Reporting outputs | Closed → **D26** (two tiers + strategy series) |
| ~~O4~~ | Journal format | Closed — raw post-merge stream plus commands and reports, in consumption order, same framing as the transport. Retention is operational policy |
| ~~O8~~ | MCX connectivity | Closed → protocol architecture in §4; endpoints are deployment configuration per **D27** |
| ~~O10~~ | Market-maker fee scheme | Closed → **none.** Standard retail rates. See **D23** |

**Two items also carry residual actions that are not design questions:** the phase-1 premise depends on **Crude and Natural Gas being covered by T7 EOBI** (Q-MCX-1), and **ETI message-rate limits** must be obtained so the Simulated Exchange can enforce them (Q-MCX-6). Both are in the MCX question set.

---

## 8. Explicitly out of scope

| Item | Status |
|---|---|
| NSE CM and F&O | Deferred to a later phase |
| NSE Currency Derivatives, NSE Commodity | Out of scope entirely |
| Options market making | Deferred; architecture accepts pricing and volatility **plug-in modules** later |
| Stop Market, Stop Limit, auction order types | Out of scope |
| Iceberg orders | Out of scope; changes queue mechanics enough to need deliberate modelling |
| Mass quote / quote interface | Not offered by MCX ETI; extension points preserved (D11) |
| Generic risk limit framework | Deferred (D17) |
| NUMA placement, CPU-core pinning | Deferred to the kernel-bypass phase (D02) |
| Kernel bypass | Committed later phase |

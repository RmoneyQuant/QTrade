# Open Questions

**Companion to [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md).**
**Last updated:** 2026-08-19

Everything answerable by internal discussion has been settled — 30 decisions. What remains falls into three categories:

1. **Open design decisions** — awaiting internal confirmation (§1)
2. **Open facts** — need documents or answers from counterparties (§2, §3). *These sections are written to be sent directly to Quincy and MCX.*
3. **Deliberately deferred** — decided *not* to decide yet, with the trigger for revisiting (§5)

**None of the open items invalidate a settled decision.** They fill in adapter detail, test infrastructure and deployment values.

---

## 1. Open design decisions

**None.** All design decisions are settled — 31 recorded in [ARCHITECTURE-DECISIONS.md](ARCHITECTURE-DECISIONS.md).

The section below is retained because it documents the validation approach in more operational detail than the decision log carries.

### ~~O9~~ — Simulated Exchange validation approach → **RESOLVED, see D31**

**Status:** Agreed. All four layers, Layers 1 and 2 before the first strategy.

**Why it matters more than a normal testing question.** The Simulated Exchange is the component that can lie *silently*. A wrong book crashes or visibly diverges; a wrong fill model produces plausible numbers you will trust for months and then bet money on. There is no natural ground truth, because no recording exists of what *would* have happened had your orders been present.

**The proposal is four layers, each answering a different question.**

#### Layer 1 — Book correctness (has real ground truth)

**Primary: the snapshot channel.** Snapshot cycles publish **all visible orders** in the book, product by product, continuously throughout the session. Compare your incrementally-built book against each arriving cycle. This is **full-depth** validation, not BBO-only — a book can carry a correct best bid and offer while being wrong at depth three, which a BBO check would never catch.

The snapshot channel is already required for bootstrap (D14), so this validation adds comparison logic rather than new infrastructure.

**Secondary: `Top Of Book (13504)` as an end-of-session checkpoint.** *Note the constraint* — the spec states Top Of Book is published "starting from **post trading state until end of day trading state**." **It is not disseminated during continuous trading**, so it cannot provide continuous validation. At post-trading, compare your final BBO against MCX's published value: a cheap daily integrity assertion that tells you *whether* you diverged, though not *when*.

- **Backtest Mode:** standing assertions — any divergence fails the run.
- **Live Mode:** continuous metrics — divergence raises an alert.
- **Cost:** low. Near-free correctness on the most fundamental component.
- **No external dependency** — the snapshot channel is already in the design.

#### Layer 2 — Automatic simulator invariants (no extra data required)

Checked on every backtest run. The strongest:

> **Simulated fills at a given price and time must not exceed the volume that actually traded at that price in the recording.**

If the strategy fills 100 lots where 20 traded, the simulator has fabricated liquidity. This runs against any historical recording with no additional inputs.

Supporting invariants:
- A **Book-or-Cancel** order that would cross must be **rejected, never filled** (D12)
- Fill price must be **at or better than** the limit price
- **Queue position never improves** except through consumption ahead of you
- **Market-to-Limit** residual must become a resting limit at the traded price, not vanish (D12)
- Simulated **OTR** and **message-rate** counters must never exceed configured venue limits (D19)

#### Layer 3 — Hand-traceable scenarios

Synthetic order flow small enough to verify with pen and paper, kept as regression tests. This is where phase 1's choice of a **scalar-inventory futures contract** pays off — one quote, one queue position, one fill, arithmetic you can check yourself. Would not be feasible against an option chain.

#### Layer 4 — Shadow comparison (once live)

Run the Simulated Exchange **in parallel during live trading**, fed the same market data and the same orders you actually sent. Compare simulated fills against real fills.

This is the only layer that measures the simulator against **reality** rather than against your own assumptions, and it is the venue-level counterpart to **FR-12**. Divergence is a simulator defect, quantified.

**Recommendation:** all four, with **Layers 1 and 2 built before the first strategy**. A simulator that fabricates liquidity is worse than having no backtest, because the results carry false confidence.

**What you said you needed to confirm is unknown to me** — if it is whether `13504` is available on your products, that is Q-MCX-2 below and I can work it into Layer 1 either way.

---

## 2. Questions for Quincy

> *This section can be sent as-is.*

**Context:** we consume QED for CME (5-level depth) and DGCX/GME (BBO) as pricing signals for market making on MCX. We need to write a decoder and model the feed's behaviour accurately in a backtesting environment.

### O5 — Wire format and protocol

**Blocks:** writing the QED Feed Adapter — the second of our two phase-1 decoders.

| # | Question | Why we need it |
|---|---|---|
| Q-QED-1 | Full wire format specification — message layouts, field offsets, encoding (proprietary binary / SBE / other), endianness | Cannot write a decoder without it |
| Q-QED-2 | Transport — UDP multicast, TCP, or both? Is there an A/B redundant service? | Determines Transport implementation and whether arbitration is needed |
| Q-QED-3 | Sequence numbering — per channel, per venue, per instrument? | Gap detection, and our deterministic merge ordering key |
| Q-QED-4 | Gap detection and recovery — retransmission, or is loss unrecoverable? | Recovery path design; what `STALE` means for a signal venue |
| Q-QED-5 | Symbology — how are CME and DGCX instruments identified? Is there a mapping to native exchange instrument IDs? | Instrument identity and reference data (D15) |
| Q-QED-6 | Timestamp semantics — which clock, which epoch? Is it exchange time, Quincy receipt time, or both? | Normalisation, and our capture-timestamp merge (D05, D30) |
| Q-QED-7 | Price and quantity representation — integer scaling factor, or decimal? | Normalisation to internal `i64` ticks |
| Q-QED-8 | Update semantics — is each depth update a full level replace, or a delta? | Book maintenance logic for `MbpBook` |
| Q-QED-9 | Heartbeat or liveness mechanism, and its interval | Our staleness detection requires it (FR-01, FR-02, D28) |
| **Q-QED-10** | **Is the data conflated or throttled?** If so, at what rate or under what conditions? | **Critical.** Microwave bandwidth is constrained, so conflation is plausible. If CME updates are conflated, we do not see every change and must model that blindness rather than assume a continuous view |

### O6 — Snapshot and book refresh

**Blocks:** bootstrap for signal venues (D14), and recovery behaviour after a gap.

| # | Question | Why we need it |
|---|---|---|
| Q-QED-11 | Is there a snapshot channel or periodic full book refresh? | Determines whether a mid-session start can build a CME book at all |
| Q-QED-12 | If not, how does a late-joining consumer construct a book? | We restart processes mid-session; this is unavoidable in live |
| Q-QED-13 | After detected loss, how does a consumer resynchronise? | Recovery path |
| Q-QED-14 | Is there an explicit "book invalid / cleared" indication? | Whether we can distinguish a stale book from a valid quiet one |

**If the answer is "no snapshot":** a restarted or gap-recovered process has an **unusable CME book for an indeterminate period** — potentially long on quiet instruments, since a 5-level book only fills in as updates arrive. The strategy would have to treat CME as `STALE` until some heuristic confidence threshold, which directly complicates D28's watchdog tuning.

### O7 — Venue session state

**Blocks:** D16 for CME and DGCX.

| # | Question | Why we need it |
|---|---|---|
| Q-QED-15 | Does QED carry exchange session or state messages — open, closed, halt, auction, maintenance break? | D16 sources session state from the venue where published |
| Q-QED-16 | Are trading halts and auction states indicated? | A halted market is not a quiet market |
| Q-QED-17 | If no state is carried, is there any liveness indicator distinguishing "market closed" from "feed down"? | This is the distinction our critical alert exists to make (FR-02) |

**If the answer is "no state carried":** D16's fallback applies — calendar plus data-presence inference, **in both modes**. The calendar then becomes load-bearing for CME and DGCX, and D28's watchdog becomes harder to tune, since distinguishing CME's scheduled maintenance break from a microwave link failure would rest entirely on the calendar being correct.

---

## 3. Questions for MCX

> *This section can be sent as-is.*

| # | Question | Why we need it | Relates to |
|---|---|---|---|
| **Q-MCX-1** | Are **Crude Oil and Natural Gas futures** within the "selected group of derivatives market benchmark products" covered by T7 EOBI? | **This is the load-bearing assumption of phase 1.** Our entire design assumes MBO data on the instruments we quote. Without EOBI on these contracts, the queue-position modelling that justifies the architecture does not apply | Whole design |
| Q-MCX-2 | Confirm **snapshot channel cycle cadence** for these products — how frequently does a full cycle complete? | Determines how often we can validate our book against ground truth, and how long a gap-recovering process waits for a usable book | O9 Layer 1, D14 |
| Q-MCX-3 | Multicast group addresses and ports for EOBI **incremental** and **snapshot** channels, both A and B services, for production, UAT and simulation environments | Deployment configuration values | D27 |
| Q-MCX-4 | Is **colocation** available, and what are its characteristics — rack space, cross-connect, latency to matching engine? | Determines achievable latency and whether the D02 bypass phase is worthwhile | D02 |
| Q-MCX-5 | Is a **PTP time source** available in colocation? | NTP accuracy is coarser than our latency budget, which would make latency calibration mostly clock error | D30, D18 |
| **Q-MCX-6** | **ETI session message-rate limits and order-rate throttles** — what are the ceilings, and how are breaches handled? | A market maker requoting continuously will approach these. If the simulator does not enforce them, the backtest runs a strategy that physically cannot exist | D19 |
| Q-MCX-7 | Current **OTR (order-to-trade ratio)** framework, thresholds and penalties for energy futures | We track OTR on both engine and simulator sides; thresholds move by circular | D19 |
| Q-MCX-8 | Is **Book-or-Cancel** enabled for these products? (ETI reason code `212` exists — confirming it is available to us) | BOC is central to safe quoting | D12 |
| Q-MCX-9 | **Market Maker Protection** — is it available to us, what parameters are configurable, and what triggers it? (Cancellation reason code `3` exists) | MMP cancellations must be modelled or backtests quote through adverse runs that production would have exited | D21 |
| Q-MCX-10 | Confirmation that **Lean orders** are available to us, and any conditions attached | Our quoting order category | D13 |
| Q-MCX-11 | **Freeze quantity** and **tick size** per contract, and where these are published for programmatic ingestion | Phase-1 Order Validation checks | D17 |
| Q-MCX-12 | **Certification / simulation environment** access and the conformance test requirements | Gates go-live; typically long lead time | D27 |
| Q-MCX-13 | Current **transaction charge schedule** and confirmation that no market-maker concession applies | Cost model rates. Round-trip cost is a hard floor on quotable spread | D23 |

---

## 4. Internal items to determine

| # | Item | Why | Relates to |
|---|---|---|---|
| I-1 | Confirm **NIC hardware timestamping** support on the chosen network cards | Capture timestamps excluding OS jitter | D30, FR-04 |
| I-2 | Decide **journal retention policy** — how long raw captures are kept, and storage sizing | Operational; the format is settled | D05 |
| I-3 | Measure the **minimum quotable spread** from the cost model against historical MCX Crude spread distribution | **Should be the first phase-1 calculation.** Determines whether one-tick market making is viable at all before any alpha work | D23 |
| I-4 | Confirm **which contracts** phase 1 targets — Crude Oil vs Crude Oil Mini, Natural Gas vs Natural Gas Mini | Liquidity, tick size and freeze quantity all differ | D15 |

---

## 5. Deliberately deferred

**These are decisions not to decide yet, recorded so they are not mistaken for oversights.**

| Item | Why deferred | Trigger to revisit |
|---|---|---|
| **Generic risk limit framework** (D17) | Speculative before a working simulator. Order Validation covers phase-1 needs | **Before live trading.** Going live with market making and no max-net-position and no daily-loss halt is materially different from deferring them in backtest |
| **Margin modelling** | Belongs inside the risk framework. D29 already requires daily settlement cash flows | With the risk framework |
| **Recovery-failure policy** — what happens when snapshot recovery itself fails to complete | Better specified against observed MCX recovery behaviour than guessed | On first contact with the MCX simulation environment |
| **NUMA placement and CPU-core pinning** (D02) | Live-path optimisation that would distort the backtester | With the kernel-bypass phase |
| **Kernel bypass** (D02) | Correctness first | Committed later phase |
| **Options pricing and volatility modules** | Phase 2. Architecture accepts them as plug-ins | When options market making begins |
| **Quote interface / mass quote** (D11) | Not offered by MCX ETI. Extension points preserved | If MCX introduces it |
| **NSE CM and F&O** | Later phase. Protocol facts already recorded | Phase 2+ |
| **Iceberg orders** | Changes queue mechanics enough to need deliberate modelling | If showing less than full size becomes desirable |

---

## 6. Priority

**Blocking phase-1 implementation:**
- **Q-MCX-1** — if Crude and Natural Gas are not on EOBI, the phase-1 premise needs rethinking
- **O5 / Q-QED-1** — no decoder can be written without the wire format

**Blocking phase-1 correctness:**
- **O9** — validation approach
- **Q-MCX-2** — snapshot cycle cadence, which sets both validation frequency and gap-recovery time
- **Q-MCX-6, Q-MCX-7** — rate limits and OTR the simulator must enforce

**Blocking go-live, not development:**
- Q-MCX-3, Q-MCX-4, Q-MCX-5, Q-MCX-12 — deployment values, colocation, time source, certification
- The deferred risk framework (§5)

**Informational but decision-shaping:**
- **I-3** — minimum quotable spread. Cheap to compute, and it either validates the phase-1 premise or redirects it

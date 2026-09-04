# Simulator — component documentation

**What this component does, in one sentence:** stands in for MCX itself
during a backtest — its own independent order books, its own queue-position
tracking, its own fill logic for every in-scope order type — built directly
from `decoder`'s message stream, with zero read access to `cache` or `book`.

Code: [`simulator.rs`](simulator.rs) (this folder). Validation harness (not
part of the public API): [`validate.rs`](validate.rs), built as the
`simulator-validate` binary — see §6.

---

## 1. Why independence from `cache` is not a formality

D10 states it as an architectural decision; this section is about why it
matters *concretely*, not just "because the doc says so."

**A real exchange cannot become confused because your receiver dropped a
packet.** In production, `cache`'s book is a *view* of the market, assembled
from a live feed. If a network hiccup causes a gap, `cache` marks that
book `STALE` (FR-B10) — a statement about *our own uncertainty*, not about
the market. The real MCX matching engine has no idea our feed handler
glitched, and it keeps matching orders exactly as if nothing had happened.

If `simulator` read `cache`'s book to decide fills, two failure modes
follow, both silent:

1. **Corruption inheritance.** A gap that made `cache`'s book `STALE` for
   thirty seconds would make `simulator`'s fills wrong for exactly that
   window too — but a backtest has no live Transport, so this specific
   failure mode can't even occur *unless* someone builds it in by wiring
   the two together. Keeping them apart means it structurally can't happen,
   not "we were careful."
2. **Coupling that doesn't exist live.** In production, the venue is a
   separate company's matching engine on a separate machine. Nothing in
   `qtrade`'s process can reach into it. If `simulator` depended on `cache`
   in backtest, the two modes (`live` vs `backtest`) would have a different
   *shape* of dependency graph — exactly the kind of asymmetry ARCHITECTURE-
   DECISIONS.md's D16/D20 spend real effort avoiding elsewhere, reintroduced
   here for convenience.

**What independence costs, concretely, in this codebase:** `simulator`
re-derives *the same* MBO book from *the same* `decoder::decode_messages`
stream `book` (T03) consumes, but as a **separate Rust struct**
(`SimBookImpl`, a `BTreeMap`-per-side design — see §2), with its own
`apply_real_event`, its own trade-matching, its own modify-priority logic,
independently re-checked against `references/MCX_Feeder.cpp` (§3). It does
not call anything in `crate::book` or `crate::cache`, and this document was
written without reading `cache`'s source at all — only `book.rs` and
`book_user_doc.md`, which the task brief explicitly authorizes reading for
the shared, already-hard-won business-rule findings (see §3), while still
requiring an independent implementation (D10).

**What this buys, beyond principle:** `simulator` becomes unit-testable
against synthetic order flow with zero dependency on anything else in the
engine (§7's 18 tests construct `DecodedMessage`s by hand, no file I/O, no
`cache`, no `book`), and Layer 4 (shadow comparison, once live — out of
scope this round) becomes meaningful specifically *because* nothing was
shared: a divergence between simulated and real fills is then a genuine
simulator defect, not something `cache`'s own state could have caused.

---

## 2. What's built

- `trait Book` / `trait MboBook` — the same public shape as `book::Book` /
  `book::MboBook` (`best_bid`/`best_ask`/`depth(n)`/`qty_at_price`/`state`,
  plus `queue_position`), for interface consistency across the codebase.
  **Not the same code**: `SimBookImpl` uses a `BTreeMap<i64, Level>` per
  side rather than `book`'s dense pre-sized array — a legitimate, different
  internal design (no price band to size up front, no panic-on-out-of-band
  risk), arrived at independently while building this component, not copied.
- `SimBookImpl` — one per filtered instrument. Each price level's FIFO holds
  **both** real resting orders (identified by wire `priority_ts`, exactly
  `book`'s convention) *and* our own simulated resting orders (identified by
  an internally-issued id drawn from a range — `9 x 10^18` upward — that
  provably can't collide with a real epoch-nanosecond `priority_ts`), sharing
  **one combined queue**, ordered by arrival. This is what makes queue
  position and fills exact: a simulated order inserted at the back of the
  real queue sits in the *same* FIFO a real trade's cascade walks through.
- `LatencyModel` trait (FR-B20) — `fn outbound(&mut self, venue) -> Duration`,
  `fn inbound(&mut self, venue) -> Duration`. Two implementations:
  - `Fixed` — a constant per venue/direction.
  - `Sampled` — seeded (`splitmix64`, no external crate — this project
    declares zero dependencies in `Cargo.toml`, and a tiny seeded generator
    is all determinism requires), draws an `Exp(1)`-shaped sample scaled by
    a configured mean via inverse-CDF (`-mean * ln(U)`). Same seed, same
    call sequence, same latencies — every run.
- `OrderType` (FR-B22) — `LimitDay(Price)`, `BookOrCancel(Price)`,
  `Ioc(Price)`, `MarketToLimit`.
- `SimExchange` — owns one `SimBookImpl` per filtered instrument, all
  resting-order bookkeeping, the OTR/message-rate governor (D19), and an
  `AuditLog` the validation binary asserts FR-B24 against. Interface is
  exactly two directions (FR-B19): `submit`/`cancel`/`modify` (commands in)
  and `apply_market_event` (the independent real feed), both returning
  `Vec<ExecReport>` (reports out) — indistinguishable in shape from what a
  live gateway would hand the `ExecutionEngine`.
- Instrument filter (D32) — `default_filter`, a predicate over native
  `SecurityID`, covering `CRUDEOIL_ID` (467013) and `NATURALGAS_ID` (465849).
  A real strategy-declared predicate would be `underlying == "CRUDEOIL" &&
  front_two_expiries`, resolved once at `on_start` (D15); this milestone
  hand-declares the resolved token set directly, the same scope `book` used
  for the same reason — no strategy layer exists yet to declare one. The two
  token values are real, independently-verifiable facts about the recording
  (native `SecurityID`s), not anything computed by `cache`.

---

## 3. Business rules encoded, and where they came from

Every rule below was checked directly against `references/MCX_Feeder.cpp`
while writing this component — not copied from `book`'s write-up
uninspected (D10's "build as if a separate process" applies to *verification*
too). Line numbers are from the version of the file read during this task.

- **`OrderAdd` (13100)** — new resting order, pushed to the back of its
  price level's FIFO. (`~line 863` on: creates a fresh price-bucket entry,
  `Count += 1`.)
- **`OrderModify` (13101)** — priority **lost**. Confirmed at `~line 527` on:
  the *previous* price bucket's `Count` is decremented
  (`BUY_ORDER_PAISA_QUANTITY_INFO[pDPR_Gap].Count -= 1`) and the *new*
  bucket's `Count` is incremented as a fresh entrant
  (`...[DPR_Gap].Count += 1`) — **even when the price is unchanged**
  (`if(Price==pPrice){...}` still runs the decrement/increment pair before
  that branch). The old `(side, price, priority_ts)` identity is removed
  entirely and a new one is added at the back of whatever level the new
  price maps to.
- **`OrderModifySamePriority` (13106)** — quantity changes in place, `~line
  813` on: only the per-price-bucket `Quantity` is adjusted
  (`-= pQuantity; += Quantity`); no `Count` adjustment anywhere in this
  branch. Same FIFO slot, same identity.
- **`OrderDelete` (13102)** — removes the exact order identified by
  `(side, price, priority_ts)`.
- **`OrderMassDelete` (13103)** — confirmed at `~line 1682`
  (`Purger_Market_Depth`): clears `BuyPrice[]`/`BuyQty[]`/`NoOfBuyOrds[]`
  **and** `SellPrice[]`/`SellQty[]`/`NoOfSellOrds[]` in the same call, plus
  every `BUY_ORDER_PAISA_QUANTITY_INFO`/`SELL_ORDER_PAISA_QUANTITY_INFO`
  bucket's `Quantity`/`Count` — the whole token, both sides, one event.
  Because this really happens at the exchange to *every* resting order for
  the instrument, `simulator` also cancels any of *our own* resting orders
  caught in it (the venue does not distinguish "ours" from "theirs" — out
  of scope items like STP/MMP aside, a mass delete is unconditional).
- **`Trade` (13104 full / 13105 partial) — the matching-key finding.**
  `book` (T03) discovered, the hard way (three attempts, traced through a
  real 103-divergence NATURALGAS bug), that a real `Trade`'s `event_time`
  field (wire offset 24) is **not a wall-clock timestamp** — its value is
  the *specific resting order's own* `priority_ts` that this trade actually
  matched. `simulator` uses the **identical rule**, independently
  implemented in `SimBookImpl::apply_trade`: look the targeted slot up
  directly by `matched_priority_ts` first; if found, cascade *forward* from
  that position (never backward) through the combined real+simulated FIFO
  if the trade quantity exceeds what the target has left; fall back to
  FIFO-front only if the targeted `priority_ts` isn't resting at all (a
  pre-replay-window order, or a genuine race). This is the single most
  consequential rule in this component — see §5's worked trace for what
  happens when a real trade's cascade reaches a simulated order sitting
  right behind the real ones it consumed.
- **`ExecutionSummary` (13202)`, `TopOfBook` (13504)`, `PacketHeader`
  (13003)`, `Heartbeat` (13001)`** — confirmed not book-mutating (`13202`'s
  handling is commented out in the reference code; the others never touch
  `OrderBookPtr`).

**A second, independently-found wire-data landmine** (not documented by
`book`, which never needed a wall-clock reading from this field for
anything besides `Trade`'s already-known special case): the validation
harness's own real-time clock (used to drive the OTR/message-rate window
and the re-quote throttle, §6) initially latched onto `u64::MAX` and froze
there for the rest of a multi-hour run. Root cause, found by direct
inspection (`SIM_DEBUG_TS`-style tracing of raw records): a real capture's
very first `OrderAdd`/`OrderDelete` records for CRUDEOIL — orders that
pre-existed the capture window (the same "multi-day resident order" case
`book_user_doc.md` §5.3 documents for snapshot bootstrap, confirmed here by
their `priority_ts` values coming from materially earlier dates than the
surrounding traffic) — carry `event_time` (`TrdRegTSTimeIn`) set to the
**all-ones sentinel** (`0xFFFFFFFFFFFFFFFF`), not a real timestamp. A naive
`if t > now_ns { now_ns = t }` clock advance latches onto that sentinel on
the very first such record and can never be exceeded again for the rest of
the run, silently freezing every time-driven mechanism downstream. Fixed in
`validate.rs`'s `event_time_of` by filtering the sentinel out before it ever
reaches the clock. This does not affect `simulator.rs` itself (which never
reads wall-clock time — see §4's note on why), only the validation
harness's own bookkeeping; it is recorded here because it is exactly the
kind of "field doesn't mean what the name suggests, in some subset of real
records" trap this whole milestone exists to catch, and it materially
changed the acceptance run's evidence quality once fixed (§7).

---

## 4. Queue position (FR-B21) — a fully worked real trace

**Why MBO makes this exact, not estimated:** because every resting order is
individually identified (`priority_ts`), and every trade names the *exact*
order it matched (§3's finding), `qty_ahead` for a simulated order is not a
statistical guess — it is the literal sum of real, individually-tracked
quantities sitting in front of it, decremented only when those *specific*
orders are consumed or removed.

This trace is real output from `simulator-validate hand-trace` against
`/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin`
(CRUDEOIL, native token 467013), lightly reformatted for readability. **Note
the self-guard the harness applies before trusting a candidate level**: the
first candidate price it found (542400000000 raw, ~₹5424.00) was
*discarded* because a second source of quantity — an `OrderModify` landing
there from a different price — had silently contributed to it; the raw
`OrderAdd` sum (120,000) didn't match the book's own tracked quantity
(70,000) at that price. This is not a hypothetical concern: it is the same
class of trap as `book`'s `Trade.event_time` finding, just for the
hand-trace's own bookkeeping rather than the simulator's — a naive "3 adds,
done" selection would have produced a false hand-computation.

The next candidate, ₹5465.00, passed the guard cleanly:

```
Real sell-side orders at the chosen price 546500000000 (raw units), in arrival order:
  OrderAdd  price=546500000000 priority_ts=1768793406026238339 qty=10000
  OrderAdd  price=546500000000 priority_ts=1768793411020758221 qty=10000
  OrderAdd  price=546500000000 priority_ts=1768793411026023501 qty=10000

Hand computation: qty_ahead = 10000 + 10000 + 10000 = 30000
Cross-check: simulator's own qty_at_price(Sell, 546500000000) *before* our
order arrives = 30000 (matches the hand sum exactly, confirming no other
real event silently contributed to this level).
```

A simulated Sell `LimitDay` order (`client_order_id=777`, qty 10,000 — one
lot at decoder's wire scale) is submitted at that exact price, the instant
the third real order is applied:

```
Simulator's own report on submission:
  RESTING    id=777 side=Sell price=546500000000 qty=10000 priority_ts=9000000000000000000

Simulator's qty_ahead() after insertion: Some(30000)
MATCH: hand computation (30000) == simulator's qty_ahead (Some(30000))
```

**30,000 by hand, 30,000 from the tool — exact.** Streaming further real
events at that price shows the mechanics working correctly over real
market activity, unedited (seq numbers are the wire's own message sequence):

```
seq=10517  ORDER_DELETE  Side=SELL Price=546500000000 Qty=10000  -> qty_ahead now: Some(20000)
seq=10594  ORDER_MODIFY  Side=SELL Prev[546500000000 x 10000] -> New[542500000000 x 10000] [priority LOST] -> qty_ahead now: Some(20000)
  ... (dozens of further ORDER_MODIFY events, orders arriving at and
       leaving 546500000000 from/to other prices -- all of them arriving
       *after* our own order, so none of them are "ahead" of us and
       qty_ahead correctly stays flat at 20000 through every one) ...
seq=208828 ORDER_DELETE  Side=SELL Price=546500000000 Qty=10000  -> qty_ahead now: Some(10000)
```

The first `ORDER_DELETE` (seq 10517) removed one of the three original
orders that really were counted in our hand sum — `qty_ahead` drops from
30,000 to 20,000, **exactly** by that order's quantity, not approximately.
Every subsequent `ORDER_MODIFY` at that price is a *different* order
arriving after ours (repricing in from elsewhere, or repricing back out) —
correctly invisible to our position, because those orders are genuinely
behind us in the real queue. The second real delete (seq 208828) removes
the second of the three original orders — `qty_ahead` drops to 10,000, again
exactly. **Queue position never moved except through genuine consumption
of the orders that were actually ahead of it** — FR-B24 invariant #4,
observed directly, not just asserted.

---

## 5. Order types (FR-B22) and modify semantics (FR-B23)

| Type | Behaviour, as implemented |
|---|---|
| `LimitDay(price)` | Attempts an immediate marketable sweep against the opposite side's *currently resting* liquidity, bounded by `price`; any unfilled remainder rests at the back of the combined FIFO at `price`. |
| `BookOrCancel(price)` | **Never sweeps.** Checked against `would_cross` (does the opposite touch already satisfy this price) *before* any book mutation; if it would cross, rejected outright (`RejectReason::WouldCross`) — no partial fill, ever. |
| `Ioc(price)` | Same immediate sweep as `LimitDay`, but any unfilled remainder is cancelled (`CancelReason::IocRemainder`), never rests. |
| `MarketToLimit` | Sweeps with **no price bound** — but only against liquidity genuinely resting *at arrival*, a single pass, never re-checking as if more could show up mid-sweep. Any unfilled remainder rests as a `LimitDay` at the price of the last level it touched. If zero opposite-side liquidity exists at all, there is no "traded price" to rest a residual at — rejected (`RejectReason::NoLiquidityForResidual`) rather than resting at an arbitrary price, a conservative, documented choice. |

**Two fill mechanisms, two different ground-truth bounds** (why this
matters is §7's invariant #1 discussion): a **passive** fill is a literal
sub-quantity of one specific real `Trade` message's own quantity, produced
only when that message's cascade (§3) reaches one of our resting orders.
An **aggressive** fill (the immediate sweep above) is bounded by quantity
that was genuinely resting in the book *at that instant* — real,
verifiable liquidity, just not tied to a specific real `Trade` message,
because no real trade actually happened at that moment (our hypothetical
order was never really sent to MCX).

**Fill estimator, not matching engine (2026-09-03).** Earlier, an
aggressive fill *physically* removed that quantity from the real slot it
matched — `simulator` was, in that one respect, acting as a matching
engine against its own book rather than only estimating what a real one
would have done. That's now closed: `sweep_opposite` reads a real slot's
`qty` and FIFO position but never writes them. What we took is tracked
separately (`SimBookImpl::consumed_by_us`, keyed by the real order's own
`priority_ts`), and every subsequent real event — a genuine `Trade`, a
`Delete`, a price-changing `Modify` — still finds that slot exactly as
the replay produced it. Two consequences:

- **The book `simulator` reports** (`best_bid`/`best_ask`/`depth`/
  `qty_at_price`) **never reflects our own trading.** It's the replay's
  ground truth, full stop — the same guarantee independence from `cache`
  (§1) gives against *inheriting* corruption now also holds against
  *causing* it.
- **Our own queue-position reads do** account for it: `qty_ahead_of` and
  `MboBook::queue_position` net a real slot's `qty` against
  `consumed_by_us` before counting it as "ahead" of one of our resting
  orders — so if we've already aggressively taken 12 of a real order's 20
  lots, a *different* order of ours resting behind that same slot
  correctly sees 8 ahead, not 20.

This closes D21's leak #3 ("you would have absorbed flow that
historically went elsewhere") for the replay itself, but not entirely for
us: our own fill genuinely happened, so nothing stops a *second*
aggressive order of ours (or, independently, the historical tape itself)
from also trading through the same real quantity in this simulation —
`consumed_by_us` only prevents *us* from re-claiming what we've already
virtually taken (`a_second_aggressive_order_cannot_reclaim_liquidity_we_already_virtually_took`,
§6.1); it doesn't reserve that liquidity against the recording's own
future trades. That's the accepted "no market impact" approximation this
whole design rests on (§8) — the recording plays out exactly as captured,
regardless of what we do alongside it.

**Modify (FR-B23):**
- Quantity **reduction only** (`new_qty <= current_remaining` **and** price
  unchanged) — same FIFO slot, same identity, in place.
- Quantity **increase**, or **any price change** — old identity removed
  entirely, a fresh one added at the back of the (possibly new) price
  level. Independently confirmed against `MCX_Feeder.cpp`'s real `OrderModify`
  handler (§3) for real orders; the same rule is applied to our own
  simulated resting orders in `SimExchange::modify`.

---

## 6. The validation binary — `simulator-validate`

Two modes, neither part of the public API, neither wired into `main.rs`
(off-limits this round, same as every other component's validation
harness — see `book_user_doc.md` §5.4 for the original precedent this
follows: no `[lib]` target exists in this crate, so a second `[[bin]]`
entry pointing at `src/simulator/validate.rs` is the only way to compile,
run, and test this component's own code without touching `main.rs`).

```
simulator-validate hand-trace   <increment-capture-file>
simulator-validate full-session <increment-capture-file> [loose]
```

- **`hand-trace`** — §4's worked example, generated fresh each run (self-
  selecting and self-validating a real, hand-checkable queue-position
  scenario, not a fixed hard-coded price).
- **`full-session`** — streams a full real session (record-by-record, via
  a `BufReader`-backed `RecordSource` reading the documented `[8B
  length][8B capture ts][payload]` outer framing directly off disk — the
  6.8GB CRUDEOIL file is never loaded whole into memory), runs a small
  resting quote-maintenance strategy (track best bid/best ask, re-quote via
  `modify` when the market moves) plus periodic `BookOrCancel`/`Ioc`/
  `MarketToLimit` probes, then asserts every FR-B24 invariant against the
  accumulated `AuditLog` and reports pass/fail with real counts.
  `loose` selects a second, generous OTR configuration for a supplementary
  evidence-gathering pass (see §7 for why both variants are reported,
  not a way of picking the friendlier number).

Both processes were launched as real background OS processes
(`nohup ... > log 2>&1 &`, polled via a monitor rather than waited on
synchronously) — the exact discipline `book_user_doc.md` credits with
saving results twice already on this project.

### 6.1 Unit tests (28, all pass)

`cargo test --release --bin simulator-validate` — small synthetic
`DecodedMessage`s, no real files needed: queue position exact on arrival
and exact decrement via real consumption ahead (never spontaneous
improvement); `BookOrCancel` rejects on cross and never fills; `IOC` fills
available then cancels the remainder; `MarketToLimit` residual rests
(never vanishes) and rejects cleanly when there is no reference price;
quantity-reduction keeps the FIFO slot, price-change and quantity-increase
both lose it (even at the same price); real `OrderMassDelete` cancels our
own resting order too; real `OrderModify` losing priority and
`OrderModifySamePriority` keeping it, each checked against our own resting
order's `qty_ahead`; a real `OrderDelete` ahead decrementing `qty_ahead` to
exactly zero; invariant #1's ceiling never exceeded by a real trade's own
cascade, plus a dedicated test proving the assertion pattern itself is a
live check, not decorative; and two dedicated OTR-governor tests — the
message-rate cap actually rejecting the message that would breach it (and
admitting again once the window has genuinely slid past), and a `modify`
call being gated by the same governor as a new order (D19: OTR counts
*messages sent*, not just new orders).

Four more (2026-09-03, the "fill estimator, not matching engine" change):
an aggressive fill leaves the real book's own displayed depth untouched
(`aggressive_fill_leaves_real_book_depth_untouched`); a second aggressive
order cannot re-claim quantity we already virtually took from the same
real slot (`a_second_aggressive_order_cannot_reclaim_liquidity_we_already_virtually_took`);
the virtual-consumption ledger for a real order is forgotten once that
order is genuinely deleted, so it can never bleed onto an unrelated new
order at the same price (`consumed_ledger_is_forgotten_once_the_real_order_it_tracked_is_gone`);
and one of our own resting orders sees a *reduced* `qty_ahead` for a real
slot we'd already aggressively drawn down, not the slot's full untouched
size (`our_own_resting_order_sees_reduced_qty_ahead_after_our_earlier_aggressive_fill`).

---

## 7. FR-B24 invariant results — full real session, CRUDEOIL

Run against `/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin`
(stream 4, 6.4GB, streamed record-by-record). `free -h` showed ~32GB
available at run time; the larger NATURALGAS capture (~30GB) was not also
run given CRUDEOIL alone already exercises every invariant with real
evidence and the task only requires "at least one instrument."

**114,423,913 total records processed** (all instruments, all templates);
**1,128,602 of them for CRUDEOIL**. Two configurations were run, both to
completion, both passing identically on every invariant:

- **Governed** — `max_messages_per_window=60`, `max_otr_ratio=200` (a
  believable venue-side cap).
- **Loose** — a much larger cap, run *in addition to* the governed pass
  specifically to harvest deeper evidence for invariants #1/#3/#4/#5 (a
  strategy that gets starved by a tight, realistic cap produces thin
  evidence for everything *except* the cap itself). Both are reported,
  not just the friendlier one — the governed run's numbers are the ones
  that matter for invariant #6's own claim.

Both runs produced **identical** invariant counts (899 restings, 896 reprice
attempts, 683 fills, 225 rejects, 7 cancels) — meaning this session's real
order-flow and our own quote-maintenance strategy's message rate never
actually burst past even the tighter (governed) cap. That is itself useful
evidence (the governed cap is realistic, not artificially tight to dodge
enforcement) but it means invariant #6's *rejection* behaviour is
demonstrated by dedicated unit tests (§6.1), not by this particular
session's traffic pattern — reported honestly rather than glossed over.

| # | Invariant | Result | Evidence |
|---|---|---|---|
| 1 | **Simulated fills at a price/time never exceed the volume that actually traded there** (strongest) | **PASS** | **10,801 real `Trade` messages checked, unconditionally, one at a time** (`assert!`, not `debug_assert!` — runs in every build). Each check compares the quantity this simulator attributed to our own resting orders against that *specific* trade message's own quantity — the tightest possible grain, stronger than aggregating by price+time bucket. **1** of those 10,801 trades produced a real passive fill (5,000 raw units, well within that trade's own quantity). **0 violations.** |
| 1b | Aggressive fills never exceed genuinely-resting quantity (supplementary — see §5 on why aggressive fills need a different ground truth) | **PASS** | 12 aggressive fill legs checked (each against the exact quantity resting at that price immediately before the sweep touched it), 0 violations. |
| 2 | `BookOrCancel` that would cross always rejects, never fills | **PASS** | 225 BOC submissions, all 225 correctly identified as crossing, all 225 rejected, **0 improperly filled**. |
| 3 | Fill price at-or-better than the order's limit | **PASS** | 558 fills checked (passive and aggressive together), 0 violations. |
| 4 | Queue position never improves except through genuine consumption ahead | **PASS** | 1,946 de-duplicated observations (only genuine transitions recorded, not every event — see `SimExchange::note_qty_ahead`), 0 violations. §4's worked trace is a hand-auditable slice of this same mechanism. |
| 5 | `MarketToLimit` residual rests, never vanishes | **PASS** | 125 `MarketToLimit` submissions, 0 with a vanished residual — but all 125, on this real session, happened to be filled completely at arrival (`residual_rested=false` on every single one; independently confirmed by grepping the run's own per-order log). The "residual actually rests" branch was **not** exercised by this real-data run at all — it's covered separately by the dedicated unit test `market_to_limit_residual_rests_never_vanishes` (§6.1), same honesty standard as invariant 6's note below. Worth re-running this invariant against a session/instrument where a real `MarketToLimit` genuinely outruns available liquidity, to get real-data coverage of the branch that actually matters here. |
| 6 | Simulated OTR/message-rate never exceeds configured limits | **PASS** | Enforced by construction (the governor rejects *before* admitting, in both `submit` and `modify`); this session's own traffic never forced a rejection (see above), so the enforcement itself is demonstrated by two dedicated deterministic unit tests (§6.1) rather than this run's numbers. |

**A real bug found and fixed during this same validation work, worth
recording as part of the evidence, not hidden:** the harness's own
real-time clock initially froze on the `u64::MAX` sentinel described in
§3, which silently suppressed almost all of the quote-maintenance
strategy's re-quoting for the *entire* session (2 reprice attempts and 6
`qty_ahead` observations, full-file, before the fix — compare to 896 and
1,946 after). All the numbers in the table above are from the run *after*
the fix. This is exactly the failure mode D31 warns about: a bug that
produces plausible-looking, passing output (the invariants technically
still held, vacuously, because almost nothing was happening) rather than
an obvious crash — caught here only by noticing the evidence was
suspiciously thin and tracing it back to its root cause, not by the
invariant checks themselves.

**Re-run after the "fill estimator, not matching engine" change
(2026-09-03), same file, governed OTR config:** intervening feature work
(`venue_order_id`, the quote-maintenance strategy this harness drives)
means the raw counts below aren't directly comparable to the table
above — reported separately rather than overwriting it. All six
invariants still **PASS**, including **1b now under the new per-slot,
effective-remaining accounting** described in §5 (6,381 aggressive fill
legs checked, 0 violations) — the tightened bound (genuinely-resting
*net of what we've already virtually taken*, not the level's raw
pre-sweep quantity) held on every real leg, not just the unit tests.
114,423,913 events processed, 1,128,602 for CRUDEOIL; 6,342 fills, 3,047
rejects, 42 cancels, 267,472 restings; invariant #4 saw 10,063 qty-ahead
observations (0 violations) — including, on real data, orders of ours
resting behind a real slot we'd already aggressively drawn down, exactly
the case `our_own_resting_order_sees_reduced_qty_ahead_after_our_earlier_aggressive_fill`
(§6.1) covers synthetically.

---

## 8. What this component deliberately does not do

Out of scope this milestone (FR-B25): self-trade prevention, Market Maker
Protection cancellations, OTR *enforcement* details beyond the governor
mechanism itself (real venue-specific formulas), watchdog cancellations,
Lean-order end-of-day cancellation. `simulator.rs`'s aggressive sweep does
allow a mechanical self-trade to occur (two of our own resting orders on
opposite sides matching each other) rather than silently preventing it —
correct for now, since STP is explicitly deferred, not simultaneous with
the core fill logic this milestone had to get solid first.

**No market impact.** Since 2026-09-03 (§5), the replay's own book is
never mutated by our own trading, on purpose — but our fills are still
real, so nothing in this design prevents the same genuinely-resting
quantity from being awarded to us *and* to the historical tape's own
future trades (`consumed_by_us` only stops *us* from re-claiming what
we've already virtually taken, not the recording from trading through it
independently). The model assumes the recording plays out exactly as
captured regardless of what we do alongside it; that assumption degrades
as our own order size grows relative to displayed depth, and faster when
quoting inside the spread — no participation cap or displacement model
exists yet to bound it.

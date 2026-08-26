# Execution — component documentation

**What this component does, in one sentence:** owns the order lifecycle
(eleven states, three gates, two genuinely different rejection paths),
two-level accounting, a direction-asymmetric cost model, and the run's
Tier 1/Tier 2 reports — BACKTEST-PHASE1.md §M7 (FR-B26 through FR-B31).

Code: [`execution.rs`](execution.rs) (this folder). Validation harness
(not part of the public API): [`validate.rs`](validate.rs), built as the
`execution-validate` binary — see §7.

---

## 1. The eleven-state order machine, and the race that matters

STRATEGY-GUIDE.md §7a's eleven states, verbatim (Nautilus's fifteen minus
`EMULATED`/`RELEASED`/`TRIGGERED`/`VOIDED` — no order emulator, no stop
orders, no contingent orders, per D12): `Initialized`, `Denied`,
`Submitted`, `Accepted`, `Rejected`, `PartiallyFilled`, `Filled`,
`PendingUpdate`, `PendingCancel`, `Canceled`, `Expired`.

Three groupings, each a pure function of the state (`OrderState::is_open`/
`is_inflight`/`is_terminal`):

| Grouping | States | Meaning |
|---|---|---|
| `is_open` | `Accepted`, `PartiallyFilled`, `PendingUpdate`, **`PendingCancel`** | "Do I still have a quote in the market?" |
| `is_inflight` | `Submitted`, `PendingUpdate`, `PendingCancel` | A message is out, awaiting a venue response |
| `is_terminal` | `Denied`, `Rejected`, `Filled`, `Canceled`, `Expired` | Nothing further can happen |

`PendingCancel` is deliberately in `is_open`, not just `is_inflight` — an
order you've asked to cancel is still genuinely resting and fillable
until the venue actually confirms the cancel. That is exactly what makes
the race below constructible rather than a hypothetical edge case.

### The `PendingCancel → Filled` race

A cancel is modelled as two phases, matching how a real message actually
travels: `request_cancel` marks the order `PendingCancel` locally — the
cancel has been *decided*, not yet *delivered* — and only
`deliver_cancel_to_venue` actually calls `SimExchange::cancel`, at
whatever later, latency-adjusted simulated instant the scheduler fires
that leg's arrival (T04/D36). `SimExchange` itself has no notion of
"pending cancel" — it only knows what's actually resting in its book —
so a real trade can fill an order at the venue in the gap between phase
1 and phase 2. `handle_exec_reports`'s `Filled` arm makes the resolution
explicit and unconditional:

```rust
// The race, made concrete: this assignment happens unconditionally,
// overriding PendingCancel (or any other non-terminal state) alike --
// a fill always wins.
order.state = if order.leaves_qty.0 <= 0 { OrderState::Filled } else { OrderState::PartiallyFilled };
```

And the `Canceled` arm refuses to regress a terminal order:

```rust
if !order.state.is_terminal() {
    order.state = OrderState::Canceled;
    ...
}
```

So when the moot cancel confirmation finally arrives late (nothing left
resting to cancel, `SimExchange` returns an empty `Vec<ExecReport>`), the
order stays exactly `Filled` — not regressed, not double-counted.
`pending_cancel_to_filled_race_the_fill_wins_not_silently_dropped_or_double_counted`
exercises all four steps explicitly (submit → resting; request cancel →
`PendingCancel`, `venue_cancel_calls() == 0`; a real trade fills it →
`Filled`, exactly one fill record, position reflects the fill; the cancel
is finally delivered → still `Filled`, still one fill record,
`venue_cancel_calls() == 1`). This is the acceptance bar's own named
scenario, not an incidental test.

## 2. Three gates, two rejection paths (FR-B27, D36)

`submit_order` runs, in order: an instrument-registry lookup, **Order
Validation** (tick size, freeze quantity — stateless, D17), **RMS**
(`trait Rms`, phase 1's `AlwaysAllowRms` always returns `true` — the call
site exists now so a real margin/cash-aware implementation slots in
later without touching this component, D34), then the **engine's own
local OTR/message-rate governor** (`LocalOtrGovernor`, independent state
from `SimExchange`'s own governor — D19: "they do not share state,
preserving the venue independence of D10"). Any of these failing returns
`GateOutcome::Denied` **synchronously**, with an `Order` record created
in state `Denied` (terminal) and `deny_reason` set — and, critically, no
call to `self.venue.submit(...)` at all. `tick_size_violation_...` and
`freeze_qty_violation_...` both assert `eng.venue_submit_calls() == 0`
from outside, which is the only way to actually prove "never reached the
venue" rather than merely asserting an error variant.

An order that passes all three gates gets `GateOutcome::Submitted`, and
*only then* does `self.venue.submit(...)` get called — genuinely reaching
`SimExchange`, which can refuse it on its own grounds (a `BookOrCancel`
that would cross, an unknown instrument, its own OTR breach). That
refusal arrives as an `ExecReport::Rejected` and is handled in
`handle_exec_reports`, moving the order to `Rejected` (terminal,
`reject_reason` set, `deny_reason` stays `None`) — a field and a state
entirely separate from `Denied`.

`venue_rejection_is_a_genuinely_different_terminal_state_from_denied`
makes both paths concrete side by side: a `BookOrCancel` buy priced to
cross a real resting sell passes every local gate
(`GateOutcome::Submitted`, `venue_submit_calls() == 1`) and only the
venue refuses it (`state == Rejected`, `reject_reason ==
Some(WouldCross)`, `deny_reason.is_none()`). D36 calls these "genuinely
different, not two flavours of one" precisely because of this: `Denied`
is decided in-process in nanoseconds and the venue never hears about it;
`Rejected` is a message that had to leave, travel, and come back.

## 3. Client order ID: `(session_id, counter)`, never wall-clock (FR-B28, D40)

`ClOrdIdGen` packs `session_id` (upper 24 bits) and a monotonic `counter`
(lower 40 bits) into one `u64`. `session_id` is injected at construction
— a deterministic value taken from `[run]` config in a backtest, process
start time in live — and `next()` never reads a clock or any OS-provided
randomness.

Two reasons this specific shape, both load-bearing, both with a test:

- **A wall-clock-derived id would break FR-12's determinism
  requirement.** Two runs of the identical backtest must produce
  byte-identical output; `two_identical_backtest_runs_produce_identical_client_order_ids`
  seeds two independent generators from the same `session_id` and asserts
  their first five ids are pairwise equal — nothing here can vary run to
  run because nothing here reads anything that varies run to run.
- **The `SimClock` does not advance within one scheduler callback**, so a
  bid and an ask submitted in the same dispatch share an identical
  `now_ns`. If uniqueness were derived from the timestamp, they would
  collide. `client_order_ids_are_distinct_within_the_same_simulated_instant`
  calls `next()` twice with no time passing between calls and asserts the
  two ids differ while still decoding to the same `session_id` (`a >>
  COUNTER_BITS == b >> COUNTER_BITS == 7`).

## 4. Two-level accounting (FR-B29, D08)

`Portfolio` owns one `SubAccount` per `strategy_id` — position (signed,
per instrument), weighted-average entry price, realised P&L, unrealised
P&L, total cost paid. `Portfolio::firm()` derives a `FirmAccount` by
netting every sub-account together, **computed on every call, never
independently stored or mutated** — there is no setter for `FirmAccount`
anywhere in the module. That is what actually makes "a strategy skews on
its own inventory, reads the firm view to degrade gracefully" a real
guarantee rather than a naming convention: nothing exposes a path to
write into the firm view from outside the aggregation itself.

`firm_account_nets_across_strategies_sub_accounts_stay_independent` makes
this concrete: strategy 1 buys 10, strategy 2 sells 4 of the *same*
instrument — each sub-account sees only its own fill (`+10`, `-4`), and
`portfolio().firm()` shows the net (`6`), because the exchange itself
sees one member, one session, not one strategy at a time.

## 5. Cost model: direction-asymmetric by regulation, not by choice (FR-B30, D23)

`CostModel::round_trip(instrument, qty, price, side) -> Cost` is the
**same function**, called both pre-trade (a strategy checking whether a
quoted edge clears its cost, STRATEGY-GUIDE.md §9's
`edge_ticks <= cost.in_ticks(instrument)`) and against every realised
fill (`on_fill`) — so a quoting assumption and the realised accounting
can never disagree about which components applied. Rates are `CostConfig`
fields, not literals baked into the arithmetic (a config with different
numbers changes the output; nothing here special-cases specific values).

**Why "round trip" is a per-leg function, not a two-legged one.** A
genuine buy-then-sell round trip always pays exactly one CTT leg (sell
side) and one stamp-duty leg (buy side) regardless of which leg came
first — so a *combined* round-trip total would always be side-symmetric
by simple arithmetic (add A's stamp duty + B's CTT, or B's stamp duty +
A's CTT — same sum either way). That symmetry would hide the actual
regulatory asymmetry FR-B30 requires proving exists. The asymmetry that
matters, and that the acceptance bar checks, is at the **leg** level: a
buy fill and a sell fill of identical qty/price/instrument cost
concretely different totals, because CTT (sell-side only) and stamp duty
(buy-side only) sit at different rates by regulation
(`CostConfig::default()`: `ctt_rate = 0.0001`, `stamp_duty_rate =
0.00002` — five times apart, not a rounding artifact). This is why
`round_trip` takes one `side`, computes one leg, and is applied identically
to each fill as it happens rather than pairing fills up after the fact.

Concrete numbers from the real, passing test
(`cost_model_buy_and_sell_round_trip_costs_concretely_differ`, `qty=10`,
`price=Rs 500.00`, `CostConfig::default()`), reproduced by
`execution-validate` scenario 3:

| Side | stamp_duty | ctt | total |
|---|---|---|---|
| Buy  | Rs 0.0100 | Rs 0.0000 | **Rs 236.0102** |
| Sell | Rs 0.0000 | Rs 0.0500 | **Rs 236.0502** |

Buy pays stamp duty and zero CTT; sell pays CTT and zero stamp duty; the
totals genuinely differ (`sell > buy`, not just "not bitwise equal") —
this is a hard floor on quotable spread, not a simplification a strategy
can opt out of, which is why the concession-free direction asymmetry is
this milestone's own explicit requirement rather than a nice-to-have.

`cost_is_queryable_pretrade_and_the_same_function_is_applied_to_the_realised_fill`
confirms the "can't disagree" property directly: a pre-trade query and
the `cost` field on the resulting `FillRecord` for the identical
qty/price/side come back bit-for-bit equal (`Rs 236.0502` both times in
`execution-validate`'s run).

## 6. Reporting (FR-B31, D26)

**Tier 1 — always on.** `ExecutionEngine::tier1_report()` returns a
`Tier1Summary`: firm-level gross/net/realised/unrealised P&L and total
cost, firm inventory, per-strategy breakdown, OTR admitted/rejected at
both the local and venue governor, message counts (attempts, denied,
submitted-to-venue, cancels, modifies, market events applied), terminal
state counts (`denied`/`rejected`/`filled`/`canceled`/`expired`, kept
separate — the same D08/D26 "tell these apart" requirement applied at
the summary level), and a markout distribution per configured horizon. It
embeds a `RunIdentity { config_hash, build_hash }` — `config_hash` is a
`DefaultHasher` hash of `RunConfig`'s `Debug` output (deterministic run
to run for an identical config, not cryptographic, and explicitly not
required to be by this milestone's acceptance bar); `build_hash` is
today a hardcoded literal (`"phase1-execution-v0"`), which
BACKTEST-PHASE1.md's own gate explicitly permits in place of D22's full
config-file/build-hash infrastructure.

Real printed output (`tier1_report_embeds_a_run_identity_and_prints_it`,
reproduced by `execution-validate` scenario 4):

```
=== qtrade run report (Tier 1) ===
run identity: config_hash=0xdfa7ff97a1fa3699 build_hash=phase1-execution-v0
--- firm level ---
gross_pnl=0.0000 net_pnl=0.0000 realized=0.0000 unrealized=0.0000 total_cost=0.0000
inventory: 
--- per-strategy ---
--- OTR ---
local: admitted=0 rejected=0 | venue: admitted=0 rejected=0
--- messages ---
new_order_attempts=0 denied=0 submitted_to_venue=0 cancel_requests=0 modify_requests=0 market_events_applied=0
--- terminal state counts ---
denied=0 rejected=0 filled=0 canceled=0 expired=0
--- markout ---
horizon_ns=1000000 observations=0 mean_raw_price_units=0.0000
horizon_ns=5000000 observations=0 mean_raw_price_units=0.0000
```

(A freshly-constructed engine with no activity — the point of this
specific test is the run-identity line's presence and exact format, not
a populated run; §7's `execution-validate` scenarios 1–3 exercise a
populated engine's state/cost numbers separately.)

**Tier 2 — switchable, per-event detail.** Every order command/response
with `Denied`/`Rejected` reasons distinguished, and a `FillRecord` per
fill carrying price, quantity, side, timestamp, `queue_position_at_fill`,
`spread_improving`, realised `Cost`, and a `markouts: Vec<(horizon_ns,
Option<i64>)>` **pre-populated with every configured horizon at creation
time**, values filled in later by `observe_markout` as that much
simulated time actually elapses. This pre-population is the literal
mechanism behind D26's "not optional, retrofitting means re-running
everything already trusted": the *slot* exists on every fill from the
instant it's recorded, even though the *value* genuinely cannot yet
(`queue_position_and_markout_fields_exist_on_every_fill_from_creation`
asserts both horizon slots exist and are `None` immediately after the
fill, before either is ever observed).

**Tier 3 — deferred.** Strategy-published series need `ctx.publish(...)`,
which doesn't exist on `strategy::Ctx` yet (out of scope, same as this
task's brief states) — a different gap from live fill/order-event
delivery, which is real now (see §6.1).

### 6.1 `ExecOutcome` — live delivery, not just end-of-run reporting (2026-08-25)

Until this pass, `fills()`/`order_events()` were read-only accessors a
caller queried once, after a run finished — the data existed live
(pushed the instant `handle_exec_reports`/`log_event` decide something
happened) but nothing was *notified*. `control_dispatcher` needed a way
to receive "what just happened," so each mutating method that can
produce a fill or order-event (`submit_order`, `on_market_event`,
`request_cancel`, `deliver_cancel_to_venue`, `request_modify`,
`deliver_modify_to_venue`, `mark_expired`) now returns an `ExecOutcome`
alongside its original result:

```rust
pub struct ExecOutcome { pub fills: Vec<FillRecord>, pub order_events: Vec<OrderEventRecord> }

pub fn submit_order(&mut self, intent: NewOrderIntent, now_ns: u64) -> (GateOutcome, ExecOutcome) {
    let (fills_before, events_before) = (self.fills.len(), self.order_events.len());
    let outcome = self.submit_order_inner(intent, now_ns);   // original body, renamed, unchanged
    (outcome, ExecOutcome { fills: self.fills[fills_before..].to_vec(), order_events: self.order_events[events_before..].to_vec() })
}
```

Every one of the seven methods follows this exact shape: snapshot
`self.fills`/`self.order_events`' lengths, call the renamed `..._inner`
(the pre-existing, unchanged logic), slice off whatever got added.
`handle_exec_reports`/`on_fill`/`log_event`/`deny` themselves are
**untouched** — this is a thin wrapper at the public boundary, not a new
accounting path, and every one of this file's existing tests still
passes with only mechanical call-site updates (`eng.submit_order(...)`
returning a tuple now, ~35 call sites here, ~14 more in
`execution-validate`).

**A real, discussed alternative was not taken:** `main.rs` could instead
diff `engine.fills()`/`engine.order_events()`'s lengths itself, before
and after calling `on_market_event`, with zero changes to this file at
all. Both options deliver identically to a strategy through
`control_dispatcher` — the choice here was for `ExecutionEngine` itself
to be the source of truth for "what did this call just produce," at the
accepted cost of the ~50 call-site updates above.

## 7. The queue-position bug: found, root-caused, fixed

### 7.1 What was wrong

`FillRecord.queue_position_at_fill` is supposed to answer "how much real
size was ahead of this order when it finally got filled" — the number
D26/FR-B31 call non-negotiable from day one. The original mechanism
(`ExecutionEngine::on_market_event`) snapshotted `SimExchange::
resting_qty_ahead` for every open order **immediately before applying
each market event**, into a `pre_event_qty_ahead: HashMap<u64, i64>` that
was **cleared and fully recomputed on every single call**:

```rust
// (before the fix)
self.pre_event_qty_ahead.clear();
for (&id, order) in self.orders.iter() {
    if order.state.is_open() {
        if let Some(ahead) = self.venue.resting_qty_ahead(id) {
            self.pre_event_qty_ahead.insert(id, ahead);
        }
    }
}
```

This is exact for a fill caused by the *very same* market event that
also consumes the quantity ahead of the order — the read happens before
`apply_market_event` runs, so it can't see that event's own effect on
itself. It breaks the moment a fill takes **more than one** market event
to arrive: an earlier event consumes the real quantity resting ahead
(driving the snapshot to `0`, correctly, for *that* event); a later,
separate event is the one that actually fills the order; and because the
map was cleared and freshly recomputed on that later call, the value
`on_fill` reads back is whatever was true immediately before *that*
event — `0` — not the `10` that genuinely had to trade through before
this order ever got filled.

`queue_position_and_markout_fields_exist_on_every_fill_from_creation`
caught this directly: it rests an order behind a real qty-10 sell order
at the same price, then delivers the fill across **two separate**
`Trade` events — the first consumes the 10 real ahead (no fill yet, our
order simply moves to the front), the second actually fills us. The
assertion failed exactly as this mechanism predicts:

```
assertion `left == right` failed: genuine pre-fill queue position captured, not fabricated
  left: Some(0)
 right: Some(10)
```

### 7.2 The fix

Price/time priority guarantees a resting order's qty-ahead can only ever
*shrink* while it stays resting (consumption ahead), never grow — nothing
can cut in front of an order once it is already resting at a price. That
means **the first reading taken while an order is open already is its
genuine, established queue position** — everything after that is just
watching it get consumed. The fix stops clearing the map and switches the
insert to `entry(...).or_insert(...)` — write once, never overwrite:

```rust
// (after the fix -- on_market_event)
for (&id, order) in self.orders.iter() {
    if order.state.is_open() {
        if let Some(ahead) = self.venue.resting_qty_ahead(id) {
            self.pre_event_qty_ahead.entry(id).or_insert(ahead);
        }
    }
}
```

The stored value now survives across however many separate market events
it actually takes for the fill to arrive, and `on_fill` reads back the
number that genuinely answers "how much did this order wait through" —
`10` in the regression case, matching the test's own expectation and
`execution-validate` scenario 5's real printed output
(`queue_position_at_fill = Some(10)`).

Entries are removed (`self.pre_event_qty_ahead.remove(&client_order_id)`)
at every point an order actually reaches a terminal state — fully
filled, rejected, canceled, expired — so the map stays bounded by the
number of *currently open* orders over a long run rather than growing
for the run's whole lifetime; `client_order_id`s are never reused, so a
terminal order's entry would otherwise sit unread forever.

### 7.3 How this was found

Not by inspection — by running the existing test suite for real (`cargo
test`) and reading the one failure's exact `left`/`right` values rather
than assuming the field was simply unpopulated. The specific numbers
(`Some(0)` vs `Some(10)`) immediately pointed at "stale/overwritten", not
"missing": a genuinely missing value would have printed `None`, not a
concrete, wrong `0`. Tracing the one call site that constructs
`queue_position_at_fill` (`on_fill`, reading `pre_event_qty_ahead`) back
to its only writer (`on_market_event`) showed the `clear()` at the top of
every call as the exact mechanism erasing the earlier, correct reading —
confirmed by walking the specific two-`Trade`-event scenario the test
constructs by hand against that code, before changing anything.

## 8. `Lots` vs `Qty`: a 10,000x cost bug, found and fixed

### 8.1 What was wrong

`NewOrderIntent.qty` used to be a plain `types::Qty` — and that one field
fed two consumers that silently disagreed about what scale it was in:

- `simulator::SimExchange` replays real `decoder::DecodedMessage`s
  directly, so real order matching needs wire-raw units (raw integer ÷
  10,000 = lots — confirmed directly by `simulator/validate.rs`'s own
  real-data test, which submits `Qty(10_000)` to mean **one lot** against
  a real CRUDEOIL book).
- `CostModel::round_trip`'s turnover formula (`rupees(price) * qty.0 *
  instrument.multiplier`) and its `brokerage_per_lot * qty.0` term were
  written and tested (this file's own `cost_model_buy_and_sell_...` unit
  test) assuming `qty.0` is a small literal **lot count** (e.g. `Qty(10)`
  = 10 lots).

`NewOrderIntent.qty` flowed into both uses completely unconverted. A
quantity correct for order matching (wire-raw) was exactly
`RAW_QTY_PER_LOT` (10,000x) too large every time it reached the cost
model. `dummy_strategy.rs`'s first real end-to-end run against a real
CRUDEOIL capture surfaced this concretely: a single 1-lot IOC fill
produced a reported round-trip cost in the hundreds of thousands of
rupees — see `dummy_strategy.md`'s "two real integration findings"
section for the actual before/after numbers.

### 8.2 The fix

Two new types in `types.rs` make the two scales impossible to confuse at
the type level, rather than relying on a comment to keep them straight:

```rust
pub struct Lots(pub i64);              // plain lot count -- what a strategy means by "1 lot"
pub const RAW_QTY_PER_LOT: i64 = 10_000;
impl Qty  { pub fn to_lots(self) -> Lots { Lots(self.0 / RAW_QTY_PER_LOT) } }
impl Lots { pub fn to_raw_qty(self) -> Qty { Qty(self.0 * RAW_QTY_PER_LOT) } }
```

`NewOrderIntent.qty` is now `Lots`, not `Qty` — a strategy expresses "how
many lots" as human input, and the engine converts explicitly at the one
place that actually needs the wire-raw scale:

```rust
// submit_order -- the only place intent.qty's Lots -> Qty conversion happens
let requested_qty = intent.qty.to_raw_qty();
let req = NewOrderRequest { ..., qty: requested_qty };   // simulator's native scale
```

`Order`'s own fill-tracking fields (`requested_qty`/`filled_qty`/
`leaves_qty`) **stay `Qty`, unchanged type, unchanged meaning** — they
interact with real fills from `simulator`, which is itself untouched and
native to that scale. `CostModel::round_trip` now takes `Lots` instead of
`Qty` in its own signature, so a caller holding a real fill's wire-raw
quantity must convert explicitly:

```rust
// on_fill -- the only place a real fill quantity meets the cost model
let cost = self.cost_model.round_trip(&instrument, qty.to_lots(), price, side);
```

`instrument.freeze_qty`'s check in `validate()` now compares directly
against `intent.qty` (`Lots`) — a real trading-limits concept like freeze
quantity is naturally expressed in lots, not wire-raw units, so this
comparison is finally in the right unit space (`freeze_qty` itself is
still always `0` from `refdata`'s own documented stub — a separate,
pre-existing data-completeness gap, not a units bug).

Because the type changed, not just a call site, the compiler enforces the
boundary: any future code that tries to hand `simulator` a `Lots` value
or `CostModel::round_trip` a raw `Qty` fails to compile rather than
silently producing a wrong number a fourth time.

### 8.3 Real before/after numbers

See `dummy_strategy.md`'s "two real integration findings" section for the
actual real-data run showing the fix — a 1-lot round-trip cost dropping
from an inflated ~10,000x figure to a real, sensible tens-of-rupees
magnitude.

## 9. The validation binary — `execution-validate`

Same reason `book`/`cache`/`simulator` each added a second `[[bin]]`
target: this crate has no `[lib]` target (every component is a
`#[path = "..."] mod` included directly from a binary's own root), and
`main.rs` is intentionally untouched this round (another pass wires
`execution` in for real later) — so a `tests/*.rs` integration test has
nothing to link against, and `execution.rs`'s own `#[cfg(test)] mod
tests` only compiles once *some* binary target declares `mod execution;`.
`src/execution/validate.rs` declares
`mod types; mod decoder; mod simulator; mod execution;` — the same
minimal, transitive-only dependency list `execution.rs` itself actually
`use`s (it does not touch `scheduler`/`book`/`refdata`/`cache` at all;
D10's venue-independence argument for `simulator`'s own validate binary
applies here too, one level up).

**Why synthetic scenarios, not a real capture-file replay** (unlike
`book-validate`/`cache-validate`/`simulator-validate`, which each stream a
real session). Every scenario this milestone's acceptance bar names — the
`PendingCancel → Filled` race, `Denied` never reaching the venue, the
cost model's direction asymmetry, a Tier 1 report with a real run
identity, genuine `queue_position_at_fill` — is about `execution`'s own
gate/state-machine/accounting logic, which does not depend on which real
session is replayed. A small, hand-checkable synthetic scenario is
*more* convincing evidence for this component specifically (every number
in it can be verified by hand, the same reasoning `simulator-validate`'s
own hand-trace mode already uses for its Layer 3 evidence), not a
shortcut taken in place of a harder real-data check.

Run: `cargo run --bin execution-validate` (prints all five scenarios and
a pass/fail summary); `cargo test --bin execution-validate` runs
`execution.rs`'s own 18 unit tests plus `simulator.rs`'s 18 (pulled in
transitively) — 36 total, all passing.

## 10. What this component deliberately does not do

**Historical list, from this component's original M7 build — two items below are now out of date, corrected inline rather than rewritten:**

- No margin or cash checks — a later, real `Rms` implementation's job
  (D34's own explicit deferral); `AlwaysAllowRms` exists so the call site
  is real today.
- ~~No `Strategy` trait / strategy-authoring API~~ — **partially superseded, 2026-08-25**: `event_dispatcher::MarketHandler` and `control_dispatcher::ControlHandler` are real now (see §6.1 and each component's own doc). Still no `ctx.submit()`/`ctx.cancel()` on `strategy::Ctx` — a strategy can *receive* fills/order-updates live, but still can't *cause* one through the `Ctx` handle; that remains separate follow-on work.
- No Tier 3 (strategy-published series) — nothing exists yet to publish
  from.
- ~~No `main.rs` wiring~~ — **superseded, 2026-08-25**: `main.rs` is the one real entry point now (see `main_user_doc.md`); this component's own validation binary (`execution-validate`) remains, for the same reason `book-validate`/`cache-validate`/`simulator-validate` still do.
- No real build-hash/config-file infrastructure (D22) — `BUILD_HASH` is
  a hardcoded literal and `RunConfig::hash()` is a `Debug`-formatted
  `DefaultHasher` hash, both explicitly permitted as placeholders by this
  milestone's own acceptance bar.

## 11. `Lots` vs `Qty`, a third time: realised/unrealised P&L was 100x wrong

### 11.1 What was wrong

§8 fixed the cost model's own Lots-vs-`Qty` confusion. `Portfolio::apply_fill`
(P&L accounting, called from `on_fill` for every real fill) and
`Portfolio::mark_to_market` had **the same class of bug, independently**,
in a different formula:

```rust
// (before the fix)
fn apply_fill(&mut self, ..., qty: Qty, price: Price, cost_rupees: f64) {
    let signed = match side { Side::Buy => qty.0, Side::Sell => -qty.0 };  // wire-raw, e.g. 10,000 for "1 lot"
    ...
    sub.realized_pnl += pnl_per_unit * closing_qty as f64;  // closing_qty derived from `signed` -- still wire-raw
    ...
    sub.position.insert(instrument, new_pos);  // position itself stored wire-raw
```

`qty` here is the fill's real `simulator`-native wire-raw quantity (e.g.
`Qty(10_000)` for one lot) — never converted to a lot count, and
`instrument.multiplier` (the real per-lot contract size, e.g. 100
barrels/lot for CRUDEOIL) never entered the formula at all.
`mark_to_market` had the identical shape:
`sub.unrealized_pnl = (mark - avg) * pos as f64` — `pos` read back from
the same wire-raw-scale `position` map, again with no `multiplier`.

**Hand-verified proof**, from a real `dummy-strategy` run against real
CRUDEOIL data (3 round-trip legs, 1 lot each, real fill prices from
`fills.log`):

| Leg | Bought at | Sold at | Real P&L (price diff × 1 lot × 100 barrels/lot) |
|---|---|---|---|
| 1 | Rs 5424 | Rs 5421 | (5421−5424) × 100 = **−300** |
| 2 | Rs 5422 | Rs 5417 | (5417−5422) × 100 = **−500** |
| 3 | Rs 5420 | Rs 5410 | (5410−5420) × 100 = **−1,000** |
| **Real total** | | | **−1,800** |

The actual pre-fix `gross_pnl`/`realized_pnl` was **−180,000** —
reproducible exactly by hand as `Σ (price_diff_rupees × 10,000)` over the
three legs: `(-3×10000) + (-5×10000) + (-10×10000) = -180,000`. That is
`raw_qty(10,000)` used directly in place of `lots(1) × multiplier(100)` —
`10,000 / 100 = 100`, a **different** net error factor from §8's cost bug
(which was a clean `RAW_QTY_PER_LOT` = 10,000x, since the cost formula's
`multiplier` argument was itself already correct at 1 in that bug's own
test instrument) — this one lands at 100x because `multiplier=100` for
real CRUDEOIL partially, but not fully, cancels the missing conversion.

This is a *third*, independent occurrence of the same underlying
confusion (`NewOrderIntent.qty`/`CostModel::round_trip` in §8, now
`Portfolio::apply_fill`/`Portfolio::mark_to_market` here) — each one in
different arithmetic, each one found because a real fill quantity met a
formula written assuming the other scale.

### 11.2 The fix

`apply_fill` now converts the fill's wire-raw `qty` to a lot count via
`Qty::to_lots()` at the top (the same conversion `on_fill` already made
before calling `CostModel::round_trip`), and takes a new `multiplier: i64`
parameter — the closing fill's real per-lot contract size — applied
exactly where a price *difference* becomes a real rupee P&L:

```rust
// (after the fix)
fn apply_fill(&mut self, ..., qty: Qty, price: Price, cost_rupees: f64, multiplier: i64) {
    let lots = qty.to_lots().0;
    let signed = match side { Side::Buy => lots, Side::Sell => -lots };
    ...
    sub.realized_pnl += pnl_per_lot * closing_qty_lots as f64 * multiplier as f64;
```

`on_fill` passes `instrument.multiplier` through — the same `instrument`
it already looked up for the cost-model call, so no new lookup is needed.
`mark_to_market` now takes `&Instrument` (not just `InstrumentId`) for the
same reason — `multiplier` has to come from somewhere, and the instrument
record is the only place it lives — and applies it identically:
`sub.unrealized_pnl = (mark - avg) * pos_lots as f64 * instrument.multiplier as f64`.
(`mark_to_market` has no caller yet in this codebase — no driver calls it
today — so this signature change has no other call site to update.)

**`SubAccount::position`/`FirmAccount::position` now store lots, not
wire-raw units.** This was a deliberate decision, not a forced one: every
real consumer of position (P&L math above, Tier 1/Tier 2 reporting) wants
a lot count, and there is no real use for wire-raw-scale position at the
accounting layer (unlike `Order.requested_qty`/`filled_qty`/`leaves_qty`,
which genuinely do need to stay wire-raw because they interact directly
with `simulator`'s own fill tracking). The field's own type stays a plain
`i64` rather than becoming `types::Lots` — `Lots` has no arithmetic
operators defined on it in `types.rs` (`+`, unary `-`, `.signum()`,
`.unsigned_abs()`, all needed here), and adding them was judged out of
scope for a fix confined to `execution.rs`/`validate.rs`/
`dummy_strategy.rs`, per this task's own constraint on touching
`types.rs`. Both fields now carry an explicit doc comment stating the
unit instead, so a future reader can't reintroduce this ambiguity a
fourth time by assuming raw scale from the bare `i64`.

**Every other `Qty` arithmetic site in `execution.rs` was audited** and
found either already correct or genuinely not applicable:

- `CostModel::round_trip` (§8) — already fixed, confirmed still correct
  after this change (`execution-validate` scenario 3 and
  `cost_is_queryable_pretrade_and_the_same_function_is_applied_to_the_realised_fill`
  both still pass unchanged).
- `FillRecord.qty: Qty` — left as wire-raw deliberately: it is a record
  of what actually happened at the wire level (audit trail, matches
  against real market data), and nothing reads it to do its own rupee
  math (`dummy_strategy.rs`'s `fills.log` writer divides by `LOT_RAW`
  purely for *display*, the same pattern its BBO printer already uses —
  not a money computation).
- `Order.requested_qty`/`filled_qty`/`leaves_qty` — confirmed correct to
  stay `Qty` (wire-raw): they track directly against `simulator`'s own
  fills, which is unchanged and native to that scale. Left untouched.
- `instrument.freeze_qty` check in `validate()` — already compares
  against `intent.qty` (`Lots`) since §8's fix; unaffected by this one.
- No OTR/message-rate/value-at-risk computation anywhere in this file
  combines `qty` and `price` into a rupee figure outside the two
  functions this section fixes and the cost model §8 already fixed.

### 11.3 Real before/after numbers

Same real `dummy-strategy` run against the real CRUDEOIL capture (6 fills,
3 round trips):

```
before (raw wire qty used directly, no multiplier):
  gross_pnl=-180000.0000  net_pnl=-180337.7045  realized=-180000.0000  total_cost=337.7045

after (lots * instrument.multiplier):
  gross_pnl=-1800.0000    net_pnl=-2137.7045    realized=-1800.0000    total_cost=337.7045
```

`total_cost` is unchanged (`337.7045`, §8's already-fixed figure) —
confirming this fix touched only the P&L formula, not the cost stack.
`-180000 / -1800 = 100`, exactly `RAW_QTY_PER_LOT (10,000) / multiplier
(100)` — the net factor this section's root-cause analysis predicts.
`inventory: InstrumentId(467013)=0` is unchanged in the printed report
(the demo's buy/sell alternation always ends flat), but the *scale* that
value is now expressed in changed from raw units to lots — a non-flat run
would previously have shown e.g. `10000` for a 1-lot position and now
correctly shows `1`.

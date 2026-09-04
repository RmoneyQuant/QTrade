# Writing a Strategy for qtrade

**A getting-started guide for strategy developers.**

This document assumes you have never seen this codebase before. It explains
everything a strategy can do in qtrade, everything it cannot, and every file
you have to touch to get from an empty editor to a finished backtest report.

Read Sections 1–7 before you write any code. Section 11 is a complete,
working strategy you can copy as a starting point.

---

## Table of contents

| §  | Section |
|----|---------|
| 0  | [What qtrade is, and what a "strategy" is here](#0-what-qtrade-is-and-what-a-strategy-is-here) |
| 1  | [Required language, toolchain, and how to build](#1-required-language-toolchain-and-how-to-build) |
| 2  | [Where your strategy lives](#2-where-your-strategy-lives) |
| 3  | [The `Strategy` trait — every callback qtrade offers](#3-the-strategy-trait--every-callback-qtrade-offers) |
| 4  | [`StartCtx` — resolving and subscribing to instruments](#4-startctx--resolving-and-subscribing-to-instruments) |
| 5  | [`Ctx` — everything you can read](#5-ctx--everything-you-can-read) |
| 6  | [`Ctx` — everything you can do (submit / cancel / modify)](#6-ctx--everything-you-can-do-submit--cancel--modify) |
| 7  | [Types and units — read this before you write a number](#7-types-and-units--read-this-before-you-write-a-number) |
| 8  | [Order types qtrade supports](#8-order-types-qtrade-supports) |
| 9  | [The order lifecycle — the states you will observe](#9-the-order-lifecycle--the-states-you-will-observe) |
| 10 | [Logging from your strategy](#10-logging-from-your-strategy) |
| 11 | [A complete worked example](#11-a-complete-worked-example) |
| 12 | [Wiring your strategy into `main.rs`](#12-wiring-your-strategy-into-mainrs) |
| 13 | [The config file — every key explained](#13-the-config-file--every-key-explained) |
| 14 | [Terminal commands — running your strategy](#14-terminal-commands--running-your-strategy) |
| 15 | [Reports — the four output files and every field in them](#15-reports--the-four-output-files-and-every-field-in-them) |
| 16 | [Rules, limits, and gotchas](#16-rules-limits-and-gotchas) |
| 17 | [What does *not* exist yet](#17-what-does-not-exist-yet) |
| A  | [Appendix A — one-page cheat sheet](#appendix-a--one-page-cheat-sheet) |

---

## 0. What qtrade is, and what a "strategy" is here

qtrade is a **backtester for MCX commodity futures**. It replays a real
recorded MCX market-data capture file — the exact bytes the exchange sent that
day — rebuilds the order book from those bytes, and hands your strategy the
same view of the market a live system would have had, at the same moments.

When your strategy places an order, that order goes to a **simulated
exchange** built into qtrade. The simulator holds your order in the real book
it just rebuilt, gives it a real place in the queue behind the orders that
were genuinely resting ahead of it, and fills it only when real market
activity would actually have filled it.

**A strategy in qtrade is one Rust struct.** It holds whatever state you want,
and it implements one trait — `Strategy`. qtrade calls your methods when
things happen (the book moved, a trade printed, your order filled), and you
call methods on a context object (`Ctx`) to look at the market and to place,
change, or cancel orders.

That is the whole model:

```
   recorded MCX capture file
             |
             v
   [ decoder ] --> [ book builder ] --> [ Cache ]
                                           |
                                           |  on_book / on_trade
                                           v
                                    +--------------+
                                    | YOUR STRATEGY|
                                    +--------------+
                                           |
                                           |  ctx.submit / cancel / modify
                                           v
                                  [ ExecutionEngine ]  <- local risk gates
                                           |
                                           v
                                  [ SimExchange ]      <- queue, matching, latency
                                           |
                                           |  on_fill / on_order_update
                                           v
                                    +--------------+
                                    | YOUR STRATEGY|
                                    +--------------+
```

### Two clocks (why this matters to you)

qtrade replays every captured packet **twice**, on two different clocks:

- **The exchange clock** (`exchange_ts` — the timestamp MCX itself stamped on
  the packet) drives the **simulated exchange's** book. This is the book your
  orders match against.
- **The capture clock** (`recorder_ts` — when our recording server actually
  received the packet) drives the **Cache**, which is the book *you* see from
  `ctx.book()`.

The capture clock is always *behind* the exchange clock by however long the
network really took that day. So **the book you see is slightly stale, exactly
as stale as it would have been live.** You are never allowed to trade on
information you could not have had. This is not a setting you can turn off,
and you do not have to do anything to get it — it is just how replay works.

On top of that, orders take time to reach the venue. See
[§16.4](#164-order-latency-is-real).

---

## 1. Required language, toolchain, and how to build

### 1.1 Language

**Rust.** There is no Python, C++, or scripting interface. Your strategy is
compiled into the qtrade binary.

- **Edition:** 2021
- **Toolchain built and tested against:** `rustc 1.98.0`
- **Crate name:** `qtrade`

If you do not have Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version      # confirm it prints a version
```

### 1.2 How much Rust do you need?

Not much, and mostly the boring parts:

- `struct` with fields, and `impl` blocks with methods
- `&mut self` methods (all your callbacks take `&mut self`, so you can freely
  mutate your own state)
- `Option<T>` and `match` / `if let Some(x) = ...`
- `enum` for your own state machine (strongly recommended — see §11)

You do **not** need to understand lifetimes, async, threads, or unsafe code.
qtrade is single-threaded and there is no `async` anywhere in the strategy
path. Your callbacks are called one at a time, in order, and never
concurrently.

### 1.3 Building

From the repository root:

```bash
cd qtrade
cargo build --release
```

The binary lands at `qtrade/target/release/qtrade`.

Always use `--release` for real runs. A full trading day's capture file is
tens of gigabytes and hundreds of millions of messages; a debug build is
roughly an order of magnitude slower and will make a full-day backtest
take hours instead of minutes.

To run the test suite (do this after you change anything shared):

```bash
cargo test --release
```

---

## 2. Where your strategy lives

Every strategy gets **its own subfolder** under `qtrade/src/strategy/`, holding
exactly two files with the same name as the folder:

```
qtrade/src/strategy/<your_strategy_name>/
├── <your_strategy_name>.rs    the code
└── <your_strategy_name>.md    what it does, how it wires in, what it deliberately doesn't do
```

The `.md` file is not optional in this project's convention. It should say, in
plain language: what the strategy watches, what it does when it wakes up, and
what it explicitly does *not* handle. Someone reading your `.md` should be able
to decide whether your strategy is relevant to them without opening the `.rs`.

Strategies already in the tree, useful as references:

| Folder | What it is |
|---|---|
| `limit_order_book_generator/` | Pure observer. Subscribes, prints the book, **submits no orders.** Simplest possible starting point. |
| `naturalgas_bracket/` | First order-placing strategy: a time-triggered bracket trade on NATURALGAS. |
| `multi_instrument_bracket/` | Trades two instruments at once with real resting limit orders, `modify()` and `cancel()`. |
| `order_lifecycle_demo/` | A scripted walk through *every* order state qtrade can produce. Best reference for "what does this callback actually receive". |

> **Only one strategy is compiled in at a time.** There is no runtime strategy
> loader and no multi-strategy config. Switching strategies is a source edit to
> `main.rs` — see [§12](#12-wiring-your-strategy-into-mainrs).

Shared infrastructure — the `Strategy` trait, `Ctx`, `StartCtx` — lives in
`qtrade/src/strategy/strategy.rs`, directly in the parent folder. Do not put
your strategy there.

---

## 3. The `Strategy` trait — every callback qtrade offers

This is the complete trait. It has **ten** methods. Only `on_start` has no
default body, so `on_start` is the only one you are *required* to write;
override the rest as you need them.

```rust
pub trait Strategy {
    // ---- required ----
    fn on_start(&mut self, ctx: &mut StartCtx);

    // ---- live and wired ----
    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, seq: u64, packet_transact_time_ns: u64) {}
    fn on_trade(&mut self, ctx: &mut Ctx, instrument: InstrumentId, trade: &Trade, seq: u64, packet_transact_time_ns: u64) {}
    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {}
    fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord) {}
    fn on_stop(&mut self, ctx: &mut Ctx) {}

    // ---- declared, but NOTHING CALLS THEM TODAY (see §17) ----
    fn on_warmup_complete(&mut self, ctx: &mut Ctx) {}
    fn on_timer(&mut self, ctx: &mut Ctx, timer: TimerId) {}
    fn on_session_change(&mut self, ctx: &mut Ctx, venue: Venue, phase: SessionPhase) {}
    fn on_book_state_change(&mut self, ctx: &mut Ctx, instrument: InstrumentId, state: BookState) {}
}
```

> **Important:** the last four compile, but **no code anywhere in qtrade ever
> calls them.** If you put logic in `on_timer`, it will never run. There is no
> timer facility, no warmup lifecycle, and no session-phase tracking yet. They
> are placeholders for future work. Do not build on them. See
> [§17](#17-what-does-not-exist-yet).

### 3.1 `on_start` — declare what you want to watch

```rust
fn on_start(&mut self, ctx: &mut StartCtx);
```

Called **once**, before any market data has been replayed. This is the *only*
place you can subscribe to instruments.

- You get a `StartCtx`, **not** a `Ctx`. You cannot read books (no data
  exists yet) and you cannot place orders here.
- Use it to resolve instrument names to ids and subscribe. Store the ids in
  your own struct — you will need them in every later callback.

### 3.2 `on_book` — the book for an instrument you subscribed to changed

```rust
fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, seq: u64, packet_transact_time_ns: u64);
```

| Parameter | Meaning |
|---|---|
| `ctx` | Your full context. **Orders are allowed here.** |
| `instrument` | Which instrument moved. You will get calls for every instrument you subscribed to, so always check which one this is. |
| `seq` | The exchange's own sequence number for the packet that caused this change. Monotonic per stream. Useful for logging and for detecting gaps. |
| `packet_transact_time_ns` | **The parameter name is misleading.** Despite the name, this is the *recorder* timestamp — when our capture server received the packet — and it is **exactly the same value `ctx.now()` returns**. It is not the exchange's `TransactTime`. The name is a leftover from before the dual-clock change; use whichever of the two you find clearer, they are identical. |

This is your main decision point. It fires **very often** — hundreds of
millions of times across a full day — so keep it cheap. Return early as fast
as you can when nothing interesting has happened.

### 3.3 `on_trade` — a real trade printed on the tape

```rust
fn on_trade(&mut self, ctx: &mut Ctx, instrument: InstrumentId, trade: &Trade, seq: u64, packet_transact_time_ns: u64);
```

`trade` is a `&Trade` with these public fields:

```rust
pub struct Trade {
    pub seq: u32,             // sequence number of this trade message
    pub full: bool,           // true if this was a full-size (non-partial) message
    pub security_id: i64,     // native MCX token this trade is on
    pub aggressor_side: Side, // Side::Buy = a buyer lifted the offer; Side::Sell = a seller hit the bid
    pub price: Price,         // trade price, in wire units (see §7)
    pub qty: Qty,             // trade quantity, in raw units (see §7)
    pub event_time: u64,      // the exchange's TransactTime for this trade, ns
}
```

`on_trade`'s own `seq` and `packet_transact_time_ns` parameters behave exactly
as in `on_book` — the latter is the recorder timestamp, equal to `ctx.now()`,
**not** the exchange's send time. If you want the exchange's own send time for
a trade, read `trade.event_time`, which genuinely is `TransactTime`.

**Orders are allowed here.** This is where you react to tape — momentum,
aggressor imbalance, sweep detection.

Note this is somebody *else's* trade on the public feed. Your own fills come
through `on_fill`, not here.

### 3.4 `on_fill` — one of *your* orders got filled

```rust
fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord);
```

```rust
pub struct FillRecord {
    pub fill_id: u64,                          // sequential id, unique within this run
    pub client_order_id: u64,                  // which of YOUR orders this fill belongs to
    pub strategy_id: StrategyId,
    pub instrument: InstrumentId,
    pub side: Side,                            // your side: Buy means you bought
    pub price: Price,                          // fill price, wire units
    pub qty: Qty,                              // filled quantity, raw units (NOT lots)
    pub kind: FillKind,                        // Passive (you were resting) or Aggressive (you crossed)
    pub timestamp_ns: u64,
    pub queue_position_at_fill: Option<i64>,   // how much size was ahead of you when you filled; None if you were aggressive
    pub spread_improving: bool,                // true if your order improved the touch when submitted
    pub cost: Cost,                            // full tax/fee breakdown for this fill (see §15.4)
    pub markouts: Vec<(u64, Option<i64>)>,     // (horizon_ns, price move) - see §15.3
}
```

A single order can produce **several** `on_fill` calls (a partial fill now,
more later). Always accumulate; never assume one fill completes an order.

> **You CANNOT submit, cancel, or modify from `on_fill`.** Calling
> `ctx.submit()` here returns `Err(CtxError::SubmitNotAllowedHere)`. See
> [§6.4](#64-the-can_submit-rule--the-single-most-important-rule-in-this-document).

### 3.5 `on_order_update` — one of *your* orders changed state

```rust
fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord);
```

```rust
pub struct OrderEventRecord {
    pub client_order_id: u64,
    pub timestamp_ns: u64,
    pub description: String,       // human-readable reason, e.g. "denied: MAX_SINGLE_TXN_QTY"
    pub resulting_state: OrderState,
}
```

You get one of these for **every** state transition your order makes:
`Submitted`, `Accepted`, `Rejected`, `PartiallyFilled`, `Filled`,
`PendingUpdate`, `PendingCancel`, `Canceled`, `Denied`. See
[§9](#9-the-order-lifecycle--the-states-you-will-observe) for the full list
and what each one means.

This is where you drive your state machine: "my entry order was accepted, so
now I am waiting for a fill".

> **You CANNOT submit, cancel, or modify from `on_order_update` either.**

### 3.6 `on_stop` — the replay finished

```rust
fn on_stop(&mut self, ctx: &mut Ctx);
```

Called exactly once, right after the last record in the capture file has been
replayed. Use it for a final summary log, or to record end-of-run state.

Two things to know:

- `ctx.now()` returns **0** here — there is no "current event" once the run is
  over, so there is no meaningful timestamp to report.
- Orders are **not** allowed here (`can_submit` is false). You cannot flatten
  your book in `on_stop`. If you want to be flat at the end of the day, you
  must place the closing order from an `on_book`/`on_trade` callback before
  the data runs out.

### 3.7 Which callbacks can place orders?

| Callback | Read `ctx`? | `submit` / `cancel` / `modify`? |
|---|---|---|
| `on_start` | no (you get `StartCtx`, not `Ctx`) | no |
| `on_book` | yes | **yes** |
| `on_trade` | yes | **yes** |
| `on_fill` | yes | **no** — returns `Err` |
| `on_order_update` | yes | **no** — returns `Err` |
| `on_stop` | yes | no — returns `Err` |

---

## 4. `StartCtx` — resolving and subscribing to instruments

`StartCtx` is handed only to `on_start`. It has exactly two methods.

```rust
impl StartCtx<'_> {
    pub fn resolve(&self, name: &str) -> Option<InstrumentId>;
    pub fn subscribe(&mut self, instrument: InstrumentId, depth: Depth);
}
```

### 4.1 `resolve(name)` — turn a name into an instrument id

```rust
let Some(id) = ctx.resolve("NATURALGAS") else { return };
```

`name` is an **underlying/commodity name**, not a contract symbol and not a
token number. `resolve` returns the `InstrumentId` for **this trading day's
front-month future** on that underlying, looked up out of that day's real MCX
contract master file (`MCXScrips.bcp`).

This matters a lot: you never hardcode a token. Point qtrade at a different
day's capture file and `resolve("NATURALGAS")` automatically finds *that*
day's front-month NATURALGAS future — a different token number, resolved for
you. Your strategy code does not change.

Returns `None` if the name does not resolve to a real front-month future in
that day's reference data. Always handle it.

**Which names are valid?** Only names your strategy declared in its
`UNDERLYINGS` constant (see §12.3). `main.rs` reads that constant, resolves
each name against the day's contract file, and builds the lookup table that
`resolve` reads. A name not in `UNDERLYINGS` will always return `None`.

### 4.2 `subscribe(instrument, depth)` — start receiving that instrument's data

```rust
ctx.subscribe(id, Depth::Bbo);
```

`Depth` is:

```rust
pub enum Depth {
    Bbo,        // best bid and best offer only
    Top(u8),    // the best N price levels on each side
}
```

- `Depth::Bbo` — you will be woken on changes to the touch. Cheapest.
- `Depth::Top(5)` — you will be woken on changes within the top 5 levels.

**If you do not subscribe to an instrument, you will never get `on_book` or
`on_trade` for it.** The `Depth` you request controls when you are *woken*;
`ctx.book()` always gives you the full rebuilt book regardless.

How the wake-up filter actually works: on every book touch, the dispatcher
compares your subscribed slice of the book against the previous one and calls
`on_book` **only if that slice changed value**. With `Depth::Bbo` it compares
`best_bid()`/`best_ask()`; with `Depth::Top(n)` it compares `book.depth(n)`.

> **Prefer `Depth::Bbo` unless you genuinely need depth.** The `Top(n)`
> comparison calls `book.depth(n)`, which **allocates a `Vec` on every book
> touch for that instrument** — hundreds of millions of times across a full
> day. `Bbo` compares two `Option<PriceLevel>`s and allocates nothing.

You may subscribe to several instruments, and you will be woken for each of
them. **Always check the `instrument` parameter** at the top of `on_book` /
`on_trade` — you will otherwise act on the wrong contract.

### 4.3 A complete `on_start`

```rust
const UNDERLYINGS: &[&str] = &["NATURALGAS", "CRUDEOIL"];

fn on_start(&mut self, ctx: &mut StartCtx) {
    for name in UNDERLYINGS {
        match ctx.resolve(name) {
            Some(id) => {
                ctx.subscribe(id, Depth::Bbo);
                self.instruments.push((*name, id));
                tracing::info!("{}", logging::line(
                    "MyStrategy", None, "SUBSCRIBE",
                    &format!("{name} -> token {}", id.0)));
            }
            None => {
                tracing::info!("{}", logging::line(
                    "MyStrategy", None, "SUBSCRIBE",
                    &format!("{name} -- NOT resolved in this day's refdata")));
            }
        }
    }
}
```

---

## 5. `Ctx` — everything you can read

`Ctx` is handed to every callback except `on_start`. Reads work from *any*
callback. This section covers all seven read methods; §6 covers the three
write methods.

### 5.1 `ctx.book(instrument) -> Option<&dyn Book>`

The rebuilt order book for that instrument, as *you* are allowed to see it
(on the capture clock — see §0).

Returns `None` if you never subscribed to that instrument, or if no data for
it has arrived yet.

The `Book` trait gives you five methods:

```rust
pub trait Book {
    fn best_bid(&self) -> Option<PriceLevel>;
    fn best_ask(&self) -> Option<PriceLevel>;
    fn depth(&self, n: usize) -> Vec<PriceLevel>;
    fn qty_at_price(&self, side: Side, price: Price) -> Qty;
    fn state(&self) -> BookState;
}

pub struct PriceLevel {
    pub price: Price,     // wire units - see §7
    pub qty: Qty,         // raw units - see §7
    pub order_count: u32, // how many separate orders make up this level
}
```

- **`best_bid()` / `best_ask()`** — the touch. `None` when that side is empty.
  This is what you will use 95% of the time.
- **`depth(n)`** — the best `n` **bid** levels, best-to-worst, **followed by**
  the best `n` **ask** levels, best-to-worst, in one flat `Vec`. There is no
  marker between the two halves, and if either side has fewer than `n` levels
  the vector is shorter and you cannot tell where the split is. **Use
  `best_bid()`/`best_ask()`/`qty_at_price()` when you need certainty**; use
  `depth(n)` only for display or for coarse shape metrics.
- **`qty_at_price(side, price)`** — total resting quantity at exactly that
  price on that side. `Qty(0)` if nothing is there.
- **`state()`** — see below.

### 5.2 `BookState` — is this book trustworthy?

```rust
pub enum BookState {
    Uninit,      // no data has arrived for this instrument yet
    Recovering,  // a gap was detected; the book is being rebuilt and is NOT reliable
    Ok,          // healthy
    Stale,       // no update for an unusually long time
}
```

**Check this before trading.** A defensive strategy starts every decision with:

```rust
let Some(book) = ctx.book(instrument) else { return };
if book.state() != BookState::Ok { return; }
let (Some(bid), Some(ask)) = (book.best_bid(), book.best_ask()) else { return };
```

### 5.3 `ctx.refdata() -> &InstrumentMaster`

The day's reference data — the parsed MCX contract master file. This is where
you get an instrument's *rules*.

```rust
let inst = ctx.refdata().get(instrument);   // Option<&Instrument>
```

```rust
pub struct Instrument {
    pub id: InstrumentId,
    pub venue: Venue,
    pub native_id: i64,             // the exchange's own token number
    pub kind: InstrumentKind,       // Future / Option / ...
    pub tick_size: Price,           // minimum price increment, wire units
    pub lot_size: i64,              // units of the commodity per lot
    pub multiplier: i64,
    pub max_single_order_qty: i64,  // MAX ORDER SIZE IN LOTS - see below
    pub price_band: Option<...>,    // that day's lower/upper circuit
    pub currency: ...,
}
```

The two fields you will actually use constantly:

**`tick_size`** — every limit price you submit **must** be an exact multiple of
this, or the order is denied locally with `DenyReason::TickSize`. Always round
to it:

```rust
fn tick_raw(ctx: &Ctx, instrument: InstrumentId) -> i64 {
    ctx.refdata().get(instrument).map(|i| i.tick_size.0).filter(|t| *t > 0).unwrap_or(10_000_000)
}

// round a raw price DOWN to the tick grid
let px = Price((raw / tick) * tick);
```

**`max_single_order_qty`** — the maximum number of **lots** a single order may
carry. This is a real, per-instrument, per-day value parsed straight out of
MCX's contract file (MCX calls it the *Maximum single transaction quantity*;
you may hear it called "freeze quantity"). It is a start-of-day constant — it
does not change during the session. For NATURALGAS on a recent day it was
**48 lots**, not some large round number. If you submit more than this, the
order is denied locally with `DenyReason::MaxSingleOrderQty` and it never
reaches the venue.

```rust
fn max_order_lots(ctx: &Ctx, instrument: InstrumentId) -> i64 {
    ctx.refdata().get(instrument).map(|i| i.max_single_order_qty).filter(|q| *q > 0).unwrap_or(1)
}
```

If you want a bigger position than one order allows, you must slice it into
several orders yourself. qtrade will not do it for you.

### 5.4 `ctx.now() -> u64`

The current simulation time in **nanoseconds since the Unix epoch**, on *your*
clock (the capture clock). This is the timestamp of the event currently being
delivered to you.

This is what you use for all time-based logic — "has 30 seconds passed since I
entered?", "is it past 14:30 IST?".

```rust
const ENTRY_NS: u64 = 1_787_286_600_000_000_000;  // 10:00 IST on 21-08-2026
if ctx.now() < ENTRY_NS { return; }
```

What it actually is, per callback:

| Callback | `ctx.now()` returns |
|---|---|
| `on_book` / `on_trade` | The recorder timestamp of the packet being delivered — the same value as the `packet_transact_time_ns` parameter. |
| `on_fill` | `fill.timestamp_ns` |
| `on_order_update` | `update.timestamp_ns` |
| `on_stop` | **`0`** — there is no current event once the run is over. |

To turn a nanosecond timestamp into a readable IST string for a log message,
use `logging::fmt_ist(now_ns)`.

### 5.5 `ctx.order(client_order_id) -> Option<&Order>`

Look up one of your own orders by the id `submit()` returned.

```rust
pub struct Order {
    pub client_order_id: u64,
    pub venue_order_id: Option<u64>,     // the exchange's own id; None until the venue accepts it
    pub strategy_id: StrategyId,
    pub instrument: InstrumentId,
    pub side: Side,
    pub order_type: OrderType,
    pub requested_qty: Qty,              // raw units
    pub state: OrderState,               // see §9
    pub filled_qty: Qty,                 // raw units
    pub leaves_qty: Qty,                 // raw units still working
    pub working_price: Option<Price>,    // where it is resting now, if it is resting
    pub deny_reason: Option<DenyReason>,
    pub reject_reason: Option<RejectReason>,
    pub cancel_reason: Option<CancelReason>,
    pub spread_improving: bool,
}
```

Returns `None` if that id was never issued by this run.

This is the honest way to answer "is my order still working?":

```rust
let still_working = ctx.order(id)
    .map(|o| o.leaves_qty.0 > 0)
    .unwrap_or(false);
```

> **You do not need `venue_order_id` for anything.** qtrade handles the
> translation to the exchange's own order id internally. Every method you call
> — `cancel`, `modify`, `order` — takes the `client_order_id` that `submit`
> gave you. `venue_order_id` is exposed for logging and diagnostics only.

### 5.6 `ctx.position(instrument) -> i64`

Your strategy's **net position in that instrument, in lots**. Positive is
long, negative is short, `0` if you have never traded it.

This is *your* position, not the firm's. If two strategies were running (not
possible today — one strategy at a time), each would see only its own.

### 5.7 `ctx.pnl() -> Pnl`

```rust
pub struct Pnl {
    pub gross: f64,   // rupees, before costs
    pub net: f64,     // rupees, after all taxes/fees/brokerage
}
```

Again, *your* strategy's P&L, not the firm's.

`gross` is mark-to-market plus realised. `net` is `gross` minus every cost
charged so far. The difference between them is real and often large — see
§15.4.

### 5.8 `ctx.cost(instrument, qty, price, side) -> Option<Cost>`

**Pre-trade cost query.** Asks: "if I filled `qty` lots of `instrument` at
`price` on `side`, what would that cost me?"

```rust
let c = ctx.cost(instrument, Lots(5), px, Side::Buy);
if let Some(c) = c {
    if c.total_rupees > my_expected_edge { return; }   // not worth it
}
```

Returns `None` only if the instrument is not in the engine's registry at all.

This runs the **exact same cost model** that a real fill is later charged
through, so your pre-trade estimate and your realised accounting can never
quietly disagree.

See §15.4 for the full `Cost` breakdown.

---

## 6. `Ctx` — everything you can do (submit / cancel / modify)

Three methods. All three return `Result<_, CtxError>` and all three fail with
the same error if called from a callback that does not allow orders.

```rust
pub enum CtxError {
    SubmitNotAllowedHere,
}
```

### 6.1 `ctx.submit(instrument, side, order_type, qty) -> Result<u64, CtxError>`

```rust
let id = ctx.submit(
    instrument,            // InstrumentId
    Side::Buy,             // Side::Buy or Side::Sell
    OrderType::LimitDay(px),  // see §8
    Lots(1),               // QUANTITY IN LOTS
)?;
```

Returns your **`client_order_id`** — a `u64` you must keep. Everything you do
to that order later (`cancel`, `modify`, `order`) uses this id, and every
`FillRecord` and `OrderEventRecord` you receive carries it.

**Important:** `Ok(id)` does **not** mean the order was accepted. It means
qtrade issued you an id. The order may already have been denied by a local
gate, and you will find out via `on_order_update` with `state == Denied`.
`submit` returns `Err` **only** when you called it from a forbidden callback.

The quantity parameter is `Lots` — a lot count, e.g. `Lots(5)` for 5 lots.
This is the one place in the strategy API that takes lots rather than raw
units. See §7.

### 6.2 `ctx.cancel(client_order_id) -> Result<(), CtxError>`

```rust
ctx.cancel(id)?;
```

Requests cancellation of one of your working orders. If the order was not open
(already filled, already cancelled, never reached the venue), nothing is sent
and nothing happens — this is not an error.

You will see the outcome as `on_order_update` with `PendingCancel`, then
`Canceled`.

### 6.3 `ctx.modify(client_order_id, new_qty, new_price) -> Result<(), CtxError>`

```rust
// change price and quantity
ctx.modify(id, Qty(1 * RAW_QTY_PER_LOT), Some(new_px))?;

// change quantity only, leave the price where it is
ctx.modify(id, Qty(2 * RAW_QTY_PER_LOT), None)?;
```

> **⚠ Watch the units.** `submit` takes `Lots`. `modify` takes **`Qty`**, which
> is in **raw units**, where `1 lot == 10_000 raw units`
> (`types::RAW_QTY_PER_LOT`). Passing `Qty(1)` when you meant one lot will
> silently ask for 1/10000th of a lot. This is the most common mistake new
> strategy authors make in this codebase.

**Queue-priority rules, which are real and matter:**

- **Reducing quantity at the same price keeps your place in the queue.** This
  is how you shrink a quote without losing your spot.
- **Increasing quantity loses priority**, even at the same price — you go to
  the back.
- **Changing the price loses priority** — you are a new order at the new price.

You will see `on_order_update` with `PendingUpdate`, then `Accepted`.

### 6.4 The `can_submit` rule — the single most important rule in this document

**You may only place, cancel, or modify orders from `on_book` and
`on_trade`.**

From `on_fill`, `on_order_update`, or `on_stop`, all three methods return
`Err(CtxError::SubmitNotAllowedHere)` and do nothing.

This is deliberate, not a limitation to work around. `on_fill` and
`on_order_update` are *notification* callbacks — they tell you what already
happened. Letting a strategy chain new orders directly off a fill notification
produces re-entrancy that is impossible to reason about, and does not match how
a real trading system's control path works.

**The correct pattern** is to set a flag in your state machine from the
notification callback, and act on it from the next `on_book`:

```rust
enum State { Idle, WaitingForEntryFill, ReadyToExit, Done }

fn on_fill(&mut self, _ctx: &mut Ctx, fill: &FillRecord) {
    // NOTE: no orders here. Just record and advance the state machine.
    if self.state == State::WaitingForEntryFill && fill.client_order_id == self.entry_id {
        self.entry_price = Some(fill.price);
        self.state = State::ReadyToExit;
    }
}

fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, _t: u64) {
    if self.state == State::ReadyToExit {
        // ...and NOW we can act on it.
        let id = ctx.submit(instrument, Side::Sell, OrderType::LimitDay(px), Lots(1)).unwrap();
        self.exit_id = id;
        self.state = State::Done;
    }
}
```

Because `on_book` fires constantly, the delay between setting the flag and
acting on it is negligible — typically the very next market message.

---

## 7. Types and units — read this before you write a number

qtrade uses newtype wrappers over `i64` so you cannot accidentally pass a
price where a quantity belongs. All of them live in `qtrade::types`.

```rust
pub struct Price(pub i64);          // a price, in WIRE UNITS
pub struct Qty(pub i64);            // a quantity, in RAW UNITS
pub struct Lots(pub i64);           // a quantity, in LOTS
pub struct InstrumentId(pub u32);   // MCX's native token number
pub enum Side { Buy, Sell }
```

You get at the inner number with `.0`, e.g. `bid.price.0`.

### 7.1 The two scale constants — memorise these

```rust
pub const WIRE_PRICE_PER_RUPEE: i64 = 100_000_000;   // 1e8
pub const RAW_QTY_PER_LOT:      i64 = 10_000;        // 1e4
```

| Concept | Type | Unit | Conversion |
|---|---|---|---|
| Price | `Price(i64)` | wire units | `rupees = raw / 100_000_000` |
| Quantity (books, fills, `modify`) | `Qty(i64)` | raw units | `lots = raw / 10_000` |
| Quantity (`submit` only) | `Lots(i64)` | lots | — |

### 7.2 Worked conversions

```rust
const RUPEE_RAW: f64 = 100_000_000.0;

// wire price -> rupees, for printing
let rupees = bid.price.0 as f64 / RUPEE_RAW;         // 26_330_000_000 -> 263.30

// rupees -> wire price (avoid; prefer building from an existing book price)
let px = Price((263.30 * RUPEE_RAW) as i64);

// raw qty -> lots
let lots = level.qty.0 / crate::types::RAW_QTY_PER_LOT;   // 420_000 -> 42

// lots -> raw qty, for modify()
let q = Qty(3 * crate::types::RAW_QTY_PER_LOT);           // 3 lots -> Qty(30_000)
```

### 7.3 Prices must sit on the tick grid

A limit price that is not an exact multiple of the instrument's `tick_size` is
denied locally, before it ever reaches the venue. Never compute a price by
multiplying a float; always derive it from the book and step by ticks:

```rust
let tick = ctx.refdata().get(instrument).map(|i| i.tick_size.0).unwrap_or(0);
if tick <= 0 { return; }

// three ticks below the bid - guaranteed on-grid because `bid.price` already is
let px = Price(bid.price.0 - 3 * tick);
```

If you must round an arbitrary raw number onto the grid:

```rust
let down = Price((raw / tick) * tick);                      // round down
let up   = Price(((raw + tick - 1) / tick) * tick);         // round up
```

### 7.4 Other types you will meet

```rust
pub enum Venue { Mcx, ... }
pub struct Date(pub i64);
pub struct YearMonth { pub year: i32, pub month: u32 }
```

---

## 8. Order types qtrade supports

qtrade supports **four** order types. They are the ones an MCX CTCL member can
actually send; there is no "market order" that rests forever, no stop order, no
iceberg.

```rust
pub enum OrderType {
    LimitDay(Price),       // a normal resting limit order
    BookOrCancel(Price),   // post-only
    Ioc(Price),            // immediate-or-cancel
    MarketToLimit,         // sweep, then rest the remainder
}
```

### 8.1 `LimitDay(price)` — the workhorse

MCX `OrdType=2`, `TimeInForce=0` (Day).

A normal limit order. If it crosses the book on arrival, it fills immediately
against whatever is available; the unfilled remainder **rests** at your limit
price for the rest of the day.

```rust
ctx.submit(inst, Side::Buy, OrderType::LimitDay(Price(bid.price.0 - 3 * tick)), Lots(1))?;
```

Use this for: passive quoting, resting entries, resting exits, and for
aggressive fills where you also want to leave the remainder working.

### 8.2 `BookOrCancel(price)` — post-only

You are telling the exchange: *I only want to make, never take.* If the order
would cross the opposite side on arrival, the venue **rejects it outright** —
`RejectReason::WouldCross` — and nothing is placed. Otherwise it rests.

```rust
// safe passive quote: guaranteed to never pay the spread
ctx.submit(inst, Side::Buy, OrderType::BookOrCancel(Price(bid.price.0)), Lots(1))?;
```

Use this when accidentally crossing would be worse than not trading at all.

### 8.3 `Ioc(price)` — immediate-or-cancel

MCX `TimeInForce=3`. Fills whatever is available at or better than your limit,
right now, and **cancels the rest instantly**. Nothing ever rests.

You will see a `Filled`/`PartiallyFilled` update, then a `Canceled` update with
`CancelReason::IocRemainder`.

Use this to take liquidity without leaving a footprint behind.

### 8.4 `MarketToLimit` — sweep, then rest

MCX `OrdType=5`. Takes no price from you. It sweeps the opposite side
aggressively, and any remainder is left **resting at the price of the last
trade it made**.

If there is no liquidity at all on the other side, it is rejected with
`RejectReason::NoLiquidityForResidual`.

Use this for urgent fills where you still want the remainder working at a sane
price.

### 8.5 Choosing

| You want to... | Use |
|---|---|
| quote passively and stay in the queue | `LimitDay` or `BookOrCancel` |
| guarantee you never cross | `BookOrCancel` |
| take liquidity now, leave nothing behind | `Ioc` |
| take liquidity now, leave the rest working | `LimitDay` at an aggressive price, or `MarketToLimit` |
| get out urgently | `MarketToLimit` or an aggressive `LimitDay` |

---

## 9. The order lifecycle — the states you will observe

Every order you submit walks through a subset of these eleven states. You are
told about every transition via `on_order_update`.

```rust
pub enum OrderState {
    Initialized,      // created internally; you will not normally observe this
    Denied,           // OUR OWN gate refused it. It never left the building.
    Submitted,        // passed our gates, on its way to the venue
    Accepted,         // the venue accepted it and it is resting in the book
    Rejected,         // the VENUE refused it
    PartiallyFilled,  // some quantity filled; the remainder is still working
    Filled,           // fully filled. Terminal.
    PendingUpdate,    // you called modify(); waiting for the venue
    PendingCancel,    // you called cancel(); waiting for the venue
    Canceled,         // terminal
    Expired,          // terminal (no expiry mechanism is wired up today)
}
```

### 9.1 Denied vs Rejected — an important distinction

- **`Denied`** — *we* stopped it. It never reached the exchange. This is a
  local pre-trade risk gate catching a bad order. Your fault, and cheap.
- **`Rejected`** — the *venue* stopped it. It went out, and came back refused.

**`DenyReason` — why *we* refused it:**

| Reason | Meaning | How to avoid |
|---|---|---|
| `TickSize` | Your limit price is not a multiple of `tick_size` | §7.3 |
| `MaxSingleOrderQty` | More lots than `instrument.max_single_order_qty` allows | §5.3 — slice it |
| `RmsRejected` | The risk-management layer said no | check your position limits |
| `LocalOtrOrRate` | You exceeded the local order-to-trade ratio / message rate | slow down |
| `UnknownInstrument` | That `InstrumentId` is not registered for order entry | you subscribed to something not in `UNDERLYINGS` |

**`RejectReason` — why the *venue* refused it:**

| Reason | Meaning |
|---|---|
| `WouldCross` | A `BookOrCancel` order that would have taken liquidity |
| `NoLiquidityForResidual` | A `MarketToLimit` order with nothing to sweep |
| `UnknownInstrument` | The venue does not know that instrument |
| `OtrOrRateExceeded` | The venue's own order-to-trade / rate limit |

**`CancelReason` — why an order ended up cancelled:**

| Reason | Meaning |
|---|---|
| `Strategy` | You called `ctx.cancel()` |
| `IocRemainder` | The unfilled part of an `Ioc` order |
| `MassCancel` | A mass-cancel was issued |
| `Watchdog` | A watchdog timer fired |
| `Mmp` | Market-maker protection |
| `SessionLoss` | Connection to the venue was lost |
| `EndOfDay` | End-of-session cleanup |
| `Risk` | A risk trigger |

### 9.2 The transitions you will actually see

```
  submit()
     |
     +-- local gate fails -----> Denied                                [terminal]
     |
     +-- local gate passes ---> Submitted
                                    |
                                    +-- venue refuses -----> Rejected  [terminal]
                                    |
                                    +-- rests -------------> Accepted
                                    |                           |
                                    |            modify() -> PendingUpdate -> Accepted
                                    |            cancel() -> PendingCancel -> Canceled  [terminal]
                                    |                           |
                                    |            market trades through you
                                    |                           v
                                    +-- fills immediately -> PartiallyFilled -> ... -> Filled  [terminal]
```

A real `orders.log` excerpt showing exactly this (from `order_lifecycle_demo`):

```
t=1787286923915582739 client_order_id=1099511627776  state=Denied          denied: MAX_SINGLE_TXN_QTY
t=1787286944257313039 client_order_id=1099511627777  state=Submitted       submit: gates passed, forwarding to venue
t=1787286944257813039 client_order_id=1099511627777  state=Rejected        venue rejected: WouldCross
t=1787286968100472974 client_order_id=1099511627778  state=Submitted       submit: gates passed, forwarding to venue
t=1787286968100972974 client_order_id=1099511627778  state=Accepted        resting
t=1787286989833718452 client_order_id=1099511627778  state=PendingUpdate   modify requested
t=1787286989834218452 client_order_id=1099511627778  state=Accepted        resting
t=1787287009961719895 client_order_id=1099511627778  state=PendingCancel   cancel requested
t=1787287009962219895 client_order_id=1099511627778  state=Canceled        canceled: Explicit
t=1787287095844041581 client_order_id=1099511627779  state=Submitted       submit: gates passed, forwarding to venue
t=1787287095844541581 client_order_id=1099511627779  state=PartiallyFilled partially filled qty=420000 kind=Aggressive (leaves=60000)
```

Notice the **500 microsecond gap** between `Submitted` and the venue's reply
(`...313039` -> `...813039`). That is the configured order round-trip latency,
and it is real — see §16.4.

### 9.3 Passive vs aggressive fills

`FillRecord::kind` tells you which you got:

- **`FillKind::Passive`** — you were resting, and somebody traded into you.
  `queue_position_at_fill` is `Some(n)`: how much quantity was ahead of you in
  the queue at that price when you got hit. A large number means you were deep
  in the queue and got lucky.
- **`FillKind::Aggressive`** — you crossed the spread and took liquidity.
  `queue_position_at_fill` is `None` (you never queued).

Passive fills are the ones the simulator works hardest to get right: your order
is placed at the back of the real queue that existed at that price, and it only
fills once enough real volume has traded through the orders that were genuinely
ahead of you.

---

## 10. Logging from your strategy

Everything your strategy prints goes into `events.log`, interleaved in
timestamp order with qtrade's own component logs. That interleaving is the
single most useful debugging tool you have.

Use `logging::line(component, now_ns, tag, message)`:

```rust
use crate::logging;

tracing::info!("{}", logging::line(
    "MyStrategy",           // component name - use your struct's name
    Some(ctx.now()),        // Some(ns) for a timestamped line; None before replay starts
    "ENTRY",                // a short ALL-CAPS tag
    &format!("submitting BUY {} lots @ Rs {:.2}", 1, px.0 as f64 / 100_000_000.0),
));
```

Most strategies in this tree define a small local macro to avoid repeating the
component name:

```rust
macro_rules! log {
    ($tag:expr, $($arg:tt)*) => {{
        tracing::info!("{}", logging::line("MyStrategy", Some(now_ns), $tag, &format!($($arg)*)))
    }};
}

log!("ENTRY", "submitting BUY {} lots @ Rs {:.2}", lots, px.0 as f64 / RUPEE_RAW);
```

The resulting lines look like this:

```
t=1787286902453872508 (2026-08-21 10:05:02.453 IST) [OrderLifecycleDemo] SCRIPT: starting -- NATURALGAS bid Rs 263.20 / ask Rs 263.40, tick Rs 0.10
t=1787286923915582739 (2026-08-21 10:05:23.915 IST) [ExecutionEngine] Denied: client_order_id=1099511627776 venue_order_id=NA denied: MAX_SINGLE_TXN_QTY
t=1787286923915582739 (2026-08-21 10:05:23.915 IST) [ControlDispatcher] DISPATCH: on_order_update(client_order_id=1099511627776 state=Denied)
t=1787286923915582739 (2026-08-21 10:05:23.915 IST) [OrderLifecycleDemo] ORDER_UPDATE: client_order_id=1099511627776 state=Denied -- denied: MAX_SINGLE_TXN_QTY
```

You can see your own line, the engine's line, and the dispatcher's line for the
same event, all at the same nanosecond. That is how you find out *why*
something happened.

> **Do not log from inside a hot path unconditionally.** `on_book` fires
> hundreds of millions of times. Log only when your state machine actually does
> something. A strategy that logs every book update will produce a
> multi-gigabyte `events.log` and slow the run to a crawl.

---

## 11. A complete worked example

This is a full, self-contained strategy. It:

1. Subscribes to the front-month NATURALGAS future.
2. Waits until 10:05 IST.
3. Places a passive bid three ticks below the touch.
4. If the market moves away, follows it with `ctx.modify()`.
5. When it fills, immediately arms an exit.
6. Places the exit on the next book update, five ticks above the entry.
7. Logs everything.

Save it as `qtrade/src/strategy/passive_follower/passive_follower.rs`.

```rust
//! passive_follower -- a minimal but complete example strategy.
//!
//! Places one passive bid, follows the market with modify(), and exits
//! five ticks above the entry once filled.

use crate::book::Book;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord};
use crate::logging;
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{BookState, InstrumentId, Lots, Price, Qty, Side, RAW_QTY_PER_LOT};

/// `main.rs` reads this to decide which instruments to resolve and load
/// reference data for. Every name you intend to `ctx.resolve()` must be here.
pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

const RUPEE_RAW: f64 = 100_000_000.0;

/// 10:05 IST on 2026-08-21. Change this to match your capture file's date.
const START_NS: u64 = 1_787_286_600_000_000_000 + 300 * 1_000_000_000;

/// How far below the touch we quote, and how far above entry we exit.
const ENTRY_OFFSET_TICKS: i64 = 3;
const EXIT_OFFSET_TICKS: i64 = 5;
const SIZE_LOTS: i64 = 1;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    /// Waiting for START_NS.
    Waiting,
    /// Entry order is resting; we follow the market with modify().
    Quoting,
    /// Entry filled; place the exit on the next book update.
    ArmExit,
    /// Exit order is resting.
    Exiting,
    /// Done for the day.
    Finished,
}

pub struct PassiveFollower {
    instrument: Option<InstrumentId>,
    state: State,
    entry_id: Option<u64>,
    exit_id: Option<u64>,
    /// Where our entry order is currently resting, so we only modify when it moved.
    quoted_at: Option<Price>,
    /// The price we actually got filled at.
    entry_price: Option<Price>,
}

impl PassiveFollower {
    pub fn new() -> Self {
        PassiveFollower {
            instrument: None,
            state: State::Waiting,
            entry_id: None,
            exit_id: None,
            quoted_at: None,
            entry_price: None,
        }
    }

    /// This instrument's tick size in wire units, or 0 if refdata has no entry.
    fn tick(ctx: &Ctx, instrument: InstrumentId) -> i64 {
        ctx.refdata().get(instrument).map(|i| i.tick_size.0).filter(|t| *t > 0).unwrap_or(0)
    }

    /// This instrument's max single order size, in lots.
    fn max_lots(ctx: &Ctx, instrument: InstrumentId) -> i64 {
        ctx.refdata().get(instrument).map(|i| i.max_single_order_qty).filter(|q| *q > 0).unwrap_or(1)
    }
}

impl Strategy for PassiveFollower {
    // ---------------------------------------------------------------
    // on_start: the ONLY place you can subscribe. No Ctx, no orders.
    // ---------------------------------------------------------------
    fn on_start(&mut self, ctx: &mut StartCtx) {
        for name in UNDERLYINGS {
            match ctx.resolve(name) {
                Some(id) => {
                    ctx.subscribe(id, Depth::Bbo);
                    self.instrument = Some(id);
                    tracing::info!("{}", logging::line(
                        "PassiveFollower", None, "SUBSCRIBE",
                        &format!("{name} -- front-month token id={}, depth=Bbo", id.0)));
                }
                None => {
                    tracing::info!("{}", logging::line(
                        "PassiveFollower", None, "SUBSCRIBE",
                        &format!("{name} -- NOT resolved in this day's refdata")));
                }
            }
        }
        tracing::info!("{}", logging::line(
            "PassiveFollower", None, "START", "armed -- first action at 10:05 IST"));
    }

    // ---------------------------------------------------------------
    // on_book: our decision point. Orders ARE allowed here.
    // ---------------------------------------------------------------
    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, _seq: u64, _packet_ts: u64) {
        // We may be woken for instruments we don't care about.
        if self.instrument != Some(instrument) {
            return;
        }
        if self.state == State::Finished {
            return;
        }

        let now_ns = ctx.now();
        if now_ns < START_NS {
            return;
        }

        macro_rules! log {
            ($tag:expr, $($arg:tt)*) => {{
                tracing::info!("{}", logging::line("PassiveFollower", Some(now_ns), $tag, &format!($($arg)*)))
            }};
        }

        // Never trade off a book we don't trust.
        let Some(book) = ctx.book(instrument) else { return };
        if book.state() != BookState::Ok {
            return;
        }
        let (Some(bid), Some(ask)) = (book.best_bid(), book.best_ask()) else { return };

        let tick = Self::tick(ctx, instrument);
        if tick <= 0 {
            return;
        }

        // Respect the exchange's max single order size.
        let lots = SIZE_LOTS.min(Self::max_lots(ctx, instrument));
        if lots <= 0 {
            return;
        }

        match self.state {
            State::Waiting => {
                // Passive bid, ENTRY_OFFSET_TICKS below the touch. Derived from
                // bid.price, which is already on the tick grid, so this is too.
                let px = Price(bid.price.0 - ENTRY_OFFSET_TICKS * tick);
                match ctx.submit(instrument, Side::Buy, OrderType::LimitDay(px), Lots(lots)) {
                    Ok(id) => {
                        log!("ENTRY", "submit BUY {lots} lot(s) LimitDay @ Rs {:.2} (bid Rs {:.2} / ask Rs {:.2})",
                             px.0 as f64 / RUPEE_RAW, bid.price.0 as f64 / RUPEE_RAW, ask.price.0 as f64 / RUPEE_RAW);
                        self.entry_id = Some(id);
                        self.quoted_at = Some(px);
                        self.state = State::Quoting;
                    }
                    Err(e) => log!("ENTRY_ERR", "submit refused: {e:?}"),
                }
            }

            State::Quoting => {
                // Follow the market: if the touch moved, re-price our resting order.
                let want = Price(bid.price.0 - ENTRY_OFFSET_TICKS * tick);
                let Some(id) = self.entry_id else { return };

                // Only modify an order that is genuinely still working.
                let working = ctx.order(id).map(|o| o.leaves_qty.0 > 0).unwrap_or(false);
                if !working {
                    return;
                }
                if self.quoted_at == Some(want) {
                    return; // nothing changed
                }

                // NOTE: modify() takes Qty (RAW units), not Lots.
                match ctx.modify(id, Qty(lots * RAW_QTY_PER_LOT), Some(want)) {
                    Ok(()) => {
                        log!("FOLLOW", "modify client_order_id={id} -> Rs {:.2} (was Rs {:.2})",
                             want.0 as f64 / RUPEE_RAW,
                             self.quoted_at.map(|p| p.0 as f64 / RUPEE_RAW).unwrap_or(0.0));
                        self.quoted_at = Some(want);
                    }
                    Err(e) => log!("FOLLOW_ERR", "modify refused: {e:?}"),
                }
            }

            State::ArmExit => {
                let Some(entry_px) = self.entry_price else { return };
                let px = Price(entry_px.0 + EXIT_OFFSET_TICKS * tick);
                match ctx.submit(instrument, Side::Sell, OrderType::LimitDay(px), Lots(lots)) {
                    Ok(id) => {
                        log!("EXIT", "submit SELL {lots} lot(s) LimitDay @ Rs {:.2} (entry was Rs {:.2})",
                             px.0 as f64 / RUPEE_RAW, entry_px.0 as f64 / RUPEE_RAW);
                        self.exit_id = Some(id);
                        self.state = State::Exiting;
                    }
                    Err(e) => log!("EXIT_ERR", "submit refused: {e:?}"),
                }
            }

            State::Exiting | State::Finished => {}
        }
    }

    // ---------------------------------------------------------------
    // on_fill: NOTIFICATION ONLY. No orders allowed here.
    // ---------------------------------------------------------------
    fn on_fill(&mut self, ctx: &mut Ctx, fill: &FillRecord) {
        let now_ns = ctx.now();
        tracing::info!("{}", logging::line(
            "PassiveFollower", Some(now_ns), "FILL",
            &format!("client_order_id={} {:?} {} lot(s) @ Rs {:.2} kind={:?} cost=Rs {:.2}",
                     fill.client_order_id,
                     fill.side,
                     fill.qty.0 / RAW_QTY_PER_LOT,
                     fill.price.0 as f64 / RUPEE_RAW,
                     fill.kind,
                     fill.cost.total_rupees)));

        if Some(fill.client_order_id) == self.entry_id && self.state == State::Quoting {
            self.entry_price = Some(fill.price);
            // Just flip the flag -- on_book will actually place the exit.
            self.state = State::ArmExit;
        } else if Some(fill.client_order_id) == self.exit_id {
            self.state = State::Finished;
        }
    }

    // ---------------------------------------------------------------
    // on_order_update: NOTIFICATION ONLY. No orders allowed here.
    // ---------------------------------------------------------------
    fn on_order_update(&mut self, ctx: &mut Ctx, update: &OrderEventRecord) {
        tracing::info!("{}", logging::line(
            "PassiveFollower", Some(ctx.now()), "ORDER_UPDATE",
            &format!("client_order_id={} state={:?} -- {}",
                     update.client_order_id, update.resulting_state, update.description)));
    }

    // ---------------------------------------------------------------
    // on_stop: the run is over. ctx.now() is 0 here. No orders allowed.
    // ---------------------------------------------------------------
    fn on_stop(&mut self, ctx: &mut Ctx) {
        let pnl = ctx.pnl();
        let pos = self.instrument.map(|i| ctx.position(i)).unwrap_or(0);
        tracing::info!("{}", logging::line(
            "PassiveFollower", None, "STOP",
            &format!("final state={:?} position={pos} lots gross=Rs {:.2} net=Rs {:.2}",
                     self.state, pnl.gross, pnl.net)));
    }
}
```

And its companion `qtrade/src/strategy/passive_follower/passive_follower.md`:

```markdown
# passive_follower

Quotes one passive lot three ticks below the NATURALGAS touch from 10:05 IST,
follows the market with `ctx.modify()`, and exits five ticks above the entry
once filled.

## What it watches
- Front-month NATURALGAS future, `Depth::Bbo`.

## What it does
Waiting -> Quoting -> ArmExit -> Exiting -> Finished.

## What it deliberately does not do
- No stop loss.
- No end-of-day flattening (impossible today: orders are not allowed in `on_stop`).
- Single instrument, single lot, no sizing logic.
```

---

## 12. Wiring your strategy into `main.rs`

qtrade compiles **one** strategy at a time. There is no runtime strategy
loader and no config key that selects a strategy — which one is active is a
source-code edit in `qtrade/src/main.rs`.

There are **four** places to change, plus one optional fifth. Search `main.rs`
for the name of the strategy currently wired in (`order_lifecycle_demo` at the
time of writing) and you will find them all.

### 12.1 The module declaration (near the top, ~line 68)

```rust
#[path = "strategy/order_lifecycle_demo/order_lifecycle_demo.rs"]
mod order_lifecycle_demo;
```

becomes

```rust
#[path = "strategy/passive_follower/passive_follower.rs"]
mod passive_follower;
```

The `#[path]` attribute is what lets the file live in a subfolder without a
`mod.rs`.

### 12.2 The `use` line (~line 84)

```rust
use order_lifecycle_demo::OrderLifecycleDemo;
```

becomes

```rust
use passive_follower::PassiveFollower;
```

### 12.3 The `UNDERLYINGS` references (~lines 327 and 330)

This is the one people miss.

```rust
let resolved: Vec<(&str, Option<InstrumentId>)> = order_lifecycle_demo::UNDERLYINGS
    .iter()
    .map(|name| (*name, feed_replay::resolve_front_month(&master, name)))
    .collect();
...
eprintln!("none of {:?} resolved to a real front-month future in this day's refdata",
          order_lifecycle_demo::UNDERLYINGS);
```

Both `order_lifecycle_demo::UNDERLYINGS` become `passive_follower::UNDERLYINGS`.

**This is why your strategy must export `pub const UNDERLYINGS: &[&str]`.**
`main.rs` reads it *before* your strategy is even constructed, in order to:

1. resolve each name to that day's front-month token,
2. build the instrument filter so the decoder only decodes those tokens
   (this is a large performance win — a full day's feed carries thousands of
   instruments you do not care about),
3. load the `Instrument` records the execution engine needs for tick-size and
   max-order-qty validation,
4. build the `name -> id` table that backs your `ctx.resolve()`.

If a name is not in `UNDERLYINGS`, `ctx.resolve()` will return `None` for it,
no matter what you subscribe to.

### 12.4 The construction site (~line 476)

```rust
let strategy = OrderLifecycleDemo::new();
```

becomes

```rust
let strategy = PassiveFollower::new();
```

Everything after this line is generic — `main.rs` wraps it in
`Rc<RefCell<_>>` and registers the same instance with both dispatchers
(`EventDispatcher` calls `on_book`/`on_trade`; `ControlDispatcher` calls
`on_fill`/`on_order_update`).

### 12.5 Optional: the end-of-run summary hook (~line 616)

`main.rs`'s final stdout summary calls a method that is **not** part of the
`Strategy` trait:

```rust
println!("round trips: {}", strategy.borrow().round_trips().len());
for (i, (name, entry_raw, exit_raw, reason)) in strategy.borrow().round_trips().iter().enumerate() {
    ...
}
```

`round_trips()` is `order_lifecycle_demo`'s own instrumentation, not something
every strategy has. If your strategy does not implement it, **delete or comment
out this block**, or the code will not compile.

If you want your own end-of-run summary, the portable way is to write it from
`on_stop()` into `events.log` (see §11), which needs no `main.rs` change at all.

### 12.6 Checklist

```
[ ] created qtrade/src/strategy/<name>/<name>.rs
[ ] created qtrade/src/strategy/<name>/<name>.md
[ ] exported `pub const UNDERLYINGS: &[&str]`
[ ] exported `pub fn new() -> Self`
[ ] implemented `Strategy for <Struct>` with at least `on_start`
[ ] main.rs: #[path] mod declaration
[ ] main.rs: use line
[ ] main.rs: both UNDERLYINGS references
[ ] main.rs: construction site
[ ] main.rs: removed/replaced the round_trips() summary block
[ ] cargo build --release
```

---

## 13. The config file — every key explained

A run is described by a TOML file. By convention these live in
`qtrade/configs/`, named after the strategy and the capture date.

### 13.1 A complete, real config

`qtrade/configs/order_lifecycle_demo_21_08_2026.toml`:

```toml
[run]
mode = "backtest"
session_id = 1
recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin"
report_dir = "logs/qtrade"
order_outbound_latency_ns = 250000
order_inbound_latency_ns = 250000
max_feed_delta_ns = 250000000
log_level = "debug"

[deployment]
```

### 13.2 Required keys

These four must be present or the run aborts.

| Key | Type | Meaning |
|---|---|---|
| `mode` | string | **Must be `"backtest"`.** Any other value exits with an error — no live feed source exists in this codebase yet. |
| `session_id` | integer | Identifies this trading session. It is baked into the high bits of every `client_order_id` (`client_order_id = (session_id << 40) | counter`), so two runs with different `session_id`s can never produce colliding order ids. |
| `report_dir` | string | Parent directory for output. A **new timestamped subfolder** is created under it for every run. Relative paths are relative to your working directory. |
| `recording_path` **or** `recording_paths` | string / array of strings | The capture file(s) to replay. Provide **exactly one of the two.** |

### 13.3 `recording_path` vs `recording_paths`

```toml
# one file - the common case
recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin"
```

```toml
# several files from the SAME DAY, k-way merged in timestamp order
recording_paths = [
  "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin",
  "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_2_4.bin",
]
```

Use `recording_paths` when your strategy's instruments live on different MCX
stream files on the same day. The files are merged in timestamp order, so the
replay stays correct across streams.

**The day's reference data is derived from the capture filename automatically.**
The first path is parsed for its date and the matching `MCXScrips.bcp` contract
master is loaded from it. You never pass a date separately and there is nothing
to keep in sync — point at a different day's file and everything (front-month
resolution, tick sizes, max order sizes, price bands) follows.

### 13.4 Optional keys and their defaults

| Key | Type | Default | Meaning |
|---|---|---|---|
| `max_outer_records` | integer | `0` | Stop after this many records from the capture file. `0` means no limit (full day). **Set this to something small like `2000000` for your first runs** — a full day takes minutes and hundreds of millions of messages. |
| `order_outbound_latency_ns` | integer | `0` | Nanoseconds from your `submit()` call to the venue seeing it. |
| `order_inbound_latency_ns` | integer | `0` | Nanoseconds from the venue's reply to you seeing it. |
| `max_feed_delta_ns` | integer | `250000000` | Sanity bound on the gap between the exchange clock and the capture clock. A packet whose two timestamps differ by more than this is treated as suspect. 250 ms by default. |
| `log_level` | string | `"normal"` | `"normal"` or `"debug"`. Anything else is an error. `"debug"` adds per-component dispatch tracing to `events.log`. |
| `max_feed_stdout_lines` | integer | `200` | Cap on feed lines printed to stdout. Only meaningful for observer strategies like `limit_order_book_generator` that print a book feed. |

### 13.5 The latency settings — use them

The defaults are **zero**, which is not realistic. A zero-latency backtest lets
you react to a book update and be filled before anyone else could have moved,
which flatters every strategy that reacts to fast information.

`250000` ns each way (250 µs out, 250 µs back = **500 µs round trip**) is the
value used in the reference configs and is a defensible starting point for MCX
co-located order entry. Set both:

```toml
order_outbound_latency_ns = 250000
order_inbound_latency_ns  = 250000
```

You will see this directly in `orders.log`: the gap between your `Submitted`
line and the venue's `Accepted`/`Rejected` line is exactly the round trip.

### 13.6 `[deployment]`

The section is parsed and must be present, but has no required keys today.
Leave it empty:

```toml
[deployment]
```

### 13.7 A recommended config for your first run

```toml
[run]
mode = "backtest"
session_id = 1
recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin"
report_dir = "logs/qtrade"

# stop after ~2M records so the run finishes in seconds while you iterate
max_outer_records = 2000000

# realistic 500us round trip
order_outbound_latency_ns = 250000
order_inbound_latency_ns  = 250000

max_feed_delta_ns = 250000000
log_level = "debug"

[deployment]
```

> **The recording drive is read-only.** Never write, move, or delete anything
> under `/mnt/`. `report_dir` must point somewhere inside your own workspace.

---

## 14. Terminal commands — running your strategy

### 14.1 Build

```bash
cd /home/vaibhav/QTrade/qtrade
cargo build --release
```

### 14.2 Run

```bash
./target/release/qtrade configs/passive_follower_21_08_2026.toml
```

The single argument is the path to your config file.

Or, in one step:

```bash
cargo run --release -- configs/passive_follower_21_08_2026.toml
```

(The `--` separates cargo's own arguments from your program's.)

### 14.3 What you will see on stdout

The shape of it (line formats are real; the specific numbers below are
illustrative — yours depend on your capture file and strategy):

```
run output folder: logs/qtrade/20260903_143122

refdata: 17094 instruments loaded, filter admits 1 native ids, 1 of them resolved for order entry
NATURALGAS -> InstrumentId(561496) ...

... replay progress ...

--- summary ---
outer records processed: 12874331
events (decoded messages) processed: 466349510
elapsed: 184.22s (69884 records/s, 2531661 messages/s)
final NATURALGAS (as seen by cache): bid=263.20 x 12 ask=263.40 x 8 state=Ok

--- report (Tier 1) ---
=== qtrade run report (Tier 1) ===
...

logs written:
  logs/qtrade/20260903_143122/events.log  (component-level event trail, level=Debug)
  logs/qtrade/20260903_143122/orders.log  (11 order events)
  logs/qtrade/20260903_143122/fills.log  (2 fills)
  logs/qtrade/20260903_143122/report.txt
```

### 14.4 Useful follow-ups

```bash
# jump to the newest run
cd logs/qtrade/$(ls -t logs/qtrade | head -1)

# the headline numbers
cat report.txt

# your order's whole life
grep 1099511627778 orders.log

# only your strategy's own log lines, in order
grep '\[PassiveFollower\]' events.log

# everything that happened around one nanosecond timestamp
grep -n 1787286944257 events.log
```

### 14.5 Running the test suite

If you changed anything outside your own strategy folder:

```bash
cargo test --release
```

### 14.6 Typical iteration loop

```bash
cd /home/vaibhav/QTrade/qtrade
cargo build --release \
  && ./target/release/qtrade configs/passive_follower_21_08_2026.toml \
  && cat "logs/qtrade/$(ls -t logs/qtrade | head -1)/report.txt"
```

With `max_outer_records` set small, this cycle is a few seconds.

---

## 15. Reports — the four output files and every field in them

Every run creates **one new timestamped folder** under `report_dir`:

```
logs/qtrade/20260903_143122/
├── events.log    the full interleaved event trail (biggest, most useful for debugging)
├── orders.log    every order-state transition
├── fills.log     every fill
└── report.txt    the Tier 1 summary — the headline numbers
```

The folder name is the run's start time in IST (`YYYYMMDD_HHMMSS`). Runs never
overwrite each other, so you can compare two runs side by side.

> If your strategy never submits an order, `orders.log`, `fills.log` and the
> body of `report.txt` are legitimately empty. That is not a bug — the pure
> observer strategy produces exactly that.

### 15.1 `events.log` — the interleaved event trail

Every component writes here, in strict timestamp order, with the component
name in brackets. This is where you find out *why* something happened. Format
and content were covered in §10.

Written live during the run. Size scales with how much you log — see the
warning in §10.

### 15.2 `orders.log` — every order-state transition

```
# order report -- every order-state transition this run produced
t=1787286968100472974 client_order_id=1099511627778  state=Submitted       submit: gates passed, forwarding to venue
t=1787286968100972974 client_order_id=1099511627778  state=Accepted        resting
t=1787286989833718452 client_order_id=1099511627778  state=PendingUpdate   modify requested
```

| Field | Meaning |
|---|---|
| `t=` | Timestamp in nanoseconds since epoch. |
| `client_order_id=` | The id `ctx.submit()` returned. Grep on this to see one order's entire life. |
| `state=` | The `OrderState` after this transition — see §9. |
| trailing text | The human-readable reason (`OrderEventRecord::description`). |

One line per transition, in time order across all orders. To follow one order,
`grep` its id.

### 15.3 `fills.log` — every fill

```
# fills / trade report -- every real fill this run produced
fill_id=0  client_order_id=1099511627779  instrument=InstrumentId(561496) side=Buy   price=Rs 263.30  qty=42.0  kind=Aggressive queue_pos_at_fill=--  cost=Rs 1272.3096
fill_id=1  client_order_id=1099511627780  instrument=InstrumentId(561496) side=Sell  price=Rs 263.20  qty=1.0   kind=Aggressive queue_pos_at_fill=--  cost=Rs 56.6105
```

| Field | Meaning |
|---|---|
| `fill_id=` | Sequential, unique within the run. |
| `client_order_id=` | Which of your orders this fill belongs to. |
| `instrument=` | The MCX token. |
| `side=` | Your side. `Buy` means you bought. |
| `price=Rs` | Fill price, already converted to rupees for you. |
| `qty=` | Filled quantity **in lots**, one decimal place (`42.0` = 42 lots). |
| `kind=` | `Passive` (you were resting) or `Aggressive` (you crossed). |
| `queue_pos_at_fill=` | How much quantity was ahead of you in the queue when you filled. `--` for aggressive fills, which never queued. |
| `cost=Rs` | Total cost charged for **this fill alone** — see §15.4. |

**Markouts** are recorded on each `FillRecord` in memory (`fill.markouts`) but
are summarised in `report.txt` rather than printed per-fill here. A markout is
"how had the price moved N nanoseconds after my fill?" — a direct measure of
adverse selection. qtrade measures two horizons by default: **1 ms and 5 ms**
after the fill.

### 15.4 The `Cost` breakdown — what those numbers actually are

`Cost` is the same struct you get from `ctx.cost()` pre-trade and from
`fill.cost` after the fact.

```rust
pub struct Cost {
    pub exchange_txn_charge: f64,  // MCX's own transaction charge
    pub sebi_fee: f64,             // SEBI turnover fee
    pub ctt: f64,                  // Commodities Transaction Tax (SELL side only)
    pub stamp_duty: f64,           // state stamp duty (BUY side only)
    pub gst: f64,                  // GST on brokerage + exchange charges
    pub brokerage: f64,            // your broker's commission
    pub total_rupees: f64,         // the sum of the above
}
```

**These are taxes and fees, not slippage.** They are large on MCX and they are
charged on **notional turnover**, not on profit. In the sample above, one
42-lot NATURALGAS buy at Rs 263.30 cost **Rs 1,272.31** — because 42 lots of
NATURALGAS is a notional of several crore rupees, and CTT/stamp duty/exchange
charges are all basis points of that notional.

This is why `report.txt` can show `gross_pnl=-125` alongside
`net_pnl=-1453.92`: the trade itself lost Rs 125, and costs took the other
Rs 1,328.92. **A strategy that ignores costs will look profitable in `gross`
and lose money in reality.** Always read `net_pnl`, and use `ctx.cost()` before
you trade to check the edge is bigger than the friction.

Note the asymmetry: **CTT is charged on sells only, stamp duty on buys only.**
A round trip pays both. Brokerage is a flat per-lot amount; everything else is
a rate applied to notional turnover, and GST is applied on top of brokerage
plus exchange charges.

> **⚠ The rates are placeholders, not verified against a live MCX rate
> circular.** They live in `CostConfig::default()` in
> `qtrade/src/execution/execution.rs` and are described there as
> *"representative-of-MCX rates -- not verified"*. The **mechanism** is real
> and correct — config-driven, direction-asymmetric, and the same model serves
> both `ctx.cost()` and realised accounting — but the specific numbers are
> not yet audited. Treat `net_pnl` as a realistic *shape* of cost drag, not
> as a settlement-accurate figure, until someone reconciles
> `CostConfig::default()` against a current circular. Current defaults:
>
> | Component | Default |
> |---|---|
> | `exchange_txn_rate` | 0.0000002 of notional |
> | `sebi_fee_rate` | 0.0000001 of notional |
> | `ctt_rate` | 0.0001 of notional (**sell side only**) |
> | `stamp_duty_rate` | 0.00002 of notional (**buy side only**) |
> | `gst_rate` | 0.18, applied to brokerage + exchange charges |
> | `brokerage_per_lot` | Rs 20.00 flat per lot |

### 15.5 `report.txt` — the Tier 1 summary

A real one, end to end:

```
=== qtrade run report (Tier 1) ===
run identity: config_hash=0xb1bad96c0e2d4886 build_hash=phase1-execution-v0
--- firm level ---
gross_pnl=-125.0000 net_pnl=-1453.9202 realized=-125.0000 unrealized=0.0000 total_cost=1328.9202
inventory: InstrumentId(561496)=41
--- per-strategy ---
strategy=1 gross_pnl=-125.0000 net_pnl=-1453.9202 realized=-125.0000 unrealized=0.0000 cost=1328.9202
--- OTR ---
local: admitted=4 rejected=0 | venue: admitted=5 rejected=0
--- messages ---
new_order_attempts=5 denied=1 submitted_to_venue=4 cancel_requests=2 modify_requests=1 market_events_applied=466349510
--- terminal state counts ---
denied=1 rejected=1 filled=1 canceled=2 expired=0
--- markout ---
horizon_ns=1000000 observations=0 mean_raw_price_units=0.0000
horizon_ns=5000000 observations=0 mean_raw_price_units=0.0000
```

Section by section:

**`run identity`**

| Field | Meaning |
|---|---|
| `config_hash` | A hash of the config that produced this run. Two runs with the same hash used identical settings. Use it to prove you are comparing like with like. |
| `build_hash` | Identifies the qtrade build. |

**`--- firm level ---`** — totals across everything (today, one strategy, so
this equals the per-strategy line).

| Field | Meaning |
|---|---|
| `gross_pnl` | Rupees, before costs. `realized + unrealized`. |
| `net_pnl` | Rupees, after costs. **This is the number that matters.** `gross_pnl - total_cost`. |
| `realized` | P&L on positions you have closed. |
| `unrealized` | Mark-to-market on what you are still holding at the end of the run. |
| `total_cost` | Every tax, fee and commission charged, summed. See §15.4. |
| `inventory:` | Your open position per instrument, **in lots**, at the end of the run. `InstrumentId(561496)=41` means you finished 41 lots long. A non-zero inventory means you did not flatten. |

**`--- per-strategy ---`** — one line per strategy, same fields.

**`--- OTR ---`** — order-to-trade ratio accounting, in two places.

| Field | Meaning |
|---|---|
| `local: admitted / rejected` | How many messages our own pre-trade OTR governor let through vs blocked. A rejection here shows up as `DenyReason::LocalOtrOrRate`. |
| `venue: admitted / rejected` | The same at the simulated venue. A rejection here is `RejectReason::OtrOrRateExceeded`. |

Exchanges penalise members who send many orders per trade. Non-zero rejections
mean you are messaging too aggressively and would be penalised live.

**`--- messages ---`** — raw traffic counts.

| Field | Meaning |
|---|---|
| `new_order_attempts` | How many times you called `ctx.submit()`. |
| `denied` | How many never left the building (local gates). |
| `submitted_to_venue` | How many actually reached the venue. `attempts - denied`. |
| `cancel_requests` | How many times you called `ctx.cancel()`. |
| `modify_requests` | How many times you called `ctx.modify()`. |
| `market_events_applied` | How many decoded market messages the simulator processed. This is the size of the day, not something you control — 466 million in this run. |

**`--- terminal state counts ---`** — where your orders ended up.

| Field | Meaning |
|---|---|
| `denied` | Blocked by our own gates. Should be 0 in a well-behaved strategy; a non-zero count means you are computing bad prices or sizes. |
| `rejected` | Refused by the venue. |
| `filled` | Fully filled. |
| `canceled` | Cancelled — by you, or as an IOC remainder. |
| `expired` | Expired. Always 0 today (no expiry mechanism is wired up). |

These are **terminal** counts, so they will not sum to `new_order_attempts` if
some orders were still resting when the run ended.

**`--- markout ---`** — adverse selection.

| Field | Meaning |
|---|---|
| `horizon_ns` | The measurement horizon: `1000000` = 1 ms, `5000000` = 5 ms after each fill. |
| `observations` | How many fills had enough subsequent data to measure. `0` means no fill had a later reference price — common in short runs. |
| `mean_raw_price_units` | Average price move after your fills, **in wire units** (divide by 100,000,000 for rupees), signed in your favour. |

A **negative** markout means the price consistently moved against you right
after you traded — you are being picked off. A **positive** markout means you
were on the right side of the very next move. For a passive strategy, markout
is the single most diagnostic number in this report.

---

## 16. Rules, limits, and gotchas

A checklist of everything that will bite you. Most of these are stated
elsewhere in this document; they are collected here so you can scan them.

### 16.1 Orders are only allowed from `on_book` and `on_trade`

From `on_fill`, `on_order_update`, or `on_stop` you get
`Err(CtxError::SubmitNotAllowedHere)` and nothing happens. Set a flag, act on
the next book update. See §6.4.

### 16.2 `submit` takes `Lots`, `modify` takes `Qty`

```rust
ctx.submit(inst, side, order_type, Lots(3))?;                    // 3 lots
ctx.modify(id, Qty(3 * crate::types::RAW_QTY_PER_LOT), None)?;   // also 3 lots
```

`Qty(3)` is **not** three lots. It is 3/10000ths of a lot. See §7.

### 16.3 Prices must be exact multiples of `tick_size`

Otherwise the order is denied with `DenyReason::TickSize` before it leaves.
Derive prices from book prices (already on-grid), never from floats. See §7.3.

### 16.4 Order latency is real

With `order_outbound_latency_ns` / `order_inbound_latency_ns` set, the venue
does not see your order the instant you call `submit()`, and you do not learn
the outcome the instant the venue decides. In between, **the market keeps
moving.** An order you sent against a price you saw may arrive to find that
price gone.

`ctx.submit()` still returns `Ok(id)` immediately — that is your local id being
issued, not an acceptance. The real outcome arrives later, via
`on_order_update`.

### 16.5 The book you see is deliberately stale

`ctx.book()` runs on the capture clock; the simulated exchange's book runs on
the exchange clock, which is ahead. You cannot see the future, by design. See
§0.

### 16.6 `max_single_order_qty` is small and real

For NATURALGAS it was 48 lots on a recent day, not a large round number. It is
a per-instrument, per-day constant from MCX's contract file and does not change
during the session. Read it with
`ctx.refdata().get(id).unwrap().max_single_order_qty` and slice bigger orders
yourself. See §5.3.

### 16.7 Always check `BookState` before trading

`Uninit` and `Recovering` books are not trustworthy. Guard every decision:

```rust
if book.state() != BookState::Ok { return; }
```

### 16.8 Handle empty book sides

`best_bid()` and `best_ask()` return `Option`. Early in the session, or in a
thin contract, one or both can be `None`.

### 16.9 One order can produce many fills

Accumulate. Never assume one `on_fill` completes an order. Check
`ctx.order(id).leaves_qty` to know what is still working.

### 16.10 `on_book` is a hot path

It fires hundreds of millions of times in a full day. Return early. Do not log
unconditionally. Do not allocate per call.

### 16.11 `depth(n)` has an ambiguous split

It returns up to `n` bids then up to `n` asks in one flat `Vec` with no marker
between them, and either side may be shorter than `n`. Use
`best_bid()`/`best_ask()`/`qty_at_price()` when correctness matters. See §5.1.

### 16.12 `ctx.now()` returns 0 in `on_stop`

There is no current event once the run is over.

### 16.12a `packet_transact_time_ns` is not the exchange's TransactTime

Despite the parameter name in `on_book`/`on_trade`, it carries the **recorder**
timestamp and is identical to `ctx.now()`. For a trade's real exchange send
time, use `trade.event_time`. See §3.2.

### 16.13 You cannot flatten in `on_stop`

Orders are not allowed there. If you want to be flat at end of day, place the
closing order from an `on_book` callback while data is still flowing.

### 16.14 Never write to `/mnt/`

The recording drive is read-only. `report_dir` must be inside your own
workspace.

### 16.15 `Ok(id)` from `submit` is not acceptance

It means an id was issued. The order may be denied a moment later. `submit`
returns `Err` only for the `can_submit` violation in §16.1.

### 16.16 Gross P&L is not net P&L

Costs on MCX are large and charged on notional. Read `net_pnl`. See §15.4.

### 16.17 Your strategy must export `UNDERLYINGS` and `new()`

`main.rs` needs both. `UNDERLYINGS` is read before your struct exists. See
§12.3.

---

## 17. What does *not* exist yet

Stated plainly, so you do not build on sand.

### 17.1 Four trait methods are dead

`on_warmup_complete`, `on_timer`, `on_session_change`, and
`on_book_state_change` compile, but **nothing in qtrade calls them.** There is
no timer facility, no warmup lifecycle, no session-phase tracking, and no
book-state-change notification. Code you put in them never runs.

If you need time-based behaviour, check `ctx.now()` inside `on_book` — that is
what every existing strategy does.

### 17.2 Only `mode = "backtest"` works

There is no live feed source and no real exchange gateway in this codebase.
Any other `mode` value exits with an error at startup.

The architecture is deliberately built so the *same compiled strategy* will
run live when those two edges exist — you would not rewrite your strategy —
but that day has not arrived.

### 17.3 One strategy at a time

No runtime strategy loader, no multi-strategy config. Switching strategies is
the source edit in §12.

### 17.4 No firm-level view from a strategy

`ctx.position()` and `ctx.pnl()` are always *your* strategy's own. There is no
`ctx.firm_position()` / `ctx.firm_pnl()`.

### 17.5 Only four order types

`LimitDay`, `BookOrCancel`, `Ioc`, `MarketToLimit`. No stop orders, no
icebergs, no GTC/GTD, no auction orders.

### 17.6 No order expiry

`OrderState::Expired` exists in the enum and will always be `0` in
`report.txt`. Nothing wires up GTD or end-of-day expiry.

### 17.7 The simulator does not model your market impact

Your aggressive fills consume liquidity from the replayed book for **your**
accounting, but they do not change what the rest of the market subsequently
does. The recording is a fixed script; nobody in it reacts to you. This is a
documented approximation and it is fine for modest size, but a strategy that
would genuinely have moved the market will look better in backtest than it
would live.

### 17.8 The cost model's rates are placeholders

The cost *mechanism* is real and is the same code path for pre-trade
`ctx.cost()` and realised accounting. The *rates* in `CostConfig::default()`
are labelled in the source as representative but unverified. See §15.4.

### 17.9 `venue_order_id` is informational

It is exposed on `Order` and printed in `events.log`, but you never need it —
every API call takes the `client_order_id`.

---

## Appendix A — one-page cheat sheet

### The imports you will need

```rust
use crate::book::Book;
use crate::decoder::Trade;
use crate::event_dispatcher::Depth;
use crate::execution::{FillRecord, OrderEventRecord};
use crate::logging;
use crate::simulator::OrderType;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{BookState, InstrumentId, Lots, Price, Qty, Side, RAW_QTY_PER_LOT};
```

### The trait

| Method | Called? | Orders allowed? |
|---|---|---|
| `on_start(&mut self, ctx: &mut StartCtx)` | once, before data | no |
| `on_book(&mut self, ctx, instrument, seq, packet_ts)` | constantly | **yes** |
| `on_trade(&mut self, ctx, instrument, &Trade, seq, packet_ts)` | on every tape print | **yes** |
| `on_fill(&mut self, ctx, &FillRecord)` | on each of your fills | no |
| `on_order_update(&mut self, ctx, &OrderEventRecord)` | on each state change | no |
| `on_stop(&mut self, ctx)` | once, at the end | no |
| `on_warmup_complete` / `on_timer` / `on_session_change` / `on_book_state_change` | **never** | — |

### `StartCtx`

```rust
ctx.resolve("NATURALGAS")                  // -> Option<InstrumentId>  (this day's front month)
ctx.subscribe(id, Depth::Bbo)              // or Depth::Top(5)
```

### `Ctx` reads

```rust
ctx.book(id)                               // -> Option<&dyn Book>
ctx.refdata().get(id)                      // -> Option<&Instrument>  (tick_size, max_single_order_qty, ...)
ctx.now()                                  // -> u64 ns since epoch (0 in on_stop)
ctx.order(client_order_id)                 // -> Option<&Order>
ctx.position(id)                           // -> i64 LOTS, signed
ctx.pnl()                                  // -> Pnl { gross, net } in rupees
ctx.cost(id, Lots(n), price, side)         // -> Option<Cost>
```

### `Book`

```rust
book.best_bid()                            // -> Option<PriceLevel { price, qty, order_count }>
book.best_ask()
book.depth(n)                              // -> Vec<PriceLevel>: n bids then n asks, ambiguous split
book.qty_at_price(side, price)             // -> Qty
book.state()                               // -> BookState { Uninit, Recovering, Ok, Stale }
```

### `Ctx` writes (only from `on_book` / `on_trade`)

```rust
let id = ctx.submit(inst, Side::Buy, OrderType::LimitDay(px), Lots(1))?;
ctx.cancel(id)?;
ctx.modify(id, Qty(2 * RAW_QTY_PER_LOT), Some(new_px))?;   // Qty is RAW, not lots
```

### Order types

```rust
OrderType::LimitDay(px)       // normal resting limit; fills what it can, rests the rest
OrderType::BookOrCancel(px)   // post-only; venue REJECTS it if it would cross
OrderType::Ioc(px)            // fills now, cancels the remainder instantly
OrderType::MarketToLimit      // sweeps, rests the residual at the last traded price
```

### Units

```rust
WIRE_PRICE_PER_RUPEE = 100_000_000     // rupees  = price.0 / 100_000_000
RAW_QTY_PER_LOT      =      10_000     // lots    = qty.0   /      10_000
```

### The defensive preamble every callback should start with

```rust
if self.instrument != Some(instrument) { return; }
let now_ns = ctx.now();
let Some(book) = ctx.book(instrument) else { return };
if book.state() != BookState::Ok { return; }
let (Some(bid), Some(ask)) = (book.best_bid(), book.best_ask()) else { return };
let tick = ctx.refdata().get(instrument).map(|i| i.tick_size.0).unwrap_or(0);
if tick <= 0 { return; }
```

### Wiring into `main.rs`

```rust
#[path = "strategy/<name>/<name>.rs"]  mod <name>;     // ~line 68
use <name>::<Struct>;                                  // ~line 84
<name>::UNDERLYINGS                                    // ~lines 327, 330 (twice)
let strategy = <Struct>::new();                        // ~line 476
// and remove main.rs's round_trips() summary block    // ~line 616
```

### Run

```bash
cd qtrade && cargo build --release
./target/release/qtrade configs/<your_config>.toml
cat "logs/qtrade/$(ls -t logs/qtrade | head -1)/report.txt"
```

### Output files

```
logs/qtrade/<YYYYMMDD_HHMMSS>/
├── events.log    everything, interleaved, timestamp-ordered
├── orders.log    every order-state transition
├── fills.log     every fill
└── report.txt    P&L, inventory, OTR, message counts, terminal states, markouts
```

---

## Where to go next

- **`qtrade/src/strategy/order_lifecycle_demo/`** — a scripted walk through
  every order state. The best reference for "what does this callback actually
  receive".
- **`qtrade/src/strategy/multi_instrument_bracket/`** — a real two-instrument
  strategy with resting orders, `modify()` and `cancel()`.
- **`qtrade/src/strategy/limit_order_book_generator/`** — the simplest possible
  strategy: subscribes and observes, submits nothing.
- **`qtrade/src/strategy/strategy.rs`** — the trait, `Ctx` and `StartCtx`
  themselves, with extensive comments explaining *why* each decision was made.
- **`qtrade/src/strategy/README.md`** — the folder convention and its history.
- **`qtrade/src/execution/execution_user_doc.md`** — the order lifecycle in
  depth.
- **`ARCHITECTURE.md`** and **`ARCHITECTURE-DECISIONS.md`** — the system design
  and the numbered decisions behind it.

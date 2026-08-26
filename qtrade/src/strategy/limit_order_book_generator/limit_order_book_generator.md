# `limit_order_book_generator` — a pure observer strategy

**Folder:** `qtrade/src/strategy/limit_order_book_generator/` → `limit_order_book_generator.rs` + this file
**Depends on:** `types`, `event_dispatcher` (`MarketHandler`), `control_dispatcher` (`ControlHandler`, all-default), `strategy` (`Ctx`/`StartCtx`) — nothing else. No `simulator`, no `decoder`, no `refdata`, no `feed_replay`: this strategy never submits an order and has no idea what a capture file, a token, or a bcp path is.
**Renamed from `dummy_strategy` (2026-08-25).** `dummy_strategy` was never meant to be a permanent name — it described what the very first strategy did ("very basic operations": fire a handful of test IOC orders to prove the order path worked). Each real strategy plugged into `main.rs` gets its own name and its own folder going forward; this is the first one named for what it actually does.
**`on_start`/`on_book` (2026-08-25):** this strategy now implements `event_dispatcher::MarketHandler` for real — see below.

---

## What it is

One strategy, one job: print a C++-style limit order book feed (`feed.csv`) for whichever underlyings it's given. **Submits no orders at all.**

- **`pub const UNDERLYINGS: &[&str] = &["NATURALGAS"]`** — the strategy declares *names*, never tokens. `on_start` resolves each via `ctx.resolve(name)` to that day's real front-month token before subscribing (a token is only meaningful for one specific day, FR-16). `main.rs` also resolves the same names independently, before any strategy code runs, to build its own `InstrumentFilter`/`Cache`/`ExecutionEngine` — two legitimate, separate uses of one constant, not a contradiction of "the strategy owns its own subscriptions."
- **`pub const DEPTH_LEVELS: usize = 5`** — how many price levels per side this strategy wants to see, and therefore how deep `on_start` subscribes it (D25: depth of interest is the strategy's own declaration). Exported, not just a private constant, so this file's own subscription and its own row-printing can't silently drift out of sync.
- **`on_start(ctx: &mut StartCtx)`** — D33: `Strategy -> subscribe() -> Control Dispatcher -> Data Engine`. Loops over `UNDERLYINGS`, resolves each, and calls `ctx.subscribe(id, Depth::Top(DEPTH_LEVELS as u8))`. This is the strategy's own declared choice now — `main.rs` no longer decides it.
- **`on_book(ctx: &mut Ctx, instrument, seq, packet_transact_time_ns)`** — called by `event_dispatcher` once per real book change on a subscribed instrument (what used to be called `on_wake`). Reads `ctx.book(instrument).depth(5)`, diffs it against the last row written for that instrument, and appends one CSV row if either side actually changed. The row's instrument-name column comes from `ctx.refdata().get(instrument)`'s `InstrumentKind::Future { underlying, .. }` — not threaded in by hand. `on_trade` is left at its trait default (empty): this strategy only cares about book state.

## Why this is a strategy's own choice, not orchestrator tooling

Until 2026-08-25, this logic (then called `FeedLogger`) lived in `main.rs` itself, unconditionally — every strategy plugged into the orchestrator got `feed.csv` whether it wanted one or not. That's backwards: some strategies will want this view, some won't, and a future trading strategy shouldn't be forced to produce a diagnostic file it never asked for just because it happens to run inside the same orchestrator. Moving the logic here makes it what it should always have been — *this* strategy's explicit choice, exercised through the same read-only `Ctx` access every strategy already has (subscription only governs waking, not access — D25). Run this strategy, get `feed.csv`; run a different one instead, don't.

**Consequence, and it's correct, not a bug:** `orders.log`/`fills.log`/`report.txt` come out empty (`gross_pnl=0.0000`, `filled=0`) whenever this strategy runs, because it never calls `ExecutionEngine::submit_order`. `main.rs` still constructs the full `Cache`/`ExecutionEngine` stack regardless — the orchestrator doesn't know or care whether the plugged-in strategy trades, and correctly produces an honest, empty report when it doesn't.

## The `feed.csv` format

```
timestamp_ns,seq,instrument,side,bid0_price,bid0_qty,ask0_price,ask0_qty,bid1_price,bid1_qty,ask1_price,ask1_qty,bid2_price,bid2_qty,ask2_price,ask2_qty
1768793400147127106,2990,CRUDEOIL,ASK,5356.00,1.0,5474.00,1.0,,,,,,,
```

- **`timestamp_ns`** — the real exchange feed-handler send time (`PacketHeader.TransactTime`), tracked by `main.rs` as it streams `PacketHeader` messages and passed into `on_book`. Not a per-message timestamp: those exist on the wire but are unsafe to trust directly (some resting orders carry a sentinel; `Trade`'s own `event_time` field is actually the matched order's `priority_ts`, not a timestamp).
- **`side`** — `BID`, `ASK`, or `BOTH`: which side's top-`DEPTH_LEVELS` actually changed since the last row for this instrument.
- **`bidN_price`/`bidN_qty`/`askN_price`/`askN_qty`** — up to `DEPTH_LEVELS` levels per side, best first, blank if that level has no resting orders.

Real, verified byte-for-byte behavior: renaming this strategy and moving its logic out of `main.rs` produced an identical `feed.csv` to the pre-rename run (`diff` clean, 697,270 rows, `19_01_2026`) — the move changed *where* the code lives, not what it computes. Re-verified again on the `event_dispatcher`/`on_start`/`on_book` rewrite (2026-08-25): the full `19_08_2026` NATURALGAS run's `feed.csv` is byte-identical (`md5sum` match, 991,128 rows) to the pre-rewrite run — routing changed, output didn't.

## What this strategy deliberately does not do

- No trading logic of any kind — no order submission, no position, no P&L. That's a different strategy's job when one gets written.
- No file paths, no token resolution, no CLI, no report-directory decisions — all of that is `main.rs`'s job (see `../../main_user_doc.md`).
- No `[[bin]]` target and no `main()` of its own — a plain module `main.rs` includes and drives.
- No real `on_fill`/`on_order_update` reactions — `impl ControlHandler for LimitOrderBookGenerator {}` exists (all-default), proving the fill/order-update delivery plumbing compiles and runs end to end, but this strategy submits no orders so has nothing to actually receive there.

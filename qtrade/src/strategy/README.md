# `strategy/` — one subfolder per strategy

Each real strategy plugged into `main.rs` gets its own subfolder here, named for what it does, holding exactly the same two-file shape every other component in this project uses:

```
qtrade/src/strategy/<name>/
├── <name>.rs   the code
└── <name>.md   what it does, how it wires in, what it deliberately doesn't do
```

Today:
- [`limit_order_book_generator/`](limit_order_book_generator/limit_order_book_generator.md) — a pure observer, prints a C++-style limit order book feed, submits no orders.
- [`naturalgas_bracket/`](naturalgas_bracket/naturalgas_bracket.md) — the first real order-placing strategy: a time-triggered bracket trade on NATURALGAS.

**Only one strategy is compiled into `main.rs` at a time.** There's no runtime-loadable strategy set or multi-strategy config yet (D08); which strategy is active is a source-code edit to `main.rs`'s own `#[path]`/`use`/call-site lines, not something chosen at runtime. Swapping strategies means pointing `main.rs` at a different subfolder here.

**`strategy::Strategy` is real, as of 2026-08-26** — one real trait, matching `STRATEGY-GUIDE.md` §2's own design exactly: *"a struct holding your own state, implementing the `Strategy` trait."* Write a strategy by writing `impl Strategy for YourStruct { ... }`, overriding whichever of its 10 methods you actually use — only `on_start` has no default. `EventDispatcher`/`ControlDispatcher` remain two separate components underneath (D33's real argument — different lookup, different cardinality, different delivery guarantee — is about the dispatch *mechanism*, not the callback interface); each just calls its own subset of `Strategy`'s methods (`EventDispatcher`: `on_start`/`on_book`/`on_trade`; `ControlDispatcher`: `on_fill`/`on_order_update`) on the same `Rc<RefCell<dyn Strategy>>` handle. (This was first built as two separate traits, `MarketHandler`/`ControlHandler`, one per dispatcher — merged back into one the same session once it became clear the dispatcher split didn't actually require a trait split too. See `strategy.rs`'s own header for the full reasoning.)

Five of the ten methods are honestly unbacked placeholders today — `on_warmup_complete`/`on_timer`/`on_session_change`/`on_book_state_change` have no real machinery anywhere in this codebase yet (no warmup lifecycle, no scheduler wiring, no session-state tracking). `on_stop` is the exception: real and wired, `main.rs` calls it once, right after the replay loop ends.

Both `Ctx`/`StartCtx` (`strategy.rs`, sitting directly in this folder — shared infrastructure every strategy uses, not a strategy of its own) are handed to `Strategy`'s methods. `LimitOrderBookGenerator` implements `on_start`/`on_book` for real, leaving the other 8 at their defaults, since it submits no orders; `NaturalGasBracket` implements 5 for real (`on_start`/`on_book`/`on_trade`/`on_fill`/`on_order_update`).

**`ctx.submit()`/`cancel()`/`modify()`/`order()`/`position()`/`pnl()`/`cost()` are real (Phase C)** — a strategy can both cause and receive execution activity. Reads work from any callback; writes only from `on_book`/`on_trade` (calling one from `on_fill`/`on_order_update` returns `Err`, deliberately, rather than silently doing nothing — see `strategy.rs`'s own header).

`dummy_strategy` (this project's very first strategy, since renamed) is the reason this convention exists: it started as one file doing everything — CLI, feed reading, instrumentation, and trading — and every one of those concerns not actually about strategy *decisions* has since moved out to where it belongs (`feed_replay/`, `main.rs`). What's left in a strategy's own folder should only ever be: what it watches, and what it does when woken.

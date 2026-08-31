# Event Dispatcher — component documentation

**What this component does, in one sentence:** routes market data outward to a strategy's own callbacks — `on_book` when a subscribed slice of a book actually changes, `on_trade` on every real `Trade` message — at high rate, direct function calls, no allocation on the hot path (D07/D33).

Code: [`event_dispatcher.rs`](event_dispatcher.rs) (this folder). Built 2026-08-25 by relocating and generalizing what used to be `cache::Dispatcher` — see `cache/cache_user_doc.md` §4 for that predecessor's own history and measured numbers, still true of the logic this component inherited unchanged.

---

## 1. Why this exists as its own component, not part of `Cache`

D33 ("two dispatchers, because they are two different lookups") settles this: market-data dispatch was never really `Cache`'s job, it was just built inside `cache.rs` first because there was nowhere else for it to live at the time. The keying (`(instrument, depth)`, a pre-sized lookup, no allocation, no dynamic topic registry) and the workload (fires on every message) are genuinely different from what a control/command dispatcher needs — see `control_dispatcher/control_dispatcher_user_doc.md` for that comparison in full. `Cache` no longer knows this component exists at all; `main.rs` owns both as siblings and drives them explicitly, per event.

## 2. What it builds

```rust
pub enum Depth { Bbo, Top(u8) }
pub type SubscriberId = usize;

pub struct EventDispatcher { /* subs_by_key, by_instrument, handlers, snapshots, stats */ }
impl EventDispatcher {
    pub fn new() -> Self;
    pub fn register(&mut self, handler: Rc<RefCell<dyn strategy::Strategy>>) -> SubscriberId;
    pub fn subscribe(&mut self, id: SubscriberId, instrument: InstrumentId, depth: Depth);
    pub fn on_book_touched(&mut self, book: &dyn Book, instrument: InstrumentId, cache: &Cache, engine: &mut ExecutionEngine, seq: u64, recorder_ts: u64, handles: RunHandles) -> ExecOutcome;
    pub fn on_trade(&mut self, cache: &Cache, engine: &mut ExecutionEngine, instrument: InstrumentId, trade: &Trade, seq: u64, recorder_ts: u64, handles: RunHandles) -> ExecOutcome;
}
```

**`packet_transact_time_ns` renamed to `recorder_ts`, and `handles: RunHandles` added (2026-08-27, dual-clock pass).** The renamed parameter is exactly the same value as before in spirit — the timestamp a strategy's `Ctx::now()` reports for this callback — but is now genuinely the recorder's own real capture timestamp, not the pre-dual-clock single clock. `handles` is the new `strategy::RunHandles<'a> { venue: &'a SimExchange, scheduler: &'a mut Scheduler, pending_ops: &'a mut HashMap<u64, PendingVenueOp>, latency_ns: u64 }` bundle — the capability `Ctx::submit`/`cancel`/`modify` need to schedule their venue-reaching half as a real event rather than call it in the same instant (see `execution_user_doc.md` §2 and `main_user_doc.md` §3 item 6). Since `RunHandles` isn't `Copy`, both methods reborrow it fresh at each handler-loop iteration (`RunHandles { venue: handles.venue, scheduler: &mut *handles.scheduler, pending_ops: &mut *handles.pending_ops, latency_ns: handles.latency_ns }`) rather than moving it once.

**Calls `strategy::Strategy`'s `on_book`/`on_trade` (2026-08-26, revised).** Originally called a `MarketHandler` trait defined in this file (Phase A); reconsidered the same session and merged into one real `Strategy` trait in `strategy.rs`, matching `STRATEGY-GUIDE.md` §2's own design instead of splitting it further than D33 actually requires. This component itself didn't change at all -- same registry, same keying, same snapshot-diffing, same reason it's a separate component from `control_dispatcher` (D33's real argument is about the dispatch *mechanism*, not the callback *interface*). Only the type its registry holds changed, from `Rc<RefCell<dyn MarketHandler>>` to `Rc<RefCell<dyn Strategy>>` -- this module still only ever calls `.on_book()`/`.on_trade()` on it, never `.on_fill()`/`.on_order_update()`/anything else `Strategy` also declares.

`register`/`subscribe` are two separate steps (the original `cache::Dispatcher::subscribe` combined them) because they now happen at different times: `register` once, at `main.rs`'s construction time; `subscribe` later, inside the strategy's own `on_start` (D33 — subscribing is the strategy's declared choice, not the orchestrator's).

**Phase C (2026-08-25, later the same day):** `on_book_touched`/`on_trade` gained `engine: &mut ExecutionEngine` and now return `ExecOutcome` — `ctx.submit()`/`cancel()`/`modify()` are real from `on_book`/`on_trade` now (see `strategy/strategy.rs`), and whatever they produce is accumulated inside the `Ctx` used for that call, drained (`Ctx::take_outcome`), and merged across every handler into the value this method returns.

**Since the 2026-08-27 dual-clock pass, that return value is no longer forwarded straight to `ControlDispatcher::dispatch`.** A `ctx.submit()` call inside `on_book`/`on_trade` only ever produces the *local gate* half of an `ExecOutcome` now (`Denied`, or "gates passed" — see `execution_user_doc.md` §2); the venue's own response arrives later, via a scheduled `OrderArrival`/`ReportDelivery` pair `main.rs`'s loop drives independently. `main.rs` still owns the "what happens with this return value" decision either way — this method never talks to `ControlDispatcher` directly, same independence D07 asks of the two dispatchers — it's just that "what happens" is now "stash it, maybe schedule a report," not "dispatch it immediately."

## 3. Two callbacks, two different mechanisms

**`on_book`** — unchanged logic from `cache::Dispatcher::on_book_touched`: compares the current top-of-book (or top-`n`) against the last observed snapshot for that `(instrument, depth)` key, and calls `on_book` only for the depths that actually changed value. An order added ten price levels deep touches the book but never fires a BBO subscriber's `on_book` — proven by `on_book_wakes_on_bbo_change_not_on_deeper_level`.

**`on_trade`** — deliberately bypasses all of that. A `Trade` message's own fields (price/qty/side) are a fact to report once, not a book-state comparison — there is no "did it change" question, since there's nothing to diff against. `on_trade` reuses `by_instrument` (does this instrument have *any* registered interest, at any depth — `Depth` is meaningless for a trade) and calls every matching handler unconditionally, once per real message. Proven by `on_trade_fires_unconditionally_regardless_of_book_state_change` (fires twice for two identical trade messages, where `on_book`'s diffing would have fired at most once) and `on_trade_does_not_fire_for_an_unsubscribed_instrument`.

**Who decides "this message was a `Trade`"?** `main.rs`, not this component. `Cache::apply` only ever returns `Option<InstrumentId>` (a generic "this instrument's book was touched"), with no message-kind information — teaching it to preserve and forward that would be a `Cache` change for a fact only `event_dispatcher` needs. `main.rs`'s own `dispatch_event` function (its `MarketData{target: Target::Cache, ...}` arm, since the 2026-08-27 dual-clock pass — see `main_user_doc.md` §3 item 6) holds the raw, undecoded-further `DecodedMessage` carried on that scheduled event before it hands anything to `Cache`; it pattern-matches `DecodedMessage::Trade` itself and calls `EventDispatcher::on_trade` directly, independent of whatever `on_book_touched` decides for the same event. This is left as an explicitly open design question from this session — flagged here for whoever revisits it, not settled as permanent.

## 4. `Rc<RefCell<dyn Strategy>>`, not `Box`

One strategy instance must be reachable from *every* `(instrument, depth)` key it subscribes to — a strategy watching two underlyings still has one `on_book` implementation, one set of `self` fields (see `LimitOrderBookGenerator`'s own `last_seen` map, keyed by instrument, not duplicated per subscription). `Box<dyn Strategy>` (the original `cache::Dispatcher`'s storage shape, one owner per subscriber id) can't express that without requiring the strategy to be `Clone` — which would either duplicate real state (wrong) or be impossible (a strategy holding a `File`, as `LimitOrderBookGenerator` does, isn't `Clone` at all). `Rc<RefCell<_>>` lets `main.rs` keep a concrete handle for its own use (e.g. `NaturalGasBracket::round_trips()` in the final summary) while a cloned, coerced reference lives inside this dispatcher's registry — same underlying strategy, two ways to reach it.

## 5. Scope notes

- Kept the full multi-subscriber shape (`Vec<Rc<RefCell<dyn Strategy>>>`, `SubscriberId`-keyed lookups) even though only one strategy is ever compiled into `main.rs` today — this is relocated, already-tested code, not new speculative generality; removing it now would mean rebuilding it later when a second strategy actually exists, at strictly higher total cost.
- `ctx.submit()`/`cancel()`/`modify()`/`order()`/`position()`/`pnl()`/`cost()` are real now (Phase C, `strategy/strategy.rs`) — but only from `on_book`/`on_trade`; calling a write method from `on_fill`/`on_order_update` fails loudly (`Err(CtxError::SubmitNotAllowedHere)`), a deliberate choice to avoid building recursive delivery nothing needs yet.
- `on_fill`/`on_order_update` are not this component's job at all — see `control_dispatcher/control_dispatcher_user_doc.md`. This module only ever calls the `on_start`/`on_book`/`on_trade` third of `Strategy`'s 10 methods, even though the type it holds carries the other 7 too.
- Confirmed: nothing under `/mnt/` or `references/` was touched building this.

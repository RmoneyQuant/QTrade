# Control Dispatcher — component documentation

**What this component does, in one sentence:** D33's "commands inward, reports outward, low rate" dispatcher, now doing both real jobs it names — forwarding a strategy's `on_start`-time `subscribe()` into `event_dispatcher::EventDispatcher`, and delivering real fills/order-updates to a strategy's `on_fill`/`on_order_update` the moment `ExecutionEngine` produces them.

Code: [`control_dispatcher.rs`](control_dispatcher.rs) (this folder). Subscribing built 2026-08-25 (Phase A); delivery built the same day (Phase B), once `execution::ExecutionEngine`'s own methods were changed to hand back what they produced; a strategy's own `ctx.submit()`/`cancel()`/`modify()` (called from `on_book`/`on_trade`, delivered back here) built later the same day (Phase C, `strategy/strategy.rs`). Calls `strategy::Strategy` directly since 2026-08-26 (was a separate `ControlHandler` trait defined in this file — merged into `Strategy`, see `strategy.rs`'s own header for why).

---

## 1. Why two passes, same file

D07/D33 name two dispatchers, and this one two jobs: routing subscriptions (a command, inward) and delivering fills/order-updates (reports, outward). Phase A built only the first — subscribing needed no new data from `execution.rs` at all, while delivery needed `ExecutionEngine` to stop only accumulating fills/order-events privately and start handing back what each call actually produced. Building that (`ExecOutcome`, see `execution.rs`) was the real prerequisite Phase B needed; once it existed, `dispatch` here was the small remaining piece.

## 2. What it builds

```rust
pub struct ControlDispatcher { /* one registered handler, or none */ }
impl ControlDispatcher {
    pub fn new() -> Self;
    pub fn register(&mut self, handler: Rc<RefCell<dyn strategy::Strategy>>);
    pub fn subscribe(&mut self, event_dispatcher: &mut EventDispatcher, id: SubscriberId, instrument: InstrumentId, depth: Depth);
    pub fn dispatch(&mut self, cache: &Cache, engine: &mut ExecutionEngine, outcome: &execution::ExecOutcome);
}
```

`dispatch` takes `&mut ExecutionEngine` (Phase C) purely so it can build the same `Ctx` type `EventDispatcher` does — `on_fill`/`on_order_update` can read `ctx.order()`/`position()`/`pnl()`/`cost()` like any other callback. `Ctx` here is always built with `can_submit: false`; a write attempt fails at the strategy's own call site (`strategy/strategy.rs`'s `CtxError::SubmitNotAllowedHere`), so `dispatch` itself never needs to notice or forward anything from a failed one.

`dispatch` calls `strategy::Strategy::on_fill` for every fill in `outcome.fills`, then `on_order_update` for every event in `outcome.order_events`, in that order, against whichever handler is registered — a no-op if none is (or if `outcome` is empty, the common case today: see §3). This module only ever calls those two of `Strategy`'s 10 methods — never `on_book`/`on_trade`/anything else, even though the registered type carries them too (same independence `event_dispatcher_user_doc.md` §5 notes in the other direction).

## 3. "One destination," still, and why it's not empty in practice yet

Same reasoning as Phase A's `subscribe`: only one strategy is ever compiled into `main.rs`, so `handler: Option<Rc<RefCell<dyn Strategy>>>` — not a `strategy_id`-keyed registry — is the honest shape today. Registering a second handler silently replaces the first; there's no real multi-strategy config (D08) yet for anything more careful to mean.

**Real fact, `19_08_2026`:** `NaturalGasBracket` (the strategy actually compiled in) does produce real fills through this path — see its own doc for the run's real numbers. `LimitOrderBookGenerator` (the pure observer, not currently compiled in) implements `Strategy` with `on_fill`/`on_order_update` left at their trait defaults — it submits no orders, so every `ExecOutcome` a run against it produces is empty, and `dispatch` never actually calls anything through it. Both cases were proven with synthetic data before either ran for real: `a_real_fill_reaches_on_fill_through_control_dispatcher` submits directly via `ExecutionEngine`; `a_strategy_submitted_order_is_filled_and_delivered_through_on_fill` goes further — a synthetic `Strategy` calls `ctx.submit()` from `on_book`, and the resulting fill is confirmed to arrive via `on_fill`, proving the whole path (`ctx.submit` → `EventDispatcher` → `ExecutionEngine`/`SimExchange` → `ExecOutcome` → this `dispatch` → `on_fill`) end to end.

## 4. `ExecOutcome` — how `execution.rs` hands back what it produced

Confirmed, discussed choice (not the only option): `ExecutionEngine`'s own mutating methods (`submit_order`, `on_market_event`, `request_cancel`, `deliver_cancel_to_venue`, `request_modify`, `deliver_modify_to_venue`, `mark_expired`) each return an `ExecOutcome` alongside their original result — a before/after snapshot of `self.fills`/`self.order_events`' lengths around each method's original, unchanged body (renamed to a private `..._inner`). The alternative — `main.rs` diffing `engine.fills()`/`engine.order_events()` itself, with zero changes to `execution.rs` — would have delivered identically to a strategy; this option was chosen anyway, accepting ~50 mechanical call-site updates across `execution.rs`'s own tests and `execution-validate`'s harness, so that `ExecutionEngine` itself is the source of truth for "what did this call just produce," not an external diff. See `execution_user_doc.md` for the full account.

`main.rs`'s replay loop (Phase C: reordered so `engine.on_market_event` runs *before* `on_book`/`on_trade` fire — `strategy/strategy.rs`'s own header explains why):
```rust
let outcome = engine.on_market_event(ev.event, ev.now_ns);
control_dispatcher.dispatch(&cache, &mut engine, &outcome);
// ... then cache.apply / event_dispatcher.on_book_touched / on_trade, each
// forwarding whatever ExecOutcome a strategy's own ctx.submit() produced
// to control_dispatcher.dispatch the same way.
```
`request_cancel`/`deliver_cancel_to_venue`/`request_modify`/`deliver_modify_to_venue`/`mark_expired` are now real, callable paths too — via `ctx.cancel()`/`ctx.modify()` (`strategy/strategy.rs`, Phase C), scoped to `on_book`/`on_trade` only (calling them from `on_fill`/`on_order_update` returns `Err(CtxError::SubmitNotAllowedHere)` rather than silently doing nothing).

## 5. Known, disclosed gaps (not fixed this pass)

- **Coupling to `tier2_enabled`.** `ExecutionEngine::fills`/`order_events` are only populated when `tier2_enabled` is `true` (see `handle_exec_reports`/`on_fill`'s own `if self.tier2_enabled` guards) — live delivery through this dispatcher inherits that same gate, since `ExecOutcome` is computed from those same lists. Not a new dependency, just worth stating: turn off Tier 2 reporting, and a strategy stops receiving `on_fill`/`on_order_update` too.
- **`OrderEventRecord` has no structured `CancelReason`.** `STRATEGY-GUIDE.md`'s illustrative `on_order_update` example matches on `OrderState::Canceled { reason }` — real `OrderState::Canceled` is a plain unit variant; the actual reason lives on `Order.cancel_reason` (a separate field, not reachable via `OrderEventRecord`) or, today, only as free text in `OrderEventRecord.description`. A strategy wanting to distinguish `CancelReason::Mmp` from `CancelReason::Strategy` programmatically can't yet, from `on_order_update` alone.

## 6. Scope notes

- Confirmed: nothing under `/mnt/` or `references/` was touched building this.

# order_lifecycle_demo — strategy component doc

**Not a trading strategy.** It makes no attempt at P&L and deliberately leaves inventory un-flat. Its only job: walk an order through **every `OrderState` qtrade can actually reach**, one scripted action at a time on NATURALGAS, so a single `events.log` shows the whole state machine instead of just the `Submitted → Filled` path a pure-taker (`multi_instrument_bracket`) exercises.

## How to run

Point `main.rs` at this strategy (module + `use` + constructor — `strategy/README.md`), then a normal config:

```toml
[run]
mode = "backtest"
session_id = 1
recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin"
report_dir = "logs/qtrade"
max_outer_records = 40000000     # ~10:08 IST is enough; the script finishes by 10:08
order_outbound_latency_ns = 250000
order_inbound_latency_ns = 250000
log_level = "normal"
```

The 8 scripted steps fire 20 s of sim time apart, starting 10:05 IST (`START_NS`), so the whole script runs 10:05–10:08 IST. Every `on_order_update` / `on_fill` is logged verbatim to `events.log`.

## The script → states it reaches

| Step | Action | States logged |
|---|---|---|
| 1 | `submit` BUY 1100 lots (over the 1000-lot demo freeze qty) | `Denied` (`FREEZE_QTY`, local Validation gate — never reaches the venue) |
| 2 | `submit` `BookOrCancel` BUY at the ask (would cross) | `Submitted` → `Rejected` (`WouldCross`, venue) |
| 3 | `submit` deep passive `LimitDay` BUY (cannot fill) | `Submitted` → `Accepted` (resting) |
| 4 | `modify` that order to a new passive price | `PendingUpdate` → `Accepted` |
| 5 | `cancel` that order | `PendingCancel` → `Canceled` (`Explicit`) |
| 6 | `submit` BUY ~900 lots `LimitDay` at the ask (touch only, under freeze) — fills the best-ask size (~81 lots), rests the rest | `Submitted` → `PartiallyFilled` (executed chunk) → `PartiallyFilled` (remainder working) |
| 7 | `cancel` the working remainder from step 6 | `PendingCancel` → `Canceled` |
| 8 | `submit` SELL 1 lot `MarketToLimit` | `Submitted` → `Filled` (no `Accepted` — matches MCX's `10103 Immediate Execution Response`, one message that is both ack and fill for a marketable order) |

Verified against real `21_08_2026` stream-4 data: report terminal counts `denied=1 rejected=1 filled=1 canceled=2 expired=0` (step 6's order ends terminally `Canceled` via step 7, so it counts as canceled, not filled).

**9 of 11 states reach a strategy:** `Submitted`, `Denied`, `Rejected`, `Accepted`, `PendingUpdate`, `PendingCancel`, `Canceled`, `PartiallyFilled`, `Filled`.

## Not reachable from a strategy today

- **`Initialized`** — internal pre-gate state; it becomes `Submitted` or `Denied` before any `on_order_update` is emitted.
- **`Expired`** — `ExecutionEngine::mark_expired` exists and is unit-tested but has no caller (no GTD / Lean-EOD expiry wired into the replay loop).

(An earlier version of this doc listed `PartiallyFilled` here too — building this demo surfaced that both `log_event` sites hardcoded `Filled` / `Accepted` for a partial. Fixed in `execution.rs` the same day, 2026-09-01: both now pass `order.state`. See `execution_user_doc.md` §1.)

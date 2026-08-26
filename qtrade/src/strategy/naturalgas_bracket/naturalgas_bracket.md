# `naturalgas_bracket` — a real, order-placing strategy

**Folder:** `qtrade/src/strategy/naturalgas_bracket/` → `naturalgas_bracket.rs` + this file
**Depends on:** `types`, `decoder` (`Trade`), `event_dispatcher` (`MarketHandler`, `Depth`), `control_dispatcher` (`ControlHandler`), `execution` (`FillRecord`, `OrderEventRecord`), `simulator` (`OrderType`), `strategy` (`Ctx`, `StartCtx`) — the first strategy in this project to actually use the execution side of that list. Implements `strategy::Strategy` for free (both halves implemented for real).

---

## What it does

Buy 1 lot of NATURALGAS at a fixed time of day, bracket the fill with a stop-loss and a take-profit, and repeat for the rest of the session:

1. **Entry**: the first market event at or after **10:00:00 IST** — buy 1 lot, `OrderType::MarketToLimit`.
2. **Bracket**: once filled, watch the book's best bid every tick. If it falls to **98%** of the fill price, or rises to **102%**, sell 1 lot (`MarketToLimit`) immediately.
3. **Re-entry**: 5 minutes after the exit fill, buy again — repeats for the rest of the session, not just once.
4. **End of day**: at **23:30:00 IST**, force-close (sell) any still-open position.

## Real, verified timestamps — not guesses

`ENTRY_NS`/`EOD_NS` are literal Unix-epoch nanosecond constants, derived and checked against the real `19_08_2026` capture file before being hardcoded:

- `PacketHeader.TransactTime` (`packet_transact_time_ns` at the call site) is confirmed to be a genuine Unix-epoch nanosecond timestamp: a real sample from the file decodes to `2026-08-19 09:00:00.185 IST` — a plausible session-open time, not 1970 or an arbitrary counter.
- `10:00:00 IST, 2026-08-19` → `1_787_113_800_000_000_000` ns.
- `23:30:00 IST, 2026-08-19` → `1_787_162_400_000_000_000` ns — chosen after scanning the real tail of the capture file and finding activity runs to approximately this time.

## Why 98%/102%, not the original 95%/105%

Requested initially as a 95%/105% bracket; tightened before running, since ±5% is unrealistically wide for NATURALGAS's real intraday range and would likely never trigger at all in one session — confirmed by the real band scan this same run performs: the day's full-session circuit band was `[Rs 254.30, Rs 275.30]` around a ~Rs 266.50 open, roughly ±3–4% total. A ±5% bracket would have sat outside the band on the stop-loss side.

## A real result, `19_08_2026`

```
10:00:00.07 IST  BUY  filled @ Rs 266.50
17:25:42.27 IST  TP   SELL filled @ Rs 271.90   -- round trip: +Rs 5.40/unit before costs
17:30:42.64 IST  BUY  filled @ Rs 271.50        -- re-entry, 5 minutes after the exit
23:30:00.15 IST  EOD  SELL REJECTED: NoLiquidityForResidual
```

One completed round trip: gross **Rs 6,750** (NATURALGAS's real contract multiplier is 1,250 — Rs 5.40/unit × 1,250), net **Rs 6,631.42** after real transaction costs (Rs 118.58 across 3 fills). **The second position never closed** — by 23:30 IST the order book had genuinely gone empty (no resting bids or asks at all), so `MarketToLimit`'s own real, tested behavior (`simulator.rs`'s `market_to_limit_with_zero_liquidity_rejects_rather_than_resting_with_no_reference_price`) rejected the force-close outright instead of resting it. That lot is still open, unmarked (nothing calls `Portfolio::mark_to_market` anywhere in this codebase yet), and **not included** in the `gross_pnl`/`net_pnl` figures above.

## What this strategy deliberately does not do

- **No retry on a rejected order.** `on_order_update` is left at its default — a `Denied`/`Rejected` (like the real EOD rejection above) leaves the strategy stuck waiting for a fill that will never come. A real strategy would need to notice and react; this one doesn't yet.
- **No exchange-native stop order.** `simulator::OrderType` has no stop-order variant at all. The bracket is this strategy watching the book and reacting a tick after the price crosses — not a resting order sitting at the venue.
- **No wall-clock timer.** `ctx.set_timer()` doesn't exist anywhere in this codebase. "10:00 IST" and "5 minutes later" are both threshold checks on `packet_transact_time_ns`, evaluated per tick — accurate to the nearest market event, not the nearest second.
- **Fixed 1 lot, no position sizing, no risk budget.** Every entry is exactly 1 lot regardless of cost, volatility, or existing exposure.

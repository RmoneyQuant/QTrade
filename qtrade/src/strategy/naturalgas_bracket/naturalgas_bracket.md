# `naturalgas_bracket` — a real, order-placing strategy

**Folder:** `qtrade/src/strategy/naturalgas_bracket/` → `naturalgas_bracket.rs` + this file
**Depends on:** `types`, `decoder` (`Trade`), `event_dispatcher` (`MarketHandler`, `Depth`), `control_dispatcher` (`ControlHandler`), `execution` (`FillRecord`, `OrderEventRecord`), `simulator` (`OrderType`), `strategy` (`Ctx`, `StartCtx`) — the first strategy in this project to actually use the execution side of that list. Implements `strategy::Strategy` for free (both halves implemented for real).

---

## What it does

Buy 1 lot of NATURALGAS at a fixed time of day, bracket the fill with a stop-loss and a take-profit, and repeat for the rest of the session:

1. **Entry**: the first market event at or after **10:00:00 IST** — buy 1 lot, `OrderType::MarketToLimit`.
2. **Bracket**: once filled, watch the book's best bid every tick. If it falls to **99.5%** of the fill price, or rises to **100.5%**, sell 1 lot (`MarketToLimit`) immediately.
3. **Re-entry**: 5 minutes after the exit fill, buy again — repeats for the rest of the session, not just once.
4. **End of day**: at **23:30:00 IST**, force-close (sell) any still-open position.

## Real, verified timestamps — not guesses, and not the same day as before

`ENTRY_NS`/`EOD_NS` moved from `19_08_2026` to **`21_08_2026`** on 2026-08-27, alongside the dual-clock replay pass (see `../../main_user_doc.md` §4 and `../../feed_replay/feed_replay_user_doc.md` §2a for the full account). **This wasn't a routine date bump** — `19_08_2026` predates the recording rig's cross-server NTP sync (~2026-08-20) and a real negative feed-latency delta (`-135ns`) was found partway through that file's own session, correctly tripping the new Q1 fail-fast. `21_08_2026` is the first verified-clean day (zero negative deltas across a real 60-million-record scan) and is what every real run in this project uses from this point on. **Pick a day at or after `21_08_2026`** for any future work here — this codebase has no way to detect an unsafe day on its own.

- `PacketHeader.TransactTime` is confirmed genuine Unix-epoch nanoseconds on this file too: the first real record decodes to `2026-08-21 08:30:23.67 IST` — a plausible pre-session time.
- `10:00:00 IST, 2026-08-21` → `1_787_286_600_000_000_000` ns.
- `23:30:00 IST, 2026-08-21` → `1_787_335_200_000_000_000` ns.

## Why ±0.5% (995/1005 per-mille), not the original ±2% (98/102)

The bracket started at 95%/105%, was tightened once to 98%/102% (percent-based) against the `19_08_2026` run — and then, on 2026-08-27, tightened **again**, this time to **per-mille**, because the real `21_08_2026` session showed **zero triggers all day at ±2%**: NATURALGAS's real intraday range that day never moved that far from any single entry price. `SL_PER_MILLE = 995` / `TP_PER_MILLE = 1_005` (±0.5%) is the replacement — still a real, chosen band, not tick-jitter noise trading, but tight enough to actually produce visible round trips against this instrument's real intraday behavior. The units changed too (percent → per-mille) purely so the threshold can express a value this fine without a fractional constant.

## Detailed, timestamped logging (2026-08-27)

Every decision this strategy makes is now logged through one shared `log(now_ns, msg)` helper, prefixed `t=<raw ns> (<YYYY-MM-DD HH:MM:SS.mmm IST>)`. Critically, **`now_ns` is always `ctx.now()` — a real physical/simulated timestamp, never wall-clock-at-log-time**: under the dual-clock replay, that's the recorder's own receipt time for whatever tick triggered `on_book`/`on_trade`, or the scheduled `ReportDelivery` time for `on_fill`/`on_order_update`. A strategy's own printed log is exactly as trustworthy a timeline as `orders.log`/`fills.log`, because it's built from the same clock.

What gets logged, end to end:
- **`on_start`**: fires before any event has been scheduled or dispatched, so there is genuinely no sim timestamp yet — logged honestly as `"on_start fired (pre-replay -- no sim timestamp exists yet)"`, not faked. Followed by one `"SUBSCRIBED: {name} -- native/MCX token id={}, depth=Bbo"` line per resolved underlying, naming the real MCX token id `ctx.resolve` returned (or a "did not resolve" line if it didn't).
- **`"ALARM: ..."`** — every threshold crossing that fires a handler: entry time reached, SL/TP crossed (with the real bid, entry, sl, and tp prices all in the line), re-entry cooldown elapsed, EOD reached.
- **`"PLACING ORDER: ..."`** — every `ctx.submit(...)` call, with side, order type, quantity, instrument, and the real `client_order_id` returned.
- **`"ORDER UPDATE: ..."`** — every `on_order_update` callback, printing `client_order_id`, `resulting_state`, and the real description string (this is how the one real EOD rejection below is visible at all).
- **`"FILL RECEIVED: ..."`** — every `on_fill` callback, with the real fill price and (on an exit) the completed round trip's entry→exit prices and per-unit P&L before costs.
- **`"PORTFOLIO updated: ..."`** — called right after every fill, reading `ctx.position`/`ctx.pnl` live (not computed by the strategy itself) so it reports exactly what `Portfolio::apply_fill` already committed inside `ExecutionEngine`.

## A real result, `21_08_2026`

```
config: latency_ns=100,000  SL=99.5%  TP=100.5%

t=1787286605675081299 (2026-08-21 10:00:05.68 IST)  BUY  filled @ Rs 263.50
t=1787293574633226721 (...)                         TP   SELL filled @ Rs 264.90
...(9 more round trips)...
t=1787328594675012821 (...)                         BUY  filled @ Rs 265.00   -- 11th entry, never closed
t=1787335200114838038 (2026-08-21 23:30:00.11 IST)  EOD  SELL REJECTED: NoLiquidityForResidual
```

**10 completed round trips** (5 closed by TP, 5 by SL), all entries and exits landing between Rs 263.50 and Rs 267.50:

| # | Entry | Exit | Reason |
|---|---|---|---|
| 1 | Rs 263.50 | Rs 264.90 | TP |
| 2 | Rs 265.00 | Rs 266.40 | TP |
| 3 | Rs 266.80 | Rs 265.50 | SL |
| 4 | Rs 265.70 | Rs 267.10 | TP |
| 5 | Rs 267.50 | Rs 266.10 | SL |
| 6 | Rs 266.70 | Rs 265.40 | SL |
| 7 | Rs 265.50 | Rs 264.10 | SL |
| 8 | Rs 264.60 | Rs 265.90 | TP |
| 9 | Rs 265.60 | Rs 267.00 | TP |
| 10 | Rs 266.10 | Rs 264.60 | SL |

The 11th entry (buy @ Rs 265.00) never closed — by 23:30 IST the book had genuinely gone empty (no resting bids or asks at all), so `MarketToLimit`'s own real, tested behavior (`simulator.rs`'s `market_to_limit_with_zero_liquidity_rejects_rather_than_resting_with_no_reference_price`) rejected the force-close outright instead of resting it — the same disclosed limitation as the `19_08_2026` run had, now visible directly in the strategy's own log via the new `on_order_update` line rather than only in `orders.log`.

**Real `Tier1Summary` numbers** (`report.txt`, this run): `gross_pnl≈-0.0000`, `net_pnl=-903.1182`, `total_cost=903.1182`, 22 orders attempted (0 denied), 21 filled, 1 rejected (the EOD close), `inventory: 1` lot still open. `gross_pnl` sits almost exactly at zero because a symmetric ±0.5% band nearly cancels out in price terms across 10 round trips — the real, non-zero cost is what the ±0.5% band's own trading frequency (22 orders in one session) actually paid, not a sign the strategy is directionally flat by design.

**Direct proof the scheduled-delivery mechanism (not a synchronous shortcut) produced these fills**: every `Submitted`→`Filled`/`Rejected` pair in this run's `orders.log` is exactly `100,000ns` apart — this run's configured `latency_ns`, applied by the real `OrderArrival`/`ReportDelivery` scheduling described in `../../main_user_doc.md` §3 item 6, not a same-instant illusion.

## What this strategy deliberately does not do

- **No retry on a rejected order.** `on_order_update` logs the real rejection (2026-08-27) but still doesn't act on it — a `Denied`/`Rejected` (like the real EOD rejection above) leaves the strategy stuck waiting for a fill that will never come. A real strategy would need to notice and react; this one only notices, out loud, now.
- **No exchange-native stop order.** `simulator::OrderType` has no stop-order variant at all. The bracket is this strategy watching the book and reacting a tick after the price crosses — not a resting order sitting at the venue.
- **No wall-clock timer.** `ctx.set_timer()` doesn't exist anywhere in this codebase (`scheduler::Scheduler` itself is real and driving order/market events since 2026-08-27, but nothing yet lets a strategy schedule its own wake-up on it — see `strategy/README.md`). "10:00 IST" and "5 minutes later" are both threshold checks on `ctx.now()`, evaluated per tick — accurate to the nearest market event, not the nearest second. Logged as an "ALARM" regardless, since it *behaves* like one from the strategy's point of view even though the mechanism is tick-polling.
- **Fixed 1 lot, no position sizing, no risk budget.** Every entry is exactly 1 lot regardless of cost, volatility, or existing exposure.

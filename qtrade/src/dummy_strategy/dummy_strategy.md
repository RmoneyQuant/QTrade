# `dummy_strategy` — a minimal end-to-end demo, not a real strategy

**Folder:** `qtrade/src/dummy_strategy/` → `dummy_strategy.rs` + this file
**Depends on:** `types`, `decoder`, `refdata`, `book`, `cache`, `simulator`, `execution` — everything already built and independently verified in phase 1 (see `agent_tasks/STATUS.md`)
**Not part of phase 1's task list.** This exists to answer one question concretely: *does the whole engine — reading real data and actually trading on it — work end to end, right now, today* — before any real `Strategy` trait exists.

---

## What it is

The dumbest possible thing that can sit on top of the real engine and prove it's real, in two halves:

1. **Read side**: subscribes to CRUDEOIL and NATURALGAS at best-bid/best-offer via `cache`, and prints every line the moment either instrument's BBO genuinely changes.
2. **Trade side**: every so often, fires one aggressive 1-lot IOC order (alternating buy, then sell) through `execution::ExecutionEngine` against its own independent `simulator::SimExchange` — real order, real fill, real accounting — just so there's something to report.

It places no thought into *when* or *why* to trade beyond "every N wakes, alternate sides." It is not a market maker and was never meant to resemble one — "dummy" is the actual design, not modesty.

## Why build this at all

`book`/`cache`/`simulator`/`execution` were each already validated against full real sessions **in isolation** — that's what their own `*-validate` harnesses are for. But nothing had ever run the *whole* chain — reading real data, deciding, submitting a real order, getting a real fill, producing a real report — from one `main()`. Doing that for the first time is exactly what surfaced the two real integration findings below, neither of which any single component's own test suite could have caught, because each one only shows up when two independently-built, individually-correct components are handed the same real number at the same time.

## The pipeline

```
decoder::decode_messages(bytes)        -- raw capture bytes -> typed DecodedMessage values
        |                    |
        v                    v
cache::Cache::on_message   execution::ExecutionEngine::on_market_event
(filter -> book -> dispatch,   (feeds simulator::SimExchange's OWN
 what the strategy "sees")      independent book -- D10, never shares
        |                       state with `cache`)
        v
DummyStrategy::on_wake -- fires only when the subscribed instrument's BBO moved
        |
main()'s loop: reads cache.book(id) to print + decide, calls
engine.submit_order(...) on the trade-side schedule
```

The one structural wrinkle worth calling out: `Subscriber::on_wake` (in `cache.rs`) deliberately does **not** hand the subscriber a `&Cache` — a real strategy shouldn't be able to reach back into `Cache` mid-dispatch. So `DummyStrategy::on_wake` just records *that* a wake happened; `main`'s own loop does the actual reading, deciding, and order submission.

## How to run it

```bash
cd qtrade
cargo build --release --bin dummy-strategy

# Default: first 20MB of the real CRUDEOIL capture file, up to 200 printed BBO lines
./target/release/dummy-strategy

# Explicit: <capture-file> <bytes-to-read> <max-BBO-lines-to-print>
./target/release/dummy-strategy /mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_5.bin 20000000 15
```

CRUDEOIL lives on capture stream 4, NATURALGAS on stream 5 (confirmed during `book`'s own validation). Reference data always loads from the real `MCXScrips.bcp` for `19_01_2026`. Only a bounded prefix of the file is read (20MB by default) — this is a demo meant to finish in a couple of seconds and produce something readable, not another full-session correctness gate (that's what `book-validate`/`cache-validate`/`simulator-validate` already are).

## Where the logs are

Every run writes three real files to **`qtrade/logs/dummy_strategy/`** (relative to wherever you run the binary from — `qtrade/` if you follow the command above), overwritten each run:

| File | What it is | Source |
|---|---|---|
| `orders.log` | **Order report** — every order-state transition (`Submitted`, `Filled`, ...) with its timestamp and client order id | `engine.order_events()` — `execution::OrderEventRecord` |
| `fills.log` | **Fill / trade report** — every real fill: price, qty, side, fill kind, queue position at fill, cost breakdown | `engine.fills()` — `execution::FillRecord` |
| `report.txt` | **Position / summary report (Tier 1)** — P&L at both firm and per-strategy level, inventory, OTR consumption, message counts, terminal-state counts, markout | `engine.tier1_report()` — `execution::Tier1Summary`, the same struct/`Display` impl `execution-validate`'s own acceptance scenario 4 already proved works |

The same content is also printed to stdout at the end of the run, so you don't need to open the files to see it — but the files are real, on disk, and are what "the log" actually refers to.

## Four real integration findings — all found, all now fixed

All four were found by doing the one thing no prior task did: feeding a **real** `refdata::Instrument` and a **real**, `decoder`-scaled `Price`/`Qty` into the **same** `execution` call. Each component was independently correct on its own terms; wiring them together for the first time is what surfaced these. All four are now fixed at the root — the first two at the type level (new, distinct types introduced in `types.rs` — see `refdata_user_doc.md` §4a and `execution_user_doc.md` §8), the third by carrying the real instrument's `multiplier` into the two P&L formulas that were missing it (`execution_user_doc.md` §11), the fourth by unifying `refdata`'s `InstrumentId` assignment onto the convention every other component already used (`refdata_user_doc.md` §6a) — rather than patched at the one call site that happened to surface each. History kept below, marked fixed, in the same style `book_user_doc.md` uses for its own found-and-fixed bugs.

### 1. FIXED — `refdata`'s tick size wasn't in `decoder`/`book`'s number space

**Originally found:** `refdata::Instrument.tick_size` for CRUDEOIL came straight from `MCXScrips.bcp`'s own `TickSize` column — reported as the raw integer `100`, put directly into `Price(100)`. But `decoder`/`book`'s `Price` is wire-scaled (raw integer ÷ 10^8 = rupees), and `book.rs`'s own `band_config` already empirically confirmed CRUDEOIL's *real* tick is `100_000_000` raw units (= Rs 1.00) — six orders of magnitude off refdata's raw `100`. This demo originally worked around it by overriding `tick_size` to `book.rs`'s own validated value.

**Root cause, found since:** `parts[21]` is denominated in **paise** (confirmed against 24+ real rows across 8 commodities — every one reproduces that commodity's real, publicly documented MCX tick size when read as paise; see `refdata_user_doc.md` §4a). `types::ContractFilePaise` now carries this value and `refdata::load_mcx_instruments` converts via `.to_wire_price()` (paise → rupees `/100`, rupees → wire-raw `×100,000,000`) before populating `Instrument.tick_size` — so `refdata` itself now produces the correct value, and this demo's override is gone entirely (not just relocated).

**Verified, real data, before/after:**

```
before (raw column value put directly into Price, no conversion):
  CRUDEOIL (467013):   tick_size = Price(100)          <- Rs 0.000001, six orders of magnitude wrong
  NATURALGAS (465849): tick_size = Price(10)            <- same bug

after (ContractFilePaise(raw).to_wire_price()):
  CRUDEOIL (467013):   tick_size = Price(100_000_000)   <- Rs 1.00, matches book.rs's band_config exactly
  NATURALGAS (465849): tick_size = Price(10_000_000)    <- Rs 0.10, matches book.rs's band_config exactly
```

`refdata` also still has no source column for freeze quantity at all (T01's own documented scope) and defaults every instrument's `freeze_qty` to `0` — **this part is unchanged, a separate, still-open gap**, not part of the tick-size fix. `execution::validate()` now compares `freeze_qty` against `intent.qty` as `Lots` (see finding 2), so this demo's override is expressed in lots — still a demo-only workaround for a real, undocumented-column gap, not a claim about MCX's real freeze quantity.

### 2. FIXED — `execution`'s cost formula and `simulator`'s order matching disagreed by exactly 10,000× on what `Qty` meant

**Originally found:**

- `simulator::SimExchange` replays **real** `decoder::DecodedMessage`s directly, so its own resting-order quantities are in `decoder`/`book`'s wire-raw scale (raw integer ÷ 10,000 = lots). Confirmed directly: `simulator/validate.rs`'s own real-data hand-trace test submits `NewOrderRequest { ..., qty: Qty(10_000), ... }` to represent **one lot**, against the real CRUDEOIL book.
- `execution::CostModel::round_trip`'s turnover formula is `rupees(price) * qty.0 as f64 * instrument.multiplier as f64`. `execution.rs`'s own unit tests exercised this with small, literal lot counts — e.g. the cost-asymmetry acceptance test uses `qty=10` to mean **ten lots**, not ten wire-raw units.
- `NewOrderIntent.qty` flowed into **both** uses completely unconverted. There was no shared "what does `Qty` mean" contract between the two components — each was internally consistent, and each was tested only against its own convention, never the other's, until this demo combined them.

**Root cause fix:** two new, distinct types in `types.rs` — `Lots` (plain lot count) and the existing wire-raw `Qty`, with explicit `Qty::to_lots()`/`Lots::to_raw_qty()` conversions (`RAW_QTY_PER_LOT = 10_000`). `NewOrderIntent.qty` is now `Lots`, not `Qty`; `execution::submit_order` converts explicitly (`intent.qty.to_raw_qty()`) at the one place `simulator::NewOrderRequest` is built; `CostModel::round_trip` now takes `Lots` in its own signature, so `on_fill` must convert a real fill's wire-raw `Qty` back via `.to_lots()` before calling it. The type change means a future call that gets this backwards fails to compile, not just silently produces a wrong number a second time. Full account: `execution_user_doc.md` §8.

**Verified, real data, before/after** (same run: first 20MB of the real CRUDEOIL capture, 6 real 1-lot IOC fills):

```
before (inflated ~10,000x):
  total_cost = 3,377,044.7040   net_pnl = -3,557,044.7040   (gross_pnl = -180,000.0000, unaffected -- it never involved qty/cost at all)

after (fixed):
  total_cost = 337.7045         net_pnl = -180,337.7045     (gross_pnl = -180,000.0000, unchanged, as expected)

per-fill cost, after the fix (fills.log):
  fill 0  Buy  1 lot @ Rs 5424.00  cost = Rs 34.6302
  fill 1  Sell 1 lot @ Rs 5421.00  cost = Rs 77.9921
  fill 2  Buy  1 lot @ Rs 5422.00  cost = Rs 34.6262
  fill 3  Sell 1 lot @ Rs 5417.00  cost = Rs 77.9520
  fill 4  Buy  1 lot @ Rs 5420.00  cost = Rs 34.6221
  fill 5  Sell 1 lot @ Rs 5410.00  cost = Rs 77.8818
```

`337.7045 / 3,377,044.7040 ≈ 1 / 10,000` — the exact `RAW_QTY_PER_LOT` factor the root-cause analysis predicted. Per-leg costs are now tens to low-hundreds of rupees, a real, sensible magnitude for a 1-lot CRUDEOIL round-trip leg, not a fabricated "looks plausible" number. The inflated-cost disclaimers previously printed on stdout and written into `report.txt`/`fills.log` are gone, because the bug they were disclosing no longer exists.

**Note on `net_pnl` above:** at the time this second finding was fixed, `gross_pnl`/`realized_pnl` (`-180,000.0000`) was still itself wrong — the third finding below — so "`net_pnl` now close to `gross_pnl`" was describing two numbers that were each individually broken in different ways and happened to be close by coincidence, not evidence either was correct. See finding 3 for the real `gross_pnl`/`realized_pnl` and the real `net_pnl` that follows from it.

### 3. FIXED — `Portfolio::apply_fill`/`mark_to_market` used the fill's raw wire quantity directly in P&L, with no `instrument.multiplier` applied at all

**Originally found:** the same real 6-fill CRUDEOIL run above reported `gross_pnl=-180000.0000`/`realized=-180000.0000` in `report.txt`. Hand-computing the real P&L from `fills.log`'s own real fill prices — three round-trip legs, 1 lot each, multiplier 100 (barrels/lot) for real CRUDEOIL — gives a very different number:

| Leg | Bought at | Sold at | Real P&L (price diff × 1 lot × 100 barrels/lot) |
|---|---|---|---|
| 1 | Rs 5424 | Rs 5421 | (5421−5424) × 100 = **−300** |
| 2 | Rs 5422 | Rs 5417 | (5417−5422) × 100 = **−500** |
| 3 | Rs 5420 | Rs 5410 | (5410−5420) × 100 = **−1,000** |
| **Real total** | | | **−1,800** |

`-180,000` is reproducible exactly by hand as `Σ (price_diff_rupees × 10,000)` over the three legs — the fill's raw wire quantity (`10,000` for "1 lot") used directly in place of `lots(1) × multiplier(100)`, in `Portfolio::apply_fill`'s realised-P&L formula and `Portfolio::mark_to_market`'s unrealised one identically. A *different* net error factor from finding 2's cost bug (`10,000/100 = 100x` here, vs. a clean `10,000x` there) because this formula's missing `multiplier` term partially, but not fully, cancels the missing `Qty`→`Lots` conversion for an instrument whose real multiplier happens to be 100, not 1.

**Root cause fix:** `apply_fill` now converts the fill's wire-raw `qty` via `Qty::to_lots()` (the same conversion `on_fill` already made before calling `CostModel::round_trip` for finding 2) and takes the instrument's real `multiplier` as a new parameter, applied exactly where a price difference becomes a rupee P&L: `pnl_per_lot * closing_qty_lots * multiplier`. `mark_to_market` gets the identical fix for unrealised P&L. `SubAccount`/`FirmAccount`'s `position` maps now store **lots**, not wire-raw units, since every real consumer (this P&L math, Tier 1/Tier 2 reporting) wants a lot count — doc-commented explicitly rather than changed to `types::Lots` itself, since that type has no arithmetic operators defined on it and adding them was out of scope for a fix confined to this component. Full account, including why this is a *third*, independent occurrence of the same Lots-vs-`Qty` confusion: `execution_user_doc.md` §11.

**Verified, real data, before/after** (same run, same 6 fills):

```
before (raw wire qty used directly, no multiplier):
  gross_pnl=-180000.0000  net_pnl=-180337.7045  realized=-180000.0000  total_cost=337.7045

after (lots * instrument.multiplier):
  gross_pnl=-1800.0000    net_pnl=-2137.7045    realized=-1800.0000    total_cost=337.7045
```

`total_cost` is unchanged (`337.7045`, finding 2's own already-fixed figure) — this fix touched only the P&L formula, not the cost stack. `-180000 / -1800 = 100`, exactly `RAW_QTY_PER_LOT (10,000) / multiplier (100)`, matching the hand-computation above.

### 4. FIXED — `refdata` and every other component used two different, disagreeing `InstrumentId` numbering schemes

**Originally found:** getting to the fix above required building `trade_instruments` (this file, `main()`) with a line that took each real `refdata::Instrument` and overwrote its `.id` field: `i.id = InstrumentId(i.native_id as u32)`. That line existed because `refdata::load_mcx_instruments` assigned `id` as a dense counter (0, 1, 2... in file order, per FR-B02's original literal wording — "interned, dense") while `book`, `cache`, `simulator`, and `execution` had all independently settled on a *different* convention: the native exchange token, cast directly (see `book.rs`'s `CRUDEOIL_ID = InstrumentId(467_013)`). Two live, disagreeing numbering schemes for "which instrument is this," reconciled only by a one-line manual patch, right here, for exactly the two tokens this demo already knew the answer for by name.

**Root cause:** `book` was built needing to route real decoded messages (which carry the native token, not a dense counter) to the right per-instrument book, and took the cheaper path — use the token directly as `InstrumentId` — rather than depending on `refdata` and translating through a lookup table. Every component built afterward (`cache`, `simulator`, `execution`) matched `book`'s convention to stay compatible with it, so it propagated forward while `refdata` (built earlier, to the original FR-B02 wording) kept the different, older one. Nobody was ever asked to reconcile the two, because every later milestone was scoped narrowly enough that the mismatch stayed invisible until this file combined `refdata`'s output with `book`/`execution`'s expectations directly.

**Fix:** `refdata::load_mcx_instruments` now assigns `id: InstrumentId(token as u32)` directly — the same convention everyone else already used. There is exactly one `InstrumentId` space in qtrade now. The manual `.id` remap line in this file is gone entirely (not relocated) — see the git history around `main()`'s `trade_instruments` construction, or `refdata_user_doc.md` §6a for the full account, including why this required changing `InstrumentMaster::get()` from a direct array index to a hash lookup (the token is large and sparse, not small and dense, so it can no longer index a `Vec` directly) and why that costs nothing in practice (nothing else in qtrade ever used the old dense numbering for array indexing either — `book`/`cache`/`simulator` already store instruments in `HashMap`s).

**Verified, real data, after the fix:** `master.get(InstrumentId(467_013))` returns CRUDEOIL (`tick_size = Price(100_000_000)`); `master.get(InstrumentId(465_849))` returns NATURALGAS (`tick_size = Price(10_000_000)`); a full `dummy-strategy` re-run with the `.id` remap line deleted produces byte-identical output to before (`gross_pnl=-1800.0000`, `total_cost=337.7045`, 6 fills) — proving the fix is transparent, not just non-crashing.

### 5. FIXED (later task) — `book`'s price band is generic now, and this demo's increment-only feed needs an explicit seed for it

**Originally found:** `book`'s price-band mechanism moved from a hardcoded per-id table (`band_config`) to learning each instrument's real band from a real `InstrumentInfo` (13603) message in the applied stream — see `book_user_doc.md`'s "generic price band" section. Wiring this demo through unchanged would have made its very first real CRUDEOIL order panic: `book::BookBuilder::apply` fails loudly on a real order-mutating event for an instrument whose band is still unknown, and checked against real bytes, CRUDEOIL's real `19_01_2026` increment capture (the only file this demo ever reads) never carries a *valid* 13603 during the session at all (its DPR never changed that day, and this capture starts recording after the one-time Start-of-Day broadcast that would have carried it — only the *snapshot* channel, which this demo doesn't read, repeats it every cycle).

**Fix:** `main()` now calls `cache.seed_book_band(book::CRUDEOIL_ID, ...)` / `cache.seed_book_band(book::NATURALGAS_ID, ...)` right after `Cache::new`, with the real, snapshot-verified bands (`book-validate`'s own harness independently learned the same numbers from the paired snapshot file's real 13603 stream — see book_user_doc.md). Not a return of the old hardcoded `band_config` (removed from `book.rs` entirely, and no longer used for any other instrument) — a caller-supplied real value for the one real gap an increment-only feed has.

**Verified:** a full re-run (`./target/release/dummy-strategy`) produces the same real BBO/order/fill output as before this change — see the sample output below.

## Sample real output (CRUDEOIL, default settings)

```
refdata: 140 instruments loaded, filter admits 2 native ids (narrowed to book's validated bands), 2 of them resolved for order entry
streaming the first 20000000 bytes of .../mcx_feeder_Increment_capture_19_01_2026_1_4.bin (a bounded prefix, not a full session...)

[  2989] CRUDEOIL   bid=Rs 5356.00 x 1.0   ask=--                 spread=--
[  2990] CRUDEOIL   bid=Rs 5356.00 x 1.0   ask=Rs 5474.00 x 1.0   spread=Rs 118.00
...
  >> CRUDEOIL order #1: Buy 1.0 lot @ Rs 5424.00 IOC -> Submitted { client_order_id: 1099511627776 }
  >> CRUDEOIL order #2: Sell 1.0 lot @ Rs 5421.00 IOC -> Submitted { client_order_id: 1099511627777 }
  ... (6 orders total, alternating buy/sell)

--- summary ---
events processed: 324730
BBO lines printed: 200 (capped at 200)
final CRUDEOIL (as seen by cache): bid=Rs 5407.00 x 3.0 ask=Rs 5410.00 x 28.0 state=Ok

--- report (Tier 1) ---
=== qtrade run report (Tier 1) ===
run identity: config_hash=0xb1bad96c0e2d4886 build_hash=phase1-execution-v0
--- firm level ---
gross_pnl=-1800.0000 net_pnl=-2137.7045 realized=-1800.0000 unrealized=0.0000 total_cost=337.7045
inventory: InstrumentId(467013)=0
--- terminal state counts ---
denied=0 rejected=0 filled=6 canceled=0 expired=0

logs written:
  logs/dummy_strategy/orders.log  (12 order events)
  logs/dummy_strategy/fills.log  (6 fills)
  logs/dummy_strategy/report.txt
```

Six orders round-trip the position back to flat (`inventory: InstrumentId(467013)=0`) since the buy/sell alternation always ends on a sell after starting on a buy — `inventory` is now printed in **lots** (finding 3), not raw wire units, though a flat `0` reads the same either way; the P&L and cost numbers are real, sensible magnitudes now that all three integration findings above are fixed (`gross_pnl=-1800.0000`, matching the hand-computation in finding 3, not the pre-fix `-180000.0000`; `total_cost=337.7045` for six 1-lot fills, not finding 2's pre-fix `3377044.7040`).

Running against NATURALGAS's own capture file (stream 5) instead exercises the same trade logic against that instrument; CRUDEOIL is left `Uninit` in that run, same as the read-only version of this demo.

## What this deliberately does not do

- No real strategy logic — "every N wakes, alternate buy/sell, 1 lot, IOC" is not a market-making policy, and isn't trying to be.
- No full-session run, no correctness assertions — that's what `book-validate`/`cache-validate`/`simulator-validate` already are.
- No general-purpose instrument filter — hardcoded to the two tokens `book` actually has real validated price bands for.
- Not wired into `main.rs` — a separate `[[bin]]` target (`dummy-strategy`), same convention every other component's own harness already uses.

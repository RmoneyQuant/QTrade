# `types` — shared vocabulary

**Folder:** `qtrade/src/types/` → `types.rs`
**Depends on:** nothing
**Used by:** `refdata` (T01), `book` (T03) directly; `simulator` (T06) transitively

## What this is

The handful of value types more than one component needs, defined once
here instead of each component inventing its own copy. `decoder` still
has its own *private* `Price`/`Qty`/`Side` — that's deliberate and
unchanged by this file; decoder's copies exist to model the raw wire
representation during decode, while these are the clean, strategy-facing
versions. They happen to look similar; they are not the same types and
are not meant to merge into one.

Nothing here has behavior. Every type is a plain data carrier — parsing,
validation, and querying belong to the component that needs them
(`refdata` for `Instrument`, `book` for anything book-shaped).

## Types, one line each

| Type | For | Introduced by |
|---|---|---|
| `Price(i64)` | A price in ticks — never `f64` (STRATEGY-GUIDE.md §11) | needed by everything downstream |
| `Qty(i64)` | An order/level quantity | needed by everything downstream |
| `Side` (`Buy`/`Sell`) | Which side of the book | needed by everything downstream |
| `InstrumentId(u32)` | Interned, dense identity for an instrument (FR-B02) — not the exchange-native token | `refdata` |
| `Venue` (`Mcx`, `#[non_exhaustive]`) | Which exchange; more variants later, not guessed at now | `refdata`, `simulator`'s per-venue `LatencyModel` |
| `Date(i64)` | Days-since-epoch; widen only if a real need appears | `refdata` (`Instrument.kind`'s expiry fields) |
| `YearMonth { year, month }` | A futures contract month | `refdata` (`InstrumentKind::Future`) |
| `Settlement` (`Cash`/`Physical`) | How a contract settles | `refdata` |
| `Right` (`Call`/`Put`) | Option right — stub, `Option` kind isn't implemented yet | `refdata` (D37 taxonomy completeness) |
| `Exercise` (`European`/`American`) | Option exercise style — same stub status as `Right` | `refdata` |
| `Currency` (`Inr`) | Instrument's settlement currency | `refdata` |
| `InstrumentKind` | The D37 taxonomy — only `Future` is really implemented; `Option`/`Equity`/`Spread` are stubs so the enum shape is complete without pretending they work | `refdata` |
| `Instrument` | The full reference-data record for one contract (FR-B01, verbatim shape) | `refdata` |
| `BookState` (`Uninit`/`Recovering`/`Ok`/`Stale`) | A book's lifecycle state (FR-B10) — only `Uninit`→`Ok` is reachable until a live Transport exists | `book` |
| `OrderHandle` | Identifies a resting order by where it sits (instrument, side, price, priority timestamp) — MCX has no broadcast order id (FR-B05) | `book` |
| `PriceLevel` | One level of a book: price, aggregate qty, order count. Used by `book`'s `Book` trait (`best_bid`/`best_ask`/`depth`) — BACKTEST-PHASE1.md's own FR-B08 code uses this shape but never defines it; defined here since nothing else does | `book` |

## Debug vs Display

Every type derives `Debug` (full field dump, `{:?}`) — no exceptions,
same convention `decoder` established. Hand-written `Display` (`{}`)
exists only on the types a person actually reads directly: `Price`,
`Qty`, `Side`, `Venue`. Everything else (`Instrument`, `InstrumentKind`,
`OrderHandle`, `PriceLevel`, ...) is a structured record meant to be
inspected via `Debug`, not printed as a sentence — so no `Display` impl
was written for those, on purpose.

## Scope discipline

Only what `refdata` and `book` actually need today is here. Nothing was
added in anticipation of `scheduler`/`cache`/`simulator`/`execution` —
those components add to this file when they're actually built, and only
for a need that's real at that point, not a guessed one.

## How to verify this compiles standalone

```
cd qtrade
cargo build
```

`main.rs` wires this in as `#[allow(dead_code)] mod types;` — nothing
calls into it yet, and that's expected at this stage; the allow is there
so the build stays clean rather than warning on every unused type until
`refdata`/`book` exist to use them.

# T00 — `types`

**Folder:** `qtrade/src/types/` → `types.rs` + `types_user_doc.md`
**Depends on:** nothing
**Blocks:** `refdata` (T01), `book` (T03) — both need these types before they can compile

---

## What it is

Shared vocabulary — the handful of value types more than one component needs, so they're defined once instead of each component inventing its own. `decoder.rs` currently has *private* copies of `Price`/`Qty`/`Side` (see its top section) — that was fine when it was the only component; it stops being fine the moment a second component needs the same shapes.

**Scope discipline:** only include what `refdata` (T01) and `book` (T03) actually need right now. Do not add fields or types for `scheduler`/`cache`/`simulator`/`execution` in anticipation — those components add to this file when they're actually built, per YAGNI. Adding a type here is cheap; guessing wrong about a future component's needs isn't.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M1 FR-B01 (the `InstrumentKind`/`Instrument` shape, given verbatim) and §M3 FR-B05, FR-B08, FR-B10 (order handle, book trait signature, book state enum)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D37 (instrument taxonomy — implement `Future`, stub the rest) and the "two findings" in §4 (no broadcast order ID on MCX — composite key, not an integer)
- `qtrade/src/decoder/decoder.rs` — read its private `Side`/`Price`/`Qty` definitions (near the top) as the starting point; this task promotes them to shared, it doesn't redesign them

## Build

```rust
pub struct Price(pub i64);   // ticks, never f64 — see STRATEGY-GUIDE.md §11
pub struct Qty(pub i64);
pub enum Side { Buy, Sell }

pub struct InstrumentId(pub u32);      // interned, dense — FR-B02
pub enum Venue { Mcx }                  // #[non_exhaustive] — more venues later, not now

pub enum InstrumentKind {                // D37 — Future implemented, rest are stubs
    Future { underlying: String, expiry: Date, contract_month: YearMonth, settlement: Settlement },
    Option { .. },  Equity { .. },  Spread { leg1: InstrumentId, leg2: InstrumentId },
}

pub struct Instrument {                  // FR-B01, verbatim shape from BACKTEST-PHASE1.md
    pub id: InstrumentId, pub venue: Venue, pub native_id: i64, pub kind: InstrumentKind,
    pub tick_size: Price, pub lot_size: i64, pub multiplier: i64, pub freeze_qty: i64,
    pub price_band: Option<(Price, Price)>, pub currency: Currency,
}

pub enum BookState { Uninit, Recovering, Ok, Stale }   // FR-B10, verbatim

pub struct OrderHandle {                 // FR-B05 — MCX has no broadcast order id
    pub instrument: InstrumentId, pub side: Side, pub price: Price, pub priority_ts: u64,
}
```

Give every type here `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` where it fits (matches the Debug/Display convention already established in `decoder`) — `Debug` always, `Display` only where a human reads it directly (probably `Price`/`Qty`/`Side`, same as `decoder` already does).

## Out of scope

`Event`/normalized-message types (that's `book`'s concern once it knows what it needs from `decoder`'s output — don't guess the shape here first). Anything scheduler/cache/simulator/execution-specific.

## Acceptance

`cargo build` succeeds with `types` added to `main.rs` (even if nothing uses it yet — a `#[allow(dead_code)] mod types;` is fine at this stage). No behavior to test yet; this is pure data shape.

## Done when

- [ ] `types.rs` compiles standalone
- [ ] Every type has `Debug`; human-facing ones have hand-written `Display`
- [ ] `types_user_doc.md` written — what each type is for, one line each, and which component introduced the need for it

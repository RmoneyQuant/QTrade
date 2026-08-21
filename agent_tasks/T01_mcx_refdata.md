# T01 — MCX reference data / contract loader

**Wave:** 1 (parallel with T02)
**Depends on:** T00 (workspace + shared types — done, see [OUTPUT_T00_structure.md](OUTPUT_T00_structure.md))
**Owns two files, in two crates — nothing else:**
- `qtrade/crates/qtrade-refdata/src/lib.rs` — generic, venue-agnostic `InstrumentMaster` + query builder (already scaffolded; you're filling in `todo!()` bodies, not designing the shape)
- `qtrade/crates/adapters/qtrade-adapter-mcx/src/refdata.rs` — the actual `MCXScrips.bcp` parser (also scaffolded, one `todo!()` function: `load_mcx_instruments`)

**Why two files, not one:** `qtrade-refdata` is venue-agnostic core — it never parses a file itself and must never depend on the MCX adapter crate. `qtrade-adapter-mcx::refdata` is the only place that knows `MCXScrips.bcp`'s column layout exists. It produces a plain `Vec<qtrade_types::Instrument>`; the caller (a test harness for now, the real wiring later) feeds that into `InstrumentMaster::new(..)`. Do not add a dependency from `qtrade-refdata` back to `qtrade-adapter-mcx`, or from the adapter crate to `qtrade-refdata` — the boundary is deliberate, it's what lets a second venue's loader plug into the same `InstrumentMaster` later with zero changes to it.

---

## Context

Every trading day, MCX publishes a contract file (`MCXScrips.bcp`) listing every tradable instrument for that session — token, symbol, tick size, lot size, price band, expiry, instrument type. Per [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D15/FR-16, instrument identifiers are **not stable across days**, so this loader has to run fresh per trading day, and the file used for a backtest must be the one archived alongside that day's recording — never a "current" one.

`references/Contract.cpp` already does this parsing, for MCX and six other exchanges in one function. **You only care about the `case EXCHG_MCX:` branch**, roughly lines 123–458. The other branches (`EXCHG_BSE_CD`, `EXCHG_INX`, `EXCHG_CME`, `EXCHG_NSE_FO`, `EXCHG_NSE_CM`, `EXCHG_NSE_CD`, `EXCHG_DGCX`, `EXCHG_ICX`) are not your concern — read past them.

## Required reading

- `../references/Contract.cpp` lines 1–22 (defines/includes — note `EXCHG_MCX_OMS` is defined, meaning the MCX branch is active) and lines 96–114 (constructor, `set_min_max_token` helper), then the full `case EXCHG_MCX:` block, lines ~123–458.
- [../ARCHITECTURE.md](../ARCHITECTURE.md) §4.7 (Data Engine — reference data) and §4.4 (instrument filter — you provide the metadata query interface this depends on)
- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M1 in full (FR-B01 Instrument taxonomy, FR-B02 Interned identity, FR-B03 Query interface)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D15 (instruments named by strategy, engine supplies metadata+query) and D37 (instrument taxonomy — implement `Future` only, per FR-B01's `InstrumentKind` enum)

## What to port, field by field

`MCXScrips.bcp` is a comma-separated file, no reliable header row assumption, with 100+ columns per line — read it as a plain CSV split on `,`, don't assume named columns. From the C++ (all column indices are 0-based into the split line):

| Field | Source column(s) | Notes |
|---|---|---|
| Token (key) | `parts[5]` | Skip the line entirely if this is `0` |
| Filter condition | `parts[9]==1 && parts[108]==0 && parts[38][0]=='N'` | **Replicate exactly.** This is the real filter the legacy loader applies before accepting a row — not every row in the file is a live tradable contract. `parts[108]` is a "Spread Type" flag (must be 0 — spreads are out of scope, see D37 — `Spread` is a stub). |
| StreamID | `parts[2]` | Which of the MCX capture streams (`_1` through `_N` in the `.bin` filenames) carries this token's data — this is the join key to the decoder's per-stream files |
| TickSize | `parts[21]`, floored to minimum `5` if smaller | |
| LotSize | `parts[20]` | |
| HighPriceRange / LowPriceRange (DPR bounds) | `parts[64]` / `parts[65]` | With the fallback rules in the code: if `HighPriceRange < 0`, use `TickSize * 10`; if `LowPriceRange < 0`, use `TickSize`; then `LowPriceRange` is floored to a multiple of `TickSize` (or `0` if already below one tick) |
| ExpiryDate | `parts[54]`, minus `19800` if `> 0`, else `0` | The `-19800` is a 5.5-hour (IST offset in seconds) correction — confirm this produces a sane UTC epoch second against a real file before trusting it blindly |
| StrikePrice | `parts[55]` | Always `0`/irrelevant for the `Future` instruments phase 1 cares about, but parse it — it's needed once `Option` support is unstubbed later |
| BasePrice | computed: `(parts[20] * parts[77]) / parts[62] * 100`, cast through `double` in the original | Port the arithmetic exactly, including the `* 100` |
| Multiplier | same formula as BasePrice, same three columns | |
| Symbol | `parts[6]`, with spaces stripped | |
| InstrumentType | `parts[53]`, take the substring up to the first space, then classify | **The mapping function (`String_To_Instrument_Type`) is not in the files we have.** For phase 1 you only need to recognise `FUTCOM` (or whatever token this file actually uses for commodity futures) reliably — inspect real values of `parts[53]` in a live `MCXScrips.bcp` file (read-only) for CRUDEOIL/NATURALGAS rows and confirm the exact string before hardcoding the match. Do not guess the full enum; only the `Future` case matters this round. |
| OptionType | `parts[56]` | Similarly, only needs to resolve to "none" for futures this round |

## Deliverable

**In `qtrade-adapter-mcx::refdata::load_mcx_instruments`:**
1. Takes a path to a dated `MCXScrips.bcp` (the caller supplies it — do not hardcode `/mnt/...` inside library code; that's a test-only concern) and parses it into `Vec<qtrade_types::Instrument>`, using the `InstrumentKind`/`Instrument` shape already defined in `qtrade-types` (FR-B01 — don't redefine it here, `use qtrade_types::Instrument`).
2. Assigns a stable, interned `InstrumentId` per FR-B02 as it parses — a dense `u32` counter over accepted rows, with the native MCX token kept in `Instrument::native_id`, never conflated with the identity used elsewhere in the system.

**In `qtrade-refdata::InstrumentQuery`:**
3. Fill in `front_n_expiries` and `collect` per FR-B03 — filter by venue + underlying symbol + kind + "front N expiries ordered by expiry," matching the shape [../STRATEGY-GUIDE.md](../STRATEGY-GUIDE.md) §4 shows a strategy actually calling. This part of your work has no MCX-specific logic in it at all — if you find yourself writing anything that mentions `MCXScrips.bcp` or `SecurityID` here, it belongs in the other file instead.

## Out of scope

Every non-MCX branch of `Contract.cpp`. The SPAN margin file (`MCXRPF.spn`) and `Contact_Files` calendar directory. Anything beyond parsing + querying — no book building, no decoding. Any other file in either crate — in particular, `qtrade-adapter-mcx/src/{wire,decode,normalize}.rs` belong to T02.

## Constraints

- **Read-only on `/mnt/*` and `references/*`** — no exceptions.
- Test against a real dated contract file under `/mnt/MCX_Recording_Files/CONTRACT/<date>/MCXScrips.bcp` (pick one date, state which in your output) — but never write anything there.

## Acceptance

Load a real `MCXScrips.bcp`. Report: total row count, total accepted-after-filter count, and specifically list every CRUDEOIL and NATURALGAS future found with its token, tick size, lot size, and expiry, ordered by expiry. A human should be able to look at that output and immediately tell whether it's plausible (a small number of near-month/far-month contracts, sane tick sizes, expiries in the near future relative to the contract file's date).

## Done when

- [ ] Parses a real file without panicking, with the exact filter condition replicated
- [ ] Query interface matches FR-B03's shape (`front_n_expiries`, etc.)
- [ ] `InstrumentId` is interned and stable within a single load
- [ ] Acceptance report produced and included in your final output
- [ ] `cargo tree -p qtrade-refdata` still shows only `qtrade-types` as a dependency

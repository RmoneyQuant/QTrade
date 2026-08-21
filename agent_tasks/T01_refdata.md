# T01 — `refdata`

**Folder:** `qtrade/src/refdata/` → `refdata.rs` + `refdata_user_doc.md`
**Depends on:** `types` (T00)
**Milestone:** M1 — Reference data and instrument taxonomy

---

## What it is

Loads MCX's daily contract file (`MCXScrips.bcp`) into `types::Instrument` records, and answers queries over them — "give me the two nearest Crude Oil futures by expiry." Every trading day gets a fresh load; instrument identifiers (`SecurityID`/token) are **not stable across days** (FR-16), so this can't be cached across sessions.

## Required reading

- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M1 in full — FR-B01 (taxonomy, done in T00), FR-B02 (interned identity), FR-B03 (query interface, exact shape given)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D15 (strategies name specific contracts, engine supplies metadata + query, roll policy stays in strategy code — you're building the "supplies metadata + query" half only)
- [../STRATEGY-GUIDE.md](../STRATEGY-GUIDE.md) §4 — shows the exact call shape (`ctx.instruments().venue(...).underlying(...).kind_is_future().front_n_expiries(2).collect()`) this component's query API needs to support
- `../references/Contract.cpp` — **only** the `case EXCHG_MCX:` branch, roughly lines 123–458. Every other exchange branch in that file is irrelevant; read past them

## What to port, field by field

`MCXScrips.bcp` is comma-separated, no reliable header row, 100+ columns per line. Column indices below are 0-based into the split line, taken directly from the C++:

| Field | Source column(s) | Notes |
|---|---|---|
| Token (native id) | `parts[5]` | Skip the row if `0` |
| Filter | `parts[9]==1 && parts[108]==0 && parts[38][0]=='N'` | **Replicate exactly** — not every row is a live tradable contract. `parts[108]` is a spread-type flag (must be 0; spreads are a stub per D37) |
| StreamID | `parts[2]` | Which capture stream carries this token — not needed by `refdata` itself, but useful to carry through if a later component wants it |
| TickSize | `parts[21]`, floored to `5` if smaller | |
| LotSize | `parts[20]` | |
| DPR bounds | `parts[64]` / `parts[65]` | **Correction found during the decoder pilot:** for CRUDEOILM these came out as small values (`4`/`4`), which look like a **percentage circuit band**, not an absolute rupee range as the field names imply. Verify this against a few real rows before trusting either interpretation — don't assume the C++'s naming is accurate, it's already been wrong once this project (see `decoder`'s user doc, price scaling) |
| ExpiryDate | `parts[54]`, minus `19800` if `>0` else `0` | IST-offset correction in seconds; sanity-check against a real file |
| Symbol | `parts[6]`, spaces stripped | |
| InstrumentType | `parts[53]`, substring to first space | The mapping function (`String_To_Instrument_Type`) isn't in the files we have — inspect real `CRUDEOIL`/`NATURALGAS` rows and confirm the exact string (likely `FUTCOM`) before hardcoding the match. Only `Future` matters this round |

## Build

1. `pub fn load_mcx_instruments(path: &Path) -> Result<Vec<Instrument>, RefDataError>` — parses the file, assigns each accepted row a dense `InstrumentId` (a counter over accepted rows, not derived from the token — FR-B02).
2. `pub struct InstrumentMaster { .. }` holding the loaded `Vec<Instrument>`, plus a query builder matching the STRATEGY-GUIDE.md §4 shape: `.venue(..)`, `.underlying(..)`, `.kind_is_future()`, `.front_n_expiries(n)`, `.collect()`.

## Out of scope

Every non-MCX branch of `Contract.cpp`. The SPAN margin file, `Contact_Files` calendar directory. Anything beyond parse + query.

## Constraints

Read-only on `/mnt/*` and `references/*`. Test against a real dated file, e.g. `/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp` (same date the decoder was validated against) — state which date you used.

## Acceptance (FR-B01/02/03)

Load a real file. Report: total rows, accepted-after-filter count, and every CRUDEOIL/NATURALGAS future found with token, tick size, lot size, expiry, ordered by expiry. `front_n_expiries(2)` on a day with four live Crude contracts must return exactly the two nearest, ordered — verify this explicitly, don't just eyeball the full list.

## Done when

- [ ] Parses a real file without panicking, exact filter condition replicated
- [ ] `InstrumentId` interned and stable within one load
- [ ] Query interface matches the STRATEGY-GUIDE.md §4 call shape
- [ ] DPR-bounds interpretation (percentage vs absolute) resolved and documented, not assumed
- [ ] `refdata_user_doc.md` written — same depth as `decoder`'s: what it does, how to run/test it standalone, which file path it reads, the exact column mapping table above (so nobody re-derives it from the C++ twice)

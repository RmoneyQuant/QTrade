# `refdata` — component documentation

**What this component does, in one sentence:** loads MCX's daily contract
file (`MCXScrips.bcp`) into typed `Instrument` records and answers
"which contracts" queries over them — e.g. "the two nearest Crude Oil
futures by expiry."

Code: [`refdata.rs`](refdata.rs) (this folder). Not yet wired into
`main.rs` — this component is data-in/data-out and has no CLI of its own;
see §2 for how it was actually run and verified against real data.

---

## 1. What it builds

Two public pieces, per T01:

```rust
pub fn load_mcx_instruments(path: &Path) -> Result<Vec<Instrument>, RefDataError>;

pub struct InstrumentMaster { /* holds the loaded Vec<Instrument> */ }
impl InstrumentMaster {
    pub fn load_mcx(path: &Path) -> Result<Self, RefDataError>;
    pub fn all(&self) -> &[Instrument];
    pub fn get(&self, id: InstrumentId) -> Option<&Instrument>; // dense array index, no hash lookup (FR-B02)
    pub fn instruments(&self) -> InstrumentQuery<'_>;
}
```

The query builder matches the exact call shape from STRATEGY-GUIDE.md §4
/ BACKTEST-PHASE1.md FR-B03:

```rust
master.instruments()
      .venue(Venue::Mcx)
      .underlying("CRUDEOIL")
      .kind_is_future()
      .front_n_expiries(2)
      .collect()   // -> Vec<InstrumentId>
```

`.collect()` returns `InstrumentId`s, not full `Instrument` records — a
strategy subscribes and declares dependencies by id (see
STRATEGY-GUIDE.md §4's `for i in &quoted { ctx.subscribe(*i, ...) }`,
which dereferences a `Copy` value). Resolve metadata back through
`InstrumentMaster::get(id)` only when a field is actually needed, so the
common path never copies a full record.

**Every trading day gets a fresh load.** `InstrumentId` is a dense `u32`
counter assigned at load time over accepted rows — not derived from the
file's own `Token` (`SecurityID`), because FR-16 states tokens are not
stable across days. Do not cache an `InstrumentMaster` across sessions.

---

## 2. How to run/test it standalone

`refdata.rs` is not yet wired into `main.rs` (another agent owns that
file for a different, concurrent change), so it was verified with a
throwaway harness that compiles the *real* files in place via `#[path]`
— no copies, nothing under `/mnt` or `references/` touched:

```rust
// scratch main.rs, anywhere outside the qtrade tree
#[path = "/home/.../qtrade/src/types/types.rs"]
mod types;
#[path = "/home/.../qtrade/src/refdata/refdata.rs"]
mod refdata;

fn main() {
    let master = refdata::InstrumentMaster::load_mcx(
        std::path::Path::new("/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp")
    ).unwrap();
    // ... query and print ...
}
```

Once `refdata` is wired into the real crate (`#[path = "refdata/refdata.rs"] mod refdata;`
in `main.rs`, same pattern as `decoder`/`types`), the unit tests below run
the normal way:

```bash
cd qtrade
cargo test refdata::
```

**Unit tests included** (`#[cfg(test)] mod tests` at the bottom of
`refdata.rs`) cover the epoch-day → `YearMonth` conversion (`year_month_from_days`)
against known dates — this is the one piece of nontrivial logic in the
file that isn't a straight column read.

---

## 3. Which file it reads

**Input:** MCX's daily contract file, one instrument per line, comma
separated, 118 columns, no header row:

```
/mnt/<date>/MCXScrips.bcp
```

**Test file used for everything in this doc:**
`/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp` — the same
date `decoder` was validated against.

**Read-only.** `load_mcx_instruments` only calls `std::fs::read_to_string` —
never writes, moves, or deletes anything under `/mnt`.

---

## 4. Column mapping (from `Contract.cpp`'s `EXCHG_MCX` branch, lines ~123–458)

All indices are 0-based into the comma-split line.

| Field | Source column(s) | Notes |
|---|---|---|
| Token (native id) | `parts[5]` | Skip the row if `0` |
| Filter | `parts[9]==1 && parts[108]==0 && parts[38][0]=='N'` | Replicated exactly. `parts[108]` is the spread-type flag (must be 0; spreads are a stub per D37) |
| StreamID | `parts[2]` | Not consumed by `refdata` — noted here for a later component, not carried in `Instrument` |
| TickSize | `parts[21]`, in **paise** (see §4a), floored to `5` paise if smaller, then converted to wire-raw via `ContractFilePaise::to_wire_price()` | |
| LotSize | `parts[20]` | |
| DPR bounds | `parts[64]` / `parts[65]` | **Resolved as a percentage circuit band, not absolute rupees — see §5** |
| ExpiryDate | `parts[54]`, minus `19800` if `>0` else `0` | IST-offset correction in seconds, then converted to days-since-epoch for `Date` |
| Symbol | `parts[6]`, spaces stripped | Also used as `InstrumentKind::Future`'s `underlying` — see note below |
| InstrumentType | `parts[53]`, substring to first space | Matched against the literal string `"FUTCOM"`. Only `Future` is built this round (D37) — any other accepted instrument type (`OPTFUT`, `OPTIDX`, `FUTIDX`, `COM`) is silently skipped |

**On `Symbol` doubling as `underlying`:** MCX's contract file has no
column distinct from the per-contract symbol that names "the underlying"
separately from "the contract." `CRUDEOIL` and `CRUDEOILM` (Mini) are
*different* symbols in `parts[6]` with different tokens, lot sizes and
expiries — exactly the two things D15 says a strategy names explicitly
(`ctx.instruments().underlying("CRUDEOIL")` matches only the standard
contract, never the Mini). Expiry is carried in the separate `ExpiryDate`
column, so the symbol alone is already the right granularity for
`underlying`.

### Fields not in T01's scope, and what they default to

`types::Instrument` requires a few fields this task's column table never
names a source for. Rather than guess at a formula, each is defaulted and
documented here so nobody mistakes a placeholder for a derived value:

| Field | Default | Why |
|---|---|---|
| `multiplier` | `lot_size` | `Contract.cpp` computes its own `Multiplier` from a formula combining columns 20/62/77 not in T01's mapping table, and its exact meaning there is unclear (looks like a margin-calc helper, not "units per lot"). `lot_size` is the only defensible value without a real column to read |
| `freeze_qty` | `0` | No source column given in T01, and `Contract.cpp`'s `EXCHG_MCX` branch never sets a freeze quantity either (`grep -i freeze` on the whole file: no match) |
| `price_band` | `None` | See §5 — the only candidate source (DPR bounds) is a percentage, and converting it to an absolute `(Price, Price)` band needs a reference/settlement price this task's column set doesn't supply |
| `kind`'s `settlement` | `Settlement::Cash` | No settlement-type column in scope. MCX Crude Oil, Crude Oil Mini, Natural Gas and Natural Gas Mini are all cash-settled in practice, so this is a safe default for every `FUTCOM` row loaded here — not derived from the file |

None of these needed a change to `types.rs` — every field already exists
there with the right shape; `refdata` just doesn't have a column to
populate a few of them precisely yet.

---

## 4a. TickSize units — resolved, not assumed (a bug found and fixed)

**This was a real, shipped bug**, not a hypothetical: an earlier version
of `load_mcx_instruments` put `parts[21]`'s raw integer straight into
`Instrument.tick_size: Price` with no conversion at all. `types::Price`
everywhere else in this codebase (`decoder`, `book`, `simulator`,
`execution`) means "real rupees × 100,000,000" (the wire's own raw
scale) — so a raw column value like `100` was being handed out as
`Price(100)`, six orders of magnitude off `book.rs`'s own independently
validated real value of `Price(100_000_000)` for CRUDEOIL's real Rs 1.00
tick. `execution::validate()`'s tick-size gate (`price.0 %
instrument.tick_size.0`) silently comparing a real wire-raw `Price`
against this near-zero `tick_size` is what first surfaced the bug — see
`execution_user_doc.md` and `dummy_strategy.md` for the actual inflated
output this produced before the fix.

**Question:** what unit is `parts[21]` (`TickSize`) actually in?

**Evidence gathered from real rows** (19 Jan 2026 file):

| Symbol | `parts[21]` (raw) | Read as paise → rupees | Real, publicly documented MCX tick size |
|---|---|---|---|
| CRUDEOIL / CRUDEOILM (all 6 expiries each) | `100` | Rs 1.00 | Rs 1.00 per barrel |
| NATURALGAS / NATGASMINI (all 6 expiries each) | `10` | Rs 0.10 | Rs 0.10 per mmBtu |
| GOLD | `100` | Rs 1.00 | Rs 1 per 10 gram |
| SILVER | `100` | Rs 1.00 | Rs 1 per kg |
| COPPER | `5` | Rs 0.05 | 5 paise per kg |
| ALUMINIUM | `5` | Rs 0.05 | 5 paise per kg |
| ZINC | `5` | Rs 0.05 | 5 paise per kg |
| LEAD | `5` | Rs 0.05 | 5 paise per kg |

Every one of these 24+ real rows, read as **paise** (hundredths of a
rupee, i.e. divide by 100 to get rupees), reproduces that commodity's
real, publicly documented MCX tick size exactly — not just "plausible,"
exactly. This is far stronger evidence than the four data points
(CRUDEOIL ×3 expiries + NATURALGAS) that first suggested the paise
reading: eight different commodities, two different real tick sizes
(Rs 1.00 and Rs 0.05), all agreeing.

**Conclusion: `parts[21]` is denominated in paise.** `types::ContractFilePaise`
(`types.rs`) exists specifically to carry this value before conversion —
`refdata` cannot construct an `Instrument.tick_size: Price` without
routing through `ContractFilePaise::to_wire_price()`, which makes the raw
column value → wire-raw `Price` conversion (paise → rupees is `/100`;
rupees → wire-raw is `×100,000,000`; combined, `×1,000,000`) impossible to
skip by accident a second time.

**On the floor ("5 if smaller"):** the original stub floored the raw
column value at `5` before it was known what unit that `5` was in. Now
that the unit is known to be paise, the floor is applied to the raw paise
value (protecting against a `0`–`4` paise, i.e. sub-5-paise, degenerate
column value) rather than to the wire-raw `Price` — a floor of "5
wire-raw units" would have been ~Rs 0.00000005, an effectively
meaningless guard that only ever caught a literal `0`. None of the 24
real FUTCOM rows this component targets ever need the floor (the
smallest observed is `5`, already at the floor).

---

## 5. The DPR-bounds question — resolved, not assumed

**Question:** are `parts[64]`/`parts[65]` (`Contract.cpp`'s
`HighPriceRange`/`LowPriceRange`) an absolute rupee price band, or a
percentage circuit band?

**Evidence gathered from real rows** (19 Jan 2026 file, every accepted
`FUTCOM` row, one line per commodity group):

| Symbol | `parts[64]`/`parts[65]` | Underlying price level |
|---|---|---|
| GOLD, GOLDGUINEA, GOLDM, GOLDPETAL, GOLDTEN | `3` / `3` | ~₹80,000–90,000 (per 10g/8g/1g unit, varies) |
| SILVER, SILVERM, SILVERMIC | `4` / `4` | ~₹1,00,000+ (per kg) |
| CRUDEOIL, CRUDEOILM | `4` / `4` | ~₹5,000–5,500 (per barrel) |
| NATURALGAS, NATGASMINI | `4` / `4` | ~₹250–350 (per mmBtu) |
| COPPER | `4` / `4` | ~₹800–900 (per kg) |
| ALUMINIUM, ZINC, LEAD (+ mini variants) | `4` / `4` | ~₹200–260 (per kg) |
| ELECDMBL | `6` / `6` | (electricity derivative, different unit) |

Two independent findings settle this:

1. **The value is flat across wildly different price levels within a
   commodity, and identical across unrelated commodities at very
   different price levels** (Crude at ~₹5,000/bbl and Aluminium at
   ~₹250/kg both show `4`). An absolute rupee band would have to scale
   with the instrument's price level — it manifestly does not.
2. **The same value persists seven months later.** Checked the contract
   file from `20_08_2026`: GOLD is still `3`, CRUDEOIL/CRUDEOILM/
   NATURALGAS are still `4`, despite different contract months and
   different settlement prices in between. A rupee band would need
   re-deriving as prices drifted; a percentage-tier classification
   (MCX's own risk policy grouping commodities into 3%/4%/6% circuit
   bands) would not change unless the policy itself changed — which is
   exactly what's observed.

**Conclusion: `parts[64]`/`parts[65]` are a percentage circuit/DPR band**
(e.g. `4` = ±4%), not an absolute rupee range, despite `Contract.cpp`'s
own field names (`HighPriceRange`/`LowPriceRange`) implying an absolute
value. This is the same class of naming trap the `decoder` pilot hit with
its price-scaling constant (see `decoder`'s user doc §6) — the legacy
C++'s field names are not a reliable guide to units.

**Consequence for `Instrument.price_band`:** converting a percentage into
an absolute `(Price, Price)` tick band requires a reference price (e.g.
previous close or a computed base price) that isn't among the columns
T01 scopes for this task. Rather than fabricate a band using the wrong
unit (treating `4` as ±4 rupees, which is obviously wrong at every price
level above), `price_band` is left `None` for every loaded instrument.
Computing the real band is a follow-up task, not a guess made here.

---

## 6. Acceptance — real numbers, from the real file

**File:** `/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp`
(50,081 lines).

```
total rows:              50081
accepted after filter:   49722     (parts[9]==1 && parts[108]==0 && parts[38][0]=='N', token != 0)

instrument type breakdown among the 49722 accepted rows:
  OPTFUT      48632
  OPTIDX        904
  FUTCOM        140   <- what refdata builds Instrument records from (D37: Future only)
  COM            39
  FUTIDX          7

instruments loaded (FUTCOM only): 140
```

**Every CRUDEOIL / NATURALGAS future found**, ordered by expiry (both
standard and Mini contracts, since the task's underlyings are named
generically and MCX trades both sizes):

| Underlying | Token | Tick size (raw paise, `parts[21]`) | `Instrument.tick_size` (wire-raw `Price`, post-fix) | Lot size | Expiry |
|---|---|---|---|---|---|
| CRUDEOIL | 467013 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-02-19 |
| CRUDEOIL | 472789 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-03-19 |
| CRUDEOIL | 486502 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-04-20 |
| CRUDEOIL | 488290 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-05-18 |
| CRUDEOIL | 499095 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-06-18 |
| CRUDEOIL | 520702 | 100 | 100,000,000 (Rs 1.00) | 100 | 2026-07-20 |
| CRUDEOILM | 467014 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-02-19 |
| CRUDEOILM | 472790 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-03-19 |
| CRUDEOILM | 486503 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-04-20 |
| CRUDEOILM | 488291 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-05-18 |
| CRUDEOILM | 499096 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-06-18 |
| CRUDEOILM | 520703 | 100 | 100,000,000 (Rs 1.00) | 10 | 2026-07-20 |
| NATURALGAS | 465849 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-01-27 |
| NATURALGAS | 467385 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-02-24 |
| NATURALGAS | 475111 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-03-26 |
| NATURALGAS | 487465 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-04-27 |
| NATURALGAS | 488505 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-05-26 |
| NATURALGAS | 504265 | 10 | 10,000,000 (Rs 0.10) | 1250 | 2026-06-25 |
| NATGASMINI | 465850 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-01-27 |
| NATGASMINI | 467386 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-02-24 |
| NATGASMINI | 475112 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-03-26 |
| NATGASMINI | 487466 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-04-27 |
| NATGASMINI | 488506 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-05-26 |
| NATGASMINI | 504266 | 10 | 10,000,000 (Rs 0.10) | 250 | 2026-06-25 |

(See §4a for the paise finding and the conversion. `Instrument.tick_size`
is now `types::ContractFilePaise(raw).to_wire_price()` — verified by
reloading this exact file after the fix: CRUDEOIL (467013) resolves to
exactly `Price(100_000_000)` and NATURALGAS (465849) to exactly
`Price(10_000_000)`, matching `book.rs`'s own independently-validated
`band_config` values exactly.)

**On "four live Crude contracts":** T01 and BACKTEST-PHASE1.md's FR-B03
both use "a day with four live Crude contracts" as the illustrative
acceptance scenario. The real 19 Jan 2026 file has **six** live CRUDEOIL
futures, not four — noted here rather than silently substituted. The
`front_n_expiries(2)` verification below runs against the real six-contract
day and is not weakened by the count not matching the example.

**`front_n_expiries(2)` verification, CRUDEOIL, real data:**

```
all live CRUDEOIL futures (6), by expiry:
  token=467013 expiry=2026-02-19
  token=472789 expiry=2026-03-19
  token=486502 expiry=2026-04-20
  token=488290 expiry=2026-05-18
  token=499095 expiry=2026-06-18
  token=520702 expiry=2026-07-20

front_n_expiries(2) result (2 items):
  token=467013 expiry=2026-02-19
  token=472789 expiry=2026-03-19

VERIFIED: front_n_expiries(2) == the two nearest CRUDEOIL expiries, in order.
```

This was checked with a hard assertion in the verification harness
(`front2 == all_crude[..2]`), not eyeballed — the harness panics if the
query returns the wrong count or the wrong order.

---

## 6a. On `InstrumentId` unification — a found-and-fixed bug, not a design choice

`Instrument.id` used to be a dense counter assigned here at load time (0, 1,
2... in file order) — FR-B02's original, literal wording ("interned,
dense"). That was a real bug, found while wiring `refdata` and `execution`
together for the first time (see `dummy_strategy.md`'s findings list):
`book`, `cache`, `simulator`, and `execution` had all independently settled
on a *different* convention — the native exchange token, cast directly into
`InstrumentId` (see `book.rs`'s `CRUDEOIL_ID`/`NATURALGAS_ID` constants) —
because that's what real decoded market messages carry, and nothing routes
through `refdata` to translate it. Two live numbering schemes for the same
concept meant anything needing both `refdata`'s metadata and `book`'s book
for the same instrument had to manually bridge them (`dummy_strategy.rs`
used to do exactly this with a one-line `.map(|mut i| { i.id = ... })`).

**Fixed by unifying on the convention everything else already used**:
`load_mcx_instruments` now sets `id: InstrumentId(token as u32)` directly —
the same value as `native_id`, just wrapped. No separate counter, no
bridge needed anywhere downstream.

**Why this doesn't slow anything down**: the only place the old dense
numbering was actually load-bearing for performance was
`InstrumentMaster::get()`, which used to index directly into a `Vec` by
`id.0`. Nothing else in qtrade ever used `InstrumentId` as an array index —
`book`, `cache`, and `simulator` all store instruments in `HashMap`s
already, where a large sparse key (467013) costs exactly the same as a
small dense one (3). `get()` now does one hash lookup through a
`HashMap<InstrumentId, usize>` built once at load time instead of a direct
array index — real, measured overhead is a single hash of a 4-byte integer,
irrelevant next to actual per-message decode/book-update cost (`cache`'s
own full-session run processes ~1.9M messages/sec doing hashmap lookups
like this on every one).

Verified against real data after the fix: `master.get(InstrumentId(467_013))`
correctly returns CRUDEOIL (`tick_size = Price(100_000_000)`),
`master.get(InstrumentId(465_849))` correctly returns NATURALGAS
(`tick_size = Price(10_000_000)`), and an unknown id correctly returns
`None`.

## 7. What this component deliberately does not do

- No instrument types beyond `Future` — D37 stubs `Option`/`Equity`/`Spread`,
  and this loader silently skips any accepted row of another type rather
  than mis-modelling it.
- No absolute price-band computation — `price_band` is `None` for every
  instrument; see §5.
- No roll policy, no "front month" resolution beyond the literal
  `front_n_expiries(n)` the query exposes — D15 keeps that in strategy
  code.
- No caching across trading days — a fresh `InstrumentMaster` is expected
  per session (FR-16: tokens are not stable across days).
- No SPAN margin file, no `Contact_Files` calendar directory — out of
  scope per T01.

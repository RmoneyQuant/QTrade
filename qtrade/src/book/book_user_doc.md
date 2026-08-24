# Book — component documentation

**What this component does, in one sentence:** builds one incremental
MBO (market-by-order) book per instrument from `decoder`'s message
stream, and proves it correct by replaying a full real session against
MCX's own periodic full-book snapshots.

Code: [`book.rs`](book.rs) (this folder). Validation harness (not part of
the public API): [`validate.rs`](validate.rs), built as the
`book-validate` binary — see §5.

---

## 1. What's built

- `trait Book` — `best_bid` / `best_ask` / `depth(n)` / `qty_at_price` /
  `state`, the shape FR-B08 gives verbatim.
- `trait MboBook: Book` — adds `queue_position`, MBO-only by type (an
  MBP book, when one exists, simply won't implement it — the compiler
  catches a strategy asking for queue position on the wrong kind of
  book, rather than it silently returning a guess).
- `MboBookImpl` — the one implementation. A dense array of price levels
  per side, indexed by tick offset from a per-instrument price band; each
  level is a FIFO of resting-order slots plus aggregate quantity and
  count.
- `BookBuilder` — owns one `MboBookImpl` per instrument it's constructed
  with, routes decoded events to the right book by native `SecurityID`,
  and is what `cache` (T05) / `simulator` (T06) will actually hold or
  drive (ARCHITECTURE.md §4.8).

**Update, a later task: price band and tick size are now fully generic.**
The two-instrument hardcoded `band_config` this section originally
described is gone. Tick size comes from `refdata::Instrument.tick_size`
(correct for every instrument now); price band is learned from a real
`InstrumentInfo` (template 13603) message in the applied stream, or
seeded explicitly by a caller whose own feed doesn't carry one. See §3
for the full mechanism and §7a for the real evidence this generalizes
(CRUDEOIL/NATURALGAS reproduce the exact old numbers; a third,
previously-unsupported instrument, ALUMINIUM, now builds and validates
end to end). `BookBuilder::new`'s signature changed accordingly — see §3's
"what changed for callers" box.

## 2. Which files this reads

**`book.rs` itself reads nothing** — it's a pure function of whatever
`DecodedMessage`s are handed to `BookBuilder::apply`. All file I/O lives
in `validate.rs`, which reads two paired real capture files per
instrument:

```
mcx_feeder_Increment_capture_19_01_2026_1_<stream>.bin   (builds the book)
mcx_feeder_snapshot_capture_19_01_2026_1_<stream>.bin    (ground truth)
```

**Stream mapping used, and why it differs from the task brief.** The
brief stated CRUDEOIL (467013) on stream 3 and NATURALGAS (465849) on
stream 4. Both were wrong for this capture set — checked before writing
any book code, the same "don't blindly trust, cross-check against real
bytes" discipline `decoder` already established for its price scaling.
Verified by scanning real `OrderAdd` (13100) records and their decoded
prices:

| Instrument | Native token | Actual stream | Real traded price range seen | Segment (`MarketSegmentID`) |
|---|---|---|---|---|
| CRUDEOIL | 467013 | **4** (not 3) | ~5,232–5,666 (matches decoder's own CRUDEOILM cross-check) | 294 |
| NATURALGAS | 465849 | **5** (not 4) | ~246.8–314.0 (matches MCX's real NATURALGAS range) | 401 |

Token 467013 does not appear anywhere in stream 3's `OrderAdd` records at
all (checked ~12M records); it appears 69,575 times in the first 1.5GB of
stream 4 alone. Same story for 465849 on stream 5. Both instruments' own
`OrderAdd` records were sampled and their decoded prices cross-checked
against known real MCX price ranges for these products before trusting
the mapping.

Also note: neither file pair turned out to need the 60.8GB-file OOM
concern the brief raised — that size belongs to stream 3, which isn't
used here. The files actually validated against are 6.8GB (CRUDEOIL
increment) and 30.4GB (NATURALGAS increment), both comfortably under the
observed ~65–120GB free RAM. The streaming reader (§5) was built anyway,
since it's the safer default regardless of file size and the brief asked
for it.

## 3. The dense array: price band and tick size — now fully generic

FR-B08 calls for a dense array over "the day's price band," since MCX's
circuit limits bound the range. This used to be a hardcoded `band_config()`
match covering exactly two instruments (CRUDEOIL, NATURALGAS); a later
task made both inputs real and generic for **any** instrument `refdata`
knows about — the point being that a brand-new instrument (proved below
with ALUMINIUM, §7a) now Just Works without a code change.

### 3.1 Tick size — from `refdata`, unchanged mechanism, now actually wired up

`types::Instrument.tick_size` was already correct for every instrument by
the time this task started (a separate, earlier fix converted
`MCXScrips.bcp`'s `TickSize` column, in paise, into qtrade's wire-raw
scale — see `refdata_user_doc.md`). `book` just wasn't using it:
`BookBuilder::new` now takes `(InstrumentId, tick_raw)` pairs instead of
bare `InstrumentId`s, and every caller sources `tick_raw` from a real
`Instrument.tick_size.0` (`cache::Cache::new` does this internally now;
`book-validate`'s own harness loads the real `19_01_2026` contract file
itself, generically, rather than hardcoding CRUDEOIL/NATURALGAS's already-
known tick values).

### 3.2 Price band — learned from a real `InstrumentInfo` (13603) in the stream

Unlike tick size, MCX's contract file can't give an absolute price band
generically: its own DPR columns are a *percentage* circuit band with no
reference price to convert against (see `refdata_user_doc.md` — this is
why `Instrument.price_band` is always `None`). The real, authoritative
source turned out to be template **13603**, `InstrumentInfo` — MCX's own
EOBI spec (circular MCX/CTCL/057/2024 §4.19) describes this message as
published whenever an instrument's daily price range changes, carrying
`UpperDailyPriceLimit`/`LowerDailyPriceLimit` as **absolute** wire-raw
prices, not a percentage.

**A real discrepancy in MCX's own reference material, resolved against
actual bytes, not by picking one document over the other:** the EOBI
spec's own §4.19 section heading and its master message-type table both
say this message's template id is **13603** — but that section's own
field-table row contradicts itself with a leftover "Value: 13203
(MarketDataTrade, MsgType = U22)" description line, and separately,
`references/MCX_Feeder.h` defines this exact struct (`SecurityID`,
`ClosePrice`, `PrevClosePrice`, `UpperCktLimit`, `LowerCktLimit` — field-
for-field identical) under its own comment `//TemplateID : 13203`. Two
independent MCX documents naming `13203`, but checked against real bytes
rather than trusted: a full scan of the real `19_01_2026` CRUDEOIL
snapshot capture (3,140,083 outer records) found **zero** messages tagged
`13203` anywhere, and 2,848,887 tagged `13603` with exactly this struct's
48-byte shape, decoding (for CRUDEOIL, 467013) to `UpperDailyPriceLimit`=
5,666.00 / `LowerDailyPriceLimit`=5,232.00 — matching this very document's
own already-recorded real DPR bounds for CRUDEOIL exactly (see the old
version of the table this section used to have, and §7a below for the
full real-value account). **13603 is the real wire id.** See
`decoder::InstrumentInfo`'s doc comment in `decoder.rs` for the byte-level
account; `decoder/user_doc.md` §4 records it in the template-id table too.

### 3.3 `BookBuilder`'s real sequencing design: `Pending` → `Ready`

A book's dense array can't be sized until both tick size and band are
known, but the band arrives as a message in the same stream `book`
already consumes — not something available up front the way tick size
is. `BookBuilder` reflects this explicitly with a per-instrument
`BookSlot`:

```rust
enum BookSlot {
    Pending { tick_raw: i64 },  // tick size known, band not yet
    Ready(MboBookImpl),          // both known -- the book actually exists
}
```

- `BookBuilder::new(instruments: &[(InstrumentId, i64)])` starts every
  instrument `Pending`, holding its real tick size.
- A real `InstrumentInfo` (13603) for a tracked instrument, arriving
  through `BookBuilder::apply` like any other decoded message, finalizes
  `Pending` → `Ready` (constructing the `MboBookImpl` at that moment,
  band from the message) or, if already `Ready`, **widens** the existing
  book to cover the new band if it's wider than the current one (see
  §3.4 — this is a real, not hypothetical, need).
- **A real order-mutating message for an instrument still `Pending`
  panics loudly**, per this component's existing "fail on a wrong
  assumption, never silently guess" philosophy (the same spirit as
  `MboBookImpl::idx_of`'s own panic). Per the EOBI spec's own product-
  state table (§3.3, "Availability of Enhanced Order Book Service"):
  `InstrumentInfo`/setup-type messages are published during Start-Of-Day
  and Pre-Trading states, *before* Trading state's real order flow
  begins — so in a well-behaved real feed that starts recording from
  Start-of-Day, this should never fire. It is a genuine, reachable
  condition against a feed that starts recording mid-session and never
  sees a later revision — see §3.5's real example (CRUDEOIL) and
  `BookBuilder::seed_band`'s doc comment for the sanctioned escape hatch.
- An `InstrumentInfo` (or any event) for an instrument `BookBuilder`
  wasn't constructed with is silently ignored, same FR-B16 filtering
  intent as before.
- A real, empirically-found corruption is guarded against explicitly:
  both real `19_01_2026` increment capture files end with one `13603` per
  instrument whose `PrevClosePrice`/`UpperDailyPriceLimit`/
  `LowerDailyPriceLimit` are all exactly `i64::MIN` — a genuine End-of-Day
  rebroadcast artifact for the *next* day's not-yet-computed reference
  prices, not a decode bug (see `decoder::InstrumentInfo`'s doc comment).
  `plausible_band()` rejects any band with `lower >= upper` or either
  bound near `i64::MIN` before it's ever trusted.

**What changed for callers, concretely:**

```rust
// Before:
BookBuilder::new(&[InstrumentId(467_013), InstrumentId(465_849)])

// Now:
BookBuilder::new(&[(InstrumentId(467_013), 100_000_000), (InstrumentId(465_849), 10_000_000)])
// ... then either a real InstrumentInfo (13603) arrives through `apply`,
// or the caller calls `seed_band` explicitly (§3.5).
```

`MboBookImpl::idx_of` (used when *applying* a real event) still panics if
a price falls outside the configured band or off the tick grid, unchanged
from before — the full-session validation run (§7) is exactly the place
to have caught a band sized wrong, and it never fired, for any of the
three instruments now validated.

### 3.4 Widening a live book: NATURALGAS's real DPR moved six times in one session

The first version of this mechanism (learn once, size once) would have
been wrong: NATURALGAS's real `19_01_2026` DPR genuinely revised **six**
separate times over the session, both bounds moving outward together —
269.20/291.60 (first observed) → 263.60/297.20 → 255.20/305.60 →
246.80/314.00 → 238.40/322.40 → 230.00/330.80 → 221.60/339.20 (final).
Each step is a real, distinct `InstrumentInfo`, confirmed identically on
*both* the increment and snapshot channels for every mid-session
revision (the increment channel only omits the very first, Start-of-Day
value — see §3.5). A book sized only from the first band it ever saw
would have hit `idx_of`'s panic the moment a real NATURALGAS order
landed outside it.

`MboBookImpl::widen_band_if_needed` handles this for real: given a new
`(min, max)`, it takes the union with the book's current band, and if
that's strictly wider, rebuilds the dense arrays at the new size and
remaps every currently-resting order (both sides) into its new index —
FIFOs, quantities, and order counts all preserved, nothing dropped. Never
shrinks (a narrower later `InstrumentInfo` — which never happened for
either real instrument, but is handled defensively — is a no-op). Covered
by `book.rs`'s own unit tests
(`later_wider_instrument_info_widens_the_book_preserving_orders`,
`narrower_later_instrument_info_never_shrinks_the_band`) with small
synthetic data, since exercising it against real data would need the book
to already hold real resting orders at the moment a widening
`InstrumentInfo` arrives — which, for `book-validate`'s own harness (§3.5
below), happens before `bootstrap()`, i.e. against an empty book. The
*mechanism* is real regardless of which harness first has real resting
orders in flight when a revision lands.

### 3.5 A real gap: the increment channel doesn't always carry a valid `InstrumentInfo`

Checked against real bytes, not assumed: **CRUDEOIL's real `19_01_2026`
increment capture never carries a valid 13603 at all during the trading
session.** Its DPR never changed that day (constant 5,232.00–5,666.00
throughout — see §7a), so nothing re-published it on the increment
channel; the one *unique* 13603 found there is the corrupted End-of-Day
one at the very tail of the file (§3.3), and the genuine Start-of-Day
broadcast that would have carried the real value predates this capture's
start (it began recording already in Trading state). NATURALGAS is
different only because its DPR *did* change mid-session — each real
revision is what shows up on the increment channel; had it also never
revised, its increment capture would show the identical gap.

This means a caller whose only real feed *is* the increment stream —
this crate's `cache-validate`/`dummy-strategy` binaries, both of which
never read the paired snapshot capture — has no way to learn CRUDEOIL's
band from `apply` alone. `BookBuilder::seed_band(id, band_min_raw,
band_max_raw)` is the sanctioned way to close this gap: an explicit,
caller-supplied real band (same idiom as `MboBookImpl::bootstrap`'s
explicit seeding from out-of-band real data, not the normal streaming
path — not a fallback to a guess). `cache::Cache::seed_book_band` forwards
to it; `cache-validate`/`dummy-strategy`'s own `main()`s call it once per
instrument, right after `Cache::new`, with the same real, snapshot-
verified numbers `book-validate`'s own harness independently learned from
the paired snapshot file's 13603 stream (§3.6) — not a re-introduction of
the old hardcoded `band_config` (gone, and not used for any other
instrument).

`book-validate`'s own harness doesn't need `seed_band` at all: it already
streams each instrument's paired *snapshot* file in full to build FR-B11
checkpoints (§5.1), so it unions every valid `InstrumentInfo` seen there
in the same pass and feeds the result to `BookBuilder::seed_band` before
replaying the increment file — a real, legitimate MCX channel (the
snapshot stream), not a workaround.

### 3.6 Regression check: the generic mechanism reproduces the exact old numbers

Re-running `book-validate`'s full FR-B11 gate with tick size from real
`refdata` and band learned from the snapshot file's own real 13603
stream, in place of the old hardcoded `band_config`:

| Instrument | Real tick size (generic) | Real band learned (generic) | Old hardcoded band |
|---|---|---|---|
| CRUDEOIL (467013) | ₹1.00 (100,000,000 raw) | [₹5,232.00, ₹5,666.00] | [₹3,000, ₹9,000] (manually padded) |
| NATURALGAS (465849) | ₹0.10 (10,000,000 raw) | [₹221.60, ₹339.20] (full-session union of all 6 revisions) | [₹100, ₹600] (manually padded) |

The *learned* bands are tighter than the old hand-padded ones (as
expected — they're the real DPR, not a guess padded well past it) but
still comfortably wide enough to contain every real order either
instrument saw all session: **8,024/8,024 CRUDEOIL cycles and
1,270/1,270 NATURALGAS cycles, 0 divergences, 0 diagnostic misses on
both** — byte-for-byte the same result §7 already reported with the old
hardcoded mechanism. See §7 for the full run.

## 4. Business rules encoded, and where they came from

Read from `references/MCX_Feeder.cpp`'s modify-handling section (~line
860 on) for *business rules only* — its nested price-bucket data
structure was not ported; FR-B08's dense array is the design used.

- **`OrderAdd` (13100)** — new resting order, pushed to the back of its
  price level's FIFO.
- **`OrderModify` (13101)** — priority **lost**: the old
  `(side, price, priority_ts)` identity is removed and a new one
  `(side, new_price, new_priority_ts)` is added at the back, even if the
  price didn't change. Confirmed by the presence of two distinct
  timestamp fields on the wire (`TrdRegTSPrevTimePriority` and a new
  `TrdRegTSTimePriority`) — the priority identity itself changes.
- **`OrderModifySamePriority` (13106)** — quantity reduced only, in
  place, same FIFO slot, same `priority_ts` — confirmed the wire struct
  carries no "previous price" field at all (price cannot change here),
  and only one `TrdRegTSTimePriority` (unchanged from before).
- **`OrderDelete` (13102)** — removes the exact order identified by
  `(side, price, priority_ts)`.
- **`OrderMassDelete` (13103)** — every resting order for the
  instrument, **both sides**, cleared at once. Confirmed against
  `MCX_Feeder.cpp`'s mass-delete handler, which clears the whole
  per-token book rather than one side.
- **`Trade` (13104 full / 13105 partial)** — a genuine finding, checked
  against the reference code rather than assumed: despite being named
  `aggressor_side` in `decoder.rs` (the pilot's original guess), this
  field identifies the **resting** order's side, not the side that
  aggressed. `MCX_Feeder.cpp`'s own trade handler (~line 1328)
  decrements the *buy*-side book when this field reads `1` (Buy) — which
  only makes sense if `1` means "the resting order that got hit was a
  buy," since a buy aggressor hits resting *sells*, not resting buys.
  Because of this, `book` treats `13104`/`13105` as book-mutating events:
  a trade always consumes the **front** (oldest-priority) resting order
  at that price — price-time priority means the aggressor always matches
  whoever has waited longest at the best price. A full trade removes
  that order entirely; a partial trade reduces its quantity and leaves
  it at the front, priority unchanged.
- **`ExecutionSummary` (13202)`, `TopOfBook` (13504)`, `PacketHeader`
  (13003)`, `Heartbeat` (13001)`** — confirmed **not** book-mutating
  (`MCX_Feeder.cpp` never touches its own order-book state for these;
  the `13202` handling branch is even commented out in the reference
  code). `book` no-ops on all of them.

**FR-B09, crossed books:** `best_bid()`/`best_ask()` are computed
completely independently — there is no cross-check between them anywhere
in the code. A crossed state (`best_bid >= best_ask`) is a normal
transient on an order-by-order feed (an aggressive order publishes before
the trade it causes) and is never asserted against.

**Book state machine (FR-B10):** `state` starts `Uninit` and moves to
`Ok` the first time any event is successfully applied to that
instrument's book. `Recovering`/`Stale` need a live Transport and are out
of scope this round.

## 5. The FR-B11 validation harness — the actual gate

**What it checks, precisely.** MCX's snapshot broadcast periodically
re-transmits the *entire* order book, order by order, for every
instrument. Every time a full cycle for CRUDEOIL or NATURALGAS finishes
arriving, the harness compares the complete multiset of
`(price, priority_ts, qty)` triples the incrementally-built book holds on
each side against the complete multiset the snapshot just delivered.
Equal after sorting both, or it's a divergence — reported with exact
detail, not summarized away.

This is stronger than the "full depth" the brief's own wording anchors
on (per-price-level aggregate qty/count): comparing the individual order
population, not just the price/qty aggregate per level, would also catch
a bug where two different orders' quantities happened to sum to the
right total at the right price while their actual composition (who's
next in the queue) was wrong — the case `qty_at_price` alone would miss.

### 5.1 Reverse-engineering the snapshot cycle's real wire format

`MCX_Feeder.h` defines three templates for this — `SnapshotProductSummary`
(13600), `SnapshotInstrumentSummary` (13601), `SnapshotOrder` (13602) —
but, same story as `decoder`'s framing and price-scaling corrections,
the real capture didn't match the header file byte-for-byte, and had to
be checked against actual bytes:

- **`SnapshotOrder`'s real wire size is 40 bytes, not the 48 the first
  draft of this code assumed.** `decoder.rs` originally required
  `len >= 48` for template 13602, which is *larger* than every real
  13602 message (`sizeof` the actual struct: 8-byte header +
  `TrdRegTSTimePriority`(8) + `DisplayQty`(8) + `Side`(1) + `OrderType`(1)
  + `Pad6`(6) + `Price`(8) = 40). Every single 13602 in both real files
  therefore silently fell through to `DecodedMessage::Unknown` and was
  skipped — this was the first real bug this validation caught, not a
  hypothetical. Fixed to `len >= 40`.
- **`SnapshotOrder` carries no `SecurityID` of its own.** It inherits its
  instrument identity from whichever `SnapshotInstrumentSummary` (13601)
  most recently preceded it in the stream — confirmed by decoding real
  bytes and cross-checking that the count of `13602` records between two
  different instruments' `13601` headers always exactly equals the
  `TotNoOrders` field on the *first* of those two headers (verified over
  1,060 consecutive real cycles for CRUDEOIL with zero count mismatches,
  independently of any book comparison, before writing the Rust harness
  at all).
- **One instrument's order dump can span multiple outer records
  (packets).** A busy instrument's `TotNoOrders` can be in the hundreds
  or thousands (NATURALGAS alone reached ~2,500 resting orders); these
  don't all fit in one network packet, so the dump for one instrument
  continues, unheaded, across as many subsequent outer records as it
  takes, with the next instrument's own `13601` only appearing once the
  current one's orders are exhausted. The harness tracks "current
  instrument" purely from the *most recent* `13601` seen, regardless of
  how many outer records that spans.
- **Template `13603`** always immediately follows each `13601` one-to-one.
  At the time this was first written, its handful of fields looked static
  per session and were left decoding as `Unknown` since they didn't
  affect the FR-B11 comparison. **Update, a later task: this was
  half-right.** `13603` is `InstrumentInfo` — `references/MCX_Feeder.h`
  does define this struct (just files it under a different, wrong
  template id — see §3.2's account of the 13203/13603 discrepancy, and
  `decoder.rs`'s `InstrumentInfo` doc comment). Its fields *are* genuinely
  constant for an instrument whose DPR never revises intraday (true for
  CRUDEOIL — confirmed unchanged across all 8,025 real snapshot cycles),
  but not in general: NATURALGAS's real `13603` value changed six times
  over the session (§3.4). Now decoded for real and load-bearing for
  `book`'s own generic price-band mechanism (§3) — this harness's
  `collect_checkpoints` unions every valid one it sees per instrument and
  feeds the result to `BookBuilder::seed_band` before replay (§3.5).

### 5.2 Aligning two independently-broadcast streams by real wall-clock time

The increment and snapshot files are two *separate* broadcasts of the
same session — there's no shared sequence-number space to splice them by.
`13600`'s `LastMsgSeqNumProcessed` field *looks* like it should give a
cross-stream cursor, but was checked and rejected: for the same
`MarketSegmentID` at the same real time, its values (2, 339, ...) are
nowhere near the increment stream's own per-segment `ApplSeqNum` at that
moment (in the thousands) — they are not the same counter, and no
consistent relationship between them was found. Relying on it uninspected
would have been exactly the kind of unverified assumption this whole
project is built to avoid.

Instead, alignment uses **`PacketHeader.TransactTime`** — a real epoch
timestamp `decoder` already validated, stamped once per captured network
packet by the exchange's own feed engine. The harness streams the
increment file once, tracking this as its "current time," and streams
the snapshot file once beforehand into an ordered list of per-cycle
checkpoints (§5.1), each carrying its own cutoff
(`SnapshotInstrumentSummary.last_update_time` — also a real epoch
timestamp, confirmed empirically to sit consistently 18ms–490ms *before*
its enclosing packet's own `TransactTime`, exactly as "book last touched
at T, snapshot broadcast shortly after" should look). Every time the
increment stream's current time passes a checkpoint's cutoff, that
checkpoint is complete — every increment event that could affect it has
already been applied, in file order, before the comparison runs.

**Two things were tried and rejected before landing on packet-level
time, both found by running the harness against real data, not by
inspection alone:**

1. **Per-order business timestamps as the merge key.** Each `OrderAdd`/
   `OrderModify`/etc. carries `TrdRegTSTimeIn` (`event_time`) alongside
   `priority_ts`, and using it promised finer granularity than one
   timestamp per packet. It was **unsafe**: a real resting order found
   during validation (CRUDEOIL, `priority_ts` =
   1768585271701586294 — several days before the capture date, a
   genuine multi-day/GTC-style resting order) has its *own* `OrderAdd`'s
   `TrdRegTSTimeIn` set to `0xFFFFFFFFFFFFFFFF` — a sentinel, not a real
   time. Treated as a real timestamp, that sentinel is larger than every
   later checkpoint's cutoff, so encountering this one message caused
   the harness to finalize *every remaining checkpoint* in one shot
   against whatever the book happened to hold at that moment (empty, this
   early) — producing "book has 0 orders, snapshot expects thousands"
   across nearly the whole session. Business timestamps can encode
   history unrelated to *when this message was actually sent*;
   `PacketHeader.TransactTime` is a packet-transmission fact and doesn't
   have this failure mode. `decoder.rs`'s `event_time` fields are kept
   (still useful, e.g. for display/debugging) but are not used for
   alignment.
2. **A single global "current time" across the whole file.** Both
   capture files interleave *multiple market segments* (more than one
   product broadcast on the same stream/file). Updating "current time"
   from *any* segment's `PacketHeader` let an unrelated segment's
   clock — advancing independently, not in lockstep with our
   instrument's own segment — perturb the exact cutoff, producing small,
   persistent off-by-a-few-orders divergences concentrated right after
   the pre-market-to-live transition. Fixed by scoping the time cursor to
   the instrument's own `MarketSegmentID` — confirmed empirically that
   CRUDEOIL lives entirely on segment 294 and NATURALGAS entirely on
   segment 401 (91,239/91,239 and 42,298/42,298 sampled `OrderAdd`
   records respectively, zero mixing).

### 5.3 Bootstrap: seeding from the first checkpoint, not starting empty

A book built purely by replaying this session's increments, starting
empty, cannot ever match a snapshot that reflects orders which were
already resting **before this capture began** — orders carried over from
an earlier session (the multi-day/GTC order in §5.2's example) simply
have no `OrderAdd` anywhere in *this* file to replay. This isn't a gap to
paper over: it's D14's own real initialization path (`Uninit` → `Ok` via
a snapshot), which the harness has to exercise for the comparison to mean
anything. `MboBookImpl::bootstrap` seeds a book directly from a snapshot
cycle's resting-order list (sorting by `priority_ts` first, to reconstruct
correct FIFO position within each price level), bypassing `apply()`
entirely. The harness bootstraps from checkpoint 0 (the first, earliest
cycle) before replaying any increments, and skips any increment event for
that instrument at or before the bootstrap's own cutoff (already
reflected in the seed — reapplying it would double-count). For the
session validated here, checkpoint 0 happens to be the pre-market state
with zero resting orders, so the seed is a no-op in practice — but the
mechanism is real and necessary in general, and is exercised (and
tested — §6) regardless.

### 5.4 Why a second `[[bin]]` target instead of `main.rs` or `tests/`

T03's brief explicitly forbids editing `main.rs` (another agent wires
`book` in later). This crate has no `[lib]` target — every component is
a `#[path = "..."] mod` included directly from a binary's own root
(`main.rs` does this for `decoder`/`types`/`scheduler`/`refdata`; `book`
follows the same convention, using `crate::types::...` /
`crate::decoder::...`). That means a `tests/*.rs` integration test
(which can only see a crate's `pub` *library* surface) has literally
nothing to link against here, and `#[cfg(test)] mod tests` inside
`book.rs` itself (used for the synthetic unit tests, §6) only compiles
when *some* binary target declares `mod book;` — which nothing does
until `main.rs` is edited.

The only way to compile, run, and test this component's own code without
touching `main.rs` was a second `[[bin]]` entry in `Cargo.toml` pointing
at `src/book/validate.rs`, which itself declares
`mod types; mod decoder; mod book;` via the same `#[path]` convention.
Safe to delete once `book` is wired into `main.rs` for real and this
validation is folded into a normal integration test.

## 6. Unit tests

`book.rs`'s own `#[cfg(test)] mod tests` (run via
`cargo test --bin book-validate`, since that's the only binary target
that currently compiles `book.rs` — see §5.4) covers, with small
synthetic in-memory `DecodedMessage`s (no real files needed): add then
best bid/ask, a deliberately crossed book not panicking, modify losing
priority (moves to the back of the FIFO), modify-same-priority keeping
its FIFO slot, delete removing the exact matching order, mass-delete
clearing both sides, partial and full trades each reducing/removing only
the front order, bootstrap reconstructing correct FIFO order from
`priority_ts` regardless of insertion order (§5.3), and `BookBuilder`
routing by instrument while ignoring an unfiltered one, a trade hitting
a specific non-front resting order rather than the FIFO front, that
consumption cascading past the target into the *next* order when the
trade is larger than the target's remainder, and a trade whose target
isn't resting in this book at all falling back to the old FIFO-front
policy rather than being dropped. All 14 passed at the time this was
written.

**Update, a later task — 7 more, covering the generic price-band
mechanism (§3):** a real order-mutating event before any band is known
panics loudly (`order_before_any_band_known_panics_loudly`); a real
`InstrumentInfo` finalizes a `Pending` book
(`instrument_info_finalizes_a_pending_book`); one for an untracked
instrument is ignored
(`instrument_info_for_an_untracked_instrument_is_ignored`); the real,
empirically-found End-of-Day sentinel (`i64::MIN` fields) is rejected,
not trusted (`instrument_info_sentinel_garbage_is_rejected_not_trusted`);
a later, wider `InstrumentInfo` widens the book's dense array while
preserving every already-resting order on both sides
(`later_wider_instrument_info_widens_the_book_preserving_orders` — the
synthetic regression test for §3.4's real NATURALGAS finding); a later,
narrower one is a no-op, never shrinking
(`narrower_later_instrument_info_never_shrinks_the_band`); and
`seed_band` finalizes a book without any stream `InstrumentInfo` at all
(`seed_band_finalizes_a_book_without_any_stream_instrument_info` — the
synthetic regression test for §3.5's real CRUDEOIL gap). **21 pass** in
`book.rs` itself now (14 + 7); `book-validate`'s own test binary reports
**23** (the same 21, plus 2 pre-existing `refdata::tests` now reachable
through it — see §5.8's `mod refdata` addition for real tick-size
lookup).

### 5.5 A real trade-matching bug, found only because the harness ran to completion (first fix — superseded by §5.7)

NATURALGAS specifically showed one persistent phantom resting order at an
extremely active round-number price (₹310.80), invisible to the real
book but permanently stuck in the incremental one from the moment it was
added onward. Traced (via the diagnostic counters in §5.6) to `Trade`
handling: the original `apply_trade` matched the front (oldest-priority)
resting order at that price, per the business rule in §4, but blindly
subtracted the trade's reported quantity from it regardless of whether
that front order actually had that much left. On this specific,
extremely churny price level, one trade's reported quantity exceeded
what its (correctly FIFO-ordered) front order had remaining — driving
that order's quantity **negative** and leaving it wedged at the front
forever, blocking every later trade at that price from ever reaching the
orders actually being matched in reality.

The fix applied at the time made trade consumption **magnitude-based and
cascading**: a trade quantity larger than the front order's remainder
fully consumes that order and carries the remainder into the next one,
rather than trusting the wire's full/partial flag alone to decide
whether to remove the front order.

**This was a real improvement but not the actual root cause** — see
§5.7, found by continuing the FR-B11 investigation to its end rather than
stopping at the first passing-looking fix. The FIFO-front assumption
itself was wrong; §5.7 replaces it. The magnitude-based, cascading
*mechanics* of this fix were kept (see `apply_trade`'s doc comment) —
only *where* cascading starts changed.

### 5.6 Diagnostic counters: `MboBookImpl::diagnostics()`

Two counters, incremented (never used to change behavior) whenever
`remove_order`/`modify_same_priority` can't find the order they were
asked for, or `apply_trade` is asked to consume a price level with
nothing resting on it. Both should be exactly zero on a correctly
replayed book — a nonzero count means some earlier event was dropped or
misrouted, and the FR-B11 harness prints them alongside the cycle/
divergence counts for exactly this reason: §5.5's bug was found by this
counter going to 1 before the resulting divergence became visible
several hundred cycles later. `BOOK_DEBUG_MISSES=1` (env var) additionally
prints the offending price/side/priority and the full contents of the
level it was looked up in, to `stderr`, when a miss occurs.
`BOOK_DEBUG_PRIO=<u64>[,<u64>...]` and `BOOK_DEBUG_STOP_AFTER_CYCLES=<n>`,
used during this same investigation, trace every message touching one or
more specific `priority_ts` values and stop the replay early
respectively; `BOOK_DEBUG_PRICE=<raw>[,<raw>...]` (added while
root-causing §5.7 — `Trade` messages carry no `priority_ts` of their own,
so `BOOK_DEBUG_PRIO` alone can't show them) traces every message,
including trades, touching one or more specific raw prices. All four are
harness-only, env-gated, and inert unless set.

### 5.7 The real root cause: trades target a specific order, not the FIFO front

Fixing §5.5 reduced but did not eliminate NATURALGAS's divergences: 103
divergences remained across the full session (vs. CRUDEOIL's 0), plus 6
`remove_order`/`modify_same_priority` misses and 2 `apply_trade` misses
(diagnostics, §5.6) — all on NATURALGAS, none on CRUDEOIL, same code path
for both. Root-caused by tracing each of the 6 miss `priority_ts` values
individually through the full NATURALGAS increment file with
`BOOK_DEBUG_PRIO` (every `OrderAdd`/`OrderModify`/`OrderDelete` touching
that exact identity, in file order) and, separately, every message at a
suspect price with `BOOK_DEBUG_PRICE` (added during this investigation —
needed because `Trade` messages carry no `priority_ts` of their own, so
`BOOK_DEBUG_PRIO` alone can't show them).

**The finding:** `decoder::Trade`'s `event_time` field (wire offset 24,
named `TransactTime` in `references/MCX_Feeder.h`'s struct definition —
the offset itself is decoded correctly, exactly matching the header) does
**not** carry a wall-clock time on a real `13104`/`13105` record. Its
value is the *specific resting order's own* `priority_ts`
(`TrdRegTSTimePriority`) that this trade actually matched — the header's
field name is simply wrong for this template, the same class of
head-vs-real-bytes mismatch already found for `SnapshotOrder`'s size
(§5.1) and `OrderAdd`'s sentinel timestamp (§5.2).

Confirmed two ways on real NATURALGAS data:

1. **Direct, clean case.** `OrderAdd` adds a Sell order (qty 400,000,
   `priority_ts`=1768801652123314937). A long run of separate trades over
   the following ~40 seconds — quantities 10,000 / 30,000 / 10,000 /
   10,000 / 10,000 / 10,000 / 10,000 / 10,000 / 20,000 / 20,000 / 40,000 /
   ... — every one of them carries `event_time` = exactly
   1768801652123314937, i.e. every trade names this one order by its own
   `priority_ts`, regardless of how many *other* orders were added to the
   same price in between.
2. **The actual miss, explained.** One of the 6 known misses
   (`priority_ts`=1768801654028116641, added right behind the order in
   case 1 in arrival order — same price, 31400000000): no trade in the
   entire file ever carries `event_time` = this order's own
   `priority_ts`. It was never the real target of any trade. But the
   *old* FIFO-front logic doesn't know that — it blindly consumes
   whichever order sits at index 0 once the order ahead of it (case 1's
   400,000-lot order) is exhausted, which is exactly this order. The old
   logic ate into it anyway, eventually removing it outright — so when
   the real, legitimate `OrderDelete` for it finally arrived (an ordinary
   cancel, ~3 minutes later), it was already gone: the miss. The same
   shape explained all 6 known misses and the 103 divergences once
   checked individually — every one is a real order that a *different*
   order's trade stream ran into after its own intended target was used
   up, because the old code treated "next in the FIFO" and "the order
   the exchange says was hit" as the same thing. They are not, whenever
   more than one order rests at a price at once (rare enough on CRUDEOIL
   that its FR-B11 run never hit it in 8,024 cycles; routine on
   NATURALGAS, which reached ~2,500 resting orders on one side).

**Fix:** `apply_trade` now takes the matched order's `priority_ts`
(`t.event_time`, unchanged in `decoder.rs` — the value was always decoded
correctly, only `book`'s interpretation of it was missing) and looks that
exact order up in the level first. Consumption is still magnitude-based
and cascades (§5.5's mechanic, kept) — but starting from the *target's*
position in the FIFO, continuing into subsequent orders in arrival order
only if the trade size exceeds what the target has left, never touching
orders ahead of the target. If the matched `priority_ts` isn't resting in
this book at all (a legitimate rare case — e.g. an order from before this
replay's bootstrap window), the old FIFO-front policy is kept as an
explicit fallback rather than dropping the trade's effect on the book.
See `apply_trade`'s doc comment in `book.rs`, and the regression tests
`trade_targets_specific_order_by_event_time_not_fifo_front`,
`trade_cascades_from_targeted_order_not_from_front`, and
`trade_falls_back_to_fifo_front_when_target_not_resting` (the original
`trade_larger_than_front_order_cascades_to_next` from §5.5 still passes
unmodified — its trades use `event_time: 0`, which matches no resting
order in that test, so they exercise the same fallback path).

### 5.8 The harness updated for the generic price-band mechanism, plus a third instrument

Three real changes to `validate.rs`, all needed to keep FR-B11 running
against the new generic mechanism (§3) rather than the old hardcoded one,
none changing what the harness actually checks:

1. **Real, generic tick size.** `validate.rs` now declares
   `#[path = "../refdata/refdata.rs"] mod refdata;` and loads the real
   `19_01_2026` contract file itself in `main()`, looking up each
   instrument's `tick_size` the same way any other real caller would —
   not re-hardcoding the already-known CRUDEOIL/NATURALGAS values.
2. **Real band, sourced from the snapshot file's own `InstrumentInfo`
   stream.** `collect_checkpoints` (which already streams the paired
   snapshot file in full to build FR-B11 checkpoints) now also unions
   every valid `InstrumentInfo` it sees for the target instrument along
   the way, returning it alongside the checkpoint list.
   `validate_instrument` feeds that union to `BookBuilder::seed_band`
   before bootstrapping — a real, legitimate MCX channel (§3.5), not a
   hardcoded stand-in for what `apply` would have learned on its own from
   a feed that also carried the snapshot stream.
3. **A third case: ALUMINIUM (467731), proving generalization, not just
   describing it.** Added directly to `main()`'s `cases` array, reusing
   every part of the harness (checkpoint collection, bootstrap, packet-
   time alignment, per-order comparison) unmodified — the exact same code
   path CRUDEOIL/NATURALGAS go through. See §7a for how the real stream
   mapping was found and the full result.

## 7. Result of the real FR-B11 run

Run against the full `19_01_2026` session for both instruments (full
day, not a slice), streamed off disk record by record (§5.4's streaming
reader), with the bootstrap-plus-packet-time-alignment design from
§5.2/§5.3, and the target-order trade-matching fix from §5.7 (which
superseded §5.5's first, incomplete fix):

| Instrument | Snapshot cycles checked | Divergences | Diagnostic misses |
|---|---|---|---|
| CRUDEOIL (467013) | 8,024 | **0** | 0 remove/modify, 0 trade |
| NATURALGAS (465849) | 1,270 | **0** | 0 remove/modify, 0 trade |

Zero divergences across both instruments, full session, full order-level
depth (not just BBO, not just aggregate qty per level) — the FR-B11
acceptance bar. This is the real, final run (not a truncated slice): both
instruments streamed to completion (56,602,508 outer records for
CRUDEOIL, 242,321,672 for NATURALGAS), both counted to their file's last
snapshot cycle, both fully clean.

Before this, NATURALGAS had 103 divergences and 8 diagnostic misses (6
`remove_order`/`modify_same_priority`, 2 `apply_trade`) across the same
1,270 cycles, tracked down and fixed in §5.7 — every single one traced
back to the one root cause described there, not 103 independent bugs.
Getting to zero took five real, investigated bugs along the way (not
narrowed around to force a pass): the `13602` length guard (§5.1), the
sentinel-timestamp merge corruption (§5.2), missing bootstrap seeding
(§5.3), the trade-consumption negative-quantity cascade (§5.5, later
superseded), and the trade target-matching fix (§5.7) that finally
brought NATURALGAS's diagnostics and divergences to zero without
touching CRUDEOIL's (still 0/0, confirming the fix doesn't regress the
path that was already correct).

**Re-verified, a later task, via the generic price-band mechanism (§3),
same exact numbers:** with `band_config` gone — tick size sourced from
real `refdata`, band learned from the paired snapshot file's real 13603
stream (§3.6/§5.8) — a full re-run reproduced this table exactly:
CRUDEOIL 8,024/8,024 cycles, 0 divergences; NATURALGAS 1,270/1,270
cycles, 0 divergences; 0 diagnostic misses on both. The generic mechanism
is not merely "doesn't crash" — it reproduces the identical, previously
hand-verified correct result. See §7a for the third instrument this
mechanism now supports that the old one never could.

## 7a. Generalization proof: ALUMINIUM (467731), a genuinely new instrument

The actual point of this task: any FUTCOM instrument `refdata` knows
about should now be buildable, not just the two `band_config` used to
hand-pick. ALUMINIUM was the target — real tick size ₹0.05 (`refdata`'s
`TickSize` column reads `5` for every real ALUMINIUM row, i.e. 5 paise),
never supported by the old `band_config` (which would have panicked on
the very first real ALUMINIUM order under its 1-rupee fallback tick).

### Finding the real stream, the same way CRUDEOIL/NATURALGAS's mapping was found — not by trusting a column

The `19_01_2026` contract file lists five real ALUMINIUM (not the mini
"ALUMINI") native tokens across five expiries: 467731 (front month,
nearest expiry), 477166, 487656, 488790, 510472 (furthest). Per this
project's own established discipline (a prior naive guess based on the
contract file's own `StreamID` column was wrong for CRUDEOIL/NATURALGAS
— see §2's table), the real stream was found empirically: a full scan of
every `OrderAdd` (13100) record in **all five** real
`Increment_capture_19_01_2026_1_<1..5>.bin` files (163,953,436 +
268,810,618 + 480,131,812 + 56,602,508 + 242,321,672 = 1,211,820,046
outer records total) for these five tokens found:

| Stream | ALUMINIUM `OrderAdd`s found (any of the 5 tokens) |
|---|---|
| 1 | 0 |
| 2 | 0 |
| 3 | 0 |
| 4 (CRUDEOIL's own stream) | 0 |
| **5** (NATURALGAS's own stream) | **19,498** (510472: 1, 487656: 2,554, 488790: 36, 467731: 8,107, 477166: 8,800) |

All 19,498 real ALUMINIUM orders live on **stream 5** — the same
physical file pair already used for NATURALGAS — on `MarketSegmentID`
**358** (NATURALGAS's own segment is 401; the same stream multiplexes
multiple products, each on its own segment, exactly as §2's table already
established for this capture set). Sampled decoded prices cluster tightly
at ₹310–330/kg, a single plausible real ALUMINIUM range (not scattered
garbage), cross-checking both the token identity and the decode before
trusting either. 467731 (front month, 8,107 real orders) was chosen to
build a book for — enough real order flow for a meaningful FR-B11-style
check, not just a handful of orders.

### The book, built with the generic mechanism only — no hardcoding

Added directly as a third `book-validate` case (§5.8), reusing the exact
same harness code path as CRUDEOIL/NATURALGAS — same `BookBuilder`, same
bootstrap, same packet-time alignment, same per-order snapshot
comparison. Nothing about ALUMINIUM's tick size or band is hardcoded
anywhere in `book.rs` or `validate.rs`:

- **Tick size**, from real `refdata`: **5,000,000 raw = ₹0.05** — matches
  the contract file's `TickSize=5` (paise) exactly.
- **Price band**, learned from the real snapshot file's own
  `InstrumentInfo` (13603) stream for 467731: **[₹303.85, ₹329.15]** —
  matching the real order-flow price cluster (₹310–330) found above
  exactly, independent confirmation that both the token and the band are
  right.

### Result: a full FR-B11-style zero-divergence run, not just "didn't crash"

Same snapshot file already used for NATURALGAS (`mcx_feeder_snapshot_
capture_19_01_2026_1_5.bin`) turns out to carry ALUMINIUM's own snapshot
cycles too (its distinct-`SecurityID` list includes 467731 among many
other real tokens on the same physical stream) — so the *complete*
FR-B11 comparison was possible, not just a plausibility check:

| Instrument | Snapshot cycles checked | Divergences | Diagnostic misses |
|---|---|---|---|
| ALUMINIUM (467731, front month) | **1,270** | **0** | 0 remove/modify, 0 trade |

Full order-level depth, full session, zero divergences — the same
FR-B11 bar CRUDEOIL/NATURALGAS were held to, met by a genuinely new
instrument using only the generic mechanism this task built. This is
stronger evidence than the "replay and check a plausible BBO" minimum
the task allowed for — a real, complete correctness proof was available
and used.

## 8. What this component deliberately does not do

- ~~No `refdata`/DPR-bounds-driven price band~~ — **resolved, a later
  task** (§3): tick size is real and generic (`refdata::Instrument
  .tick_size`); price band is learned from a real `InstrumentInfo`
  (13603) in the applied stream, or seeded explicitly (`seed_band`) for a
  caller whose own feed can't supply one. Proved to generalize to a third
  instrument, ALUMINIUM, never supported by the old hardcoded mechanism
  (§7a).
- No strategy-declared instrument filter (M5/T05) — `BookBuilder` is
  still constructed with an explicit instrument list (now `(id,
  tick_raw)` pairs, §3.3); building it over an entire recording's
  instrument universe is exactly the runtime cost D32/FR-B16 exist to
  avoid, and this component still has no opinion on *which* instruments a
  caller should pick (that's `cache`'s `InstrumentFilter`, D32).
- No gap recovery / `Recovering` state — needs a live Transport, out of
  scope this milestone (FR-B10).
- No Cache, Scheduler wiring, dispatch, Simulated Exchange, or execution.
- No performance tuning beyond "don't do anything obviously wasteful" —
  `best_bid`/`best_ask`/`depth` scan the dense array linearly; at a few
  thousand ticks per instrument this is not what NFR-05 (M5) is about.
  `MboBookImpl::widen_band_if_needed` (§3.4) rebuilds the dense array
  when a real DPR revision widens it, which is O(band size), not O(1) —
  fine in practice (an intraday DPR revision is a rare event, observed
  six times for NATURALGAS across a full session, never on every message)
  but worth naming as a real, not "free," resize.

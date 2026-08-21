# T02 — MCX EOBI decoder

**Wave:** 1 (parallel with T01)
**Depends on:** T00 (workspace + shared types — done, see [OUTPUT_T00_structure.md](OUTPUT_T00_structure.md))
**Owns three files in one crate — nothing else:**
- `qtrade/crates/adapters/qtrade-adapter-mcx/src/wire.rs` — the EOBI wire structs, byte-exact from `MCX_Feeder.h`
- `qtrade/crates/adapters/qtrade-adapter-mcx/src/decode.rs` — file framing + the per-message dispatch loop, producing `WireMessage`
- `qtrade/crates/adapters/qtrade-adapter-mcx/src/normalize.rs` — `WireMessage` → `qtrade_types::Event`, the actual Normalizer boundary

`qtrade-adapter-mcx/src/refdata.rs` is T01's file — don't touch it. `qtrade-adapter-mcx/src/lib.rs` only wires the four modules together and is already written; you shouldn't need to touch it either.

**The three-file split matters, not just as a filing convention:** `wire.rs` and `decode.rs` are allowed to be as MCX-shaped as they need to be — nothing outside this crate ever sees them. `normalize.rs` is the boundary where that has to stop: its output type is `qtrade_types::Event`, already defined (see that crate's `lib.rs` — do not add a second, parallel event enum). If you find yourself wanting to add an MCX-specific field to `Event` to make normalization easier, that's the signal D32 warns about: the vocabulary should be defined by what the book builder needs, not by what MCX happens to send. Raise it rather than widening the type unilaterally.

---

## Context

MCX publishes market data as T7 EOBI — binary, little-endian, `#pragma pack(1)` structs, one `TemplateID` per message layout. `references/MCX_Feeder.h` already defines the exact struct layout for every template we need, verified against a real, working decoder. `references/MCX_Feeder.cpp` shows how a real system reads these from recorded capture files on disk and dispatches on `TemplateID`. Your job is to port the **decode** side of this into Rust — not the book-building side (that's T03), and not live multicast (out of scope this round, MCX-only backtest work).

## Required reading

- `../references/MCX_Feeder.h` in full. Ignore everything under the `// #else` branch (the `Price_Point_*` structs) — that's a dead, older MCX protocol guarded out by `#ifdef MCX_NEW_API` (which is defined), not what's in the recorded files you'll be reading.
- `../references/MCX_Feeder.cpp` lines 451–620 (`generate_feed` — the per-message dispatch entry point) and lines 1601–1679 (`Start_FileReplay` — the actual file-reading loop against recorded captures).
- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §M2 in full (FR-B04 Message dispatch, FR-B05 Order identity, FR-B06 Priority semantics, FR-B07 Normalizer)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) §4 "Verified protocol facts" for MCX EOBI, and the "Two findings that shaped the design" (no broadcast order ID; priority semantics published via `13101` vs `13106`)

## Ground truth already extracted — start here, don't re-derive it

**Outer file framing**, from `Start_FileReplay` (line 1636–1639): the recorded `.bin` files are a repeating sequence of records:

```
[8 bytes]  int64_t local capture timestamp, raw binary
[5 bytes]  ASCII decimal string (NOT binary!) giving the message length, parsed via atoi
[N bytes]  the raw EOBI message payload, N = the parsed length
```

**⚠️ This framing is read directly from the C++ but has not yet been empirically confirmed against a real file.** A first attempt (outside this task, using a quick Python script) to parse `mcx_feeder_Increment_capture_19_01_2026_1_1.bin` this way did not produce a valid 5-byte ASCII length field at the expected offset — the bytes there looked like binary garbage, not digits. **Your first concrete step must be to root-cause this before building anything on top of it.** Things to check, in rough order of likelihood:

1. An off-by-one or field-order error in the transcription above — re-read `MCX_Feeder.cpp` lines 1601–1679 yourself, byte by byte, rather than trusting this summary.
2. A leading file/session header before the very first record (some capture formats prepend a fixed-size preamble that isn't part of the per-record loop).
3. A different endianness or field width than assumed.
4. Whether `Increment` files and `Snapshot` files (both exist per stream, e.g. `mcx_feeder_snapshot_capture_19_01_2026_1_1.bin`) use the *same* framing — check both.

Do not proceed to inner-message decoding until you can point at a real file and show the outer framing loop consuming it cleanly end-to-end (every record's declared length lands you exactly on the next record's timestamp field, with no drift, until EOF).

**Inner message framing**, from `generate_feed` (line 1532, confirmed unambiguous): once you have one payload, it may contain several embedded EOBI messages back to back. Each starts with:

```rust
struct MessageHeader {
    body_len: u16,      // offset 0 — includes the header itself; this is your stride
    template_id: u16,   // offset 2
    msg_seq_num: u32,   // offset 4
}
```

Advance to the next embedded message by exactly `body_len` bytes. Template `13003` is a packet-level header (`PacketHeader` in the .h — carries `MarketSegmentID`, sets which product subsequent messages belong to) and is not itself a market event. Template `13001` is heartbeat.

**Struct layouts** for every other template — `13100` (OrderAdd), `13101`/`13106` (OrderModify / OrderModifySamePriority), `13102` (OrderDelete), `13103` (OrderMassDelete), `13104`/`13105` (Trade, full/partial execution), `13202` (ExecutionSummary), `13504` (TopOfBook), `13600`/`13601`/`13602` (snapshot messages — needed by T03, decode them here regardless) — are given verbatim, byte-for-byte, in `MCX_Feeder.h`. Port field types faithfully: several fields are `int64_t`/`uint64_t` even though the values they carry are small (e.g. `Side`, which is really 0/1/2) — **do not narrow these to smaller Rust integer types**, preserve the wire width exactly, even where it looks wasteful, since a future field revision could use the extra bits and a narrowed type would silently truncate it.

**Price/quantity scaling — verify before committing.** `generate_feed` divides raw wire price by `MCX_PRICE_MULTIPLIER = 1_000_000` and raw display quantity by `10_000` to get the values it actually uses. This does not match the EOBI spec text's claim of "integer including 8 decimals" cited elsewhere in this project's docs. **Treat the code as ground truth, not the spec summary** — but confirm it empirically: decode real Crude Oil / Natural Gas messages from a real file and check the resulting price is in a plausible rupee range (Crude Oil trades in the few-thousands; Natural Gas in the low hundreds) before locking this scaling into your types.

## Deliverable

**`decode.rs`:** given a path to one raw capture file (read-only), `decode_file` yields a stream of `WireMessage`s (already stubbed as an enum in `decode.rs` — extend it if a template you need is missing, don't replace it). Unknown/unhandled template IDs must be skipped cleanly using `body_len`, never treated as a fatal error (per FR-B04's acceptance criterion) — that's `WireMessage::Unknown`.

**`normalize.rs`:** turns each `WireMessage` into `Option<qtrade_types::Event>` (already stubbed — `normalize()` has a `todo!()` per message type; `OrderAdd`'s case is filled in as a worked example, follow its shape for the rest). This needs a `SecurityIdResolver` to turn a raw MCX `SecurityID` into an `InstrumentId` — the trait is already defined in `normalize.rs`; a real implementation (backed by T01's loaded instrument set) is test/wiring code, not something this module owns.

Per FR-B05: **do not model order identity as a plain integer ID field.** MCX has no broadcast order ID — `OrderDelete` publishes only `(SecurityID, Side, Price, TransactTime)`. `qtrade_types::OrderHandle` is already the composite-key type for this — construct it, don't invent a second one.

Per FR-B06: carry `priority_retained: bool` on `Event::OrderModified` — `true` when normalizing a `WireMessage::OrderModifySamePriority` (template `13106`), `false` for `WireMessage::OrderModify` (template `13101`). This falls out of which `WireMessage` variant you're matching on, not something to infer from field values.

## Out of scope

Book building of any kind (T03 — `qtrade-book`, a separate crate you should not need to touch or even read from). Instrument filtering by strategy predicate (later, once the Data Engine exists). Live multicast transport. CME/DGCX/Quincy. `qtrade-adapter-mcx/src/refdata.rs` (T01's file).

## Constraints

- **Read-only on `/mnt/*` and `references/*`** — no exceptions.
- Test against real files under `/mnt/MCX_Recording_Files/` (state which date and stream you used).

## Acceptance (this is FR-B04's real test, not a formality)

Decode a full real capture file end to end. Report a table of message counts per template ID. Assert internally that the sum of consumed bytes across all records/messages reconciles exactly with the file size (accounting for the outer framing overhead per record) — a byte-accounting mismatch means something in the framing or stride logic is wrong, and the milestone isn't done until this reconciles cleanly on a real file with zero panics and zero misparsed messages.

## Done when

- [ ] Outer file framing verified empirically against a real file (not just transcribed from C++) — document what you found, especially if it differs from the framing described above
- [ ] Inner message loop decodes every template ID listed above, skips unknowns cleanly
- [ ] Byte-count reconciliation passes on a real file
- [ ] Price/quantity scaling confirmed against plausible real values, documented
- [ ] `priority_retained` flag correctly set from `13101` vs `13106`
- [ ] `normalize()` produces `qtrade_types::Event` directly — no second event enum introduced anywhere

# Decoder — component documentation

**What this component does, in one sentence:** turns one raw MCX T7 EOBI message's bytes into a typed value you can print, inspect, or feed into a book — the one real decoding entry point, shared by every mode.

Code: [`decoder.rs`](decoder.rs) (this folder). No `[[bin]]` target of its own — every real consumer includes it as a module: `main.rs` (`[[bin]] qtrade`, via `feed_replay::replay`), and each component's own `*-validate` harness.

---

## 1. There is no standalone decode-only CLI anymore

Until 2026-08-25, `main.rs` was frozen as a minimal CLI whose only job was calling a now-removed `decoder::decode_file(...)` — read a whole capture file, print per-template-ID counts and (optionally) a per-message dump. That mode was retired once `main.rs` became the real orchestrator (`qtrade <config-file>` — see `../main_user_doc.md`): decoding is no longer something you ask for on its own, it's step one of every real run, done automatically. `decode_file` and its `Summary` return type are gone from `decoder.rs` entirely (confirmed, at removal time, to have no other real caller) — what's left, `decode_message`/`decode_messages` below, is unchanged and exactly as load-bearing as before.

## 2. The real API

```rust
pub fn decode_message(template_id: u16, seq: u32, m: &[u8]) -> DecodedMessage;
pub fn decode_messages(data: &[u8]) -> impl Iterator<Item = DecodedMessage> + '_;
```

- **`decode_message`** — the one real dispatch: given a template ID and one message's already-sliced-out bytes, returns the typed `DecodedMessage`. Every real consumer (`feed_replay::replay`, `book`'s own streaming validation, `cache`/`execution`'s test harnesses) reads the outer/inner wire framing itself (§3 below) and calls this directly once it has one message's bytes in hand.
- **`decode_messages`** — a convenience iterator over an already-loaded buffer, for a caller that has (or wants) the whole file in memory rather than streaming it record by record.

To just look at what's inside a real file without building anything else, the shortest path today is `feed-replay-validate` or any `*-validate` binary's own real-data run (see each component's own user doc) — there's no smaller-purpose tool than that anymore.

---

## 2. Which file it works on

**Input:** one raw MCX T7 EOBI capture file, e.g.:
```
/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_<DD>_<MM>_<YYYY>_1_<stream>.bin
/mnt/MCX_Recording_Files/mcx_feeder_snapshot_capture_<DD>_<MM>_<YYYY>_1_<stream>.bin
```

- `Increment_capture` files carry the live order-by-order stream (order add/modify/delete/trade).
- `snapshot_capture` files carry periodic full-book snapshots (used later for book validation, not yet consumed by anything here).
- `<stream>` is one of several parallel channels MCX splits its products across — each file is one stream, one day. A full trading day is spread across several of these files, decoded independently.

**This location is read-only.** The decoder only opens the file for reading (`std::fs::read`) — it never writes to, moves, or deletes anything under `/mnt/`.

---

## 3. How the packet framing works

This is the part that had to be reverse-engineered from the real bytes — the reference C++ (`references/MCX_Feeder.cpp`) describes something slightly different from what these actual files contain. What's below is confirmed: decoding a full 20GB file with this framing consumes *exactly* the number of bytes the file contains, zero drift, across 164 million records.

### 3.1 Outer framing — one "record" per captured network packet

The file is a back-to-back sequence of records, each shaped like this:

```
┌──────────────┬──────────────────┬─────────────────────────┐
│  8 bytes      │  8 bytes          │  payload_len bytes       │
│  u64, LE      │  i64, LE          │  raw EOBI message(s)     │
│  "length"     │  "capture_ts"     │                          │
└──────────────┴──────────────────┴─────────────────────────┘
```

- **`length`** — a plain 64-bit little-endian integer. It is **not** the payload size directly — it's `8 (the timestamp field) + payload_len`. Subtract 8 to get how many payload bytes follow.
- **`capture_ts`** — a local timestamp stamped when this packet was captured. **This is not a wall-clock date** — converting it to a calendar date gives 1970, because it behaves like a monotonic clock reading (arbitrary reference point, e.g. time since some internal clock start), not nanoseconds-since-1970. It's only meaningful for ordering records relative to each other, never for "what time did this happen" — for that, see `TransactTime` inside the `PacketHeader` message in §3.2, which *is* a real epoch timestamp.
- **`payload`** — the actual EOBI message bytes for this packet. May contain more than one message (see §3.2).

The decoder reads `length`, computes `payload_len = length - 8`, and jumps forward `8 (length field) + 8 (timestamp) + payload_len` bytes to find the next record. See `feed_replay::RecordSource` (`feed_replay.rs`) for the real streaming implementation, or `decode_messages()` in `decoder.rs` for the whole-buffer-in-memory equivalent.

### 3.2 Inner framing — one or more messages per packet

Each packet's payload is itself a sequence of individual EOBI messages, back to back:

```
┌────────────┬──────────────┬───────────────┬──────────────────┐
│  2 bytes    │  2 bytes      │  4 bytes       │  (body_len - 8)   │
│  u16, LE    │  u16, LE      │  u32, LE       │  bytes            │
│  body_len   │  template_id  │  msg_seq_num   │  message-specific │
└────────────┴──────────────┴───────────────┴──────────────────┘
```

- **`body_len`** is the *entire* message's size **including this 8-byte header** — it's the exact stride to the next message. This is what lets the decoder skip a message type it doesn't understand (see §4) without knowing its internal layout at all: jump `body_len` bytes and keep going.
- **`template_id`** says what kind of message this is (order add, trade, heartbeat, ...). See §4 for the full list.
- **`msg_seq_num`** is a per-stream sequence number, used later for gap detection (not yet implemented here — this component only decodes, it doesn't check for missing messages).

`13003` (`PacketHeader`) is special: it's the *first* message in nearly every packet and describes the packet itself (which product group — `MarketSegmentID` — the following messages belong to, and the real wall-clock `TransactTime`) rather than being a market event on its own.

---

## 4. What gets decoded

| Template ID | Rust type | What it means |
|---|---|---|
| `13003` | `PacketHeader` | Precedes almost every packet. Carries the real epoch `TransactTime`. Not a market event. |
| `13001` | `Heartbeat` | Venue had nothing to send; carries `LastSeqNo` for future gap detection. |
| `13100` | `OrderAdd` | A new resting order. |
| `13101` | `OrderModify` | An existing order changed — price moved, or quantity increased. **Loses queue priority.** |
| `13106` | `OrderModifySamePriority` | An existing order's quantity was *reduced* only. **Keeps queue priority.** |
| `13102` | `OrderDelete` | An order was removed. |
| `13103` | `OrderMassDelete` | Every resting order for an instrument removed at once. |
| `13104` / `13105` | `Trade` (`full: bool`) | A trade — full or partial execution against a resting order. |
| `13202` | `ExecutionSummary` | The aggregate view of a match event. |
| `13504` | `TopOfBook` | Best bid/ask snapshot (only published post-trading per the spec, not during continuous trading). |
| `13600` | `SnapshotProductSummary` | Opens a periodic full-book snapshot cycle (see `book_user_doc.md` §5). |
| `13601` | `SnapshotInstrumentSummary` | Opens one instrument's block within a snapshot cycle. |
| `13602` | `SnapshotOrder` | One resting order line within a snapshot cycle (real wire size 40 bytes, not 48 — see `book_user_doc.md` §5.1). |
| `13603` | `InstrumentInfo` | An instrument's daily price range (circuit limit) — `UpperDailyPriceLimit`/`LowerDailyPriceLimit`. **A real MCX-reference-material discrepancy, resolved against actual bytes, not assumed:** both `references/MCX_Feeder.h`'s own comment and the EOBI spec's own field-table description line say this message's template id is `13203` — checked against real capture bytes and found wrong; `13203` never appears anywhere in the real files, `13603` does, decoding to values that match already-independently-documented real DPR bounds exactly. See `InstrumentInfo`'s doc comment in `decoder.rs` and `book_user_doc.md`'s "generic price band" section (this is `book`'s real, generic price-band mechanism now, not just reference data). |

**Everything else — notably `13300`/`13301` (product/instrument state changes) — decodes as `UnknownMessage` and is safely skipped using `body_len`.** This isn't a bug: `references/MCX_Feeder.h` doesn't define their byte layout, so rather than guess at field offsets and silently misparse them, they're left unread. `decode_message()` in `decoder.rs` is the dispatch table — every `template_id` not listed above falls through to the `_ =>` case.

---

## 5. The two output views: Debug and Display

Every decoded message is a real Rust value (a struct or enum), never a string built ad hoc. It has two different text representations:

- **`Display`** (`{}`, the default) — a hand-written, one-line, human-readable summary. e.g. `ORDER_ADD Token=467014 Side=BUY Price=5400.00 Qty=1.00`.
- **`Debug`** (`{:?}`, via `--debug`) — Rust's automatic field-by-field dump. e.g. `OrderAdd { seq: 21, security_id: 467014, side: Buy, price: Price(540000000000), qty: Qty(10000) }`.

Reach for `--debug` when a `Display` line looks wrong and you need the exact raw values (e.g. the untouched wire integer before the price/qty scaling in §6 was applied) rather than the rounded, formatted version.

---

## 6. Price and quantity scaling

Raw wire integers are converted to human-readable values by two constants at the top of `decoder.rs`:

```rust
const MCX_PRICE_MULTIPLIER: f64 = 100_000_000.0;  // raw price / this = rupees
const MCX_QTY_DIVISOR: f64 = 10_000.0;             // raw qty / this = lots
```

**This was corrected during validation, not assumed.** The reference C++ (`MCX_Feeder.cpp`) divides price by `1,000,000`, which puts a real Crude Oil Mini order at an implausible ₹540,000. Cross-checking against the real contract file (token `467014` = CRUDEOILM) and the EOBI spec's own claim of "integer including 8 decimals" (÷`100,000,000`) gives ₹5,400 — the right order of magnitude, confirmed across many real bid/ask orders. Trust this constant, not the legacy code's.

---

## 7. What this component deliberately does not do

- No order book — this decodes messages, it doesn't maintain state across them.
- No instrument filtering — decodes every product in the file.
- No token → symbol lookup — prints `Token=467014`, not `CRUDEOILM` (done by hand during validation, not automated here).
- No gap detection despite carrying `msg_seq_num`/`LastSeqNo` — that logic doesn't exist yet.
- No file reading of its own at all — `decode_message`/`decode_messages` only ever see bytes a caller already has. Whether that caller streams record-by-record (`feed_replay::RecordSource`, used for real multi-GB files) or loads a whole buffer up front (`decode_messages`) is the caller's choice, not this component's.

These are scope boundaries, not missing features waiting to be discovered as bugs.

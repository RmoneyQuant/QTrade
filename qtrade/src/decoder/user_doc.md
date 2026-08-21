# Decoder — component documentation

**What this component does, in one sentence:** reads one raw recorded MCX T7 EOBI capture file, byte by byte, and turns every message inside it into a typed value you can print or inspect.

Code: [`decoder.rs`](decoder.rs) (this folder). Entry point: `qtrade/src/main.rs` calls `decoder::decode_file(...)`.

---

## 1. How to run it

From `qtrade/`:

```bash
source "$HOME/.cargo/env"     # once per new terminal, puts cargo/rustc on PATH
cargo build --release
./target/release/mcx-decoder <capture-file> [max-records-to-print] [skip-records] [--debug]
```

| Argument | Default | Meaning |
|---|---|---|
| `<capture-file>` | required | Path to a raw `.bin` recording (see §2 for what this file actually is) |
| `max-records-to-print` | 20 | How many outer records to print to screen. The whole file is always decoded and counted regardless — this only limits *printed* output |
| `skip-records` | 0 | Skip this many records before printing starts. Useful because the first few thousand records of a session are nothing but pre-market heartbeats |
| `--debug` | off | Print each message's full field dump (`{:?}`) instead of the one-line human summary (`{}`) — see §5 |

Example — print 5 real orders from partway into a real session:
```bash
./target/release/mcx-decoder /mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_1.bin 5 20000
```

Example — just get the correctness summary for the whole file, no per-message printing:
```bash
./target/release/mcx-decoder /mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_1.bin 0 0
```

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

The decoder reads `length`, computes `payload_len = length - 8`, and jumps forward `8 (length field) + 8 (timestamp) + payload_len` bytes to find the next record. See `decode_file()` in `decoder.rs`.

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
- Reads the whole file into memory — fine at this stage, not the eventual streaming design.

These are scope boundaries, not missing features waiting to be discovered as bugs.

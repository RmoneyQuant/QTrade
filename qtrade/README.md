# mcx-decoder — pilot

**Status: pilot succeeded.** This proved the MCX T7 EOBI byte layout is understood correctly, against real recorded data, before committing to building the full `qtrade` engine around it. Everything else in the project (`ARCHITECTURE.md`, `ARCHITECTURE-DECISIONS.md`, `BACKTEST-PHASE1.md`, `agent_tasks/`) describes the target system this pilot is one validated piece of.

## What it is

A single Rust program, one file (`src/main.rs`). Give it a raw recorded MCX EOBI capture file; it reads the file, decodes every message inside it, and prints them in plain English. That's the whole scope — no book building, no filtering, no live mode. Those come later, in the full build.

## How to run it

```bash
source "$HOME/.cargo/env"          # once per new terminal
cargo build --release
./target/release/mcx-decoder <capture-file> [max-records-to-print] [skip-records] [--debug]
```

- `max-records-to-print` (default 20) — how many outer records to print. The file is enormous (20GB / 164M records for a single day), so this is deliberately small by default; the whole file is still decoded and counted regardless, only *printing* is limited.
- `skip-records` (default 0) — start printing from this record onward. Useful because the very start of a session is nothing but pre-market heartbeats — see `sample_output.txt` for a worked example of picking a representative window instead.
- `--debug` — print each message's full field dump (`{:?}`) instead of the one-line human summary (`{}`). See "Debug vs Display" below.

Example:
```bash
./target/release/mcx-decoder /mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_1.bin 50 20000
```

`sample_output.txt` in this directory is a saved, annotated example run — open it directly rather than re-running the decoder if you just want to see what the messages look like.

## What it decodes

| Template ID | Message | Notes |
|---|---|---|
| 13003 | Packet Header | Precedes almost every packet; carries the real wall-clock `TransactTime`, not itself a market event |
| 13001 | Heartbeat | Carries `LastSeqNo`, used for gap detection later |
| 13100 | Order Add | |
| 13101 | Order Modify | Priority **lost** (price changed, or qty increased) |
| 13106 | Order Modify (same priority) | Priority **kept** (qty reduced only) |
| 13102 | Order Delete | |
| 13103 | Order Mass Delete | |
| 13104 / 13105 | Trade (full / partial) | |
| 13202 | Execution Summary | Aggregate match event |
| 13504 | Top Of Book | |

Everything else (notably **13300**/**13301**, product/instrument state changes) is decoded as `Unknown` and safely skipped by length — `references/MCX_Feeder.h` doesn't define their layout, so rather than guess at field offsets, they're left unread. This is correct behavior per the design docs' FR-B04 ("unknown template IDs must be skipped safely, never treated as an error"), not a bug.

## What this pilot found and validated

**The outer file framing had to be reverse-engineered from real bytes, not just read off the C++.** The literal reading of `MCX_Feeder.cpp`'s `Start_FileReplay` (an 8-byte timestamp then a 5-byte ASCII length) does not match what these actual recorded files contain. The real framing, confirmed by hex-dumping and cross-checking against known message sizes across multiple consecutive records with zero drift:

```
repeat:
  [8 bytes]  u64 LE — length of what follows (8-byte timestamp + payload)
  [8 bytes]  i64 LE — local capture timestamp (monotonic-looking; NOT a wall-clock date — don't try to convert it)
  [payload_len bytes] — one or more EOBI messages, each starting with
                          MessageHeader{body_len: u16, template_id: u16, msg_seq_num: u32}
                          and advancing by body_len (which includes its own header)
```

**Price scaling in the legacy code was wrong for these files.** `MCX_Feeder.cpp` divides raw wire price by `1,000,000`. Cross-checking a real order (token `467014`, confirmed via the real `MCXScrips.bcp` contract file to be CRUDEOILM — Crude Oil Mini) against that scaling gave an implausible ₹540,000. The original EOBI spec's own claim of "integer including 8 decimals" (÷ `100,000,000`) gives ₹5,400 — the right order of magnitude, and consistent across a run of real bid/ask orders. **The corrected constant is what's in the code now; the legacy C++ was misleading on this specific point.**

**The strongest evidence of correctness: exact byte accounting across the entire real file.** Decoding all 164,000,000+ records of a full 20GB trading day consumes exactly the number of bytes the file contains — zero drift, zero misparsed messages. A single wrong struct offset anywhere would have shown up as a mismatch at this scale; it didn't.

**`PacketHeader`'s `TransactTime` field converts to a real, correct date** — 2026-01-19 09:00:03 AM IST, effectively the exact instant MCX's market opens on that (confirmed) trading day.

## Debug vs Display — the convention for the rest of qtrade

Every decoded message is a real typed value (a struct or enum), never a string assembled ad hoc in the printing code. Two traits, two jobs, and this split is meant to carry forward into the full engine, not just live here:

- **`Debug`** (`#[derive(Debug)]`, printed with `{:?}`) — on every type, no exceptions. A mechanical field-by-field dump. Reach for this when something looks wrong and you need every raw value, not a summary.
- **`Display`** (hand-written `impl fmt::Display`, printed with `{}`) — only on types meant for a person to read. One line, human-readable (`ORDER_ADD Token=467014 Side=BUY Price=5400.00 Qty=1.00`).

The formatting logic lives on the type itself, once, rather than being reconstructed by whatever happens to be printing it that day. Every future consumer — this CLI, a log line, a test failure message, a journal-replay diff — gets the same representation for free, and there's exactly one place to fix if it's ever wrong.

## Known gaps (deliberate, not oversights)

- No book building — this only decodes messages, doesn't construct an order book (that's the next milestone).
- No instrument filtering — decodes every product in the file, not just Crude Oil / Natural Gas.
- No token→symbol lookup — prints `Token=467014`, not `CRUDEOILM`. Cross-checking against `MCXScrips.bcp` was done by hand for this pilot, not automated.
- `13300`/`13301` (session state changes) not decoded — layout not available in the reference code.
- Reads a whole file into memory — fine for a pilot, not the eventual streaming design.
- No tests — this was a correctness-by-construction exercise against real data (byte accounting, price cross-check), not unit-tested code.

None of these are bugs in what exists; they're explicitly out of scope for a pilot whose only job was proving the byte-level understanding is right before committing to the full build.

## Where things live

- `references/` (repo root) — the legacy C++ this was ported from. Read-only, never modify.
- `/mnt/MCX_Recording_Files/` — the real recorded data used to validate this. Read-only, never modify.
- `agent_tasks/` (repo root) — planning for the full `qtrade` build. Written before this pilot ran; the crate/module structure described there should be revisited in light of what this pilot found (the corrected price scaling and the real file-framing discovery in particular) before that work resumes.

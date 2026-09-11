# CME decoder — component documentation

**What this component does, in one sentence:** reads a CME feeder's **already-decrypted** recording file (`cme_feeder_<DD_MM_YYYY>.bin`) and prints every record in human-readable form, byte-for-byte identical to the vendor's own C++ reader.

Code: [`cme_decoder.rs`](cme_decoder.rs) (this folder). Own binary: `cme-decoder` (`[[bin]]` in `../../Cargo.toml`). Reference implementation it reproduces: `references/CME_Data_Check.cpp`. Not wired into `main.rs` — `types::Venue` still has only `Mcx`; this is a standalone reader, not yet a feed source.

---

## 1. What it is for

The decoder exists to answer one question: **does our Rust understanding of the CME record format match the vendor's?** It is a verification tool first and a scaffold second.

It does **not** decrypt anything. It does not connect to a feed. It does not write files. It reads one `.bin` from disk and prints text to stdout.

### 1a. The decrypted-recording pipeline (important)

There are two completely different `.bin` files in circulation, and reading the wrong one wastes days (it did — 2026-09-01 through 09-08):

```
CME multicast
     │
     ▼
CME_Recorder  ──writes──►  cme_feeder_capture_<Tier>_<DD_MM_YYYY>.bin
                           ENCRYPTED. Variable-length records. NOT readable here.
     │
     ▼
Multicast_Feeder_CME  ──decrypts, maintains a per-symbol book, writes──►
                           cme_feeder_<DD_MM_YYYY>.bin
                           DECRYPTED. Fixed 232-byte records. ◄── THIS is what we read.
     │
     ▼
cme-decoder (this component)  ──prints──►  readable text
```

| File name | What it is | Readable by this decoder? |
|---|---|---|
| `cme_feeder_capture_<Tier>_<date>.bin` | raw `CME_Recorder` output, the feeder's **input** | **No** — encrypted |
| `cme_feeder_<date>.bin` | the feeder's **output** | **Yes** |

Settled from `Multicast_Feeder_CME.cpp` itself, not from the naming convention: line 244 opens the `_capture_` name `"r"` (input), line 371 opens `cme_feeder_%s.bin` `"wb"` (output).

**So: the decoder currently consumes recordings that have already been decrypted by the vendor's feeder, and its job is to render them readable.** The decryption step runs elsewhere (it needs `libqedcrypt.a`, which is built against LibreSSL 2.1 and does not link against the OpenSSL 3.5.1 on our boxes).

### 1b. Each record is a full snapshot, not a delta

`Multicast_Feeder_CME.cpp` keeps a persistent `unordered_map<string, FEED_DETAILS> SymbolData` and writes the **entire current book** for a symbol on every update. So consecutive records for one symbol repeat all five levels even when only one changed. Nothing downstream needs to accumulate deltas.

One consequence worth knowing: at the start of a session the map is empty, so `LastTradedPrice`/`LastTradedQty` read `0` for a symbol until its first trade is observed. On `10_09_2026` that affected the first 8–2,920 records per symbol. Not a decode fault — the feeder genuinely had nothing yet.

## 2. Usage

```text
cme-decoder <path-to-cme_feeder_<date>.bin> [max-records]
```

```bash
cd /home/vaibhav/QTrade/qtrade

# five records to the screen
./target/release/cme-decoder ../references/ashok_cme_feeder_10_09_2026.bin 5

# page through it
./target/release/cme-decoder ../references/ashok_cme_feeder_10_09_2026.bin | less

# save a sample
./target/release/cme-decoder ../references/ashok_cme_feeder_10_09_2026.bin 5000 > sample.txt
```

Build it with `cargo build --release --bin cme-decoder`.

- **`max-records`** is the one intentional addition over the C++, which always prints the whole file. Omit it for exactly the C++'s behaviour.
- **Nothing is written to disk.** Output goes to stdout; redirect it yourself if you want a file.
- **A full trading day is ~103 million lines (≈2 GB of text).** `10_09_2026` holds 11,466,269 records. Always pass a limit or pipe to `less`/`head` unless you genuinely want that file.
- Piping to `head` is safe: a closed pipe exits 0 rather than erroring.

## 3. Output format

Nine lines per record, reproducing `CME_Data_Check.cpp` lines 150–161 exactly:

```text
RcTime:1789008961601762180
sz:224 payloadSize:216
----F.XNYM.CLV6------
1789008961762439:1789008961840305:1789008961840305
1-9621|9623-4
5-9620|9624-10
5-9619|9625-9
7-9618|9626-9
10-9617|9627-10
```

| Line | Content | Notes |
|---|---|---|
| 1 | `RcTime:<u64>` | our recorder's clock, **nanoseconds** |
| 2 | `sz:<nTot> payloadSize:<nTot-8>` | always `224` / `216` — the framing is fixed |
| 3 | `----<SymbolCode>------` | 4 dashes before, 6 after (the C++'s own format string) |
| 4 | `<Src>:<DS_rcv>:<DS_snd>` | vendor timestamps, **microseconds** |
| 5–9 | `<buyQty>-<buyPx>` \| `<sellPx>-<sellQty>` | five depth levels, top of book first; the two halves are separated by a literal `\|` |

**Note the column asymmetry on line 5–9:** the buy side prints quantity first, the sell side prints price first. That is what the C++ `printf` does, and it is reproduced deliberately rather than "corrected".

At EOF:

```text
EOF reached ApplSeqNum: 0
Total Packet Count:11466269 Missed:0
```

`ApplSeqNum` and `Missed` are declared but never assigned in the C++, so both are always `0`. Kept for output fidelity.

## 4. Value formats

This is the section to read before consuming any of these numbers.

### 4a. Prices — integers scaled by 100

`9621` means **$96.21**. Divide by 100. Applies to `BuyPrice`, `SellPrice`, `LastTradedPrice`. The decoder prints them raw, unscaled, exactly as the C++ does.

### 4b. Timestamps — mixed units, and this is the trap

| Field | Unit | Digits | Source |
|---|---|---|---|
| `RcTime` | **nanoseconds** | 19 | `clock_gettime(CLOCK_REALTIME)` in `CME_Recorder`, taken immediately after `recvfrom` |
| `SrcTimestamp` | **microseconds** | 16 | CME, via the QED library |
| `DS_rcv_timestamp` | **microseconds** | 16 | vendor's distribution server |
| `DS_snd_timestamp` | **microseconds** | 16 | vendor's distribution server |

`RcTime` is a thousand times finer-grained than the other three. **Any code differencing `RcTime` against a QED timestamp must promote the microsecond value to nanoseconds first** — multiply by 1000 — never divide `RcTime` down, which truncates and throws away real precision:

```rust
// correct: promote us -> ns, subtract in ns
let gap_ms = (rc_time_ns as i64 - ds_snd_us as i64 * 1000) as f64 / 1e6;

// wrong: truncates the sub-microsecond digits
let gap_ms = (rc_time_ns / 1000 - ds_snd_us) as f64 / 1000.0;

// wrong: float64 near 1.789e9 seconds has a 238 ns ULP; the
// subtraction quantises before you get an answer
let gap_ms = (rc_time_ns as f64 / 1e9 - ds_snd_us as f64 / 1e6) * 1000.0;
```

Skipping the conversion entirely yields a gap of roughly 1.787 × 10⁹ ms (about 56 years) — a useful smoke-test signature.

`RcTime` was confirmed to be stamped at *arrival*, not at write time, by disassembling `CME_Recorder`: `recvfrom@plt` → `clock_gettime@plt` with `clockid 0` → three `fwrite@plt` calls. It imports no `SO_TIMESTAMPING`, so these are software timestamps, not hardware ones.

### 4c. Order counts — `65535` is a sentinel

`NoOfBuyOrds` / `NoOfSellOrds` carry `65535` (`0xFFFF`) meaning **"not published"**. CME supplies order counts only for the top two levels; levels 3–5 are `65535` on essentially every record. Treat it as null, not as a count. (The parquet builder maps it to `-1`.)

The reference C++ never prints these fields, so this decoder doesn't either — but they are parsed and available on the struct.

### 4d. `SymbolCode` and the `F.` / `H.` prefixes

Symbols look like `F.XNYM.CLV6` and `H.XNYM.CLV6`. These are the **same events on two redundant channels**:

| Prefix | `Index` field | Multicast group | Tier |
|---|---|---|---|
| `F.` | 69 | 239.219.33.69 | Tier_1 |
| `H.` | 89 | 239.219.33.89 | Tier_2 |

An `F.`/`H.` pair shares an identical `SrcTimestamp`. **Any consumer must deduplicate**, or it double-counts every book update. On `10_09_2026` the counts were near-identical per contract (e.g. `F.XNYM.CLV6` 3,162,100 vs `H.XNYM.CLV6` 3,161,793).

The remainder of the symbol is CME's own scheme: `XNYM` = NYMEX, `CL` = crude oil, `NG` = natural gas, and the final characters are the contract month/year (`V6` = October 2026, `X6` = November 2026).

### 4e. `ExpiryDate` — epoch seconds

A plain `int32` Unix timestamp. `1793385000` = 2026-10-30 18:30 UTC = 2026-10-31 00:00 IST.

### 4f. The bitfield at offset 184

`LastTradedQty:12` and `FeedEventAggressor:2` share one `uint16_t`:

```rust
last_traded_qty      = bitfield & 0x0FFF;          // bits 0-11
feed_event_aggressor = (bitfield >> 12) & 0x3;     // bits 12-13
```

Observed aggressor values on real data: `0`, `1`, `2`, `3`. Neither field is printed by the reference reader; both are parsed here.

## 5. Record framing and struct layout

### 5a. Framing — fixed 232 bytes

`Multicast_Feeder_CME.cpp` lines 337–340 write three values per record:

```c
ssize_t nTot = sizeof(FEED_DETAILS) + sizeof(uint64_t);   // 216 + 8 = 224, constant
fwrite(&nTot,       1, sizeof(ssize_t), feedCapture);
fwrite(&RcTime,     1, sizeof(uint64_t), feedCapture);
fwrite(&itr->second, 1, sizeof(FEED_DETAILS), feedCapture);
```

So on disk: `[nTot:8][RcTime:8][FEED_DETAILS:216]` = **232 bytes, always**.

**A valid file therefore satisfies `size % 232 == 0`.** This is the single fastest integrity check on a transferred file, and it has already caught one truncated copy:

```bash
python3 -c "import os,sys;s=os.path.getsize(sys.argv[1]);print(s,'bytes |',s//232,'records |','OK' if s%232==0 else 'TRUNCATED')" <file>
```

The reader consumes `nTot` as a length prefix and then reads `nTot` bytes, of which the first 8 are `RcTime` — which is why line 2 of the output reads `sz:224 payloadSize:216`.

### 5b. `FEED_DETAILS` is NOT packed

This is the detail that silently produces garbage if missed. In `CME_Data_Check.cpp` the `#pragma pack(pop)` is at **line 85**; `FEED_DETAILS` is declared at **line 102**. It is therefore **naturally aligned**, not packed. (`CONTRACT_DETAILS`, the *other* struct in that file, is likewise unpacked — and is a different, older 292-byte format this decoder does not handle.)

Compiler-verified offsets:

| Offset | Field | Type |
|---|---|---|
| 0 | `SymbolCode[50]` | `char[50]` |
| *50–51* | *padding* | *(to 4-align `Index`)* |
| 52 | `Index` | `int32` |
| 56 | `ExpiryDate` | `int32` |
| 60 | `BuyPrice[5]` | `int32[5]` |
| 80 | `BuyQty[5]` | `uint32[5]` |
| 100 | `SellPrice[5]` | `int32[5]` |
| 120 | `SellQty[5]` | `uint32[5]` |
| 140 | `NoOfBuyOrds[5]` | `int32[5]` |
| 160 | `NoOfSellOrds[5]` | `int32[5]` |
| 180 | `LastTradedPrice` | `int32` |
| 184 | `LastTradedQty:12` + `FeedEventAggressor:2` | `uint16` bitfield |
| *186–191* | *padding* | *(to 8-align `SrcTimestamp`)* |
| 192 | `SrcTimestamp` | `uint64` |
| 200 | `DS_rcv_timestamp` | `uint64` |
| 208 | `DS_snd_timestamp` | `uint64` |
| | **total** | **216** |

Read this as packed and everything from `Index` onward is wrong — symbols will still look fine, which is what makes the mistake expensive.

## 6. Verification

Output is **byte-identical** to `CME_Data_Check`. That is the component's whole purpose, so it is tested as such rather than by unit-testing field extraction:

| Test | Result |
|---|---|
| `10_09_2026`, 100,000 records (900,000 lines) | identical, MD5 `481efe34e386e9a64e96d7430c6ba33d` |
| `08_09_2026`, 50,000 records (450,000 lines) | identical |
| 500-record file run to real EOF | identical, including the footer |
| Truncated mid-record file | identical error path |

Reproduce it yourself:

```bash
N=10000; L=$((N*9)); F=../references/ashok_cme_feeder_10_09_2026.bin
/home/vaibhav/CME_DecodeOldData/CME_Data_Check "$F" 2>/dev/null | head -n $L | md5sum
./target/release/cme-decoder "$F" $N | md5sum
```

Two identical hashes means the Rust framing, the struct offsets and the bitfield extraction all match the vendor's reference.

Throughput: the full `10_09_2026` day — 11,466,269 records, 2.66 GB in, ~2 GB of text out — takes **~12.6 s**. Output is `BufWriter`-buffered, which is where most of that comes from; `println!` per line would be several times slower.

## 7. What this component does not do yet

- **No decryption.** It reads the feeder's decrypted output only (§1a).
- **No deduplication** of the `F.`/`H.` redundant channels (§4d).
- **No scaling or unit conversion** — prices stay ×100, timestamps stay in their native units. Deliberate: it must print what the C++ prints.
- **Not wired into `main.rs`.** `types::Venue` has only `Mcx`. Making CME a real feed source means a `Venue::Cme`, a `feed_replay`-style source over these records, `×1000` at the µs→ns boundary, `/100` on prices, `F.`/`H.` dedup, and refdata for CME symbols.
- **`CONTRACT_DETAILS` (292-byte) records are not handled.** Only `FEED_DETAILS` (216-byte, `nTot == 224`). A future dispatch on `nTot` could support both.


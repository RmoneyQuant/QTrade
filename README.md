# qtrade

A backtester for MCX commodity futures. Replays a real recorded MCX market-data
capture file byte-for-byte, rebuilds the order book, and runs your strategy
against it with realistic queue position, order latency, and transaction
costs.

## 1. Clone and build

Requires Rust (`rustc 1.98.0`, edition 2021). If you don't have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Then:

```bash
git clone <this-repo-url>
cd QTrade/qtrade
cargo build --release
```

## 2. Run a backtest

qtrade ships with one working demo strategy (`order_lifecycle_demo`) and a
real config to run it against:

```bash
./target/release/qtrade configs/order_lifecycle_demo_21_08_2026.toml
```

That's the whole invocation — one argument, the path to a config file. A run
looks like this:

```
refdata: 17094 instruments loaded, filter admits 1 native ids, 1 of them resolved for order entry
... replay progress ...
--- report (Tier 1) ---
=== qtrade run report (Tier 1) ===
...
logs written:
  logs/qtrade/<timestamp>/events.log   (full event trail)
  logs/qtrade/<timestamp>/orders.log   (every order-state transition)
  logs/qtrade/<timestamp>/fills.log    (every fill)
  logs/qtrade/<timestamp>/report.txt   (P&L, costs, inventory)
```

A fresh, timestamped folder is created under `report_dir` (set in the config)
on every run — nothing is ever overwritten.

### The config file

```toml
[run]
mode = "backtest"
session_id = 1
recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_21_08_2026_1_4.bin"
report_dir = "logs/qtrade"
order_outbound_latency_ns = 250000   # order round-trip: 250us out
order_inbound_latency_ns = 250000    # + 250us back
max_feed_delta_ns = 250000000
log_level = "debug"

[deployment]
```

`recording_path` points at a real MCX capture file (`.bin`); swap it for any
other day and everything — instrument resolution, tick sizes, price bands —
follows automatically, no other change needed. Full key-by-key reference:
[`docs/write_strategy.md` §13](docs/write_strategy.md#13-the-config-file--every-key-explained).

For your first run, cap it so it finishes in seconds while you iterate:

```toml
max_outer_records = 2000000
```

## 3. Write your own strategy

qtrade is a library (`qtrade/src/lib.rs`), not just the one demo binary above.
Not yet published to crates.io — until it is, depend on it by path from your
own crate:

```toml
# your-strategy/Cargo.toml
[dependencies]
qtrade = { path = "../QTrade/qtrade" }   # adjust to wherever you cloned this repo
```

```rust
// your-strategy/src/main.rs
struct MyStrategy { /* ... */ }

impl qtrade::Strategy for MyStrategy {
    fn on_start(&mut self, ctx: &mut qtrade::StartCtx) { /* subscribe */ }
    fn on_book(&mut self, ctx: &mut qtrade::Ctx, id: qtrade::InstrumentId, ..) { /* trade */ }
}

fn main() {
    let config_path = std::path::Path::new("config.toml");
    qtrade::run_backtest(config_path, &["NATURALGAS"], MyStrategy::new()).unwrap();
}
```

**Full guide, everything the `Strategy` trait offers, order types, units, the
report fields, gotchas:** [`docs/write_strategy.md`](docs/write_strategy.md).

## Where things live

- `qtrade/src/lib.rs` — the library: `Strategy`, `Ctx`, `run_backtest`.
- `qtrade/src/main.rs` — the demo CLI binary (thin, calls into the library).
- `qtrade/src/strategy/` — reference strategies, read for patterns.
- `qtrade/configs/` — example run configs.
- `docs/write_strategy.md` — the strategy-authoring guide.
- `ARCHITECTURE.md`, `ARCHITECTURE-DECISIONS.md` — system design and the
  numbered decisions behind it.

## Status

Backtest only — no live trading, no non-MCX venues yet. See
[`docs/write_strategy.md` §17](docs/write_strategy.md#17-what-does-not-exist-yet)
for the full list of what's not built yet.

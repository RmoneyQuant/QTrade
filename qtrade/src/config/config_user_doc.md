# Config — component documentation

**What this component does, in one sentence:** parses the one declarative file (`qtrade <config-file>`) that drives a run — `[run]` (mode, dates, paths, session id — hashed into run identity) and `[deployment]` (connectivity/credentials — never hashed), per D22/D39.

Code: [`config.rs`](config.rs) (this folder). Entry point: `qtrade/src/main.rs` calls `config::load(path)`.

---

## 1. Why a config file, not CLI flags

`ARCHITECTURE-DECISIONS.md` D39 and `BACKTEST-PHASE1.md` §2.3 already specify this exact shape: one file, two sections, invoked as `qtrade <config>`. `[run]` is hashed into run identity because it affects results (dates, recording paths, the strategy set, latency model); `[deployment]` never is, because it only affects *where the process connects* (multicast endpoints, CTCL/ETI credentials — moving to a different colocation rack shouldn't invalidate a previous backtest's identity). Before this, `qtrade` (then `backtester`) took bare positional CLI args — this file replaces that with what the design already called for.

## 2. Schema

```toml
[run]
mode                  = "backtest"   # required. Only "backtest" is implemented;
                                      # anything else fails cleanly at startup --
                                      # this is where the CLI "asks" backtest vs live now.
session_id            = 1            # required. Feeds execution::RunConfig / ClOrdId (D40).
recording_path        = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin"
                                      # ONE of recording_path / recording_paths is required.
                                      # feed_replay derives that day's real MCXScrips.bcp
                                      # path from this filename (FR-16) -- no separate
                                      # contract_file field needed.
# recording_paths     = "/mnt/.../capture_21_08_2026_1_2.bin, /mnt/.../capture_21_08_2026_1_4.bin"
                                      # OR this: comma-separated list of same-day stream
                                      # files, k-way merged on exchange_ts before decode
                                      # (feed_replay_user_doc.md §2b). For a strategy whose
                                      # instruments live on different MCX streams the same
                                      # day (e.g. CRUDEOIL stream 2 + NATURALGAS stream 4).
                                      # recording_paths[0] resolves the day's MCXScrips.bcp
                                      # and wins any exact-timestamp tie. Having both keys,
                                      # or neither, is a hard error. One entry behaves
                                      # exactly like recording_path.
report_dir            = "logs/qtrade" # required. Parent of this run's own timestamped
                                      # output folder (feed.csv/orders.log/fills.log/report.txt).
max_outer_records     = 0            # optional, default 0 (no limit -- full file, start to end).
max_feed_stdout_lines = 200          # optional, default 200. How many feed.csv lines also
                                      # echo to stdout while a run is in progress.
latency_ns            = 100000       # optional, default 0. Flat, config-driven order/report
                                      # latency (dual-clock replay, 2026-08-27) -- applied
                                      # symmetrically to every OrderArrival/ReportDelivery
                                      # scheduled event. Default 0 is a legitimate value
                                      # (today's old zero-latency behavior, as a special
                                      # case) but any real run should set this explicitly.
                                      # A real probabilistic LatencyModel (D18) is separate,
                                      # later work -- this is one flat constant for now.
max_feed_delta_ns     = 250000000    # optional, default 250,000,000 (250ms). The Q1
                                      # outlier ceiling on recorder_ts - exchange_ts: a
                                      # negative delta, or one past this ceiling, is a hard
                                      # run failure (never clamped) -- see feed_replay's own
                                      # doc §2a for why a real capture file can trip this on
                                      # a bad day (recorded before ~2026-08-20).

[deployment]
# Inert today. No live feed source or exchange gateway exists in this
# codebase yet, so there is nothing real to put here -- keys inside this
# section are accepted and ignored, not rejected, so a file started now
# doesn't need restructuring once CTCL credentials/endpoints have a real
# consumer.
```

**Required: `mode`, `session_id`, `report_dir`, and exactly one of `recording_path` / `recording_paths`.** A missing required key, a missing `[run]` section entirely, or a malformed `key = value` line is a hard failure at startup with the file path and line number — same spirit as FR-16's contract-file mismatch: never a silent default standing in for a value that governs what the run actually does.

## 3. The parser is hand-rolled, not general TOML

This project has had zero external dependencies from the start (no `[dependencies]` in `Cargo.toml`). The schema above is small and fully under our own control — flat `key = value` pairs under `[section]` headers, no nested tables or arrays needed yet — so `config.rs` implements exactly this subset (`#` line comments, `"quoted strings"`, bare integers) rather than pulling in a TOML crate. Known, deliberate limitations versus real TOML: no escape sequences, no multi-line strings, and a `#` character inside a quoted value would be misread as starting a comment. None of the values this schema needs (paths, a mode name, small integers) require any of that.

## 4. What this component deliberately does not do

- Does not support multiple strategies yet (`[[run.strategy]]` as an array, per `BACKTEST-PHASE1.md`'s own example) — only one strategy (`naturalgas_bracket`, as of 2026-08-27) is plugged into `main.rs` at a time today, and which one is compiled in is a source-code edit to `main.rs`'s own `mod`/`use` lines, not a config field. Real multi-strategy config is D08's "a backtest run must declare its full strategy set," deferred until more than one real strategy exists.
- Does not expose `CostConfig`/`LocalOtrConfig` as config fields yet — `main.rs` still uses their `::default()`s. **Partially superseded, 2026-08-27**: `latency_ns`/`max_feed_delta_ns` (above) are real `[run]` fields now, the first two "run behavior" numbers to move out of a hardcoded default — but both are flat constants, not the eventual probabilistic `LatencyModel` (D18), which remains real future work.
- Does not populate any real `[deployment]` field — CTCL credential shape and MCX ETI endpoint details aren't pinned down yet, and there's no live-mode consumer to hand them to.

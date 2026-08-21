# Status tracker

**Single-writer file — updated from what actually got built; task files are read, not edited, to track progress.**

| Component | Milestone | Status | Notes |
|---|---|---|---|
| `types` | — | Not started | [T00_types.md](T00_types.md) |
| `refdata` | M1 | Not started | [T01_refdata.md](T01_refdata.md) |
| `decoder` | M2 | **Done, validated** | Real 20GB/164M-record file, byte-exact accounting. Needs one small follow-up API for `book` to consume — see [T02_decoder.md](T02_decoder.md) |
| `book` | M3 | Not started | [T03_book.md](T03_book.md) — the real gate, everything downstream depends on it |
| `scheduler` | M4 | Not started | [T04_scheduler.md](T04_scheduler.md) |
| `cache` | M5 | Not started | [T05_cache.md](T05_cache.md) |
| `simulator` | M6 | Not started | [T06_simulator.md](T06_simulator.md) — highest-risk component |
| `execution` | M7 | Not started | [T07_execution.md](T07_execution.md) |

## Environment

- Rust toolchain: `rustc`/`cargo` 1.98.0, via rustup, user-local under `~/.cargo`.
- Project: `/home/vaibhav/QTrade/qtrade/` — single Cargo package (not a workspace), builds as `mcx-decoder`.
- Convention: one folder per component under `qtrade/src/`, each holding `<component>.rs` + `<component>_user_doc.md`.
- Real recorded data confirmed available (read-only) at `/mnt/MCX_Recording_Files/` — increment + snapshot captures per stream, `CONTRACT/<date>/MCXScrips.bcp`. Validated date so far: `19_01_2026`.

## Superseded

An earlier multi-crate plan (separate `qtrade-types`/`qtrade-refdata`/`qtrade-book`/`adapters/qtrade-adapter-mcx` crates in a Cargo workspace) was built once, then abandoned in favor of the single-package, folder-per-component convention above. Nothing from that plan applies anymore.

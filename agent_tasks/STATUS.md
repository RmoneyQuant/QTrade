# Status tracker

**Single-writer file — I update this from agent reports; agents read it, they don't write it.**

| Task | Wave | Status | Notes |
|---|---|---|---|
| T00 — project structure | 0 | **Done (v2 — restructured by architectural layer, not by task)** | See [OUTPUT_T00_structure.md](OUTPUT_T00_structure.md). First pass organized crates by task and leaked MCX-specific types into the exchange-agnostic book builder; corrected on review. Awaiting your review before Wave 1 starts. |
| T01 — MCX refdata | 1 | Not started | Owns `qtrade-refdata/` (generic) + `qtrade-adapter-mcx/src/refdata.rs` (MCX-specific). Blocked on your approval of T00. |
| T02 — MCX decoder | 1 | Not started | Owns `qtrade-adapter-mcx/src/{wire,decode,normalize}.rs`. Blocked on your approval of T00. Carries forward the unresolved file-framing question. |
| T03 — MCX book builder | 2 | Not started | Owns `qtrade-book/` (standalone, no adapter dependency). Depends on the `qtrade_types::Event` shape, not on T02's code directly. |

## Environment

- Rust toolchain: installed this session — `rustc 1.98.0`, `cargo 1.98.0`, via `rustup`, user-local under `~/.cargo`.
- Workspace: `/home/vaibhav/QTrade/qtrade/`, builds clean (`cargo build`).
- Real recorded data confirmed available (read-only) at `/mnt/MCX_Recording_Files/` — increment + snapshot captures per stream, `CONTRACT/<date>/MCXScrips.bcp`.

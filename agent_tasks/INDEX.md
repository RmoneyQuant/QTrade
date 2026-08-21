# Backtest Phase 1 — component task plan

**Status: PLANNING ONLY.** Nothing beyond `decoder` (already built and validated) has been scaffolded. This supersedes the earlier multi-crate plan entirely — that approach (separate Cargo crates, `qtrade-types`/`qtrade-refdata`/`qtrade-book`/`adapters/qtrade-adapter-mcx`) is abandoned. See §1.

## 1. The convention — one package, one folder per component

**Single Cargo package** (`qtrade/`, already exists, builds as `mcx-decoder`) — not a workspace, not multiple crates. Every component is a folder under `qtrade/src/`, holding exactly two files:

```
qtrade/src/<component>/
├── <component>.rs           the code
└── <component>_user_doc.md  what it does, how it runs, how it works, key functions
```

`decoder` is the reference example — already built this way:

```
qtrade/src/decoder/
├── decoder.rs
└── user_doc.md
```

(Note: `decoder`'s doc file is named `user_doc.md`, not `decoder_user_doc.md` — that was before this exact naming was settled. Every component from here on uses `<component>_user_doc.md`, matching what's specified below.)

`main.rs` stays a thin entry point wiring components together — never where logic lives.

## 2. Why this shape, not crates

We tried the crate-per-layer version once (see git history / your own memory of that round) — it added real ceremony (workspace manifests, `cargo tree` verification, cross-crate dependency rules) for a project that's currently one person/agent working sequentially, not parallel teams needing a compiler-enforced boundary. NFR-06 ("adding a venue requires no change below the Normalizer") is a real requirement, but its actual test is Stage 10 — when a **second venue** (Quincy/CME) arrives. Until then, a folder boundary plus "book never imports from decoder" as a convention costs nothing extra to enforce by hand and costs a lot less to look at.

## 3. Component list — one per BACKTEST-PHASE1.md milestone

| # | Component | Milestone | Status |
|---|---|---|---|
| — | `types` | shared vocabulary, cross-cutting | Not started |
| M1 | `refdata` | Reference data & instrument taxonomy | Not started |
| M2 | `decoder` | MCX T7 EOBI decoder | **Done, validated against real 20GB data** |
| M3 | `book` | Book builder | Not started |
| M4 | `scheduler` | Scheduler and clock | Not started |
| M5 | `cache` | Cache, filter and dispatch | Not started |
| M6 | `simulator` | Simulated Exchange | Not started |
| M7 | `execution` | Execution, accounting and reporting | Not started |

Task briefs: [T00_types.md](T00_types.md) · [T01_refdata.md](T01_refdata.md) · [T02_decoder.md](T02_decoder.md) (status, not a build task) · [T03_book.md](T03_book.md) · [T04_scheduler.md](T04_scheduler.md) · [T05_cache.md](T05_cache.md) · [T06_simulator.md](T06_simulator.md) · [T07_execution.md](T07_execution.md)

Build order matches milestone order — each gates the next, per [BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §3's own dependency chain. `types` comes first because `refdata` and `book` both need it.

**Not a component, do it anytime in parallel, needs no code:** [BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) §7 — compute the minimum quotable spread from historical MCX Crude spreads against the cost model. Spreadsheet exercise. Answers whether the whole phase-1 premise holds before more engine gets built.

## 4. Safety rules — unchanged, still absolute

- **Read-only:** `/mnt/MCX_Recording_Files/`, `/mnt/CME_Recording_Files/`, `/mnt/DGCX_Recording_Files/`, `/mnt/Contact_Files/`, and `references/*`. Never write, move, or delete anything there.
- Rocky Linux 9.7. `cargo build` / `cargo run` from `qtrade/`.

## 5. What's actually true right now, so nothing gets assumed stale twice

- Rust toolchain installed (`rustc`/`cargo` 1.98.0, user-local via rustup).
- `decoder` is real, built, and validated: byte-exact accounting across a full 164M-record/20GB real file, corrected price scaling (÷10^8, not the legacy code's ÷10^6), and the outer file-framing reverse-engineered from real bytes (documented in `decoder`'s own user doc, not repeated here).
- `decoder` currently only **prints** — it has no public API returning a stream of decoded messages to a caller. `book` (T03) needs that. See T02's brief for the specific follow-up.

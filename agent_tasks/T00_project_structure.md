# T00 — Project folder structure & workspace scaffold

**Wave:** 0 (runs alone, first)
**Depends on:** nothing
**Blocks:** T01, T02, T03 — nothing else starts until this lands

---

## Context

The repo currently holds only design documents (`ARCHITECTURE.md`, `ARCHITECTURE-DECISIONS.md`, `CONTEXT.md`, `BACKTEST-PHASE1.md`, `STRATEGY-GUIDE.md`, `OPEN-QUESTIONS.md`), some diagrams, and a `references/` folder of legacy C++ used only for reading. No code exists yet. Your job is to decide where the actual Rust project lives and how it's divided into crates, then scaffold it so it compiles.

This round is **MCX-only** (see [../agent_tasks/INDEX.md](INDEX.md) §1) — do not create crates or modules for CME, DGCX, Quincy, live trading, the Simulated Exchange, ExecutionEngine, or strategies. The layout must not *preclude* adding those later (that's NFR-06 in the architecture — adding a venue must not require restructuring), but don't build for them now.

## Required reading

- [../ARCHITECTURE.md](../ARCHITECTURE.md) §3 (system structure — three layers) and §4.1–4.8 (Transport, Decoder, Normalizer, Data Engine, BookBuilder)
- [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) D01 (Rust), D04 (single-threaded qtrade, adapter threads), D32 (instrument filter placement), D37 (instrument taxonomy)
- [../BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) — read in full; §3 Milestones M1–M3 are what this workspace must support building next
- [../CONTEXT.md](../CONTEXT.md) — glossary. Use its terms exactly (`qtrade`, Transport, Decoder, Normalizer, Data Engine, BookBuilder, Cache, Instrument, Order Book) — do not invent synonyms.

## What to decide and produce

1. **Where the project root lives.** Propose a top-level directory name (e.g. `qtrade/`) inside `/home/vaibhav/QTrade/` for the actual Cargo workspace, separate from the existing docs at repo root.
2. **Crate boundaries for what's in scope this round**, at minimum:
   - A shared types crate (`Price`, `Qty`, `InstrumentId`, `Side`, an abstract `OrderHandle` — MCX has no broadcast order ID, see [../ARCHITECTURE-DECISIONS.md](../ARCHITECTURE-DECISIONS.md) §4 "Two findings that shaped the design", point 1 — and the `InstrumentKind` enum from D37, `Future` variant only implemented, others as unimplemented stubs per the taxonomy).
   - A reference-data crate for the MCX contract loader (feeds T01).
   - A decoder crate for MCX EOBI (feeds T02).
   - A book crate for the `Book`/`MboBook` traits and the MCX book builder (feeds T03).
   - Don't over-split — four or five crates is the right order of magnitude for this round, not sixteen.
3. **A compiling stub workspace.** Every crate should exist with a `Cargo.toml`, real public function/struct signatures matching your design, and `todo!()` bodies where logic isn't yours to write. `cargo build` must succeed on the whole workspace before you're done. This is what lets T01 and T02 start against an agreed, checked API instead of a description of one.
4. A short `OUTPUT_T00_structure.md` in this `agent_tasks/` folder: the folder tree, one paragraph per crate on its responsibility, and which later task (T01/T02/T03) owns which path.

## Constraints

- **Never write to `/mnt/*` or `./references/*`.** Read-only, no exceptions (see [INDEX.md](INDEX.md) §3).
- Rocky Linux 9.7. Use `cargo`/`rustup` conventions, not anything Windows-specific.
- If `rustc`/`cargo` are not on `PATH`, check with the user before installing anything system-wide — installing Rust via `rustup` to `$HOME/.cargo` is fine and expected if needed; do not use `sudo` or touch system package management.
- Do not modify any file at the repo root (`ARCHITECTURE.md`, etc.) — this task only creates new files under the new project directory and under `agent_tasks/`.

## Out of scope

CME/DGCX/Quincy adapters, live transport, Simulated Exchange, ExecutionEngine, RMS, Cost Model, Scheduler/Clock, Cache, reporting, any strategy trait implementation. Stub or omit entirely — do not scaffold empty crates for these yet; that's premature and just noise for T01–T03 to read past.

## Acceptance

- `cargo build` succeeds from the workspace root with zero errors.
- `OUTPUT_T00_structure.md` exists and clearly assigns ownership of each crate directory to T01, T02, or T03.
- No file under `/mnt/` or `references/` has a modified timestamp after this task started.

## Done when

- [ ] Workspace scaffolded, compiles clean
- [ ] `OUTPUT_T00_structure.md` written
- [ ] Crate ownership for T01/T02/T03 is unambiguous

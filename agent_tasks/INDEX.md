# Phase 1 — MCX-only build: agent task plan

**Status: PLANNING ONLY. No agent has been dispatched. Nothing outside this folder has been created yet.**

This folder exists so you can review and adjust the task breakdown before any agent touches a keyboard. Once you approve, I dispatch agents per the wave plan below and track progress in `STATUS.md`.

---

## 1. Scope of this round

**MCX only.** No CME, no DGCX, no Quincy — those are explicitly deferred, per your instruction. This round targets the MCX-only slice of Backtest Phase 1: reference data, the EOBI decoder, and the book builder — roughly [BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) milestones M1–M3.

Not in this round: Scheduler, Cache/Dispatch, Simulated Exchange, ExecutionEngine, reporting, any strategy code. Those come in later waves once M1–M3 are proven against real data.

## 2. What makes this round different from a cold start

We now have real ground truth that didn't exist when [BACKTEST-PHASE1.md](../BACKTEST-PHASE1.md) was written:

- **Real recorded MCX EOBI data**, daily, at `/mnt/MCX_Recording_Files/` — increment and snapshot captures per stream, plus the daily contract file (`CONTRACT/<date>/MCXScrips.bcp`).
- **Working legacy C++ reference code** in `./references/`: `Contract.cpp` (multi-exchange contract-file parser — only its `EXCHG_MCX` branch is in scope) and `MCX_Feeder.h`/`.cpp` (exact wire struct layouts for every EOBI template ID we need, plus a legacy — and NOT to be copied — book-building implementation).

Every task below is a **port of specific, cited sections of this legacy code into Rust**, not a re-derivation from the spec text alone. Agents should treat the C++ as ground truth where it's unambiguous, and treat the spec docs (`ARCHITECTURE.md`, `ARCHITECTURE-DECISIONS.md`, `BACKTEST-PHASE1.md`, `CONTEXT.md`) as the design authority for how the ported logic should be shaped in the new system.

## 3. Non-negotiable safety rules — every task, every agent, no exceptions

- **`/mnt/MCX_Recording_Files/`, `/mnt/CME_Recording_Files/`, `/mnt/DGCX_Recording_Files/`, `/mnt/Contact_Files/` are READ-ONLY.** Never write, move, rename, truncate, or delete anything under `/mnt/`. Opening files for reading is fine and expected.
- **`./references/*` (readme.md, Contract.cpp, MCX_Feeder.h, MCX_Feeder.cpp) is READ-ONLY.** It is reference material to read and port from, never to edit.
- **This is Rocky Linux 9.7, not Windows.** Use Linux path conventions (`/`, not `\`), and standard Linux/Rust tooling (`cargo`, `rustup`). Don't assume a `D:\` drive or PowerShell exists.
- Any task that needs to read a dated example file should pick one date directory under `/mnt/MCX_Recording_Files/CONTRACT/` and the matching `.bin` captures for that same date, and say explicitly which date it used, in its output.

## 4. Coordination model

Agents can't talk to each other directly. I'm the coordination point — I dispatch them, receive their reports, and write the single shared `STATUS.md` (agents read it, only I write it, so there's no concurrent-write hazard). Each task **owns an exclusive directory** inside the new project root (finalised by T00) — no two tasks ever edit the same file.

## 5. Wave plan

```
Wave 0   T00 — Project structure & workspace scaffold        (alone)
             │
             ▼ (structure + qtrade-types finalised)
Wave 1   T01 — MCX reference data / contract loader      T02 — MCX EOBI decoder
         (parallel — both depend only on T00, not on each other)
             │
             ▼ (T02's decoded event types available)
Wave 2   T03 — MCX Book Builder + snapshot validation
```

T01 and T02 can run concurrently once T00 lands, because reference data and wire decoding don't depend on each other — they both depend only on the shared types T00 defines. T03 needs T02's output.

## 6. Task briefs

- [T00_project_structure.md](T00_project_structure.md)
- [T01_mcx_refdata.md](T01_mcx_refdata.md)
- [T02_mcx_decoder.md](T02_mcx_decoder.md)
- [T03_mcx_book_builder.md](T03_mcx_book_builder.md)

## 7. Open items I need your call on before I dispatch anything

1. **Rust toolchain isn't installed on this machine yet** (confirmed earlier this session — no `rustc`/`cargo`). Options: (a) I install it now via `rustup` before any agent starts, so every task can actually run `cargo build`/`cargo test` and self-verify; (b) T00's agent installs it itself as its first step; (c) you install it yourself on your own schedule. Without it, agents produce code nobody has compiled — which is the exact failure mode I flagged earlier as the biggest risk to unverified output.
2. **Should T00 only produce a design doc, or also scaffold a compiling stub workspace** (empty crates with real signatures, `todo!()` bodies, `cargo build` green)? I'd recommend the latter — it's what lets T01/T02 start against a real, agreed API instead of a paper one.
3. **Agent isolation** — I can run each task's agent in its own git worktree so concurrent agents (T01 + T02 in Wave 1) can't collide on the working tree even before their crate directories are finalised. Want that, or is sequential (one task at a time) simpler for now given this is the first real round?
4. **One thing worth deciding explicitly, not by default:** T02's brief below flags that my own attempt to apply the file-framing logic I read out of `MCX_Feeder.cpp` didn't immediately parse cleanly against a real file. I've left that as an open verification step for T02 rather than guessing further — flagging it here so it's not a silent gap in the plan.

Tell me what you want changed, and I'll revise before anything runs.

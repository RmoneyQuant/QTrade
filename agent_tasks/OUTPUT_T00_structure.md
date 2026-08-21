# T00 output — project structure (v2, restructured by architectural layer)

**Status: scaffolded and compiling. Done directly, not by a dispatched agent, per your instruction to review this step yourself before other agents start.**

**This supersedes the first version of this document.** The first pass organized crates by *which task built them* (`qtrade-mcx-refdata`, `qtrade-mcx-decoder`, `qtrade-mcx-book`) — which is exactly the thing that would need restructuring the moment this goes to production or a second venue arrives, because it baked "MCX" into components that the architecture already says must be exchange-agnostic. This version organizes by the layering [ARCHITECTURE.md](../ARCHITECTURE.md) §3.1 already defines, so nothing here needs to move later — new venues and new core capabilities are purely additive.

## The layering, and why it's the right axis to organize by

qtrade's own design has three layers with one hard rule holding them apart (NFR-02): **"No component below the Normalizer may branch on run mode [or venue]."** That's not a coding guideline here — it's now a fact `cargo` enforces, because of how the crates depend on each other:

```
Layer 1+2 (Transport, Decoder, Normalizer)   — per venue, one crate per venue
Layer 3   (BookBuilder, reference data, ...) — exchange-agnostic, shared
```

A venue's crate depends on the shared vocabulary crate. Core crates depend on the shared vocabulary crate. **Core crates never depend on a venue crate.** That last arrow simply doesn't exist in `Cargo.toml` anywhere — so a core crate accidentally importing an MCX-specific type isn't a discipline someone has to remember, it's a compile error.

## The tree

```
QTrade/
├── ARCHITECTURE.md, ARCHITECTURE-DECISIONS.md, ...   ← untouched
├── references/                                        ← untouched, read-only
├── agent_tasks/                                        ← this planning folder
└── qtrade/                                             ← the Cargo workspace
    ├── Cargo.toml                        workspace manifest, 4 members
    ├── Cargo.lock                        tracked (determinism matters here — D22)
    ├── .gitignore                        only /target ignored
    ├── README.md
    └── crates/
        ├── qtrade-types/                 ← shared vocabulary. NO dependencies. Frozen.
        │   └── src/lib.rs
        │
        ├── qtrade-refdata/               ← CORE, venue-agnostic. Depends only on qtrade-types.
        │   └── src/lib.rs                    InstrumentMaster + query builder — OWNED BY T01
        │
        ├── qtrade-book/                  ← CORE, venue-agnostic. Depends only on qtrade-types.
        │   └── src/lib.rs                    Book/MboBook traits + MboBookImpl — OWNED BY T03
        │
        └── adapters/
            └── qtrade-adapter-mcx/       ← VENUE-SPECIFIC (Layer 1+2 for MCX). Depends only on qtrade-types.
                └── src/
                    ├── lib.rs             module wiring only — mine, not T01/T02's
                    ├── refdata.rs         MCXScrips.bcp -> Vec<Instrument>          — OWNED BY T01
                    ├── wire.rs            EOBI wire structs, verbatim from MCX_Feeder.h  — OWNED BY T02
                    ├── decode.rs          file framing + dispatch -> WireMessage        — OWNED BY T02
                    └── normalize.rs       WireMessage -> qtrade_types::Event            — OWNED BY T02
```

Verified: `cargo build` from `qtrade/` succeeds (one harmless `dead_code` warning on a struct field unused until T03 fills in real logic). `cargo tree -p qtrade-book` and `cargo tree -p qtrade-refdata` both show **only** `qtrade-types` as a dependency — confirmed by running it, not just by writing the `Cargo.toml` files that way and assuming.

## What changed from the first pass, and why each change matters

**`qtrade-mcx-book` → `qtrade-book`, moved out from under anything MCX-shaped.** In the first pass, this crate depended on `qtrade-mcx-decoder` directly — meaning the book builder, which [ARCHITECTURE.md](../ARCHITECTURE.md) §4.8 explicitly calls exchange-agnostic ("one book per subscribed instrument, shared... instance count is a wiring decision"), physically could not compile without MCX's decoder in its dependency tree. That's the exact leak NFR-06's build-order test exists to catch — "if adding the second adapter requires touching anything below the Normalizer, the exchange abstraction has leaked." Now `qtrade-book` only knows about `qtrade_types::Event`. When a second venue arrives, its adapter produces the same `Event` type, and `qtrade-book` needs zero changes.

**A generic `Event` enum now lives in `qtrade-types`, not an MCX-flavored one in the decoder.** This is the literal Normalizer boundary from ARCHITECTURE.md §4.3. It's deliberately not the union of everything MCX's wire format can say (D32's specific warning) — it's shaped by what a book builder needs: `OrderAdded`, `OrderModified{priority_retained}`, `OrderDeleted`, `OrderMassDeleted`, `Trade`, `SnapshotOrder`. `priority_retained` is where MCX's two wire templates (`13101` vs `13106`, FR-B06) get resolved into one venue-agnostic flag — the Normalizer's whole job in one field.

**`qtrade-mcx-refdata` split into a generic `qtrade-refdata` (core) and `qtrade-adapter-mcx::refdata` (venue-specific).** The `InstrumentMaster` and its query builder never need to know a `.bcp` file exists; only the loader does. Same reasoning as the book split.

**The MCX-specific crate is now named `qtrade-adapter-mcx` and lives under `crates/adapters/`**, not at the top of `crates/` alongside the core crates. This is deliberately a visual and structural statement: everything under `adapters/` is optional, swappable, and per-venue. A second venue later means a sibling directory, `crates/adapters/qtrade-adapter-cme/`, with zero edits anywhere else. `qtrade-adapter-mcx` internally splits into `refdata.rs` (T01) / `wire.rs`, `decode.rs`, `normalize.rs` (T02) so two agents can work in the same crate without touching the same file — I pre-wrote `lib.rs`'s module declarations myself so neither of them needs to.

## What's real vs. stubbed

Same as before: every function body is either trivial or `todo!()`. The `OrderAdd` case in `normalize.rs` is filled in as a **worked example** — it's the one place I wrote real (if simple) logic, specifically so T02 has a concrete pattern to extend for `OrderModify`/`OrderModifySamePriority`/`OrderDelete`/`OrderMassDelete`/`Trade` rather than inventing the shape from scratch.

## Ownership, restated plainly

| Crate/file | Owner | Depends on |
|---|---|---|
| `qtrade-types` | frozen — changes route through me | nothing |
| `qtrade-refdata` | T01 | `qtrade-types` |
| `qtrade-book` | T03 | `qtrade-types` |
| `qtrade-adapter-mcx/src/refdata.rs` | T01 | `qtrade-types` |
| `qtrade-adapter-mcx/src/{wire,decode,normalize}.rs` | T02 | `qtrade-types` |
| `qtrade-adapter-mcx/src/lib.rs` | me (module wiring only) | — |

Ready for your review again. Nothing further will be dispatched until you approve.

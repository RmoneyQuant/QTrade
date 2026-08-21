# T02 — `decoder`

**Folder:** `qtrade/src/decoder/` → `decoder.rs` + `user_doc.md`
**Status: DONE.** This file records what was built and the one real follow-up needed before `book` (T03) can consume it — it's not a build task like the others.

---

## What's built

Full detail lives in [`qtrade/src/decoder/user_doc.md`](../qtrade/src/decoder/user_doc.md) — not repeated here. Summary: reads a raw MCX T7 EOBI capture file, decodes every message into a typed value (`Debug` derived on everything, hand-written `Display` for human reading), prints them. Validated against a real 20GB/164M-record file — byte-exact accounting, zero drift.

**Two real corrections found during validation, not assumed from the spec or the reference C++:**
1. The outer file framing had to be reverse-engineered from real bytes — `MCX_Feeder.cpp`'s literal description doesn't match what these files contain.
2. Price scaling: the legacy code's ÷1,000,000 is wrong for these files; ÷100,000,000 (the EOBI spec's own "8 decimals" claim) is correct, confirmed against a real contract cross-check.

Both are documented in depth in the component's own doc — read it there, don't re-derive it.

## The one follow-up this creates for `book` (T03)

**`decode_file()` currently only prints — it has no public API that hands decoded messages to a caller.** `book` needs a stream of messages to build an order book from. Before or as part of T03, `decoder.rs` needs a second entry point alongside the existing CLI one, roughly:

```rust
pub fn decode_messages(data: &[u8]) -> impl Iterator<Item = DecodedMessage> + '_
```

(or whatever shape `book` actually needs — don't over-design this now; let T03 pull what it needs when it's actually being written, per the same YAGNI discipline as `types`.) `DecodedMessage` and its variant structs are currently private to `decoder.rs` — they'll need `pub` where `book` actually touches them, and nowhere else.

**Do not restructure `decoder.rs` beyond this.** It's validated and working; the only change justified right now is "also expose an iterator," not a redesign.

## Not otherwise a task

No further action needed on this component for M2 itself — FR-B04 through FR-B07's acceptance criteria are already met (see the component doc's "what this pilot found and validated" section for the evidence). Revisit only when `book` is actually being built and needs the API above.

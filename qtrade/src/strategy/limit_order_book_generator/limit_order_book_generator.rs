//! A pure observer strategy: prints a C++-style limit order book feed
//! (`feed.csv`) for every underlying it's given, submits no orders at
//! all. Renamed from `dummy_strategy` (2026-08-25) -- that name was
//! never meant to be permanent; it just described what the very first
//! strategy did ("very basic operations"). Each real strategy plugged
//! into `main.rs` going forward gets its own name and its own folder,
//! same as this one.
//!
//! **Why this exists as its own strategy, not orchestrator tooling:**
//! `main.rs` used to call this unconditionally for *any* strategy
//! plugged into it, regardless of whether that strategy wanted it --
//! discovered and corrected the same day this file was created. Some
//! strategies need this view, some won't, and "always print an LOB CSV"
//! being baked into the orchestrator would have made it impossible to
//! write a strategy that didn't. Putting it here means: run this
//! strategy, get `feed.csv`; run a different strategy, don't.
//!
//! **`on_start`/`on_book` (2026-08-25):** implements `strategy::Strategy`
//! -- `on_start` declares its own subscriptions via `ctx.resolve`/
//! `ctx.subscribe` (D33: subscribing is the strategy's own job, not the
//! orchestrator's); `on_book` is what used to be called `on_wake`,
//! taking `ctx: &mut Ctx` instead of `cache: &Cache` directly, and
//! reading the instrument's display name from `ctx.refdata()` instead of
//! a `label: &str` `main.rs` used to thread in by hand.
//!
//! **Not currently compiled into `main.rs`** (swapped out for
//! `naturalgas_bracket`, 2026-08-25/26, per `strategy/README.md`'s own
//! swap convention) -- kept up to date regardless, so swapping back
//! means editing `main.rs`'s own `#[path]`/`use`/construction lines,
//! never this file.
//!
//! Has no `[[bin]]` target of its own -- it's a plain module `main.rs`
//! (the crate's one real entry point, `[[bin]] qtrade`) includes and
//! drives, same as `book.rs`/`execution.rs` are included by whichever
//! bin needs them.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

use crate::event_dispatcher::Depth;
use crate::strategy::{Ctx, StartCtx, Strategy};
use crate::types::{InstrumentId, InstrumentKind};

/// Real exchange underlying names this strategy watches. A name, never
/// a token -- resolved to *that day's* real front-month token via
/// `ctx.resolve` inside `on_start` (backed, in backtest mode, by
/// `feed_replay::resolve_front_month`; `main.rs` also resolves these
/// same names independently, before any strategy code runs, to build
/// the `InstrumentFilter`/`Cache`/`ExecutionEngine` -- two legitimate,
/// separate uses of the same constant, not a contradiction of "the
/// strategy owns its own subscriptions").
pub const UNDERLYINGS: &[&str] = &["NATURALGAS"];

/// How many price levels per side this strategy wants to see, and
/// therefore how deep `on_start` subscribes it (D25: depth of interest
/// is the strategy's own declaration) -- exported so this file's own
/// row-printing agrees with its own subscription by construction, not
/// by two independently-maintained constants that could drift.
pub const DEPTH_LEVELS: usize = 5;

/// Raw wire units -> human units, same conversion `decoder`'s own
/// `Price`/`Qty` `Display` impls use -- duplicated here rather than
/// imported since `decoder`'s `Price`/`Qty` are private to that module
/// (they model the wire, not the shared vocabulary), and `main.rs` keeps
/// its own copy for the same reason (a different concern, its own
/// summary printline).
const RUPEE_RAW: f64 = 100_000_000.0;
const LOT_RAW: f64 = 10_000.0;

/// One instrument's last-logged levels, kept purely to attribute a row
/// to a side (see `format_depth_row`) -- not read by anything else.
#[derive(Default, Clone, PartialEq)]
struct LevelSnapshot {
    bids: Vec<(i64, i64)>, // (price_raw, qty_raw), best first
    asks: Vec<(i64, i64)>,
}

/// This instrument's display name, read from `Cache`'s own refdata
/// (`ctx.refdata()`) rather than threaded in by hand -- `Instrument.kind`
/// already carries it (`Future { underlying, .. }`), so there is no
/// separate name lookup for `main.rs`/`event_dispatcher` to maintain and
/// keep in sync (see this file's header, and Q7 of this session's design
/// discussion).
fn label_of(ctx: &Ctx, instrument: InstrumentId) -> String {
    match ctx.refdata().get(instrument) {
        Some(i) => match &i.kind {
            InstrumentKind::Future { underlying, .. } => underlying.clone(),
            _ => format!("{instrument:?}"),
        },
        None => format!("{instrument:?}"),
    }
}

/// Reads `Book::depth(DEPTH_LEVELS)` and splits its one combined vec back
/// into sides -- **by value, not by position**: classifies each entry
/// against the book's own real `best_ask()` price rather than trusting
/// that exactly `DEPTH_LEVELS` bids always precede the asks (wrong the
/// moment either side is thinner than `DEPTH_LEVELS`, which is common,
/// not rare -- caught in real output the first time this ran).
fn depth_snapshot(ctx: &Ctx, instrument: InstrumentId) -> Option<LevelSnapshot> {
    let book = ctx.book(instrument)?;
    let combined = book.depth(DEPTH_LEVELS);
    let best_ask_raw = book.best_ask().map(|l| l.price.0);
    let mut bids = Vec::new();
    let mut asks = Vec::new();
    for lvl in &combined {
        if best_ask_raw.is_some_and(|a| lvl.price.0 >= a) {
            asks.push((lvl.price.0, lvl.qty.0));
        } else {
            bids.push((lvl.price.0, lvl.qty.0));
        }
    }
    Some(LevelSnapshot { bids, asks })
}

/// One CSV row: real exchange timestamp, which side(s) moved, then
/// `DEPTH_LEVELS` bid/ask price+qty pairs (best first), in human units.
/// Diffs against `last_seen`'s previous snapshot for that instrument to
/// fill in the `side` column. `None` if the instrument has no book yet,
/// or neither side actually differs from last time.
fn format_depth_row(ctx: &Ctx, instrument: InstrumentId, seq: u64, transact_time_ns: u64, last_seen: &mut HashMap<InstrumentId, LevelSnapshot>) -> Option<String> {
    let snap = depth_snapshot(ctx, instrument)?;
    let prev = last_seen.get(&instrument);
    let bid_changed = prev.is_none_or(|p| p.bids != snap.bids);
    let ask_changed = prev.is_none_or(|p| p.asks != snap.asks);
    let side = match (bid_changed, ask_changed) {
        (true, true) => "BOTH",
        (true, false) => "BID",
        (false, true) => "ASK",
        (false, false) => return None,
    };

    let mut cells = vec![transact_time_ns.to_string(), seq.to_string(), label_of(ctx, instrument), side.to_string()];
    for i in 0..DEPTH_LEVELS {
        match snap.bids.get(i) {
            Some((p, q)) => {
                cells.push(format!("{:.2}", *p as f64 / RUPEE_RAW));
                cells.push(format!("{:.1}", *q as f64 / LOT_RAW));
            }
            None => {
                cells.push(String::new());
                cells.push(String::new());
            }
        }
        match snap.asks.get(i) {
            Some((p, q)) => {
                cells.push(format!("{:.2}", *p as f64 / RUPEE_RAW));
                cells.push(format!("{:.1}", *q as f64 / LOT_RAW));
            }
            None => {
                cells.push(String::new());
                cells.push(String::new());
            }
        }
    }
    last_seen.insert(instrument, snap);
    Some(cells.join(","))
}

fn depth_csv_header() -> String {
    let mut cells = vec!["timestamp_ns".to_string(), "seq".to_string(), "instrument".to_string(), "side".to_string()];
    for i in 0..DEPTH_LEVELS {
        cells.push(format!("bid{i}_price"));
        cells.push(format!("bid{i}_qty"));
        cells.push(format!("ask{i}_price"));
        cells.push(format!("ask{i}_qty"));
    }
    cells.join(",")
}

/// The strategy itself. Submits no orders -- `on_book` only ever reads
/// `ctx` and writes a row. `orders.log`/`fills.log` are legitimately
/// empty when this strategy runs; that's correct, not a bug, since it
/// never calls `ExecutionEngine::submit_order` at all.
pub struct LimitOrderBookGenerator {
    file: File,
    last_seen: HashMap<InstrumentId, LevelSnapshot>,
    rows_written: u64,
    rows_printed: usize,
    max_stdout_lines: usize,
}

impl LimitOrderBookGenerator {
    /// `path` is where to write the feed -- decided by `main.rs` (the
    /// run's timestamped output folder), not this strategy; location
    /// config stays the orchestrator's job, same principle as the rest
    /// of this project. `max_stdout_lines` caps how many rows also echo
    /// to stdout while a run is in progress; the file always gets the
    /// complete, uncapped feed regardless.
    pub fn new(path: &str, max_stdout_lines: usize) -> io::Result<Self> {
        let mut file = File::create(path)?;
        // `timestamp_ns` is the real exchange feed-handler send time
        // (`PacketHeader.TransactTime`) -- the enclosing packet's
        // timestamp, not a per-message one. Per-message timestamps exist
        // on the wire but are unsafe to use directly: some resting
        // orders carry a sentinel instead of a real time, and `Trade`'s
        // own `event_time` field isn't a timestamp at all.
        writeln!(file, "{}", depth_csv_header())?;
        Ok(LimitOrderBookGenerator { file, last_seen: HashMap::new(), rows_written: 0, rows_printed: 0, max_stdout_lines })
    }

    pub fn rows_written(&self) -> u64 {
        self.rows_written
    }

    pub fn rows_printed(&self) -> usize {
        self.rows_printed
    }
}

impl Strategy for LimitOrderBookGenerator {
    /// Declares this strategy's own subscriptions -- D33: `Strategy ->
    /// subscribe() -> Control Dispatcher -> Data Engine`. Resolution
    /// (`ctx.resolve`) and subscription (`ctx.subscribe`) are both the
    /// strategy's own call now; `main.rs` no longer decides either for
    /// it.
    fn on_start(&mut self, ctx: &mut StartCtx) {
        for name in UNDERLYINGS {
            if let Some(id) = ctx.resolve(name) {
                ctx.subscribe(id, Depth::Top(DEPTH_LEVELS as u8));
            }
        }
    }

    /// Called by `event_dispatcher` once per real book change on a
    /// subscribed instrument -- this strategy's entire reaction, and the
    /// only thing left in this file that does anything. `on_trade` is
    /// left at its default (empty): this strategy only cares about book
    /// state, not individual trade prints. `on_fill`/`on_order_update`
    /// are left at their defaults too -- it submits no orders, so it has
    /// nothing to react to on either.
    fn on_book(&mut self, ctx: &mut Ctx, instrument: InstrumentId, seq: u64, packet_transact_time_ns: u64) {
        if let Some(row) = format_depth_row(ctx, instrument, seq, packet_transact_time_ns, &mut self.last_seen) {
            writeln!(self.file, "{row}").unwrap();
            self.rows_written += 1;
            if self.rows_printed < self.max_stdout_lines {
                println!("{row}");
                self.rows_printed += 1;
            }
        }
    }
}

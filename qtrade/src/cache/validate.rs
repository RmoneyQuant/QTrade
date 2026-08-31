//! `cache`'s acceptance harness (BACKTEST-PHASE1.md §M5's own acceptance
//! bar, §5.1): a no-op strategy runs `decoder -> filter -> book -> cache
//! -> dispatch` against a full real session for both validated
//! instruments (CRUDEOIL, NATURALGAS), streamed record-by-record off
//! disk (same streaming discipline `book`'s own FR-B11 harness used --
//! see `book_user_doc.md` §5.4), and reports real throughput plus a
//! real, measured allocation count on the dispatch/book-apply hot path
//! (NFR-05).
//!
//! **Not part of `cache`'s public API**, and deliberately not wired into
//! `main.rs` (T05's brief: main.rs is left untouched, another agent
//! wires modules in). Same reason as `book-validate`
//! (`src/book/validate.rs`): this crate has no `[lib]` target, so a
//! second `[[bin]]` entry pointing here is the only way to compile and
//! run this component's own code standalone. See `cache_user_doc.md` §5
//! for the full account of this run and its numbers.

#[allow(dead_code)]
#[path = "../types/types.rs"]
mod types;
#[path = "../decoder/decoder.rs"]
mod decoder;
#[path = "../book/book.rs"]
mod book;
#[path = "../refdata/refdata.rs"]
mod refdata;
#[path = "cache.rs"]
mod cache;
// Dispatch moved out of `cache` (see cache.rs's own header) -- this
// harness now drives it explicitly, the same way `main.rs` does.
#[path = "../event_dispatcher/event_dispatcher.rs"]
mod event_dispatcher;
// `control_dispatcher` now depends on `execution` (Phase B: `ControlHandler`
// takes `execution::FillRecord`/`OrderEventRecord` directly) -- pulled in
// here purely to satisfy that, not because this harness exercises order
// submission itself.
#[allow(dead_code)]
#[path = "../simulator/simulator.rs"]
mod simulator;
// `strategy` now depends on `scheduler` (dual-clock replay, 2026-08-27:
// `Ctx`/`RunHandles` need `Scheduler`/`EventClass`/`EventPayload` to
// schedule a deliver-phase event from `submit`/`cancel`/`modify`) --
// pulled in purely to satisfy that, same reasoning as `simulator` above.
#[allow(dead_code)]
#[path = "../scheduler/scheduler.rs"]
mod scheduler;
#[allow(dead_code)]
#[path = "../execution/execution.rs"]
mod execution;
#[path = "../logging/logging.rs"]
mod logging;
#[path = "../control_dispatcher/control_dispatcher.rs"]
mod control_dispatcher;
#[path = "../strategy/strategy.rs"]
mod strategy;

use cache::{Cache, InstrumentFilter};
use decoder::DecodedMessage;
use event_dispatcher::{Depth, EventDispatcher};
use strategy::Strategy;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use types::InstrumentId;

// ---------------------------------------------------------------------
// A real allocation-counting method: a global allocator that forwards
// to `System` but counts every call and every byte, process-wide. This
// is what "measured, not assumed" means here -- NFR-05's zero-allocation
// claim is checked against this counter's before/after delta around the
// exact calls that matter (`Cache::apply`/`Cache::dispatch`), not
// eyeballed from source.
// ---------------------------------------------------------------------

static ALLOC_COUNT: AtomicU64 = AtomicU64::new(0);
static ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static DEALLOC_COUNT: AtomicU64 = AtomicU64::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // A realloc is still a (re-)allocation event for this
        // measurement's purposes -- growth counts the same as a fresh
        // alloc would have, since it's the same "the hot path needed
        // more heap" signal NFR-05 cares about.
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
        System.realloc(ptr, layout, new_size)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOC_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

#[inline]
fn alloc_snapshot() -> (u64, u64) {
    (ALLOC_COUNT.load(Ordering::Relaxed), ALLOC_BYTES.load(Ordering::Relaxed))
}

// ---------------------------------------------------------------------
// Streaming reader -- same outer framing decoder.rs documents
// ([8B length][8B capture timestamp][payload]), same discipline as
// book/validate.rs's own `RecordSource`: never load a whole file into
// memory. Duplicated here (not imported) because this is a *different*
// `[[bin]]` target with no `[lib]` to share it through -- see this
// file's own header comment.
// ---------------------------------------------------------------------

struct RecordSource {
    reader: BufReader<File>,
}

impl RecordSource {
    fn open(path: &str) -> io::Result<Self> {
        Ok(RecordSource {
            reader: BufReader::with_capacity(1 << 20, File::open(path)?),
        })
    }

    fn next_record(&mut self, payload: &mut Vec<u8>) -> io::Result<bool> {
        let mut hdr = [0u8; 16];
        match self.reader.read_exact(&mut hdr) {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
            Err(e) => return Err(e),
        }
        let length = u64::from_le_bytes(hdr[0..8].try_into().unwrap()) as usize;
        if length < 8 {
            return Ok(false);
        }
        let payload_len = length - 8;
        payload.resize(payload_len, 0);
        self.reader.read_exact(payload)?;
        Ok(true)
    }
}

// ---------------------------------------------------------------------
// No-op strategy -- the acceptance bar's own words: "subscribes, does
// nothing." Counts its own wakes (so this run can report the real
// wake/touch ratio D25 predicts), but takes no action on one -- that
// counting is bookkeeping for this report, not strategy logic.
// ---------------------------------------------------------------------

struct NoOpStrategy {
    wakes: Rc<Cell<u64>>,
}

impl Strategy for NoOpStrategy {
    // This harness registers/subscribes `NoOpStrategy` directly (below,
    // mirroring `main.rs`'s own setup-time wiring), not via a real
    // `on_start`/`StartCtx` call -- there is nothing for this method to
    // do.
    fn on_start(&mut self, _ctx: &mut strategy::StartCtx) {}

    fn on_book(&mut self, _ctx: &mut strategy::Ctx, _instrument: InstrumentId, _seq: u64, _packet_transact_time_ns: u64) {
        self.wakes.set(self.wakes.get() + 1);
    }
}

// ---------------------------------------------------------------------
// Per-instrument run: streams one capture file record by record,
// decoding each inner EOBI message inline (no per-payload `Vec`
// collection -- unlike book/validate.rs's `parse_inner`, deliberately,
// so this harness's own bookkeeping doesn't add allocations that would
// blur the measurement below) and feeding it through
// `Cache::apply` -> `Cache::dispatch`, instrumented with the counting
// allocator's before/after delta around exactly those two calls.
// ---------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy)]
struct RunCounters {
    outer_records: u64,
    messages: u64,
    apply_alloc_count: u64,
    apply_alloc_bytes: u64,
    dispatch_alloc_count: u64,
    dispatch_alloc_bytes: u64,
}

fn stream_file(label: &str, path: &str, cache: &mut Cache, dispatcher: &mut EventDispatcher, engine: &mut execution::ExecutionEngine, venue: &mut simulator::SimExchange, stop_after: Option<u64>) -> io::Result<RunCounters> {
    let mut source = RecordSource::open(path)?;
    let mut payload = Vec::new();
    let mut counters = RunCounters::default();

    while source.next_record(&mut payload)? {
        counters.outer_records += 1;
        let mut off = 0usize;
        while off + 8 <= payload.len() {
            let body_len = u16::from_le_bytes([payload[off], payload[off + 1]]) as usize;
            let template_id = u16::from_le_bytes([payload[off + 2], payload[off + 3]]);
            let seq = u32::from_le_bytes(payload[off + 4..off + 8].try_into().unwrap());
            if body_len == 0 || off + body_len > payload.len() {
                break;
            }
            let msg: DecodedMessage = decoder::decode_message(template_id, seq, &payload[off..off + body_len]);
            off += body_len;
            counters.messages += 1;

            // Measured, not assumed (NFR-05): snapshot the global
            // counting allocator immediately around each of `apply` and
            // `on_book_touched` separately, so a nonzero delta can be
            // attributed to the right one of the two.
            let (a0, b0) = alloc_snapshot();
            if let Some(instrument) = cache.apply(&msg) {
                let (a1, b1) = alloc_snapshot();
                counters.apply_alloc_count += a1 - a0;
                counters.apply_alloc_bytes += b1 - b0;

                if let Some(book) = cache.book(instrument) {
                    // `venue` passed directly now (2026-08-27, second
                    // pass) -- `Ctx` holds `&mut SimExchange` itself, no
                    // `RunHandles`/`Scheduler` involved; `NoOpStrategy`
                    // never calls `ctx.submit` anyway.
                    dispatcher.on_book_touched(book, instrument, cache, engine, venue, seq as u64, 0);
                }

                let (a2, b2) = alloc_snapshot();
                counters.dispatch_alloc_count += a2 - a1;
                counters.dispatch_alloc_bytes += b2 - b1;
            }
        }

        if counters.outer_records % 20_000_000 == 0 {
            eprintln!(
                "  [{label}] ... {} outer records, {} messages processed so far",
                counters.outer_records, counters.messages
            );
        }
        if let Some(n) = stop_after {
            if counters.outer_records >= n {
                eprintln!("  [{label}] stopping early after {n} records (CACHE_DEBUG_STOP_AFTER_RECORDS)");
                break;
            }
        }
    }
    Ok(counters)
}

fn main() -> io::Result<()> {
    println!("cache acceptance run -- decoder -> filter -> book -> cache -> dispatch, date 19_01_2026\n");

    let refdata_path = Path::new("/mnt/MCX_Recording_Files/CONTRACT/19_01_2026/MCXScrips.bcp");
    let master = refdata::InstrumentMaster::load_mcx(refdata_path).expect("load real MCXScrips.bcp for 19_01_2026");
    println!("refdata: {} instruments loaded from {}", master.all().len(), refdata_path.display());

    // The roll-trap-proof filter shape (D32, FR-B16), for real: "all
    // CRUDEOIL futures" and "all NATURALGAS futures", by underlying
    // symbol, no expiry restriction at all -- see cache.rs's
    // `InstrumentFilter` doc comment for the reasoning, and
    // cache_user_doc.md for the real MCXScrips.bcp rows this was checked
    // against. Computed here (against the real day's master) purely to
    // report its size -- not what `Cache` is actually constructed with
    // below. See the following comment for why.
    let broad_filter = InstrumentFilter::from_predicate(&master, |i| match &i.kind {
        types::InstrumentKind::Future { underlying, .. } => underlying == "CRUDEOIL" || underlying == "NATURALGAS",
        _ => false,
    });
    println!(
        "roll-trap-proof filter shape would admit {} native SecurityIDs today (all CRUDEOIL + all NATURALGAS futures, every expiry)",
        broad_filter.len()
    );

    // **What this run's `Cache` is actually constructed with, and why it
    // is narrower than the line above:** `book::BookBuilder` derives a
    // price band/tick size per instrument from a hand-tuned table
    // (`book.rs`'s private `band_config`) that only has real, validated
    // entries for `CRUDEOIL_ID`/`NATURALGAS_ID` -- book.rs's own doc
    // comment says so explicitly ("this milestone supports exactly these
    // two"). Feeding the broader filter's full native-id set into
    // `BookBuilder::new` was tried and it panics on real data: a
    // different real NATURALGAS-family contract (native id 475111, not
    // 465849) trades at sub-rupee prices that don't land on the
    // fallback band's 1-rupee tick grid, tripping `MboBookImpl::idx_of`'s
    // deliberate panic-on-misconfiguration (book.rs's `idx_of` doc
    // comment: exactly the "silent wrong book" failure mode it exists to
    // catch). Widening `band_config` to safely cover every MCX
    // instrument is `book`'s job, not `cache`'s, and book.rs is out of
    // this task's edit scope -- so this run's own `Cache` is
    // deliberately scoped to the two instruments `book` actually
    // validated (matching the task brief's own acceptance instructions),
    // while the filter *mechanism* above (general, roll-trap-safe,
    // predicate-based) is proven correct independently by cache.rs's own
    // unit tests (`filter_covers_a_contract_not_yet_rolled_into`, using a
    // synthetic second contract that doesn't hit this band limitation).
    // See cache_user_doc.md for the full account.
    let filter = InstrumentFilter::from_predicate(&master, |i| i.native_id == 467_013 || i.native_id == 465_849);
    println!(
        "this run's Cache is constructed with {} native SecurityIDs (467013, 465849 -- the two book.rs has real validated bands for)\n",
        filter.len()
    );

    let mut cache = Cache::new(master, filter);

    // **Real gap, not a hardcoded stand-in for `book`'s generic
    // mechanism:** `book::BookBuilder` now learns each instrument's price
    // band from a real `InstrumentInfo` (13603) message in whatever
    // stream applies it -- but this run below only ever feeds `cache`
    // the *increment* capture files (see `stream_file`/`cases`), never
    // the paired snapshot capture that also carries one. Checked against
    // real bytes: CRUDEOIL's real 19_01_2026 increment capture never
    // carries a *valid* 13603 during the session at all (its DPR never
    // changed that day, so nothing re-published it there -- the one-time
    // Start-of-Day broadcast predates this capture's start; only the
    // *snapshot* channel repeats it every cycle). Without seeding, the
    // very first real CRUDEOIL order would hit `BookBuilder::apply`'s
    // documented "band still Pending" panic. `Cache::seed_book_band`
    // (a thin forward to `book::BookBuilder::seed_band`) is the same
    // real, snapshot-verified numbers `book-validate`'s own harness
    // learned from the paired snapshot file's 13603 stream (see
    // book_user_doc.md's "generic price band" section) -- not a guess,
    // and not a return of the old hardcoded `band_config`, which this
    // run's `Cache`/`BookBuilder` no longer has or uses for any other
    // instrument.
    cache.seed_book_band(book::CRUDEOIL_ID, 523_200_000_000, 566_600_000_000); // Rs 5,232.00 - Rs 5,666.00
    cache.seed_book_band(book::NATURALGAS_ID, 22_160_000_000, 33_920_000_000); // Rs 221.60 - Rs 339.20 (full-session union)

    // A throwaway engine -- this harness measures Cache/EventDispatcher
    // allocation behavior only; it has no interest in execution at all.
    // Passed through purely because `on_book_touched` now needs one to
    // construct `Ctx` (Phase C).
    let run_config = execution::RunConfig {
        session_id: 1,
        cost_config: execution::CostConfig::default(),
        local_otr: execution::LocalOtrConfig { window_ns: 1_000_000_000, max_messages_per_window: 10_000 },
        venue_otr: execution::OtrConfigSummary { window_ns: 1_000_000_000, max_messages_per_window: 10_000, max_otr_ratio_bits: 0 },
        markout_horizons_ns: vec![],
    };
    let venue_otr = simulator::OtrConfig { window: std::time::Duration::from_secs(1), max_messages_per_window: 10_000, max_otr_ratio: 1_000_000.0 };
    // `SimExchange` built here now, not inside `ExecutionEngine::new`
    // (dual-clock replay, 2026-08-27) -- see main.rs's own construction
    // for why. Just as throwaway as `engine` itself for this harness.
    let mut venue = simulator::SimExchange::new(&[], venue_otr);
    let mut engine = execution::ExecutionEngine::new(run_config, vec![], Box::new(execution::AlwaysAllowRms), execution::CostConfig::default(), vec![], false);

    let wakes_crude = Rc::new(Cell::new(0u64));
    let wakes_gas = Rc::new(Cell::new(0u64));
    let mut dispatcher = EventDispatcher::new();
    let id_crude = dispatcher.register(Rc::new(RefCell::new(NoOpStrategy { wakes: wakes_crude.clone() })));
    dispatcher.subscribe(id_crude, book::CRUDEOIL_ID, Depth::Bbo);
    let id_gas = dispatcher.register(Rc::new(RefCell::new(NoOpStrategy { wakes: wakes_gas.clone() })));
    dispatcher.subscribe(id_gas, book::NATURALGAS_ID, Depth::Bbo);

    let stop_after: Option<u64> = std::env::var("CACHE_DEBUG_STOP_AFTER_RECORDS")
        .ok()
        .and_then(|s| s.parse().ok());

    let cases: [(&str, &str); 2] = [
        ("CRUDEOIL (467013, stream 4)", "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin"),
        ("NATURALGAS (465849, stream 5)", "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_5.bin"),
    ];

    let mut total = RunCounters::default();
    let start = Instant::now();
    for (label, path) in cases {
        println!("--- {label} ---");
        let file_start = Instant::now();
        let counters = stream_file(label, path, &mut cache, &mut dispatcher, &mut engine, &mut venue, stop_after)?;
        let elapsed = file_start.elapsed();
        println!(
            "  {} outer records, {} messages in {:.2}s -> {:.0} records/s, {:.0} messages/s",
            counters.outer_records,
            counters.messages,
            elapsed.as_secs_f64(),
            counters.outer_records as f64 / elapsed.as_secs_f64(),
            counters.messages as f64 / elapsed.as_secs_f64(),
        );
        println!(
            "  apply() allocs: count={} bytes={}   dispatch() allocs: count={} bytes={}",
            counters.apply_alloc_count, counters.apply_alloc_bytes, counters.dispatch_alloc_count, counters.dispatch_alloc_bytes
        );
        total.outer_records += counters.outer_records;
        total.messages += counters.messages;
        total.apply_alloc_count += counters.apply_alloc_count;
        total.apply_alloc_bytes += counters.apply_alloc_bytes;
        total.dispatch_alloc_count += counters.dispatch_alloc_count;
        total.dispatch_alloc_bytes += counters.dispatch_alloc_bytes;
        println!();
    }
    let elapsed = start.elapsed();

    let stats = dispatcher.stats;
    println!("=== TOTAL ===");
    println!(
        "{} outer records, {} decoded messages, {:.2}s wall -> {:.0} records/s, {:.0} messages/s",
        total.outer_records,
        total.messages,
        elapsed.as_secs_f64(),
        total.outer_records as f64 / elapsed.as_secs_f64(),
        total.messages as f64 / elapsed.as_secs_f64(),
    );
    println!(
        "dispatch: {} book touches, {} wakes fired (CRUDEOIL subscriber: {}, NATURALGAS subscriber: {})",
        stats.book_touches,
        stats.wakes_fired,
        wakes_crude.get(),
        wakes_gas.get()
    );
    println!(
        "allocations -- apply(): count={} bytes={}   dispatch(): count={} bytes={}   (measured via a global counting allocator, see cache_user_doc.md)",
        total.apply_alloc_count, total.apply_alloc_bytes, total.dispatch_alloc_count, total.dispatch_alloc_bytes
    );
    if total.dispatch_alloc_count == 0 {
        println!("NFR-05: dispatch path is zero-allocation over this full real-session run -- confirmed, not assumed.");
    } else {
        println!("NFR-05: dispatch path allocated {} times over this run -- see cache_user_doc.md for the traced cause.", total.dispatch_alloc_count);
    }

    Ok(())
}

//! One shared component-level event log for a whole run (2026-08-27,
//! requested directly: "each part of qtrade should send the response
//! that... confirmed" -- inspired by, but deliberately not copied from,
//! Nautilus's own `[CMD]-->`/`<--[EVT]` strategy-logger trail; see
//! `main_user_doc.md` for the design discussion).
//!
//! **Low-latency, off the hot path (2026-08-27, second pass).** A first
//! version of this module wrote directly to a `File` and `stdout` from
//! whichever thread called `log_event`/`dispatch`/etc. -- fine for
//! correctness, wrong for an HFT-shaped system: a disk write or a
//! terminal write on the same thread that's deciding whether to submit
//! an order is exactly the kind of hidden latency this project's own
//! `STRATEGY-GUIDE.md` warns a real strategy never to eat. This project
//! breaks its own long-standing zero-external-dependency streak here, on
//! purpose, for `tracing`/`tracing-subscriber`/`tracing-appender` --
//! Rust's standard, battle-tested answer to the same problem C++'s
//! `quill` solves: a log call is just a cheap, non-blocking push onto a
//! channel; a dedicated background thread (`tracing-appender`'s
//! `non_blocking` worker) owns the actual file/stdout write, entirely off
//! whatever thread is driving the replay loop.
//!
//! **The clock is always the sim clock, never wall time.** `tracing`
//! itself doesn't know about `scheduler::Scheduler`'s own notion of
//! "now" -- so this module never uses `tracing`'s own timestamp. Every
//! call site builds its own line via `line()`, embedding whatever
//! `now_ns` it was actually handed (the same simulated instant
//! `scheduler::Scheduler` is driving at that point in the run: recorder
//! time for a market event, the scheduled delivery time for an
//! `OrderArrival`/`ReportDelivery`), and hands the whole string to
//! `tracing::info!`/`tracing::debug!` as one opaque message -- `tracing`
//! is used purely as the async delivery mechanism here, not for its own
//! structured-field/timestamp machinery. `on_start`/subscription lines
//! run before any event has been scheduled at all -- honestly rendered
//! as "(pre-replay)" via `now_ns: None` rather than faking a timestamp
//! that doesn't exist yet.
//!
//! **Two levels, one file.** `line()`'s caller decides `info!` (normal --
//! a real run always needs this to audit: subscription confirmed, gate
//! check passed/denied, venue response received, handed off to the
//! strategy) or `debug!` (finer detail on top, never instead of --
//! gated behind `[run] log_level = "debug"` in the config file, silent
//! otherwise via `init()`'s max-level filter). Both land in the same two
//! places: the run's own `events.log`, mirrored live to stdout -- both
//! non-blocking writers, both flushed by their own background thread.
//!
//! **Who calls this.** `ExecutionEngine::log_event` is the single choke
//! point for every order-state-transition message this project already
//! crafts (`"submit: gates passed..."`, `"denied: ..."`, `"resting"`,
//! `"filled qty=..."`, `"venue rejected: ..."`, `"canceled: ..."`) --
//! tagged here by the order's own `OrderState` rather than a second,
//! parallel tag vocabulary invented just for this file, so a reader can
//! cross-reference straight against `execution_user_doc.md`'s state
//! machine. `ControlDispatcher::dispatch` logs one `DISPATCH` line right
//! before it hands a fill/order-update to the strategy's own `on_fill`/
//! `on_order_update` -- the moment the strategy actually learns
//! something, distinct from the strategy's own reaction to it.
//! `ControlDispatcher::subscribe` logs `SUBSCRIBE_OK` once a subscription
//! is registered. A strategy may log through the same macros too, via
//! `Ctx::log`/`StartCtx::log` (see `strategy/strategy.rs`) -- so
//! `events.log` is the one place a whole run's real trail lives, not
//! split between stdout-only strategy prints and file-only engine
//! prints.
//!
//! **`SimExchange` itself logs nothing** (a deliberate choice, discussed
//! directly): `ExecutionEngine` narrates whatever the venue answered,
//! right after the venue call returns, so `SimExchange` keeps its D10
//! independence -- zero knowledge of logging, strategies, or reporting,
//! same as it has zero knowledge of `Cache`.

use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::prelude::*;

/// Selects `tracing`'s own max-level filter. `Normal` admits `info!`/
/// `warn!`/`error!` only (every `debug!` call becomes a cheap no-op,
/// filtered before it ever reaches the channel -- `tracing`'s whole
/// design point); `Debug` admits `debug!` too. Config-driven
/// (`[run] log_level`), never toggled mid-run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Normal,
    Debug,
}

impl LogLevel {
    /// Parses `config::RunSection::log_level`'s already-validated string
    /// ("normal"/"debug", enforced at config load time) -- `Normal` is
    /// the fallback for anything else, so this never panics even if a
    /// caller (e.g. a test) hands in something unvalidated.
    pub fn parse(raw: &str) -> LogLevel {
        if raw == "debug" {
            LogLevel::Debug
        } else {
            LogLevel::Normal
        }
    }

    fn as_tracing_level(self) -> tracing::Level {
        match self {
            LogLevel::Normal => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
        }
    }
}

/// Installs the process-wide `tracing` subscriber for a real run: two
/// non-blocking writers (the run's own `events.log`, and stdout), both
/// gated at `level`, neither ever touched by the thread that calls
/// `info!`/`debug!` -- that thread only ever pushes a formatted `String`
/// onto a channel (see this file's own header). Returns both
/// `WorkerGuard`s; **the caller (`main.rs`) must hold them for the whole
/// run** -- dropping a guard early stops its worker thread, and any
/// lines still sitting in that channel are lost rather than flushed.
///
/// Deliberately bare-bones formatting (`.without_time().with_target(false)
/// .with_level(false)`): every decoration `tracing_subscriber` would
/// otherwise add (its own wall-clock timestamp, module path, level name)
/// is switched off, so the only thing that ever reaches the file/stdout
/// is exactly the string `line()` built -- including the one real
/// timestamp that matters, the sim clock, embedded in that string itself.
pub fn init(level: LogLevel, events_log_path: &Path) -> std::io::Result<(WorkerGuard, WorkerGuard)> {
    let file = std::fs::File::create(events_log_path)?;
    let (file_writer, file_guard) = tracing_appender::non_blocking(file);
    let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    let max_level = level.as_tracing_level();

    let file_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_level(false)
        .with_writer(file_writer.with_max_level(max_level));
    let stdout_layer = tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .with_level(false)
        .with_writer(stdout_writer.with_max_level(max_level));

    tracing_subscriber::registry().with(file_layer).with(stdout_layer).init();

    Ok((file_guard, stdout_guard))
}

/// Builds the one-line message every `info!`/`debug!` call in this
/// project hands to `tracing` -- `t=<raw ns> (<IST>) [<component>] <tag>:
/// <msg>`, `t=--  (pre-replay -- no sim timestamp exists yet)` when
/// `now_ns` is `None` (before any event has been scheduled, e.g.
/// `on_start`/subscription). The one place this format is defined, so
/// every caller (`execution.rs`, `control_dispatcher.rs`, a strategy's
/// own `Ctx::log`) renders identically.
pub fn line(component: &str, now_ns: Option<u64>, tag: &str, msg: &str) -> String {
    let time = match now_ns {
        Some(ns) => format!("t={ns} ({})", fmt_ist(ns)),
        None => "t=--  (pre-replay -- no sim timestamp exists yet)".to_string(),
    };
    format!("{time} [{component}] {tag}: {msg}")
}

/// Howard Hinnant's `civil_from_days` (public domain, proleptic
/// Gregorian) -- the same algorithm `main.rs`'s and (until this pass)
/// `naturalgas_bracket.rs`'s own private copies used. One copy now,
/// here, since every component that logs needs IST rendering, not just
/// the strategy or the orchestrator individually.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// A raw Unix-epoch-nanosecond `now_ns` value, rendered as human
/// `YYYY-MM-DD HH:MM:SS.mmm IST`. Public: a strategy rendering its own
/// timestamps outside `line()` (e.g. a summary line) can reuse it rather
/// than keeping a private copy.
pub fn fmt_ist(now_ns: u64) -> String {
    let ist_ns = now_ns as i64 + (5 * 3600 + 30 * 60) * 1_000_000_000;
    let days = ist_ns.div_euclid(86_400_000_000_000);
    let ns_of_day = ist_ns.rem_euclid(86_400_000_000_000);
    let (y, m, d) = civil_from_days(days);
    let ms_of_day = ns_of_day / 1_000_000;
    let (hh, mm, ss, ms) = (ms_of_day / 3_600_000, (ms_of_day / 60_000) % 60, (ms_of_day / 1000) % 60, ms_of_day % 1000);
    format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}.{ms:03} IST")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_normal_and_debug_defaulting_to_normal() {
        assert_eq!(LogLevel::parse("normal"), LogLevel::Normal);
        assert_eq!(LogLevel::parse("debug"), LogLevel::Debug);
        assert_eq!(LogLevel::parse("anything-else"), LogLevel::Normal);
    }

    #[test]
    fn fmt_ist_renders_a_known_real_timestamp() {
        // 2026-08-21 10:00:00 IST, the real ENTRY_NS this project's own
        // naturalgas_bracket strategy uses -- cross-checked, not
        // arbitrary.
        assert_eq!(fmt_ist(1_787_286_600_000_000_000), "2026-08-21 10:00:00.000 IST");
    }

    #[test]
    fn line_renders_pre_replay_honestly_when_now_ns_is_none() {
        let l = line("Strategy", None, "START", "fired");
        assert!(l.contains("pre-replay"), "{l}");
        assert!(l.contains("[Strategy] START: fired"), "{l}");
    }

    #[test]
    fn line_embeds_the_real_sim_timestamp_when_given_one() {
        let l = line("ExecutionEngine", Some(1_787_286_600_000_000_000), "Submitted", "client_order_id=1");
        assert!(l.starts_with("t=1787286600000000000 (2026-08-21 10:00:00.000 IST) [ExecutionEngine] Submitted: client_order_id=1"), "{l}");
    }
}

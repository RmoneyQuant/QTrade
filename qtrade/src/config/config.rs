//! The run's declarative config file (D22, D39): one file, two sections
//! -- `[run]` (affects results, hashed into `execution::RunConfig` --
//! see `main.rs`) and `[deployment]` (connectivity/credential detail,
//! never hashed, per D39's own reasoning: "anything that affects results
//! goes in run config... anything that affects only where the process
//! connects goes in deployment config").
//!
//! Hand-rolled parser for exactly this project's own schema -- not
//! general TOML. This crate has had zero external dependencies the
//! whole way through (no `[dependencies]` in `Cargo.toml`); the schema
//! here is small, fixed, and fully under our own control (flat
//! `key = value` pairs under `[section]` headers, no nested tables or
//! arrays needed yet), so a ~100-line parser covers it without pulling
//! in a general-purpose TOML crate. See `config_user_doc.md` for the
//! full schema reference and an example file.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

/// What actually affects a run's results -- hashed into
/// `execution::RunConfig` (see `main.rs`), per D22/D39.
#[derive(Debug, Clone)]
pub struct RunSection {
    /// Only `"backtest"` is implemented. Any other value fails cleanly
    /// at startup rather than silently running a backtest -- the CLI
    /// asking "is this backtest or live" is now this field, not a flag.
    pub mode: String,
    pub session_id: u32,
    /// The capture file to replay. `feed_replay::load_refdata` derives
    /// that day's real `MCXScrips.bcp` path from this filename (FR-16)
    /// -- no separate `contract_file` field needed.
    pub recording_path: String,
    /// Parent of this run's own timestamped output folder (was the
    /// hardcoded `LOG_DIR` constant).
    pub report_dir: String,
    /// 0 = no limit -- stream the whole file, start to end.
    pub max_outer_records: u64,
    /// How many `feed.csv` lines also echo to stdout while a run is in
    /// progress. The complete feed always goes to the file regardless.
    pub max_feed_stdout_lines: usize,
}

/// Connectivity/credential detail -- deliberately empty today. No live
/// feed source or exchange gateway exists anywhere in this codebase yet,
/// so there is nothing real to put here. Kept as its own section (rather
/// than omitted entirely) so a config file authored now, and a
/// `[deployment]` table someone starts filling in ahead of live support,
/// doesn't need restructuring once CTCL credentials/endpoints have a
/// real consumer -- unrecognized keys inside it are accepted and
/// ignored, not rejected.
#[derive(Debug, Clone, Default)]
pub struct DeploymentSection {}

#[derive(Debug, Clone)]
pub struct Config {
    pub run: RunSection,
    pub deployment: DeploymentSection,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

/// Loads and parses a real config file. A missing file, a malformed
/// line, or a missing required `[run]` key is a **hard failure at
/// startup** with a clear diagnostic -- same spirit as FR-16's
/// contract-file mismatch, never a silent default standing in for a
/// value that governs what this run actually does.
pub fn load(path: &Path) -> Result<Config, ConfigError> {
    let text = fs::read_to_string(path).map_err(|e| ConfigError(format!("failed to read config file {}: {e}", path.display())))?;
    parse(&text, path)
}

fn parse(text: &str, path: &Path) -> Result<Config, ConfigError> {
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current: Option<String> = None;

    for (lineno, raw_line) in text.lines().enumerate() {
        // `#` starts a comment to end of line -- crude (a `#` inside a
        // quoted string value would be misread), but every value this
        // schema actually needs (paths, dates, small integers, mode
        // names) is `#`-free in practice, and this keeps the parser
        // small. Documented explicitly in config_user_doc.md.
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            let name = name.trim().to_string();
            sections.entry(name.clone()).or_default();
            current = Some(name);
            continue;
        }
        let Some(section) = current.as_ref() else {
            return Err(ConfigError(format!("{}:{}: key outside any [section]: {line:?}", path.display(), lineno + 1)));
        };
        let Some((key, value)) = line.split_once('=') else {
            return Err(ConfigError(format!("{}:{}: expected `key = value`, got {line:?}", path.display(), lineno + 1)));
        };
        let key = key.trim().to_string();
        let value = unquote(value.trim());
        sections.get_mut(section).unwrap().insert(key, value);
    }

    let run_raw = sections.remove("run").ok_or_else(|| ConfigError(format!("{}: missing required [run] section", path.display())))?;
    // [deployment] is optional and its keys are accepted-but-ignored --
    // see DeploymentSection's own doc comment for why.
    sections.remove("deployment");

    let require = |key: &str| -> Result<String, ConfigError> {
        run_raw.get(key).cloned().ok_or_else(|| ConfigError(format!("{}: [run] missing required key `{key}`", path.display())))
    };
    let parse_int = |raw: &str, key: &str| -> Result<u64, ConfigError> {
        raw.parse::<u64>().map_err(|_| ConfigError(format!("{}: [run].{key} = {raw:?} is not a valid non-negative integer", path.display())))
    };

    let mode = require("mode")?;
    let session_id = parse_int(&require("session_id")?, "session_id")? as u32;
    let recording_path = require("recording_path")?;
    let report_dir = require("report_dir")?;
    let max_outer_records = match run_raw.get("max_outer_records") {
        Some(raw) => parse_int(raw, "max_outer_records")?,
        None => 0,
    };
    let max_feed_stdout_lines = match run_raw.get("max_feed_stdout_lines") {
        Some(raw) => parse_int(raw, "max_feed_stdout_lines")? as usize,
        None => 200,
    };

    Ok(Config {
        run: RunSection { mode, session_id, recording_path, report_dir, max_outer_records, max_feed_stdout_lines },
        deployment: DeploymentSection::default(),
    })
}

/// Strips a surrounding `"..."` if present; a bare token (an integer, or
/// eventually `true`/`false`) is returned as-is. Every value this schema
/// needs is either a quoted string or a bare integer -- no escape
/// sequences, no multi-line strings, none of general TOML's string
/// grammar.
fn unquote(raw: &str) -> String {
    if raw.len() >= 2 && raw.starts_with('"') && raw.ends_with('"') {
        raw[1..raw.len() - 1].to_string()
    } else {
        raw.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dummy_path() -> PathBuf {
        PathBuf::from("test.toml")
    }

    #[test]
    fn parses_a_real_shaped_run_section() {
        let text = r#"
            # a comment line
            [run]
            mode = "backtest"
            session_id = 1
            recording_path = "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin"
            report_dir = "logs/qtrade"

            [deployment]
        "#;
        let cfg = parse(text, &dummy_path()).unwrap();
        assert_eq!(cfg.run.mode, "backtest");
        assert_eq!(cfg.run.session_id, 1);
        assert_eq!(cfg.run.recording_path, "/mnt/MCX_Recording_Files/mcx_feeder_Increment_capture_19_01_2026_1_4.bin");
        assert_eq!(cfg.run.report_dir, "logs/qtrade");
        assert_eq!(cfg.run.max_outer_records, 0, "optional key defaults to 0 (no limit)");
        assert_eq!(cfg.run.max_feed_stdout_lines, 200, "optional key defaults to 200");
    }

    #[test]
    fn optional_keys_override_their_defaults() {
        let text = r#"
            [run]
            mode = "backtest"
            session_id = 1
            recording_path = "x.bin"
            report_dir = "logs"
            max_outer_records = 2000000
            max_feed_stdout_lines = 20
        "#;
        let cfg = parse(text, &dummy_path()).unwrap();
        assert_eq!(cfg.run.max_outer_records, 2_000_000);
        assert_eq!(cfg.run.max_feed_stdout_lines, 20);
    }

    #[test]
    fn missing_run_section_is_a_hard_error() {
        let err = parse("[deployment]\n", &dummy_path()).unwrap_err();
        assert!(err.to_string().contains("missing required [run] section"), "{err}");
    }

    #[test]
    fn missing_required_key_is_a_hard_error() {
        let text = "[run]\nmode = \"backtest\"\n";
        let err = parse(text, &dummy_path()).unwrap_err();
        assert!(err.to_string().contains("missing required key `session_id`"), "{err}");
    }

    #[test]
    fn malformed_line_is_a_hard_error_with_line_number() {
        let text = "[run]\nmode backtest\n";
        let err = parse(text, &dummy_path()).unwrap_err();
        assert!(err.to_string().contains(":2:"), "{err}");
    }

    #[test]
    fn key_outside_any_section_is_a_hard_error() {
        let text = "mode = \"backtest\"\n[run]\n";
        let err = parse(text, &dummy_path()).unwrap_err();
        assert!(err.to_string().contains("outside any [section]"), "{err}");
    }

    #[test]
    fn unrecognized_deployment_keys_are_accepted_and_ignored() {
        let text = r#"
            [run]
            mode = "backtest"
            session_id = 1
            recording_path = "x.bin"
            report_dir = "logs"

            [deployment]
            ctcl_id = "whatever"
            environment = "sim"
        "#;
        // Should not error even though DeploymentSection has no fields
        // for these keys yet -- see its own doc comment.
        parse(text, &dummy_path()).unwrap();
    }

    #[test]
    fn invalid_integer_is_a_hard_error() {
        let text = "[run]\nmode = \"backtest\"\nsession_id = \"not-a-number\"\n";
        let err = parse(text, &dummy_path()).unwrap_err();
        assert!(err.to_string().contains("is not a valid non-negative integer"), "{err}");
    }
}

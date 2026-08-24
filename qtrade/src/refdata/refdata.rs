//! Loads MCX's daily contract file (`MCXScrips.bcp`) into `types::Instrument`
//! records and answers venue/underlying/expiry queries over them.
//!
//! See `refdata_user_doc.md` in this folder for the full column mapping,
//! how to run this standalone against a real file, and the resolution of
//! the DPR-bounds (percentage vs. absolute rupee range) question.
//!
//! Every trading day gets a fresh load. `InstrumentId` is the native
//! exchange token (`SecurityID`) cast directly -- **not** a separately
//! interned dense counter, despite FR-B02's original wording. This used
//! to be a real inconsistency: `refdata` assigned its own dense 0,1,2...
//! numbering here while `book`/`cache`/`simulator`/`execution` all
//! independently settled on "the raw token, cast directly" as their own
//! `InstrumentId` convention (see e.g. `book.rs`'s `CRUDEOIL_ID`/
//! `NATURALGAS_ID` constants). The mismatch meant every caller needing
//! both `refdata`'s metadata and `book`'s book for the same instrument
//! had to manually bridge the two numberings (see `dummy_strategy.rs`'s
//! git history for exactly this, before this fix). Unified here on the
//! convention everything else already used, so there is now exactly one
//! `InstrumentId` space, not two.
//!
//! Note: the token is *not* stable across days (FR-16) -- a fresh load
//! is required every trading day regardless of which convention is used.

use crate::types::{
    ContractFilePaise, Currency, Date, Instrument, InstrumentId, InstrumentKind, Settlement, Venue, YearMonth,
};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

/// MCX's contract file stamps `ExpiryDate` (`parts[54]`) as a Unix-epoch
/// second count that is off by the IST offset -- see refdata_user_doc.md
/// for how this was verified against real rows.
const IST_OFFSET_SECONDS: i64 = 19_800; // 5 hours 30 minutes
const SECONDS_PER_DAY: i64 = 86_400;

/// The minimum number of comma-separated columns a row needs for every
/// index this module reads (highest used is `parts[108]`).
const MIN_COLUMNS: usize = 109;

#[derive(Debug)]
pub enum RefDataError {
    Io(std::io::Error),
}

impl fmt::Display for RefDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefDataError::Io(e) => write!(f, "refdata: failed to read contract file: {e}"),
        }
    }
}

impl std::error::Error for RefDataError {}

impl From<std::io::Error> for RefDataError {
    fn from(e: std::io::Error) -> Self {
        RefDataError::Io(e)
    }
}

/// Parses `MCXScrips.bcp` at `path` into `Instrument` records.
///
/// Only rows that pass the exact acceptance filter from `Contract.cpp`'s
/// `EXCHG_MCX` branch **and** carry `InstrumentType == "FUTCOM"` become an
/// `Instrument` -- D37 implements `Future` only this round, so an
/// accepted row of any other instrument type (option, index future, spot
/// commodity, ...) is silently skipped rather than mis-modelled.
///
/// A malformed line (too few columns, non-numeric token) is skipped, not
/// treated as an error -- the file is allowed to contain rows this
/// component doesn't care about (spreads, other exchanges' quirks never
/// apply here, but defensive skipping costs nothing and keeps a load from
/// panicking on the one field. an operator will actually feed us).
pub fn load_mcx_instruments(path: &Path) -> Result<Vec<Instrument>, RefDataError> {
    let data = fs::read_to_string(path)?;
    let mut out = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < MIN_COLUMNS {
            continue;
        }

        // Token (native id) -- parts[5]. Skip the row if 0.
        let token: i64 = match parts[5].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if token == 0 {
            continue;
        }

        // Filter -- parts[9]==1 && parts[108]==0 && parts[38][0]=='N'.
        // Replicated exactly from Contract.cpp's EXCHG_MCX branch.
        let f9: i64 = match parts[9].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let f108: i64 = match parts[108].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let f38_first = parts[38].as_bytes().first().copied();
        if !(f9 == 1 && f108 == 0 && f38_first == Some(b'N')) {
            continue;
        }

        // InstrumentType -- parts[53], substring to first space. Only
        // "FUTCOM" is handled this round (D37); everything else accepted
        // by the filter above is real but out of scope for now.
        let inst_type = parts[53].split(' ').next().unwrap_or("");
        if inst_type != "FUTCOM" {
            continue;
        }

        // TickSize -- parts[21]. Confirmed empirically to be denominated
        // in *paise* (hundredths of a rupee), not qtrade's internal
        // wire-raw Price scale -- see refdata_user_doc.md's "TickSize
        // units" section for the real-data evidence (every CRUDEOIL-
        // family row reads `100`, every NATURALGAS-family row reads `10`,
        // and GOLD/SILVER/COPPER/ALUMINIUM/ZINC/LEAD each reproduce their
        // real, publicly documented MCX tick size when read as paise).
        // Floored to 5 *paise* (Rs 0.05) if smaller, guarding against a
        // literal-zero or otherwise degenerate column value -- applied to
        // the raw paise value, not the post-conversion wire-raw Price, so
        // the floor's meaning ("smallest tick this loader will ever
        // report is half a rupee-paisa... no, 5 paise") stays legible on
        // its own terms rather than being an arbitrary wire-raw literal.
        // None of the 24 real FUTCOM rows this component targets ever
        // need it (the smallest observed is `5`, already at the floor).
        let tick_paise_raw: i64 = parts[21].trim().parse().unwrap_or(0);
        let tick_paise = if tick_paise_raw < 5 { 5 } else { tick_paise_raw };
        let tick_size = ContractFilePaise(tick_paise).to_wire_price();

        // LotSize -- parts[20].
        let lot_size: i64 = parts[20].trim().parse().unwrap_or(0);

        // ExpiryDate -- parts[54], minus the IST offset if >0 else 0.
        let expiry_raw: i64 = parts[54].trim().parse().unwrap_or(0);
        let expiry_secs = if expiry_raw > 0 {
            expiry_raw - IST_OFFSET_SECONDS
        } else {
            0
        };
        let expiry_days = expiry_secs.div_euclid(SECONDS_PER_DAY);
        let expiry = Date(expiry_days);
        let contract_month = year_month_from_days(expiry_days);

        // Symbol -- parts[6], spaces stripped. Doubles as InstrumentKind::
        // Future's `underlying` -- MCX's contract file has no separate
        // "underlying" column distinct from the per-contract symbol; the
        // symbol here (e.g. "CRUDEOIL" vs "CRUDEOILM") already names the
        // exact tradable product, with expiry carried in a separate
        // column. See refdata_user_doc.md §underlying for the reasoning.
        let symbol: String = parts[6].chars().filter(|c| !c.is_whitespace()).collect();

        // DPR bounds -- parts[64]/parts[65]. Resolved as a *percentage*
        // circuit band, not an absolute rupee range -- see
        // refdata_user_doc.md for the evidence. Since converting a
        // percentage into an absolute (Price, Price) band requires a
        // reference/settlement price this file doesn't hand us in the
        // columns this task scopes, `price_band` is left `None` rather
        // than fabricating a band from the wrong unit.
        let price_band = None;

        let kind = InstrumentKind::Future {
            underlying: symbol,
            expiry,
            contract_month,
            // No settlement-type column in the T01 scope or in
            // Contract.cpp's MCX branch. MCX Crude Oil / Crude Oil Mini
            // and Natural Gas / Natural Gas Mini futures are all cash
            // settled in practice, so `Cash` is used as the default for
            // every FUTCOM row loaded here. Documented, not derived.
            settlement: Settlement::Cash,
        };

        let instrument = Instrument {
            // The native token, cast directly -- the one `InstrumentId`
            // convention used everywhere in qtrade now (see this file's
            // module doc comment for why this used to be a separate,
            // conflicting numbering here).
            id: InstrumentId(token as u32),
            venue: Venue::Mcx,
            native_id: token,
            kind,
            tick_size,
            lot_size,
            // No dedicated "multiplier" column in the T01 scope (the
            // formula Contract.cpp uses for its own `Multiplier` field
            // combines columns not in this task's mapping table and its
            // meaning there is unclear -- see refdata_user_doc.md).
            // Defaulted to `lot_size`, the only defensible value.
            multiplier: lot_size,
            // No freeze-quantity column in the T01 scope, and
            // Contract.cpp's MCX branch never sets one either. Defaulted
            // to 0 and documented as a stub for a later, real need.
            freeze_qty: 0,
            price_band,
            currency: Currency::Inr,
        };

        out.push(instrument);
    }

    Ok(out)
}

/// Days-since-epoch (1970-01-01) -> (year, month). Howard Hinnant's
/// `civil_from_days` algorithm (proleptic Gregorian), used because this
/// crate has no date/time dependency and every date `refdata` needs is a
/// contract expiry derived from an epoch-seconds column -- pulling in a
/// full calendar crate for one conversion is not worth it.
fn year_month_from_days(days: i64) -> YearMonth {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    YearMonth {
        year: y as i32,
        month: m as u8,
    }
}

/// Extracts the field query filters and ordering care about: expiry.
/// `Date(i64::MAX)` for kinds without one so they sort last rather than
/// panicking a query that forgot `.kind_is_future()`.
fn expiry_of(instrument: &Instrument) -> Date {
    match &instrument.kind {
        InstrumentKind::Future { expiry, .. } => *expiry,
        InstrumentKind::Option { expiry, .. } => *expiry,
        _ => Date(i64::MAX),
    }
}

fn underlying_of(instrument: &Instrument) -> Option<&str> {
    match &instrument.kind {
        InstrumentKind::Future { underlying, .. } => Some(underlying.as_str()),
        InstrumentKind::Option { underlying, .. } => Some(underlying.as_str()),
        _ => None,
    }
}

/// The full instrument set for one trading day. `id` is now the native
/// token cast directly (see this module's doc comment) -- large and
/// sparse, not small and dense -- so `get` looks up through `by_id`
/// rather than indexing `instruments` directly by `id.0`. This costs one
/// hash lookup instead of one array index; see `refdata_user_doc.md`'s
/// "on `InstrumentId` unification" section for why that's not a
/// meaningful performance concern in practice (nothing else in qtrade
/// used the old dense numbering for array indexing either).
pub struct InstrumentMaster {
    instruments: Vec<Instrument>,
    by_id: HashMap<InstrumentId, usize>,
}

impl InstrumentMaster {
    pub fn new(instruments: Vec<Instrument>) -> Self {
        let by_id = instruments.iter().enumerate().map(|(idx, i)| (i.id, idx)).collect();
        Self { instruments, by_id }
    }

    /// Convenience: load and wrap in one call.
    pub fn load_mcx(path: &Path) -> Result<Self, RefDataError> {
        Ok(Self::new(load_mcx_instruments(path)?))
    }

    /// All loaded instruments, e.g. for reporting.
    pub fn all(&self) -> &[Instrument] {
        &self.instruments
    }

    /// One hash lookup by native-token id (see this struct's doc comment
    /// on why this is no longer a direct array index).
    pub fn get(&self, id: InstrumentId) -> Option<&Instrument> {
        self.by_id.get(&id).map(|&idx| &self.instruments[idx])
    }

    /// Starts a query -- see `InstrumentQuery` for the STRATEGY-GUIDE.md
    /// §4 call shape this supports.
    pub fn instruments(&self) -> InstrumentQuery<'_> {
        InstrumentQuery {
            items: self.instruments.iter().collect(),
        }
    }
}

/// Builder matching the exact call shape from STRATEGY-GUIDE.md §4 /
/// BACKTEST-PHASE1.md FR-B03:
///
/// ```ignore
/// ctx.instruments()
///    .venue(Venue::Mcx)
///    .underlying("CRUDEOIL")
///    .kind_is_future()
///    .front_n_expiries(2)
///    .collect()
/// ```
///
/// Each step filters (or sorts+truncates) the working set; order between
/// `.venue()`, `.underlying()` and `.kind_is_future()` does not matter,
/// but `.front_n_expiries()` should be called last since it also imposes
/// the final ordering.
pub struct InstrumentQuery<'a> {
    items: Vec<&'a Instrument>,
}

impl<'a> InstrumentQuery<'a> {
    pub fn venue(mut self, venue: Venue) -> Self {
        self.items.retain(|i| i.venue == venue);
        self
    }

    pub fn underlying(mut self, name: &str) -> Self {
        self.items.retain(|i| underlying_of(i) == Some(name));
        self
    }

    pub fn kind_is_future(mut self) -> Self {
        self.items
            .retain(|i| matches!(i.kind, InstrumentKind::Future { .. }));
        self
    }

    /// Sorts the remaining items by expiry ascending and keeps the
    /// nearest `n`. Acceptance (FR-B03): on a day with several live
    /// contracts for one underlying, this returns exactly the `n`
    /// nearest by expiry, in order.
    pub fn front_n_expiries(mut self, n: usize) -> Self {
        self.items.sort_by_key(|i| expiry_of(i));
        self.items.truncate(n);
        self
    }

    /// Terminal call. Returns `InstrumentId`s, not full records -- a
    /// strategy subscribes/depends-on by id (see STRATEGY-GUIDE.md §4)
    /// and resolves metadata back through `InstrumentMaster::get` only
    /// when it actually needs a field, keeping the hot path free of
    /// record copies.
    pub fn collect(self) -> Vec<InstrumentId> {
        self.items.iter().map(|i| i.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_day_zero_is_1970_01_01() {
        let ym = year_month_from_days(0);
        assert_eq!(ym, YearMonth { year: 1970, month: 1 });
    }

    #[test]
    fn known_expiry_days_convert_correctly() {
        // Cross-checked against Python's datetime for the same epoch days.
        assert_eq!(
            year_month_from_days(20654), // 2026-07-20
            YearMonth { year: 2026, month: 7 }
        );
        assert_eq!(
            year_month_from_days(20622), // 2026-06-18
            YearMonth { year: 2026, month: 6 }
        );
        assert_eq!(
            year_month_from_days(20441), // 2025-12-19 boundary sanity check
            YearMonth { year: 2025, month: 12 }
        );
    }
}

#!/usr/bin/env python3
"""Inspect MCX's daily contract file (MCXScrips.bcp) and report what each
of its 118 columns actually holds -- with real values, inferred shapes,
and cross-references to the self-documenting files shipped beside it.

MCX publishes no column layout for this file. Everything annotated as
"known" below was established one of three ways, all recorded inline:
  * read from references/Contract.cpp, which parses this same file
  * mapped already by qtrade/src/refdata/refdata.rs
  * decoded here by matching a published/self-documenting value against
    the data (e.g. freeze quantity, ELM margin) -- see NOTES per column

Usage:
    python3 references/inspect_bcp.py                       # defaults
    python3 references/inspect_bcp.py 21_08_2026 NATURALGAS
    python3 references/inspect_bcp.py 21_08_2026 CRUDEOIL -o out.txt

Read-only: never writes anything under /mnt.
"""
import argparse
import datetime
import os
import sys
from collections import Counter

CONTRACT_ROOT = "/mnt/MCX_Recording_Files/CONTRACT"

# Column meanings established so far. Anything absent is genuinely
# unidentified -- deliberately left blank rather than guessed at.
KNOWN = {
    1:   ("file generation timestamp", "changes daily"),
    2:   ("StreamID -- which capture stream carries this instrument", "references/Contract.cpp"),
    5:   ("Token (native MCX instrument id)", "refdata.rs"),
    6:   ("Symbol", "refdata.rs"),
    9:   ("filter flag (must be 1)", "refdata.rs"),
    20:  ("LotSize", "refdata.rs"),
    21:  ("TickSize (paise)", "refdata.rs"),
    37:  ("trading date", "changes daily"),
    38:  ("'N' filter flag", "refdata.rs"),
    53:  ("InstrumentType", "refdata.rs"),
    54:  ("ExpiryDate (epoch secs, +IST offset)", "refdata.rs"),
    55:  ("StrikePrice", "references/Contract.cpp"),
    56:  ("OptionType", "references/Contract.cpp"),
    62:  ("PriceQuoteQty", "references/Contract.cpp"),
    64:  ("DPR high (%)", "read by refdata.rs, currently unused"),
    65:  ("DPR low (%)", "read by refdata.rs, currently unused"),
    71:  ("FreezeQty (max order size, LOTS)", "verified: lot*col71 == published max order size"),
    77:  ("TradingUnitFactor", "references/Contract.cpp"),
    108: ("spread-type filter (must be 0)", "refdata.rs"),
    114: ("ELM Long (%)", "verified against Margin_Detail_Report.csv"),
    115: ("ELM Short (%)", "verified against Margin_Detail_Report.csv"),
}

EMPTYISH = {"", "0", "0.000", "0.0000", "-1", "-1.0000", "-1.000"}


def shape_of(v):
    v = v.strip()
    if v == "":
        return "empty"
    try:
        n = int(v)
        if 1_600_000_000 < n < 2_000_000_000:
            ts = datetime.datetime.utcfromtimestamp(n).strftime("%Y-%m-%d %H:%M")
            return f"epoch-secs -> {ts}"
        return "int"
    except ValueError:
        pass
    try:
        float(v)
        return "decimal"
    except ValueError:
        return "text"


def load_rows(path):
    rows = []
    with open(path, errors="replace") as f:
        for line in f:
            p = line.rstrip("\n").split(",")
            if len(p) > 108:
                rows.append(p)
    return rows


def find_row(rows, symbol, inst_type="FUTCOM", token=None):
    """Front-month by default: the matching contract with the earliest
    expiry (col 54). A specific `token` overrides that entirely."""
    hits = []
    for p in rows:
        if token is not None:
            if p[5].strip() == str(token):
                return p
            continue
        if p[6].strip() == symbol and p[53].strip().startswith(inst_type):
            try:
                exp = int(p[54].strip())
            except ValueError:
                exp = 1 << 62
            hits.append((exp, p))
    if not hits:
        return None
    hits.sort(key=lambda t: t[0])
    return hits[0][1]


def margin_rows_for(date_dir, symbol):
    path = os.path.join(date_dir, "Margin_Detail_Report.csv")
    if not os.path.exists(path):
        return None, []
    with open(path, errors="replace") as f:
        header = f.readline().rstrip("\n").split(",")
        hits = [l.rstrip("\n").split(",") for l in f if len(l.split(",")) > 3 and l.split(",")[3].strip() == symbol]
    return header, hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("date", nargs="?", default="21_08_2026", help="DD_MM_YYYY")
    ap.add_argument("symbol", nargs="?", default="NATURALGAS")
    ap.add_argument("-t", "--token", default=None, help="exact native token, overrides symbol lookup")
    ap.add_argument("-c", "--compare", default=None, help="second date to diff against (DD_MM_YYYY)")
    ap.add_argument("-o", "--out", default=None, help="output file (default: references/bcp_report_<symbol>_<date>.txt)")
    a = ap.parse_args()

    date_dir = os.path.join(CONTRACT_ROOT, a.date)
    bcp = os.path.join(date_dir, "MCXScrips.bcp")
    if not os.path.exists(bcp):
        sys.exit(f"no contract file at {bcp}")

    out_path = a.out or os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                     f"bcp_report_{a.symbol}_{a.date}.txt")
    o = open(out_path, "w")
    def w(s=""):
        o.write(s + "\n")

    rows = load_rows(bcp)
    row = find_row(rows, a.symbol, token=a.token)
    if row is None:
        sys.exit(f"no FUTCOM row for {a.symbol} in {bcp}")
    ncol = len(row)

    w("=" * 100)
    w(f"MCX contract-file inspection -- {a.symbol} on {a.date}")
    w("=" * 100)
    w(f"file        : {bcp}")
    w(f"file size   : {os.path.getsize(bcp):,} bytes")
    w(f"rows parsed : {len(rows):,}")
    w(f"columns     : {ncol}")
    w(f"token       : {row[5].strip()}   expiry col54: {row[54].strip()}")
    w()
    w("MCX publishes no column layout for this file. Columns marked (known) were")
    w("established from Contract.cpp, from qtrade's own refdata.rs, or by matching a")
    w("published value against the data. Unmarked columns are genuinely unidentified.")
    w()

    # ---- section 1: every column of this instrument's row --------------
    w("-" * 100)
    w("SECTION 1 -- every column of this instrument's row")
    w("-" * 100)
    w(f"{'col':>4}  {'raw value':<28} {'shape':<28} meaning")
    w("-" * 100)
    for i, v in enumerate(row):
        v = v.strip()
        blank = v in EMPTYISH
        meaning = ""
        if i in KNOWN:
            name, src = KNOWN[i]
            meaning = f"(known) {name}  [{src}]"
        elif blank:
            meaning = "-- empty/zero --"
        w(f"{i:>4}  {v[:28]:<28} {shape_of(v):<28} {meaning}")
    w()

    # ---- section 2: which columns actually carry per-instrument data ---
    # Only rows of the *same* instrument type as the one selected. Mixing
    # futures with options here is actively misleading: the two use several
    # columns for entirely different things (cols 64/65 are a circuit-band
    # percentage on FUTCOM rows but hold strike-scale values on option rows,
    # which made them look like 14,803-distinct-value noise when pooled).
    kind = row[53].strip()
    peers = [r for r in rows if len(r) > 53 and r[53].strip() == kind]
    w("-" * 100)
    w(f"SECTION 2 -- variability across the {len(peers):,} '{kind}' rows in this file")
    w(f"  (same instrument type only -- pooling futures with options misreads shared columns)")
    w("  constant  = same value in every such row (type-level flag or unused)")
    w("  varies    = genuinely carries per-instrument data")
    w("-" * 100)
    w(f"{'col':>4}  {'distinct':>8}  {'verdict':<10} sample values")
    w("-" * 100)
    for i in range(ncol):
        vals = Counter(r[i].strip() for r in peers if len(r) > i)
        distinct = len(vals)
        verdict = "constant" if distinct == 1 else "varies"
        sample = ", ".join(f"{v!r}x{c}" for v, c in vals.most_common(3))
        w(f"{i:>4}  {distinct:>8}  {verdict:<10} {sample[:70]}")
    w()

    # ---- section 3: margin file cross-reference ------------------------
    w("-" * 100)
    w("SECTION 3 -- Margin_Detail_Report.csv (self-documenting, same folder)")
    w("  Use this to decode .bcp columns: any value here that also appears in")
    w("  section 1 identifies that column. This is how cols 114/115 were decoded.")
    w("-" * 100)
    header, hits = margin_rows_for(date_dir, a.symbol)
    if header is None:
        w("  (no Margin_Detail_Report.csv in this date folder)")
    else:
        for h in hits[:6]:
            w(f"  {a.symbol} expiry {h[4].strip()}:")
            for name, val in zip(header, h):
                if val.strip() not in ("0", ""):
                    w(f"      {name.strip():<32} = {val.strip()}")
            w()

    # ---- section 4: other files shipped alongside ----------------------
    w("-" * 100)
    w("SECTION 4 -- other files in this date folder")
    w("-" * 100)
    for fn in sorted(os.listdir(date_dir)):
        fp = os.path.join(date_dir, fn)
        if os.path.isfile(fp):
            w(f"  {fn:<28} {os.path.getsize(fp):>12,} bytes")
    w()

    # ---- section 5: day-over-day diff ---------------------------------
    if a.compare:
        other = os.path.join(CONTRACT_ROOT, a.compare, "MCXScrips.bcp")
        w("-" * 100)
        w(f"SECTION 5 -- what changes between {a.compare} and {a.date} for this contract")
        w("-" * 100)
        if not os.path.exists(other):
            w(f"  (no file at {other})")
        else:
            row2 = find_row(load_rows(other), a.symbol, token=row[5].strip())
            if row2 is None:
                w(f"  ({a.symbol} not present on {a.compare})")
            else:
                diffs = [(i, row2[i].strip(), row[i].strip())
                         for i in range(min(len(row), len(row2)))
                         if row2[i].strip() != row[i].strip()]
                w(f"  {len(diffs)} of {ncol} columns differ")
                for i, old, new in diffs:
                    label = KNOWN.get(i, ("", ""))[0]
                    w(f"    col {i:>3}: {old!r} -> {new!r}   {label}")
        w()

    o.close()
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()

//! The paired-difference readout over two per-row correctness vectors —
//! the T4 frozen-read's stats half (riir-train Issue 623), reusing the
//! house `stats::paired_upper_bound` verbatim so the interval cannot
//! drift from the gate sheet's math (the arena's superiority face and
//! the T2/T5 verdicts all read the same function).
//!
//! Inputs: two JSON files, each a flat array of booleans IN THE SAME
//! ROW ORDER (the caller owns the case-identity pin — the C2
//! `cases_digest` equality — this tool deliberately does not re-derive
//! it). A = the challenger, B = the baseline; the diff is (a − b) per
//! row. Output: one JSON object {n, mean, se, lb95, ub95} on stdout.
//!
//! Usage:
//!   cargo run --release --example paired_lb95 -- --a student.json --b bar.json
//!
//! Exit 0 on a computed verdict; a length mismatch or an empty vector
//! refuses exit 2 (never a fabricated interval).

use std::io::Write;

fn read_bools(path: &str) -> Result<Vec<bool>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("read {path}: {e}"))?;
    let v: Vec<bool> = serde_json::from_str(&text)
        .map_err(|e| format!("parse {path} as [bool]: {e}"))?;
    Ok(v)
}

fn main() {
    let mut a_path: Option<String> = None;
    let mut b_path: Option<String> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--a" => a_path = Some(args.next().unwrap_or_else(|| die("--a needs a path"))),
            "--b" => b_path = Some(args.next().unwrap_or_else(|| die("--b needs a path"))),
            other => die(&format!("unknown arg {other} (--a PATH --b PATH)")),
        }
    }
    let (Some(a_path), Some(b_path)) = (a_path, b_path) else {
        die("usage: paired_lb95 --a CHALLENGER.json --b BASELINE.json");
    };

    let a = read_bools(&a_path).unwrap_or_else(|e| die(&e));
    let b = read_bools(&b_path).unwrap_or_else(|e| die(&e));
    let Some(pd) = riir_instinct::stats::paired_upper_bound(&a, &b) else {
        die(&format!(
            "paired_upper_bound refused: len mismatch ({} vs {}) or empty — \
             fix the row join before reading an interval",
            a.len(),
            b.len()
        ));
    };
    let out = serde_json::json!({
        "n": a.len(),
        "a": a_path,
        "b": b_path,
        "mean": pd.mean,
        "se": pd.se,
        "lb95": pd.lb95,
        "ub95": pd.ub95,
    });
    let mut stdout = std::io::stdout().lock();
    let _ = writeln!(stdout, "{out}");
}

fn die(msg: &str) -> ! {
    eprintln!("paired_lb95: {msg}");
    std::process::exit(2);
}

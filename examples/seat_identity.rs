//! Seat identity dump — the plan-011 C2 case-identity pin's offline half.
//!
//! Rebuilds a suite's seat through the SAME reflex `prepare_seat` the arena
//! and the harness use and prints the population identity: the seat's
//! `cases_digest` (the reflex `SuiteResult.cases_digest` law) plus the
//! per-question gold indices in read order.
//!
//! What this proves WITHOUT re-running any arm: the seat build is a pure
//! function of the datasets pool + the sampling law, so a dump whose
//! digest equals a frozen record's `test_digest` (or a reflex-side doc's
//! `cases_digest`) establishes that the record answered EXACTLY those
//! cases in EXACTLY that order — the gold array then joins the frozen
//! `picks`/`correct` soundly, which is how the macro-F1/JDI-skill columns
//! are derived for records frozen before gold was stamped (the
//! build_hybrid_doc.py `--gold-from` path).
//!
//! Usage:
//!   cargo run --release --example seat_identity -- <suite> [datasets_dir]
//!
//! Output: one JSON object on stdout (everything else to stderr).

use riir_reflex::harness::runner::seat::prepare_seat;

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(suite) = args.next() else {
        eprintln!("usage: seat_identity <suite> [datasets_dir]");
        std::process::exit(2);
    };
    let datasets_dir = args
        .next()
        .unwrap_or_else(|| "../riir-reflex/.raw/datasets_t20k".to_string());
    eprintln!("seating {suite} from {datasets_dir} …");
    let seat = match prepare_seat(&suite, std::path::Path::new(&datasets_dir)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("seat_identity: {e}");
            std::process::exit(1);
        }
    };
    let n_cases = seat.suite.cases.len();
    let gold: Vec<usize> = seat
        .suite
        .cases
        .iter()
        .flat_map(|c| c.gold.iter().map(|g| g.idx))
        .collect();
    let doc = serde_json::json!({
        "suite": suite,
        "datasets_dir": datasets_dir,
        "n_cases": n_cases,
        "n_questions": gold.len(),
        "cases_digest": seat.cases_digest(),
        "gold": gold,
    });
    println!("{}", serde_json::to_string_pretty(&doc).expect("doc json"));
}

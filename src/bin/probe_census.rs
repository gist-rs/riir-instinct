//! The issue-028 G0 collision census runner (T1): the un-collided oracle
//! ceiling per suite, machine-recorded. Whole-target gated
//! (`required-features = ["probe_repair"]` — the repo-birth gate
//! discipline; a plain build never resolves the gated module).
//!
//! Writes `.benchmarks/<next>_g0_collision_census/{results.json,
//! record.md}` — numbering read from `.benchmarks/.highwater`, value + 1,
//! written back (the monotonic law). Record-only: exit 0 on a completed
//! census whatever the verdicts (the FOLD decision is adjudicated in the
//! issue from the record, not by an exit code); a failed suite LOAD exits
//! 1 loud with the suite named.
//!
//! ```text
//! cargo run --release --features probe_repair --bin probe_census -- \
//!     [--datasets ../riir-reflex/.raw/datasets] [--suites ag_news,...] [--out <dir>]
//! ```

use std::path::PathBuf;

use riir_instinct::instinct_probe_repair::{
    CensusConfig, DEFAULT_SUITES, RecordMeta, census_suite, render_record,
};

fn main() {
    let mut datasets = PathBuf::from("../riir-reflex/.raw/datasets");
    let mut suites: Vec<String> = DEFAULT_SUITES.iter().map(|s| s.to_string()).collect();
    let mut out: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--datasets" => {
                datasets = PathBuf::from(args.next().expect("--datasets needs a value"))
            }
            "--suites" => {
                let csv = args.next().expect("--suites needs a csv");
                suites = csv
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect();
                if suites.is_empty() {
                    eprintln!("--suites parsed to zero suites");
                    std::process::exit(2);
                }
            }
            "--out" => out = Some(PathBuf::from(args.next().expect("--out needs a value"))),
            other => {
                eprintln!("unknown arg {other:?} — see the module doc for the surface");
                std::process::exit(2);
            }
        }
    }

    // Numbering: read .benchmarks/.highwater, use value + 1, write back.
    let hw_path = PathBuf::from(".benchmarks/.highwater");
    let hw = std::fs::read_to_string(&hw_path).unwrap_or_else(|e| {
        eprintln!(
            "read {}: {e} — the numbering law needs the counter",
            hw_path.display()
        );
        std::process::exit(1);
    });
    let hw_trim = hw.trim();
    let next: u64 = hw_trim.parse().unwrap_or_else(|e| {
        eprintln!("parse .highwater {hw_trim:?}: {e}");
        std::process::exit(1);
    });
    let next_padded = format!("{next:04}");
    let out_dir = out
        .unwrap_or_else(|| PathBuf::from(format!(".benchmarks/{next_padded}_g0_collision_census")));
    std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
        eprintln!("create {}: {e}", out_dir.display());
        std::process::exit(1);
    });
    std::fs::write(&hw_path, format!("{}\n", next + 1)).unwrap_or_else(|e| {
        eprintln!("write .highwater: {e}");
        std::process::exit(1);
    });
    println!(
        "record dir: {} (highwater {hw_trim} → {:04})",
        out_dir.display(),
        next + 1
    );

    let cfg = CensusConfig::default();
    let mut results = Vec::new();
    for suite in &suites {
        print!("census {suite} ... ");
        use std::io::Write as _;
        let _ = std::io::stdout().flush();
        match census_suite(&datasets, suite, &cfg) {
            Ok(c) => {
                println!(
                    "{} flagged / {} remapped | incumbent {:.4} oracle {:.4} | Δ {:+.2} pt | {:?}",
                    c.flagged_buckets.len(),
                    c.remapped_members,
                    c.incumbent.holdout_acc,
                    c.oracle.holdout_acc,
                    c.paired.delta * 100.0,
                    c.verdict,
                );
                results.push(c);
            }
            Err(e) => {
                eprintln!("\nFAIL: suite {suite}: {e}");
                std::process::exit(1);
            }
        }
    }

    let meta = RecordMeta {
        datasets_dir: datasets.display().to_string(),
        s_min: cfg.s_min,
        holdout_budget: cfg.holdout_budget,
        material_pt: riir_instinct::instinct_probe_repair::MATERIAL_PT,
        trainer: "instinct_specialist::train_arm_a(_remap) @ ArmAConfig::default() (the Bench-608 recipe)"
            .into(),
    };
    let json = serde_json::to_string_pretty(&results).expect("results serialize");
    let json_path = out_dir.join("results.json");
    std::fs::write(&json_path, &json).unwrap_or_else(|e| {
        eprintln!("write {}: {e}", json_path.display());
        std::process::exit(1);
    });
    let md = render_record(&results, &meta);
    let md_path = out_dir.join("record.md");
    std::fs::write(&md_path, &md).unwrap_or_else(|e| {
        eprintln!("write {}: {e}", md_path.display());
        std::process::exit(1);
    });
    let material: Vec<&str> = results
        .iter()
        .filter(|c| c.verdict == riir_instinct::instinct_probe_repair::SuiteVerdict::Material)
        .map(|c| c.suite.as_str())
        .collect();
    if material.is_empty() {
        println!(
            "FOLD VERDICT: every suite immaterial at 2^17 — class 2 folds (record-only; adjudicate in the issue)"
        );
    } else {
        println!("FOLD VERDICT: class 2 stays open — material: {material:?}");
    }
    println!("wrote {} + {}", json_path.display(), md_path.display());
}

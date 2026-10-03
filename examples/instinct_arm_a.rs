//! Issue 576 T2 — Arm A over the six above-gate arena suites: load TRAIN
//! rows only, label-stratified round-robin holdout (train-side selection),
//! train the one-vs-all logistic ([`riir_instinct::instinct_specialist`]),
//! and report per suite: rows, labels, holdout accuracy, the lr=0 control,
//! wall time, and the frozen artifact (BLAKE3 seal) written under
//! `--out` (default `data/trained_specialists/` — gitignored; weights are
//! never committed).
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_arm_a -- \
//!     [--suites ag_news,emotion] [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--epochs 3] [--holdout 200] [--out data/trained_specialists]
//! ```
//!
//! The lr=0 control is computed, not trained: zero parameters pick class 0,
//! so the control is exactly the holdout share of class 0 — every trained
//! row must beat it (asserted here; a suite that fails is printed LOUD and
//! fails the run).

use riir_instinct::instinct_specialist::{
    decode_artifact, encode_artifact, label_universe, load_suite_rows, stratified_holdout,
    train_arm_a, ArmAConfig,
};

const DEFAULT_SUITES: [&str; 6] = [
    "ag_news",
    "emotion",
    "sst5",
    "massive_intent_en",
    "banking77",
    "xnli_en",
];

struct Args {
    suites: Vec<String>,
    datasets: String,
    epochs: usize,
    holdout: usize,
    out: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        suites: DEFAULT_SUITES.iter().map(|s| (*s).to_string()).collect(),
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        epochs: 3,
        holdout: 200,
        out: "data/trained_specialists".into(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let val = |i: &mut usize, name: &str| -> String {
            *i += 1;
            args.get(*i)
                .unwrap_or_else(|| panic!("{name} needs a value"))
                .clone()
        };
        match args[i].as_str() {
            "--suites" => {
                a.suites = val(&mut i, "--suites").split(',').map(str::to_string).collect();
            }
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--epochs" => a.epochs = val(&mut i, "--epochs").parse().expect("--epochs number"),
            "--holdout" => {
                a.holdout = val(&mut i, "--holdout").parse().expect("--holdout number")
            }
            "--out" => a.out = val(&mut i, "--out"),
            other => panic!("unknown flag {other}"),
        }
        i += 1;
    }
    a
}

fn main() {
    let a = parse_args();
    let dir = std::path::PathBuf::from(&a.datasets);
    println!(
        "instinct_arm_a: datasets {} · suites {:?} · epochs {} · holdout {}",
        dir.display(),
        a.suites,
        a.epochs,
        a.holdout
    );
    let mut failures = 0usize;
    for suite in &a.suites {
        let t0 = std::time::Instant::now();
        let rows = match load_suite_rows(&dir, suite) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  {suite}: LOAD FAILED — {e}");
                failures += 1;
                continue;
            }
        };
        let labels = label_universe(&rows);
        let (holdout, train) = stratified_holdout(&rows, a.holdout);
        let cfg = ArmAConfig { epochs: a.epochs, ..ArmAConfig::default() };
        let run = match train_arm_a(&rows, &labels, &train, &holdout, &cfg) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  {suite}: TRAIN FAILED — {e}");
                failures += 1;
                continue;
            }
        };
        let secs = t0.elapsed().as_secs_f64();
        let ok = run.holdout_accuracy > run.control_accuracy;
        if !ok {
            failures += 1;
        }
        println!(
            "  {suite}: rows {} · labels {} · train {} · holdout {} · steps {} · \
             loss {:.4} · holdout acc {:.4} (lr0 control {:.4}) {} · skipped_empty {} · \
             {secs:.1}s",
            rows.len(),
            labels.len(),
            train.len(),
            holdout.len(),
            run.steps,
            run.final_loss,
            run.holdout_accuracy,
            run.control_accuracy,
            if ok { "OK" } else { "FAILED the lr0 control — LOUD" },
            run.skipped_empty,
        );
        // Export + verify (the producer's artifact must decode; weights are
        // bytes on disk, never committed).
        let bytes = encode_artifact(suite, &run.model);
        let seal = blake3::hash(&bytes);
        if let Err(e) = decode_artifact(&bytes) {
            eprintln!("  {suite}: ARTIFACT SELF-CHECK FAILED — {e}");
            failures += 1;
            continue;
        }
        std::fs::create_dir_all(&a.out).expect("create out dir");
        let path = std::path::Path::new(&a.out).join(format!("{suite}_armA_v1.bin"));
        std::fs::write(&path, &bytes).expect("write artifact");
        println!(
            "  {suite}: artifact {} ({} bytes, blake3 {}) — decoded OK",
            path.display(),
            bytes.len(),
            &seal.to_hex()[..16]
        );
    }
    if failures > 0 {
        eprintln!("instinct_arm_a: {failures} failure(s) — LOUD");
        std::process::exit(1);
    }
    println!(
        "instinct_arm_a: PASSED — {} suite(s), every trained row beat its lr=0 control",
        a.suites.len()
    );
}

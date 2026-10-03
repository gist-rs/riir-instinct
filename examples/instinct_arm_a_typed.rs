//! Issue 578 T1 — Arm A over the typed_decisions option-conditioned rows
//! ([`riir_instinct::instinct_specialist_typed`]): load TRAIN rows
//! only, a workflow-stratified ROW-level holdout (every event of a
//! holdout row stays holdout — the serve-time unseen-state direction),
//! train the one-vs-all logistic over an epochs grid, and report — in the
//! serving bridge's option-restricted space — the event-level and
//! case-level (question[0]) holdout accuracies beside the lr=0 control
//! and the workflow-majority baseline.
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_arm_a_typed -- \
//!     [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--epochs 2,3,5,8] [--holdout-rows 160] [--out data/trained_specialists]
//! ```
//!
//! Selection key: the CASE metric (the arena's per-case question[0]
//! space — the thing the superiority gate will later read). Hard gates:
//! the selected run must beat BOTH the lr=0 control (event space) and
//! the workflow-majority baseline (case space); a miss is a recorded
//! NEGATIVE — the artifact is still written under the armA name, the
//! winner rename belongs to the consumer landing (riir-train Issue 578
//! T3 + the riir-instinct typed bridge; minting the winner name first
//! errors the arena's typed_decisions run via the shape-law gate).

use riir_instinct::instinct_specialist::{
    decode_artifact, encode_artifact, label_universe, train_arm_a, ArmAConfig,
};
use riir_instinct::instinct_specialist_typed::{
    load_typed_rows, typed_case_eval, typed_case_majority_baseline, typed_event_eval,
    typed_row_holdout,
};

struct Args {
    datasets: String,
    epochs: Vec<usize>,
    holdout_rows: usize,
    out: String,
    lr: f32,
    weight_decay: f32,
}

fn parse_args() -> Args {
    let mut a = Args {
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        epochs: vec![2, 3, 5, 8],
        holdout_rows: 160,
        out: "data/trained_specialists".into(),
        lr: 0.05,
        weight_decay: 0.01,
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        let val = |i: &mut usize, name: &str| -> String {
            *i += 1;
            args.get(*i)
                .unwrap_or_else(|| panic!("{name} needs a value"))
                .clone()
        };
        match args[i].as_str() {
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--epochs" => {
                a.epochs = val(&mut i, "--epochs")
                    .split(',')
                    .map(|s| s.parse().expect("--epochs numbers"))
                    .collect();
            }
            "--holdout-rows" => {
                a.holdout_rows = val(&mut i, "--holdout-rows")
                    .parse()
                    .expect("--holdout-rows number")
            }
            "--lr" => a.lr = val(&mut i, "--lr").parse().expect("--lr number"),
            "--wd" => {
                a.weight_decay = val(&mut i, "--wd").parse().expect("--wd number")
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
        "instinct_arm_a_typed: datasets {} · epochs {:?} · holdout {} rows · lr {} · wd {}",
        dir.display(),
        a.epochs,
        a.holdout_rows,
        a.lr,
        a.weight_decay
    );
    let t0 = std::time::Instant::now();
    let rows = load_typed_rows(&dir).expect("typed_decisions train rows load");
    let n_events: usize = rows.iter().map(|r| r.events.len()).sum();
    let mut wf_tally: Vec<(String, usize)> = Vec::new();
    for r in &rows {
        match wf_tally.iter_mut().find(|(w, _)| w == &r.workflow) {
            Some((_, n)) => *n += 1,
            None => wf_tally.push((r.workflow.clone(), 1)),
        }
    }
    wf_tally.sort();
    println!(
        "  rows {} · workflows {:?} · scorable events {}",
        rows.len(),
        wf_tally,
        n_events
    );

    // The ROW-level workflow-stratified holdout.
    let (holdout_rows, train_rows) = typed_row_holdout(&rows, a.holdout_rows);
    println!(
        "  holdout: {} rows · train: {} rows (workflow round-robin)",
        holdout_rows.len(),
        train_rows.len()
    );

    // Event SuiteRows in TRAIN-then-HOLDOUT order; the class universe is
    // TRAIN-derived (a holdout label outside it would panic the trainer's
    // class_of lookup — asserted here as the data-honesty check first).
    let mut train_events: Vec<_> = train_rows
        .iter()
        .flat_map(|&i| rows[i].event_rows())
        .collect();
    let holdout_events: Vec<_> = holdout_rows
        .iter()
        .flat_map(|&i| rows[i].event_rows())
        .collect();
    let mut labels = label_universe(&train_events);
    // The universe must also carry every PRESENTED key — a distractor
    // option can be never-gold in train (measured: customer_service
    // `action` presents `close_no_action`, 0/300 gold), and a missing row
    // would leave every case presenting it unanswered. Extending the
    // universe (NOT adding filler rows) is what makes the distractor
    // trainable: the trainer's dense per-class update gives it the same
    // negative exposure as every other non-gold class, so it can never
    // dominate through zero-init (an under-trained class scores ABOVE any
    // trained non-gold class — the bias-drift trap).
    for r in &rows {
        for e in &r.events {
            for k in e.positions() {
                if !labels.contains(&k) {
                    labels.push(k);
                }
            }
        }
    }
    labels.sort();
    let missing: Vec<&String> = holdout_events
        .iter()
        .filter(|e| !labels.contains(&e.label))
        .map(|e| &e.label)
        .collect();
    assert!(
        missing.is_empty(),
        "holdout carries labels the train universe never saw: {missing:?} — \
         the templated-question premise is broken; investigate before training"
    );
    // Same completeness claim for PRESENTED keys: the templates are
    // workflow-fixed, so a holdout-presented key the train set never
    // presents is a template drift, not noise.
    let missing_presented: Vec<String> = holdout_rows
        .iter()
        .flat_map(|&i| rows[i].events.iter())
        .flat_map(|e| e.positions())
        .filter(|k| !labels.contains(k))
        .collect();
    assert!(
        missing_presented.is_empty(),
        "holdout presents keys outside the trained universe: {missing_presented:?}"
    );
    let n_train = train_events.len();
    let mut all = std::mem::take(&mut train_events);
    all.extend(holdout_events);
    let train_idx: Vec<usize> = (0..n_train).collect();
    let holdout_idx: Vec<usize> = (n_train..all.len()).collect();
    println!(
        "  events: {} train · {} holdout · {} classes",
        n_train,
        all.len() - n_train,
        labels.len()
    );

    // The majority baseline (case space) is config-independent.
    let majority = typed_case_majority_baseline(&rows, &train_rows, &holdout_rows);
    println!(
        "  workflow-majority baseline (case space): {:.4} ({}/{})",
        majority.accuracy(),
        majority.correct,
        majority.total
    );

    // The grid. Selection key: case accuracy, then event accuracy, then
    // fewer epochs.
    let mut best: Option<(usize, f64, f64, f64)> = None; // (epochs, case, event, control)
    for &epochs in &a.epochs {
        let cfg = ArmAConfig {
            epochs,
            lr: a.lr,
            weight_decay: a.weight_decay,
            ..ArmAConfig::default()
        };
        let run = train_arm_a(&all, &labels, &train_idx, &holdout_idx, &cfg)
            .unwrap_or_else(|e| panic!("epochs {epochs}: train failed: {e}"));
        let ev = typed_event_eval(&run.model, &rows, &holdout_rows);
        let case = typed_case_eval(&run.model, &rows, &holdout_rows);
        // Per-workflow case split (diagnosis: where the case metric loses).
        for (wf, _) in &wf_tally {
            let idxs: Vec<usize> = holdout_rows
                .iter()
                .copied()
                .filter(|&i| rows[i].workflow == *wf)
                .collect();
            let w = typed_case_eval(&run.model, &rows, &idxs);
            println!(
                "    case[{wf}]: {:.4} ({}/{})",
                w.accuracy(),
                w.correct,
                w.total
            );
        }
        println!(
            "  epochs {epochs}: case {:.4} ({}/{}) · event {:.4} ({}/{}, \
             unanswered {}) · unrestricted {:.4} (lr0 control {:.4}) · loss {:.4}",
            case.accuracy(),
            case.correct,
            case.total,
            ev.accuracy(),
            ev.correct,
            ev.total,
            ev.unanswered,
            run.holdout_accuracy,
            run.control_accuracy,
            run.final_loss
        );
        let better = match best {
            None => true,
            Some((_, bc, be, _)) => {
                (case.accuracy(), ev.accuracy()) > (bc, be)
            }
        };
        if better {
            best = Some((epochs, case.accuracy(), ev.accuracy(), run.control_accuracy));
            let bytes = encode_artifact("typed_decisions", &run.model);
            decode_artifact(&bytes).expect("producer artifact must decode");
            std::fs::create_dir_all(&a.out).expect("create out dir");
            let path = std::path::Path::new(&a.out).join("typed_decisions_armA_v1.bin");
            std::fs::write(&path, &bytes).expect("write artifact");
            let seal = blake3::hash(&bytes);
            println!(
                "  typed_decisions: artifact {} ({} bytes, blake3 {}) — decoded OK",
                path.display(),
                bytes.len(),
                &seal.to_hex()[..16]
            );
        }
    }
    let secs = t0.elapsed().as_secs_f64();
    let Some((epochs, case, event, control)) = best else {
        eprintln!("instinct_arm_a_typed: no config trained — LOUD");
        std::process::exit(1);
    };
    // Hard gates: the selected run must beat the lr0 control (event
    // space) AND the workflow-majority baseline (case space).
    let beat_control = event > control;
    let beat_majority = case > majority.accuracy();
    println!(
        "  selected epochs {epochs}: case {case:.4} vs majority {:.4} · \
         event {event:.4} vs control {control:.4} · {secs:.1}s",
        majority.accuracy()
    );
    if !beat_control || !beat_majority {
        eprintln!(
            "instinct_arm_a_typed: NEGATIVE — beat_control={beat_control} \
             beat_majority={beat_majority}; record the negative in riir-train \
             Issue 578 and do NOT mint the winner name"
        );
        std::process::exit(1);
    }
    println!(
        "instinct_arm_a_typed: PASSED — typed_decisions artifact selected \
         (epochs {epochs}); the winner rename waits for the consumer bridge \
         (riir-train Issue 578 T3 + the riir-instinct typed bridge)"
    );
}

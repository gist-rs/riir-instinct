//! Issue 579 T1/T2 — the v2 lane runner (emotion, and banking77's
//! Arm-A-only arm). Two stages, one command:
//!
//! 1. **The baseline validation** (issue 579 T1 step 1, the ONE sanctioned
//!    test-split read): the [`riir_instinct::instinct_nbsvm::NbsvmRidge`]
//!    port is fit on the probe's pool (train minus the 8-per-label
//!    stratified front) and read pure-argmax on the emotion test split —
//!    the anchor is the probe's recorded **0.8925** (k=2048, λ=10,
//!    per-class quirk ratios). A hard floor below `VALIDATION_FLOOR` is
//!    the reflex 0.29-class port bug tripwire — the run refuses instead of
//!    publishing a bar nobody validated.
//! 2. **The holdout gate** (issue 579 T1 steps 2–3, all train-side): the
//!    label-stratified round-robin holdout splits train; the ridge
//!    baseline (same posture, fit on the rest) and the v2 sweep
//!    ([`riir_instinct::instinct_nbsvm::train_nbsvm`]) are scored on
//!    the holdout; the best v2 faces the GATE arm through the paired
//!    LB95 gate — the arm is SUITE-NAMED: emotion's reflex row IS the
//!    ridge blend (gate vs the ridge baseline); banking77's published
//!    0.8260 is the modelless lane and its ridge ladder DECLINED
//!    (Bench 057), so the incumbent to beat is the v1 specialist whose
//!    composition was already refused at the LB95 gate. v1 is reported
//!    for context either way. No test row is read in this stage — the
//!    arena's single frozen test read (T3, the instinct bridge) decides
//!    registration.
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_emotion_v2 -- \
//!     [--suite emotion|banking77] [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--holdout 1000] [--front-per-label 8] [--lambda 10] [--k 2048]
//! ```

use riir_instinct::instinct_nbsvm::{
    load_suite_split, paired_lb95, presence_bag_into, presence_ids_into, probe_front_split,
    train_nbsvm, NbsvmConfig, NbsvmRidge,
};
use riir_instinct::instinct_specialist::{
    bag_into, decode_artifact, encode_artifact, label_universe, load_suite_rows,
    stratified_holdout, train_arm_a, ArmAConfig, SuiteRow,
};

/// The validation hard floor: far below the anchor AND far above the
/// 0.29-class port bug. The exact read is printed for the record.
const VALIDATION_FLOOR: f64 = 0.87;

/// The probe-recorded pure-argmax anchors (`issue038_t7_probe*.py`), per
/// suite: emotion 0.8925 (the issue-579 step-1 anchor); banking77 0.8384
/// (the declined-ladder probe — reflex's own records).
fn probe_anchor(suite: &str) -> Option<f64> {
    match suite {
        "emotion" => Some(0.8925),
        "banking77" => Some(0.8384),
        _ => None,
    }
}

/// The suite's reflex row core, which names the GATE arm: emotion's
/// published 0.8850 IS the ridge blend (gate v2 vs the ridge baseline);
/// banking77's published 0.8260 is the MODELLESS lane — the ridge ladder
/// DECLINED there (Bench 057) — so the incumbent to beat train-side is
/// the v1 Arm A specialist (whose H1 composition was already refused vs
/// 0.8260 at the LB95 gate).
fn gate_vs_v1(suite: &str) -> bool {
    suite == "banking77"
}

struct Args {
    datasets: String,
    suite: String,
    holdout: usize,
    front_per_label: usize,
    lambda: f32,
    k: usize,
    out: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        suite: "emotion".into(),
        holdout: 1000,
        front_per_label: 8,
        lambda: 10.0,
        k: 2048,
        out: "data/trained_specialists".into(),
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
            "--suite" => a.suite = val(&mut i, "--suite"),
            "--holdout" => a.holdout = val(&mut i, "--holdout").parse().expect("number"),
            "--front-per-label" => {
                a.front_per_label = val(&mut i, "--front-per-label").parse().expect("number")
            }
            "--lambda" => a.lambda = val(&mut i, "--lambda").parse().expect("number"),
            "--k" => a.k = val(&mut i, "--k").parse().expect("number"),
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
    let t0 = std::time::Instant::now();
    println!(
        "instinct_emotion_v2: suite {} · datasets {} · λ {} · k {} · holdout {} · front {}/label",
        a.suite, dir.display(), a.lambda, a.k, a.holdout, a.front_per_label
    );
    let train = load_suite_rows(&dir, &a.suite).expect("load train");
    let labels = label_universe(&train);
    println!(
        "train rows {} · labels {} {:?} · {}",
        train.len(),
        labels.len(),
        labels,
        t0.elapsed().as_secs_f64()
    );

    // ── stage 1: the sanctioned baseline validation (test read #1 of 1) ──
    println!(
        "── stage 1: baseline validation — the ONE sanctioned test-split read \
         (suite {}; all selection below stays train-side)",
        a.suite
    );
    let (front, pool) = probe_front_split(&train, &labels, a.front_per_label);
    let pool_rows: Vec<SuiteRow> = pool.iter().map(|&i| train[i].clone()).collect();
    let t_fit = std::time::Instant::now();
    let ridge_probe =
        NbsvmRidge::fit(&pool_rows, &labels, a.lambda, a.k).expect("fit probe ridge");
    println!(
        "probe pool {} (front {}) · ridge fit k={} · {}",
        pool_rows.len(),
        front.len(),
        ridge_probe.feats_len(),
        t_fit.elapsed().as_secs_f64()
    );
    let test = load_suite_split(&dir, &a.suite, "test").expect("load test");
    let mut presence_ids = Vec::new();
    let mut scratch = Vec::new();
    let mut hit = 0usize;
    for row in &test {
        presence_ids_into(row.text.as_bytes(), &mut presence_ids, &mut scratch);
        let class_of = labels.iter().position(|l| *l == row.label).expect("label");
        if ridge_probe.pick(&presence_ids) == class_of {
            hit += 1;
        }
    }
    let val_acc = hit as f64 / test.len().max(1) as f64;
    let anchor = match probe_anchor(&a.suite) {
        Some(anchor) => anchor,
        None => {
            eprintln!(
                "no probe anchor registered for suite {} — extend probe_anchor() first",
                a.suite
            );
            std::process::exit(2);
        }
    };
    let delta = val_acc - anchor;
    println!(
        "VALIDATION: pure-argmax test acc {val_acc:.4} ({hit}/{}) vs probe anchor \
         {anchor} (delta {delta:+.4})",
        test.len()
    );
    if val_acc < VALIDATION_FLOOR.min(anchor - 0.01) {
        eprintln!(
            "VALIDATION FLOOR MISSED ({val_acc:.4}) — the port diverged \
             from the probe math; investigate BEFORE trusting the bar (the 0.29-class bug)"
        );
        std::process::exit(1);
    }

    // ── stage 2: the holdout gate (train-side only) ──────────────────────
    println!("── stage 2: holdout gate — v2 vs the ridge baseline, paired LB95");
    let (holdout_idx, rest_idx) = stratified_holdout(&train, a.holdout);
    println!(
        "holdout {} · train-rest {} (label-stratified round-robin, the 576 law)",
        holdout_idx.len(),
        rest_idx.len()
    );
    let class_of: std::collections::HashMap<&str, usize> = labels
        .iter()
        .enumerate()
        .map(|(i, l)| (l.as_str(), i))
        .collect();

    // Baseline: the same ridge posture, fit on the train-rest only.
    let rest_rows: Vec<SuiteRow> = rest_idx.iter().map(|&i| train[i].clone()).collect();
    let t_fit = std::time::Instant::now();
    let ridge = NbsvmRidge::fit(&rest_rows, &labels, a.lambda, a.k).expect("fit ridge");
    let mut baseline_correct = Vec::with_capacity(holdout_idx.len());
    let mut base_hit = 0usize;
    for &i in &holdout_idx {
        presence_ids_into(train[i].text.as_bytes(), &mut presence_ids, &mut scratch);
        let ok = !presence_ids.is_empty()
            && ridge.pick(&presence_ids) == class_of[train[i].label.as_str()];
        baseline_correct.push(ok);
        base_hit += usize::from(ok);
    }
    println!(
        "baseline ridge: holdout acc {:.4} · {}",
        base_hit as f64 / holdout_idx.len().max(1) as f64,
        t_fit.elapsed().as_secs_f64()
    );

    // v1 context: the 576 Arm A over the count bag.
    let t_run = std::time::Instant::now();
    let v1 = train_arm_a(
        &train,
        &labels,
        &rest_idx,
        &holdout_idx,
        &ArmAConfig::default(),
    )
    .expect("train v1");
    let mut v1_correct = Vec::with_capacity(holdout_idx.len());
    let mut bag = Vec::new();
    let mut bag_scratch = Vec::new();
    for &i in &holdout_idx {
        bag_into(train[i].text.as_bytes(), &mut bag, &mut bag_scratch);
        let ok = !bag.is_empty() && v1.model.pick(&bag) == class_of[train[i].label.as_str()];
        v1_correct.push(ok);
    }
    println!(
        "v1 arm A:      holdout acc {:.4} (control {:.4}) · {}",
        v1.holdout_accuracy,
        v1.control_accuracy,
        t_run.elapsed().as_secs_f64()
    );

    // The v2 sweep — stage A probes the saturation/step axes at a fixed
    // schedule (input_scale × lr), stage B refines the schedule around
    // stage A's winner (epochs × weight_decay). All train-side selection.
    let mut tried: Vec<(String, NbsvmConfig)> = Vec::new();
    for &scale in &[1.0f32, 0.5, 0.25, 0.125] {
        for &lr in &[0.05f32, 0.2] {
            tried.push((
                format!("v2 scale={scale} lr={lr} norm k=all e=6 wd=0.01"),
                NbsvmConfig {
                    epochs: 6,
                    lr,
                    presence_norm: true,
                    top_k: None,
                    input_scale: scale,
                    ..NbsvmConfig::default()
                },
            ));
        }
    }
    // Stage B is seeded after stage A is scored (below): epochs × wd
    // around the winner's (scale, lr).
    let mut score = |cfg: &NbsvmConfig| -> (Vec<bool>, f64) {
        let run = train_nbsvm(&train, &labels, &rest_idx, &holdout_idx, cfg)
            .unwrap_or_else(|e| panic!("train: {e}"));
        let mut correct = Vec::with_capacity(holdout_idx.len());
        for &i in &holdout_idx {
            presence_bag_into(
                train[i].text.as_bytes(),
                cfg.presence_norm,
                &mut bag,
                &mut bag_scratch,
            );
            let ok = !bag.is_empty()
                && run.model.pick(&bag) == class_of[train[i].label.as_str()];
            correct.push(ok);
        }
        (correct, run.holdout_accuracy)
    };
    let mut best: Option<(String, NbsvmConfig, Vec<bool>, f64)> = None;
    for (name, cfg) in &tried {
        let t_run = std::time::Instant::now();
        let (correct, acc) = score(cfg);
        println!(
            "  {name}: holdout acc {acc:.4} · {}",
            t_run.elapsed().as_secs_f64()
        );
        let better = best.as_ref().is_none_or(|(_, _, _, a)| acc > *a);
        if better {
            best = Some((name.clone(), *cfg, correct, acc));
        }
    }
    // Stage B: the schedule grid around stage A's winner.
    let (sa_lr, sa_scale) = {
        let (_, c, _, _) = best.as_ref().expect("stage A produced a run");
        (c.lr, c.input_scale)
    };
    println!(
        "  stage A winner: {} (scale {} lr {})",
        best.as_ref().expect("stage A").0,
        sa_scale,
        sa_lr
    );
    for &epochs in &[3usize, 6, 12, 24, 48] {
        for &wd in &[0.01f32, 0.1, 0.5] {
            let cfg = NbsvmConfig {
                epochs,
                lr: sa_lr,
                presence_norm: true,
                top_k: None,
                input_scale: sa_scale,
                weight_decay: wd,
                ..NbsvmConfig::default()
            };
            let name = format!(
                "v2 scale={sa_scale} lr={sa_lr} norm k=all e={epochs} wd={wd}"
            );
            let t_run = std::time::Instant::now();
            let (correct, acc) = score(&cfg);
            println!(
                "  {name}: holdout acc {acc:.4} · {}",
                t_run.elapsed().as_secs_f64()
            );
            let better = best.as_ref().is_none_or(|(_, _, _, a)| acc > *a);
            if better {
                best = Some((name, cfg, correct, acc));
            }
        }
    }
    let (best_name, best_cfg, best_correct, best_acc) = best.expect("grid produced a run");

    // Determinism: rerun the winner, assert bit-identical picks.
    let rerun = train_nbsvm(&train, &labels, &rest_idx, &holdout_idx, &best_cfg)
        .expect("rerun best");
    let mut rerun_correct = Vec::with_capacity(holdout_idx.len());
    for &i in &holdout_idx {
        presence_bag_into(
            train[i].text.as_bytes(),
            best_cfg.presence_norm,
            &mut bag,
            &mut bag_scratch,
        );
        let ok = !bag.is_empty() && rerun.model.pick(&bag) == class_of[train[i].label.as_str()];
        rerun_correct.push(ok);
    }
    assert_eq!(
        best_correct, rerun_correct,
        "winner rerun must reproduce picks bit-identically"
    );

    // The paired gate. The comparison arm is suite-named (see
    // [`gate_vs_v1`]): emotion gates against the ridge baseline (the
    // reflex row's own core); banking77 against the v1 specialist (the
    // incumbent whose composition was already refused vs the modelless
    // 0.8260 — the ridge is context there, not the bar).
    let n = best_correct.len();
    let gate_correct: &[bool] = if gate_vs_v1(&a.suite) {
        &v1_correct
    } else {
        &baseline_correct
    };
    let gate_acc = if gate_vs_v1(&a.suite) {
        v1.holdout_accuracy
    } else {
        base_hit as f64 / n as f64
    };
    let vs_baseline = paired_lb95(&best_correct, &baseline_correct).expect("paired");
    let vs_v1 = paired_lb95(&best_correct, &v1_correct).expect("paired");
    let vs_gate = paired_lb95(&best_correct, gate_correct).expect("paired");
    println!(
        "GATE {best_name}: acc {best_acc:.4} vs {} {:.4} · paired mean {:+.5} \
         (se {:.5}) · LB95 {:+.5} {} 0",
        if gate_vs_v1(&a.suite) { "v1 arm A" } else { "baseline ridge" },
        gate_acc,
        vs_gate.mean,
        vs_gate.se,
        vs_gate.lb95,
        if vs_gate.lb95 > 0.0 { ">" } else { "<=" }
    );
    println!(
        "  context — vs ridge baseline: acc {:.4} · mean {:+.5} · LB95 {:+.5} | vs v1: \
         acc {:.4} · mean {:+.5} · LB95 {:+.5}",
        base_hit as f64 / n as f64,
        vs_baseline.mean,
        vs_baseline.lb95,
        v1.holdout_accuracy,
        vs_v1.mean,
        vs_v1.lb95
    );
    if vs_gate.lb95 > 0.0 {
        // The T3 export (the 576 shape): encode → verify → write. The
        // artifact is the plain Specialist format; the SERVING INPUT
        // convention (L2-normalized presence bags) is the hand-off spec
        // the instinct bridge must implement — printed here and recorded
        // in the bench. Deliberately NOT a `winner_v1` name: winner names
        // are coupled to the arena bridge (the 578 lesson); the rename is
        // the bridge window's act.
        let bytes = encode_artifact(&a.suite, &rerun.model);
        let seal = blake3::hash(&bytes);
        decode_artifact(&bytes).expect("winner artifact must decode");
        std::fs::create_dir_all(&a.out).expect("create out dir");
        let path = std::path::Path::new(&a.out).join(format!("{}_nbsvm_v2.bin", a.suite));
        std::fs::write(&path, &bytes).expect("write artifact");
        println!(
            "EXPORT: {} ({} bytes, blake3 {}) — serving convention: \
             L2-normalized PRESENCE bags (presence_norm, input_scale {}, k=all)",
            path.display(),
            bytes.len(),
            &seal.to_hex()[..16],
            best_cfg.input_scale
        );
        println!(
            "HOLDOUT GATE: PASS — v2 strictly above the {} on the train holdout; \
             the arena's single frozen test read (T3, the instinct bridge) decides \
             registration",
            if gate_vs_v1(&a.suite) { "v1 specialist" } else { "ridge baseline" }
        );
    } else {
        println!(
            "HOLDOUT GATE: NEGATIVE — the lever did not clear the bar on the train holdout; \
             file the negative in issue 579 and stop (never a test-split re-read)"
        );
    }
}

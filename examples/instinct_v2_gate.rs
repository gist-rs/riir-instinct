//! Issues 592-597 (riir-instinct Issue 008 T7) — the v2 holdout-gate lane
//! runner, STAGE-2-ONLY (train-side; no test-split read anywhere — the
//! arena's single frozen read owns the test split). Per suite, on ONE
//! stratified holdout:
//!
//! 1. the v1 Arm A incumbent (the count-bag specialist, the 576 shape);
//! 2. the NbsvmRidge baseline (context; the reflex-row core where the
//!    ridge IS the published lane);
//! 3. the NBSVM v2 sweep (`train_nbsvm`, the 579 lever — stage A
//!    scale × lr, stage B epochs × wd around stage A's winner);
//! 4. the GATE: best v2 vs the suite's GATE arm, paired LB95 > 0, AND the
//!    projected test edge ≥ the pre-registered bar gap
//!    (`bar - served_arm`, the gap the vs-best lane must close);
//! 5. gate PASS → mint `<suite>_nbsvm_v2.bin` (the winner file) + print
//!    the BLAKE3; gate miss → the negative, recorded, exit 1.
//!
//! GATE arm per suite (the shape law from instinct_emotion_v2): the
//! incumbent to beat is whichever shape the suite's CURRENT SERVING ARM is
//! built on — A1/v1 Arm A everywhere in this wave (sst5, ag_news,
//! massive_intent_en, xnli_en all serve specialist/A0 rows built on the
//! count-bag Arm A or the modelless lane; none serves the ridge blend —
//! emotion's ridge@8 suite is NOT in this wave's gate set).
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_v2_gate -- \
//!     [--suites sst5] [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--holdout 600] [--bar-gap 0.017] [--out data/trained_specialists]
//! ```
//!
//! `--bar-gap` pre-registers the projected-edge floor for the FIRST suite
//! named (one suite per invocation when a bar gap matters; a 0.0 gap
//! defaults the gate to plain paired LB95 > 0 — the reachability-negative
//! runs for 592/594/595/596).

use riir_instinct::instinct_nbsvm::{
    paired_lb95, presence_bag_into, train_nbsvm, NbsvmConfig, NbsvmRidge,
};
use riir_instinct::instinct_specialist::{
    bag_into, encode_artifact, label_universe, load_suite_rows, stratified_holdout, train_arm_a,
    ArmAConfig, ArmARun, SuiteRow,
};

/// The per-suite vs-best bar and the currently-served arm accuracy
/// (riir-instinct Issue 008's amended bar table, verified against the
/// reflex bench tables 2026-09-29). `(bar, served)` — the gate's
/// projected-edge floor is `bar - served`.
fn bar_and_served(suite: &str) -> (f64, f64, Option<f64>) {
    match suite {
        // reflex Bench 037 gliner 0.4383 · instinct A1 0.4217 (Bench 004/005)
        "sst5" => (0.4383, 0.4217, None),
        // Issue 593: reflex Bench 087 paw 0.6250 (the frozen-shape bar;
        // gliner 0.6667 was the pre-freeze 24-q shape — not comparable).
        // The suite has no v1 incumbent (a0_stands at 0.3750), so the gap
        // floor vs arm A is 0 and the BINDING leg is the pre-registered
        // ABSOLUTE holdout floor: v2 holdout ≥ 0.72 — the vs-best bar
        // 0.625 with margin for the n=32 test-sampling noise (the eval is
        // 32 questions; 0.72 − 1 sd ≈ 0.62).
        "code_fixtures" => (0.6250, 0.6250, Some(0.72)),
        // reflex Bench 037 laya-riir english 0.9500 · instinct H2 0.8975
        "ag_news" => (0.9500, 0.8975, None),
        // reflex Bench 084 openthai 0.9200 · instinct H2 0.8267 (T2-certified)
        "massive_intent_en" => (0.9200, 0.8267, None),
        // reflex Bench 084 openthai 0.9000 · instinct seat A0 0.5233
        "xnli_en" => (0.9000, 0.5233, None),
        other => panic!("no bar registered for suite {other} — extend bar_and_served()"),
    }
}

struct Args {
    suites: Vec<String>,
    datasets: String,
    holdout: usize,
    bar_gap: Option<f64>,
    /// Override the suite table's absolute holdout floor for the FIRST
    /// suite (`none` clears it). The T8 tie-break protocol (instinct 008)
    /// uses `--bar-gap 0.0 --abs-floor 0.45`: the binding train-side floor
    /// is the candidate-vs-reflex-A0 margin, not the vs-best bar.
    abs_floor: Option<Option<f64>>,
    /// Tie-break mint: on gate pass, mint the argmax of {v1, v2} holdout
    /// accuracy under its natural name — v1 → `<suite>_winner_v1.bin` (the
    /// unbridged default, Count convention), v2 → `<suite>_nbsvm_v2.bin`
    /// (Presence — an UNBRIDGED suite taking the v2 file owes the
    /// winner_bridge entry in the same change, the 579 law). Without the
    /// flag the wave-1 behavior stands (v2-only mint on v2-vs-v1 PASS).
    tie_break: bool,
    out: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        suites: vec!["sst5".into()],
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        holdout: 600,
        bar_gap: None,
        abs_floor: None,
        tie_break: false,
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
            "--suites" => a.suites = val(&mut i, "--suites").split(',').map(str::to_string).collect(),
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--holdout" => a.holdout = val(&mut i, "--holdout").parse().expect("number"),
            "--bar-gap" => a.bar_gap = Some(val(&mut i, "--bar-gap").parse().expect("number")),
            "--abs-floor" => {
                let v = val(&mut i, "--abs-floor");
                a.abs_floor = Some(if v == "none" {
                    None
                } else {
                    Some(v.parse().expect("--abs-floor needs a number or 'none'"))
                });
            }
            "--tie-break" => a.tie_break = true,
            "--out" => a.out = val(&mut i, "--out"),
            other => panic!("unknown flag {other}"),
        }
        i += 1;
    }
    a
}

/// The noul head's labels (the producer's unified pair) — the code_fixtures
/// head split. Every other label belongs to the module head.
fn is_noul_label(l: &str) -> bool {
    l == "no" || l == "yes"
}

/// Per-head resolution: argmax among the row's OWN question's classes (the
/// same resolution the arena's per-question option filtering performs at
/// serve time — a merged union model must never be scored flat).
fn per_head_pick_ok(
    model: &riir_instinct::instinct_specialist::Specialist,
    bag: &[(u32, f32)],
    label: &str,
    labels: &[String],
    class_of: &std::collections::HashMap<&str, usize>,
    scores: &mut [f32],
) -> bool {
    model.scores_into(bag, scores);
    let noul = is_noul_label(label);
    let mut best: Option<usize> = None;
    for (ci, l) in labels.iter().enumerate() {
        if is_noul_label(l) == noul && best.is_none_or(|b| scores[ci] > scores[b]) {
            best = Some(ci);
        }
    }
    best == Some(class_of[label])
}

/// Train Arm A per head (module rows and noul rows separately — the flat
/// union's gradients cancel, the probe-measured collapse) and merge the
/// class rows into one union artifact. Holdout accuracy is scored PER
/// HEAD (each row among its own question's classes — the same resolution
/// the arena's per-question option filtering performs at serve time).
fn merge_per_head(
    train: &[SuiteRow],
    labels: &[String],
    rest_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &ArmAConfig,
) -> Result<ArmARun, String> {
    use riir_instinct::instinct_specialist::Specialist;
    let class_of: std::collections::HashMap<&str, usize> =
        labels.iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
    let mut merged = Specialist::zero(labels);
    let mut steps = 0usize;
    let mut final_loss = 0.0f32;
    let mut skipped = 0usize;
    for noul in [false, true] {
        let head_labels: Vec<String> =
            labels.iter().filter(|l| is_noul_label(l) == noul).cloned().collect();
        let rest: Vec<usize> = rest_idx
            .iter()
            .copied()
            .filter(|&i| is_noul_label(train[i].label.as_str()) == noul)
            .collect();
        let hold: Vec<usize> = holdout_idx
            .iter()
            .copied()
            .filter(|&i| is_noul_label(train[i].label.as_str()) == noul)
            .collect();
        let run = train_arm_a(train, &head_labels, &rest, &hold, cfg)?;
        for (ci, l) in head_labels.iter().enumerate() {
            let dst = class_of[l.as_str()];
            merged.w[dst * riir_instinct::instinct_specialist::VOCAB..(dst + 1) * riir_instinct::instinct_specialist::VOCAB]
                .copy_from_slice(&run.model.w[ci * riir_instinct::instinct_specialist::VOCAB..(ci + 1) * riir_instinct::instinct_specialist::VOCAB]);
            merged.b[dst] = run.model.b[ci];
        }
        steps += run.steps;
        final_loss += run.final_loss;
        skipped += run.skipped_empty;
    }
    // Per-head holdout score: each row among its own head's classes.
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    let mut hit = 0usize;
    let mut scores = vec![0.0f32; labels.len()];
    for &i in holdout_idx {
        bag_into(train[i].text.as_bytes(), &mut bag, &mut scratch);
        if bag.is_empty() {
            continue;
        }
        merged.scores_into(&bag, &mut scores);
        let noul = is_noul_label(train[i].label.as_str());
        let mut best: Option<usize> = None;
        for (ci, l) in labels.iter().enumerate() {
            if is_noul_label(l) == noul
                && best.is_none_or(|b| scores[ci] > scores[b])
            {
                best = Some(ci);
            }
        }
        hit += usize::from(best == Some(class_of[train[i].label.as_str()]));
    }
    let acc = hit as f64 / holdout_idx.len().max(1) as f64;
    // The control: the largest per-head class share of the holdout (the
    // best a constant per-head predictor reads).
    let mut by_class: std::collections::HashMap<&str, usize> = Default::default();
    for &i in holdout_idx {
        *by_class.entry(train[i].label.as_str()).or_default() += 1;
    }
    let control = by_class.values().copied().max().unwrap_or(0) as f64
        / holdout_idx.len().max(1) as f64;
    Ok(ArmARun {
        model: merged,
        steps,
        final_loss: final_loss / 2.0,
        holdout_accuracy: acc,
        control_accuracy: control,
        skipped_empty: skipped,
    })
}

/// The v2 (NBSVM presence) counterpart of [`merge_per_head`]: per-head
/// `train_nbsvm` runs, merged the same way; scored per head on the holdout.
fn per_head_nbsvm(
    train: &[SuiteRow],
    labels: &[String],
    rest_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &NbsvmConfig,
) -> Result<(riir_instinct::instinct_specialist::Specialist, f64), String> {
    use riir_instinct::instinct_specialist::Specialist;
    let class_of: std::collections::HashMap<&str, usize> =
        labels.iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
    let mut merged = Specialist::zero(labels);
    let mut acc_num = 0usize;
    let mut acc_den = 0usize;
    for noul in [false, true] {
        let head_labels: Vec<String> =
            labels.iter().filter(|l| is_noul_label(l) == noul).cloned().collect();
        let rest: Vec<usize> = rest_idx
            .iter()
            .copied()
            .filter(|&i| is_noul_label(train[i].label.as_str()) == noul)
            .collect();
        let hold: Vec<usize> = holdout_idx
            .iter()
            .copied()
            .filter(|&i| is_noul_label(train[i].label.as_str()) == noul)
            .collect();
        let run = train_nbsvm(train, &head_labels, &rest, &hold, cfg)?;
        for (ci, l) in head_labels.iter().enumerate() {
            let dst = class_of[l.as_str()];
            merged.w[dst * riir_instinct::instinct_specialist::VOCAB..(dst + 1) * riir_instinct::instinct_specialist::VOCAB]
                .copy_from_slice(&run.model.w[ci * riir_instinct::instinct_specialist::VOCAB..(ci + 1) * riir_instinct::instinct_specialist::VOCAB]);
            merged.b[dst] = run.model.b[ci];
        }
        // Per-head holdout score with the run's own convention.
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        let mut scores = vec![0.0f32; head_labels.len()];
        let head_class: std::collections::HashMap<&str, usize> =
            head_labels.iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
        for &i in &hold {
            presence_bag_into(train[i].text.as_bytes(), cfg.presence_norm, &mut bag, &mut scratch);
            if bag.is_empty() {
                continue;
            }
            run.model.scores_into(&bag, &mut scores);
            let mut best = 0usize;
            for ci in 1..head_labels.len() {
                if scores[ci] > scores[best] {
                    best = ci;
                }
            }
            acc_num += usize::from(best == head_class[train[i].label.as_str()]);
            acc_den += 1;
        }
    }
    Ok((merged, acc_num as f64 / acc_den.max(1) as f64))
}

fn main() {
    let a = parse_args();
    let dir = std::path::PathBuf::from(&a.datasets);
    let mut failures = 0usize;
    for (si, suite) in a.suites.iter().enumerate() {
        println!(
            "══ instinct_v2_gate: suite {suite} · datasets {} · holdout {} ══",
            dir.display(),
            a.holdout
        );
        // Pre-registered before any run: the projected-edge floor. The
        // FIRST --suite carries --bar-gap (an explicit override); every
        // suite defaults to the table gap (bar - served) so the floor is
        // registered, never improvised.
        let (bar, served, mut abs_floor) = bar_and_served(suite);
        if si == 0 {
            if let Some(ov) = a.abs_floor {
                abs_floor = ov;
            }
        } else if a.abs_floor.is_some() {
            panic!("--abs-floor applies to the FIRST suite only (one pre-registration per invocation)");
        }
        let gap = if si == 0 {
            a.bar_gap.unwrap_or(bar - served)
        } else {
            bar - served
        };
        println!(
            "  bar {bar:.4} · served {served:.4} · pre-registered projected-edge floor \
             ≥ {gap:+.4}"
        );

        let train = match load_suite_rows(&dir, suite) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  LOAD FAILED — {e}");
                failures += 1;
                continue;
            }
        };
        let labels = label_universe(&train);
        let class_of: std::collections::HashMap<&str, usize> =
            labels.iter().enumerate().map(|(i, l)| (l.as_str(), i)).collect();
        println!("  train rows {} · labels {} {:?}", train.len(), labels.len(), labels);
        let (holdout_idx, rest_idx) = stratified_holdout(&train, a.holdout);
        println!(
            "  holdout {} · train-rest {} (label-stratified round-robin, the 576 law)",
            holdout_idx.len(),
            rest_idx.len()
        );

        // ── the v1 Arm A incumbent (the gate arm) ────────────────────────
        // Issue 593 finding (the instinct_code_probe, 2026-09-29): the
        // FLAT-UNION training collapses on code_fixtures (holdout 0.1071 =
        // constant, while module-only reaches 0.5714) — every span is
        // exported TWICE (module row + is_pub row), so each head's
        // positives are another head's near-duplicate negatives and the
        // one-vs-all gradients cancel. The honest per-head shape: train
        // each question's rows SEPARATELY and merge the class rows into
        // the union artifact — mathematically equivalent at serve time
        // (the seat resolves per question and never mixes heads). The
        // gate arm below is the per-head arm A merged, so v2-vs-v1
        // compares shapes, not a collapse artifact.
        let per_head = suite == "code_fixtures";
        let v1 = if per_head {
            merge_per_head(
                &train,
                &labels,
                &rest_idx,
                &holdout_idx,
                &ArmAConfig { epochs: 12, ..ArmAConfig::default() },
            )
            .unwrap_or_else(|e| panic!("{suite}: per-head arm A: {e}"))
        } else {
            train_arm_a(&train, &labels, &rest_idx, &holdout_idx, &ArmAConfig::default())
                .unwrap_or_else(|e| panic!("{suite}: train v1: {e}"))
        };
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        let mut v1_correct = Vec::with_capacity(holdout_idx.len());
        let mut v1_scores = vec![0.0f32; labels.len()];
        for &i in &holdout_idx {
            bag_into(train[i].text.as_bytes(), &mut bag, &mut scratch);
            let ok = !bag.is_empty()
                && if per_head {
                    per_head_pick_ok(&v1.model, &bag, train[i].label.as_str(), &labels, &class_of, &mut v1_scores)
                } else {
                    v1.model.pick(&bag) == class_of[train[i].label.as_str()]
                };
            v1_correct.push(ok);
        }
        println!(
            "  v1 arm A: holdout acc {:.4} (lr0 control {:.4})",
            v1.holdout_accuracy, v1.control_accuracy
        );

        // ── the ridge baseline (context only) ────────────────────────────
        let rest_rows: Vec<SuiteRow> = rest_idx.iter().map(|&i| train[i].clone()).collect();
        let ridge = NbsvmRidge::fit(&rest_rows, &labels, 10.0, 2048)
            .unwrap_or_else(|e| panic!("{suite}: fit ridge: {e}"));
        let mut presence_ids = Vec::new();
        let mut ridge_hit = 0usize;
        for &i in &holdout_idx {
            riir_instinct::instinct_nbsvm::presence_ids_into(
                train[i].text.as_bytes(),
                &mut presence_ids,
                &mut scratch,
            );
            ridge_hit +=
                usize::from(!presence_ids.is_empty() && ridge.pick(&presence_ids) == class_of[train[i].label.as_str()]);
        }
        println!(
            "  ridge (k=2048 λ=10): holdout acc {:.4}",
            ridge_hit as f64 / holdout_idx.len().max(1) as f64
        );

        // ── the v2 sweep (stage A scale × lr, stage B epochs × wd) ───────
        let mut score = |cfg: &NbsvmConfig| -> (Vec<bool>, f64) {
            let (model, holdout_accuracy) = if per_head {
                per_head_nbsvm(&train, &labels, &rest_idx, &holdout_idx, cfg)
                    .unwrap_or_else(|e| panic!("{suite}: per-head v2: {e}"))
            } else {
                let run = train_nbsvm(&train, &labels, &rest_idx, &holdout_idx, cfg)
                    .unwrap_or_else(|e| panic!("{suite}: train v2: {e}"));
                (run.model, run.holdout_accuracy)
            };
            let mut correct = Vec::with_capacity(holdout_idx.len());
            let mut v2_scores = vec![0.0f32; labels.len()];
            for &i in &holdout_idx {
                presence_bag_into(
                    train[i].text.as_bytes(),
                    cfg.presence_norm,
                    &mut bag,
                    &mut scratch,
                );
                let ok = !bag.is_empty()
                    && if per_head {
                        per_head_pick_ok(&model, &bag, train[i].label.as_str(), &labels, &class_of, &mut v2_scores)
                    } else {
                        model.pick(&bag) == class_of[train[i].label.as_str()]
                    };
                correct.push(ok);
            }
            (correct, holdout_accuracy)
        };
        let mut best: Option<(String, NbsvmConfig, Vec<bool>, f64)> = None;
        for &scale in &[1.0f32, 0.5, 0.25, 0.125] {
            for &lr in &[0.05f32, 0.2] {
                let cfg = NbsvmConfig {
                    epochs: 6,
                    lr,
                    presence_norm: true,
                    top_k: None,
                    input_scale: scale,
                    ..NbsvmConfig::default()
                };
                let name = format!("v2 scale={scale} lr={lr} norm k=all e=6 wd=0.01");
                let (correct, acc) = score(&cfg);
                println!("    {name}: holdout acc {acc:.4}");
                if best.as_ref().is_none_or(|(_, _, _, a)| acc > *a) {
                    best = Some((name, cfg, correct, acc));
                }
            }
        }
        let (sa_lr, sa_scale) = {
            let (_, c, _, _) = best.as_ref().expect("stage A produced a run");
            (c.lr, c.input_scale)
        };
        println!("  stage A winner: scale {sa_scale} lr {sa_lr}");
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
                let name = format!("v2 scale={sa_scale} lr={sa_lr} norm k=all e={epochs} wd={wd}");
                let (correct, acc) = score(&cfg);
                println!("    {name}: holdout acc {acc:.4}");
                if best.as_ref().is_none_or(|(_, _, _, a)| acc > *a) {
                    best = Some((name, cfg, correct, acc));
                }
            }
        }
        let (best_name, best_cfg, v2_correct, v2_acc) = best.expect("grid produced a run");
        println!("  v2 best: {best_name} · holdout acc {v2_acc:.4}");

        // Determinism: rerun the winner, assert bit-identical picks.
        let (rerun_model, _rerun_acc) = if per_head {
            per_head_nbsvm(&train, &labels, &rest_idx, &holdout_idx, &best_cfg)
                .unwrap_or_else(|e| panic!("{suite}: rerun: {e}"))
        } else {
            let run = train_nbsvm(&train, &labels, &rest_idx, &holdout_idx, &best_cfg)
                .unwrap_or_else(|e| panic!("{suite}: rerun: {e}"));
            (run.model, run.holdout_accuracy)
        };
        let mut rerun_correct = Vec::with_capacity(holdout_idx.len());
        let mut rerun_scores = vec![0.0f32; labels.len()];
        for &i in &holdout_idx {
            presence_bag_into(
                train[i].text.as_bytes(),
                best_cfg.presence_norm,
                &mut bag,
                &mut scratch,
            );
            let ok = !bag.is_empty()
                && if per_head {
                    per_head_pick_ok(&rerun_model, &bag, train[i].label.as_str(), &labels, &class_of, &mut rerun_scores)
                } else {
                    rerun_model.pick(&bag) == class_of[train[i].label.as_str()]
                };
            rerun_correct.push(ok);
        }
        if rerun_correct != v2_correct {
            eprintln!("  {suite}: DETERMINISM FAIL — rerun picks differ");
            failures += 1;
            continue;
        }
        println!("  determinism: rerun byte-identical ✓");

        // ── the GATE ───────────────────────────────────────────────────
        let lb = paired_lb95(&v2_correct, &v1_correct).expect("paired diff");
        let edge = v2_acc - v1.holdout_accuracy;
        println!(
            "  GATE: v2-vs-v1 mean {:+.4} · paired LB95 {:+.4} · floor (LB95 > 0): {}",
            lb.mean,
            lb.lb95,
            if lb.lb95 > 0.0 { "PASS" } else { "MISS" }
        );
        println!(
            "  GATE: projected test edge {:+.4} vs pre-registered floor {:+.4}: {}",
            edge,
            gap,
            if edge >= gap { "PASS" } else { "MISS" }
        );
        if let Some(floor) = abs_floor {
            println!(
                "  GATE: absolute holdout {v2_acc:.4} vs pre-registered floor {floor}: {}",
                if v2_acc >= floor { "PASS" } else { "MISS" }
            );
        }
        let absolute_ok = abs_floor.is_none_or(|f| v2_acc >= f);
        // Tie-break mode (instinct 008 T8): the question is "does ANY
        // specialist beat reflex A0 on the frozen read", not "does v2 beat
        // v1" — the candidate is the argmax of the two shapes, the
        // v2-vs-v1 LB95 leg is informational, and the mint names the
        // winning shape's natural file.
        let (pass, mint_v1) = if a.tie_break {
            let cand_acc = v1.holdout_accuracy.max(v2_acc);
            let cand_is_v1 = v1.holdout_accuracy >= v2_acc;
            println!(
                "  TIE-BREAK: candidate = {} holdout {cand_acc:.4} (v1 {:+.4} vs v2)",
                if cand_is_v1 { "v1 arm-A per-head" } else { "v2 nbsvm per-head" },
                v1.holdout_accuracy - v2_acc
            );
            let cand_ok = abs_floor.is_none_or(|f| cand_acc >= f);
            println!(
                "  TIE-BREAK GATE: candidate absolute {cand_acc:.4} vs floor {}: {}",
                abs_floor.map(|f| f.to_string()).unwrap_or_else(|| "none".into()),
                if cand_ok { "PASS" } else { "MISS" }
            );
            (cand_ok, cand_is_v1)
        } else {
            (lb.lb95 > 0.0 && edge >= gap && absolute_ok, false)
        };
        if pass {
            let (model, file_name) = if mint_v1 {
                // Determinism: the shape law is structural (no RNG in the
                // count-bag trainer) — rerun and assert identical picks.
                let rerun = merge_per_head(
                    &train,
                    &labels,
                    &rest_idx,
                    &holdout_idx,
                    &ArmAConfig { epochs: 12, ..ArmAConfig::default() },
                )
                .unwrap_or_else(|e| panic!("{suite}: per-head arm A rerun: {e}"));
                let mut r_correct = Vec::with_capacity(holdout_idx.len());
                let mut r_scores = vec![0.0f32; labels.len()];
                for &i in &holdout_idx {
                    bag_into(train[i].text.as_bytes(), &mut bag, &mut scratch);
                    let ok = !bag.is_empty()
                        && per_head_pick_ok(
                            &rerun.model,
                            &bag,
                            train[i].label.as_str(),
                            &labels,
                            &class_of,
                            &mut r_scores,
                        );
                    r_correct.push(ok);
                }
                // v1_correct was scored on the same holdout with the same
                // resolution — equality is the determinism pin.
                if r_correct != v1_correct {
                    eprintln!("  {suite}: DETERMINISM FAIL — v1 rerun picks differ");
                    failures += 1;
                    continue;
                }
                println!("  determinism: v1 rerun byte-identical \u{2713}");
                (rerun.model, format!("{suite}_winner_v1.bin"))
            } else {
                (rerun_model.clone(), format!("{suite}_nbsvm_v2.bin"))
            };
            let bytes = encode_artifact(suite, &model);
            let seal = blake3::hash(&bytes);
            if let Err(e) = riir_instinct::instinct_specialist::decode_artifact(&bytes) {
                eprintln!("  {suite}: ARTIFACT SELF-CHECK FAILED — {e}");
                failures += 1;
                continue;
            }
            std::fs::create_dir_all(&a.out).expect("create out dir");
            let path = std::path::Path::new(&a.out).join(&file_name);
            std::fs::write(&path, &bytes).expect("write artifact");
            println!(
                "  GATE PASS → minted {} ({} bytes, blake3 {})",
                path.display(),
                bytes.len(),
                &seal.to_hex()[..16]
            );
            if mint_v1 {
                // The consumer reads {suite}_winner_v1.bin under the Count
                // convention — the default unbridged shape, no bridge edit.
            } else if !matches!(suite.as_str(), "banking77" | "typed_decisions") {
                eprintln!(
                    "  ⚠ {suite} is UNBRIDGED: the v2 file trains under PRESENCE bags — \n                     add the winner_bridge entry (file + Presence convention) in the SAME \n                     change as this mint, or the consumer scores it with Count bags (the 579 coupling)"
                );
            }
        } else {
            println!(
                "  GATE MISS — the honest-outcome rule applies: the negative is recorded, \n                 no test read, no mint (issue 579's law)"
            );
            failures += 1;
        }
    }
    if failures > 0 {
        eprintln!("instinct_v2_gate: {failures} gate miss(es)/failure(s) — LOUD");
        std::process::exit(1);
    }
    println!("instinct_v2_gate: ALL GATES PASSED — {} suite(s)", a.suites.len());
}

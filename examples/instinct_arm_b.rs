//! Issue 576 T3+T4 — Arm B (laya-teacher distillation) vs Arm A on the SAME
//! held-out train slice, then the winner exported as a BLAKE3-sealed frozen
//! artifact (`<suite>_winner_v1.bin`). T3's comparison is a TRAIN-SIDE
//! decision (the no-cheat law, riir-reflex Issue 038): the test split is
//! never read here — the arena reads it once, in riir-instinct.
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_arm_b -- \
//!     [--suites ag_news,emotion] [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--teacher-dir ../riir-reflex/.raw/distill_teacher] \
//!     [--epochs 3] [--holdout 200] [--mixes 0.0,0.5] [--out data/trained_specialists]
//! ```
//!
//! Arms per suite: A (gold, the T2 posture), B at each `--mixes` gold mix
//! (0.0 = pure distillation, the protocol posture), plus — when
//! `--brier-lambda` is given — the 623 T5 rider arms: the SAME postures at
//! each λ (Research 464 §2.1's `(1−λ)·BCE + λ·Brier`; isolation law: the
//! rider is compared PAIRED against its own base posture on the shared
//! holdout — pick-Brier LB95 < 0 AND accuracy LB95 ≥ 0, the §3 gate). The
//! winner = the highest holdout accuracy, ties resolved to the EARLIER
//! candidate (A first — the already-validated posture wins a tie). Every
//! arm is reported, losses included. The run FAILS loud if any winner
//! fails to beat its lr=0 control (the Arm A law).
//!
//! `--dedup-audit` prints the 471 item-5 hygiene read per suite
//! (exact-duplicate (text,label) train rows — the shortcut surface at our
//! scale) WITHOUT changing training; `--no-export` skips the T4 artifact
//! mint (rider measurement runs — the T4 winners are already sealed).

use riir_instinct::instinct_specialist::{
    bag_into, decode_artifact, encode_artifact, label_universe, load_suite_rows, load_teacher_dump,
    s1mb_presented_union, stratified_holdout, train_arm_a, train_arm_b, ArmAConfig, ArmBConfig,
    Specialist,
};
use riir_instinct::stats::paired_upper_bound_f64;

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
    teacher_dir: String,
    epochs: usize,
    holdout: usize,
    mixes: Vec<f32>,
    out: String,
    /// Plan 010 T4 (reflex plan 010: "the gold-only path"): train Arm A
    /// ONLY — no teacher dump is loaded and no Arm B mix runs. S1MB has
    /// no teacher (their eval code is the deferred repro scope), so the
    /// gold path is the whole lane. The winner-vs-control gate and the
    /// artifact export are unchanged.
    arm_a_only: bool,
    /// Issue 623 T5 rider (464 §2.1): run the SAME postures at each of
    /// these λ values in `(1−λ)·BCE + λ·Brier`, each compared PAIRED
    /// against its base posture (the §3 isolation gate).
    brier_lambda: Vec<f32>,
    /// 471 item-5 hygiene: report exact-duplicate (text,label) train rows
    /// per suite (report-only — training data unchanged).
    dedup_audit: bool,
    /// Skip the T4 winner mint (rider measurement runs).
    no_export: bool,
}

fn parse_args() -> Args {
    let mut a = Args {
        suites: DEFAULT_SUITES.iter().map(|s| (*s).to_string()).collect(),
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        teacher_dir: "../riir-reflex/.raw/distill_teacher".into(),
        epochs: 3,
        holdout: 200,
        mixes: vec![0.0, 0.5],
        out: "data/trained_specialists".into(),
        arm_a_only: false,
        brier_lambda: Vec::new(),
        dedup_audit: false,
        no_export: false,
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
            "--suites" => a.suites = val(&mut i, "--suites").split(',').map(str::to_string).collect(),
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--teacher-dir" => a.teacher_dir = val(&mut i, "--teacher-dir"),
            "--epochs" => a.epochs = val(&mut i, "--epochs").parse().expect("--epochs number"),
            "--holdout" => a.holdout = val(&mut i, "--holdout").parse().expect("--holdout number"),
            "--mixes" => {
                a.mixes = val(&mut i, "--mixes")
                    .split(',')
                    .map(|m| m.trim().parse::<f32>().expect("--mixes floats"))
                    .collect();
            }
            "--out" => a.out = val(&mut i, "--out"),
            "--arm-a-only" => a.arm_a_only = true,
            "--brier-lambda" => {
                a.brier_lambda = val(&mut i, "--brier-lambda")
                    .split(',')
                    .map(|m| m.trim().parse::<f32>().expect("--brier-lambda floats in [0,1]"))
                    .collect();
                if a.brier_lambda.iter().any(|&l| !(0.0..=1.0).contains(&l)) {
                    panic!("--brier-lambda values must lie in [0, 1]");
                }
            }
            "--dedup-audit" => a.dedup_audit = true,
            "--no-export" => a.no_export = true,
            other => panic!("unknown flag {other}"),
        }
        i += 1;
    }
    a
}

struct ArmRow {
    name: String,
    accuracy: f64,
    control: f64,
    model: riir_instinct::instinct_specialist::Specialist,
    /// The 464 §3 readout + per-row pairs for the paired rider gate.
    calibration: riir_instinct::instinct_specialist::PickCalibration,
    pick_conf: Vec<f32>,
    pick_hit: Vec<u8>,
}

impl ArmRow {
    /// The base-posture name a rider arm pairs against: strips the
    /// `,brier=λ` suffix the rider naming law appends.
    fn base_name(&self) -> &str {
        match self.name.rfind(",brier=") {
            Some(i) => &self.name[..i],
            None => &self.name,
        }
    }

    fn is_rider(&self) -> bool {
        self.name.contains(",brier=")
    }
}

fn main() {
    let a = parse_args();
    let datasets = std::path::PathBuf::from(&a.datasets);
    let teacher_dir = std::path::PathBuf::from(&a.teacher_dir);
    println!(
        "instinct_arm_b: datasets {} · teacher {} · suites {:?} · epochs {} · holdout {} · mixes {:?}",
        datasets.display(),
        teacher_dir.display(),
        a.suites,
        a.epochs,
        a.holdout,
        a.mixes
    );
    let mut failures = 0usize;
    for suite in &a.suites {
        let t0 = std::time::Instant::now();
        let rows = match load_suite_rows(&datasets, suite) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  {suite}: LOAD FAILED — {e}");
                failures += 1;
                continue;
            }
        };
        // Plan 010 T4: the s1mb specialist's class space is the PRESENTED
        // union over BOTH halves (the bridge resolves presented keys by
        // name; cal cases come from the train half). Gold-only classes
        // would leave never-gold keys unresolvable and unseat the suite.
        let mut labels = label_universe(&rows);
        if suite.starts_with("s1mb_") {
            let union = match s1mb_presented_union(&datasets, suite) {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("  {suite}: PRESENTED-UNION FAILED — {e}");
                    failures += 1;
                    continue;
                }
            };
            labels.extend(union);
            labels.sort();
            labels.dedup();
        }
        // Arm A only: the teacher dump is never touched.
        let dump = if a.arm_a_only {
            None
        } else {
            let dump_path = teacher_dir.join(format!("{suite}_teacher.bin"));
            match load_teacher_dump(&dump_path) {
                Ok(d) => Some(d),
                Err(e) => {
                    eprintln!("  {suite}: TEACHER LOAD FAILED — {e}");
                    failures += 1;
                    continue;
                }
            }
        };
        let (holdout, train) = stratified_holdout(&rows, a.holdout);
        let cfg = ArmAConfig { epochs: a.epochs, ..ArmAConfig::default() };

        if a.dedup_audit {
            // 471 item-5 (issue 623 T5): exact-duplicate (text,label) train
            // rows — the shortcut surface at our scale. Report-only; the
            // training data is untouched (a fix would enter as its own
            // paired arm, the §3 law).
            let mut seen = std::collections::HashSet::new();
            let mut dups = 0usize;
            for &i in &train {
                if !seen.insert((&rows[i].text, &rows[i].label)) {
                    dups += 1;
                }
            }
            println!(
              "  {suite}: dedup-audit train rows {} · exact-dup (text,label) rows {} · unique {}",
                train.len(),
                dups,
                train.len() - dups
            );
        }

        let mut arms: Vec<ArmRow> = Vec::new();
        let run_a = match train_arm_a(&rows, &labels, &train, &holdout, &cfg) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  {suite}: ARM A TRAIN FAILED — {e}");
                failures += 1;
                continue;
            }
        };
        arms.push(ArmRow {
            name: "A".into(),
            accuracy: run_a.holdout_accuracy,
            control: run_a.control_accuracy,
            model: run_a.model,
            calibration: run_a.calibration,
            pick_conf: run_a.holdout_pick_conf,
            pick_hit: run_a.holdout_pick_hit,
        });
        // The 623 T5 rider arms: Arm A at each λ (464 §2.1).
        for &lam in &a.brier_lambda {
            let rcfg = ArmAConfig { brier_lambda: lam, ..cfg };
            let run = match train_arm_a(&rows, &labels, &train, &holdout, &rcfg) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("  {suite}: ARM A(brier={lam}) TRAIN FAILED — {e}");
                    failures += 1;
                    continue;
                }
            };
            arms.push(ArmRow {
                name: format!("A,brier={lam}"),
                accuracy: run.holdout_accuracy,
                control: run.control_accuracy,
                model: run.model,
                calibration: run.calibration,
                pick_conf: run.holdout_pick_conf,
                pick_hit: run.holdout_pick_hit,
            });
        }
        if let Some(dump) = &dump {
            for (mi, mix) in a.mixes.iter().enumerate() {
            let run_b = match train_arm_b(
                &rows,
                &labels,
                dump,
                &train,
                &holdout,
                &ArmBConfig { base: cfg, gold_mix: *mix },
            ) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("  {suite}: ARM B[{mi}] TRAIN FAILED — {e}");
                    failures += 1;
                    continue;
                }
            };
            arms.push(ArmRow {
                name: format!("B(gold_mix={mix})"),
                accuracy: run_b.holdout_accuracy,
                control: run_b.control_accuracy,
                model: run_b.model,
                calibration: run_b.calibration,
                pick_conf: run_b.holdout_pick_conf,
                pick_hit: run_b.holdout_pick_hit,
            });
            // The 623 T5 rider arms: B(mix) at each λ (464 §2.1).
            for &lam in &a.brier_lambda {
                let rcfg = ArmBConfig { base: ArmAConfig { brier_lambda: lam, ..cfg }, gold_mix: *mix };
                let run = match train_arm_b(&rows, &labels, dump, &train, &holdout, &rcfg) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("  {suite}: ARM B[{mi}](brier={lam}) TRAIN FAILED — {e}");
                        failures += 1;
                        continue;
                    }
                };
                arms.push(ArmRow {
                    name: format!("B(gold_mix={mix}),brier={lam}"),
                    accuracy: run.holdout_accuracy,
                    control: run.control_accuracy,
                    model: run.model,
                    calibration: run.calibration,
                    pick_conf: run.holdout_pick_conf,
                    pick_hit: run.holdout_pick_hit,
                });
            }
            }
        }

        let teacher_acc = match &dump {
            Some(dump) => {
                let hits = dump
                    .gold
                    .iter()
                    .zip(dump.picks.iter())
                    .filter(|(g, p)| g == p)
                    .count();
                format!("{:.4}", hits as f64 / dump.n_rows().max(1) as f64)
            }
            None => "n/a (arm-a-only)".to_string(),
        };
        let control = arms[0].control;
        let winner = arms
            .iter()
            .enumerate()
            .reduce(|best, cand| if cand.1.accuracy > best.1.accuracy { cand } else { best })
            .map(|(_, r)| r)
            .expect("at least one arm");
        // Plan 010 T4: the presented-constrained holdout read — the
        // specialist picks among the case's PRESENTED options (the arena's
        // A1/H1/H2 pick space), not the whole class union. The union-argmax
        // number above drives the winner-vs-control gate; this is the
        // honest headline for the variable-option suites.
        let presented_holdout = presented_constrained_holdout(&rows, &holdout, &labels, &winner.model);
        for arm in &arms {
            println!(
                "  {suite}: {:>24} holdout acc {:.4} (control {:.4}) pick-Brier {:.4} · NLL {:.4} · ECE {:.4} [binned {}/{}] {}",
                arm.name,
                arm.accuracy,
                arm.control,
                arm.calibration.brier,
                arm.calibration.nll,
                arm.calibration.ece,
                arm.calibration.binned,
                arm.calibration.n,
                if arm.accuracy > arm.control { "OK" } else { "UNDER CONTROL — LOUD" },
            );
        }
        // The 623 T5 rider verdicts (464 §3): each rider arm vs its OWN
        // base posture, PAIRED over the shared holdout — Δacc LB95 ≥ 0 AND
        // Δ(pick-Brier) LB95 < 0 wins BOTH axes (the promotion bar); a
        // calibration-only win is a recorded NEGATIVE (the bench-013 Platt
        // shape); ECE stays disclosure.
        for (i, rider) in arms.iter().enumerate() {
            if !rider.is_rider() {
                continue;
            }
            let base_name = rider.base_name().to_string();
            let Some(base) = arms.iter().take(i).find(|b| b.name == base_name) else {
                eprintln!("  {suite}: rider {} has no base posture — LOUD", rider.name);
                failures += 1;
                continue;
            };
            let pairs = rider
                .pick_conf
                .iter()
                .zip(base.pick_conf.iter())
                .zip(rider.pick_hit.iter().zip(base.pick_hit.iter()));
            let mut acc_delta: Vec<f64> = Vec::with_capacity(rider.pick_conf.len());
            let mut brier_delta: Vec<f64> = Vec::with_capacity(rider.pick_conf.len());
            for ((&rc, &bc), (&rh, &bh)) in pairs {
                if rc.is_nan() || bc.is_nan() {
                    continue; // skipped rows pair as skipped or not at all
                }
                acc_delta.push(f64::from(i32::from(rh) - i32::from(bh)));
                let rb = (f64::from(rc) - f64::from(rh)).powi(2);
                let bb = (f64::from(bc) - f64::from(bh)).powi(2);
                brier_delta.push(rb - bb);
            }
            let (acc_v, brier_v) = match (
                riir_instinct::stats::paired_upper_bound_f64(&acc_delta),
                paired_upper_bound_f64(&brier_delta),
            ) {
                (Some(x), Some(y)) => (x, y),
                _ => {
                    eprintln!("  {suite}: rider {} paired stats empty — LOUD", rider.name);
                    failures += 1;
                    continue;
                }
            };
            let verdict = if brier_v.lb95 < 0.0 && acc_v.lb95 >= 0.0 {
                "WINS BOTH AXES (promotion candidate)"
            } else if brier_v.lb95 < 0.0 && acc_v.mean < 0.0 {
                "CALIBRATION-ONLY — recorded NEGATIVE (accuracy cost, the bench-013 Platt shape)"
            } else if brier_v.lb95 < 0.0 {
                "CALIBRATION-WON, accuracy LB95 not held — not promotable this run"
            } else {
                "NEGATIVE (no significant pick-Brier gain)"
            };
            println!(
                "  {suite}: RIDER {} vs {} — Δacc {:+.4} (LB95 {:+.4}) · Δpick-Brier {:+.4} (LB95 {:+.4}) → {}",
                rider.name,
                base_name,
                acc_v.mean,
                acc_v.lb95,
                brier_v.mean,
                brier_v.lb95,
                verdict
            );
        }
        println!(
            "  {suite}: teacher(train) acc {teacher_acc} · rows {} · labels {} · presented-constrained holdout {:.4} ({} rows) · WINNER {} — joined in {:.1}s",
            rows.len(),
            labels.len(),
            presented_holdout.0,
            presented_holdout.1,
            winner.name,
            t0.elapsed().as_secs_f64()
        );

        if winner.accuracy <= control {
            eprintln!("  {suite}: WINNER FAILED the lr=0 control — LOUD");
            failures += 1;
            continue;
        }
        if a.no_export {
            println!("  {suite}: --no-export — T4 mint skipped (rider measurement run)");
            continue;
        }
        // T4: the winner ships as a sealed frozen artifact (the
        // rule_embed_frozen law; decode-verified by its producer).
        let bytes = encode_artifact(suite, &winner.model);
        if let Err(e) = decode_artifact(&bytes) {
            eprintln!("  {suite}: ARTIFACT SELF-CHECK FAILED — {e}");
            failures += 1;
            continue;
        }
        std::fs::create_dir_all(&a.out).expect("create out dir");
        let path = std::path::Path::new(&a.out).join(format!("{suite}_winner_v1.bin"));
        std::fs::write(&path, &bytes).expect("write artifact");
        let seal = blake3::hash(&bytes);
        println!(
            "  {suite}: artifact {} ({} bytes, blake3 {}) — decoded OK",
            path.display(),
            bytes.len(),
            &seal.to_hex()[..16]
        );
    }
    if failures > 0 {
        eprintln!("instinct_arm_b: {failures} failure(s) — LOUD");
        std::process::exit(1);
    }
    println!(
        "instinct_arm_b: PASSED — {} suite(s); every winner beat its lr=0 control; {}",
        a.suites.len(),
        if a.no_export {
            "--no-export: rider measurement run, no T4 mint"
        } else {
            "winners exported (T4)"
        }
    );
}

/// Plan 010 T4 — holdout accuracy with the pick RESTRICTED to the case's
/// presented options (the arena's A1/H1/H2 pick space): score every class
/// with [`Specialist::scores_into`], argmax over the presented subset with
/// the same lowest-tie law as [`riir_instinct::instinct_specialist`]’s
/// serving pick. Rows without presented keys (fixed-label suites) return
/// (nan, 0) — the union-argmax number is their honest read.
fn presented_constrained_holdout(
    rows: &[riir_instinct::instinct_specialist::SuiteRow],
    holdout_idx: &[usize],
    labels: &[String],
    model: &Specialist,
) -> (f64, usize) {
    let class_of: std::collections::HashMap<&str, usize> = labels
        .iter()
        .enumerate()
        .map(|(i, l)| (l.as_str(), i))
        .collect();
    let mut scores = vec![0.0f32; labels.len()];
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    let mut hits = 0usize;
    let mut n = 0usize;
    for &i in holdout_idx {
        let Some(keys) = rows[i].presented.as_ref() else {
            continue;
        };
        bag_into(rows[i].text.as_bytes(), &mut bag, &mut scratch);
        if bag.is_empty() {
            continue;
        }
        model.scores_into(&bag, &mut scores);
        let classes: Vec<usize> = keys
            .iter()
            .filter_map(|k| class_of.get(k.as_str()).copied())
            .collect();
        if classes.is_empty() {
            continue;
        }
        let mut best = classes[0];
        for &c in &classes[1..] {
            if scores[c] > scores[best] {
                best = c;
            }
        }
        let gold = class_of[rows[i].label.as_str()];
        if best == gold {
            hits += 1;
        }
        n += 1;
    }
    (hits as f64 / n.max(1) as f64, n)
}

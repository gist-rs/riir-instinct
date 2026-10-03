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
//! (0.0 = pure distillation, the protocol posture). The winner = the
//! highest holdout accuracy, ties resolved to the EARLIER candidate
//! (A first — the already-validated posture wins a tie). Every arm is
//! reported, losses included. The run FAILS loud if any winner fails to
//! beat its lr=0 control (the Arm A law).

use riir_instinct::instinct_specialist::{
    bag_into, decode_artifact, encode_artifact, label_universe, load_suite_rows, load_teacher_dump,
    s1mb_presented_union, stratified_holdout, train_arm_a, train_arm_b, ArmAConfig, ArmBConfig,
    Specialist,
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
        });
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
            });
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
                "  {suite}: {:>16} holdout acc {:.4} (control {:.4}) {}",
                arm.name,
                arm.accuracy,
                arm.control,
                if arm.accuracy > arm.control { "OK" } else { "UNDER CONTROL — LOUD" },
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
        "instinct_arm_b: PASSED — {} suite(s); every winner beat its lr=0 control; \
         winners exported (T4)",
        a.suites.len()
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

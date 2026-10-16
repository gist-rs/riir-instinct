//! riir-train 632 T3 — the Eq-9 typed head trainer: trains the FOUR heads
//! (dual-shuffle distilled choice, hard-label noul, per-qid score,
//! coverage) over the typed train pool + the sealed corpus artifacts,
//! selects epochs on a row-stratified holdout, REFITS the selected
//! schedule on ALL rows (the frozen-read artifact), and writes the
//! artifacts + the sealed manifest.
//!
//! Selection is on holdout metrics ONLY (never the test seat — the
//! around-the-test law); the refit is disclosed in the manifest
//! (`selection` vs `artifact` provenance per head).
//!
//! ```sh
//! cargo run --release -p riir-instinct --example instinct_typed_eq9 -- \
//!     [--datasets ../riir-reflex/.raw/datasets_t20k] \
//!     [--coverage ../riir-train/.raw/t3_coverage/typed_coverage_rows.json] \
//!     [--rho ../riir-train/.raw/t3_coverage/typed_coverage_rho.json] \
//!     [--dual-shuffle ../riir-train/.raw/t3_coverage/typed_dual_shuffle.json] \
//!     [--epochs 2,3,5,8] [--holdout 160] \
//!     [--out data/trained_specialists/typed_eq9]
//! ```

use std::path::PathBuf;

use riir_instinct::instinct_specialist::{
    encode_artifact, label_universe, train_arm_a, ArmAConfig, Specialist,
};
use riir_instinct::instinct_typed_eq9::{
    coverage_holdout_auroc, hard_label_rows, load_choice_eq9, load_coverage_eq9,
    paired_holdout, restricted_eval, row_holdout, score_aux_eval, train_choice_eq9,
    GOLD_MIX_QUALIFIED,
};

struct Args {
    datasets: String,
    coverage: String,
    rho: String,
    dual_shuffle: Option<String>,
    epochs: Vec<usize>,
    holdout: usize,
    out: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        coverage: "../riir-train/.raw/t3_coverage/typed_coverage_rows.json".into(),
        rho: "../riir-train/.raw/t3_coverage/typed_coverage_rho.json".into(),
        // The qualification file is OPT-IN BY ABSENCE-OF-DEFAULT: no flag =
        // the pure-gold dev posture (the manifest DISCLOSES it). The real
        // lane always passes --dual-shuffle explicitly — a stale default
        // path would silently train pure gold while looking armed.
        dual_shuffle: None,
        epochs: vec![2, 3, 5, 8],
        holdout: 160,
        out: "data/trained_specialists/typed_eq9".into(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        let val = |i: &mut usize, name: &str| -> String {
            *i += 1;
            args.get(*i).unwrap_or_else(|| panic!("{name} needs a value")).clone()
        };
        match args[i].as_str() {
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--coverage" => a.coverage = val(&mut i, "--coverage"),
            "--rho" => a.rho = val(&mut i, "--rho"),
            "--dual-shuffle" => a.dual_shuffle = Some(val(&mut i, "--dual-shuffle")),
            "--epochs" => {
                a.epochs = val(&mut i, "--epochs")
                    .split(',')
                    .map(|s| s.trim().parse().expect("--epochs numbers"))
                    .collect();
            }
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

fn digest_of(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// One selection-grid entry: (epochs, holdout metric, auxiliary readout).
struct GridRow {
    epochs: usize,
    metric: f64,
    aux: Option<f64>,
}

fn main() {
    let a = parse_args();
    let datasets = PathBuf::from(&a.datasets);
    let coverage = PathBuf::from(&a.coverage);
    let rho = PathBuf::from(&a.rho);
    let ds = a.dual_shuffle.as_ref().map(PathBuf::from);
    println!(
        "instinct_typed_eq9: datasets {} · coverage {} · dual-shuffle {} · epochs {:?} · holdout {}",
        a.datasets,
        a.coverage,
        ds.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "<none — pure-gold dev posture>".into()),
        a.epochs,
        a.holdout,
    );

    // ── the choice head (Eq-9 distilled) ────────────────────────────────
    std::fs::create_dir_all(&a.out).unwrap_or_else(|e| panic!("create {}: {e}", a.out));
    let choice = load_choice_eq9(&datasets, &coverage, &rho, ds.as_deref())
        .unwrap_or_else(|e| panic!("choice data: {e}"));
    println!(
        "  choice: {} events · universe {} keys · corpus {} (qualified {} = {:.4})",
        choice.rows.len(),
        choice.universe.len(),
        choice.n_corpus_rows,
        choice.n_qualified,
        choice.n_qualified as f64 / choice.n_corpus_rows.max(1) as f64,
    );
    let strata: Vec<String> = choice.meta.iter().map(|m| m.row_id.clone()).collect();
    let (holdout, train) = row_holdout(&strata, a.holdout);
    println!("  choice holdout: {} rows · train: {} rows", holdout.len(), train.len());
    let mut grid: Vec<GridRow> = Vec::new();
    for &ep in &a.epochs {
        let cfg = ArmAConfig { epochs: ep, ..ArmAConfig::default() };
        let run = train_choice_eq9(&choice, &train, &holdout, &cfg)
            .unwrap_or_else(|e| panic!("choice epoch {ep}: {e}"));
        let (acc, unanswered) =
            restricted_eval(&run.model, &choice.rows, &choice.universe, &holdout);
        println!(
            "    epochs {ep}: restricted holdout acc {acc:.4} (unanswered {unanswered}) · lr0-control-not-run-here",
        );
        grid.push(GridRow { epochs: ep, metric: acc, aux: Some(unanswered as f64) });
    }
    let best = grid
        .iter()
        .max_by(|x, y| x.metric.partial_cmp(&y.metric).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty grid");
    let choice_epochs = best.epochs;
    println!("  choice selected: {choice_epochs} epochs (holdout acc {:.4})", best.metric);
    // The lr=0 control: a zero model picks the first presented key.
    let zero = Specialist::zero(&choice.universe);
    let (control_acc, control_unanswered) =
        restricted_eval(&zero, &choice.rows, &choice.universe, &holdout);
    println!("  choice control (zero model): {control_acc:.4} (unanswered {control_unanswered})");
    let cfg = ArmAConfig { epochs: choice_epochs, ..ArmAConfig::default() };
    let all_choice: Vec<usize> = (0..choice.rows.len()).collect();
    let choice_final = train_choice_eq9(&choice, &all_choice, &[], &cfg)
        .unwrap_or_else(|e| panic!("choice refit: {e}"))
        .model;
    let (full_acc, _) = restricted_eval(&choice_final, &choice.rows, &choice.universe, &holdout); // seen rows
    println!("  choice refit(full) seen-row acc {full_acc:.4} (holdout was in refit train — memorization readout)");

    // ── the noul head (hard labels, pure Brier) ─────────────────────────
    let heads = hard_label_rows(&datasets).unwrap_or_else(|e| panic!("hard-label rows: {e}"));
    println!("  noul: {} events", heads.noul_rows.len());
    let noul_labels = label_universe(&heads.noul_rows);
    let (noul_holdout, noul_train) = row_holdout(&heads.noul_strata, a.holdout);
    let mut noul_grid: Vec<GridRow> = Vec::new();
    for &ep in &a.epochs {
        let cfg = ArmAConfig { epochs: ep, brier_lambda: 1.0, ..ArmAConfig::default() };
        let run = train_arm_a(&heads.noul_rows, &noul_labels, &noul_train, &noul_holdout, &cfg)
            .unwrap_or_else(|e| panic!("noul epoch {ep}: {e}"));
        println!("    epochs {ep}: holdout acc {:.4}", run.holdout_accuracy);
        noul_grid.push(GridRow { epochs: ep, metric: run.holdout_accuracy, aux: None });
    }
    let best_noul = noul_grid
        .iter()
        .max_by(|x, y| x.metric.partial_cmp(&y.metric).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty noul grid");
    let noul_epochs = best_noul.epochs;
    println!("  noul selected: {noul_epochs} epochs (holdout acc {:.4})", best_noul.metric);
    let all_noul: Vec<usize> = (0..heads.noul_rows.len()).collect();
    let noul_final = train_arm_a(
        &heads.noul_rows,
        &noul_labels,
        &all_noul,
        &[],
        &ArmAConfig { epochs: noul_epochs, brier_lambda: 1.0, ..ArmAConfig::default() },
    )
    .unwrap_or_else(|e| panic!("noul refit: {e}"))
    .model;

    // ── the score heads (per qid, hard labels) ──────────────────────────
    let mut score_records = serde_json::Map::new();
    for (qid, head) in &heads.score {
        let labels = label_universe(&head.rows);
        let (h, t) = row_holdout(&head.strata, a.holdout);
        let mut g: Vec<GridRow> = Vec::new();
        for &ep in &a.epochs {
            let cfg = ArmAConfig { epochs: ep, ..ArmAConfig::default() };
            let run = train_arm_a(&head.rows, &labels, &t, &h, &cfg)
                .unwrap_or_else(|e| panic!("score {qid} epoch {ep}: {e}"));
            g.push(GridRow { epochs: ep, metric: run.holdout_accuracy, aux: None });
        }
        let best = g
            .iter()
            .max_by(|x, y| x.metric.partial_cmp(&y.metric).unwrap_or(std::cmp::Ordering::Equal))
            .expect("non-empty score grid");
        let ep_sel = best.epochs;
        let all_rows: Vec<usize> = (0..head.rows.len()).collect();
        let final_model = train_arm_a(
            &head.rows,
            &labels,
            &all_rows,
            &[],
            &ArmAConfig { epochs: ep_sel, ..ArmAConfig::default() },
        )
        .unwrap_or_else(|e| panic!("score {qid} refit: {e}"))
        .model;
        let (mae, cdm) = score_aux_eval(&final_model, &head.rows, &labels, &h);
        let art_name = format!("typed_eq9_score_{qid}");
        let art = encode_artifact(&art_name, &final_model);
        let art_digest = digest_of(&art);
        let art_path = out_path(&a.out, &format!("{art_name}.bin"));
        std::fs::write(&art_path, &art)
            .unwrap_or_else(|e| panic!("write score {qid}: {e}"));
        std::fs::write(format!("{}.blake3", art_path.display()), format!("{art_digest}\n"))
            .unwrap_or_else(|e| panic!("write score {qid} sidecar: {e}"));
        println!(
            "  score {qid}: {} events · {} levels · epochs {ep_sel} (holdout acc {:.4}) · aux MAE {mae:.4} CDM {cdm:.4}",
            head.rows.len(),
            labels.len(),
            best.metric,
        );
        score_records.insert(
            qid.clone(),
            serde_json::json!({
                "n_events": head.rows.len(),
                "labels": labels,
                "epochs_selected": ep_sel,
                "holdout_acc_selection": best.metric,
                "aux_expectation_mae": mae,
                "aux_gold_cdm": cdm,
                "artifact": format!("{art_name}.bin"),
                "blake3": art_digest,
            }),
        );
    }

    // ── the coverage head (c_y hard labels, road (c)) ───────────────────
    let cov = load_coverage_eq9(&coverage).unwrap_or_else(|e| panic!("coverage data: {e}"));
    let cov_strata: Vec<String> = cov
        .meta
        .iter()
        .map(|(id, _)| id.split('#').next().unwrap_or(id).to_string())
        .collect();
    let (cov_holdout, cov_train) = paired_holdout(&cov_strata, a.holdout);
    let cov_labels = cov.labels.clone();
    let mut cov_grid: Vec<GridRow> = Vec::new();
    for &ep in &a.epochs {
        let cfg = ArmAConfig { epochs: ep, ..ArmAConfig::default() };
        let run = train_arm_a(&cov.rows, &cov_labels, &cov_train, &cov_holdout, &cfg)
            .unwrap_or_else(|e| panic!("coverage epoch {ep}: {e}"));
        let auc = coverage_holdout_auroc(&run.model, &cov, &cov_holdout)
            .unwrap_or_else(|| panic!("coverage epoch {ep}: holdout AUROC"));
        println!("    epochs {ep}: holdout AUROC {auc:.4}");
        cov_grid.push(GridRow { epochs: ep, metric: auc, aux: None });
    }
    let best_cov = cov_grid
        .iter()
        .max_by(|x, y| x.metric.partial_cmp(&y.metric).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty coverage grid");
    let cov_epochs = best_cov.epochs;
    println!("  coverage selected: {cov_epochs} epochs (holdout AUROC {:.4})", best_cov.metric);
    let all_cov: Vec<usize> = (0..cov.rows.len()).collect();
    let cov_final = train_arm_a(
        &cov.rows,
        &cov_labels,
        &all_cov,
        &[],
        &ArmAConfig { epochs: cov_epochs, ..ArmAConfig::default() },
    )
    .unwrap_or_else(|e| panic!("coverage refit: {e}"))
    .model;
    let cov_auc_full = coverage_holdout_auroc(&cov_final, &cov, &cov_holdout);
    println!("  coverage refit(full) seen-row AUROC {cov_auc_full:?} (memorization readout)");

    // ── artifacts + the sealed manifest ─────────────────────────────────
    let write_artifact = |name: &str, model: &Specialist| -> String {
        let art = encode_artifact(name, model);
        let p = out_path(&a.out, &format!("{name}.bin"));
        std::fs::write(&p, &art).unwrap_or_else(|e| panic!("write {name}: {e}"));
        let d = digest_of(&art);
        std::fs::write(format!("{}.blake3", p.display()), format!("{d}\n"))
            .unwrap_or_else(|e| panic!("write {name} sidecar: {e}"));
        d
    };
    let choice_digest = write_artifact("typed_eq9_choice", &choice_final);
    let noul_digest = write_artifact("typed_eq9_noul", &noul_final);
    let cov_digest = write_artifact("typed_eq9_coverage", &cov_final);

    let manifest = serde_json::json!({
        "kind": "riir_train_632_t3_eq9_typed_heads",
        "version": 1,
        "inputs": {
            "datasets": a.datasets,
            "coverage_rows": a.coverage,
            "coverage_rows_blake3": digest_of(
                &std::fs::read(&coverage).unwrap_or_else(|e| panic!("read coverage: {e}")),
            ),
            "rho": a.rho,
            "dual_shuffle": ds.as_ref().map(|p| p.display().to_string()),
            "posture": if ds.is_some() { "dual-shuffle qualified blend" } else { "pure-gold dev posture (NO qualification file)" },
        },
        "blend": {
            "gold_mix_qualified": GOLD_MIX_QUALIFIED,
            "law": "qualified rows = 0.3*gold + 0.7*teacher conditionals; rejected rows = pure gold; non-corpus choice events = pure gold",
        },
        "choice": {
            "n_events": choice.rows.len(),
            "n_corpus": choice.n_corpus_rows,
            "n_qualified": choice.n_qualified,
            "universe": choice.universe,
            "epochs_selected": choice_epochs,
            "selection": grid.iter().map(|g| serde_json::json!({
                "epochs": g.epochs, "holdout_acc": g.metric, "unanswered": g.aux,
            })).collect::<Vec<_>>(),
            "control_zero_model": {"acc": control_acc, "unanswered": control_unanswered},
            // SEEN-ROW readout: the refit trains on ALL rows, so this
            // holdout accuracy is memorization, not generalization — the
            // honest generalization evidence is `selection`.
            "refit_seen_row_acc": full_acc,
            "artifact": "typed_eq9_choice.bin",
            "blake3": choice_digest,
        },
        "noul": {
            "n_events": heads.noul_rows.len(),
            "labels": noul_labels,
            "loss": "pure Brier (brier_lambda = 1.0, the (p-y)^2 shape); margin hinge skipped, disclosed",
            "epochs_selected": noul_epochs,
            "selection": noul_grid.iter().map(|g| serde_json::json!({
                "epochs": g.epochs, "holdout_acc": g.metric,
            })).collect::<Vec<_>>(),
            "artifact": "typed_eq9_noul.bin",
            "blake3": noul_digest,
        },
        "score": score_records,
        "coverage": {
            "n_rows": cov.rows.len(),
            "labels": cov_labels,
            "loss": "BCE on the c_y gold-presence labels (road (c); rho is analysis-only per T2b)",
            "text_law": "state + instructions + one 'key: description' line per criteria key in presented order",
            "epochs_selected": cov_epochs,
            "selection": cov_grid.iter().map(|g| serde_json::json!({
                "epochs": g.epochs, "holdout_auroc": g.metric,
            })).collect::<Vec<_>>(),
            // SEEN-ROW readout — same disclosure as the choice head's.
            "refit_seen_row_auroc": cov_auc_full,
            "artifact": "typed_eq9_coverage.bin",
            "blake3": cov_digest,
        },
        "provenance": {
            "selection_law": "epochs selected on the row-stratified holdout only; the artifact REFITS the selected schedule on ALL rows",
            "test_seat_never_read": true,
            "generated_by": "examples/instinct_typed_eq9.rs",
        },
    });
    let mp = out_path(&a.out, "typed_eq9_manifest.json");
    let text = serde_json::to_string_pretty(&manifest).expect("manifest json");
    std::fs::write(&mp, &text).unwrap_or_else(|e| panic!("write manifest: {e}"));
    let md = digest_of(text.as_bytes());
    std::fs::write(format!("{}.blake3", mp.display()), format!("{md}\n"))
        .unwrap_or_else(|e| panic!("write manifest sidecar: {e}"));
    println!("manifest: {} (blake3 {md})", mp.display());
    println!("done: choice {choice_epochs}e · noul {noul_epochs}e · coverage {cov_epochs}e");
}

fn out_path(out: &str, name: &str) -> PathBuf {
    PathBuf::from(out).join(name)
}

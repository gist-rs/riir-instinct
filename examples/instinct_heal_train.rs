//! Plan 225 T2 — the heal specialist trainer (the A1 candidate).
//!
//! Trains a 2-class NBSVM presence-bag head over (buggy module, candidate
//! fix) pairs mined from the FROZEN Bench-114 harvest (`cases.jsonl` +
//! `split.json`), selects the abstention thresholds on a case-level
//! holdout, and exports the RISP artifact the heal seat server decodes.
//!
//! Laws this file binds itself to:
//! - TRAIN-side rows only: the four test lints of the frozen split are
//!   filtered out BEFORE any training structure exists (the count is
//!   disclosed; a test-side pair that reaches `rows` is a bug).
//! - The holdout is CASE-level: every pair of one case stays on one side.
//! - Scoring matches T1.5: pick-a-gold = 1, abstain-when-none-gold = 1,
//!   pick-a-non-gold = 0; the strata are reported separately, never
//!   pooled into a single number without both n's beside it.
//! - The bag law is shared with the server: trainer
//!   `presence_bag_into(text, norm=true)` ⇔ server
//!   `specialist::presence_bag_into` — the exported head is decoded
//!   consumer-side and re-scored on one pair as the agreement check.
//! - `presence_norm` is PINNED true (the consumer side only carries that
//!   exact bag shape); the model trains ONCE and only the abstention
//!   grid varies.
//!
//! ```text
//! cargo run --release --example instinct_heal_train -- \
//!   ../riir-refine/.benchmarks/114_issue161_pool_widened_harvest/cases.jsonl \
//!   ../riir-refine/.refine/heal_seat/pool_texts.jsonl \
//!   ../riir-refine/.benchmarks/114_issue161_pool_widened_harvest/split.json \
//!   ../riir-refine/.refine/heal_seat/heal_head_v1.bin
//! ```

use std::collections::BTreeMap;
use std::fs;

use riir_instinct::instinct_nbsvm::{paired_lb95, presence_bag_into, train_nbsvm, NbsvmConfig};
use riir_instinct::instinct_specialist::{encode_artifact, SuiteRow};
use riir_instinct::specialist;

const GOLD_LABEL: &str = "gold";
const NOGOLD_LABEL: &str = "nogold";

/// The pair-composition law — the server MUST compose identically.
///
/// The candidates within one case are near-duplicate rewrites of the
/// module: a bag over the whole pair text is nearly identical across a
/// case's options and carries no ranking signal (measured: below-majority
/// holdout accuracy on the full-pair composition). The bag is therefore
/// taken over the EDIT — the words the candidate adds and removes versus
/// the buggy module — which differs across options exactly where the
/// fixes differ. Tokenized as plain words so the shared `events_into`
/// law (lowercased words + bigrams) sees `add <w…> del <w…>`.
fn pair_text(state: &str, option: &str) -> String {
    let m: Vec<&str> = state.split_whitespace().collect();
    let c: Vec<&str> = option.split_whitespace().collect();
    let mut p = 0usize;
    while p < m.len() && p < c.len() && m[p] == c[p] {
        p += 1;
    }
    let mut s = 0usize;
    while s < m.len() - p && s < c.len() - p && m[m.len() - 1 - s] == c[c.len() - 1 - s] {
        s += 1;
    }
    let added = &c[p..c.len() - s];
    let removed = &m[p..m.len() - s];
    let mut out = String::from("add");
    for w in added {
        out.push(' ');
        out.push_str(w);
    }
    out.push_str("\ndel");
    for w in removed {
        out.push(' ');
        out.push_str(w);
    }
    out
}

/// The verify-in-order control (T4.1) over the frozen gate fields: the
/// first pool candidate that compiles and resolves the target lint; else
/// the row's served fallback (the smallest-diff stand-in the harvest
/// recorded).
fn a0_pick(row: &serde_json::Value) -> Option<usize> {
    let pool = row["pool"].as_array()?;
    for (i, c) in pool.iter().enumerate() {
        if c["compiled"].as_bool().unwrap_or(false)
            && c["target_resolved"].as_bool().unwrap_or(false)
        {
            return Some(i);
        }
    }
    row["served_idx"].as_u64().map(|i| i as usize)
}

fn gold_at(row: &serde_json::Value, i: usize) -> bool {
    row["pool"]
        .as_array()
        .and_then(|p| p.get(i))
        .and_then(|c| c["gold_a_c"].as_bool())
        .unwrap_or(false)
}

struct Case {
    stratum: String,
    texts: Vec<String>,
    gold: Vec<bool>,
    a0_score: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!(
            "usage: instinct_heal_train <cases.jsonl> <pool_texts.jsonl> <split.json> <out-head.bin>"
        );
        std::process::exit(2);
    }
    let (cases_path, pools_path, split_path, out_path) = (&args[1], &args[2], &args[3], &args[4]);

    let cases_bytes = fs::read_to_string(cases_path).expect("cases.jsonl reads");
    let pools_bytes = fs::read_to_string(pools_path).expect("pool_texts.jsonl reads");
    let split_bytes = fs::read(split_path).expect("split.json reads");
    let split: BTreeMap<String, String> =
        serde_json::from_slice(&split_bytes).expect("split.json parses");
    // case_id → (texts, pins_ok) — the POOLTEXTS replay's verified texts.
    let mut pools: BTreeMap<String, (Vec<String>, bool)> = BTreeMap::new();
    for line in pools_bytes.lines() {
        let row: serde_json::Value = serde_json::from_str(line).expect("pool row parses");
        let id = row["case_id"].as_str().unwrap_or_default().to_string();
        let texts: Vec<String> = row["texts"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|t| t.as_str().unwrap_or_default().to_string())
                    .collect()
            })
            .unwrap_or_default();
        let ok = row["blake_ok"].as_bool().unwrap_or(false)
            && row["served_ok"].as_bool().unwrap_or(false);
        pools.insert(id, (texts, ok));
    }

    let mut rows: Vec<SuiteRow> = Vec::new();
    let mut cases: Vec<Case> = Vec::new();
    let (mut n_test_skipped, mut n_no_control, mut n_all_gold, mut n_pin_dropped) =
        (0usize, 0usize, 0usize, 0usize);

    for line in cases_bytes.lines() {
        let row: serde_json::Value = serde_json::from_str(line).expect("case row parses");
        let lint = row["target_lint"].as_str().unwrap_or_default().to_string();
        if lint.is_empty() {
            n_no_control += 1;
            continue;
        }
        if split.get(&lint).map(String::as_str) != Some("train") {
            n_test_skipped += 1;
            continue;
        }
        let module = row["module"].as_str().unwrap_or_default().to_string();
        let pool = row["pool"].as_array().cloned().unwrap_or_default();
        let stratum = row["stratum"].as_str().unwrap_or_default().to_string();
        if stratum == "all_gold" {
            n_all_gold += 1;
        }
        let a0 = a0_pick(&row).map(|i| gold_at(&row, i)).unwrap_or(false);
        let case_id = row["case_id"].as_str().unwrap_or_default().to_string();
        let Some((texts, pins_ok)) = pools.get(&case_id) else {
            n_pin_dropped += 1;
            continue;
        };
        if !pins_ok || texts.len() != pool.len() {
            n_pin_dropped += 1;
            continue;
        }
        // Belt: re-verify every text against the frozen blake3_12 pin.
        for (text, frozen) in texts.iter().zip(&pool) {
            let h = blake3::hash(text.as_bytes()).to_hex();
            assert_eq!(
                &h[..12],
                frozen["blake3_12"].as_str().unwrap_or_default(),
                "pool text does not match the frozen pin for {case_id}"
            );
        }
        let first = rows.len();
        for (pi, text) in texts.iter().enumerate() {
            let gold = pool[pi]["gold_a_c"].as_bool().unwrap_or(false);
            rows.push(SuiteRow {
                label: if gold {
                    GOLD_LABEL.to_string()
                } else {
                    NOGOLD_LABEL.to_string()
                },
                text: pair_text(&module, text),
                presented: None,
            });
        }
        cases.push(Case {
            stratum,
            texts: (first..rows.len()).map(|i| rows[i].text.clone()).collect(),
            gold: (first..rows.len())
                .map(|i| rows[i].label == GOLD_LABEL)
                .collect(),
            a0_score: a0,
        });
    }

    // Case-level holdout: every 4th scored case (deterministic stride),
    // deciding + none_gold only; all_gold pairs train but never evaluate.
    let holdout: Vec<usize> = cases
        .iter()
        .enumerate()
        .filter(|(i, c)| (c.stratum == "deciding" || c.stratum == "none_gold") && i % 4 == 0)
        .map(|(i, _)| i)
        .collect();
    let holdout_row_set: std::collections::BTreeSet<usize> = holdout
        .iter()
        .flat_map(|&ci| {
            let start = cases[..ci].iter().map(|c| c.texts.len()).sum::<usize>();
            start..start + cases[ci].texts.len()
        })
        .collect();
    let rest: Vec<usize> = (0..rows.len())
        .filter(|i| !holdout_row_set.contains(i))
        .collect();

    let (mut dec_tr, mut none_tr) = (0usize, 0usize);
    for (i, c) in cases.iter().enumerate() {
        match c.stratum.as_str() {
            "deciding" if i % 4 == 0 => dec_tr += 1,
            "none_gold" if i % 4 == 0 => none_tr += 1,
            _ => {}
        }
    }
    let gold_pairs = rows.iter().filter(|r| r.label == GOLD_LABEL).count();
    eprintln!(
        "[heal-train] pairs={} (gold {}/nogold {}) cases={} holdout cases {} (deciding {dec_tr}, none_gold {none_tr}) all_gold_train={n_all_gold} test_skipped={n_test_skipped} no_control={n_no_control} pin_dropped={n_pin_dropped}",
        rows.len(),
        gold_pairs,
        rows.len() - gold_pairs,
        cases.len(),
        holdout.len(),
    );

    let labels = vec![NOGOLD_LABEL.to_string(), GOLD_LABEL.to_string()];
    let cfg = NbsvmConfig {
        presence_norm: true,
        ..NbsvmConfig::default()
    };
    let run = train_nbsvm(
        &rows,
        &labels,
        &rest,
        &holdout_row_set.iter().copied().collect::<Vec<_>>(),
        &cfg,
    )
    .expect("train_nbsvm");
    eprintln!(
        "[train] holdout_acc={:.4} control_acc={:.4} loss={:.4} feats={} steps={}",
        run.holdout_accuracy, run.control_accuracy, run.final_loss, run.feats_used, run.steps
    );
    let model = &run.model;
    let gold_idx = model
        .labels
        .iter()
        .position(|l| l == GOLD_LABEL)
        .expect("gold label present");

    // Pre-score every holdout pair once; the τ grid then walks picks.
    let mut holdout_p: Vec<Vec<f32>> = Vec::with_capacity(holdout.len());
    let mut scratch: Vec<u32> = Vec::new();
    let mut bag: Vec<(u32, f32)> = Vec::new();
    for &ci in &holdout {
        let c = &cases[ci];
        let mut ps = Vec::with_capacity(c.texts.len());
        for text in &c.texts {
            bag.clear();
            presence_bag_into(text.as_bytes(), true, &mut bag, &mut scratch);
            let mut out = [0f32; 2];
            model.scores_into(&bag, &mut out);
            ps.push(out[gold_idx]);
        }
        holdout_p.push(ps);
    }

    let mut best: Option<(f64, usize, f64, f32, f32)> = None; // (dec_lb95, none_abstains, all_lb95, tau_p, tau_m)
    for tau_p in [0.0f32, 0.3, 0.5, 0.65, 0.8] {
        for tau_m in [0.0f32, 0.05, 0.1, 0.2, 0.3] {
            let mut dec_a1: Vec<bool> = Vec::new();
            let mut dec_a0: Vec<bool> = Vec::new();
            let mut all_a1: Vec<bool> = Vec::new();
            let mut all_a0: Vec<bool> = Vec::new();
            let (mut abstained, mut none_abstains) = (0usize, 0usize);
            for (k, &ci) in holdout.iter().enumerate() {
                let c = &cases[ci];
                let ps = &holdout_p[k];
                if std::env::var("HEAL_DEBUG")
                    .map(|v| v == "1")
                    .unwrap_or(false)
                    && k < 5
                {
                    eprintln!(
                        "[debug] case {ci} stratum={} gold={:?} ps={:?} text_lens={:?}",
                        c.stratum,
                        c.gold,
                        ps.iter().map(|p| format!("{p:.3}")).collect::<Vec<_>>(),
                        c.texts.iter().map(|t| t.len()).collect::<Vec<_>>(),
                    );
                }
                let (mut top1, mut top2) = (0usize, 0usize);
                for (i, &p) in ps.iter().enumerate() {
                    if p > ps[top1] {
                        top2 = top1;
                        top1 = i;
                    } else if i != top1 && p > ps[top2] {
                        top2 = i;
                    }
                }
                let margin = if c.texts.len() > 1 {
                    ps[top1] - ps[top2]
                } else {
                    ps[top1]
                };
                let abstain = ps[top1] < tau_p || margin < tau_m;
                let a1 = if abstain {
                    abstained += 1;
                    if c.stratum == "none_gold" {
                        none_abstains += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    c.gold[top1]
                };
                if c.stratum == "deciding" {
                    dec_a1.push(a1);
                    dec_a0.push(c.a0_score);
                }
                all_a1.push(a1);
                all_a0.push(c.a0_score);
            }
            let dec = paired_lb95(&dec_a1, &dec_a0).expect("deciding holdout non-empty");
            let all = paired_lb95(&all_a1, &all_a0).expect("scored holdout non-empty");
            println!(
                "[grid] tau_p={tau_p} tau_m={tau_m} deciding_lb95={:.4} ({}/{}) all_lb95={:.4} ({}/{}) abstained={abstained}",
                dec.lb95, dec_a1.iter().filter(|b| **b).count(), dec_a0.iter().filter(|b| **b).count(),
                all.lb95, all_a1.iter().filter(|b| **b).count(), all_a0.iter().filter(|b| **b).count(),
            );
            let better = match best {
                None => true,
                Some((b_dec, b_none, b_all, _, _)) => {
                    (dec.lb95, none_abstains, all.lb95) > (b_dec, b_none, b_all)
                }
            };
            if better {
                best = Some((dec.lb95, none_abstains, all.lb95, tau_p, tau_m));
            }
        }
    }
    let (dec_lb95, none_abstains, all_lb95, tau_p, tau_m) = best.expect("grid non-empty");
    println!(
        "[best] tau_p={tau_p} tau_m={tau_m} deciding_lb95={dec_lb95:.4} all_lb95={all_lb95:.4} none_gold_abstains={none_abstains}"
    );

    let bytes = encode_artifact("heal", model);
    // Consumer-side agreement check: decode through the SERVER's decode
    // path and re-score one pair with the consumer bag law.
    let spec = specialist::decode_artifact(&bytes).expect("artifact re-decodes");
    let c_gold_idx = spec
        .labels
        .iter()
        .position(|l| l == GOLD_LABEL)
        .expect("consumer gold label");
    let probe = &rows[0].text;
    let mut c_bag = Vec::new();
    let mut c_scratch = Vec::new();
    specialist::presence_bag_into(probe.as_bytes(), &mut c_bag, &mut c_scratch);
    let consumer_p = spec.score_class(&c_bag, c_gold_idx);
    let mut t_bag = Vec::new();
    let mut t_scratch = Vec::new();
    presence_bag_into(probe.as_bytes(), true, &mut t_bag, &mut t_scratch);
    let mut t_scores = [0f32; 2];
    model.scores_into(&t_bag, &mut t_scores);
    let train_p = t_scores[gold_idx];
    // The LAW under test is the bag: the consumer and trainer bags must be
    // identical (same events, same values). The sigmoid delta is i8
    // artifact quantization (the server decodes the quantized head, the
    // trainer probes the f32 model) — disclosed and bounded, not asserted
    // to zero.
    let bags_equal = c_bag == t_bag;
    let delta = (consumer_p - train_p).abs();
    println!(
        "[agreement] bags_equal={bags_equal} consumer_p={consumer_p:.6} train_p={train_p:.6} quantization_delta={delta:.6}"
    );
    assert!(bags_equal, "bag law drift between trainer and server");
    assert!(delta < 0.05, "quantization delta beyond bound: {delta}");

    if let Some(parent) = std::path::Path::new(out_path).parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(out_path, &bytes).expect("head writes");
    // The τ travels WITH the artifact as a digest-checked sidecar — the
    // server refuses to serve without it (a hardcoded default is how a
    // bench reads the wrong posture silently; that bug class dies here).
    let sidecar = serde_json::json!({
        "tau_prob": tau_p,
        "tau_margin": tau_m,
        "head_blake3": blake3::hash(&bytes).to_hex().to_string(),
        "trainer": "instinct_heal_train (plan 225 T2)",
        "holdout": {
            "cases": holdout.len(),
            "a1_correct": run.holdout_accuracy,
        },
    });
    let sidecar_path = format!("{out_path}.tau.json");
    fs::write(
        &sidecar_path,
        serde_json::to_vec_pretty(&sidecar).expect("sidecar serializes"),
    )
    .expect("sidecar writes");
    println!(
        "[heal-train] exported {out_path} bytes={} blake3={} labels={:?} tau_p={tau_p} tau_m={tau_m}",
        bytes.len(),
        blake3::hash(&bytes).to_hex(),
        spec.labels,
    );
    println!(
        "[heal-train] INPUT_DIGESTS cases_blake3={} pools_blake3={} split_blake3={}",
        blake3::hash(cases_bytes.as_bytes()).to_hex(),
        blake3::hash(pools_bytes.as_bytes()).to_hex(),
        blake3::hash(&split_bytes).to_hex(),
    );
}

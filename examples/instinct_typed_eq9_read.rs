//! riir-train 632 T3 — the ONE frozen acceptance read for the Eq-9 typed
//! heads: our four heads vs the rethink encoder v2 bar (0.7550) on the
//! SAME seat, scored by the house paired stats.
//!
//! The read walks the reflex seat (`prepare_seat("typed_decisions",
//! datasets_t20k)`) — the EXACT population the encoder's published cell
//! answered (the seat digest asserted against the pick dump's pin) — and
//! picks with the Eq-9 heads:
//! - choice: presented-restricted argmax (ties to the first presented);
//! - noul: the fixed `["no", "yes"]` pair (the wire's false/true law);
//! - score: the qid's head over the presented levels.
//!
//! A pick is recorded as its PRESENTED POSITION (the gold space — choice
//! gold = the key's position in the criteria order; score gold = the
//! level index; noul = 0/1). The encoder's per-row picks join by
//! (case_id, qid) with a total-join + seat-digest assert.
//!
//! Reported: the paired (ours − encoder) mean + LB95 — the ACCEPTANCE
//! gate (LB95 > 0 against the better incumbent) — plus Wilson LB95 on our
//! accuracy, the encoder's accuracy beside the published 0.7550 cell
//! (consistency), pick-confidence Brier/ECE, the majority-class chance
//! floor, a split-half conformal pick floor (Report-the-Floor), and the
//! K-order permutation probe (structural 0 — bag-of-words state features
//! never read the presentation order; ties resolve by presented position,
//! so a flip is exactly a tie broken differently).
//!
//! No latency number is produced here (a pick read is load-invariant);
//! the box state rides the run log's bench_preflight line.
//!
//! ```sh
//! cargo run --release -p riir-instinct --example instinct_typed_eq9_read -- \
//!     --heads ../riir-train/.raw/t3_heads \
//!     --encoder-picks ../riir-train/.raw/t3_heads/typed_encoder_picks.json \
//!     [--datasets ../riir-reflex/.raw/datasets_t20k] [--perm-k 8] \
//!     --out ../riir-train/.benchmarks/639_typed_eq9_frozen_read/read_doc.json
//! ```

use std::collections::HashMap;
use std::path::PathBuf;

use riir_instinct::instinct_specialist::{bag_into, decode_artifact, Specialist};
use riir_instinct::instinct_specialist_typed::{choice_keys, score_levels};
use riir_instinct::instinct_typed_eq9::pick_restricted;
use riir_reflex::harness::runner::case_questions;
use riir_reflex::harness::runner::seat::prepare_seat;

struct Args {
    heads: String,
    encoder_picks: String,
    datasets: String,
    perm_k: usize,
    out: Option<String>,
}

fn parse_args() -> Args {
    let mut a = Args {
        heads: "../riir-train/.raw/t3_heads".into(),
        encoder_picks: "../riir-train/.raw/t3_heads/typed_encoder_picks.json".into(),
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        perm_k: 8,
        out: None,
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        let val = |i: &mut usize, name: &str| -> String {
            *i += 1;
            args.get(*i).unwrap_or_else(|| panic!("{name} needs a value")).clone()
        };
        match args[i].as_str() {
            "--heads" => a.heads = val(&mut i, "--heads"),
            "--encoder-picks" => a.encoder_picks = val(&mut i, "--encoder-picks"),
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--perm-k" => a.perm_k = val(&mut i, "--perm-k").parse().expect("--perm-k number"),
            "--out" => a.out = Some(val(&mut i, "--out")),
            other => panic!("unknown flag {other}"),
        }
        i += 1;
    }
    a
}

fn die(msg: &str) -> ! {
    eprintln!("REFUSE: {msg}");
    std::process::exit(2);
}

/// One sealed artifact, decoded + digest-checked. Returns (model, digest).
fn decode_sealed(dir: &std::path::Path, name: &str) -> (Specialist, String) {
    let p = dir.join(format!("{name}.bin"));
    let bytes = std::fs::read(&p).unwrap_or_else(|e| die(&format!("read {}: {e}", p.display())));
    let side = format!("{}.blake3", p.display());
    let want = std::fs::read_to_string(&side)
        .unwrap_or_else(|e| die(&format!("read {side}: {e}")))
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string();
    let got = blake3::hash(&bytes).to_hex().to_string();
    if got != want {
        die(&format!("{}: seal mismatch ({got} != {want})", p.display()));
    }
    let (suite, model) =
        decode_artifact(&bytes).unwrap_or_else(|e| die(&format!("{}: {e}", p.display())));
    if suite != name {
        die(&format!("{}: artifact names {suite:?}, expected {name:?}", p.display()));
    }
    (model, got)
}

/// The Eq-9 head bundle, decoded from the sealed manifest.
struct Heads {
    choice: Specialist,
    choice_universe: Vec<String>,
    noul: Specialist,
    noul_labels: Vec<String>,
    score: HashMap<String, (Specialist, Vec<String>)>,
    posture: String,
    choice_digest: String,
}

fn load_heads(dir: &std::path::Path) -> Heads {
    let mpath = dir.join("typed_eq9_manifest.json");
    let text = std::fs::read_to_string(&mpath)
        .unwrap_or_else(|e| die(&format!("read {}: {e} — train first", mpath.display())));
    let side = format!("{}.blake3", mpath.display());
    let expected = std::fs::read_to_string(&side)
        .unwrap_or_else(|e| die(&format!("read {side}: {e}")))
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string();
    let got = blake3::hash(text.as_bytes()).to_hex().to_string();
    if got != expected {
        die(&format!("manifest seal mismatch ({got} != {expected}) — refuse"));
    }
    let m: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("manifest: {e}")));
    let (choice, choice_digest) = decode_sealed(dir, "typed_eq9_choice");
    let (noul, _) = decode_sealed(dir, "typed_eq9_noul");
    let choice_universe: Vec<String> =
        serde_json::from_value(m["choice"]["universe"].clone())
            .unwrap_or_else(|e| die(&format!("manifest choice.universe: {e}")));
    let noul_labels: Vec<String> = serde_json::from_value(m["noul"]["labels"].clone())
        .unwrap_or_else(|e| die(&format!("manifest noul.labels: {e}")));
    let mut score = HashMap::new();
    if let Some(map) = m["score"].as_object() {
        for qid in map.keys() {
            let labels: Vec<String> =
                serde_json::from_value(map[qid]["labels"].clone())
                    .unwrap_or_else(|e| die(&format!("manifest score {qid} labels: {e}")));
            let (model, _) = decode_sealed(dir, &format!("typed_eq9_score_{qid}"));
            score.insert(qid.clone(), (model, labels));
        }
    }
    let posture = m["inputs"]["posture"].as_str().unwrap_or("unknown").to_string();
    if !posture.contains("dual-shuffle") {
        eprintln!(
            "WARNING: the head bundle's posture is {posture:?} — the frozen read expects the dual-shuffle lane"
        );
    }
    Heads { choice, choice_universe, noul, noul_labels, score, posture, choice_digest }
}

/// A deterministic u64 from a question's identity (the perm-probe seed
/// law — blake3's first 8 bytes, big-endian).
fn seed_u64(s: &str) -> u64 {
    let d = blake3::hash(s.as_bytes());
    u64::from_be_bytes(d.as_bytes()[..8].try_into().expect("8 bytes"))
}

fn main() {
    let a = parse_args();
    let heads_dir = PathBuf::from(&a.heads);
    let heads = load_heads(&heads_dir);
    println!(
        "instinct_typed_eq9_read: heads {} ({}) · encoder picks {} · perm K {}",
        a.heads, heads.posture, a.encoder_picks, a.perm_k,
    );

    // ── the seat: the EXACT population the encoder cell answered ────────
    let seat = prepare_seat("typed_decisions", &PathBuf::from(&a.datasets))
        .unwrap_or_else(|e| die(&format!("prepare_seat: {e}")));
    let seat_digest = seat.cases_digest();
    let cases = &seat.suite.cases;
    let strs = &seat.state_strs;
    let gold: Vec<usize> =
        cases.iter().flat_map(|c| c.gold.iter().map(|g| g.idx)).collect();
    println!("seat: {seat_digest} · {} cases / {} questions", cases.len(), gold.len());

    // ── the encoder pick dump: join key + population assert ─────────────
    let enc_text = std::fs::read_to_string(&a.encoder_picks).unwrap_or_else(|e| {
        die(&format!(
            "read {}: {e} — run arena_encoder_read --dump-picks first",
            a.encoder_picks
        ))
    });
    let enc: serde_json::Value =
        serde_json::from_str(&enc_text).unwrap_or_else(|e| die(&format!("encoder picks: {e}")));
    if enc["kind"].as_str() != Some("typed_encoder_picks") {
        die("encoder picks: wrong kind");
    }
    if enc["seat_digest"].as_str() != Some(seat_digest.as_str()) {
        die(&format!(
            "encoder picks seat digest {} != our seat {seat_digest} — different populations",
            enc["seat_digest"].as_str().unwrap_or("?")
        ));
    }
    let enc_rows =
        enc["rows"].as_array().unwrap_or_else(|| die("encoder picks: no rows"));
    if enc_rows.len() != gold.len() {
        die(&format!(
            "encoder picks: {} rows vs our seat's {} questions",
            enc_rows.len(),
            gold.len()
        ));
    }
    let mut enc_by_key: HashMap<(String, String), bool> = HashMap::new();
    for r in enc_rows {
        let key = (
            r["case_id"].as_str().unwrap_or_default().to_string(),
            r["qid"].as_str().unwrap_or_default().to_string(),
        );
        let correct =
            r["correct"].as_bool().unwrap_or_else(|| die("encoder pick row: no correct"));
        if enc_by_key.insert(key, correct).is_some() {
            die("encoder picks: duplicate (case_id, qid)");
        }
    }

    // ── our picks (the frozen walk) ─────────────────────────────────────
    // Per question: (presented position or None, picked sigmoid, gold-key
    // sigmoid, presented keys). The position is the gold space.
    struct Pick {
        pos: Option<usize>,
        conf: f32,
        p_gold: f32,
    }
    let mut picks: Vec<Pick> = Vec::with_capacity(gold.len());
    let mut walk_keys: Vec<(String, String)> = Vec::with_capacity(gold.len());
    let mut perm_flips = 0usize;
    let mut perm_reads = 0usize;

    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    for (ci, case) in cases.iter().enumerate() {
        bag.clear();
        bag_into(strs[ci].as_bytes(), &mut bag, &mut scratch);
        let qs = case_questions(case);
        if qs.len() != case.gold.len() {
            die(&format!("case {}: questions/gold lockstep broken", case.id));
        }
        for (qi, (qid, qdef)) in qs.iter().enumerate() {
            let kind = qdef.get("type").and_then(serde_json::Value::as_str).unwrap_or("");
            let crit = qdef.get("criteria").cloned().unwrap_or(serde_json::Value::Null);
            let (presented, model, universe): (Vec<String>, &Specialist, &[String]) = match kind
            {
                "choice" => (
                    choice_keys(&crit).unwrap_or_default(),
                    &heads.choice,
                    &heads.choice_universe,
                ),
                "noul" => (
                    vec!["no".to_string(), "yes".to_string()],
                    &heads.noul,
                    &heads.noul_labels,
                ),
                "score" => match heads.score.get(qid) {
                    Some((m, uni)) => (score_levels(&crit).unwrap_or_default(), m, uni),
                    None => {
                        // An untrained score qid — unanswered, counted.
                        picks.push(Pick { pos: None, conf: 0.0, p_gold: 0.0 });
                        walk_keys.push((case.id.clone(), qid.clone()));
                        continue;
                    }
                },
                other => die(&format!("case {} q{qi}: unknown kind {other:?}", case.id)),
            };
            let mut scores = vec![0.0f32; universe.len()];
            model.scores_into(&bag, &mut scores);
            let picked = pick_restricted(model, &bag, universe, &presented);
            // The gold key's own sigmoid (the conformal floor's
            // nonconformity input), read off the same score vector.
            let p_gold = case
                .gold
                .get(qi)
                .and_then(|g| presented.get(g.idx))
                .and_then(|gk| universe.iter().position(|u| u == gk))
                .map(|gcls| scores[gcls])
                .unwrap_or(0.0);
            let pick = picked.map(|(cls, _conf)| {
                // class index → presented position (the gold space)
                let picked_key: &String = &universe[cls];
                presented.iter().position(|k| k == picked_key).unwrap_or(usize::MAX)
            });
            picks.push(Pick {
                pos: pick,
                conf: picked.map(|(_, c)| c).unwrap_or(0.0),
                p_gold,
            });
            walk_keys.push((case.id.clone(), qid.clone()));

            // The permutation probe: K seeded shuffles of the presented
            // keys — the bag never reads order, so a flip is exactly a
            // tie broken differently (structural 0 expected).
            if a.perm_k > 0 && pick.is_some() && presented.len() > 1 {
                for k in 1..=a.perm_k {
                    let mut order = presented.clone();
                    let seed_src = format!("{}|{}|{}", case.id, qid, k);
                    let mut state = seed_u64(&seed_src);
                    for i in (1..order.len()).rev() {
                        state = state
                            .wrapping_mul(6364136223846793005)
                            .wrapping_add(1442695040888963407);
                        let j = ((state >> 33) as usize) % (i + 1);
                        order.swap(i, j);
                    }
                    let flipped = match kind {
                        "choice" => {
                            pick_restricted(&heads.choice, &bag, &heads.choice_universe, &order)
                        }
                        "noul" => pick_restricted(&heads.noul, &bag, &heads.noul_labels, &order),
                        _ => heads
                            .score
                            .get(qid)
                            .and_then(|(m, uni)| pick_restricted(m, &bag, uni, &order)),
                    };
                    perm_reads += 1;
                    // Flip = the picked POSITION changed (compare in the
                    // presented-position space the gold speaks).
                    let flipped_pos = flipped.map(|(cls, _)| {
                        let key: &String = &universe[cls];
                        presented.iter().position(|k| k == key).unwrap_or(usize::MAX)
                    });
                    if flipped_pos != picks.last().and_then(|p| p.pos) {
                        perm_flips += 1;
                    }
                }
            }
        }
    }

    // ── the pairing + the gate ──────────────────────────────────────────
    let n = picks.len();
    if n != gold.len() {
        die(&format!("walk produced {n} picks for {} gold rows", gold.len()));
    }
    let mut ours_correct: Vec<bool> = Vec::with_capacity(n);
    let mut enc_correct: Vec<bool> = Vec::with_capacity(n);
    let mut confs: Vec<f64> = Vec::with_capacity(n);
    let mut p_golds: Vec<f64> = Vec::with_capacity(n);
    let mut answered = 0usize;
    for (i, key) in walk_keys.iter().enumerate() {
        let enc_hit = match enc_by_key.get(key) {
            Some(v) => *v,
            None => die(&format!(
                "encoder picks missing {}:{} — the join must be total",
                key.0, key.1
            )),
        };
        let p = &picks[i];
        let hit = p.pos == Some(gold[i]);
        ours_correct.push(hit);
        enc_correct.push(enc_hit);
        confs.push(p.conf as f64);
        p_golds.push(p.p_gold as f64);
        if p.pos.is_some() {
            answered += 1;
        }
    }
    let ours_hits = ours_correct.iter().filter(|&&b| b).count();
    let ours_acc = ours_hits as f64 / n as f64;
    let enc_acc = enc_correct.iter().filter(|&&b| b).count() as f64 / n as f64;
    let pd = riir_instinct::stats::paired_upper_bound(&ours_correct, &enc_correct)
        .unwrap_or_else(|| die("paired stats: empty"));
    let wilson = riir_instinct::stats::wilson_lb(ours_hits, n);
    println!();
    println!("OURS: {ours_acc:.4} ({ours_hits}/{n} · answered {answered}) · Wilson LB95 {wilson:.4}");
    println!(
        "ENCODER: {enc_acc:.4} — the published cell reads 0.7550; consistency {}",
        if (enc_acc - 0.7550).abs() < 0.005 { "OK (±0.5 pt)" } else { "MATERIAL DRIFT — read before quoting" }
    );
    println!(
        "PAIRED (ours − encoder): mean {:+.4} · LB95 {:+.4} · UB95 {:+.4} — GATE {}",
        pd.mean,
        pd.lb95,
        pd.ub95,
        if pd.lb95 > 0.0 { "PASS (LB95 > 0)" } else { "FAIL" },
    );
    println!(
        "PERM-PROBE: {perm_flips} flips / {perm_reads} reads (K = {}) — structural 0 expected",
        a.perm_k,
    );

    // ── Report-the-Floor: calibration + chance + split-half conformal ──
    let cal_pairs: Vec<(f64, bool)> =
        confs.iter().copied().zip(ours_correct.iter().copied()).collect();
    let brier = riir_reflex::harness::metrics::brier_of(&cal_pairs);
    let ece = riir_reflex::harness::metrics::ece_of(&cal_pairs);
    let chance = riir_reflex::harness::metrics::chance_majority(&gold);
    // Split-half conformal (the Plan-340 naive shape, classification
    // form): nonconformity = 1 − p(gold) on the cal half → the ⌈0.9·n⌉-th
    // ordered value τ; a test pick is IN the singleton set iff
    // 1 − p(pick) ≤ τ. Coverage = gold-in-set (correct AND confident).
    let half = n / 2;
    let mut cal_nc: Vec<f64> =
        (0..half).map(|i| (1.0 - p_golds[i]).clamp(0.0, 1.0)).collect();
    cal_nc.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q_idx = ((cal_nc.len() as f64) * 0.9).ceil() as usize;
    let tau = cal_nc.get(q_idx.saturating_sub(1)).copied().unwrap_or(1.0);
    let mut covered = 0usize;
    for i in half..n {
        let in_set = (1.0 - confs[i]) <= tau;
        if in_set && ours_correct[i] {
            covered += 1;
        }
    }
    let cov = covered as f64 / (n - half).max(1) as f64;
    println!(
        "FLOOR: chance(majority) {chance:.4} · pick-Brier {brier:.4} · ECE {ece:.4} · conformal τ {tau:.4} → singleton coverage {cov:.4} (α = 0.1, split-half)"
    );
    println!(
        "G1 (Report-the-Floor): our acc {ours_acc:.4} vs chance {chance:.4} — skill {:+.4}",
        ours_acc - chance,
    );

    // ── the read doc ────────────────────────────────────────────────────
    let doc = serde_json::json!({
        "kind": "riir_train_632_t3_eq9_frozen_read",
        "version": 1,
        "heads": {
            "dir": a.heads,
            "posture": heads.posture,
            "choice_blake3": heads.choice_digest,
        },
        "seat": {
            "digest": seat_digest,
            "cases": cases.len(),
            "questions": n,
            "datasets": a.datasets,
        },
        "encoder_picks": {
            "path": a.encoder_picks,
            "head_blake3": enc["head_blake3"].clone(),
            "acc": enc_acc,
            "published_cell_acc": 0.7550,
            "consistent_with_published_cell": (enc_acc - 0.7550).abs() < 0.005,
        },
        "ours": {
            "acc": ours_acc,
            "hits": ours_hits,
            "n": n,
            "answered": answered,
            "wilson_lb95": wilson,
        },
        "paired": {
            "mean": pd.mean,
            "lb95": pd.lb95,
            "ub95": pd.ub95,
            "gate": "paired (ours - encoder) LB95 > 0 vs the per-suite better incumbent (typed bar = rethink encoder 0.7550, NOT the served H2 0.6475)",
            "verdict": if pd.lb95 > 0.0 { "PASS" } else { "FAIL" },
        },
        "perm_probe": {
            "flips": perm_flips,
            "reads": perm_reads,
            "k": a.perm_k,
            "expected": "structural 0 (bag-of-words state features never read the presentation order; ties resolve by presented position)",
        },
        "floor": {
            "chance_majority": chance,
            "pick_brier": brier,
            "ece": ece,
            "conformal": {
                "tau": tau,
                "coverage": cov,
                "alpha": 0.1,
                "shape": "split-half, nonconformity = 1 - p(gold) cal / 1 - p(pick) test, singleton sets (Report-the-Floor, katgpt-rs Plan 340 classification form)",
            },
        },
        "box_state_note":
            "pick read — latency deliberately not quoted; the run log carries the bench_preflight line for the box state",
        "generated_by": "examples/instinct_typed_eq9_read.rs",
    });
    let text = serde_json::to_string_pretty(&doc).expect("read doc json");
    match &a.out {
        Some(p) => {
            if let Some(dir) = std::path::Path::new(p).parent() {
                if !dir.as_os_str().is_empty() {
                    std::fs::create_dir_all(dir)
                        .unwrap_or_else(|e| die(&format!("create: {e}")));
                }
            }
            std::fs::write(p, &text).unwrap_or_else(|e| die(&format!("write {p}: {e}")));
            println!("read doc: {p}");
        }
        None => println!("{text}"),
    }
}

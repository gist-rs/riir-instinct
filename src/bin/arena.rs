//! The "Reflex · instinct" arena (riir-instinct Issue 003 T3/T4, Issue
//! 005 T2–T5 — Plan 001 P3/P3a): the GOAT run over the trained-specialist
//! suites.
//!
//! Per suite, the arena prepares the byte-identical question seat (reflex
//! harness), fits the DEPLOYED modelless posture (the Bench 051 protocol —
//! head-select + nb-select, registry caps), builds the seat engine, and
//! loads the sealed winner artifact. Then every arm runs — A0 reflex, A1
//! instinct, H1 cascade, and the H2 prior fusion grid (β·n_min·τ_n) —
//! first on the cal front, where the pre-registration instrument (Pareto
//! rank-0, then argmax Beta-LCB) registers the suite's arm, and then ONCE
//! on the test split. The single frozen read lands in `predictions.json`.
//!
//! The gates of Issue 005 close the run. G0 identity — in-code, unit
//! tests, and the A0 drift pin against reflex's own `run()`. G1
//! calibration — uncalibrated vs Platt vs the conformal-naive floor. G2
//! latency — the fusion-overhead micro, absolute p99, and the laya paired
//! face behind `--features arena-laya`. G3 — paired non-inferiority on
//! the reflex-won suites. G5 — the pre-registered arm on the gap suites.
//! G4 alloc-free lives in `tests/g4_alloc.rs`. G6 purity — frozen counts
//! and the sealed artifact; the HOSTED-ONLY vessel lands with P4
//! (disclosed, never claimed here).
//!
//! Output: `<out>/RESULTS.md` + `predictions.json` + `registration.json`.
//!
//! Population (Issue 010 T1): every reflex DATASET suite the seat can
//! prepare — the six specialist suites carry the full arm grid, and a
//! suite WITHOUT a winner artifact still runs its honest A0/G0-only
//! posture (T2: a missing specialist is not a crash; the row's verdict
//! is `a0_stands`, never a silent absence).

use std::collections::HashMap;
use std::hint::black_box;
use std::path::{Path, PathBuf};

use riir_instinct::{
    A0Answer, ArmStat, Cascade, HybridLane, MAX_TOP_K, NOUL_PAIR, PairedDiff, PriorFusion,
    SpecialistLane, delta_suite, paired_upper_bound, pareto_rank0, prior_fusion_pick, select_arm,
    wilson_lb,
};
use riir_reflex::embed::EMBED_DIM;
use riir_reflex::engine::DecisionEngine;
use riir_reflex::harness::metrics::{CalibrationPair, conformal_naive_floor, ece_of};
use riir_reflex::harness::runner::seat::{
    PostureKnobs, Seat, SeatEval, SeatPosture, build_seat_engine, eval_seat, fit_posture,
    prepare_seat,
};
use riir_reflex::harness::suites::{QKind, SuiteCase, SuiteQuestion};
use riir_reflex::nb_scope::NbView;

/// The arena population (Issue 010 T1): every reflex DATASET suite. The
/// first six carry riir-train Issue 576 winner artifacts (the full arm
/// grid); `typed_decisions` + `prompt_injections` carry the Issue 578
/// winners through Plan 003's noul bridge. Label-arity dispatch below must
/// cover their train-derived label counts (4 workflows / 2 noul classes,
/// measured 2026-09-28 — the plan's 3-workflow guess was stale).
/// Issue 010 T6 (2026-09-28): the harness families + `code_fixtures` join
/// — the reflex Issue-049 seat seam now seats them (marked synthetic);
/// no winner artifacts exist for them, so they run the A0/G0-only
/// posture and publish `a0_stands` rows (the expected outcome per 049's
/// scope note). `harness_cache_reuse` stays OUT (the seat refuses it —
/// the modelless lane has no KV cache; a modelless answer would be a
/// fake task), disclosed here and by the seat's own refusal.
const SUITES: &[&str] = &[
    "ag_news",
    "emotion",
    "sst5",
    "xnli_en",
    "massive_intent_en",
    "banking77",
    "typed_decisions",
    "prompt_injections",
    "harness_visibility",
    "harness_permissions",
    "harness_tool_fit",
    "harness_routing",
    "harness_sensitivity",
    "code_fixtures",
];
/// Reflex already wins these (Bench 051) — G3's non-inferiority duty.
const REFLEX_WON: &[&str] = &["emotion", "sst5", "massive_intent_en", "banking77"];
/// Laya wins these (Bench 051) — G5's pre-registration duty.
const GAP_SUITES: &[&str] = &["ag_news", "xnli_en"];

/// The H2 grid (train-side selection only): β, n_min, τ_n.
const BETA_GRID: &[f32] = &[0.0, 0.25, 0.5, 1.0, 2.0];
const N_MIN_GRID: &[f32] = &[2.0, 4.0, 8.0];
const TAU_GRID: &[f32] = &[2.0, 4.0, 8.0];

/// One arm's evaluation over a case set (flattened questions — the six
/// suites are one question per case, asserted at the seat).
#[derive(Debug, Clone)]
struct ArmOut {
    name: String,
    correct: Vec<bool>,
    picks: Vec<usize>,
    probs: Vec<Vec<f64>>,
    confs: Vec<f64>,
    escalated: Vec<bool>,
    /// The fused gate's abstention, per question — recorded so the
    /// serving-side parity gate can compare the SERVE's first-class
    /// abstention against the record. A0's picks/correct follow reflex's
    /// HARD convention (abstains forced to their argmax pick — the
    /// published accuracy), so WITHOUT this field an A0 abstain was
    /// invisible in the record and the serve's `pick: None` read as a
    /// drift (measured: ag_news case 0, Bench 004's first serve_gates
    /// run). A1/H2 never abstain.
    abstained: Vec<bool>,
    /// The arm's OWN per-question work (µs): A0 = the reflex solve, H1 =
    /// its decision alone, A1/H2 = their forward alone.
    own_durs_us: Vec<f64>,
    /// The composed per-question latency (µs). The SCOPE is per-arm and
    /// named by `contains_seat_solve` (Issue 007): seat-composing arms
    /// (A0, H1) include the reflex solve; specialist-side arms (A1/H2)
    /// time their own forward alone — they do not run reflex.
    total_durs_us: Vec<f64>,
    /// The scope law (Issue 007): does `total_durs_us` include the reflex
    /// seat solve? "End-to-end" is a claim only a true arm may make.
    contains_seat_solve: bool,
}

impl ArmOut {
    fn accuracy(&self) -> f64 {
        if self.correct.is_empty() {
            0.0
        } else {
            self.correct.iter().filter(|&&c| c).count() as f64 / self.correct.len() as f64
        }
    }
    fn consult_rate(&self) -> f64 {
        if self.escalated.is_empty() {
            0.0
        } else {
            self.escalated.iter().filter(|&&e| e).count() as f64 / self.escalated.len() as f64
        }
    }
    fn p50(&self) -> f64 {
        pct(&self.total_durs_us, 0.50)
    }
    fn p99(&self) -> f64 {
        pct(&self.total_durs_us, 0.99)
    }
    fn n_correct(&self) -> u32 {
        self.correct.iter().filter(|&&c| c).count() as u32
    }
}

/// Nearest-rank percentile over µs samples (the harness law).
fn pct(durs: &[f64], q: f64) -> f64 {
    if durs.is_empty() {
        return 0.0;
    }
    let mut d = durs.to_vec();
    d.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = ((d.len() as f64) * q).ceil() as usize;
    d[idx.saturating_sub(1).min(d.len() - 1)]
}

/// A registration candidate: the arm identity + its H2 params.
#[derive(Debug, Clone, PartialEq)]
enum Cand {
    A0,
    A1,
    H1,
    H2 { beta: f32, n_min: f32, tau_n: f32 },
}

impl Cand {
    fn name(&self) -> String {
        match self {
            Cand::A0 => "A0".into(),
            Cand::A1 => "A1".into(),
            Cand::H1 => "H1".into(),
            Cand::H2 { beta, n_min, tau_n } => {
                format!("H2(β={beta},nmin={n_min},τ={tau_n})")
            }
        }
    }
}

struct RegRow {
    cand: Cand,
    arm: String,
    cal_acc: f64,
    acc_lcb: f64,
    consult: f64,
    p99_us: f64,
    rank0: bool,
}

struct G1Face {
    ece_raw: f64,
    ece_platt: f64,
    ece_floor: f64,
    platt_moved: bool,
    pass: bool,
}

struct LayaFace {
    escalated_n: usize,
    lane_p50_us: f64,
    laya_p50_us: f64,
    paired_ub95_us: f64,
    pass: bool,
}

struct SuiteRun {
    suite: String,
    /// Total QUESTIONS across the test read (the accuracy denominators;
    /// a multi-question suite counts every question — reflex's own
    /// hard-metrics convention).
    n_questions: usize,
    /// Test CASES (a typed_decisions case carries 5 questions).
    n_cases: usize,
    /// False = the A0/G0-only posture ran (no winner artifact — Issue 010
    /// T2): no A1/H1/H2 exist, the fusion micro is N/A, and the row's
    /// verdict is `a0_stands`.
    specialist_present: bool,
    /// The three-state lane verdict (Issue 010 T3): `hybrid_arm` (a
    /// registered non-A0 arm, gates pass) or `a0_stands` (measured, no
    /// promotable hybrid arm — refused by the gate or no specialist).
    verdict: &'static str,
    /// Why the verdict is `a0_stands` (None when a hybrid arm serves):
    /// the refusal numbers, an outright A0 registration, or the missing
    /// specialist. The doc builder publishes it verbatim — a name list
    /// cannot carry the measurement (Issue 010 root cause 3).
    a0_note: Option<String>,
    posture: DisclosedPosture,
    registration: Vec<RegRow>,
    /// The arm that SERVES — post-product-gate (Issue 008 T2): the
    /// instrument's pick when it clears the superiority bar, else A0.
    registered: Cand,
    /// The superiority-gate face (Some iff the instrument picked a
    /// non-A0 candidate): the paired (pick − A0) reading on the frozen
    /// test read and the gate's verdict.
    superiority: Option<SuperiorityFace>,
    test_arms: Vec<ArmOut>,
    h1_ns_decision: f64,
    h1_ns_fusion: f64,
    h2_ns_per_option: f64,
    g1: Option<G1Face>,
    g3: Option<PairedDiff>,
    g3_delta: Option<f64>,
    g5: Option<(bool, String)>,
    laya: Option<LayaFace>,
}

/// The Issue-008 T2 product gate's face: the registered arm must be
/// STRICTLY above the current Reflex row — the paired delta's 95% lower
/// bound > 0 on the frozen test read (`stats::PairedDiff::lb95`). An
/// arm that fails is REFUSED: A0 serves and the suite is not sold.
#[derive(Debug, Clone)]
struct SuperiorityFace {
    /// The instrument's pick the gate judged.
    pick: Cand,
    /// Paired (pick − A0) mean accuracy delta on the test read.
    mean: f64,
    /// Its 95% lower bound — the gate: > 0 to register.
    lb95: f64,
    passed: bool,
}

struct DisclosedPosture {
    effective_cap: usize,
    head_scale: f32,
    nb_scale: f32,
    /// The cal-selected NBSVM-ridge readout scale (0.0 = off — the
    /// ladder declined). Emotion arms 8 (Bench 057); every other arena
    /// suite reads 0 by the same arming bar.
    ridge_scale: f32,
    nb_view: &'static str,
    score_threshold: f32,
    distance_threshold: f32,
    /// The winner artifact's input-bag convention (Issue 579's bridge):
    /// "count" for the v1 winners, "presence" for the banking77 nbsvm
    /// v2 lane.
    bag_convention: &'static str,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut datasets_dir = PathBuf::from("../riir-reflex/.raw/datasets_t20k");
    let mut winners_dir = PathBuf::from("../riir-train/data/instinct_specialists");
    let mut out_dir = PathBuf::from(".benchmarks/001_hybrid_goat");
    let mut top_k = 8usize;
    let mut pin_a0 = true;
    let mut only_suites: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--datasets-dir" => {
                i += 1;
                datasets_dir = PathBuf::from(&args[i]);
            }
            "--winners-dir" => {
                i += 1;
                winners_dir = PathBuf::from(&args[i]);
            }
            "--out" => {
                i += 1;
                out_dir = PathBuf::from(&args[i]);
            }
            "--top-k" => {
                i += 1;
                top_k = args[i].parse().expect("--top-k needs a number");
            }
            "--suite" => {
                i += 1;
                only_suites.push(args[i].clone());
            }
            "--skip-pin-a0" => pin_a0 = false,
            other => die(&format!("unknown arg {other}")),
        }
        i += 1;
    }
    if !(1..=MAX_TOP_K).contains(&top_k) {
        die("--top-k out of range (1..=32)");
    }
    // The focused-read filter (Plan 003's T5 frozen reads): run a subset
    // of the population; the A0 pin covers exactly what ran. Repeatable.
    let suites: Vec<&'static str> = if only_suites.is_empty() {
        SUITES.to_vec()
    } else {
        only_suites
            .iter()
            .map(|s| {
                SUITES
                    .iter()
                    .find(|x| **x == s.as_str())
                    .copied()
                    .unwrap_or_else(|| {
                        die(&format!(
                            "unknown suite {s} — the arena seats {SUITES:?}"
                        ))
                    })
            })
            .collect()
    };

    eprintln!("=== riir-instinct arena — the Reflex · instinct GOAT run ===");
    eprintln!(
        "datasets: {} · winners: {} · out: {} · top_k {top_k}",
        datasets_dir.display(),
        winners_dir.display(),
        out_dir.display()
    );
    let box_state = riir_reflex::harness::box_state::capture();
    let box_summary = format_box_state(&box_state);
    eprintln!("box: {box_summary}");

    let mut runs: Vec<SuiteRun> = Vec::new();
    for suite in &suites {
        match run_suite(suite, &datasets_dir, &winners_dir, top_k) {
            Ok(run) => runs.push(run),
            Err(e) => die(&format!("{suite}: {e}")),
        }
    }

    // The A0 drift pin: the seat path must reproduce reflex's own lane,
    // byte for byte, on EVERY arena suite that ran — and, when the
    // reflex-site checkout stands beside the workspace, reflex's rows must
    // equal the PUBLISHED bench.json numbers (the Issue-008 T1 re-baseline
    // law).
    if pin_a0 {
        if let Err(e) = pin_a0_identity(&datasets_dir, &runs, &suites) {
            die(&format!("A0 identity pin FAILED: {e}"));
        }
        eprintln!(
            "A0 identity pin: all {} arena suites — arena A0 == reflex run() at the \
             published-posture knobs ✓",
            runs.len()
        );
    }

    std::fs::create_dir_all(&out_dir).expect("create out dir");
    write_predictions(&out_dir, &runs);
    write_results(&out_dir, &runs, &box_summary);
    eprintln!("=== done — results in {} ===", out_dir.display());
}

fn die(msg: &str) -> ! {
    eprintln!("⛔ arena: {msg}");
    std::process::exit(1);
}

/// The box-state disclosure line (the Issue-021 law — the fields, flat).
fn format_box_state(b: &riir_reflex::harness::box_state::BoxState) -> String {
    format!(
        "power {:?} · powermode {:?} · load1m {:?} · swap_mb {:?} · quotable {:?} · refusals {:?}",
        b.power, b.power_mode, b.load_1m, b.swap_used_mb, b.latency_quotable, b.refusals
    )
}

fn run_suite(
    name: &str,
    datasets_dir: &Path,
    winners_dir: &Path,
    top_k: usize,
) -> Result<SuiteRun, String> {
    eprintln!("--- {name} ---");
    let seat = prepare_seat(name, datasets_dir)?;
    eprintln!(
        "  seat: {} test cases · {} cal cases · {} labels · pool {} docs",
        seat.suite.cases.len(),
        seat.cal_cases.len(),
        seat.labels.len(),
        seat.train.len()
    );
    match seat.labels.len() {
        2 => run_suite_n::<2>(name, seat, winners_dir, top_k),
        3 => run_suite_n::<3>(name, seat, winners_dir, top_k),
        4 => run_suite_n::<4>(name, seat, winners_dir, top_k),
        5 => run_suite_n::<5>(name, seat, winners_dir, top_k),
        6 => run_suite_n::<6>(name, seat, winners_dir, top_k),
        8 => run_suite_n::<8>(name, seat, winners_dir, top_k),
        59 => run_suite_n::<59>(name, seat, winners_dir, top_k),
        77 => run_suite_n::<77>(name, seat, winners_dir, top_k),
        n => Err(format!("no engine arity for {n} labels — extend the dispatch")),
    }
}

/// Per-suite context: the seat engine + the joined specialist + the
/// scratch (all allocation happens here; the per-question paths below
/// are allocation-free — G4).
struct SuiteCtx<const N: usize> {
    engine: DecisionEngine<N, EMBED_DIM>,
    lane: HybridLane,
    /// The winner artifact's training convention (Issue 579's bridge) —
    /// every bag this context builds dispatches through it.
    conv: riir_instinct::specialist::BagConvention,
    nb_view: NbView,
    nb_armed: bool,
    bag: Vec<(u32, f32)>,
    tok: Vec<u32>,
    survivors: [(usize, f64); MAX_TOP_K],
    h1_scores: [f32; MAX_TOP_K],
    in_scores: Vec<f32>,
    nb_scratch: Vec<u32>,
    /// Presented-option bridge, built once per suite: criteria key →
    /// (seat label idx, artifact class row). The engine's answer space is
    /// the question's presented criteria keys — `engine_request`'s
    /// iteration order — and gold.idx speaks the same space; the
    /// specialist's space is the label universe. The per-case position
    /// vectors below are this map's product (Issue 006: indexing the
    /// label permutation by position instead is the instrument defect
    /// that read massive at chance).
    key_map: HashMap<String, (usize, usize)>,
    /// The seat labels are CONTEXT (Plan 003's `SeatJoin::Context` —
    /// typed_decisions' workflow names): the identity-by-count resolve is
    /// disabled (a presented-key count against an unrelated label count
    /// is meaningless) and every key must resolve BY NAME — a miss is
    /// template drift against the distractor-universe law, loud.
    context_joined: bool,
    /// Seat label idx → artifact class row (the join permutation).
    perm: Vec<usize>,
    pos_label: Vec<usize>,
    pos_class: Vec<usize>,
    pos_spec: Vec<f32>,
    pos_nb: Vec<f32>,
}

/// One H2 grid point's accumulated readings: (params, picks, correct,
/// confs, per-question µs).
type GridRow = (Cand, Vec<usize>, Vec<bool>, Vec<f64>, Vec<f64>);

/// A question's presented option names, in presentation order — the
/// choice's criteria OBJECT keys, the score's criteria ARRAY levels, and
/// the noul pair (the fixed `[false, true]` rendering mapped onto
/// [`NOUL_PAIR`]). The one extraction both [`SuiteCtx::fill_positions`]
/// and the context seat's coverage gate read, so the two cannot drift.
fn presented_keys(q: &SuiteQuestion) -> Vec<String> {
    if q.kind == QKind::Noul {
        return NOUL_PAIR.iter().map(|s| (*s).to_string()).collect();
    }
    if let Some(obj) = q.criteria.as_object() {
        obj.keys().cloned().collect()
    } else if let Some(levels) = q.criteria.as_array() {
        levels
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect()
    } else {
        Vec::new()
    }
}

impl<const N: usize> SuiteCtx<N> {
    /// Fill the per-question position maps from the question's presented
    /// criteria keys, in the object's own order — the exact iteration
    /// `engine_request` feeds the engine, so position p here IS the
    /// engine's answer index p (and the gold index's space). The
    /// position → domain rule mirrors the engine's own rules
    /// (`solve_into`): NOUL questions (Plan 003) present the fixed
    /// `[false, true]` rendering — gold idx 0/1 speak it — mapped onto
    /// [`NOUL_PAIR`] by NAME through the key map (real seat indices where
    /// the suite's own universe is the pair — prompt_injections' positional
    /// join; the artifact-known sentinel where it is not —
    /// typed_decisions' workflow seats, whose NB margin then mutes).
    /// CHOICE/SCORE questions follow the engine's two rules: every key
    /// resolving to a known label → BY NAME (massive/banking77, whose
    /// keys are the label strings); otherwise count == label count →
    /// IDENTITY BY INDEX (the fixed-criteria suites ag_news/emotion/sst5/
    /// xnli, whose keys are display names over int-string domains).
    /// Anything else is the engine's route-less shape — the specialist
    /// bridge is undefined there, so REFUSE loud (Issue 006: the mismatch
    /// of these two spaces is the instrument defect that read massive at
    /// chance).
    ///
    /// A key naming an ARTIFACT-KNOWN label the seat never offers (the
    /// test-split universe can be a strict subset of the artifact's
    /// train-derived one) resolves to the sentinel seat index —
    /// scoreable by the specialist, NaN-evidence for the NB margin.
    fn fill_positions(&mut self, case_id: &str, q: &SuiteQuestion) {
        self.pos_label.clear();
        self.pos_class.clear();
        if q.kind == QKind::Noul {
            for name in NOUL_PAIR {
                let Some(&(li, cls)) = self.key_map.get(name) else {
                    panic!(
                        "case {case_id}: the noul bridge needs a {name:?} class row — the \
                         artifact's unified no/yes pair is missing from the key map"
                    );
                };
                self.pos_label.push(li);
                self.pos_class.push(cls);
            }
            return;
        }
        // The presented options, in presentation order (see
        // [`presented_keys`] — the one extraction, shared with the
        // context seat's coverage gate).
        let keys = presented_keys(q);
        if keys.is_empty() {
            panic!(
                "case {case_id}: criteria must be an object (choice), an array (score), \
                 or Null (noul)",
            );
        }
        let all_named = keys.iter().all(|key| self.key_map.contains_key(key));
        if all_named {
            for key in &keys {
                let (li, cls) = self.key_map[key];
                self.pos_label.push(li);
                self.pos_class.push(cls);
            }
        } else if !self.context_joined && keys.len() == self.perm.len() {
            for (li, &cls) in self.perm.iter().enumerate() {
                self.pos_label.push(li);
                self.pos_class.push(cls);
            }
        } else {
            let unmatched: Vec<String> = keys
                .iter()
                .filter(|k| !self.key_map.contains_key(*k))
                .cloned()
                .collect();
            if self.context_joined {
                panic!(
                    "case {case_id}: a presented key has no artifact class row — the seat is \
                     context-joined (Plan 003), every key must resolve by name, and the \
                     distractor-universe law covers every train-presented key; \
                     unmatched {unmatched:?} is template drift"
                );
            }
            panic!(
                "case {case_id}: presented options neither all name seat labels nor match the \
                 label count ({}) — the specialist bridge is undefined; \
                 unmatched {unmatched:?}; key_map {} entries; perm {}",
                self.perm.len(),
                self.key_map.len(),
                self.perm.len()
            );
        }
    }

    /// Score the presented positions' classes into `pos_spec` (position
    /// space — A1/H2's pick space). Zero-alloc after warmup.
    fn score_positions(&mut self) {
        let SuiteCtx {
            lane,
            bag,
            pos_spec,
            pos_class,
            ..
        } = self;
        pos_spec.clear();
        pos_spec.resize(pos_class.len(), 0.0);
        lane.scores_classes_into(bag, pos_class, pos_spec);
    }

    /// Gather the label-space NB in-scores into position space (the H2
    /// fusion's vectors must speak the pick space; the margin's rivals
    /// become the presented options — the decision's true alternatives).
    /// A sentinel seat index (the artifact-known, seat-unknown option)
    /// carries NO count-table evidence → NaN, the fusion's no-evidence
    /// mark (the margin term mutes to 0; never a rival).
    fn gather_positions_nb(&mut self) {
        let SuiteCtx {
            in_scores,
            pos_label,
            pos_nb,
            ..
        } = self;
        pos_nb.clear();
        pos_nb.resize(pos_label.len(), 0.0);
        for (o, &li) in pos_nb.iter_mut().zip(pos_label.iter()) {
            *o = if li == usize::MAX {
                f32::NAN
            } else {
                in_scores[li]
            };
        }
    }

    /// A0 over a case set — flattened PER QUESTION (Issue 010: the arena
    /// population now carries multi-question suites, and reflex's own
    /// hard-metrics convention scores every question; for the six
    /// one-question-per-case suites this is byte-identical to the old
    /// per-case rows). Noul answers arrive already in the [no, yes] gold
    /// space (the seat's QuestionOut), so pick == gold is the same
    /// comparison for every kind. Latency stays PER CASE (the seat times
    /// cases) — `total_durs_us.len()` == cases, disclosed via
    /// `n_cases`; `contains_seat_solve` names the scope (Issue 007).
    /// Returns the arm plus the seat eval (H1's raw material).
    /// No receiver: the caller passes the engine explicitly (it lives in
    /// this struct, and `self` + `&mut self.engine` would double-borrow).
    fn eval_a0(
        engine: &mut DecisionEngine<N, EMBED_DIM>,
        cases: &[SuiteCase],
        strs: &[String],
    ) -> Result<(ArmOut, SeatEval), String> {
        let se: SeatEval = eval_seat(engine, cases, strs)?;
        let n_q: usize = cases.iter().map(|c| c.questions.len()).sum();
        let mut a0 = ArmOut {
            name: "A0".into(),
            correct: Vec::with_capacity(n_q),
            picks: Vec::with_capacity(n_q),
            probs: Vec::with_capacity(n_q),
            confs: Vec::with_capacity(n_q),
            escalated: vec![false; n_q],
            abstained: Vec::with_capacity(n_q),
            own_durs_us: se.durs_us.iter().map(|&d| d as f64).collect(),
            total_durs_us: se.durs_us.iter().map(|&d| d as f64).collect(),
            contains_seat_solve: true,
        };
        for (ci, case) in cases.iter().enumerate() {
            for (qi, (_q, qo)) in case.questions.iter().zip(&se.cases[ci]).enumerate() {
                let gold = case.gold[qi].idx;
                a0.picks.push(qo.pick);
                a0.correct.push(qo.pick == gold);
                a0.probs.push(qo.probs.clone());
                a0.confs.push(qo.conf);
                a0.abstained.push(qo.abstained);
            }
        }
        Ok((a0, se))
    }

    /// H1 over a case set from an existing seat eval — the cascade arm
    /// (reflex pass-through + specialist over survivors). Flattened PER
    /// QUESTION (Plan 003: typed_decisions' 5-question cases each yield
    /// per-question rows — the per-question bag is the case's state string
    /// re-bagged, the per-question positions/gold come from that question's
    /// own criteria; for the one-question suites this is byte-identical to
    /// the old per-case loop).
    fn eval_h1(
        &mut self,
        se: &SeatEval,
        cases: &[SuiteCase],
        strs: &[String],
    ) -> ArmOut {
        let n_q: usize = cases.iter().map(|c| c.questions.len()).sum();
        let mut h1 = ArmOut {
            name: "H1".into(),
            correct: Vec::with_capacity(n_q),
            picks: Vec::with_capacity(n_q),
            probs: Vec::with_capacity(n_q),
            confs: Vec::with_capacity(n_q),
            escalated: Vec::with_capacity(n_q),
            abstained: Vec::with_capacity(n_q),
            own_durs_us: Vec::with_capacity(n_q),
            total_durs_us: Vec::with_capacity(n_q),
            contains_seat_solve: true,
        };
        for (ci, case) in cases.iter().enumerate() {
            let state = strs[ci].as_bytes();
            for (qi, (q, qo)) in case.questions.iter().zip(&se.cases[ci]).enumerate() {
                let gold = case.gold[qi].idx;
                let a0_ans = A0Answer {
                    probs: &qo.probs,
                    pick: qo.pick,
                    abstained: qo.abstained,
                };
                self.conv.bag_into(state, &mut self.bag, &mut self.tok);
                self.fill_positions(&case.id, q);
                self.score_positions();
                let t = std::time::Instant::now();
                let d = self.lane.h1_decide(
                    a0_ans,
                    &self.bag,
                    &self.pos_class,
                    &mut self.survivors,
                    &mut self.h1_scores,
                );
                let dt_us = t.elapsed().as_nanos() as f64 / 1000.0;
                h1.picks.push(d.pick);
                h1.correct.push(d.pick == gold);
                h1.probs.push(qo.probs.clone());
                h1.escalated.push(d.escalated);
                // H1 abstains only where reflex abstained AND the gate did
                // not escalate (the passthrough half of the cascade).
                h1.abstained.push(qo.abstained && !d.escalated);
                h1.own_durs_us.push(dt_us);
                h1.total_durs_us.push(dt_us + se.durs_us[ci] as f64);
                // The hybrid's confidence readout: reflex's when it answered,
                // the specialist's winning score when escalated (read OUTSIDE
                // the timed region — a readout, not decision work).
                h1.confs.push(if d.escalated {
                    f64::from(argmax_pos(&self.pos_spec).1)
                } else {
                    qo.conf
                });
            }
        }
        h1
    }

    /// A1 + the H2 grid over a case set (no reflex solve — the specialist
    /// forward + the frozen count tables only). Flattened PER QUESTION
    /// (Plan 003 — the same law as [`Self::eval_h1`]); the state-level NB
    /// read is hoisted per CASE (identical for every question; each
    /// question's own gather maps it into that question's position space).
    fn eval_a1_h2(&mut self, cases: &[SuiteCase], strs: &[String]) -> (ArmOut, Vec<ArmOut>) {
        let n_q: usize = cases.iter().map(|c| c.questions.len()).sum();
        let mut a1 = ArmOut {
            name: "A1".into(),
            correct: Vec::with_capacity(n_q),
            picks: Vec::with_capacity(n_q),
            probs: Vec::with_capacity(n_q),
            confs: Vec::with_capacity(n_q),
            escalated: vec![true; n_q],
            abstained: vec![false; n_q],
            own_durs_us: Vec::with_capacity(n_q),
            total_durs_us: Vec::with_capacity(n_q),
            contains_seat_solve: false,
        };
        let mut grid: Vec<GridRow> = BETA_GRID
            .iter()
            .flat_map(|&b| {
                N_MIN_GRID
                    .iter()
                    .flat_map(move |&nm| TAU_GRID.iter().map(move |&t| Cand::H2 {
                        beta: b,
                        n_min: nm,
                        tau_n: t,
                    }))
            })
            .map(|c| {
                (
                    c,
                    Vec::with_capacity(n_q),
                    Vec::with_capacity(n_q),
                    Vec::with_capacity(n_q),
                    Vec::with_capacity(n_q),
                )
            })
            .collect();
        for (ci, case) in cases.iter().enumerate() {
            let state = strs[ci].as_bytes();
            // The frozen count tables' evidence for this state (read-only;
            // state-level — identical for every question of the case).
            let (n_seen, n_tok): (usize, usize) = if self.nb_armed {
                let tables = self
                    .engine
                    .nb_scope()
                    .expect("nb_armed without tables — the posture lied");
                riir_reflex::nb_scope::view_tokens_into(self.nb_view, state, &mut self.nb_scratch);
                tables.in_scores(&self.nb_scratch, &mut self.in_scores);
                let seen = tables.seen_count(&self.nb_scratch);
                (seen, self.nb_scratch.len())
            } else {
                (0, 0)
            };
            for (qi, q) in case.questions.iter().enumerate() {
                let gold = case.gold[qi].idx;
                self.fill_positions(&case.id, q);
                let t = std::time::Instant::now();
                self.conv.bag_into(state, &mut self.bag, &mut self.tok);
                self.score_positions();
                // A1's pick in PRESENTED-OPTION space — the space gold speaks
                // (the specialist answers the question asked, among the
                // options presented; Issue 006).
                let (pick, conf) = argmax_pos(&self.pos_spec);
                let fwd_us = t.elapsed().as_nanos() as f64 / 1000.0;
                a1.picks.push(pick);
                a1.correct.push(pick == gold);
                a1.probs.push(Vec::new());
                a1.confs.push(f64::from(conf));
                a1.own_durs_us.push(fwd_us);
                a1.total_durs_us.push(fwd_us);

                let inscores: &[f32] = if self.nb_armed {
                    self.gather_positions_nb();
                    &self.pos_nb
                } else {
                    &[]
                };
                let spec_pos: &[f32] = &self.pos_spec;
                for (cand, picks, correct, confs, durs) in grid.iter_mut() {
                    let Cand::H2 { beta, n_min, tau_n } = cand else {
                        unreachable!("grid holds only H2 candidates")
                    };
                    let fusion = PriorFusion {
                        beta: *beta,
                        n_min: *n_min,
                        tau_n: *tau_n,
                    };
                    let f = prior_fusion_pick(&fusion, spec_pos, inscores, n_seen, n_tok);
                    picks.push(f.pick);
                    correct.push(f.pick == gold);
                    confs.push(f.conf);
                    durs.push(fwd_us);
                }
            }
        }
        let arms = grid
            .into_iter()
            .map(|(cand, picks, correct, confs, durs)| ArmOut {
                name: cand.name(),
                correct,
                picks,
                probs: Vec::new(),
                confs,
                escalated: vec![true; n_q],
                abstained: vec![false; n_q],
                own_durs_us: durs.clone(),
                total_durs_us: durs,
                contains_seat_solve: false,
            })
            .collect();
        (a1, arms)
    }
}

fn run_suite_n<const N: usize>(
    name: &str,
    seat: Seat,
    winners_dir: &Path,
    top_k: usize,
) -> Result<SuiteRun, String> {
    // The CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline):
    // head-select + nb-select + oc-select + ridge-select, registry caps,
    // genome off. `ridge_select` is the cal-selected NBSVM-ridge lane
    // (reflex Bench 057) and `oc_select` the cal-selected option-
    // conditioned lane (reflex issue 038 T7b): both run their selection
    // on every suite and DECLINE at the arming bar where the cal slice
    // does not support them — oc declines on every suite whose train
    // rows carry no per-question gold events (baseline posture holds,
    // byte-identical), so typed_decisions is the only suite that arms
    // oc (published: cal-selected — oc@2 on the capped pool, oc@4 at
    // the Issue-052 extended pool), emotion the only one that
    // arms ridge (@8). The 6/6+2 drift pin below holds the seat path to
    // reflex's own run() at these knobs, and the pin's site check holds
    // reflex's rows to the published bench.json — the published posture
    // drift root cause 1 names can no longer reopen silently. (The pin
    // CAUGHT this lane: with oc off the arena read typed_decisions
    // 0.3300 against the published 0.4655 (the capped-pool row; the
    // Issue-052 extension moved the published modelless row to 0.5725
    // — reflex Bench 078) — the oc-armed posture is what published.)
    let knobs = PostureKnobs {
        head_select: true,
        nb_select: true,
        oc_select: true,
        ridge_select: true,
        genome_select: false,
        genome_accept_margin: 0.0,
        cal_select_caps: vec![],
    };
    let posture = fit_posture::<N>(name, &seat, &knobs)?;
    eprintln!(
        "  posture: cap {} · head {:.2} · nb {:.2} · ridge {:.2} · gates {:.3}/{:.3}",
        posture.effective_cap,
        posture.cfg.head_scale,
        posture.cfg.nb_scale,
        posture.cfg.ridge_scale,
        posture.score_threshold,
        posture.distance_threshold
    );
    let (engine, fallbacks) =
        build_seat_engine::<N>(name, &seat, posture.effective_cap, posture.cfg.clone())?;
    if !fallbacks.is_empty() {
        eprintln!(
            "  [issue-039 guard] {} self-doc fallback label(s): {}",
            fallbacks.len(),
            fallbacks.join(", ")
        );
    }

    // The specialist: the sealed winner artifact, joined onto the seat's
    // label order (the bijection pin lives in SpecialistLane::join).
    // Issue 010 T2: a MISSING artifact is not a crash — the suite runs
    // its honest A0/G0-only posture and publishes its measured
    // `a0_stands` row. A PRESENT-but-broken artifact stays fatal (a
    // corrupt seal must never degrade into a quiet A0). The FILE and the
    // BAG CONVENTION both resolve through the winner bridge (Issue 579's
    // one home for the 578 coupling).
    let bridge = riir_instinct::specialist::winner_bridge(name);
    let winner_path = winners_dir.join(
        bridge
            .file
            .map(str::to_string)
            .unwrap_or_else(|| format!("{name}_winner_v1.bin")),
    );
    let spec = match riir_instinct::specialist::load_artifact(&winner_path) {
        Ok(spec) => Some(spec),
        Err(_) if !winner_path.is_file() => {
            eprintln!(
                "  winner: absent — {} (A0/G0-only posture, Issue 010 T2; verdict a0_stands)",
                winner_path.display()
            );
            None
        }
        Err(e) => return Err(e),
    };
    let Some(spec) = spec else {
        return run_suite_a0_only(name, seat, engine, posture, "no specialist artifact (Issue 010 T2)");
    };

    // The specialist bridge's shape law (Plan 003, relaxed from
    // one-non-noul-question-per-case): every case carries ≥1 question and
    // every question resolves through the bridge — noul via the fixed
    // [no, yes] pair (the producer's unified spelling, mapped by NAME
    // through the key map), choice/score via the presented criteria (the
    // two existing rules). A zero-question case is the bridge's undefined
    // shape — refuse loud. (typed_decisions' 5-question mixed sets and
    // prompt_injections' noul singletons both pass; A0 is flattened per
    // question and H1/A1/H2 zip per question.)
    if seat.suite.cases.iter().any(|c| c.questions.is_empty()) {
        return Err(
            "the specialist bridge's shape is ≥1 question per case — a zero-question \
             case is undefined (the assertion keeps a silent shape drift loud)"
                .into(),
        );
    }
    let artifact_labels: Vec<String> = spec.labels.clone();
    eprintln!(
        "  winner: {} ({} labels, BLAKE3 seal verified, {}-bag convention)",
        winner_path.display(),
        spec.labels.len(),
        bridge.convention.name()
    );
    // The seat↔artifact join form (Plan 003's bridge), decided once: a
    // name-joinable suite or the positional noul pair joins Named; a
    // fully disjoint seat joins Context (typed_decisions' workflow names
    // — sentinel class rows, every answer through the presented-option
    // bridge).
    let join_form = riir_instinct::hybrid::seat_join(&seat.labels, &spec.labels);
    use riir_instinct::hybrid::SeatJoin;
    // The context seat's template-coverage gate (Plan 003, found by the
    // 014 read's first attempt): a context-joined suite answers ONLY
    // through by-name resolution, so every presented key of every
    // question — cal AND test — must be an artifact class row. The train
    // side's distractor-universe law covers the TRAIN pool's presented
    // keys; the reflex test templates can present keys the train pool
    // never carried (typed_decisions: the security_incidents family and
    // others — measured 30% of test questions, all four workflows). An
    // unseatable artifact is not a crash and not a fished partial read:
    // the suite falls back to its honest A0-only posture with the gap
    // named (Issue 010 T2's posture, with a sharper reason).
    if let SeatJoin::Context = &join_form {
        let artifact_set: std::collections::HashSet<&str> =
            artifact_labels.iter().map(String::as_str).collect();
        let mut missing: std::collections::BTreeSet<String> = Default::default();
        let mut missing_cases = 0usize;
        let total_cases = seat.suite.cases.len() + seat.cal_cases.len();
        for case in seat.suite.cases.iter().chain(&seat.cal_cases) {
            let mut case_missing = false;
            for q in &case.questions {
                for key in presented_keys(q) {
                    if !artifact_set.contains(key.as_str()) {
                        missing.insert(key);
                        case_missing = true;
                    }
                }
            }
            if case_missing {
                missing_cases += 1;
            }
        }
        if missing_cases > 0 {
            let note = format!(
                "winner present but UNSEATABLE — {missing_cases} of {total_cases} case(s) \
                 present option key(s) with no artifact class row (template drift vs the \
                 train pool; {} distinct key(s), e.g. {:?}); the context seat answers only \
                 by name (Plan 003), so the specialist cannot seat — Issue 010 T2's \
                 A0-only posture with this reason",
                missing.len(),
                missing.iter().take(4).collect::<Vec<_>>()
            );
            eprintln!("  {note}");
            return run_suite_a0_only(name, seat, engine, posture, &note);
        }
    }
    // The seat↔artifact label join (Plan 003's bridge): name-joinable
    // suites pass through; the positional int spelling joins the unified
    // [no, yes] pair by position (the key map below keys off the JOINED
    // labels, so prompt_injections' two noul positions carry real seat
    // indices — live NB evidence); a fully disjoint seat joins as CONTEXT
    // (typed_decisions' workflow names — sentinel class rows, every
    // answer through the presented-option bridge).
    let context_joined;
    let joined = match join_form {
        SeatJoin::Named(labels) => {
            context_joined = false;
            SpecialistLane::join(spec, name, &labels, Cascade { top_k })?
        }
        SeatJoin::Context => {
            context_joined = true;
            SpecialistLane::join_context(spec, name, &seat.labels, Cascade { top_k })?
        }
    };
    // The presented-option bridge (SuiteCtx::key_map): every JOINED seat
    // label → (its own index, its artifact class row); a context seat
    // contributes NO seat-label entries (their class rows are sentinels —
    // nothing presents them), the artifact's own labels join at the
    // sentinel seat index (NaN NB evidence; the specialist still scores
    // the class row it trained).
    let mut key_map: HashMap<String, (usize, usize)> = if context_joined {
        HashMap::new()
    } else {
        seat.labels
            .iter()
            .enumerate()
            .map(|(li, l)| (l.clone(), (li, joined.perm[li])))
            .collect()
    };
    // Artifact-known labels the SEAT never offers (the t20k massive test
    // split carries 59 of the artifact's 60 intents): legitimate presented
    // options on the train-derived cal front (and at serving time). The
    // seat-label sentinel usize::MAX marks them — the NB gather reads NaN
    // (no evidence) for those positions, and the specialist still scores
    // the class row (the artifact trained it).
    for (ci, l) in artifact_labels.iter().enumerate() {
        key_map.entry(l.clone()).or_insert((usize::MAX, ci));
    }
    let perm: Vec<usize> = joined.perm.clone();
    let lane = HybridLane::Specialist(joined);
    let nb_armed = posture.cfg.nb_scale > 0.0 && engine.nb_scope().is_some();

    let mut ctx = SuiteCtx::<N> {
        engine,
        lane,
        conv: bridge.convention,
        nb_view: posture.cfg.nb_view,
        nb_armed,
        bag: Vec::new(),
        tok: Vec::new(),
        survivors: [(0usize, 0.0f64); MAX_TOP_K],
        h1_scores: [0.0f32; MAX_TOP_K],
        in_scores: vec![0.0; N],
        nb_scratch: Vec::new(),
        key_map,
        context_joined,
        perm,
        pos_label: Vec::new(),
        pos_class: Vec::new(),
        pos_spec: Vec::new(),
        pos_nb: Vec::new(),
    };

    // ── CAL phase (train-side): the instrument's readings ────────────
    let (a0_cal, cal_se) = SuiteCtx::<N>::eval_a0(&mut ctx.engine, &seat.cal_cases, &seat.cal_state_strs)?;
    let h1_cal = ctx.eval_h1(&cal_se, &seat.cal_cases, &seat.cal_state_strs);
    let (a1_cal, h2s_cal) = ctx.eval_a1_h2(&seat.cal_cases, &seat.cal_state_strs);
    let mut cal_arms: Vec<ArmOut> = Vec::with_capacity(3 + h2s_cal.len());
    cal_arms.push(a0_cal);
    cal_arms.push(a1_cal);
    cal_arms.push(h1_cal);
    cal_arms.extend(h2s_cal);

    // ── the pre-registration instrument (train/cal ONLY) ─────────────
    // Pareto rank-0 over (accuracy LCB ↑, consult ↓, p99 ↓) — the axes
    // minimized as (−LCB, consult, p99) — then argmax Beta-LCB.
    let cands: Vec<Cand> = {
        let mut v = vec![Cand::A0, Cand::A1, Cand::H1];
        v.extend(
            BETA_GRID
                .iter()
                .flat_map(|&b| {
                    N_MIN_GRID
                        .iter()
                        .flat_map(move |&nm| TAU_GRID.iter().map(move |&t| Cand::H2 {
                            beta: b,
                            n_min: nm,
                            tau_n: t,
                        }))
                })
                .collect::<Vec<_>>(),
        );
        v
    };
    let mut registration: Vec<RegRow> = Vec::with_capacity(cands.len());
    for (cand, arm) in cands.iter().zip(cal_arms.iter()) {
        let lcb = katgpt_core::best_belief_score(arm.n_correct(), (arm.correct.len() as u32) - arm.n_correct(), 0.05);
        registration.push(RegRow {
            cand: cand.clone(),
            arm: arm.name.clone(),
            cal_acc: arm.accuracy(),
            acc_lcb: f64::from(lcb),
            consult: arm.consult_rate(),
            p99_us: arm.p99(),
            rank0: false,
        });
    }
    let stats: Vec<ArmStat> = registration
        .iter()
        .map(|r| ArmStat {
            neg_acc_lcb: -r.acc_lcb,
            cost_rate: r.consult,
            p99_us: r.p99_us,
        })
        .collect();
    let rank0 = pareto_rank0(&stats);
    for (i, r) in registration.iter_mut().enumerate() {
        r.rank0 = rank0.contains(&i);
    }
    let successes: Vec<u32> = rank0
        .iter()
        .map(|&i| cal_arms[i].n_correct())
        .collect();
    let failures: Vec<u32> = rank0
        .iter()
        .map(|&i| (cal_arms[i].correct.len() as u32) - cal_arms[i].n_correct())
        .collect();
    let win_pos = select_arm(&rank0, &successes, &failures);
    // select_arm returns the CANDIDATE index (the rank-0 element), never
    // a position within the rank-0 set — the massive run's rank0 [0, 2,
    // 16] made the old rank0_sorted[win_pos] re-index OOB (Issue 006's
    // position-vs-index class, one level up in the instrument). rank0 is
    // already in ascending candidate order (pareto_rank0 keeps index
    // order), so argmax ties resolve identically with or without the
    // sort that used to sit here.
    let instrument_pick = cands[win_pos].clone();
    eprintln!(
        "  instrument: rank-0 {}/{} · registered {}",
        rank0.len(),
        registration.len(),
        instrument_pick.name()
    );

    // ── THE TEST READ (once; predictions frozen below) ─────────────
    let mut test_arms: Vec<ArmOut> = Vec::new();
    {
        let (a0, test_se) = SuiteCtx::<N>::eval_a0(&mut ctx.engine, &seat.suite.cases, &seat.state_strs)?;
        let h1 = ctx.eval_h1(&test_se, &seat.suite.cases, &seat.state_strs);
        let (a1, h2s) = ctx.eval_a1_h2(&seat.suite.cases, &seat.state_strs);
        test_arms.push(a0);
        test_arms.push(a1);
        test_arms.push(h1);
        test_arms.extend(h2s);
    }
    let test_acc = |c: &Cand| -> f64 {
        test_arms
            .iter()
            .find(|a| a.name == c.name())
            .map(ArmOut::accuracy)
            .unwrap_or(f64::NAN)
    };
    eprintln!(
        "  test: A0 {:.4} · A1 {:.4} · H1 {:.4} · pick {} → {:.4}",
        test_acc(&Cand::A0),
        test_acc(&Cand::A1),
        test_acc(&Cand::H1),
        instrument_pick.name(),
        test_acc(&instrument_pick)
    );

    // ── the product gate (Issue 008 T2): STRICTLY above the current ────
    // Reflex row, or the registration refuses. Reflex is free; a tie (or
    // an edge the paired data cannot certify) sells nothing. The paired
    // superiority bound (stats::PairedDiff::lb95) reads the SAME frozen
    // test read the predictions freeze — no extra read is spent.
    let mut registered = instrument_pick.clone();
    let superiority = if instrument_pick != Cand::A0 {
        let face = (|| {
            let reg = test_arms.iter().find(|a| a.name == instrument_pick.name())?;
            let a0 = test_arms.iter().find(|a| a.name == "A0")?;
            paired_upper_bound(&reg.correct, &a0.correct)
        })();
        let Some(pd) = face else {
            die(&format!(
                "{}: the superiority gate could not pair the pick with A0 \
                 on the test read — the arm record is missing",
                name
            ));
        };
        let passed = pd.lb95 > 0.0;
        eprintln!(
            "  T2 superiority: {} − A0 mean {:+.4} · LB95 {:+.4} > 0 → {}",
            instrument_pick.name(),
            pd.mean,
            pd.lb95,
            if passed { "PASS" } else { "REFUSED → A0 serves" }
        );
        if !passed {
            registered = Cand::A0;
        }
        Some(SuperiorityFace {
            pick: instrument_pick.clone(),
            mean: pd.mean,
            lb95: pd.lb95,
            passed,
        })
    } else {
        None
    };

    // ── the fusion-overhead micro (G2's <100 ns bar) ─────────────
    let (h1_ns, h1_fusion_ns, h2_ns_opt) = fusion_overhead_micro(&mut ctx, N);
    eprintln!(
        "  fusion overhead: H1 full decision {:.0} ns/q (fusion-only {:.0} ns/q) · H2 fusion {:.2} ns/option",
        h1_ns, h1_fusion_ns, h2_ns_opt
    );

    // ── the gate faces (judged on the INSTRUMENT pick — a refused arm's
    // regression/calibration disclosures survive the refusal) ──────────
    let g1 = g1_face(&cal_arms, &test_arms, &instrument_pick);
    let (g3, g3_delta) = if REFLEX_WON.contains(&name) {
        g3_face(&cal_arms, &test_arms, &instrument_pick)
    } else {
        (None, None)
    };
    let g5 = if GAP_SUITES.contains(&name) {
        Some(g5_face(&test_arms, &instrument_pick))
    } else {
        None
    };
    if let Some(g) = &g1 {
        eprintln!(
            "  G1: raw {:.4} · platt {:.4} · floor {:.4} → {}",
            g.ece_raw,
            g.ece_platt,
            g.ece_floor,
            if g.pass { "PASS" } else { "FAIL" }
        );
    }
    if let (Some(pd), Some(delta)) = (&g3, g3_delta) {
        eprintln!(
            "  G3: mean {:+.4} · UB95 {:+.4} ≤ δ {:.4} → {}",
            pd.mean,
            pd.ub95,
            delta,
            if pd.ub95 <= delta { "PASS" } else { "FAIL" }
        );
    }
    if let Some((pass, why)) = &g5 {
        eprintln!("  G5: {why} → {}", if *pass { "PASS" } else { "FAIL" });
    }

    // ── the laya paired face (behind arena-laya) ─────────────────────
    let laya = laya_face(&seat, &test_arms);

    Ok(SuiteRun {
        suite: name.to_string(),
        n_questions: test_arms
            .iter()
            .find(|a| a.name == "A0")
            .map(|a| a.correct.len())
            .unwrap_or(seat.suite.cases.len()),
        n_cases: seat.suite.cases.len(),
        specialist_present: true,
        verdict: if registered != Cand::A0 { "hybrid_arm" } else { "a0_stands" },
        a0_note: if registered == Cand::A0 {
            Some(match &superiority {
                Some(s) => format!(
                    "instrument pick {} refused — paired LB95 {:+.4} (Issue 008 T2)",
                    s.pick.name(),
                    s.lb95
                ),
                None => "the instrument registered A0 outright".to_string(),
            })
        } else {
            None
        },
        posture: DisclosedPosture {
            effective_cap: posture.effective_cap,
            head_scale: posture.cfg.head_scale,
            nb_scale: posture.cfg.nb_scale,
            ridge_scale: posture.cfg.ridge_scale,
            nb_view: match posture.cfg.nb_view {
                NbView::Bag => "bag",
                NbView::Pair => "pair",
            },
            score_threshold: posture.score_threshold,
            distance_threshold: posture.distance_threshold,
            bag_convention: bridge.convention.name(),
        },
        registration,
        registered,
        superiority,
        test_arms,
        h1_ns_decision: h1_ns,
        h1_ns_fusion: h1_fusion_ns,
        h2_ns_per_option: h2_ns_opt,
        g1,
        g3,
        g3_delta,
        g5,
        laya,
    })
}

/// The A0/G0-only posture (Issue 010 T2): a suite with no winner artifact
/// still gets its MEASURED row — seated, posture-fitted, single frozen
/// test read — with the verdict `a0_stands`. No specialist exists, so no
/// A1/H1/H2 arms are constructed (the registration is [A0] alone, the
/// instrument pick is A0 by construction, and the product gate has
/// nothing to refuse). G1 still runs — reflex's own confidence readout
/// on this suite is a real disclosure, gated by nothing. The hybrid-arm
/// faces (G3/G5/laya/fusion) are None: the verdict names the posture.
fn run_suite_a0_only<const N: usize>(
    name: &str,
    seat: Seat,
    mut engine: DecisionEngine<N, EMBED_DIM>,
    posture: SeatPosture,
    reason: &str,
) -> Result<SuiteRun, String> {
    // ── CAL phase: A0 only ───────────────────────────────────
    let (a0_cal, _) = SuiteCtx::<N>::eval_a0(&mut engine, &seat.cal_cases, &seat.cal_state_strs)?;

    // ── the test read (once) ─────────────────────────────
    let (a0, _) = SuiteCtx::<N>::eval_a0(&mut engine, &seat.suite.cases, &seat.state_strs)?;
    eprintln!(
        "  test: A0 {:.4} · no hybrid arms — verdict a0_stands (Issue 010 T2)",
        a0.accuracy()
    );

    let g1 = g1_face(
        std::slice::from_ref(&a0_cal),
        std::slice::from_ref(&a0),
        &Cand::A0,
    );
    if let Some(g) = &g1 {
        eprintln!(
            "  G1 (A0 disclosure): raw {:.4} · platt {:.4} · floor {:.4} → {}",
            g.ece_raw,
            g.ece_platt,
            g.ece_floor,
            if g.pass { "PASS" } else { "FAIL" }
        );
    }

    let lcb = katgpt_core::best_belief_score(
        a0_cal.n_correct(),
        (a0_cal.correct.len() as u32) - a0_cal.n_correct(),
        0.05,
    );
    let registration = vec![RegRow {
        cand: Cand::A0,
        arm: "A0".to_string(),
        cal_acc: a0_cal.accuracy(),
        acc_lcb: f64::from(lcb),
        consult: a0_cal.consult_rate(),
        p99_us: a0_cal.p99(),
        rank0: true,
    }];

    Ok(SuiteRun {
        suite: name.to_string(),
        n_questions: a0.correct.len(),
        n_cases: seat.suite.cases.len(),
        specialist_present: false,
        verdict: "a0_stands",
        a0_note: Some(reason.to_string()),
        posture: DisclosedPosture {
            effective_cap: posture.effective_cap,
            head_scale: posture.cfg.head_scale,
            nb_scale: posture.cfg.nb_scale,
            ridge_scale: posture.cfg.ridge_scale,
            nb_view: match posture.cfg.nb_view {
                NbView::Bag => "bag",
                NbView::Pair => "pair",
            },
            score_threshold: posture.score_threshold,
            distance_threshold: posture.distance_threshold,
            bag_convention: riir_instinct::specialist::BagConvention::Count.name(),
        },
        registration,
        registered: Cand::A0,
        superiority: None,
        test_arms: vec![a0],
        h1_ns_decision: 0.0,
        h1_ns_fusion: 0.0,
        h2_ns_per_option: 0.0,
        g1,
        g3: None,
        g3_delta: None,
        g5: None,
        laya: None,
    })
}

/// The fusion-overhead micro (G2): tight loops, ONE timer pair, results
/// consumed through a checksum the optimizer cannot fold away.
/// H1 = gate + prune + specialist-on-survivors per QUESTION; H2 = the
/// fusion arithmetic per QUESTION (divided by the option count for the
/// per-option bar).
fn fusion_overhead_micro<const N: usize>(
    ctx: &mut SuiteCtx<N>,
    n: usize,
) -> (f64, f64, f64) {
    /// (h1 full decision, h1 fusion-only, h2 fusion per option)
    const ITERS: usize = 20_000;
    let probs: Vec<f64> = (0..n).map(|i| (i as f64 * 0.61).cos().abs()).collect();
    let class_of_pos: Vec<usize> = (0..n).collect();
    ctx.conv
        .bag_into(b"alpha beta gamma", &mut ctx.bag, &mut ctx.tok);
    let a0 = A0Answer {
        probs: &probs,
        pick: 0,
        abstained: true,
    };
    // Warm (the loop's liveness is guaranteed by black_box on the
    // consumed picks — a value assert here would be wrong: the picks can
    // legitimately all be label 0).
    let mut checksum = 0usize;
    for _ in 0..100 {
        let d = ctx.lane.h1_decide(
            a0,
            &ctx.bag,
            &class_of_pos,
            &mut ctx.survivors,
            &mut ctx.h1_scores,
        );
        checksum += d.pick;
    }
    let t = std::time::Instant::now();
    for _ in 0..ITERS {
        let d = ctx.lane.h1_decide(
            a0,
            &ctx.bag,
            &class_of_pos,
            &mut ctx.survivors,
            &mut ctx.h1_scores,
        );
        checksum += black_box(d.pick);
    }
    let h1_ns = t.elapsed().as_nanos() as f64 / ITERS as f64;

    // The FUSION-ONLY micro: the H1 work that is neither the reflex solve
    // nor the specialist's scoring — the gate check, the prune, and the
    // survivor argmax over GIVEN scores. The 100 ns bar in the gate is
    // this fusion term (the specialist's scoring is the escalation's own
    // budget, compared against laya in the latency face).
    let h1_fusion_ns = {
        let probs_f = &probs;
        // Warm.
        for _ in 0..100 {
            let mut s = [(0usize, 0.0f64); MAX_TOP_K];
            let kept = Cascade { top_k: 8 }.prune(probs_f, &mut s);
            let mut best = 0usize;
            for i in 1..kept.len() {
                if kept[i].1 > kept[best].1 {
                    best = i;
                }
            }
            checksum = checksum.wrapping_add(black_box(kept[best].0));
        }
        let t = std::time::Instant::now();
        for _ in 0..ITERS {
            let mut s = [(0usize, 0.0f64); MAX_TOP_K];
            let kept = Cascade { top_k: 8 }.prune(probs_f, &mut s);
            let mut best = 0usize;
            for i in 1..kept.len() {
                if kept[i].1 > kept[best].1 {
                    best = i;
                }
            }
            checksum = checksum.wrapping_add(black_box(kept[best].0));
        }
        t.elapsed().as_nanos() as f64 / ITERS as f64
    };

    let fusion = PriorFusion {
        beta: 1.0,
        n_min: 4.0,
        tau_n: 4.0,
    };
    let mut spec_scores = vec![0.0f32; n];
    ctx.lane
        .scores_label_into(&ctx.bag, &mut spec_scores);
    // H2's per-question cost = the fusion arithmetic over the scores
    // (the specialist forward + nb read are measured separately in the
    // arm latencies; the BAR in the gate is the fusion term).
    let t = std::time::Instant::now();
    for _ in 0..ITERS {
        let f = prior_fusion_pick(&fusion, &spec_scores, &[], 99, 5);
        checksum += black_box(f.pick);
    }
    let h2_ns = t.elapsed().as_nanos() as f64 / ITERS as f64;
    black_box(checksum);
    (h1_ns, h1_fusion_ns, h2_ns / n as f64)
}

/// Argmax over f32 scores, ties to the LOWEST index — the engine argmax
/// law. Returns (position, score).
fn argmax_pos(scores: &[f32]) -> (usize, f32) {
    let mut best = 0usize;
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > scores[best] {
            best = i;
        }
    }
    (best, scores[best])
}

/// G1 — the registered arm's confidence readout on TEST: uncalibrated vs
/// Platt (the sigmoid-gate calibrator fit on the arm's CAL pairs) vs the
/// conformal-naive floor. Pass = beats BOTH, same metric (ECE) all sides.
fn g1_face(cal: &[ArmOut], test: &[ArmOut], registered: &Cand) -> Option<G1Face> {
    let name = registered.name();
    let cal_arm = cal.iter().find(|a| a.name == name)?;
    let test_arm = test.iter().find(|a| a.name == name)?;
    // Platt: fit on CAL, apply to TEST (never in-sample). The fit is an
    // explicit refit() — observe() only fills the evidence ring (the
    // apply-is-identity-until-refit contract; the arena's first run read
    // Platt == raw to 4 decimals because refit() was never called).
    let mut platt = katgpt_core::sigmoid_calibration::SigmoidGateCalibrator::new(512, 64);
    for (&conf, &ok) in cal_arm.confs.iter().zip(&cal_arm.correct) {
        platt.observe(conf as f32, ok);
    }
    let platt_moved = platt.refit();
    let raw_pairs: Vec<(f64, bool)> = test_arm
        .confs
        .iter()
        .zip(&test_arm.correct)
        .map(|(&c, &ok)| (c, ok))
        .collect();
    let ece_raw = ece_of(&raw_pairs);
    let platt_pairs: Vec<(f64, bool)> = test_arm
        .confs
        .iter()
        .zip(&test_arm.correct)
        .map(|(&c, &ok)| (f64::from(platt.apply(c as f32)), ok))
        .collect();
    let ece_platt = ece_of(&platt_pairs);
    let cal_pairs: Vec<CalibrationPair> = cal_arm
        .confs
        .iter()
        .zip(&cal_arm.correct)
        .map(|(&c, &ok)| CalibrationPair {
            conf: c,
            correct: ok,
        })
        .collect();
    let floored = conformal_naive_floor(&cal_pairs, &test_arm.confs);
    let floor_pairs: Vec<(f64, bool)> = floored
        .into_iter()
        .zip(test_arm.correct.iter().copied())
        .collect();
    let ece_floor = ece_of(&floor_pairs);
    Some(G1Face {
        ece_raw,
        ece_platt,
        ece_floor,
        platt_moved,
        pass: platt_moved && ece_platt < ece_raw && ece_platt < ece_floor,
    })
}

/// G3 — paired non-inferiority on a reflex-won suite: A0 − registered,
/// δ pre-declared from the CAL discordant rate at the TEST n
/// (δ = max(1.0 pp, 2.5·SE)).
fn g3_face(cal: &[ArmOut], test: &[ArmOut], registered: &Cand) -> (Option<PairedDiff>, Option<f64>) {
    let name = registered.name();
    let Some(cal_reg) = cal.iter().find(|a| a.name == name) else {
        return (None, None);
    };
    let Some(test_reg) = test.iter().find(|a| a.name == name) else {
        return (None, None);
    };
    let Some(a0_cal) = cal.iter().find(|a| a.name == "A0") else {
        return (None, None);
    };
    let Some(a0_test) = test.iter().find(|a| a.name == "A0") else {
        return (None, None);
    };
    let n_discordant_cal = a0_cal
        .correct
        .iter()
        .zip(&cal_reg.correct)
        .filter(|(x, y)| x != y)
        .count();
    let p_disc = if a0_cal.correct.is_empty() {
        0.0
    } else {
        n_discordant_cal as f64 / a0_cal.correct.len() as f64
    };
    let delta = delta_suite(p_disc, test_reg.correct.len());
    let pd = paired_upper_bound(&a0_test.correct, &test_reg.correct);
    (pd, Some(delta))
}

/// G5 — the registered arm on a gap suite: quality face (Wilson LB >
/// max(A0, A1) point accuracy) OR the budget face (A1 − arm UB95 ≤ δ).
fn g5_face(test: &[ArmOut], registered: &Cand) -> (bool, String) {
    let name = registered.name();
    if matches!(registered, Cand::A0 | Cand::A1) {
        return (
            false,
            format!(
                "no hybrid arm registered ({name} stands) — the gate is refused, never a pass"
            ),
        );
    }
    let Some(reg) = test.iter().find(|a| a.name == name) else {
        return (false, "registered arm missing from the test read".into());
    };
    let Some(a0) = test.iter().find(|a| a.name == "A0") else {
        return (false, "A0 missing".into());
    };
    let Some(a1) = test.iter().find(|a| a.name == "A1") else {
        return (false, "A1 missing".into());
    };
    let n = reg.correct.len();
    let lb = wilson_lb(reg.n_correct() as usize, n);
    let best_other = a0.accuracy().max(a1.accuracy());
    if lb > best_other {
        return (
            true,
            format!(
                "quality face: Wilson LB {lb:.4} > max(A0, A1) {best_other:.4}"
            ),
        );
    }
    // The budget face: A1 − arm (the latency the hybrid saves over the
    // always-specialist posture must not cost accuracy beyond δ).
    let pd = paired_upper_bound(&a1.correct, &reg.correct);
    let delta = delta_suite(0.02, n); // the conservative default discordance
    match pd {
        Some(pd) if pd.ub95 <= delta => (
            true,
            format!(
                "budget face: A1−arm UB95 {:+.4} ≤ δ {:.4} (quality face missed: LB {lb:.4} vs {best_other:.4})",
                pd.ub95, delta
            ),
        ),
        Some(pd) => (
            false,
            format!(
                "both faces missed: LB {lb:.4} vs {best_other:.4}; A1−arm UB95 {:+.4} > δ {:.4}",
                pd.ub95, delta
            ),
        ),
        None => (false, "paired budget face unavailable".into()),
    }
}

/// The laya paired face (cfg arena-laya): per-escalated-question latency,
/// laya − lane, 95% UB ≤ 0 = the lane is faster with 95% confidence.
#[cfg(feature = "arena-laya")]
fn laya_face(seat: &Seat, test_arms: &[ArmOut]) -> Option<LayaFace> {
    use riir_instinct::stats::paired_upper_bound_f64;
    let h1 = test_arms.iter().find(|a| a.name == "H1")?;
    let escalated: Vec<usize> = h1
        .escalated
        .iter()
        .enumerate()
        .filter(|(_, &e)| e)
        .map(|(i, _)| i)
        .collect();
    if escalated.is_empty() {
        return None;
    }
    let cases: Vec<SuiteCase> = escalated.iter().map(|&i| seat.suite.cases[i].clone()).collect();
    let durs = riir_reflex::harness::runner::seat::laya_escalation_latency_us(&cases).ok()?;
    let lane_durs: Vec<f64> = escalated.iter().map(|&i| h1.own_durs_us[i]).collect();
    let laya_durs: Vec<f64> = durs.iter().map(|&d| d as f64).collect();
    // The non-inferiority form: the violation direction is lane − laya
    // (positive = the lane SLOWER = bad); the gate passes when its 95%
    // upper bound ≤ 0 — i.e. the lane is faster with 95% confidence.
    // (The first recording inverted this: ub95(laya − lane) ≤ 0 fails
    // exactly when the lane is much faster — the data was favorable, the
    // sign was not.)
    let diffs: Vec<f64> = lane_durs
        .iter()
        .zip(&laya_durs)
        .map(|(m, l)| m - l)
        .collect();
    let pd = paired_upper_bound_f64(&diffs)?;
    Some(LayaFace {
        escalated_n: escalated.len(),
        lane_p50_us: pct(&lane_durs, 0.5),
        laya_p50_us: pct(&laya_durs, 0.5),
        paired_ub95_us: pd.ub95,
        pass: pd.ub95 <= 0.0,
    })
}

#[cfg(not(feature = "arena-laya"))]
fn laya_face(_seat: &Seat, _test_arms: &[ArmOut]) -> Option<LayaFace> {
    None
}

/// The A0 drift pin: reflex's own `run()` over xnli_en at the same
/// posture must reproduce the arena's A0 test accuracy exactly (the same
/// engine, the same questions — a divergence is a seat-path defect).
fn pin_a0_identity(datasets_dir: &Path, runs: &[SuiteRun], suites: &[&str]) -> Result<(), String> {
    let opts = riir_reflex::harness::runner::RunOptions {
        datasets_dir: datasets_dir.to_path_buf(),
        suites: suites.iter().map(|s| (*s).to_string()).collect(),
        laya_max_questions: 0,
        skip_laya: true,
        // the cascade lane (sibling reflex Issue 038 T4′ / 042 lever 3) —
        // off: the drift pin reproduces the plain modelless lane.
        cascade: false,
        cascade_worthiness: false,
        cascade_worthiness_margin: 0.0,
        // the LCB-armed worthiness probe (reflex issue 042 lever 3's
        // support extension) — cascade-lane-only, None = off.
        cascade_worthiness_lcb: None,
        // Issue 042's gate rate-axis levers — off: the plain T1.6
        // cal-slice fused fit is the posture the pin reproduces.
        gate_fit_selection: false,
        gate_distance_only: false,
        laya_python: false,
        gliner: false,
        agentjev: false,
        // The OpenThai comparison lane (reflex Plan 003 Phase 2) — off:
        // their served model is a comparison lane, never the pin's
        // subject.
        openthai: false,
        paw: false,
        corpus_cap_override: 0,
        cal_select_caps: vec![],
        pair_head_ab: false,
        head_scale: 0.0,
        head_select: true,
        nb_select: true,
        clm: false,
        paw_local: false,
        // The Issue-008 T1 re-baseline posture: the cal-selected NBSVM
        // ridge (reflex Bench 057) AND the cal-selected option-
        // conditioned lane (reflex issue 038 T7b). Both selections run
        // per suite and decline at the arming bar / on no gold events —
        // ridge arms only on emotion, oc only on typed_decisions, each
        // byte-identical to off elsewhere. (Measured 2026-09-27: with
        // oc off the pin read typed_decisions 0.3300 vs the then-
        // published 0.4655 — reflex Bench 078 moved that row to 0.5725
        // at the Issue-052 corpus extension.)
        ridge_select: true,
        oc_select: true,
        // Issue 038 T5's joint genome walk — off: reflex's published
        // bench rows predate the genome lane (the site rows are the
        // 052/057 postures); turning it on here would be a NEW posture
        // no published row carries.
        genome_select: false,
        genome_accept_margin: 0.0,
        // reflex's issue-044 probe arm — measurement-only upstream, and
        // never part of the published posture the pin reproduces.
        nli_feature_ab: false,
        // reflex's Issue 047 M1 reopen lane (xnli pick/confidence
        // separation) — measurement-only upstream, never the pin's
        // posture. (Field landed by the sibling's in-flight tree.)
        nli_m1: false,
    };
    let (out, errors) = riir_reflex::harness::runner::run(&opts)?;
    if !errors.is_empty() {
        return Err(format!("reflex run() reported errors: {errors:?}"));
    }

    // The site check: when the reflex-site checkout stands beside this
    // workspace, the runner's rows must equal the PUBLISHED bench.json
    // numbers — that is the "A0 == the published modelless row" half of
    // the re-baseline (Issue 008 T1). Absent checkout → a loud skip
    // (a deferral, never a green).
    let site_rows = site_published_rows(datasets_dir, suites)?;

    let mut mismatches = Vec::new();
    for run in runs {
        let sr = out
            .suites
            .iter()
            .find(|s| s.name == run.suite)
            .ok_or_else(|| format!("reflex run() produced no {} row", run.suite))?;
        let published = sr
            .modelless
            .as_ref()
            .ok_or_else(|| format!("reflex run() produced no modelless lane for {}", run.suite))?
            .hard
            .accuracy;
        let arena = run
            .test_arms
            .iter()
            .find(|a| a.name == "A0")
            .ok_or_else(|| format!("arena {} has no A0 arm", run.suite))?
            .accuracy();
        if published != arena {
            mismatches.push(format!(
                "{}: reflex run() {published:.6} != arena A0 {arena:.6} — the seat path diverged",
                run.suite
            ));
            continue;
        }
        let site_note = match site_rows.as_ref() {
            Some(rows) => match rows.get(&run.suite) {
                Some(&site) if site == published => " · site ✓".to_string(),
                Some(&site) => {
                    mismatches.push(format!(
                        "{}: reflex run() {published:.6} != PUBLISHED site row {site:.6} — \
                         reflex's published posture moved past this arena's knobs; \
                         re-baseline the knobs (Issue 008 T1)",
                        run.suite
                    ));
                    continue;
                }
                None => " · site row absent".to_string(),
            },
            None => " · site json absent (skip loud)".to_string(),
        };
        eprintln!(
            "  A0 pin · {}: arena == reflex run() == {published:.6}{}",
            run.suite, site_note
        );
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(mismatches.join("; "))
    }
}

/// The published bench.json's per-suite modelless hard accuracies, read
/// from the reflex-site checkout that stands beside the reflex datasets
/// (`<datasets>/../../../reflex-site/data/bench.json`). Only the WANTED
/// suites are extracted — the site carries non-arena rows whose hard
/// accuracy is legitimately absent (the synthetic families), and a row
/// the pin never reads must not fail it. `Ok(None)` when the checkout
/// is absent — the caller discloses the skip, never reads it as a pass.
fn site_published_rows(
    datasets_dir: &Path,
    wanted: &[&str],
) -> Result<Option<std::collections::HashMap<String, f64>>, String> {
    let Some(root) = datasets_dir.canonicalize().ok().and_then(|p| {
        p.ancestors().nth(3).map(|a| a.to_path_buf())
    }) else {
        eprintln!(
            "  A0 pin · site check skipped loud: the datasets dir does not resolve \
             to a workspace root"
        );
        return Ok(None);
    };
    let path = root.join("reflex-site/data/bench.json");
    if !path.is_file() {
        eprintln!(
            "  A0 pin · site check skipped loud: {} is absent (no reflex-site checkout) — \
             the published-row half of the pin is UNVERIFIED this run",
            path.display()
        );
        return Ok(None);
    }
    let doc: serde_json::Value = serde_json::from_reader(
        std::fs::File::open(&path).map_err(|e| format!("site bench.json unreadable: {e}"))?,
    )
    .map_err(|e| format!("site bench.json unparsable: {e}"))?;
    let suites = doc["suites"]
        .as_array()
        .ok_or("site bench.json: no suites array")?;
    let mut rows = std::collections::HashMap::new();
    for s in suites {
        let name = s["name"]
            .as_str()
            .ok_or("site bench.json: suite row without a name")?;
        if !wanted.contains(&name) {
            continue;
        }
        let acc = s["modelless"]["hard"]["accuracy"]
            .as_f64()
            .ok_or_else(|| format!("site bench.json: {name} has no modelless hard accuracy"))?;
        rows.insert(name.to_string(), acc);
    }
    Ok(Some(rows))
}

fn write_predictions(out_dir: &Path, runs: &[SuiteRun]) {
    // The freeze contract is the REGISTERED (served) arm + the controls
    // (A0/A1/H1) — the 45-point H2 grid is recomputable deterministically
    // from the seat + posture + cal front, and freezing it cost 14 MB in
    // git history (Bench 001's first landing; trimmed same-day, the arms
    // proven byte-identical across runs 3 and 4 before the trim).
    let mut suites = Vec::new();
    for run in runs {
        let arms: Vec<serde_json::Value> = run
            .test_arms
            .iter()
            .filter(|a| {
                a.name == "A0"
                    || a.name == "A1"
                    || a.name == "H1"
                    || a.name == run.registered.name()
                    || run
                        .superiority
                        .as_ref()
                        .is_some_and(|s| !s.passed && a.name == s.pick.name())
            })
            .map(|a| {
                serde_json::json!({
                    "name": a.name,
                    "accuracy": a.accuracy(),
                    "picks": a.picks,
                    "abstained": a.abstained,
                    "correct": a.correct,
                    "confs": a.confs,
                    "escalated": a.escalated,
                    "own_durs_us": a.own_durs_us,
                    "total_durs_us": a.total_durs_us,
                    "contains_seat_solve": a.contains_seat_solve,
                })
            })
            .collect();
        suites.push(serde_json::json!({
            "suite": run.suite,
            "n_questions": run.n_questions,
            "n_cases": run.n_cases,
            "specialist_present": run.specialist_present,
            "verdict": run.verdict,
            "a0_note": run.a0_note,
            "registered": run.registered.name(),
            "instrument_pick": run.superiority.as_ref().map(|s| s.pick.name()),
            "superiority": run.superiority.as_ref().map(|s| serde_json::json!({
                "pick": s.pick.name(),
                "mean": s.mean,
                "lb95": s.lb95,
                "passed": s.passed,
            })),
            "arms": arms,
        }));
    }
    let doc = serde_json::json!({ "frozen_test_predictions": suites });
    let path = out_dir.join("predictions.json");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&doc).expect("predictions json"),
    )
    .expect("write predictions");
    eprintln!("frozen predictions: {}", path.display());

    // The full pre-registration table (the RESULTS.md text has promised
    // this file since Bench 001; write it for real).
    let regs: serde_json::Value = serde_json::to_value(
        runs.iter()
            .map(|r| {
                serde_json::json!({
                    "suite": r.suite,
                    "registered": r.registered.name(),
                    "rows": r.registration.iter().map(|row| serde_json::json!({
                        "arm": row.arm,
                        "cal_acc": row.cal_acc,
                        "acc_lcb": row.acc_lcb,
                        "consult": row.consult,
                        "p99_us": row.p99_us,
                        "rank0": row.rank0,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect::<Vec<_>>(),
    )
    .expect("registration json");
    let rpath = out_dir.join("registration.json");
    std::fs::write(
        &rpath,
        serde_json::to_string_pretty(&regs).expect("registration json"),
    )
    .expect("write registration");
    eprintln!("registration table: {}", rpath.display());
}

fn write_results(out_dir: &Path, runs: &[SuiteRun], box_state: &str) {
    // The record number comes from the OUT DIR the caller chose — a
    // hand-typed title here would have kept saying "Bench 001" when the
    // 052-protocol rerun landed in 002_ (it did).
    let bench_tag = out_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| match n.split_once('_') {
            Some((tag, _)) if tag.chars().all(|c| c.is_ascii_digit()) && !tag.is_empty() => {
                format!("Bench {tag}")
            }
            _ => format!("Bench record ({n})"),
        })
        .unwrap_or_else(|| "Bench record".to_string());
    let mut md = String::with_capacity(16 << 10);
    md.push_str(&format!(
        "# {bench_tag} — the Reflex · instinct hybrid GOAT run\n\n"
    ));
    md.push_str("**Status:** MEASURED — the single frozen test read (Issue 005 T5 / \
Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + \
argmax Beta-LCB instrument; predictions frozen in `predictions.json`.\n\n");
    md.push_str(&format!(
        "Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's \
re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome \
off — oc and ridge arm only where their cal-slice selection clears the bar: oc on \
typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit \
through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin \
asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and \
— when the reflex-site checkout stands beside the workspace — that reflex's rows equal the \
PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — \
a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a \
crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ {BETA_GRID:?} × n_min ∈ {N_MIN_GRID:?} × \
τ ∈ {TAU_GRID:?} — 45 candidates, train-side only. Product gate (Issue 008 T2): the \
registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 \
on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 \
instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; \
A1/H1/H2 resolve each presented option to its specialist class row (by name for the \
suites whose keys are the label strings — massive/banking77 — by index under k == N for \
the fixed-criteria suites), and every hybrid pick is a position, directly comparable with \
gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case \
for seat-composing arms, `n_cases` disclosed)."
    ));
    md.push_str(&format!("Box state: {box_state}\n\n"));

    for run in runs {
        md.push_str(&format!("## {}\n\n", run.suite));
        md.push_str(&format!(
            "Verdict: **{}**{}.\n\n",
            run.verdict,
            run.a0_note
                .as_ref()
                .map(|n| format!(" — {n}"))
                .unwrap_or_default()
        ));
        md.push_str(&format!(
            "Posture: cap {} · head {:.2} · nb {:.2} · ridge {:.2} ({}) · fused-gate thresholds \\n{:.3}/{:.3} · specialist bag {}. Questions: {} over {} cases.{}\n\n",
            run.posture.effective_cap,
            run.posture.head_scale,
            run.posture.nb_scale,
            run.posture.ridge_scale,
            run.posture.nb_view,
            run.posture.score_threshold,
            run.posture.distance_threshold,
            run.posture.bag_convention,
            run.n_questions,
            run.n_cases,
            if !run.specialist_present {
                " A0/G0-only posture — no specialist artifact (Issue 010 T2)."
            } else {
                ""
            }
        ));

        // The registration table (train/cal — the instrument's output).
        // The refused pick stays visible beside the served arm.
        md.push_str("### Pre-registration (cal front, train-side only)\n\n");
        md.push_str("| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |\n|---|---|---|---|---|---|\n");
        let pick = run.superiority.as_ref().map(|s| &s.pick);
        for r in &run.registration {
            if r.rank0 || r.cand == run.registered || pick.is_some_and(|p| *p == r.cand) {
                md.push_str(&format!(
                    "| {} | {} | {:.4} | {:.4} | {:.3} | {:.0} |\n",
                    if r.rank0 { "✓" } else { "" },
                    r.arm,
                    r.cal_acc,
                    r.acc_lcb,
                    r.consult,
                    r.p99_us
                ));
            }
        }
        md.push_str(&format!(
            "\nServed arm: **{}** (rank-0 {}/{} candidates; the full table rides \
`registration.json`).{}\n\n",
            run.registered.name(),
            run.registration.iter().filter(|r| r.rank0).count(),
            run.registration.len(),
            match &run.superiority {
                Some(s) if !s.passed => format!(
                    " The instrument's pick **{}** was REFUSED by the superiority gate \
(paired LB95 {:+.4} ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).",
                    s.pick.name(),
                    s.lb95
                ),
                Some(s) => format!(
                    " The superiority gate certified it strictly above Reflex \
(paired LB95 {:+.4} > 0).",
                    s.lb95
                ),
                None => String::new(),
            }
        ));

        // The test table (the single read). The refused pick still shows —
        // its row is the measurement the refusal is grounded in.
        md.push_str("### The single test read\n\n");
        md.push_str("| arm | accuracy | consult | p50 µs | p99 µs |\n|---|---|---|---|---|\n");
        for a in &run.test_arms {
            let is_pick = run
                .superiority
                .as_ref()
                .is_some_and(|s| a.name == s.pick.name())
                || a.name == run.registered.name();
            if a.name == "A0" || a.name == "A1" || a.name == "H1" || is_pick {
                let mark = if a.name == "A0" || a.name == "A1" || a.name == "H1" {
                    ""
                } else if run.registered.name() == a.name {
                    " ★"
                } else {
                    " ✗"
                };
                md.push_str(&format!(
                    "| {}{} | {:.4} | {:.3} | {:.0} | {:.0} |\n",
                    a.name,
                    mark,
                    a.accuracy(),
                    a.consult_rate(),
                    a.p50(),
                    a.p99()
                ));
            }
        }
        md.push('\n');

        // H1's decomposition.
        if let (Some(a0), Some(h1)) = (
            run.test_arms.iter().find(|a| a.name == "A0"),
            run.test_arms.iter().find(|a| a.name == "H1"),
        ) {
            md.push_str(&format!(
                "H1 decomposition: escalation {:.3} · decision p50 {:.0} µs · total p50 \
{:.0} µs (reflex {:.0} µs) · total p99 {:.0} µs.\n\n",
                h1.consult_rate(),
                pct(&h1.own_durs_us, 0.5),
                h1.p50(),
                a0.p50(),
                h1.p99()
            ));
        }

        // The gates.
        md.push_str("### Gates\n\n");
        if let Some(s) = &run.superiority {
            md.push_str(&format!(
                "- **G7 product gate (Issue 008 T2):** {} − A0 mean {:+.4} · paired LB95 \
{:+.4} → {}\n",
                s.pick.name(),
                s.mean,
                s.lb95,
                if s.passed {
                    "PASS (strictly above Reflex)".to_string()
                } else {
                    format!("REFUSED — {} is not sold; A0 serves", s.pick.name())
                }
            ));
        }
        if let Some(g) = &run.g1 {
            md.push_str(&format!(
                "- **G1 calibration:** raw ECE {:.4} · Platt {:.4} (fit engaged: {}) · \
conformal floor {:.4} → {}\n",
                g.ece_raw,
                g.ece_platt,
                g.platt_moved,
                g.ece_floor,
                if g.pass { "PASS" } else { "FAIL" }
            ));
        }
        if let (Some(pd), Some(delta)) = (&run.g3, run.g3_delta) {
            md.push_str(&format!(
                "- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean {:+.4} · UB95 \
{:+.4} vs δ {:.4} (from the cal discordant rate) → {}\n",
                pd.mean,
                pd.ub95,
                delta,
                if pd.ub95 <= delta { "PASS" } else { "FAIL" }
            ));
        }
        if let Some((pass, why)) = &run.g5 {
            md.push_str(&format!(
                "- **G5 pre-registered (gap suite):** {} → {}\n",
                why,
                if *pass { "PASS" } else { "FAIL" }
            ));
        }
        if run.specialist_present {
            md.push_str(&format!(
                "- **G2 fusion overhead:** H1 fusion-only {:.0} ns/question (bar < 100; the full \
escalated decision incl. specialist scoring is {:.0} ns/q) · H2 fusion {:.2} ns/option \
(bar < 100) · absolute p99 in the table\n",
                run.h1_ns_fusion, run.h1_ns_decision, run.h2_ns_per_option
            ));
        } else {
            md.push_str(
                "- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); \
A0's absolute latency is in the table.\n",
            );
        }
        if let Some(l) = &run.laya {
            md.push_str(&format!(
                "- **G2 laya paired ({} escalated):** lane p50 {:.0} µs vs laya p50 {:.0} µs · \
lane−laya UB95 {:+.0} µs ≤ 0 → {}\n",
                l.escalated_n,
                l.lane_p50_us,
                l.laya_p50_us,
                l.paired_ub95_us,
                if l.pass { "PASS" } else { "FAIL" }
            ));
        }
        md.push_str("- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator \
over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).\n");
        md.push_str("- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is \
byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the \
A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to \
reflex's own run().\n");
        md.push_str("- **G6 purity:** the reflex half is the frozen count tables (built \
once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The \
HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.\n\n");
    }

    let path = out_dir.join("RESULTS.md");
    std::fs::write(&path, &md).expect("write results");
    eprintln!("results: {}", path.display());
}

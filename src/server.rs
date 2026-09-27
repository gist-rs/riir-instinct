//! The hosted serving half of the "Reflex · instinct" lane (Plan 001 P5 /
//! Issue 002): per-suite boot over the reflex harness seat — the SAME
//! preparation the arena runs (`prepare_seat` → the deployed Bench-051
//! posture → the seat engine → the sealed winner join) — plus the
//! per-request decision path the arena's `SuiteCtx` carries, reshaped for
//! one state at a time.
//!
//! Two laws hold by construction here:
//!
//! 1. **The serving posture table is the GOAT product verdict** (Bench
//!    002, `.benchmarks/002_hybrid_052_protocol/`), not the registration
//!    instrument's pick. They disagree exactly once: banking77's cal front
//!    registered H1, and G3 FAILED it (the hybrid pays up to ~4.7 pt on
//!    reflex-won cases at 95% confidence) — so the instrument row stays a
//!    published measurement (the site lane carries it with its ECE) while
//!    A0 is what serves. Promotion is GOAT-gated; demote the loser.
//! 2. **The served answer must be the measured answer.** The parity gate
//!    (`tests/serve_gates.rs`) replays committed test cases through
//!    `decide()` and asserts identity with the frozen `predictions.json`
//!    picks — the serve path is the arena path, never a re-derivation.

use std::collections::HashMap;
use std::path::Path;

use riir_reflex::embed::EMBED_DIM;
use riir_reflex::engine::DecisionEngine;
use riir_reflex::harness::runner::seat::{
    PostureKnobs, Seat, build_seat_engine, eval_seat, fit_posture, prepare_seat,
};
use riir_reflex::harness::suites::{GoldAnswer, QKind, SuiteCase, SuiteQuestion};
use riir_reflex::nb_scope::NbView;

use crate::hybrid::{
    A0Answer, Cascade, HybridLane, MAX_TOP_K, PriorFusion, SpecialistLane, prior_fusion_pick,
};
use crate::specialist::{bag_into, decode_artifact};

/// The cascade width the lane joins with when the serving arm is not H1
/// (the arena's default `top_k`; H1 arms join at their own width).
const DEFAULT_TOP_K: usize = 8;

/// The suites the hosted lane serves — the six specialist suites (the
/// arena's GOAT set; reflex's other dataset suites have no specialist and
/// no seat posture registered here).
pub const REGISTERED_SUITES: [&str; 6] = [
    "ag_news",
    "emotion",
    "sst5",
    "massive_intent_en",
    "banking77",
    "xnli_en",
];

/// The per-suite serving arm — the GOAT product verdict (module doc, law 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arm {
    /// G0a — the modelless lane answers; the specialist is never consulted.
    A0,
    /// The specialist alone answers (the modelless lane is not consulted).
    A1,
    /// The cascade: the modelless lane answers unless it abstains; the
    /// specialist decides among the pruned top-k survivors.
    H1 { top_k: usize },
    /// Prior fusion: `p'_i ∝ p_i · exp(g·β·m_i)` over the specialist's
    /// per-option scores, gated by the count tables' evidence.
    H2 {
        beta: f32,
        n_min: f32,
        tau_n: f32,
    },
}

impl Arm {
    /// The display name — byte-identical to the arena's `Cand::name()`
    /// spelling, because the parity gate matches it against the committed
    /// `predictions.json` `registered` strings.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Arm::A0 => "A0".into(),
            Arm::A1 => "A1".into(),
            Arm::H1 { top_k } => format!("H1(top_k={top_k})"),
            Arm::H2 {
                beta,
                n_min,
                tau_n,
            } => format!("H2(β={beta},nmin={n_min},τ={tau_n})"),
        }
    }

    /// The cascade width this arm joins the lane with.
    fn join_top_k(&self) -> usize {
        match self {
            Arm::H1 { top_k } => *top_k,
            _ => DEFAULT_TOP_K,
        }
    }
}

/// The serving posture table (module doc, law 1). `None` = not a served
/// suite.
#[must_use]
pub fn serving_posture(suite: &str) -> Option<Arm> {
    match suite {
        // G5 PASS — the registered H2 beat both controls on the gap suite.
        "ag_news" => Some(Arm::H2 {
            beta: 0.25,
            n_min: 2.0,
            tau_n: 2.0,
        }),
        // A1 — G1+G3 PASS.
        "emotion" | "sst5" => Some(Arm::A1),
        // G1+G3 PASS — the honest stratified +4.7 pt over A0.
        "massive_intent_en" => Some(Arm::H2 {
            beta: 1.0,
            n_min: 2.0,
            tau_n: 8.0,
        }),
        // G3 FAIL (Bench 002): the registered H1 pays up to ~4.7 pt on
        // reflex-won cases at 95% confidence — A0 stands as the product
        // posture; the H1 row stays a published measurement.
        "banking77" | "xnli_en" => Some(Arm::A0),
        _ => None,
    }
}

/// One suite's boot summary (the /healthz disclosure).
#[derive(Debug, Clone)]
pub struct SuiteMeta {
    pub suite: &'static str,
    pub arm: Arm,
    /// The seat's label count (= the default presented-option count).
    pub labels: usize,
    /// The artifact's label count (may exceed the seat's — the
    /// artifact-known, seat-unknown option class).
    pub artifact_labels: usize,
    pub effective_cap: usize,
    pub head_scale: f32,
    pub nb_scale: f32,
    pub score_threshold: f32,
    pub distance_threshold: f32,
    /// BLAKE3 of the sealed winner artifact file (first 16 hex in the
    /// healthz disclosure).
    pub winner_blake3: String,
}

/// One served decision (the /decide payload, pre-serialization).
#[derive(Debug, Clone)]
pub struct ServedDecision {
    pub suite: &'static str,
    pub arm: String,
    /// The presented options, in the order they were presented (the
    /// probabilities' index space).
    pub options: Vec<String>,
    /// `None` when the lane abstained (abstention is a first-class answer).
    pub pick_index: Option<usize>,
    pub pick: Option<String>,
    /// Full-arity probabilities in OPTIONS order — present when the
    /// modelless lane answered; `None` when the specialist's own scores
    /// decided (disclosed in `specialist_scores` instead — honest about
    /// not being a distribution).
    pub probabilities: Option<Vec<f64>>,
    /// The specialist's per-option sigmoid scores, in options order —
    /// present exactly when `probabilities` is `None` and the answer is
    /// not an abstention.
    pub specialist_scores: Option<Vec<f32>>,
    pub confidence: f64,
    /// Whether the decision consulted the specialist (A1/H1-escalated/H2).
    pub escalated: bool,
    pub abstained: bool,
    /// In-engine decision wall time (µs) — the measurement law. Covers
    /// the decision work from option-bridge to pick; excludes HTTP
    /// parse/serialize.
    pub us: u64,
}

/// One suite's serving state: the seat engine + the joined specialist +
/// the pre-allocated scratch. Allocation happens in [`SuiteServer::from_seat`]
/// plus what each request itself forces (the presented-option vectors).
pub struct SuiteServer<const N: usize> {
    suite: &'static str,
    arm: Arm,
    meta: SuiteMeta,
    engine: DecisionEngine<N, EMBED_DIM>,
    lane: HybridLane,
    nb_view: NbView,
    nb_armed: bool,
    labels: Vec<String>,
    /// Presented-option key → (seat label idx, artifact class row). The
    /// seat-label sentinel `usize::MAX` marks the artifact-known,
    /// seat-unknown option (NB evidence NaN; the specialist still scores
    /// the class row).
    key_map: HashMap<String, (usize, usize)>,
    /// The join permutation: seat label i ↔ artifact class row
    /// (the identity bridge's position → class map).
    perm: Vec<usize>,
    /// The question template (kind + instructions + qid) the synthesized
    /// per-request case carries — the suite's own single-question shape.
    q_kind: QKind,
    q_instructions: String,
    qid: String,
    // Scratch (pre-allocated at boot, reused per request).
    bag: Vec<(u32, f32)>,
    tok: Vec<u32>,
    survivors: [(usize, f64); MAX_TOP_K],
    h1_scores: [f32; MAX_TOP_K],
    in_scores: Vec<f32>,
    nb_scratch: Vec<u32>,
    pos_label: Vec<usize>,
    pos_class: Vec<usize>,
    pos_spec: Vec<f32>,
    pos_nb: Vec<f32>,
}

impl<const N: usize> SuiteServer<N> {
    /// Boot one suite from a PREPARED seat (the arity dispatch happens in
    /// [`AnySuiteServer::boot`], which owns `prepare_seat`).
    ///
    /// The preparation steps are the arena's `run_suite_n` prologue,
    /// verbatim: the deployed Bench-051 posture (head-select + nb-select,
    /// registry caps), the seat engine, the sealed winner join, and the
    /// presented-option bridge.
    pub fn from_seat(suite: &'static str, seat: Seat, winners_dir: &Path) -> Result<Self, String> {
        let arm =
            serving_posture(suite).ok_or_else(|| format!("suite {suite} has no serving posture"))?;
        if seat
            .suite
            .cases
            .iter()
            .any(|c| c.questions.len() != 1 || c.questions.iter().any(|q| q.kind == QKind::Noul))
        {
            return Err(format!(
                "suite {suite}: the serving shape is one non-noul question per case"
            ));
        }
        if seat.labels.len() != N {
            return Err(format!(
                "suite {suite}: {} labels vs engine arity {N}",
                seat.labels.len()
            ));
        }
        let (q_kind, q_instructions, qid) = {
            let q = &seat.suite.cases[0].questions[0];
            (q.kind, q.instructions.clone(), q.qid.clone())
        };

        // The deployed Bench 051 protocol: head-select + nb-select,
        // registry caps (no cal-slice cap ladder) — the arena's knobs.
        // (oc_select stays off — reflex's issue-038 lane is not the
        // published posture the serving arms were measured at; this repo
        // never enables reflex/option_cond, so the field is cfg'd out of
        // PostureKnobs here.)
        let knobs = PostureKnobs {
            head_select: true,
            nb_select: true,
            cal_select_caps: vec![],
        };
        let posture = fit_posture::<N>(suite, &seat, &knobs)?;
        let (engine, _fallbacks) =
            build_seat_engine::<N>(suite, &seat, posture.effective_cap, posture.cfg.clone())?;

        // The sealed winner artifact, joined onto the seat's label order
        // (the bijection pin lives in SpecialistLane::join).
        let winner_path = winners_dir.join(format!("{suite}_winner_v1.bin"));
        let winner_bytes = std::fs::read(&winner_path)
            .map_err(|e| format!("read {}: {e}", winner_path.display()))?;
        let winner_blake3 = blake3::hash(&winner_bytes).to_hex()[..16].to_string();
        let spec = decode_artifact(&winner_bytes)
            .map_err(|e| format!("{}: {e}", winner_path.display()))?;
        let artifact_labels_n = spec.labels.len();
        let artifact_labels = spec.labels.clone();
        let top_k = arm.join_top_k();
        let joined = SpecialistLane::join(spec, suite, &seat.labels, Cascade { top_k })?;
        let perm: Vec<usize> = joined.perm.clone();

        // The presented-option bridge (the arena's SuiteCtx::key_map):
        // every seat label → (its own index, its artifact class row); the
        // artifact-known labels the seat never offers join at the
        // sentinel (NaN NB evidence; the specialist still scores the
        // class row it trained).
        let mut key_map: HashMap<String, (usize, usize)> = seat
            .labels
            .iter()
            .enumerate()
            .map(|(li, l)| (l.clone(), (li, joined.perm[li])))
            .collect();
        for (ci, name) in artifact_labels.iter().enumerate() {
            key_map.entry(name.clone()).or_insert((usize::MAX, ci));
        }

        let meta = SuiteMeta {
            suite,
            arm,
            labels: seat.labels.len(),
            artifact_labels: artifact_labels_n,
            effective_cap: posture.effective_cap,
            head_scale: posture.cfg.head_scale,
            nb_scale: posture.cfg.nb_scale,
            score_threshold: posture.score_threshold,
            distance_threshold: posture.distance_threshold,
            winner_blake3,
        };

        Ok(Self {
            suite,
            arm,
            meta,
            nb_view: posture.cfg.nb_view,
            nb_armed: posture.cfg.nb_scale > 0.0 && engine.nb_scope().is_some(),
            engine,
            lane: HybridLane::Specialist(joined),
            labels: seat.labels,
            key_map,
            perm,
            q_kind,
            q_instructions,
            qid,
            bag: Vec::new(),
            tok: Vec::new(),
            survivors: [(0, 0.0); MAX_TOP_K],
            h1_scores: [0.0; MAX_TOP_K],
            in_scores: vec![0.0; N],
            nb_scratch: Vec::new(),
            pos_label: Vec::new(),
            pos_class: Vec::new(),
            pos_spec: Vec::new(),
            pos_nb: Vec::new(),
        })
    }

    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn arm(&self) -> &Arm {
        &self.arm
    }

    pub fn meta(&self) -> &SuiteMeta {
        &self.meta
    }

    /// The canonical presented-option order (the seat's label universe).
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// One decision. `options` = `None` presents the suite's canonical
    /// label universe in seat order; `Some` presents an explicit option
    /// set (a named subset or a positional full-arity set — the arena's
    /// two bridge rules, BY NAME and identity-by-count, decide which).
    pub fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        let t0 = std::time::Instant::now();
        let options: Vec<String> = match options {
            Some(opts) => opts.to_vec(),
            None => self.labels.clone(),
        };
        if state.trim().is_empty() {
            return Err("empty state".into());
        }
        if options.len() < 2 {
            return Err("need ≥2 presented options".into());
        }
        let mut seen = std::collections::HashSet::new();
        if !options.iter().all(|o| seen.insert(o.as_str())) {
            return Err("duplicate presented options — the answer space would collide".into());
        }

        // The bridge (the arena's fill_positions two rules, verbatim):
        // every key resolving through the key map → BY NAME; else count ==
        // seat-label count → IDENTITY BY INDEX; else refuse loud — the
        // specialist bridge is undefined there (Issue 006's instrument
        // defect was exactly a mismatch of these two spaces).
        let all_named = options.iter().all(|k| self.key_map.contains_key(k));
        self.pos_label.clear();
        self.pos_class.clear();
        if all_named {
            for key in &options {
                let (li, cls) = self.key_map[key];
                self.pos_label.push(li);
                self.pos_class.push(cls);
            }
        } else if options.len() == self.perm.len() {
            for (li, &cls) in self.perm.iter().enumerate() {
                self.pos_label.push(li);
                self.pos_class.push(cls);
            }
        } else {
            let unmatched: Vec<String> = options
                .iter()
                .filter(|k| !self.key_map.contains_key(*k))
                .cloned()
                .collect();
            return Err(format!(
                "presented options neither all name seat labels nor match the label count \
                 ({}) — the specialist bridge is undefined; unmatched {unmatched:?}",
                self.labels.len()
            ));
        }

        // The modelless lane's answer: the synthesized one-question case
        // goes through the SAME eval path the seat eval uses
        // (eval_seat → engine_request → decide_with), so A0 here is the
        // arena's A0 byte for byte.
        let state_value = serde_json::Value::String(state.to_string());
        let criteria: serde_json::Value = match self.q_kind {
            QKind::Choice => {
                let mut m = serde_json::Map::new();
                for key in &options {
                    m.insert(key.clone(), serde_json::Value::Null);
                }
                serde_json::Value::Object(m)
            }
            QKind::Score => serde_json::Value::Array(
                options
                    .iter()
                    .map(|k| serde_json::Value::String(k.clone()))
                    .collect(),
            ),
            QKind::Noul => serde_json::Value::Null,
        };
        let case = SuiteCase {
            id: "served".into(),
            state: state_value,
            questions: vec![SuiteQuestion {
                qid: self.qid.clone(),
                kind: self.q_kind,
                instructions: self.q_instructions.clone(),
                criteria,
            }],
            gold: vec![GoldAnswer {
                idx: 0,
                soft: vec![],
                gold_score: None,
            }],
        };
        let se = eval_seat(&mut self.engine, std::slice::from_ref(&case), &[state.to_string()])?;
        let qo = &se.cases[0][0];

        // The specialist's bag + per-position class scores.
        bag_into(state.as_bytes(), &mut self.bag, &mut self.tok);
        self.pos_spec.clear();
        self.pos_spec.resize(self.pos_class.len(), 0.0);
        self.lane
            .scores_classes_into(&self.bag, &self.pos_class, &mut self.pos_spec);

        let arm = self.arm;
        let a0_ans = A0Answer {
            probs: &qo.probs,
            pick: qo.pick,
            abstained: qo.abstained,
        };
        let (pick_index, escalated, abstained, probabilities, specialist_scores, confidence) =
            match arm {
                Arm::A0 => {
                    let abstained = qo.abstained;
                    let pick = (!abstained).then_some(qo.pick);
                    let probs = (!qo.probs.is_empty()).then(|| qo.probs.clone());
                    (pick, false, abstained, probs, None, qo.conf)
                }
                Arm::A1 => {
                    let (p, c) = argmax_pos(&self.pos_spec);
                    (
                        Some(p),
                        true,
                        false,
                        None,
                        Some(self.pos_spec.clone()),
                        f64::from(c),
                    )
                }
                Arm::H1 { .. } => {
                    let d = {
                        let SuiteServer {
                            lane,
                            bag,
                            pos_class,
                            survivors,
                            h1_scores,
                            ..
                        } = self;
                        lane.h1_decide(a0_ans, bag, pos_class, survivors, h1_scores)
                    };
                    if d.escalated {
                        let (_, c) = argmax_pos(&self.pos_spec);
                        (
                            Some(d.pick),
                            true,
                            false,
                            None,
                            Some(self.pos_spec.clone()),
                            f64::from(c),
                        )
                    } else {
                        (
                            Some(d.pick),
                            false,
                            qo.abstained,
                            (!qo.probs.is_empty()).then(|| qo.probs.clone()),
                            None,
                            qo.conf,
                        )
                    }
                }
                Arm::H2 {
                    beta,
                    n_min,
                    tau_n,
                } => {
                    let (n_seen, n_tok) = self.gather_nb(state);
                    let inscores: &[f32] = if self.nb_armed {
                        self.gather_positions_nb();
                        &self.pos_nb
                    } else {
                        &[]
                    };
                    let fusion = PriorFusion {
                        beta,
                        n_min,
                        tau_n,
                    };
                    let f = prior_fusion_pick(&fusion, &self.pos_spec, inscores, n_seen, n_tok);
                    (
                        Some(f.pick),
                        true,
                        false,
                        None,
                        Some(self.pos_spec.clone()),
                        f.conf,
                    )
                }
            };

        let pick = pick_index.map(|i| options[i].clone());
        Ok(ServedDecision {
            suite: self.suite,
            arm: arm.name(),
            options,
            pick_index,
            pick,
            probabilities,
            specialist_scores,
            confidence,
            escalated,
            abstained,
            us: u64::try_from(t0.elapsed().as_micros()).unwrap_or(u64::MAX),
        })
    }

    /// The count tables' evidence for this state (label space) + the
    /// token count — the H2 fusion's gate inputs. Mirrors the arena's
    /// eval_a1_h2 read.
    fn gather_nb(&mut self, state: &str) -> (usize, usize) {
        if !self.nb_armed {
            return (0, 0);
        }
        let tables = self
            .engine
            .nb_scope()
            .expect("nb_armed without tables — the posture lied");
        riir_reflex::nb_scope::view_tokens_into(
            self.nb_view,
            state.as_bytes(),
            &mut self.nb_scratch,
        );
        tables.in_scores(&self.nb_scratch, &mut self.in_scores);
        let seen = tables.seen_count(&self.nb_scratch);
        (seen, self.nb_scratch.len())
    }

    /// Label-space NB in-scores → presented-position space. A sentinel
    /// seat index (the artifact-known, seat-unknown option) carries NO
    /// count-table evidence → NaN, the fusion's no-evidence mark.
    fn gather_positions_nb(&mut self) {
        self.pos_nb.clear();
        self.pos_nb.resize(self.pos_label.len(), 0.0);
        for (o, &li) in self.pos_nb.iter_mut().zip(self.pos_label.iter()) {
            *o = if li == usize::MAX {
                f32::NAN
            } else {
                self.in_scores[li]
            };
        }
    }
}

/// Argmax over the presented-position scores (ties → lowest position, the
/// engine argmax law).
fn argmax_pos(scores: &[f32]) -> (usize, f32) {
    let mut best = 0usize;
    for (i, &s) in scores.iter().enumerate() {
        if s > scores[best] {
            best = i;
        }
    }
    (best, scores[best])
}

/// The arity-erased server (the registry holds one per suite; the six
/// engine arities are const-generic).
pub enum AnySuiteServer {
    S3(Box<SuiteServer<3>>),
    S4(Box<SuiteServer<4>>),
    S5(Box<SuiteServer<5>>),
    S6(Box<SuiteServer<6>>),
    S59(Box<SuiteServer<59>>),
    S77(Box<SuiteServer<77>>),
}

impl AnySuiteServer {
    /// Prepare the seat, then dispatch on its label count (the arena's
    /// `run_suite` dispatch table).
    pub fn boot(
        suite: &'static str,
        datasets_dir: &Path,
        winners_dir: &Path,
    ) -> Result<Self, String> {
        if !REGISTERED_SUITES.contains(&suite) {
            return Err(format!("suite {suite} is not a registered serving suite"));
        }
        if serving_posture(suite).is_none() {
            return Err(format!("suite {suite} has no serving posture"));
        }
        let seat = prepare_seat(suite, datasets_dir)?;
        match seat.labels.len() {
            3 => Ok(AnySuiteServer::S3(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            4 => Ok(AnySuiteServer::S4(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            5 => Ok(AnySuiteServer::S5(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            6 => Ok(AnySuiteServer::S6(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            59 => Ok(AnySuiteServer::S59(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            77 => Ok(AnySuiteServer::S77(Box::new(SuiteServer::from_seat(
                suite, seat, winners_dir,
            )?))),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    pub fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        match self {
            AnySuiteServer::S3(s) => s.decide(state, options),
            AnySuiteServer::S4(s) => s.decide(state, options),
            AnySuiteServer::S5(s) => s.decide(state, options),
            AnySuiteServer::S6(s) => s.decide(state, options),
            AnySuiteServer::S59(s) => s.decide(state, options),
            AnySuiteServer::S77(s) => s.decide(state, options),
        }
    }

    pub fn meta(&self) -> &SuiteMeta {
        match self {
            AnySuiteServer::S3(s) => s.meta(),
            AnySuiteServer::S4(s) => s.meta(),
            AnySuiteServer::S5(s) => s.meta(),
            AnySuiteServer::S6(s) => s.meta(),
            AnySuiteServer::S59(s) => s.meta(),
            AnySuiteServer::S77(s) => s.meta(),
        }
    }
}

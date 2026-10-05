//! The hosted serving half of the "Reflex · instinct" lane (Plan 001 P5 /
//! Issue 002): per-suite boot over the reflex harness seat — the SAME
//! preparation the arena runs (`prepare_seat` → the deployed Bench-051
//! posture → the seat engine → the sealed winner join) — plus the
//! per-request decision path the arena's `SuiteCtx` carries, reshaped for
//! one state at a time.
//!
//! Two laws hold by construction here:
//!
//! 1. **The serving posture is the GOAT product verdict**, and since the
//!    Issue-008 T2 product gate it is ONE verdict: the registration
//!    itself refuses any arm not strictly above the current Reflex row
//!    (paired LB95 > 0 on the frozen test read), so the instrument pick
//!    and the manifest row can no longer disagree. Reflex is free; a
//!    tie — or an edge the paired data cannot certify — sells nothing.
//!    The seat posture is the CURRENT PUBLISHED reflex posture (T1's
//!    re-baseline knobs); the verdict rows live in the arsenal manifest
//!    (Proposal 001, law A5 — the ONE selection surface); the embedded
//!    default is pinned byte-for-byte by the gates (law A6).
//! 2. **The served answer must be the measured answer.** The parity gate
//!    (`tests/serve_gates.rs`) replays committed test cases through
//!    `decide()` and asserts identity with the frozen `predictions.json`
//!    picks — the serve path is the arena path, never a re-derivation.

use std::collections::HashMap;
use std::path::Path;

use riir_reflex::embed::EMBED_DIM;
use riir_reflex::engine::DecisionEngine;
use riir_reflex::harness::runner::seat::{
    CaseEvalScratch, PostureKnobs, Seat, build_seat_engine, eval_case_into, fit_posture,
    prepare_seat,
};
use riir_reflex::harness::suites::{GoldAnswer, QKind, SuiteCase, SuiteQuestion};
use riir_reflex::nb_scope::NbView;

use crate::arsenal::ArsenalManifest;
use crate::hybrid::{
    A0Answer, Cascade, HybridLane, MAX_TOP_K, NOUL_PAIR, PriorFusion, SpecialistLane,
    prior_fusion_pick,
};
use crate::specialist::decode_artifact;

/// The fixed `[false, true]` noul rendering in PRESENTED space (the
/// engine's speak-space; a context seat's noul presentation must be this
/// pair in order, or empty — the fixed rendering).
fn is_fixed_noul_presentation(options: &[String]) -> bool {
    options.len() == 2 && options[0] == "false" && options[1] == "true"
}

/// One question of a multi-question serve request (`decide_multi`,
/// Issue 011): the wire's per-question shape, resolved against the
/// seat's bridge. A noul question may present NO options — the fixed
/// `[false, true]` rendering speaks it (the `decision_wire` law: a noul
/// question carries no option list).
pub struct ServedQuestion<'a> {
    pub qid: &'a str,
    pub kind: QKind,
    pub instructions: &'a str,
    pub options: &'a [String],
}

/// The served request → the suite-case shape the seat eval consumes —
/// the EXACT construction [`SuiteServer::decide_multi`] feeds the
/// modelless eval (`eval_case_into`, reflex issue 070 lead 2), exported
/// because a downstream lane
/// backend must evaluate the SAME bytes: Rethink's ESC lane runs its
/// selective gate over this case (Rethink Issue 017 T2) so the gate
/// flags and the served answers read one request shape. The noul
/// empty-options fixed rendering happens here (the `decision_wire`
/// law). Pure construction — no bridge resolution, no validation: the
/// caller's server validated the presentation before this runs.
pub fn synth_served_case(state: &str, questions: &[ServedQuestion<'_>]) -> SuiteCase {
    let mut case = SuiteCase {
        id: String::new(),
        state: serde_json::Value::Null,
        questions: Vec::new(),
        gold: Vec::new(),
    };
    synth_served_case_into(state, questions, &mut case);
    case
}

/// The reuse form of [`synth_served_case`] (issue 021): writes the SAME
/// case content into `case` while reusing its allocations — the serve
/// path builds the same case shape every decision (the suite's template
/// plus the presented options), so the id/state/question-Vec/gold-Vec
/// buffers persist and any field whose incoming value equals what is
/// already stored is kept, not re-allocated. The result is
/// byte-identical to a fresh [`synth_served_case`] every call, by
/// construction (keep-when-equal preserves the stored bytes; every
/// other field is overwritten from the request).
pub fn synth_served_case_into(
    state: &str,
    questions: &[ServedQuestion<'_>],
    case: &mut SuiteCase,
) {
    if case.id != "served" {
        case.id.clear();
        case.id.push_str("served");
    }
    match &mut case.state {
        serde_json::Value::String(s) if s == state => {}
        serde_json::Value::String(s) => {
            s.clear();
            s.push_str(state);
        }
        _ => case.state = serde_json::Value::String(state.to_string()),
    }
    if case.questions.len() != questions.len() {
        case.questions.clear();
        case.questions.resize(
            questions.len(),
            SuiteQuestion {
                qid: String::new(),
                kind: QKind::Choice,
                instructions: String::new(),
                criteria: serde_json::Value::Null,
            },
        );
    }
    for (cq, q) in case.questions.iter_mut().zip(questions.iter()) {
        if cq.qid != q.qid {
            cq.qid.clear();
            cq.qid.push_str(q.qid);
        }
        cq.kind = q.kind;
        if cq.instructions != q.instructions {
            cq.instructions.clear();
            cq.instructions.push_str(q.instructions);
        }
        // Issue 022 lead 1: the criteria keep-check runs against the WIRE
        // slice directly — `rendered_options` never joins the serve path.
        // It was pure waste here: a noul question's criteria is Null (the
        // rendered list was computed and dropped), and a choice/score
        // question's rendered list IS `q.options` (only the noul-empty
        // fixed `[false, true]` rendering synthesizes, and noul never
        // builds criteria). Steady state (same presented set) is now
        // zero-alloc; a changed set rebuilds from the same slice.
        match q.kind {
            QKind::Choice => set_choice_criteria(&mut cq.criteria, q.options),
            QKind::Score => set_score_criteria(&mut cq.criteria, q.options),
            QKind::Noul => cq.criteria = serde_json::Value::Null,
        }
    }
    case.gold.clear();
    case.gold.extend(questions.iter().map(|_| GoldAnswer {
        idx: 0,
        soft: vec![],
        gold_score: None,
    }));
}

/// The owned rendered option list of a served question (the fixed
/// `[false, true]` rendering when a noul question presents none) — the
/// one home for the rendering rule; validation reads the wire slice
/// directly, the receipt materializes this once per question.
fn rendered_options(q: &ServedQuestion<'_>) -> Vec<String> {
    if q.kind == QKind::Noul && q.options.is_empty() {
        vec!["false".to_string(), "true".to_string()]
    } else {
        q.options.to_vec()
    }
}

/// The Choice criteria for the presented options, keeping `old` when it
/// already holds exactly those keys — the serve path re-presents the
/// same option set every decision, so the hot path pays nothing. The
/// values are Null by construction (this fn is the field's only writer
/// and always builds Null values), which is what keep-when-equal
/// preserves. Fed the wire slice (`q.options`) — for choice/score the
/// rendered list IS the wire slice (issue 022 lead 1).
fn set_choice_criteria(old: &mut serde_json::Value, options: &[String]) {
    if let serde_json::Value::Object(m) = old {
        if m.len() == options.len()
            && m.keys()
                .zip(options.iter())
                .all(|(k, o)| k == o)
        {
            return;
        }
    }
    let mut m = serde_json::Map::new();
    for key in options {
        m.insert(key.clone(), serde_json::Value::Null);
    }
    *old = serde_json::Value::Object(m);
}

/// The Score criteria for the rendered options — the same
/// keep-when-equal law as [`set_choice_criteria`].
fn set_score_criteria(old: &mut serde_json::Value, options: &[String]) {
    if let serde_json::Value::Array(a) = old {
        if a.len() == options.len()
            && a.iter()
                .zip(options.iter())
                .all(|(v, o)| matches!(v, serde_json::Value::String(s) if s == o))
        {
            return;
        }
    }
    *old = serde_json::Value::Array(
        options
            .iter()
            .map(|k| serde_json::Value::String(k.clone()))
            .collect(),
    );
}

/// The cascade width the lane joins with when the serving arm is not H1
/// (the arena's default `top_k`; H1 arms join at their own width).
const DEFAULT_TOP_K: usize = 8;

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
    /// The encoder think-depth arm (instinct issue 016 T2): the sealed
    /// NLEH head over the live laya-english encode — the L3 deep-cognition
    /// op, spent on the salient few. Serves ONLY through a lane backend
    /// (a GPU-host posture; the bag server refuses it at boot — the class
    /// layers do not mix). The open build carries no backend — the
    /// encoder class lives in the private Rethink lane.
    Enc,
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
            Arm::Enc => "ENC".into(),
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
    /// BLAKE3 identity of the loaded weights: the sealed winner FILE's
    /// digest (raw path) or the vessel's signed-region commitment
    /// (vessel path), first 16 hex.
    pub winner_blake3: String,
    /// Where the weights came from.
    pub source: WeightSource,
}

/// The weights' provenance (the /healthz disclosure; the receipt law's
/// artifact half).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightSource {
    /// The raw `<suite>_winner_v1.bin` artifact (the pre-vessel posture).
    RawWinner,
    /// A PUBLIC-RELEASE vessel (the minted path; lineage in the
    /// commitment — the HOSTED-ONLY class reads in the Rethink lane
    /// backend, never here).
    Vessel,
    /// No artifact — the artifact-less A0 posture (owner 2026-10-02
    /// full-coverage serving): the lane IS the modelless tier, the G0a
    /// `HybridLane::ReflexOnly` arm. The receipt names this source so a
    /// no-artifact answer can never masquerade as a weighted one.
    ReflexOnly,
}

/// What the host persists after a successful vessel boot — the monotonic
/// gate's applied state and the lineage handle (vessel feature only;
/// the specialist itself is consumed by the serving lane).
#[cfg(feature = "vessel")]
#[derive(Debug, Clone)]
pub struct VesselFacts {
    pub commitment_hex: String,
    pub artifact_version: u64,
    pub parent_commitment: [u8; 32],
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
    /// The seat engine's fused-gate abstention for this question — the
    /// shipped calibrated gate (score + distance axes) the modelless half
    /// computes for EVERY arm (the eval face runs unconditionally).
    /// Distinct from `abstained`: for A0 they agree; for A1/H2 the served
    /// answer never abstains while the gate may still flag the question.
    /// The ESC lane's escalation set (Rethink Issue 017 T1) reads this —
    /// additive and NOT on the wire (the serve edge serializes a fixed
    /// field list), so the receipt contract is untouched.
    pub gate_abstained: bool,
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
    /// Option-conditioned margin armed (reflex issue 038 T7b tables
    /// present — typed_decisions' posture): the H2 margin source is the
    /// (qid, option) tables, the arena's `oc_armed` law mirrored at the
    /// serve seam so the served fusion IS the registered one.
    oc_armed: bool,
    labels: Vec<String>,
    /// Presented-option key → (seat label idx, artifact class row). The
    /// seat-label sentinel `usize::MAX` marks the artifact-known,
    /// seat-unknown option (NB evidence NaN; the specialist still scores
    /// the class row).
    key_map: HashMap<String, (usize, usize)>,
    /// The join permutation: seat label i ↔ artifact class row
    /// (the identity bridge's position → class map).
    perm: Vec<usize>,
    /// The input-bag convention the artifact was trained under (Issue
    /// 579's bridge): every bag this server builds dispatches through it.
    bag_conv: crate::specialist::BagConvention,
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
    /// Option-conditioned per-option in-scores scratch (oc-armed suites).
    oc_in: Vec<Option<f32>>,
    /// The served question's presented options scratch (the oc gather's
    /// position-aligned key list; copied per request — request-owned).
    pos_options: Vec<String>,
    pos_label: Vec<usize>,
    pos_spec: Vec<f32>,
    pos_nb: Vec<f32>,
    // Scratch (pre-allocated at boot, reused per request) — issue 021's
    // serve-path block: the bridge resolution (per-question label/class
    // positions, grown on demand and cleared per question), the
    // synthesized eval case, and the eval state string. Content is
    // rebuilt per request; the allocations persist.
    bridge_labels: Vec<Vec<usize>>,
    bridge_classes: Vec<Vec<usize>>,
    served_case: SuiteCase,
    state_scratch: String,
    /// The eval frame the request's answers refill into (reflex issue
    /// 070, lead 2): the scratch-refill eval face's per-question
    /// probabilities / picks / confidences / abstain flags. Taken out of
    /// `self` for the duration of the receipt loop (the take/replace
    /// idiom — the receipt's `&mut self` calls must not alias it) and
    /// restored before `Ok`; the early `return Err` paths drop it
    /// (error paths — the warm capacity is rebuilt on the next call).
    eval_frame: CaseEvalScratch,
    /// The suite's corpus centroid folded into the admission space (the
    /// hoarding gate's vector for this suite — Proposal 001 T5).
    centroid: [f32; crate::arsenal_ops::DIM],
}

impl<const N: usize> SuiteServer<N> {
    /// Boot one suite from a PREPARED seat, loading the specialist from
    /// the raw sealed winner artifact (the pre-vessel posture).
    /// `winner_path` comes from the arsenal row (law A5 — the manifest
    /// is the selection surface); `arm` is the row's parsed posture.
    pub fn from_seat(
        suite: &'static str,
        seat: Seat,
        winner_path: &Path,
        arm: Arm,
    ) -> Result<Self, String> {
        let winner_bytes = std::fs::read(winner_path)
            .map_err(|e| format!("read {}: {e}", winner_path.display()))?;
        let winner_blake3 = blake3::hash(&winner_bytes).to_hex()[..16].to_string();
        let spec = decode_artifact(&winner_bytes)
            .map_err(|e| format!("{}: {e}", winner_path.display()))?;
        Self::from_parts(suite, seat, spec, winner_blake3, WeightSource::RawWinner, arm)
    }

    /// Boot one suite from ALREADY-READ artifact bytes (the swap path's
    /// single-read discipline: the bytes are hashed once for the epoch
    /// tag's digest half and decoded from THOSE bytes — never a
    /// hash-then-reread window). Same shape as [`Self::from_seat`].
    pub fn from_bytes(
        suite: &'static str,
        seat: Seat,
        winner_bytes: &[u8],
        arm: Arm,
    ) -> Result<Self, String> {
        let winner_blake3 = blake3::hash(winner_bytes).to_hex()[..16].to_string();
        let spec =
            decode_artifact(winner_bytes).map_err(|e| format!("winner artifact: {e}"))?;
        Self::from_parts(suite, seat, spec, winner_blake3, WeightSource::RawWinner, arm)
    }

    /// Boot one suite from a minted PUBLIC-RELEASE vessel (the teaching
    /// posture): reflexer's public `open` — verify → class gate (the
    /// HOSTED-ONLY class refuses here fail-closed; that class reads in the
    /// Rethink lane backend) — then the monotonic gate over the caller's
    /// [`reflexer_vessel::ApplyState`] (genesis at 0), then the specialist
    /// decode. The caller persists the returned [`VesselFacts`] AFTER the
    /// suite is fully up (never before — a boot failure must not advance
    /// the gate). The specialist itself is consumed by the join.
    #[cfg(feature = "vessel")]
    pub fn from_vessel(
        suite: &'static str,
        seat: Seat,
        vessel_path: &Path,
        pins: &reflexer_vessel::PinTable,
        applied: &reflexer_vessel::ApplyState,
        arm: Arm,
    ) -> Result<(Self, VesselFacts), String> {
        let vv = reflexer_vessel::open(vessel_path, pins)
            .map_err(|e| format!("{}: {e}", vessel_path.display()))?;
        Self::from_verified_vessel(suite, seat, &vv, applied, arm)
    }

    /// Boot from ALREADY-READ vessel bytes (the swap path's single-read
    /// discipline — the bytes hashed for the tag are the bytes decoded
    /// here). Same shape as [`Self::from_vessel`].
    #[cfg(feature = "vessel")]
    pub fn from_vessel_bytes(
        suite: &'static str,
        seat: Seat,
        vessel_bytes: &[u8],
        pins: &reflexer_vessel::PinTable,
        applied: &reflexer_vessel::ApplyState,
        arm: Arm,
    ) -> Result<(Self, VesselFacts), String> {
        let vv = reflexer_vessel::decode(vessel_bytes, pins)
            .map_err(|e| format!("vessel payload: {e}"))?;
        Self::from_verified_vessel(suite, seat, &vv, applied, arm)
    }

    /// The shared tail of both public vessel boots: the monotonic gate →
    /// the specialist decode from the payload → [`Self::from_parts`]. The
    /// payload IS the sealed RISP winner artifact (the same bytes the raw
    /// path hashes), so the downstream join is the bag path verbatim.
    #[cfg(feature = "vessel")]
    fn from_verified_vessel(
        suite: &'static str,
        seat: Seat,
        vv: &reflexer_vessel::VerifiedVessel,
        applied: &reflexer_vessel::ApplyState,
        arm: Arm,
    ) -> Result<(Self, VesselFacts), String> {
        vv.check_monotonic(applied)
            .map_err(|e| format!("suite {suite}: {e}"))?;
        let commitment_hex = vv.commitment_hex();
        let facts = VesselFacts {
            commitment_hex: commitment_hex.clone(),
            artifact_version: vv.header().artifact_version,
            parent_commitment: vv.header().parent_commitment,
        };
        let spec = decode_artifact(vv.payload())
            .map_err(|e| format!("vessel payload: winner artifact: {e}"))?;
        let commitment16 = commitment_hex[..16].to_string();
        let server = Self::from_parts(suite, seat, spec, commitment16, WeightSource::Vessel, arm)?;
        Ok((server, facts))
    }

    /// The shared tail of both boot paths: shape guard, posture, engine,
    /// join, bridge, meta. `weight_id` is the 16-hex identity of the
    /// loaded weights; `artifact_labels_n` is captured by the caller
    /// BEFORE the join consumes the specialist.
    fn from_parts(
        suite: &'static str,
        seat: Seat,
        spec: crate::specialist::Specialist,
        weight_id: String,
        source: WeightSource,
        arm: Arm,
    ) -> Result<Self, String> {
        Self::from_parts_opt(suite, seat, Some(spec), weight_id, source, arm)
    }

    /// The [`Self::from_parts`] tail with the specialist OPTIONAL — the
    /// artifact-less A0 posture (owner 2026-10-02 full-coverage serving):
    /// `spec: None` boots `HybridLane::ReflexOnly` (the G0a first-class
    /// arm — the output is A0 byte-identical, no escalation ever fires)
    /// over the seat engine alone. The identity label join is built from
    /// the seat's own labels so the bridge/noul resolution paths keep
    /// their Named-join shape; under A0 the specialist is never scored.
    fn from_parts_opt(
        suite: &'static str,
        seat: Seat,
        spec: Option<crate::specialist::Specialist>,
        weight_id: String,
        source: WeightSource,
        arm: Arm,
    ) -> Result<Self, String> {
        // The encoder class never seats on the bag server (issue 016 T2):
        // the ENC arm routes to the installed lane backend at the loader,
        // and a SuiteServer constructed with it is a layer mix — refuse
        // here so every from_* path carries the same wall.
        if arm == Arm::Enc {
            return Err(format!(
                "suite {suite}: posture ENC serves through the lane backend (instinct \
                 issue 016 T2) — install the Rethink lane via \
                 riir_instinct::server::install_ext_boots; the bag \
                 server cannot serve the encoder class"
            ));
        }
        // The serving shape (Plan 003, relaxed from one-non-noul-question-
        // per-case; Issue 011 split it by the bridge's ServeContract):
        // SingleQuestion suites keep the one-question-per-case guard (the
        // synthesized request carries the case[0] template); a
        // MultiQuestion suite (typed_decisions — the decision_wire law,
        // "one state, ALL questions answered in one call") answers through
        // `decide_multi` and its single-question `decide` refuses loud.
        let contract = crate::specialist::winner_bridge(suite).serves;
        match contract {
            crate::specialist::ServeContract::SingleQuestion => {
                if seat
                    .suite
                    .cases
                    .iter()
                    .any(|c| c.questions.len() != 1)
                {
                    return Err(format!(
                        "suite {suite}: the serving shape is one question per case (the synthesized \
                         request carries the case[0] template — a multi-question set has no single \
                         serving kind; seat it arena-side or declare the MultiQuestion contract on \
                         the winner bridge)"
                    ));
                }
            }
            crate::specialist::ServeContract::MultiQuestion => {
                // The declaration must not outlive the suite's shape: an
                // empty question set has no answer space, and a suite whose
                // every case carries ONE question is a SingleQuestion suite
                // wearing the wrong declaration.
                if seat.suite.cases.iter().any(|c| c.questions.is_empty()) {
                    return Err(format!(
                        "suite {suite}: a MultiQuestion case carries zero questions — no answer space"
                    ));
                }
                if !seat
                    .suite
                    .cases
                    .iter()
                    .any(|c| c.questions.len() > 1)
                {
                    return Err(format!(
                        "suite {suite}: declared MultiQuestion but every case carries one question — \
                         fix the winner_bridge declaration, not the guard"
                    ));
                }
            }
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

        // The CURRENT PUBLISHED reflex posture (Issue 008 T1's
        // re-baseline): head-select + nb-select + OC-select + ridge-
        // select, registry caps, genome off — byte-identical knobs to the
        // arena's (`run_suite_n`), so the serve path stays the arena
        // path. `ridge_select` is the cal-selected NBSVM-ridge lane
        // (reflex Bench 057): emotion arms @8, every other suite's ladder
        // declines at the arming bar (selected 0.0 — byte-identical to
        // off, reflex's full-workspace delta 0.0000). Boot cost: the
        // emotion ladder ≈ +5 s, the wide suites' ≈ +40–60 s (Bench 057's
        // fit-cost disclosure) — deterministic derivation of the PUBLISHED
        // posture, never a fork from the arena's. Genome selection stays
        // off: reflex's published bench rows predate that lane — turning
        // it on would serve a posture no published row carries.
        // ⚠ oc_select was FALSE until Bench 020 (an upstream-gap note
        // said "the oc lane stays off"): typed's REGISTERED H2 arm fuses
        // over the (qid, option) tables, so the serve engine must carry
        // them — a seat without the oc tables would serve a pure-A1
        // fusion wearing the H2 name (the drift the serve-gate parity
        // caught at its first H2 replay). The knob declines byte-
        // identically on every suite whose train rows carry no gold
        // events (typed is the only armer — the published posture).
        let knobs = PostureKnobs {
            head_select: true,
            nb_select: true,
            oc_select: true,
            ridge_select: true,
            genome_select: false,
            genome_accept_margin: 0.0,
            cal_select_caps: vec![],
            // The served posture is the T1.6 cal-slice fused fit — never
            // the rate levers (the serve parity gates pin this face).
            gate_fit_selection: false,
            gate_distance_only: false,
        };
        let posture = fit_posture::<N>(suite, &seat, &knobs)?;
        let (engine, _fallbacks) =
            build_seat_engine::<N>(suite, &seat, posture.effective_cap, posture.cfg.clone())?;

        let (artifact_labels_n, artifact_labels, joined_lane, perm, context_joined) =
            match spec {
                Some(spec) => {
                    let artifact_labels_n = spec.labels.len();
                    let artifact_labels = spec.labels.clone();
                    let top_k = arm.join_top_k();
                    // The seat↔artifact label join (Plan 003's bridge — the arena's
                    // run_suite_n law verbatim): named (incl. the positional noul
                    // pair) or the context join; a partial overlap refuses in the
                    // join.
                    let joined = match crate::hybrid::seat_join(&seat.labels, &spec.labels) {
                        crate::hybrid::SeatJoin::Named(labels) => {
                            SpecialistLane::join(spec, suite, &labels, Cascade { top_k })?
                        }
                        crate::hybrid::SeatJoin::Context => {
                            SpecialistLane::join_context(spec, suite, &seat.labels, Cascade { top_k })?
                        }
                    };
                    let context_joined = joined.perm.contains(&usize::MAX);
                    let perm: Vec<usize> = joined.perm.clone();
                    (
                        artifact_labels_n,
                        artifact_labels,
                        HybridLane::Specialist(joined),
                        perm,
                        context_joined,
                    )
                }
                None => {
                    // The artifact-less A0 posture: ReflexOnly — the G0a
                    // first-class arm, byte-identical to A0, never a
                    // missing-file fallback. The join is the seat's own
                    // identity (the specialist space IS the seat space);
                    // the A0 decide path never consults it.
                    if arm != Arm::A0 {
                        return Err(format!(
                            "suite {suite}: an artifact-less lane requires posture A0 (ReflexOnly), \
                             got {}",
                            arm.name()
                        ));
                    }
                    (
                        0,
                        Vec::new(),
                        HybridLane::ReflexOnly,
                        (0..N).collect(),
                        false,
                    )
                }
            };

        // The presented-option bridge (the arena's SuiteCtx::key_map):
        // every JOINED seat label → (its own index, its artifact class
        // row); a context seat contributes no seat-label entries. The
        // artifact-known labels the seat never offers join at the
        // sentinel (NaN NB evidence; the specialist still scores the
        // class row it trained).
        let mut key_map: HashMap<String, (usize, usize)> = if context_joined {
            HashMap::new()
        } else {
            seat.labels
                .iter()
                .enumerate()
                .map(|(li, l)| (l.clone(), (li, perm[li])))
                .collect()
        };
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
            winner_blake3: weight_id,
            source,
        };

        // The hoarding gate's vector for this suite (Proposal 001 T5):
        // the corpus centroid over the train pool, folded into the
        // admission space under the suite's OWN bag convention (Issue
        // 579's bridge — a presence lane's direction is a presence fold).
        // Boot-time work — the hot path never touches it.
        let bridge = crate::specialist::winner_bridge(suite);
        let centroid = crate::arsenal_ops::corpus_centroid_with(
            bridge.convention,
            seat.train.iter().map(|d| d.text.as_str()),
        );

        Ok(Self {
            suite,
            arm,
            meta,
            nb_view: posture.cfg.nb_view,
            nb_armed: posture.cfg.nb_scale > 0.0 && engine.nb_scope().is_some(),
            oc_armed: posture.cfg.oc_scale > 0.0 && engine.oc().is_some(),
            engine,
            lane: joined_lane,
            labels: seat.labels,
            key_map,
            perm,
            bag_conv: bridge.convention,
            q_kind,
            q_instructions,
            qid,
            bag: Vec::new(),
            tok: Vec::new(),
            survivors: [(0, 0.0); MAX_TOP_K],
            h1_scores: [0.0; MAX_TOP_K],
            in_scores: vec![0.0; N],
            nb_scratch: Vec::new(),
            oc_in: Vec::new(),
            pos_options: Vec::new(),
            pos_label: Vec::new(),
            pos_spec: Vec::new(),
            pos_nb: Vec::new(),
            bridge_labels: Vec::new(),
            bridge_classes: Vec::new(),
            served_case: SuiteCase {
                id: String::new(),
                state: serde_json::Value::Null,
                questions: Vec::new(),
                gold: Vec::new(),
            },
            state_scratch: String::new(),
            eval_frame: CaseEvalScratch::new(),
            centroid,
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

    /// The suite's corpus centroid — the hoarding gate's vector
    /// (Proposal 001 T5).
    #[must_use]
    pub fn centroid(&self) -> [f32; crate::arsenal_ops::DIM] {
        self.centroid
    }

    /// The canonical presented-option order (the seat's label universe).
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// One decision under the suite's DEFAULT question template (the
    /// single-question contract's form; `options` = `None` presents the
    /// suite's canonical label universe in seat order, `Some` an explicit
    /// option set). A MultiQuestion suite refuses loud — its contract is
    /// [`Self::decide_multi`] (Issue 011: a case[0]-template answer for an
    /// arbitrary state would be a fished partial read).
    pub fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        if crate::specialist::winner_bridge(self.suite).serves
            == crate::specialist::ServeContract::MultiQuestion
        {
            return Err(format!(
                "suite {} serves the multi-question contract — send the question set \
                 (decision_wire: one state, ALL questions answered in one call)",
                self.suite
            ));
        }
        // Everything the question slice needs is cloned out of `self` —
        // decide_multi takes `&mut self` (the engine's eval needs it), so
        // the slice cannot borrow from the server.
        let owned_options: Vec<String> = match options {
            Some(o) => o.to_vec(),
            None => self.labels.clone(),
        };
        let qid = self.qid.clone();
        let instructions = self.q_instructions.clone();
        let kind = self.q_kind;
        let q = [ServedQuestion {
            qid: qid.as_str(),
            kind,
            instructions: instructions.as_str(),
            options: &owned_options,
        }];
        Ok(self.decide_multi(state, &q)?.remove(0))
    }

    /// One multi-question decision (Issue 011 — the `decision_wire` law:
    /// one state, ALL questions answered in one call). Each question
    /// carries its own id/kind/instructions/presented options; the answer
    /// is one [`ServedDecision`] per question, in call order. The state is
    /// bagged ONCE and evaluated through the SAME eval path the seat eval
    /// uses, so A0's per-question bytes are the arena's.
    pub fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String> {
        let t0 = std::time::Instant::now();
        if state.trim().is_empty() {
            return Err("empty state".into());
        }
        if questions.is_empty() {
            return Err("empty question set".into());
        }

        // The bridge (the arena's fill_positions rules, verbatim,
        // resolved PER question): every key resolving through the key map
        // → BY NAME; else count == seat-label count → IDENTITY BY INDEX;
        // else refuse loud — the specialist bridge is undefined there
        // (Issue 006's instrument defect was exactly a mismatch of these
        // two spaces). NOUL (Plan 003 + Issue 011): Named joins keep the
        // single-question contract's positional law; a Context join
        // resolves NOUL_PAIR by name (the arena's fill_positions law) —
        // the seat labels are the engine's domain space, so position has
        // no seat-label meaning. An EMPTY noul presentation takes the
        // fixed [false, true] rendering (the decision_wire law: a noul
        // question carries no option list).
        //
        // Positions land in the per-request scratch (allocation-reused —
        // issue 021); validation reads the wire slice directly — the
        // rendered option list is materialized ONCE per question, at the
        // receipt (the decide loop below).
        if self.bridge_labels.len() < questions.len() {
            self.bridge_labels.resize(questions.len(), Vec::new());
        }
        if self.bridge_classes.len() < questions.len() {
            self.bridge_classes.resize(questions.len(), Vec::new());
        }
        for (qi, q) in questions.iter().enumerate() {
            // Noul questions may present the FIXED rendering by sending no
            // options at all (the decision_wire law: a noul question carries
            // no option list — [false, true] speaks it).
            let noul_fixed = q.kind == QKind::Noul && q.options.is_empty();
            let rendered_len = if noul_fixed {
                NOUL_PAIR.len()
            } else {
                q.options.len()
            };
            if rendered_len < 2 {
                return Err(format!(
                    "question {:?}: need ≥2 presented options",
                    q.qid
                ));
            }
            if !noul_fixed {
                // O(k²) duplicate scan over the presented spellings (k ≤
                // the largest seat universe, 77) — allocation-free; at the
                // serve path's k the hash-set's per-question allocation is
                // the worse trade (issue 021).
                for (i, o) in q.options.iter().enumerate() {
                    if q.options[..i].contains(o) {
                        return Err(format!(
                            "question {:?}: duplicate presented options — the answer space would collide",
                            q.qid
                        ));
                    }
                }
            }
            let pos_label = &mut self.bridge_labels[qi];
            pos_label.clear();
            let pos_class = &mut self.bridge_classes[qi];
            pos_class.clear();
            if q.kind == QKind::Noul {
                if !self.perm.contains(&usize::MAX) {
                    // Named join. Name-first (the arena's fill_positions
                    // law, verbatim): NOUL_PAIR resolved through the key
                    // map — which covers BOTH the pair-is-the-seat
                    // suites (prompt_injections) and the pair-lives-in-
                    // the-artifact suites (code_fixtures: the seat
                    // offers the 8 module labels, the unified no/yes rows
                    // join at the sentinel). The caller's presentation
                    // still VALIDATES against the answer space: pick_index
                    // speaks the CALLER's option list positionally over
                    // the fixed rendering, so a presentation wider or
                    // narrower than the pair has no noul space — refuse
                    // (the positional law's guard, kept; the T8 landing
                    // dropped it and `noul_suite_serves_positionally_
                    // through_the_bridge` red on exactly this — a 3-wide
                    // hostile presentation answered over the pair).
                    let pair_named = NOUL_PAIR.iter().all(|n| self.key_map.contains_key(*n));
                    if pair_named {
                        if rendered_len != NOUL_PAIR.len() {
                            return Err(format!(
                                "suite {}: a noul presentation carries {} options — the fixed \
                                 [false, true] rendering has exactly {} (pick_index speaks that \
                                 space whatever names are presented)",
                                self.suite,
                                rendered_len,
                                NOUL_PAIR.len()
                            ));
                        }
                        for name in NOUL_PAIR {
                            let Some(&(li, cls)) = self.key_map.get(name) else {
                                unreachable!("pair_named checked the keys");
                            };
                            pos_label.push(li);
                            pos_class.push(cls);
                        }
                    } else if rendered_len != self.labels.len() {
                        // The single-question contract's positional law,
                        // byte-preserved: the presented names never reorder
                        // the fixed rendering, position p takes seat label
                        // p and the artifact's pair row p, and the count
                        // must be exactly the seat universe (a wider or
                        // narrower presentation has no noul space).
                        return Err(format!(
                            "suite {}: a noul presentation carries {} options — the fixed \
                             [false, true] rendering has exactly {} (pick_index speaks that \
                             space whatever names are presented)",
                            self.suite,
                            rendered_len,
                            self.labels.len()
                        ));
                    } else {
                        for (li, &cls) in self.perm.iter().enumerate() {
                            pos_label.push(li);
                            pos_class.push(cls);
                        }
                    }
                } else {
                    // The CONTEXT join (the arena's fill_positions law): the
                    // seat labels are the engine's domain space, disjoint
                    // from the pair — resolve NOUL_PAIR BY NAME through the
                    // key map (the artifact's unified no/yes rows; a missing
                    // row is a producer-contract break, loud).
                    if !(noul_fixed || is_fixed_noul_presentation(q.options)) {
                        return Err(format!(
                            "suite {}: a context noul presentation must be the fixed pair in order \
                             (or empty — the fixed rendering): {:?} vs [\"false\", \"true\"]",
                            self.suite,
                            rendered_options(q)
                        ));
                    }
                    for name in NOUL_PAIR {
                        let Some(&(li, cls)) = self.key_map.get(name) else {
                            return Err(format!(
                                "suite {}: the noul bridge needs a {name:?} class row — the \
                                 artifact's unified no/yes pair is missing from the key map",
                                self.suite
                            ));
                        };
                        pos_label.push(li);
                        pos_class.push(cls);
                    }
                }
            } else {
                let all_named = q.options.iter().all(|k| self.key_map.contains_key(k));
                if all_named {
                    for key in q.options {
                        let (li, cls) = self.key_map[key];
                        pos_label.push(li);
                        pos_class.push(cls);
                    }
                } else if !self.perm.contains(&usize::MAX)
                    && rendered_len == self.perm.len()
                {
                    for (li, &cls) in self.perm.iter().enumerate() {
                        pos_label.push(li);
                        pos_class.push(cls);
                    }
                } else {
                    let unmatched: Vec<String> = q.options
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
            }
        }

        // The modelless lane's answers: the synthesized multi-question case
        // goes through the SAME eval path the seat eval uses — its
        // scratch-refill face (eval_case_into → decide_with; byte-parity
        // vs eval_seat is the reflex-side contract + the frozen-picks
        // replay below), so A0 here is the arena's A0 byte for byte, per
        // question. The construction is the
        // exported [`synth_served_case`] law — [`synth_served_case_into`]
        // writes the same bytes into the per-request case scratch
        // (allocation-reused, issue 021), shared with the downstream
        // lanes that evaluate the same bytes.
        synth_served_case_into(state, questions, &mut self.served_case);
        self.state_scratch.clear();
        self.state_scratch.push_str(state);
        // The take/replace idiom: the eval frame leaves `self` so the
        // receipt loop's `&mut self` calls (lane scores, the H2 evidence
        // gathers) never alias it. Restored after the loop; the early
        // `return Err` paths below lose its warm capacity (error paths).
        let mut frame = std::mem::take(&mut self.eval_frame);
        eval_case_into(
            &mut self.engine,
            &self.served_case,
            &self.state_scratch,
            &mut frame,
        )?;
        if frame.abstained.len() != questions.len() {
            return Err(format!(
                "suite {}: the engine answered {} of {} questions — eval shape drift",
                self.suite,
                frame.abstained.len(),
                questions.len()
            ));
        }

        // The specialist's bag — built ONCE per state under the artifact's
        // training convention (Issue 579's bridge); the per-question class
        // scores read the same bag.
        self.bag_conv
            .bag_into(state.as_bytes(), &mut self.bag, &mut self.tok);

        let arm = self.arm;
        let mut decisions = Vec::with_capacity(questions.len());
        for (qi, q) in questions.iter().enumerate() {
            // The frame's per-question outputs as locals — the slice reads
            // (the frame is a local, taken out of `self` for this loop).
            let qo_pick = frame.picks[qi];
            let qo_conf = frame.confs[qi];
            let qo_abst = frame.abstained[qi];
            let qo_probs: &[f64] = &frame.probs[qi];
            let pos_class = &self.bridge_classes[qi];
            // The rendered option list, built ONCE per question and MOVED
            // into the receipt below (issue 021 — it was built in the
            // bridge pass and cloned again here).
            let options = rendered_options(q);
            self.pos_spec.clear();
            self.pos_spec.resize(pos_class.len(), 0.0);
            // The specialist scores unconditionally for the weighted arms
            // (the A0 dispatch never reads pos_spec); the ReflexOnly lane
            // has no specialist — the G0a arm is A0 byte-identical, so the
            // scores slot stays at zero and the decision's
            // specialist_scores report `None` (the honest receipt).
            if matches!(self.lane, crate::hybrid::HybridLane::Specialist(_)) {
                self.lane
                    .scores_classes_into(&self.bag, pos_class, &mut self.pos_spec);
            }
            let a0_ans = A0Answer {
                probs: qo_probs,
                pick: qo_pick,
                abstained: qo_abst,
            };
            let (pick_index, escalated, abstained, probabilities, specialist_scores, confidence) =
                match arm {
                    Arm::A0 => {
                        let abstained = qo_abst;
                        let pick = (!abstained).then_some(qo_pick);
                        let probs = (!qo_probs.is_empty()).then(|| qo_probs.to_vec());
                        (pick, false, abstained, probs, None, qo_conf)
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
                            let SuiteServer { lane, bag, survivors, h1_scores, .. } = self;
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
                                qo_abst,
                                (!qo_probs.is_empty()).then(|| qo_probs.to_vec()),
                                None,
                                qo_conf,
                            )
                        }
                    }
                    Arm::H2 {
                        beta,
                        n_min,
                        tau_n,
                    } => {
                        let (n_seen, n_tok) = self.gather_evidence(state);
                        // The margin gather mirrors the arena's law: the
                        // oc-armed posture reads the (qid, option) tables
                        // over the PRESENTED options (position-aligned —
                        // `options` IS the presentation order the
                        // positions were built from); the nb-armed label
                        // space stands otherwise. One of the two always
                        // holds for a registered H2 arm (the registration
                        // read had a margin source).
                        let inscores: &[f32] = if self.oc_armed {
                            self.pos_options.clear();
                            if q.kind == QKind::Noul {
                                // The oc tables spell noul options
                                // "no"/"yes" (the event law,
                                // `typed_gold_events`); the presented
                                // fixed rendering is ["false","true"] —
                                // position-aligned 1:1, so the LOOKUP keys
                                // translate and the positions do not move
                                // (the arena's pos_keys = NOUL_PAIR, the
                                // exact same spellings in the same order).
                                self.pos_options
                                    .extend(["no".to_string(), "yes".to_string()]);
                                } else {
                                    self.pos_options.extend_from_slice(&options);
                                }
                            self.gather_positions_oc(q.qid);
                            &self.pos_nb
                        } else if self.nb_armed {
                            self.pos_label.clear();
                            self.pos_label.extend_from_slice(&self.bridge_labels[qi]);
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
                        let f =
                            prior_fusion_pick(&fusion, &self.pos_spec, inscores, n_seen, n_tok);
                        (
                            Some(f.pick),
                            true,
                            false,
                            None,
                            Some(self.pos_spec.clone()),
                            f.conf,
                        )
                    }
                    // Unreachable by construction: from_parts refuses ENC
                    // at boot — the encoder class serves through the
                    // lane backend (issue 016 T2), never the bag
                    // server. The arm exists so the manifest grammar can
                    // name the posture; the match must stay exhaustive.
                    Arm::Enc => {
                        return Err(format!(
                            "suite {}: posture ENC serves through the lane backend, \
                             never the bag server",
                            self.suite
                        ));
                    }
                };
            let pick = pick_index.map(|i| options[i].clone());
            decisions.push(ServedDecision {
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
                gate_abstained: qo_abst,
            });
        }
        // Restore the frame (its warm scratch survives to the next
        // request); error paths above dropped it by design.
        self.eval_frame = frame;
        Ok(decisions)
    }

    /// The count tables' evidence for this state (label space) + the
    /// token count — the H2 fusion's gate inputs. Mirrors the arena's
    /// eval_a1_h2 read. The oc-armed margin reads the (qid, option)
    /// family's OWN evidence stream (the same seen-bitmap resolution over
    /// the event docs that family was fitted on).
    fn gather_evidence(&mut self, state: &str) -> (usize, usize) {
        if !self.oc_armed && !self.nb_armed {
            return (0, 0);
        }
        riir_reflex::nb_scope::view_tokens_into(
            self.nb_view,
            state.as_bytes(),
            &mut self.nb_scratch,
        );
        let n_tok = self.nb_scratch.len();
        if self.oc_armed {
            let oc = self
                .engine
                .oc()
                .expect("oc_armed without tables — the posture lied");
            (oc.seen_count(&self.nb_scratch), n_tok)
        } else {
            let tables = self
                .engine
                .nb_scope()
                .expect("nb_armed without tables — the posture lied");
            tables.in_scores(&self.nb_scratch, &mut self.in_scores);
            (tables.seen_count(&self.nb_scratch), n_tok)
        }
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

    /// Option-conditioned gather (the oc-armed margin source — typed_
    /// decisions): per-option in-scores over the state tokens, position-
    /// aligned through `pos_options` (the presented options, in
    /// presentation order — the exact list the positions were built
    /// from). A presented key with no fitted (qid, option) table reads
    /// NaN — the same no-evidence mark, so an unfitted option mutes and
    /// never rivals. The arena's `gather_positions_oc`, mirrored.
    fn gather_positions_oc(&mut self, qid: &str) {
        let Some(oc) = self.engine.oc() else {
            panic!("oc gather without tables — the posture lied");
        };
        self.oc_in.clear();
        self.oc_in.resize(self.pos_options.len(), None);
        oc.in_scores(qid, &self.pos_options, &self.nb_scratch, &mut self.oc_in);
        self.pos_nb.clear();
        self.pos_nb.resize(self.oc_in.len(), 0.0);
        for (o, slot) in self.pos_nb.iter_mut().zip(self.oc_in.iter()) {
            *o = match slot {
                Some(s) => *s,
                None => f32::NAN,
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

/// The lane-backend extension point (Proposal 052, the carve seam): the
/// contract a DOWNSTREAM lane class implements to seat itself in the
/// server. The bag lanes (S2–S77) are this crate's own; the [`Arm::Enc`]
/// class is served by whatever backend is INSTALLED at startup — the open
/// build carries none, so an ENC row refuses loud (never a silent bag
/// fallback). Rethink (the private lane repo) implements this trait and
/// installs its loader via [`install_ext_boots`]; the moat sits DOWNSTREAM
/// of this open code — the riir-refine shape.
pub trait LaneBackend: Send {
    /// One decision under the suite's default question template (the
    /// bag server's decide signature — the serve edge calls it identically).
    fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String>;
    /// The multi-question contract (Issue 011): one state, ALL questions
    /// answered in one call, one [`ServedDecision`] per question in order.
    fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String>;
    /// The boot summary — the same &SuiteMeta read the bag servers serve
    /// (the healthz disclosure shape).
    fn meta(&self) -> &SuiteMeta;
    /// The suite's corpus centroid — the hoarding gate's vector.
    fn centroid(&self) -> [f32; crate::arsenal_ops::DIM];
}

/// The bytes-boot half of the extension point: the lane artifact's raw
/// bytes (the head, for the encoder class) in, a backend lane out. The
/// route reads the row's artifact file exactly as the manifest names it
/// (law A5) and hands the bytes here — the backend owns their meaning.
pub type ExtBootBytes = fn(
    suite: &'static str,
    seat: Seat,
    artifact_bytes: &[u8],
    arm: Arm,
) -> Result<Box<dyn LaneBackend>, String>;

/// The installed lane backend (the extension point's process-global slot).
static EXT_BOOTS: std::sync::OnceLock<Option<ExtBoots>> = std::sync::OnceLock::new();

/// The backend installer's handle. A downstream crate builds one and
/// hands it to [`install_ext_boots`] before any boot.
pub struct ExtBoots {
    /// The bytes boot (the raw-lane-artifact route).
    pub bytes: ExtBootBytes,
}

/// The installed backend, if any. The open build never installs one —
/// an ENC row refuses loud. A downstream crate (Rethink) installs first
/// via [`install_ext_boots`].
fn ext_boots() -> Option<&'static ExtBoots> {
    EXT_BOOTS.get().and_then(|o| o.as_ref())
}

/// Install the lane backend (the extension point's startup seam — the
/// downstream lane crate calls this once before any boot). Errors when a
/// backend is already installed (install-once; the slot is process-global).
///
/// Post-split this is Rethink's plug-in path: the open build never calls
/// it, so an ENC manifest row refuses loud naming this function.
pub fn install_ext_boots(boots: ExtBoots) -> Result<(), String> {
    EXT_BOOTS
        .set(Some(boots))
        .map_err(|_| "lane backend already installed".to_string())
}

/// The arity-erased server (the registry holds one per suite; the
/// engine arities are const-generic). S2 (Plan 003) seats the noul
/// suites' 2-label universe — prompt_injections. The Ext variant (the
/// Proposal-052 carve seam) seats an installed lane backend — the open
/// build leaves the slot empty and an ENC row refuses loud.
pub enum AnySuiteServer {
    S2(Box<SuiteServer<2>>),
    S3(Box<SuiteServer<3>>),
    S4(Box<SuiteServer<4>>),
    S5(Box<SuiteServer<5>>),
    S6(Box<SuiteServer<6>>),
    S8(Box<SuiteServer<8>>),
    S59(Box<SuiteServer<59>>),
    S77(Box<SuiteServer<77>>),
    /// The extension-point seat: a downstream lane backend (the encoder
    /// class lives in the private Rethink lane), installed via
    /// [`install_ext_boots`]. The dispatch is the trait, byte-identically.
    Ext(Box<dyn LaneBackend>),
}

impl AnySuiteServer {
    /// The manifest row's parsed arm for one suite — the shared front of
    /// every boot path (law A5: the manifest is the only selection
    /// surface; there is no fallback table behind it).
    fn posture_of(manifest: &ArsenalManifest, suite: &str) -> Result<Arm, String> {
        let row = manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
        row.to_arm()
    }

    /// Prepare the seat, then dispatch on its label count (the arena's
    /// `run_suite` dispatch table).
    pub fn boot(
        suite: &'static str,
        datasets_dir: &Path,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
    ) -> Result<Self, String> {
        let arm = Self::posture_of(manifest, suite)?;
        let seat = prepare_seat(suite, datasets_dir)?;
        Self::boot_from_seat_arm(suite, seat, winners_dir, manifest, arm)
    }

    /// The arity dispatch over an ALREADY-PREPARED seat (the serve
    /// binary's loader shares the seat between the raw and vessel paths).
    pub fn boot_from_seat(
        suite: &'static str,
        seat: Seat,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
    ) -> Result<Self, String> {
        let arm = Self::posture_of(manifest, suite)?;
        Self::boot_from_seat_arm(suite, seat, winners_dir, manifest, arm)
    }

    /// The ESC cheap-leg boot (the sanctioned escalate-aware caller: the
    /// private Rethink ESC wrapper/loader): boots the row's OWN posture with
    /// the escalate table deliberately IGNORED — the cheap leg the ESC lane
    /// composes over. Every other boot path refuses an escalate row loud (the
    /// open build never serves the composition); this is the one deliberate
    /// exception, documented for the private lane's wrapper.
    pub fn boot_cheap_from_seat(
        suite: &'static str,
        seat: Seat,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
    ) -> Result<Self, String> {
        let arm = Self::posture_of(manifest, suite)?;
        Self::boot_from_seat_arm_inner(suite, seat, winners_dir, manifest, arm, true)
    }

    /// The ARTIFACT-LESS boot (owner 2026-10-02 full-coverage serving):
    /// the manifest row carries no digest, so the lane is the modelless
    /// tier alone — `HybridLane::ReflexOnly` (the G0a arm, byte-identical
    /// to A0) over the seat engine. Only an `A0` row may boot here; the
    /// refusal is posture-as-data, never a defaulted specialist.
    pub fn boot_a0_from_seat(
        suite: &'static str,
        seat: Seat,
        manifest: &ArsenalManifest,
    ) -> Result<Self, String> {
        let arm = Self::posture_of(manifest, suite)?;
        if arm != Arm::A0 {
            return Err(format!(
                "suite {suite}: artifact-less boot requires posture A0, got {}",
                arm.name()
            ));
        }
        macro_rules! a0_arm {
            ($variant:ident, $n:literal) => {{
                let server = SuiteServer::<$n>::from_parts_opt(
                    suite,
                    seat,
                    None,
                    "a0-reflex-only".to_string(),
                    WeightSource::ReflexOnly,
                    arm,
                )?;
                Ok(AnySuiteServer::$variant(Box::new(server)))
            }};
        }
        match seat.labels.len() {
            2 => a0_arm!(S2, 2),
            3 => a0_arm!(S3, 3),
            4 => a0_arm!(S4, 4),
            5 => a0_arm!(S5, 5),
            6 => a0_arm!(S6, 6),
            8 => a0_arm!(S8, 8),
            59 => a0_arm!(S59, 59),
            77 => a0_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    fn boot_from_seat_arm(
        suite: &'static str,
        seat: Seat,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
        arm: Arm,
    ) -> Result<Self, String> {
        Self::boot_from_seat_arm_inner(suite, seat, winners_dir, manifest, arm, false)
    }

    /// The escalate-row wall (riir-rethink Issue 017 T5): a row carrying
    /// an `escalate` table composes in the private Rethink lane ONLY — the
    /// open build refuses rather than serve the cheap leg silently (the
    /// composition's absence would read as the incumbent's verdict). The
    /// one sanctioned exception is [`Self::boot_cheap_from_seat`].
    fn escalate_refusal(suite: &str, arm: &Arm) -> String {
        format!(
            "suite {suite}: posture {} carries an `escalate` table — the ESC composition \
             lives in the private Rethink lane; the open build refuses rather than serve the \
             cheap leg silently (the sanctioned cheap boot is \
             AnySuiteServer::boot_cheap_from_seat)",
            arm.name()
        )
    }

    fn boot_from_seat_arm_inner(
        suite: &'static str,
        seat: Seat,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
        arm: Arm,
        allow_escalate: bool,
    ) -> Result<Self, String> {
        let row = manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
        // The ESC escalate wall (riir-rethink Issue 017 T5): fires BEFORE
        // the ext-backend check so the refusal names the composition, never
        // a downstream class error. boot/boot_from_seat route here with the
        // wall armed; boot_cheap_from_seat is the one caller that passes
        // `allow_escalate`.
        if !allow_escalate && row.escalate.is_some() {
            return Err(Self::escalate_refusal(suite, &arm));
        }
        // The encoder route (issue 016 T2): the head is NOT a bridged
        // winner — it loads by its own `file` before the winner-convention
        // check, and only where a lane backend is installed (the carve
        // seam: the open build refuses loud, never a silent bag fallback).
        if arm == Arm::Enc {
            let Some(boots) = ext_boots() else {
                return Err(format!(
                    "suite {suite}: posture ENC has no lane backend in this build — the \
                     encoder class lives in the private Rethink lane; a downstream crate \
                     installs it via riir_instinct::server::install_ext_boots before any \
                     boot"
                ));
            };
            let head_name = row.artifact_file(format!("{suite}_encoder_head_v1.bin"));
            let head_path = winners_dir.join(&head_name);
            let bytes =
                std::fs::read(&head_path).map_err(|e| format!("read head {head_name}: {e}"))?;
            return Ok(AnySuiteServer::Ext((boots.bytes)(suite, seat, &bytes, arm)?));
        }
        let winner_name = row.artifact_file(format!("{suite}_winner_v1.bin"));
        // The raw-mode convention coupling (Issue 579): a bridged suite
        // loads EXACTLY its bridged file, loud refusal otherwise.
        crate::specialist::check_winner_file(suite, &winner_name)?;
        let winner_path = winners_dir.join(winner_name);
        macro_rules! seat_arm {
            ($variant:ident, $n:literal) => {{
                let server = SuiteServer::<$n>::from_seat(suite, seat, &winner_path, arm)?;
                Ok(AnySuiteServer::$variant(Box::new(server)))
            }};
        }
        match seat.labels.len() {
            2 => seat_arm!(S2, 2),
            3 => seat_arm!(S3, 3),
            4 => seat_arm!(S4, 4),
            5 => seat_arm!(S5, 5),
            6 => seat_arm!(S6, 6),
            8 => seat_arm!(S8, 8),
            59 => seat_arm!(S59, 59),
            77 => seat_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The bytes-based dispatch over an ALREADY-PREPARED seat and
    /// ALREADY-READ artifact bytes — the swap/lazy loader's entry (the
    /// single-read discipline: the caller hashed these exact bytes for
    /// the epoch tag). Same shape as [`Self::boot_from_seat`].
    pub fn boot_bytes(
        suite: &'static str,
        seat: Seat,
        artifact_bytes: &[u8],
        manifest: &ArsenalManifest,
    ) -> Result<Self, String> {
        let arm = Self::posture_of(manifest, suite)?;
        // The ESC escalate wall (riir-rethink Issue 017 T5) — same law as
        // the seat path's: the composition is the private lane's; the bytes
        // entry refuses rather than serve the cheap leg silently.
        if manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?
            .escalate
            .is_some()
        {
            return Err(Self::escalate_refusal(suite, &arm));
        }
        // The encoder route (issue 016 T2) — the bytes ARE the sealed head,
        // consumed by the installed lane backend (the carve seam).
        if arm == Arm::Enc {
            let Some(boots) = ext_boots() else {
                return Err(format!(
                    "suite {suite}: posture ENC has no lane backend in this build — the \
                     encoder class lives in the private Rethink lane; a downstream crate \
                     installs it via riir_instinct::server::install_ext_boots before any \
                     boot"
                ));
            };
            return Ok(AnySuiteServer::Ext((boots.bytes)(
                suite, seat, artifact_bytes, arm,
            )?));
        }
        macro_rules! seat_arm {
            ($variant:ident, $n:literal) => {{
                let server = SuiteServer::<$n>::from_bytes(suite, seat, artifact_bytes, arm)?;
                Ok(AnySuiteServer::$variant(Box::new(server)))
            }};
        }
        match seat.labels.len() {
            2 => seat_arm!(S2, 2),
            3 => seat_arm!(S3, 3),
            4 => seat_arm!(S4, 4),
            5 => seat_arm!(S5, 5),
            6 => seat_arm!(S6, 6),
            8 => seat_arm!(S8, 8),
            59 => seat_arm!(S59, 59),
            77 => seat_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The vessel boot: the same dispatch, the specialist from a minted
    /// PUBLIC-RELEASE vessel (vessel feature only). The vessel file is the
    /// row's artifact (law A5 — no filename convention behind the
    /// manifest's back). The HOSTED-ONLY class refuses here fail-closed —
    /// that class reads in the Rethink lane backend.
    #[cfg(feature = "vessel")]
    #[allow(clippy::too_many_arguments)]
    pub fn boot_vessel(
        suite: &'static str,
        seat: Seat,
        vessels_dir: &Path,
        manifest: &ArsenalManifest,
        pins: &reflexer_vessel::PinTable,
        applied: &reflexer_vessel::ApplyState,
    ) -> Result<(Self, VesselFacts), String> {
        let arm = Self::posture_of(manifest, suite)?;
        // The ENC route (issue 016 T4): the head vessel is a HOSTED-ONLY
        // artifact — the moat class; the open build has neither the reader
        // nor the lane. Refuse loud naming the backend install seam.
        if arm == Arm::Enc {
            let _ = (&seat, vessels_dir, pins, applied);
            return Err(format!(
                "suite {suite}: posture ENC rides the vessel lane only in the private Rethink \
                 lane backend (the HOSTED-ONLY class reads there); this build boots the \
                 PUBLIC-RELEASE class for bag lanes only"
            ));
        }
        let row = manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
        // The ESC escalate wall (riir-rethink Issue 017 T5): the hosted ESC
        // lands with the v1 Phase-C composition — until then vessel mode
        // refuses an escalate row loud (the manifest's own documented law).
        if row.escalate.is_some() {
            return Err(Self::escalate_refusal(suite, &arm));
        }
        let vessel_path = vessels_dir.join(row.artifact_file(format!("{suite}_v1.vessel")));
        macro_rules! vessel_arm {
            ($variant:ident, $n:literal) => {{
                let (server, facts) = SuiteServer::<$n>::from_vessel(
                    suite,
                    seat,
                    &vessel_path,
                    pins,
                    applied,
                    arm,
                )?;
                Ok((AnySuiteServer::$variant(Box::new(server)), facts))
            }};
        }
        match seat.labels.len() {
            2 => vessel_arm!(S2, 2),
            3 => vessel_arm!(S3, 3),
            4 => vessel_arm!(S4, 4),
            5 => vessel_arm!(S5, 5),
            6 => vessel_arm!(S6, 6),
            8 => vessel_arm!(S8, 8),
            59 => vessel_arm!(S59, 59),
            77 => vessel_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The bytes-based vessel dispatch — the swap/lazy loader's vessel
    /// entry (single-read discipline, as [`Self::boot_bytes`]). The
    /// HOSTED-ONLY class refuses here fail-closed — that class reads in
    /// the Rethink lane backend.
    #[cfg(feature = "vessel")]
    #[allow(clippy::too_many_arguments)]
    pub fn boot_vessel_bytes(
        suite: &'static str,
        seat: Seat,
        vessel_bytes: &[u8],
        manifest: &ArsenalManifest,
        pins: &reflexer_vessel::PinTable,
        applied: &reflexer_vessel::ApplyState,
    ) -> Result<(Self, VesselFacts), String> {
        let arm = Self::posture_of(manifest, suite)?;
        if arm == Arm::Enc {
            let _ = (&seat, vessel_bytes, pins, applied);
            return Err(format!(
                "suite {suite}: posture ENC rides the vessel lane only in the private Rethink \
                 lane backend (the HOSTED-ONLY class reads there); this build boots the \
                 PUBLIC-RELEASE class for bag lanes only"
            ));
        }
        // The ESC escalate wall (riir-rethink Issue 017 T5) — same law as
        // boot_vessel's: the hosted ESC is Phase-C future work; an escalate
        // row refuses loud rather than serve the cheap leg silently.
        if manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?
            .escalate
            .is_some()
        {
            return Err(Self::escalate_refusal(suite, &arm));
        }
        macro_rules! vessel_arm {
            ($variant:ident, $n:literal) => {{
                let (server, facts) = SuiteServer::<$n>::from_vessel_bytes(
                    suite,
                    seat,
                    vessel_bytes,
                    pins,
                    applied,
                    arm,
                )?;
                Ok((AnySuiteServer::$variant(Box::new(server)), facts))
            }};
        }
        match seat.labels.len() {
            2 => vessel_arm!(S2, 2),
            3 => vessel_arm!(S3, 3),
            4 => vessel_arm!(S4, 4),
            5 => vessel_arm!(S5, 5),
            6 => vessel_arm!(S6, 6),
            8 => vessel_arm!(S8, 8),
            59 => vessel_arm!(S59, 59),
            77 => vessel_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The parts-based boot for the DOWNSTREAM lane host (Rethink): its
    /// HOSTED-ONLY vessel reader opens, verifies and decrypts with its
    /// own key, decodes the specialist and hands it HERE with the vessel
    /// lineage identity (the 16-hex commitment prefix the raw boot names
    /// `winner_blake3` from). The open build's own boots never call it —
    /// theirs open the PUBLIC class internally
    /// ([`Self::boot_vessel_bytes`]); a caller that has not verified the
    /// artifact has no business here (the class discipline is the
    /// caller's, the shape guard is this fn's).
    pub fn boot_hosted_parts(
        suite: &'static str,
        seat: Seat,
        spec: crate::specialist::Specialist,
        commitment16: String,
        arm: Arm,
    ) -> Result<Self, String> {
        macro_rules! parts_arm {
            ($variant:ident, $n:literal) => {{
                let server = SuiteServer::<$n>::from_parts(
                    suite,
                    seat,
                    spec,
                    commitment16,
                    WeightSource::Vessel,
                    arm,
                )?;
                Ok(AnySuiteServer::$variant(Box::new(server)))
            }};
        }
        match seat.labels.len() {
            2 => parts_arm!(S2, 2),
            3 => parts_arm!(S3, 3),
            4 => parts_arm!(S4, 4),
            5 => parts_arm!(S5, 5),
            6 => parts_arm!(S6, 6),
            8 => parts_arm!(S8, 8),
            59 => parts_arm!(S59, 59),
            77 => parts_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    pub fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        match self {
            AnySuiteServer::S2(s) => s.decide(state, options),
            AnySuiteServer::S3(s) => s.decide(state, options),
            AnySuiteServer::S4(s) => s.decide(state, options),
            AnySuiteServer::S5(s) => s.decide(state, options),
            AnySuiteServer::S6(s) => s.decide(state, options),
            AnySuiteServer::S8(s) => s.decide(state, options),
            AnySuiteServer::S59(s) => s.decide(state, options),
            AnySuiteServer::S77(s) => s.decide(state, options),
            AnySuiteServer::Ext(s) => s.decide(state, options),
        }
    }

    /// The multi-question contract (Issue 011): one state, ALL questions
    /// answered in one call (the `decision_wire` law). One
    /// [`ServedDecision`] per question, in call order.
    pub fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String> {
        match self {
            AnySuiteServer::S2(s) => s.decide_multi(state, questions),
            AnySuiteServer::S3(s) => s.decide_multi(state, questions),
            AnySuiteServer::S4(s) => s.decide_multi(state, questions),
            AnySuiteServer::S5(s) => s.decide_multi(state, questions),
            AnySuiteServer::S6(s) => s.decide_multi(state, questions),
            AnySuiteServer::S8(s) => s.decide_multi(state, questions),
            AnySuiteServer::S59(s) => s.decide_multi(state, questions),
            AnySuiteServer::S77(s) => s.decide_multi(state, questions),
            AnySuiteServer::Ext(s) => s.decide_multi(state, questions),
        }
    }

    pub fn meta(&self) -> &SuiteMeta {
        match self {
            AnySuiteServer::S2(s) => s.meta(),
            AnySuiteServer::S3(s) => s.meta(),
            AnySuiteServer::S4(s) => s.meta(),
            AnySuiteServer::S5(s) => s.meta(),
            AnySuiteServer::S6(s) => s.meta(),
            AnySuiteServer::S8(s) => s.meta(),
            AnySuiteServer::S59(s) => s.meta(),
            AnySuiteServer::S77(s) => s.meta(),
            AnySuiteServer::Ext(s) => s.meta(),
        }
    }

    /// The suite's corpus centroid — the hoarding gate's vector (T5).
    #[must_use]
    pub fn centroid(&self) -> [f32; crate::arsenal_ops::DIM] {
        match self {
            AnySuiteServer::S2(s) => s.centroid(),
            AnySuiteServer::S3(s) => s.centroid(),
            AnySuiteServer::S4(s) => s.centroid(),
            AnySuiteServer::S5(s) => s.centroid(),
            AnySuiteServer::S6(s) => s.centroid(),
            AnySuiteServer::S8(s) => s.centroid(),
            AnySuiteServer::S59(s) => s.centroid(),
            AnySuiteServer::S77(s) => s.centroid(),
            AnySuiteServer::Ext(s) => s.centroid(),
        }
    }
}

#[cfg(test)]
mod served_case_tests {
    use super::*;

    /// The exported construction (Rethink Issue 017 T2 consumes it): the
    /// criteria rendering per kind, the noul fixed rendering from an EMPTY
    /// presentation, the serialized-state envelope, and one zero-gold row
    /// per question — the exact bytes [`SuiteServer::decide_multi`] feeds
    /// `eval_case_into` (the eval face; reflex issue 070 lead 2).
    #[test]
    fn synth_served_case_renders_each_kind() {
        let opts = ["a".to_string(), "b".to_string()];
        let qs = [
            ServedQuestion {
                qid: "q_choice",
                kind: QKind::Choice,
                instructions: "pick",
                options: &opts,
            },
            ServedQuestion {
                qid: "q_score",
                kind: QKind::Score,
                instructions: "score",
                options: &opts,
            },
            ServedQuestion {
                qid: "q_noul",
                kind: QKind::Noul,
                instructions: "noul",
                options: &[],
            },
        ];
        let case = synth_served_case("the state", &qs);
        assert_eq!(case.id, "served");
        assert_eq!(case.state, serde_json::Value::String("the state".into()));
        assert_eq!(case.questions.len(), 3);
        assert_eq!(case.gold.len(), 3);
        assert!(case.gold.iter().all(|g| g.idx == 0 && g.soft.is_empty()));
        // Choice: an object of nulls in presented order (preserve_order
        // holds the insertion order).
        let criteria = |i: usize| &case.questions[i].criteria;
        let choice = criteria(0);
        let obj = choice.as_object().expect("choice criteria is an object");
        let keys: Vec<&String> = obj.keys().collect();
        assert_eq!(keys, vec![&"a".to_string(), &"b".to_string()]);
        assert!(obj.values().all(|v| v.is_null()));
        // Score: an array of the presented strings.
        let score = criteria(1);
        assert_eq!(
            score,
            &serde_json::Value::Array(vec![
                serde_json::Value::String("a".into()),
                serde_json::Value::String("b".into())
            ])
        );
        // Noul from an empty presentation: the fixed rendering speaks it.
        assert_eq!(case.questions[2].criteria, serde_json::Value::Null);
        assert_eq!(case.questions[2].qid, "q_noul");
    }

    /// Issue 022 lead 1 — the reuse form is byte-identical to a fresh
    /// construction across the SHAPES (choice unsorted, score, noul from an
    /// empty presentation) and across the TRANSITIONS the serve path can
    /// hit (steady same-set keep, set change, kind change on the same
    /// question slot, options-length change). The served bytes are pinned;
    /// the alloc savings ride on top, never through them.
    #[test]
    fn served_case_into_is_byte_identical_to_fresh_across_shapes_and_transitions() {
        // Unsorted on purpose — the keep-check is order-sensitive by law
        // (preserve_order holds the presented order).
        let ab = ["zebra".to_string(), "apple".to_string()];
        let cd = ["cherry".to_string(), "date".to_string(), "apple".to_string()];
        let one = ["only".to_string()];

        let shapes: Vec<(String, Vec<ServedQuestion<'_>>)> = vec![
            (
                "state one".into(),
                vec![
                    ServedQuestion { qid: "c", kind: QKind::Choice, instructions: "i", options: &ab },
                    ServedQuestion { qid: "s", kind: QKind::Score, instructions: "j", options: &cd },
                    ServedQuestion { qid: "n", kind: QKind::Noul, instructions: "k", options: &[] },
                ],
            ),
            // Steady state: identical inputs (the keep path).
            (
                "state one".into(),
                vec![
                    ServedQuestion { qid: "c", kind: QKind::Choice, instructions: "i", options: &ab },
                    ServedQuestion { qid: "s", kind: QKind::Score, instructions: "j", options: &cd },
                    ServedQuestion { qid: "n", kind: QKind::Noul, instructions: "k", options: &[] },
                ],
            ),
            // Set change + kind flip on the same slots + a different state.
            (
                "state two".into(),
                vec![
                    ServedQuestion { qid: "c", kind: QKind::Score, instructions: "i2", options: &one },
                    ServedQuestion { qid: "s", kind: QKind::Choice, instructions: "j2", options: &ab },
                    ServedQuestion { qid: "n", kind: QKind::Noul, instructions: "k", options: &[] },
                ],
            ),
            // Fewer questions (the questions vec shrinks-grows path).
            (
                "state three".into(),
                vec![ServedQuestion { qid: "c", kind: QKind::Choice, instructions: "i", options: &cd }],
            ),
        ];

        let mut reused = synth_served_case(&shapes[0].0, &shapes[0].1);
        for (state, qs) in shapes.iter().skip(1) {
            synth_served_case_into(state, qs, &mut reused);
            let fresh = synth_served_case(state, qs);
            assert_eq!(reused, fresh, "reuse must equal a fresh build for {state}");
            // The serialized bytes too — the eval face consumes the case,
            // and the criteria's Key/Array order is contract.
            assert_eq!(
                serde_json::to_string(&reused).unwrap(),
                serde_json::to_string(&fresh).unwrap()
            );
        }
    }
}

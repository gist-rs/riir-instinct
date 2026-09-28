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
    PostureKnobs, Seat, build_seat_engine, eval_seat, fit_posture, prepare_seat,
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
    /// A HOSTED-ONLY vessel (the minted path; lineage in the commitment).
    Vessel,
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
    pos_label: Vec<usize>,
    pos_spec: Vec<f32>,
    pos_nb: Vec<f32>,
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

    /// Boot one suite from a minted HOSTED-ONLY vessel (the P4 reader:
    /// verify → class gate → monotonic gate → decrypt → decode). The
    /// applied state comes from the host's state file (genesis at 0);
    /// the caller persists the returned [`VesselFacts`] AFTER the suite
    /// is fully up (never before — a boot failure must not advance the
    /// gate). The specialist itself is consumed by the join.
    #[cfg(feature = "vessel")]
    pub fn from_vessel(
        suite: &'static str,
        seat: Seat,
        vessel_path: &Path,
        pins: &reflexer_vessel::PinTable,
        key: &[u8; 32],
        applied: &crate::vessel::AppliedState,
        arm: Arm,
    ) -> Result<(Self, VesselFacts), String> {
        let loaded = crate::vessel::load_hosted(vessel_path, pins, key, applied)
            .map_err(|e| format!("{}: {e}", vessel_path.display()))?;
        let facts = VesselFacts {
            commitment_hex: loaded.commitment_hex.clone(),
            artifact_version: loaded.artifact_version,
            parent_commitment: loaded.parent_commitment,
        };
        let commitment16 = facts.commitment_hex[..16].to_string();
        let server = Self::from_parts(
            suite,
            seat,
            loaded.specialist,
            commitment16,
            WeightSource::Vessel,
            arm,
        )?;
        Ok((server, facts))
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
        key: &[u8; 32],
        applied: &crate::vessel::AppliedState,
        arm: Arm,
    ) -> Result<(Self, VesselFacts), String> {
        let loaded = crate::vessel::load_hosted_bytes(vessel_bytes, pins, key, applied)
            .map_err(|e| format!("vessel payload: {e}"))?;
        let facts = VesselFacts {
            commitment_hex: loaded.commitment_hex.clone(),
            artifact_version: loaded.artifact_version,
            parent_commitment: loaded.parent_commitment,
        };
        let commitment16 = facts.commitment_hex[..16].to_string();
        let server = Self::from_parts(
            suite,
            seat,
            loaded.specialist,
            commitment16,
            WeightSource::Vessel,
            arm,
        )?;
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
        // re-baseline): head-select + nb-select + ridge-select, registry
        // caps, genome off — byte-identical knobs to the arena's
        // (`run_suite_n`), so the serve path stays the arena path.
        // `ridge_select` is the cal-selected NBSVM-ridge lane (reflex
        // Bench 057): emotion arms @8, every other suite's ladder
        // declines at the arming bar (selected 0.0 — byte-identical to
        // off, reflex's full-workspace delta 0.0000). Boot cost: the
        // emotion ladder ≈ +5 s, the wide suites' ≈ +40–60 s (Bench 057's
        // fit-cost disclosure) — deterministic derivation of the PUBLISHED
        // posture, never a fork from the arena's. Genome selection stays
        // off: reflex's published bench rows predate that lane — turning
        // it on would serve a posture no published row carries.
        let knobs = PostureKnobs {
            head_select: true,
            nb_select: true,
            // option_cond rides the dep for the nb_ridge compile only
            // (upstream gap, filed reflex-side); the oc lane stays off.
            oc_select: false,
            ridge_select: true,
            genome_select: false,
            genome_accept_margin: 0.0,
            cal_select_caps: vec![],
        };
        let posture = fit_posture::<N>(suite, &seat, &knobs)?;
        let (engine, _fallbacks) =
            build_seat_engine::<N>(suite, &seat, posture.effective_cap, posture.cfg.clone())?;

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
                .map(|(li, l)| (l.clone(), (li, joined.perm[li])))
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
            engine,
            lane: HybridLane::Specialist(joined),
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
            pos_label: Vec::new(),
            pos_spec: Vec::new(),
            pos_nb: Vec::new(),
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
        let mut all_labels: Vec<Vec<usize>> = Vec::with_capacity(questions.len());
        let mut all_classes: Vec<Vec<usize>> = Vec::with_capacity(questions.len());
        let mut all_options: Vec<Vec<String>> = Vec::with_capacity(questions.len());
        for q in questions {
            // Noul questions may present the FIXED rendering by sending no
            // options at all (the decision_wire law: a noul question carries
            // no option list — [false, true] speaks it).
            let options: Vec<String> = if q.kind == QKind::Noul && q.options.is_empty() {
                vec!["false".to_string(), "true".to_string()]
            } else {
                q.options.to_vec()
            };
            if options.len() < 2 {
                return Err(format!(
                    "question {:?}: need ≥2 presented options",
                    q.qid
                ));
            }
            let mut seen = std::collections::HashSet::new();
            if !options.iter().all(|o| seen.insert(o.as_str())) {
                return Err(format!(
                    "question {:?}: duplicate presented options — the answer space would collide",
                    q.qid
                ));
            }
            let mut pos_label = Vec::with_capacity(options.len());
            let mut pos_class = Vec::with_capacity(options.len());
            if q.kind == QKind::Noul {
                if !self.perm.contains(&usize::MAX) {
                    // Named join (e.g. the positional-int noul suites): the
                    // single-question contract's law, byte-preserved — the
                    // presented names never reorder the fixed rendering,
                    // position p takes seat label p and the artifact's pair
                    // row p, and the count must be exactly the seat
                    // universe (a wider or narrower presentation has no
                    // noul space).
                    if options.len() != self.labels.len() {
                        return Err(format!(
                            "suite {}: a noul presentation carries {} options — the fixed \
                             [false, true] rendering has exactly {} (pick_index speaks that \
                             space whatever names are presented)",
                            self.suite,
                            options.len(),
                            self.labels.len()
                        ));
                    }
                    for (li, &cls) in self.perm.iter().enumerate() {
                        pos_label.push(li);
                        pos_class.push(cls);
                    }
                } else {
                    // The CONTEXT join (the arena's fill_positions law): the
                    // seat labels are the engine's domain space, disjoint
                    // from the pair — resolve NOUL_PAIR BY NAME through the
                    // key map (the artifact's unified no/yes rows; a missing
                    // row is a producer-contract break, loud).
                    if !is_fixed_noul_presentation(&options) {
                        return Err(format!(
                            "suite {}: a context noul presentation must be the fixed pair in order \
                             (or empty — the fixed rendering): {options:?} vs [\"false\", \"true\"]",
                            self.suite
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
                let all_named = options.iter().all(|k| self.key_map.contains_key(k));
                if all_named {
                    for key in &options {
                        let (li, cls) = self.key_map[key];
                        pos_label.push(li);
                        pos_class.push(cls);
                    }
                } else if !self.perm.contains(&usize::MAX)
                    && options.len() == self.perm.len()
                {
                    for (li, &cls) in self.perm.iter().enumerate() {
                        pos_label.push(li);
                        pos_class.push(cls);
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
            }
            all_labels.push(pos_label);
            all_classes.push(pos_class);
            all_options.push(options);
        }

        // The modelless lane's answers: the synthesized multi-question case
        // goes through the SAME eval path the seat eval uses
        // (eval_seat → engine_request → decide_with), so A0 here is the
        // arena's A0 byte for byte, per question.
        let case_questions: Vec<SuiteQuestion> = questions
            .iter()
            .zip(all_options.iter())
            .map(|(q, options)| SuiteQuestion {
                qid: q.qid.to_string(),
                kind: q.kind,
                instructions: q.instructions.to_string(),
                criteria: match q.kind {
                    QKind::Choice => {
                        let mut m = serde_json::Map::new();
                        for key in options {
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
                },
            })
            .collect();
        let case = SuiteCase {
            id: "served".into(),
            state: serde_json::Value::String(state.to_string()),
            questions: case_questions,
            gold: questions
                .iter()
                .map(|_| GoldAnswer {
                    idx: 0,
                    soft: vec![],
                    gold_score: None,
                })
                .collect(),
        };
        let se = eval_seat(&mut self.engine, std::slice::from_ref(&case), &[state.to_string()])?;
        let outs = &se.cases[0];
        if outs.len() != questions.len() {
            return Err(format!(
                "suite {}: the engine answered {} of {} questions — eval shape drift",
                self.suite,
                outs.len(),
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
        for (qi, _q) in questions.iter().enumerate() {
            let qo = &outs[qi];
            let pos_class = &all_classes[qi];
            let options = &all_options[qi];
            self.pos_spec.clear();
            self.pos_spec.resize(pos_class.len(), 0.0);
            self.lane
                .scores_classes_into(&self.bag, pos_class, &mut self.pos_spec);
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
                            self.pos_label.clear();
                            self.pos_label.extend_from_slice(&all_labels[qi]);
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
                };
            let pick = pick_index.map(|i| options[i].clone());
            decisions.push(ServedDecision {
                suite: self.suite,
                arm: arm.name(),
                options: options.clone(),
                pick_index,
                pick,
                probabilities,
                specialist_scores,
                confidence,
                escalated,
                abstained,
                us: u64::try_from(t0.elapsed().as_micros()).unwrap_or(u64::MAX),
            });
        }
        Ok(decisions)
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

/// The arity-erased server (the registry holds one per suite; the
/// engine arities are const-generic). S2 (Plan 003) seats the noul
/// suites' 2-label universe — prompt_injections.
pub enum AnySuiteServer {
    S2(Box<SuiteServer<2>>),
    S3(Box<SuiteServer<3>>),
    S4(Box<SuiteServer<4>>),
    S5(Box<SuiteServer<5>>),
    S6(Box<SuiteServer<6>>),
    S59(Box<SuiteServer<59>>),
    S77(Box<SuiteServer<77>>),
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

    fn boot_from_seat_arm(
        suite: &'static str,
        seat: Seat,
        winners_dir: &Path,
        manifest: &ArsenalManifest,
        arm: Arm,
    ) -> Result<Self, String> {
        let row = manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
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
            59 => seat_arm!(S59, 59),
            77 => seat_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The vessel boot: the same dispatch, the specialist from a minted
    /// HOSTED-ONLY vessel (vessel feature only). The vessel file is the
    /// row's artifact (law A5 — no filename convention behind the
    /// manifest's back).
    #[cfg(feature = "vessel")]
    #[allow(clippy::too_many_arguments)]
    pub fn boot_vessel(
        suite: &'static str,
        seat: Seat,
        vessels_dir: &Path,
        manifest: &ArsenalManifest,
        pins: &reflexer_vessel::PinTable,
        key: &[u8; 32],
        applied: &crate::vessel::AppliedState,
    ) -> Result<(Self, VesselFacts), String> {
        let arm = Self::posture_of(manifest, suite)?;
        let row = manifest
            .row(suite)
            .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
        let vessel_path = vessels_dir.join(row.artifact_file(format!("{suite}_v1.vessel")));
        macro_rules! vessel_arm {
            ($variant:ident, $n:literal) => {{
                let (server, facts) = SuiteServer::<$n>::from_vessel(
                    suite,
                    seat,
                    &vessel_path,
                    pins,
                    key,
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
            59 => vessel_arm!(S59, 59),
            77 => vessel_arm!(S77, 77),
            other => Err(format!("suite {suite}: no engine arity for {other} labels")),
        }
    }

    /// The bytes-based vessel dispatch — the swap/lazy loader's vessel
    /// entry (single-read discipline, as [`Self::boot_bytes`]).
    #[cfg(feature = "vessel")]
    #[allow(clippy::too_many_arguments)]
    pub fn boot_vessel_bytes(
        suite: &'static str,
        seat: Seat,
        vessel_bytes: &[u8],
        manifest: &ArsenalManifest,
        pins: &reflexer_vessel::PinTable,
        key: &[u8; 32],
        applied: &crate::vessel::AppliedState,
    ) -> Result<(Self, VesselFacts), String> {
        let arm = Self::posture_of(manifest, suite)?;
        macro_rules! vessel_arm {
            ($variant:ident, $n:literal) => {{
                let (server, facts) = SuiteServer::<$n>::from_vessel_bytes(
                    suite,
                    seat,
                    vessel_bytes,
                    pins,
                    key,
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
            59 => vessel_arm!(S59, 59),
            77 => vessel_arm!(S77, 77),
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
            AnySuiteServer::S59(s) => s.decide(state, options),
            AnySuiteServer::S77(s) => s.decide(state, options),
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
            AnySuiteServer::S59(s) => s.decide_multi(state, questions),
            AnySuiteServer::S77(s) => s.decide_multi(state, questions),
        }
    }

    pub fn meta(&self) -> &SuiteMeta {
        match self {
            AnySuiteServer::S2(s) => s.meta(),
            AnySuiteServer::S3(s) => s.meta(),
            AnySuiteServer::S4(s) => s.meta(),
            AnySuiteServer::S5(s) => s.meta(),
            AnySuiteServer::S6(s) => s.meta(),
            AnySuiteServer::S59(s) => s.meta(),
            AnySuiteServer::S77(s) => s.meta(),
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
            AnySuiteServer::S59(s) => s.centroid(),
            AnySuiteServer::S77(s) => s.centroid(),
        }
    }
}

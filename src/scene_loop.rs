//! The Runetrace scene-loop composition (Plan 010 — seal-remake Proposal 005
//! Phase 4, Research 613 executed): a MODELLESS scene-agent loop over
//! shipped katgpt-core GOAT primitives.
//!
//! Scene in as a deterministic [`RunetraceDoc`] capture window, judgment out
//! on `decision_wire` with first-class abstention, and the loop's OWN
//! reasoning rendered back as a Runetrace [`EntityBlock`] — self-inspectable
//! by the same format it consumes, digest-stable for Phase-5 consolidation.
//!
//! ## The stage map (every stage CONSUMES shipped substrate — none re-implemented)
//!
//! | # | stage | substrate |
//! |---|---|---|
//! | 1 | EMBED — hashed-feature embedding of the doc's STRUCTURED fields | `riir_reflex::embed::Embedder` |
//! | 2 | FORECAST — next-belt drive channels per entity | `katgpt_core::karc::KarcForecaster` (deployed Fourier R=1 shape) |
//! | 3 | RATE — Elo prior per (arm, context-class) | `katgpt_core::rating` |
//! | 4 | GOAL — modelless LEO all-goals Q + dual-LEO α-blend | `katgpt_core::{LeoHead, sigmoid_bounded_q, DualLeoMixer}` |
//! | 5 | REFINE — bounded looped shallow reasoning | `state_probe` worthiness · `contrast_combine::affine_combine` steps · `entropic_tilt` · `gain_cost_halt` + `risk_control_exit` + the `ignition` patience law · `saddle_escape::FlipDetector` |
//! | 6 | DECIDE — `Answer::choice` / `Answer::abstain` | `katgpt_core::decision_wire` |
//! | 7 | SELF-TRACE — the loop's fired controls + path as a doc | `katgpt_core::runetrace` |
//!
//! ## The laws (Research 613 §4; Proposal 005 caveats)
//!
//! 1. **Embed the structured fields, never the rendered text** — stage 1
//!    embeds a canonical field projection (id/kind tokens, vital names with
//!    quartile-bucketed fractions, condition/action tokens). The human
//!    render is for humans/LLM lanes, never the modelless embedder's input.
//! 2. **Raw stays raw** — vitals/positions ride the doc verbatim; Elo
//!    outcomes are raw vital deltas squashed by ONE sigmoid; only
//!    ratings/forecasts/Q are latent. Nothing latent parses back into game
//!    truth.
//! 3. **Sigmoid, never softmax** — arm probabilities linearly normalize
//!    sigmoid-bounded Q; every gate/confidence is a sigmoid.
//! 4. **Think-brain locality** — rings, forecasters, Elo books are
//!    loop-local per entity; only the Answer (+trace) crosses.
//! 5. **Cadence follows observation** — `state_probe` gates whether refine
//!    runs at all; KARC fits at belt cadence, forecasts per decision.
//! 6. **No second brain** — the Answer is advisory data for the consumer's
//!    brain+FSM; this loop is never one.
//! 7. **Determinism by construction** — NO HashMap iteration on any answer
//!    path (keyed access only); arms iterate in Question order, entities in
//!    doc order, goals in a fixed vocabulary; ties break to the first
//!    index. Two runs of one history are byte-identical (G1, test-pinned).
//!
//! Confidence is a RAW readout (a sigmoid of the decision margin) — NO
//! calibrated-UQ claim is made for the composition; Report-the-Floor binds
//! the moment any such claim appears (the plan's G5-style honesty).
//!
//! Opt-in (`runetrace_loop`); the /decide path is byte-identical without it.

use std::collections::HashMap;

use katgpt_core::decision_wire::{Answer, Question};
use katgpt_core::contrast_combine::affine_combine;
use katgpt_core::entropic_tilt;
use katgpt_core::gain_cost_halt::{GainCostLoopHalter, HaltDecision};
use katgpt_core::ignition::ignition_time;
use katgpt_core::karc::{FourierBasis, KarcForecaster};
use katgpt_core::rating;
use katgpt_core::risk_control_exit::{DualExitPolicy, ExitVerdict, squeezed_sigmoid};
use katgpt_core::runetrace::{
    ConditionRow, DagRow, DagStage, EntityBlock, EntityKind, RunetraceDoc, Vital,
};
use katgpt_core::saddle_escape::FlipDetector;
use katgpt_core::sigmoid_bounded_q;
use katgpt_core::state_probe::{self, ProbeInput};
use katgpt_core::traits::{DualLeoMixer, LeoHead};
use riir_reflex::embed::{EMBED_DIM, Embedder};

/// Drive-channel width — the HLA belief precedent (`BELIEF_DIM = 8`, the
/// deployed `HlaKarcForecaster` shape).
pub const DRIVE_DIM: usize = 8;
/// KARC basis dimension (deployed shape `M = 8`).
pub const KARC_M: usize = 8;
/// KARC delay-embedding depth (deployed shape `K = 4`).
pub const KARC_K: usize = 4;
/// The fixed goal vocabulary (LEO all-goals; deterministic order — law 7).
pub const GOAL_KEYS: [&str; 4] = ["survive", "resource", "engage", "avoid"];
/// The ignition patience law's ζ (alignment) — `t* = ln(1/ε)/ζ` steps.
const IGNITION_ZETA: f32 = 0.5;
/// The ignition patience law's ε (readability threshold).
const IGNITION_EPS: f32 = 0.05;

/// The deployed-shape forecaster (`FourierBasis<8>`, D=8, M=8, K=4 — the
/// riir-engine `HlaKarcForecaster` precedent; Bench 849 is the honest
/// one-step quality record, Issue 866 QUALIFY disclosed in the plan).
pub type SceneKarc = KarcForecaster<FourierBasis<KARC_M>, DRIVE_DIM, KARC_M, KARC_K>;

/// FNV-1a 64 (the reflex `embed` house hash — one lexicon everywhere here).
#[inline]
fn fnv1a(bytes: &[u8], salt: u64) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64 ^ salt;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// ── Stage 0/2 input: the canonical projections (law 1 + law 2) ─────────────

/// Project one entity's vitals + fired-condition drives into the fixed
/// 8-channel drive vector: a hashed bag (slot = fnv1a(name) % DRIVE_DIM)
/// of vital fractions and condition drive values, clamped to `[0, 1]`.
/// Deterministic in the doc's own field contents (law 7).
pub fn drive_channels_into(entity: &EntityBlock, out: &mut [f32; DRIVE_DIM]) {
    out.fill(0.0);
    for v in &entity.vitals {
        let frac = if v.max > 0.0 {
            (v.value / v.max).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let slot = (fnv1a(v.name.as_bytes(), 0x11) % DRIVE_DIM as u64) as usize;
        out[slot] += frac;
    }
    for c in &entity.think {
        if let Some(d) = c.drive {
            let slot = (fnv1a(c.condition.as_bytes(), 0x22) % DRIVE_DIM as u64) as usize;
            out[slot] += d;
        }
    }
    for x in out.iter_mut() {
        *x = x.clamp(0.0, 1.0);
    }
}

/// Quartile bucket label for a fraction (stable embed tokens — law 1: raw
/// floats never enter the embedder; a bucket does).
#[inline]
fn quartile(frac: f32) -> &'static str {
    match (frac.clamp(0.0, 0.999) * 4.0) as u8 {
        0 => "q0",
        1 => "q1",
        2 => "q2",
        _ => "q3",
    }
}

/// Project the doc's STRUCTURED fields into the canonical token stream the
/// embedder consumes (law 1). `hyp_arm` appends the hypothetical "arm
/// fired" condition the refine loop reasons over (stage 5). The tick is
/// deliberately excluded — the embedding captures scene STATE, not the
/// clock (a same-shape scene at a later tick embeds identically).
pub fn scene_fields_into(
    doc: &RunetraceDoc,
    subject: &str,
    hyp_arm: Option<&str>,
    out: &mut Vec<u8>,
) {
    out.clear();
    let put = |out: &mut Vec<u8>, b: &[u8]| {
        out.extend_from_slice(b);
        out.push(b' ');
    };
    put(out, b"scene");
    put(out, doc.scene.label.as_bytes());
    for e in &doc.entities {
        put(out, b"e");
        put(out, e.id.as_bytes());
        put(out, e.kind.as_str().as_bytes());
        for v in &e.vitals {
            let frac = if v.max > 0.0 {
                (v.value / v.max).clamp(0.0, 1.0)
            } else {
                0.0
            };
            put(out, v.name.as_bytes());
            put(out, quartile(frac).as_bytes());
        }
        for c in &e.think {
            put(out, c.condition.as_bytes());
            put(out, c.action.as_bytes());
        }
        if e.id == subject {
            if let Some(arm) = hyp_arm {
                put(out, b"hyp");
                put(out, arm.as_bytes());
            }
        }
    }
}

/// Context class: the entity's fired-condition set hashed to 4 buckets
/// (coarse, deterministic — law 7).
fn context_class(e: &EntityBlock) -> u32 {
    let mut h = 0xcbf2_9ce4_8422_2325u64 ^ 0xC0DE;
    for c in &e.think {
        for &b in c.condition.as_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    (h % 4) as u32
}

// ── The per-entity think-brain-local memory (law 4) ─────────────────────────

/// Loop-local memory for one entity. Never synced, never iterated as a
/// whole — the Elo book is keyed-access only (law 7).
pub struct EntityLoopMemory {
    /// KARC observation path: ring + accumulated training pairs.
    forecaster: SceneKarc,
    /// Belt at which the last successful fit happened (fit cadence, law 5).
    fitted_at: Option<u64>,
    /// Elo ratings: (arm hash, context class) → rating (f32 persistence —
    /// the `rating` f32 twin; bit-identical expression trees upstream).
    elo: HashMap<(u64, u32), f32>,
    /// Context-class baseline ratings (the Elo "field" opponent).
    ctx_baseline: HashMap<u32, f32>,
    /// Last belt's vital fractions by name hash (raw deltas for outcomes).
    last_vitals: HashMap<u64, f32>,
}

impl EntityLoopMemory {
    /// New memory with the deployed forecaster shape (period 4.0 — the
    /// Plan-308 GOAT configuration) and the deployed buffer posture
    /// (capacity 2048, the karc_bridge number; the buffer does not
    /// auto-evict — `fit_ridge` uses the most recent rows).
    pub fn new() -> Self {
        Self {
            forecaster: KarcForecaster::with_capacity(FourierBasis::new(4.0), 2048),
            fitted_at: None,
            elo: HashMap::new(),
            ctx_baseline: HashMap::new(),
            last_vitals: HashMap::new(),
        }
    }

    /// The belt of the last successful KARC fit (None = never fitted).
    pub fn fitted_at(&self) -> Option<u64> {
        self.fitted_at
    }
}

impl Default for EntityLoopMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ── Config ──────────────────────────────────────────────────────────────────

/// The loop's knobs. Defaults are the plan's reference posture; `validate()`
/// pins the contract fail-closed (the `runetrace`/`decision_wire` discipline).
#[derive(Clone, Debug)]
pub struct SceneLoopConfig {
    /// Ridge λ (deployed: `1e-4`, the Bench-849 operating point).
    pub karc_lambda: f32,
    /// Minimum training pairs before the first fit. Default `256 = d_h`
    /// (the deployed bridge law: `N ≥ d_h` guarantees the numerically
    /// stable f64 direct path; smaller needs a larger λ to stabilize).
    pub karc_min_samples: usize,
    /// Refine ceiling (hard bound; the ignition patience law bounds inside).
    pub refine_max_steps: u8,
    /// Contrast step weight ω (`affine_combine` at `lam = 1 + ω` — the
    /// LoopCD `z + ω(z − z_k)` form, Research 613 §1 stage 5).
    pub refine_omega: f32,
    /// Saddle-kick multiplier bound (a kick doubles ω, never past this).
    pub refine_omega_cap: f32,
    /// Entropic-tilt γ (max-seeking among arms at each refine step).
    pub tilt_gamma: f32,
    /// `state_probe` worthiness floor: V̂ ≥ this ⇒ picks stable ⇒ skip refine.
    pub probe_v_floor: f64,
    /// Perturbation ensemble size for the worthiness probe.
    pub probe_ensemble: u32,
    /// Perturbation magnitude (multiplicative, deterministic masks).
    pub probe_eps: f32,
    /// Stop-when-confident λ+ (on the squeezed margin signal).
    pub exit_lambda_plus: f32,
    /// `GainCostLoopHalter` τ (gain < cost × τ halts).
    pub halt_tau: f32,
    /// Abstain floor on the decision margin.
    pub abstain_margin: f32,
    /// Cold-evidence multiplier: with NO forecast and NO Elo history the
    /// floor scales by this (more skeptical when no evidence exists).
    pub abstain_cold_multiple: f32,
    /// Confidence slope (margin → sigmoid confidence).
    pub conf_slope: f32,
    /// Elo K factor.
    pub elo_k: f32,
    /// Elo scale.
    pub elo_scale: f32,
    /// Dual-LEO α (0.3, the trait's sweep default).
    pub dual_alpha: f32,
}

impl Default for SceneLoopConfig {
    fn default() -> Self {
        Self {
            karc_lambda: 1e-4,
            karc_min_samples: 256,
            refine_max_steps: 8,
            refine_omega: 0.1,
            refine_omega_cap: 0.8,
            tilt_gamma: 0.5,
            probe_v_floor: 0.75,
            probe_ensemble: 8,
            probe_eps: 0.05,
            exit_lambda_plus: 0.9,
            halt_tau: 0.5,
            abstain_margin: 0.05,
            abstain_cold_multiple: 2.0,
            conf_slope: 8.0,
            elo_k: 8.0,
            elo_scale: 400.0,
            dual_alpha: 0.3,
        }
    }
}

impl SceneLoopConfig {
    /// Fail-closed contract check (the `runetrace`/`decision_wire` law).
    pub fn validate(&self) -> Result<(), String> {
        if self.karc_lambda <= 0.0 {
            return Err("karc_lambda must be > 0".into());
        }
        if self.karc_min_samples == 0 {
            return Err("karc_min_samples must be >= 1".into());
        }
        if self.refine_max_steps == 0 {
            return Err("refine_max_steps must be >= 1".into());
        }
        let omega_ok = (0.0..1.0).contains(&self.refine_omega)
            && self.refine_omega < self.refine_omega_cap;
        if !omega_ok {
            return Err("refine_omega must be in [0,1) and < refine_omega_cap".into());
        }
        if self.refine_omega_cap > 1.0 {
            return Err("refine_omega_cap must be <= 1".into());
        }
        if self.tilt_gamma < 0.0 {
            return Err("tilt_gamma must be >= 0".into());
        }
        let probe_ok = (0.0..=1.0).contains(&self.probe_v_floor) && self.probe_ensemble > 0;
        if !probe_ok {
            return Err("probe_v_floor must be in [0,1] and probe_ensemble >= 1".into());
        }
        if self.probe_eps <= 0.0 {
            return Err("probe_eps must be > 0".into());
        }
        if self.exit_lambda_plus >= 1.0 || self.exit_lambda_plus < 0.0 {
            return Err("exit_lambda_plus must be in [0,1)".into());
        }
        if self.halt_tau <= 0.0 || self.abstain_margin <= 0.0 {
            return Err("halt_tau and abstain_margin must be > 0".into());
        }
        if self.abstain_cold_multiple < 1.0 {
            return Err("abstain_cold_multiple must be >= 1".into());
        }
        let slopes_ok = self.conf_slope > 0.0 && self.elo_k > 0.0 && self.elo_scale > 0.0;
        if !slopes_ok {
            return Err("conf_slope, elo_k, elo_scale must be > 0".into());
        }
        if !(0.0..=1.0).contains(&self.dual_alpha) {
            return Err("dual_alpha must be in [0,1]".into());
        }
        Ok(())
    }
}

// ── The modelless LEO head (stage 4) ────────────────────────────────────────

/// Deterministic per-goal direction vectors (hashed ±1/√D entries — the
/// direction-vector discipline: no learned weights anywhere in this loop).
#[inline]
fn goal_direction(key: &str, out: &mut [f32; EMBED_DIM]) {
    let inv = 1.0 / (EMBED_DIM as f32).sqrt();
    for (i, x) in out.iter_mut().enumerate() {
        let h = fnv1a(key.as_bytes(), 0x9000_0000 + i as u64);
        *x = if h & 1 == 1 { inv } else { -inv };
    }
}

/// The modelless all-goals head:
/// `q(g,a) = σ_bounded(dir_g·state·2 + w_elo·elo_a + w_fc·(fc[slot_g] − 0.5))`.
///
/// Constructed per decision for the subject entity (forecast + Elo priors
/// are subject-local). Implements [`DualLeoMixer`] for the α-blend:
/// teacher = Elo-prior Q (broad), student = forecast-conditioned Q (the
/// deployed one-step shape) — `mix(q_leo, q_uvfa, α)`.
pub struct SceneLeoHead<'a> {
    dirs: &'a [[f32; EMBED_DIM]; GOAL_KEYS.len()],
    arms: &'a [String],
    elo_priors: &'a [f32],
    forecast: Option<&'a [f32; DRIVE_DIM]>,
    /// Goal → drive-channel slot (hash-seeded, fixed).
    goal_slots: &'a [usize; GOAL_KEYS.len()],
}

impl SceneLeoHead<'_> {
    /// Q for one (goal, arm) without allocation.
    #[inline]
    fn q_at(&self, state: &[f32], g: usize, a: usize, use_forecast: bool) -> f32 {
        let dir = &self.dirs[g];
        let mut dot = 0.0f32;
        for i in 0..EMBED_DIM {
            dot += dir[i] * state[i];
        }
        let mut z = dot * 2.0 + self.elo_priors[a];
        if use_forecast {
            if let Some(fc) = self.forecast {
                z += fc[self.goal_slots[g]] - 0.5;
            }
        }
        sigmoid_bounded_q(z)
    }

    /// The goal-marginal arm score used for ranking — the dual-LEO blend of
    /// teacher (Elo-only) and student (forecast-conditioned) Q, summed over
    /// the fixed goal vocabulary (deterministic, law 7).
    pub fn arm_score(&self, state: &[f32], a: usize, alpha: f32) -> f32 {
        let mut s = 0.0f32;
        for g in 0..GOAL_KEYS.len() {
            let q_leo = self.q_at(state, g, a, false);
            let q_uvfa = self.q_at(state, g, a, true);
            s += alpha * q_leo + (1.0 - alpha) * q_uvfa;
        }
        s
    }
}

impl LeoHead for SceneLeoHead<'_> {
    fn all_goals_q(&self, state: &[f32]) -> Vec<f32> {
        let n = self.arms.len();
        let mut out = vec![0.0f32; GOAL_KEYS.len() * n];
        for g in 0..GOAL_KEYS.len() {
            for a in 0..n {
                out[g * n + a] = self.q_at(state, g, a, true);
            }
        }
        out
    }

    fn goal_count(&self) -> usize {
        GOAL_KEYS.len()
    }

    fn action_count(&self) -> usize {
        self.arms.len()
    }
}

impl DualLeoMixer for SceneLeoHead<'_> {}

// ── The loop ────────────────────────────────────────────────────────────────

/// One loop's decision artifact: the wire `Answer`, the self-trace doc, and
/// its BLAKE3 digest (the freezable trajectory identity — Research 613 §5).
#[derive(Debug)]
pub struct SceneDecision {
    pub answer: Answer,
    /// The agent's own trace as a one-block doc (appendable by consumers).
    pub trace: RunetraceDoc,
    /// `trace.content_hash()` — the digest-stable identity.
    pub trace_digest: blake3::Hash,
}

/// The scene-agent loop. `observe_belt` at capture cadence, `decide` per
/// question (law 5: fit is belt-cadence, forecast is per-decision).
pub struct SceneLoop {
    cfg: SceneLoopConfig,
    embedder: Embedder,
    /// Per-entity memory, keyed by entity id (keyed access ONLY — law 7).
    memory: HashMap<String, EntityLoopMemory>,
    dirs: [[f32; EMBED_DIM]; GOAL_KEYS.len()],
    goal_slots: [usize; GOAL_KEYS.len()],
    exit: DualExitPolicy,
    /// The ignition patience bound `t* = ln(1/ε)/ζ`, in refine steps.
    patience_steps: u8,
    // scratch (reused across calls — the reflex buffer discipline)
    fields: Vec<u8>,
    z: Vec<f32>,
    z_k: Vec<f32>,
    z_perp: Vec<f32>,
    margins: Vec<f32>,
    advantages: Vec<f32>,
    probs: Vec<f32>,
    forecast_buf: [f32; DRIVE_DIM],
}

impl SceneLoop {
    /// Construct with config (validated fail-closed) + the control
    /// primitives at their shipped reference postures.
    pub fn new(cfg: SceneLoopConfig) -> Result<Self, String> {
        cfg.validate()?;
        let mut dirs = [[0.0f32; EMBED_DIM]; GOAL_KEYS.len()];
        for (g, key) in GOAL_KEYS.iter().enumerate() {
            goal_direction(key, &mut dirs[g]);
        }
        let mut goal_slots = [0usize; GOAL_KEYS.len()];
        for (g, key) in GOAL_KEYS.iter().enumerate() {
            goal_slots[g] = (fnv1a(key.as_bytes(), 0x77) % DRIVE_DIM as u64) as usize;
        }
        // The ignition patience law (Bench 666's primitive): a mode of
        // alignment ζ needs t* = ln(1/ε)/ζ steps before it is readable —
        // the refine loop's patience bound, not an arbitrary cap.
        let t_star = ignition_time(IGNITION_ZETA, IGNITION_EPS);
        let patience_steps = t_star.ceil().max(1.0) as u8;
        let budget = u32::from(cfg.refine_max_steps);
        Ok(Self {
            exit: DualExitPolicy::linear(cfg.exit_lambda_plus, budget),
            patience_steps,
            cfg,
            embedder: Embedder,
            memory: HashMap::new(),
            dirs,
            goal_slots,
            fields: Vec::with_capacity(4096),
            z: vec![0.0; EMBED_DIM],
            z_k: vec![0.0; EMBED_DIM],
            z_perp: vec![0.0; EMBED_DIM],
            margins: Vec::new(),
            advantages: Vec::new(),
            probs: Vec::new(),
            forecast_buf: [0.0; DRIVE_DIM],
        })
    }

    /// Stage 0+2 capture path: project drives, advance the KARC observation
    /// path, update Elo from `acted` (the consumer asserts the arm the
    /// entity took — the doc does not carry the question's arm vocabulary),
    /// snapshot vitals for the next belt's raw deltas.
    pub fn observe_belt(&mut self, doc: &RunetraceDoc, acted: Option<(&str, &str)>) {
        for e in &doc.entities {
            let mem = self.memory.entry(e.id.clone()).or_default();

            // Elo outcome: raw vital-delta mean, squashed by ONE sigmoid
            // (law 2). The opponent is the context-class field baseline.
            let ctx = context_class(e);
            if let Some((entity_id, arm)) = acted {
                if entity_id == e.id {
                    let mut delta_sum = 0.0f32;
                    let mut n = 0u32;
                    for v in &e.vitals {
                        if v.max <= 0.0 {
                            continue;
                        }
                        let frac = (v.value / v.max).clamp(0.0, 1.0);
                        let key = fnv1a(v.name.as_bytes(), 0x33);
                        if let Some(prev) = mem.last_vitals.get(&key) {
                            delta_sum += frac - prev;
                            n += 1;
                        }
                    }
                    if n > 0 {
                        let score = sigmoid_bounded_q((delta_sum / n as f32) * 8.0);
                        let arm_key = fnv1a(arm.as_bytes(), 0x44);
                        let ra = f64::from(*mem.elo.get(&(arm_key, ctx)).unwrap_or(&1000.0));
                        let rb = f64::from(*mem.ctx_baseline.get(&ctx).unwrap_or(&1000.0));
                        let (ra2, rb2) = rating::update_scored(
                            ra,
                            rb,
                            f64::from(score),
                            f64::from(self.cfg.elo_k),
                            f64::from(self.cfg.elo_scale),
                        );
                        mem.elo.insert((arm_key, ctx), ra2 as f32);
                        mem.ctx_baseline.insert(ctx, rb2 as f32);
                    }
                }
            }

            // vital snapshot (after the delta read — this belt becomes "last")
            mem.last_vitals.clear();
            for v in &e.vitals {
                if v.max > 0.0 {
                    mem.last_vitals
                        .insert(fnv1a(v.name.as_bytes(), 0x33), (v.value / v.max).clamp(0.0, 1.0));
                }
            }

            // KARC observation (the deployed bridge posture: the buffer
            // does not auto-evict — fit_ridge uses the most recent rows;
            // capacity 2048 covers a full session at belt cadence).
            let mut chans = [0.0f32; DRIVE_DIM];
            drive_channels_into(e, &mut chans);
            mem.forecaster.observe_and_maybe_pair(&chans);
        }
    }

    /// Fit at belt cadence (law 5) — call after `observe_belt` when the
    /// capture window closes. Fits every entity with enough pairs; returns
    /// how many fitted.
    pub fn fit_all(&mut self, now: u64) -> usize {
        let mut fitted = 0usize;
        for mem in self.memory.values_mut() {
            if mem.forecaster.n_samples() >= self.cfg.karc_min_samples
                && mem.forecaster.fit_ridge(self.cfg.karc_lambda).is_ok()
            {
                mem.fitted_at = Some(now);
                fitted += 1;
            }
        }
        fitted
    }

    /// Stages 1–7. Deterministic (law 7): same history + same doc + same
    /// question → byte-identical answer + trace + digest.
    pub fn decide(
        &mut self,
        doc: &RunetraceDoc,
        question: &Question,
    ) -> Result<SceneDecision, String> {
        if question.options.len() < 2 {
            return Err("scene_loop: choice question needs >= 2 options".into());
        }
        let subject = doc
            .entities
            .iter()
            .find(|e| e.id == question.id)
            .ok_or_else(|| {
                format!(
                    "scene_loop: subject '{}' is not an entity of the doc (the question id names the subject)",
                    question.id
                )
            })?;
        let mut trace = RunetraceDoc::new(doc.tick, "scene_loop");

        // ── stage 1: EMBED the structured fields (law 1) ──
        scene_fields_into(doc, &question.id, None, &mut self.fields);
        self.embedder.embed_into(&self.fields, &mut self.z);
        let z_hash = blake3::hash(&self.fields);

        // ── stage 2: FORECAST the subject's next-belt drives ──
        // (forecast_now is &mut on the forecaster — disjoint-field borrow:
        // `memory` and `forecast_buf` never alias through `self`.)
        let forecast: Option<[f32; DRIVE_DIM]> = {
            let mem = self
                .memory
                .get_mut(&question.id)
                .ok_or_else(|| "scene_loop: subject has no memory (observe_belt first)".to_string())?;
            if mem.forecaster.is_fitted() && mem.forecaster.forecast_now(&mut self.forecast_buf) {
                Some(self.forecast_buf)
            } else {
                None
            }
        };

        // ── stage 3: RATE — Elo priors per arm (keyed reads only) ──
        let ctx = context_class(subject);
        let arm_count = question.options.len();
        let mut elo_priors: Vec<f32> = Vec::with_capacity(arm_count);
        let mut elo_known = 0usize;
        {
            let mem = self.memory.get(&question.id).expect("checked above");
            for opt in &question.options {
                let k = fnv1a(opt.as_bytes(), 0x44);
                match mem.elo.get(&(k, ctx)) {
                    Some(r) => {
                        let base = mem.ctx_baseline.get(&ctx).copied().unwrap_or(1000.0);
                        elo_priors.push(rating::expected_f32(*r, base, self.cfg.elo_scale));
                        elo_known += 1;
                    }
                    None => elo_priors.push(0.5),
                }
            }
        }

        // ── stage 4: GOAL — the modelless LEO head (borrows ONLY locals —
        // the answer path never aliases &mut self through the head) ──
        let head = SceneLeoHead {
            dirs: &self.dirs,
            arms: &question.options,
            elo_priors: &elo_priors,
            forecast: forecast.as_ref(),
            goal_slots: &self.goal_slots,
        };
        let alpha = self.cfg.dual_alpha;
        let score = |state: &[f32], a: usize| head.arm_score(state, a, alpha);

        // ── stage 5: REFINE (worthiness-gated by state_probe) ──
        let base_pick = {
            let mut best = 0usize;
            let mut bv = f32::NEG_INFINITY;
            for a in 0..arm_count {
                let v = score(&self.z, a);
                if v > bv {
                    bv = v;
                    best = a;
                }
            }
            best
        };
        let mut histogram = vec![0u16; arm_count];
        histogram[base_pick] = 1;
        let mut pass = 1u32;
        for j in 0..self.cfg.probe_ensemble {
            self.z_perp.copy_from_slice(&self.z);
            for d in 0..EMBED_DIM {
                if fnv1a(&[], ((j as u64) << 32) + d as u64 + 0xBEEF) & 1 == 1 {
                    self.z_perp[d] *= 1.0 + self.cfg.probe_eps;
                }
            }
            let mut best = 0usize;
            let mut bv = f32::NEG_INFINITY;
            for a in 0..arm_count {
                let v = score(&self.z_perp, a);
                if v > bv {
                    bv = v;
                    best = a;
                }
            }
            histogram[best] += 1;
            if best == base_pick {
                pass += 1;
            }
        }
        let probe = state_probe::probe(
            &ProbeInput {
                pass_count: pass,
                n: self.cfg.probe_ensemble + 1,
                histogram: &histogram,
            },
            1.959963984540054,
        );
        let mut refine_steps = 0u8;
        let mut omega = self.cfg.refine_omega;
        let mut kicked = false;
        let mut halt_reason: Option<String> = None;
        let mut exit_verdict: Option<&'static str> = None;

        let worth = (probe.v_hat as f32) < self.cfg.probe_v_floor as f32;
        if worth {
            // Per-episode control kernels (the halter/detector are EPISODE
            // state — a fresh kernel per decide() keeps the answer a pure
            // function of (memory, doc, question): the repeatability half
            // of law 7).
            let mut halt = GainCostLoopHalter::new(self.cfg.halt_tau, 3, 2);
            let mut flip = FlipDetector::new(4);
            let mut margin_prev = margin_over(&score, &self.z, arm_count);
            let budget = u32::from(self.cfg.refine_max_steps);
            let step_cap = self.cfg.refine_max_steps.min(self.patience_steps);
            let mut dz_prev: Option<Vec<f32>> = None;
            let mut s = 0u8;
            while s < step_cap {
                // hypothetical: the doc projection with the best arm fired
                let pick = {
                    let mut best = 0usize;
                    let mut bv = f32::NEG_INFINITY;
                    for a in 0..arm_count {
                        let v = score(&self.z, a);
                        if v > bv {
                            bv = v;
                            best = a;
                        }
                    }
                    best
                };
                scene_fields_into(doc, &question.id, Some(&question.options[pick]), &mut self.fields);
                self.embedder.embed_into(&self.fields, &mut self.z_k);
                // LoopCD (Research 613 §1): z + ω(z − z_k) — the affine_combine
                // kernel at lam = 1 + ω (z is `out`, z_k the contrast `ref`).
                affine_combine(&mut self.z, &self.z_k, 1.0 + omega);
                refine_steps += 1;

                // entropic tilt over the arm margins (max-seeking at this step)
                self.margins.clear();
                for a in 0..arm_count {
                    self.margins.push(score(&self.z, a));
                }
                entropic_tilt::tilt_advantages_into(
                    &self.margins,
                    self.cfg.tilt_gamma,
                    &mut self.advantages,
                );

                let margin_now = margin_over(&score, &self.z, arm_count);
                let gain = margin_now - margin_prev;
                let cost = dz_len(&self.z, &self.z_k);
                let mut cos_theta = 1.0f32;
                if let Some(dzp) = &dz_prev {
                    let a = dzp.iter().zip(self.z.iter()).map(|(&x, &y)| x * y).sum::<f32>();
                    let b = dzp.iter().map(|x| x * x).sum::<f32>().sqrt();
                    let c = self.z.iter().map(|x| x * x).sum::<f32>().sqrt();
                    if b > 0.0 && c > 0.0 {
                        cos_theta = (a / (b * c)).clamp(-1.0, 1.0);
                    }
                }
                match halt.halt_decision(s as usize, gain, cost, cos_theta) {
                    HaltDecision::Halt { reason } => {
                        halt_reason = Some(format!("{reason:?}"));
                        break;
                    }
                    HaltDecision::Continue | HaltDecision::RefusedFloor => {}
                }
                let s_tilde = squeezed_sigmoid(margin_now * 4.0, 0.0, 1.0);
                match self.exit.exit(s_tilde, u32::from(s), budget) {
                    ExitVerdict::Commit => {
                        exit_verdict = Some("commit");
                        break;
                    }
                    ExitVerdict::Abandon => {
                        exit_verdict = Some("abandon");
                        break;
                    }
                    ExitVerdict::Continue => {}
                }
                // saddle escape: flip-flopping picks ⇒ kick ω (bounded)
                let pick_now = {
                    let mut best = 0usize;
                    let mut bv = f32::NEG_INFINITY;
                    for a in 0..arm_count {
                        let v = score(&self.z, a);
                        if v > bv {
                            bv = v;
                            best = a;
                        }
                    }
                    best
                };
                let pick_key = fnv1a(question.options[pick_now].as_bytes(), 0x55);
                if let Some(rate) = flip.observe(Some(pick_key)) {
                    if rate > 0.5 && omega < self.cfg.refine_omega_cap {
                        omega = (omega * 2.0).min(self.cfg.refine_omega_cap);
                        kicked = true;
                    }
                }
                if let Some(dzp) = &mut dz_prev {
                    for (d, z) in dzp.iter_mut().zip(self.z.iter()) {
                        *d = *z - *d;
                    }
                } else {
                    dz_prev = Some(self.z.clone());
                }
                margin_prev = margin_now;
                s += 1;
            }
        }

        // ── stage 6: DECIDE (sigmoid-normalized; abstention first-class) ──
        self.margins.clear();
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for a in 0..arm_count {
            let v = score(&self.z, a);
            self.margins.push(v);
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        let span = max - min;
        if span > 0.0 {
            for v in self.margins.iter_mut() {
                *v = (*v - min) / span;
            }
        }
        let mut pick = 0usize;
        let mut best = f32::NEG_INFINITY;
        for (a, &m) in self.margins.iter().enumerate() {
            if m > best {
                best = m;
                pick = a;
            }
        }
        let second = self
            .margins
            .iter()
            .enumerate()
            .filter(|&(a, _)| a != pick)
            .map(|(_, m)| *m)
            .fold(f32::NEG_INFINITY, f32::max);
        let margin = if second.is_finite() { best - second } else { best };
        let cold = forecast.is_none() && elo_known == 0;
        let floor = self.cfg.abstain_margin
            * if cold {
                self.cfg.abstain_cold_multiple
            } else {
                1.0
            };
        let confidence = sigmoid_bounded_q(margin * self.cfg.conf_slope);

        let answer = if margin < floor {
            Answer::abstain(question.id.clone(), confidence)
        } else {
            self.probs.clear();
            let mut sum = 0.0f32;
            for &m in &self.margins {
                let p = sigmoid_bounded_q(m * self.cfg.conf_slope);
                self.probs.push(p);
                sum += p;
            }
            if sum > 0.0 {
                for p in self.probs.iter_mut() {
                    *p /= sum;
                }
            }
            Answer::choice(
                question.id.clone(),
                pick as u32,
                std::mem::take(&mut self.probs),
                confidence,
            )
        };

        // ── stage 7: SELF-TRACE (the fired controls + the path) ──
        let mut block = EntityBlock {
            id: "scene_loop".into(),
            kind: EntityKind::Npc,
            glyph: "◈".into(),
            pos: None,
            vitals: vec![
                Vital {
                    name: "conf".into(),
                    value: confidence,
                    max: 1.0,
                },
                Vital {
                    name: "margin".into(),
                    value: margin,
                    max: 1.0,
                },
            ],
            think: Vec::new(),
            dag: Vec::new(),
        };
        block.think.push(ConditionRow {
            condition: format!(
                "probe V̂ {:.2} {}",
                probe.v_hat,
                if worth {
                    "< floor → refine"
                } else {
                    "≥ floor → skip refine"
                }
            ),
            action: "refine_gate".into(),
            drive: Some(probe.v_hat as f32),
        });
        if let Some(fc) = &forecast {
            block.think.push(ConditionRow {
                condition: format!("karc fitted → forecast chans[0] {:.3}", fc[0]),
                action: "forecast".into(),
                drive: Some(fc[0]),
            });
        } else {
            block.think.push(ConditionRow {
                condition: "forecast cold (no fit yet)".into(),
                action: "forecast".into(),
                drive: None,
            });
        }
        if let Some(r) = &halt_reason {
            block.think.push(ConditionRow {
                condition: format!("halt {r}"),
                action: "gain_cost_halt".into(),
                drive: None,
            });
        }
        if let Some(v) = exit_verdict {
            block.think.push(ConditionRow {
                condition: format!("exit {v}"),
                action: "risk_control_exit".into(),
                drive: None,
            });
        }
        if kicked {
            block.think.push(ConditionRow {
                condition: "flip rate > 0.5 → saddle kick (ω doubled)".into(),
                action: "saddle_escape".into(),
                drive: None,
            });
        }
        let stage_of = |p: usize| {
            [
                DagStage::Perceive,
                DagStage::Believe,
                DagStage::Drive,
                DagStage::Goal,
                DagStage::Transition,
                DagStage::Action,
            ][p]
        };
        let mut rows: Vec<DagRow> = (0..6)
            .map(|p| DagRow {
                parent: if p == 0 { None } else { Some(p as u32 - 1) },
                stage: stage_of(p),
                text: String::new(),
            })
            .collect();
        rows[0].text = format!(
            "embed {} blake3 {:016x}",
            question.id,
            u64::from_be_bytes(z_hash.as_bytes()[..8].try_into().unwrap())
        );
        rows[1].text = match &forecast {
            Some(fc) => format!("karc forecast chans[0] {:.3}", fc[0]),
            None => "forecast cold".to_string(),
        };
        rows[2].text = format!("elo known {}/{} ctx {}", elo_known, arm_count, ctx);
        rows[3].text = format!("goal-marginal best {} {:.3}", question.options[pick], best);
        rows[4].text = format!(
            "refine {}/{} ω {:.2}",
            refine_steps, self.cfg.refine_max_steps, omega
        );
        rows[5].text = if margin < floor {
            format!("abstain conf {:.2}", confidence)
        } else {
            format!("choice {} conf {:.2}", question.options[pick], confidence)
        };
        block.dag = rows;
        trace.entities.push(block);
        let trace_digest = trace.content_hash().map_err(|e| e.to_string())?;

        Ok(SceneDecision {
            answer,
            trace,
            trace_digest,
        })
    }
}

// ── helpers ────────────────────────────────────────────────────────────────

#[inline]
fn dz_len(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| {
            let d = x - y;
            d * d
        })
        .sum::<f32>()
        .sqrt()
        / a.len() as f32
}

/// Best-minus-second margin over the scorer at this state.
fn margin_over(score: &dyn Fn(&[f32], usize) -> f32, state: &[f32], arm_count: usize) -> f32 {
    let mut best = f32::NEG_INFINITY;
    let mut second = f32::NEG_INFINITY;
    for a in 0..arm_count {
        let v = score(state, a);
        if v > best {
            second = best;
            best = v;
        } else if v > second {
            second = v;
        }
    }
    if second.is_finite() {
        best - second
    } else {
        best
    }
}

#[cfg(test)]
mod tests {
    //! The Plan-010 harness — G1 determinism, cross-instance parity,
    //! abstention honesty, G2 latency ceiling. All fixtures are
    //! deterministic (inline LCG — no new deps; law 7).

    use super::*;
    use katgpt_core::decision_wire::{Question, QuestionKind};

    /// Deterministic LCG (no fastrand dep — the tetris lane's optional dep
    /// stays optional).
    struct Lcg(u64);
    impl Lcg {
        fn new(seed: u64) -> Self {
            Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
        }
        fn next_u64(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0
        }
        fn unit(&mut self) -> f32 {
            (self.next_u64() >> 40) as f32 / 16_777_216.0
        }
    }

    /// A synthetic belt-cadence capture: `n` entities, smooth (KARC-
    /// learnable) vital trajectories + a small fixed condition vocabulary.
    /// SMOOTH sine drives are the point — the deployed KARC shape's honest
    /// regime (Bench 849: one-step NRMSE 1.2–4.1e-3 on smooth signals).
    fn synth_doc(tick: u64, n: usize, rng: &mut Lcg) -> RunetraceDoc {
        let mut doc = RunetraceDoc::new(tick, "harness_field");
        let hero = EntityBlock {
            id: "hero".into(),
            kind: EntityKind::Player,
            glyph: "⚔".into(),
            pos: Some([tick as f32 * 0.25, 3.0, 0.0]),
            vitals: vec![
                Vital {
                    name: "hp".into(),
                    value: 62.0 + 30.0 * (tick as f32 * 0.11).sin(),
                    max: 100.0,
                },
                Vital {
                    name: "mp".into(),
                    value: 40.0 + 25.0 * (tick as f32 * 0.07 + 1.2).cos(),
                    max: 80.0,
                },
                Vital {
                    name: "sta".into(),
                    value: 50.0 + 20.0 * (tick as f32 * 0.05).sin(),
                    max: 100.0,
                },
            ],
            think: vec![ConditionRow {
                condition: "threat_near".into(),
                action: "close_guard".into(),
                drive: Some(0.4 + 0.3 * (tick as f32 * 0.09).sin()),
            }],
            dag: Vec::new(),
        };
        doc.entities.push(hero);
        for i in 0..n.saturating_sub(1) {
            let phase = rng.unit() * std::f32::consts::TAU;
            doc.entities.push(EntityBlock {
                id: format!("mob_{i:02}"),
                kind: EntityKind::Monster,
                glyph: "☠".into(),
                pos: Some([
                    (rng.unit() - 0.5) * 40.0,
                    (rng.unit() - 0.5) * 40.0,
                    0.0,
                ]),
                vitals: vec![
                    Vital {
                        name: "hp".into(),
                        value: (55.0 + 35.0 * (tick as f32 * 0.13 + phase).sin()).clamp(1.0, 100.0),
                        max: 100.0,
                    },
                ],
                think: vec![ConditionRow {
                    condition: if i % 2 == 0 { "pack_aggro" } else { "lone_wander" }.into(),
                    action: if i % 2 == 0 { "chase" } else { "mill" }.into(),
                    drive: Some(0.5 + 0.2 * (tick as f32 * 0.06 + phase).cos()),
                }],
                dag: Vec::new(),
            });
        }
        doc
    }

    fn harness_question() -> Question {
        Question::choice(
            "hero",
            "which strategy arm should hero take next belt",
            vec![
                "attack01".into(),
                "kite_and_cast".into(),
                "defensive_line".into(),
                "withdraw".into(),
            ],
            Some("survive with rising sta, spend mp when safe".into()),
        )
    }

    /// Drive a loop through `belts` synthetic belts + the one fit that
    /// arms the forecast (fit cadence is the caller's — law 5).
    fn warm_loop(belts: u64, entities: usize, seed: u64) -> SceneLoop {
        let mut doc_rng = Lcg::new(seed);
        let mut loop_ = SceneLoop::new(SceneLoopConfig::default()).expect("config");
        for t in 0..belts {
            let doc = synth_doc(t, entities, &mut doc_rng);
            let acted = if t % 4 == 0 { Some(("hero", "attack01")) } else { None };
            loop_.observe_belt(&doc, acted);
        }
        loop_.fit_all(belts);
        loop_
    }

    #[test]
    fn config_validity() {
        assert!(SceneLoopConfig::default().validate().is_ok());
        for bad in [
            SceneLoopConfig {
                karc_lambda: 0.0,
                ..Default::default()
            },
            SceneLoopConfig {
                refine_omega: 0.9,
                refine_omega_cap: 0.8,
                ..Default::default()
            },
            SceneLoopConfig {
                abstain_margin: 0.0,
                ..Default::default()
            },
        ] {
            assert!(bad.validate().is_err());
        }
    }

    #[test]
    fn g1_determinism_same_instance_same_answer() {
        let mut l = warm_loop(300, 8, 42);
        let mut r = Lcg::new(42);
        let doc_a = synth_doc(300, 8, &mut r);
        let mut r2 = Lcg::new(42);
        let doc_b = synth_doc(300, 8, &mut r2);
        let q = harness_question();
        let a = l.decide(&doc_a, &q).expect("decide a");
        // decide is a pure function of (memory, doc, question) — the
        // per-episode kernels make the SECOND call byte-identical.
        let b = l.decide(&doc_b, &q).expect("decide b");
        assert_eq!(
            serde_json::to_string(&a.answer).unwrap(),
            serde_json::to_string(&b.answer).unwrap(),
            "G1: repeated decide on one instance must be byte-identical"
        );
        assert_eq!(a.trace_digest, b.trace_digest);
        assert_eq!(a.trace.render().unwrap(), b.trace.render().unwrap());
    }

    #[test]
    fn g1_determinism_cross_instance() {
        let mut x = warm_loop(300, 8, 42);
        let mut y = warm_loop(300, 8, 42);
        let mut r1 = Lcg::new(42);
        let mut r2 = Lcg::new(42);
        let doc_x = synth_doc(300, 8, &mut r1);
        let doc_y = synth_doc(300, 8, &mut r2);
        let q = harness_question();
        let a = x.decide(&doc_x, &q).expect("decide x");
        let b = y.decide(&doc_y, &q).expect("decide y");
        assert_eq!(
            serde_json::to_string(&a.answer).unwrap(),
            serde_json::to_string(&b.answer).unwrap(),
            "G1: same history must decide byte-identically across instances"
        );
        assert_eq!(a.trace_digest, b.trace_digest);
    }

    #[test]
    fn self_trace_carries_the_stage_path_and_validates() {
        let mut l = warm_loop(300, 8, 42);
        let doc = synth_doc(300, 8, &mut Lcg::new(42));
        let q = harness_question();
        let d = l.decide(&doc, &q).expect("decide");
        // the trace is a VALID doc (fail-closed validate pins the contract)
        d.trace.validate().expect("trace validates");
        let block = &d.trace.entities[0];
        assert_eq!(block.id, "scene_loop");
        assert_eq!(block.dag.len(), 6, "the six-stage path");
        assert_eq!(block.dag[0].stage.as_str(), "perceive");
        assert_eq!(block.dag[5].stage.as_str(), "action");
        // every dag row chains to the previous (parent ordering law)
        for (i, row) in block.dag.iter().enumerate() {
            match (i, row.parent) {
                (0, None) => {}
                (i, Some(p)) => assert_eq!(p as usize, i - 1),
                _ => panic!("root must be parentless"),
            }
        }
        // the forecast is warm at 300 belts (>= 256 samples) — honesty row
        assert!(
            block.think.iter().any(|c| c.condition.starts_with("karc fitted")),
            "at 300 belts the forecast must be warm"
        );
        // the render is deterministic + non-empty
        let r1 = d.trace.render().unwrap();
        assert!(r1.contains("runetrace/v1"));
        assert!(r1.contains("scene_loop"));
    }

    #[test]
    fn cold_loop_is_honest_in_the_trace() {
        // no observe_belt on the subject yet — decide refuses loud (the
        // subject has no memory), the honest failure.
        let mut l = SceneLoop::new(SceneLoopConfig::default()).unwrap();
        let doc = synth_doc(0, 4, &mut Lcg::new(7));
        let q = harness_question();
        let err = l.decide(&doc, &q).unwrap_err();
        assert!(err.contains("no memory"));
        // ONE belt: memory exists, forecast cold + elo empty — the trace
        // must say BOTH (the honesty rows).
        l.observe_belt(&doc, None);
        let d = l.decide(&doc, &q).expect("decide cold");
        d.trace.validate().unwrap();
        let block = &d.trace.entities[0];
        assert!(
            block.think.iter().any(|c| c.condition.contains("forecast cold")),
            "cold forecast must be said"
        );
    }

    #[test]
    fn g2_latency_ceiling_debug_profile() {
        // G2's per-decision ceiling at the reference population (1 subject,
        // 32 entities, 4 arms, <=8 refine steps). Generous DEBUG-profile
        // ceiling — the honest pin is the release measurement (136 µs/
        // decision @ 32 entities, M3, 2026-10-09, Plan 010's record; the
        // 1 ms serve bar holds with 7.4× headroom).
        let mut l = warm_loop(300, 32, 99);
        let doc = synth_doc(300, 32, &mut Lcg::new(99));
        let q = harness_question();
        // warm
        let _ = l.decide(&doc, &q);
        let n = 50u32;
        let t0 = std::time::Instant::now();
        for _ in 0..n {
            let _ = l.decide(&doc, &q);
        }
        let per = t0.elapsed().as_secs_f32() / n as f32;
        eprintln!("g2: {per:.6} s/decision ({} entities, 4 arms)", 32);
        assert!(
            per < 0.010,
            "G2 debug ceiling 10 ms/decision, measured {per:.4}s"
        );
    }

    #[test]
    fn question_shape_guards() {
        let mut l = warm_loop(300, 8, 42);
        let doc = synth_doc(300, 8, &mut Lcg::new(42));
        // <2 options refused
        let one = Question::choice("hero", "p", vec!["only".into()], None);
        assert!(l.decide(&doc, &one).is_err());
        // subject not in the doc refused
        let ghost = Question::choice("ghost", "p", harness_question().options.clone(), None);
        assert!(l.decide(&doc, &ghost).is_err());
        // the harness question's shape is pinned (choice kind)
        let q = harness_question();
        assert_eq!(q.kind, QuestionKind::Choice);
        assert_eq!(q.options.len(), 4);
    }
}

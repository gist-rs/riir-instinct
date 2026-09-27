//! The Tetris value-critic arm (instinct Issue 009 T8's modelless second
//! arm + T7's feature/fit substrate).
//!
//! ONE 1-ply afterstate value critic `Q(s,a) ≈ w·Ψ(features(afterstate))`,
//! argmax over the offered options — the exact serve shape the issue's HOW
//! section pins ("Serve: ONE 1-ply afterstate value critic"). This module
//! is the CLOSED-FORM member of that family: the Plan-308 KARC basis-ridge
//! readout ("no delay ring, plain basis expansion + ridge") —
//! [`katgpt_core::karc::ChebyshevBasis`] expands each scaled raw feature,
//! selected interactions generalize the champion's 3-mode FSM and the
//! piece-conditioned placement terms, and the readout is a closed-form
//! f64 ridge solve over the teacher's sigmoid-normalised search-root Q
//! (`ridge_solve_direct_f64`, the karc re-export).
//!
//! # Why this arm exists (modelless-first mandate)
//!
//! T7 (expert iteration) is the TRAINED path. T8's second arm is this
//! MODELLESS endpoint: if a closed-form basis-ridge fit of the teacher's Q
//! already beats the b0 baseline (the 1-ply champion evaluator, budget-0
//! PUCT prior), the modelless path wins and no training spend is justified.
//! Either way the infrastructure here — the raw feature contract, the
//! teacher-sample collection, the fit, the lane eval — is what a trained
//! critic (T7) consumes, so this is on the critical path regardless.
//!
//! # The feature contract (what serving must make derivable)
//!
//! Every raw feature is derivable from the widened serving input: the root
//! board grid, the current piece, and each option's placement (rotation +
//! column — the site knows both; the sidecar must carry them or per-option
//! afterstate rows). Two views of the placement are used, both standard:
//! the PRE-clear [`OutcomeFeatures`] (Dellacherie's convention, incl.
//! `eroded_cells`/`landing_height`, which the afterstate alone destroys)
//! and the POST-clear afterstate (what the game state actually becomes —
//! the [`katgpt_tetris::sim::BoardScan`] integer set the champion
//! evaluator consumes). No preview, no next piece, no bag state anywhere:
//! the serving information rule holds by construction.
//!
//! # Determinism
//!
//! Collection is threaded ACROSS seeds only; per-seed samples merge in
//! seed order and the Gram accumulates over them in (seed, decision,
//! option) order, so the fit is bit-deterministic for a fixed seed list.
//! The weights' BLAKE3 digest is the model identity.

use crate::tetris_lane::{start_board, GameStats, Regime, TeacherState, CHAMPION_ID, TEACHER_RNG_SALT};
use katgpt_core::chance_puct::{ChancePuct, ChancePuctConfig, RootStat};
use katgpt_core::karc::{ridge_solve_direct_f64, ChebyshevBasis, KarcBasis};
use katgpt_tetris::lookahead::{apply, Bag, LINES_SCORE};
use katgpt_tetris::rulebook::{Genome, Leaf, Mode};
use katgpt_tetris::sim::{
    Board, DropRule, OutcomeFeatures, Piece, Placement, landing_options_with, outcome_features,
};
use std::time::Instant;

// ── Raw features (the contract) ──────────────────────────────────────────

/// Numeric raw features, DOCUMENTED ORDER (the indices are contract — the
/// interaction tables below and any future served feature decoder both
/// name them).
pub mod feat {
    pub const LANDING_HEIGHT: usize = 0;
    pub const ERODED_CELLS: usize = 1;
    pub const LINES_CLEARED: usize = 2;
    pub const HOLES_PRE: usize = 3;
    pub const HOLES_DELTA: usize = 4;
    pub const BUMPINESS_PRE: usize = 5;
    pub const MAX_HEIGHT_PRE: usize = 6;
    pub const AGG_HEIGHT_PRE: usize = 7;
    pub const ROW_TRANS_PRE: usize = 8;
    pub const COL_TRANS_PRE: usize = 9;
    pub const CUM_WELLS_PRE: usize = 10;
    // Post-clear afterstate scan (the champion's integer set).
    pub const ROW_TRANS: usize = 11;
    pub const COL_TRANS: usize = 12;
    pub const HOLES: usize = 13;
    pub const HOLE_COVER: usize = 14;
    pub const HEIGHTS: usize = 15; // 10 entries: 15..=24
    pub const WELLS: usize = 25;
    pub const DEEP_WELL: usize = 26;
    pub const NINE_ONE: usize = 27;
    pub const FLAT_TOP: usize = 28;
    pub const MAX_H: usize = 29;
    pub const AGG_H: usize = 30;
    pub const BUMPINESS: usize = 31;
    pub const TETRIS: usize = 32;
    pub const NUM: usize = 33;
}

/// One option's raw critic input — everything serving can derive, nothing
/// more. `piece` = [`Piece::index`], `mode` = the champion FSM's mode of
/// the ROOT board (`Genome::mode_of`, the per-decision convention).
#[derive(Clone, Copy, Debug)]
pub struct RawOpt {
    pub num: [f64; feat::NUM],
    pub piece: u8,
    pub mode: u8,
}

/// Well depth of column `c` with walls at full height (the champion
/// rulebook's `well_depth`, which is private in katgpt-tetris — a 3-line
/// restatement; the fit reproduces the champion's feature basis so any
/// semantic drift shows up as fit quality, and the module test pins the
/// design dimension).
fn well_depth(h: &[usize; 10], c: usize) -> usize {
    let lft = if c == 0 { 20 } else { h[c - 1] };
    let rgt = if c + 1 == 10 { 20 } else { h[c + 1] };
    lft.min(rgt).saturating_sub(h[c])
}

/// Build one option's raw input: pre-clear outcome features + the
/// post-clear afterstate scan + the champion-set height extras.
/// `after`/`lines_cleared` come from one [`apply`] of the placement.
pub fn raw_opt(
    board: &Board,
    p: &Placement,
    piece: Piece,
    mode: Mode,
    after: &Board,
    lines_cleared: u32,
) -> RawOpt {
    let OutcomeFeatures {
        holes: holes_pre,
        holes_delta,
        bumpiness: bumpiness_pre,
        max_height: max_height_pre,
        aggregate_height: agg_height_pre,
        landing_height,
        row_transitions: row_trans_pre,
        col_transitions: col_trans_pre,
        cumulative_wells: cum_wells_pre,
        eroded_cells,
        ..
    } = outcome_features(board, p);

    let scan = after.scan();
    let heights = after.heights();
    let wells: usize = (0..10).map(|c| well_depth(&heights, c)).sum();
    let deep_well: usize = (0..10)
        .map(|c| {
            let d = well_depth(&heights, c);
            if d > 2 { (d - 2) * (d - 2) } else { 0 }
        })
        .sum();
    let nine_one = well_depth(&heights, 9).min(4);
    let flat_top: usize = (1..9).map(|c| heights[c].abs_diff(heights[c - 1])).sum();
    let max_h = heights.iter().copied().max().unwrap_or(0);
    let agg_h: usize = heights.iter().sum();
    let bumpiness: usize = (1..10).map(|c| heights[c].abs_diff(heights[c - 1])).sum();

    let mut num = [0.0f64; feat::NUM];
    num[feat::LANDING_HEIGHT] = f64::from(landing_height);
    num[feat::ERODED_CELLS] = eroded_cells as f64;
    num[feat::LINES_CLEARED] = lines_cleared as f64;
    num[feat::HOLES_PRE] = holes_pre as f64;
    num[feat::HOLES_DELTA] = holes_delta as f64;
    num[feat::BUMPINESS_PRE] = bumpiness_pre as f64;
    num[feat::MAX_HEIGHT_PRE] = max_height_pre as f64;
    num[feat::AGG_HEIGHT_PRE] = agg_height_pre as f64;
    num[feat::ROW_TRANS_PRE] = row_trans_pre as f64;
    num[feat::COL_TRANS_PRE] = col_trans_pre as f64;
    num[feat::CUM_WELLS_PRE] = cum_wells_pre as f64;
    num[feat::ROW_TRANS] = scan.row_trans as f64;
    num[feat::COL_TRANS] = scan.col_trans as f64;
    num[feat::HOLES] = scan.holes as f64;
    num[feat::HOLE_COVER] = scan.hole_cover as f64;
    for (dst, h) in num[feat::HEIGHTS..feat::HEIGHTS + 10].iter_mut().zip(heights) {
        *dst = h as f64;
    }
    num[feat::WELLS] = wells as f64;
    num[feat::DEEP_WELL] = deep_well as f64;
    num[feat::NINE_ONE] = nine_one as f64;
    num[feat::FLAT_TOP] = flat_top as f64;
    num[feat::MAX_H] = max_h as f64;
    num[feat::AGG_H] = agg_h as f64;
    num[feat::BUMPINESS] = bumpiness as f64;
    num[feat::TETRIS] = f64::from(lines_cleared == 4);

    RawOpt { num, piece: piece.index() as u8, mode: mode as u8 }
}

// ── Basis (KARC Plan-308, no delay ring) ─────────────────────────────────

/// Chebyshev order per numeric coordinate (T0..T3: constant, linear,
/// quadratic, cubic — bounded |Tⱼ|≤1 on the scaled domain).
const CHEB_M: usize = 4;

/// One-hot context axes for the interaction segments (a constant within a
/// decision is argmax-irrelevant on its own — context enters only as
/// interactions with option-varying features).
const N_MODES: usize = 3;
const N_PIECES: usize = 7;

/// The champion-set features the mode interactions generalize (the FSM
/// switches these 11 weights per mode; the ridge learns smooth blends).
const MODE_FEATS: [usize; 11] = [
    feat::LINES_CLEARED,
    feat::ROW_TRANS,
    feat::COL_TRANS,
    feat::HOLES,
    feat::WELLS,
    feat::MAX_H,
    feat::DEEP_WELL,
    feat::NINE_ONE,
    feat::TETRIS,
    feat::FLAT_TOP,
    feat::HOLE_COVER,
];

/// Piece-conditioned placement terms (Dellacherie's piece-specific
/// eroded/landing structure).
const PIECE_FEATS: [usize; 4] = [
    feat::LANDING_HEIGHT,
    feat::ERODED_CELLS,
    feat::LINES_CLEARED,
    feat::HOLES_DELTA,
];

/// Curated pairwise interactions beyond the one-hot blocks (the quadratic
/// structure a 1-ply eval plausibly needs: hole pressure, stack mass,
/// placement reward shapes).
const PAIRS: [(usize, usize); 8] = [
    (feat::HOLES, feat::HOLE_COVER),
    (feat::MAX_H, feat::AGG_H),
    (feat::BUMPINESS, feat::WELLS),
    (feat::LANDING_HEIGHT, feat::ERODED_CELLS),
    (feat::LINES_CLEARED, feat::ERODED_CELLS),
    (feat::HOLES, feat::MAX_H),
    (feat::HOLES_PRE, feat::LINES_CLEARED),
    (feat::WELLS, feat::NINE_ONE),
];

/// Design dimension: bias + Chebyshev(4) × 33 numerics + mode×11 + piece×4
/// one-hot interactions + 8 curated pairs = 202.
pub const DESIGN_DIM: usize =
    1 + feat::NUM * CHEB_M + N_MODES * MODE_FEATS.len() + N_PIECES * PIECE_FEATS.len() + PAIRS.len();

/// Per-coordinate train scaling: raw → clip((raw − lo)/(hi − lo), 0, 1) →
/// 2x − 1 ∈ [−1, 1] (Chebyshev's domain; train min/max, eval clips).
#[derive(Clone, Debug)]
pub struct FeatureScale {
    pub lo: [f64; feat::NUM],
    pub hi: [f64; feat::NUM],
}

impl FeatureScale {
    /// Train-split min/max. A constant coordinate (e.g. `nine_one` on
    /// empty boards) pins to [x−1, x+1] so the scaled value lands at the
    /// domain midpoint instead of dividing by zero.
    pub fn fit(samples: &[Sample]) -> Self {
        Self::fit_iter(samples.iter())
    }

    /// [`Self::fit`] over any sample iterator (the bench folds per-regime
    /// blocks without concatenating them).
    pub fn fit_iter<'a>(samples: impl Iterator<Item = &'a Sample>) -> Self {
        let mut lo = [f64::INFINITY; feat::NUM];
        let mut hi = [f64::NEG_INFINITY; feat::NUM];
        for s in samples {
            for (k, &v) in s.raw.num.iter().enumerate() {
                lo[k] = lo[k].min(v);
                hi[k] = hi[k].max(v);
            }
        }
        for k in 0..feat::NUM {
            if hi[k] <= lo[k] {
                let x = if lo[k].is_finite() { lo[k] } else { 0.0 };
                lo[k] = x - 1.0;
                hi[k] = x + 1.0;
            }
        }
        Self { lo, hi }
    }

    #[inline]
    fn scaled(&self, k: usize, v: f64) -> f64 {
        let x = (v - self.lo[k]) / (self.hi[k] - self.lo[k]);
        x.clamp(0.0, 1.0) * 2.0 - 1.0
    }
}

/// Fill the design vector `h` (length [`DESIGN_DIM`]) for one raw input.
/// Fixed segment order: bias | Chebyshev blocks (coordinate-major) |
/// mode interactions | piece interactions | curated pairs.
pub fn expand_into(raw: &RawOpt, scale: &FeatureScale, h: &mut [f32]) {
    debug_assert_eq!(h.len(), DESIGN_DIM);
    let basis = ChebyshevBasis::<CHEB_M>::new();
    h[0] = 1.0;
    let mut k = 1usize;
    let mut xs = [0.0f64; feat::NUM];
    for (i, &v) in raw.num.iter().enumerate() {
        let x = scale.scaled(i, v);
        xs[i] = x;
        let mut cell = [0.0f32; CHEB_M];
        basis.eval_into(x as f32, &mut cell);
        h[k..k + CHEB_M].copy_from_slice(&cell);
        k += CHEB_M;
    }
    for m in 0..N_MODES {
        let on = f64::from(raw.mode == m as u8);
        for &f in &MODE_FEATS {
            h[k] = (on * xs[f]) as f32;
            k += 1;
        }
    }
    for p in 0..N_PIECES {
        let on = f64::from(raw.piece == p as u8);
        for &f in &PIECE_FEATS {
            h[k] = (on * xs[f]) as f32;
            k += 1;
        }
    }
    for &(a, b) in &PAIRS {
        h[k] = (xs[a] * xs[b]) as f32;
        k += 1;
    }
    debug_assert_eq!(k, DESIGN_DIM);
}

// ── Samples, accumulation, fit ───────────────────────────────────────────

/// One collected (option, teacher-Q) pair. `seed`/`decision` identify the
/// decision the option belongs to (groups are (seed, decision)); `is_pick`
/// marks the teacher's chosen option (the top-1-agreement target).
#[derive(Clone, Debug)]
pub struct Sample {
    pub seed: u64,
    pub decision: u32,
    pub raw: RawOpt,
    /// Teacher search-root Q ∈ (0,1) — the sigmoid-normalised chance_puct
    /// value (0.5 = "as good as where we stand now"; terminal backs up 0).
    pub q: f64,
    pub is_pick: bool,
}

/// Contiguous per-decision groups over an ordered sample list — the shape
/// [`FitAccumulator::accumulate`] / [`agreement_and_r2`] consume. Samples
/// must already be in (seed, decision) collection order.
pub fn decision_groups(samples: &[Sample]) -> Vec<(usize, usize)> {
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    while i < samples.len() {
        let (seed, decision) = (samples[i].seed, samples[i].decision);
        let mut j = i + 1;
        while j < samples.len()
            && samples[j].seed == seed
            && samples[j].decision == decision
        {
            j += 1;
        }
        groups.push((i, j));
        i = j;
    }
    groups
}

/// The one-pass Gram/cov accumulator: `HᵀH` (f64, no ridge yet) and `Hᵀy`
/// stream group-by-group through the KARC [`chunked_gram_into`] inner loop
/// without materializing `H`. Accumulate ONCE, then [`Self::solve`] per λ —
/// a Cholesky per candidate, never a re-accumulation.
pub struct FitAccumulator {
    gram: Vec<f64>,
    cov: Vec<f64>,
    pub n: usize,
}

impl Default for FitAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl FitAccumulator {
    pub fn new() -> Self {
        Self { gram: vec![0.0; DESIGN_DIM * DESIGN_DIM], cov: vec![0.0; DESIGN_DIM], n: 0 }
    }

    /// Accumulate over grouped samples (contiguous, as
    /// [`decision_groups`] yields).
    ///
    /// ⚠ Deliberately NOT [`katgpt_core::karc::chunked_gram_into`]: that
    /// API ZEROES `out_gram` on every call (batch semantics — the
    /// "chunking" is the inner loop's 4-unroll), so per-group calls would
    /// silently keep only the LAST group (measured in this module's first
    /// run: g[bias][bias] = one group's count instead of n). This local
    /// streaming outer product is its row loop, incremental across calls,
    /// upper triangle + mirror, same f64 accumulation order per entry.
    pub fn accumulate(&mut self, samples: &[Sample], groups: &[(usize, usize)], scale: &FeatureScale) {
        let mut rows: Vec<f32> = vec![0.0; DESIGN_DIM];
        for &(start, end) in groups {
            for s in &samples[start..end] {
                expand_into(&s.raw, scale, &mut rows);
                let gram = self.gram.as_mut_slice();
                for i in 0..DESIGN_DIM {
                    let row_i = rows[i] as f64;
                    if row_i == 0.0 {
                        continue; // the outer product's row+column are zero
                    }
                    gram[i * DESIGN_DIM + i] += row_i * row_i;
                    let mut j = i + 1;
                    while j + 4 <= DESIGN_DIM {
                        let a0 = row_i * rows[j] as f64;
                        gram[i * DESIGN_DIM + j] += a0;
                        gram[j * DESIGN_DIM + i] += a0;
                        let a1 = row_i * rows[j + 1] as f64;
                        gram[i * DESIGN_DIM + j + 1] += a1;
                        gram[(j + 1) * DESIGN_DIM + i] += a1;
                        let a2 = row_i * rows[j + 2] as f64;
                        gram[i * DESIGN_DIM + j + 2] += a2;
                        gram[(j + 2) * DESIGN_DIM + i] += a2;
                        let a3 = row_i * rows[j + 3] as f64;
                        gram[i * DESIGN_DIM + j + 3] += a3;
                        gram[(j + 3) * DESIGN_DIM + i] += a3;
                        j += 4;
                    }
                    while j < DESIGN_DIM {
                        let a = row_i * rows[j] as f64;
                        gram[i * DESIGN_DIM + j] += a;
                        gram[j * DESIGN_DIM + i] += a;
                        j += 1;
                    }
                }
                for (c, &hi) in rows.iter().enumerate() {
                    self.cov[c] += hi as f64 * s.q;
                }
                self.n += 1;
            }
        }
    }

    /// Solve the closed-form ridge at `λ_total = lambda_mult · n`:
    /// `w = (HᵀH + λI)⁻¹ Hᵀy`. Deterministic for a fixed accumulator.
    pub fn solve(&self, scale: FeatureScale, lambda_mult: f64) -> CriticModel {
        assert!(self.n > 0, "solve: no samples accumulated");
        let mut gram_reg = self.gram.clone();
        let lambda_total = lambda_mult * self.n as f64;
        for i in 0..DESIGN_DIM {
            gram_reg[i * DESIGN_DIM + i] += lambda_total;
        }
        let mut w = vec![0.0f64; DESIGN_DIM];
        let mut l_scratch = vec![0.0f64; DESIGN_DIM * DESIGN_DIM];
        let mut z_scratch = vec![0.0f64; DESIGN_DIM];
        ridge_solve_direct_f64(&mut w, &mut l_scratch, &mut z_scratch, &gram_reg, &self.cov, DESIGN_DIM, 1);
        CriticModel { scale, w, lambda_mult, n: self.n }
    }
}

/// A fitted closed-form critic.
#[derive(Clone, Debug)]
pub struct CriticModel {
    pub scale: FeatureScale,
    /// Ridge solution, design order (bias first).
    pub w: Vec<f64>,
    /// The per-sample ridge multiplier the solve used (λ_total = mult·n).
    pub lambda_mult: f64,
    /// Sample count the solve saw (digest provenance).
    pub n: usize,
}

impl CriticModel {
    /// The critic score of one option (higher = better; only
    /// within-decision comparisons are meaningful).
    pub fn score(&self, raw: &RawOpt, h: &mut [f32]) -> f64 {
        expand_into(raw, &self.scale, h);
        let mut s = 0.0f64;
        for (i, &hi) in h.iter().enumerate() {
            s += self.w[i] * hi as f64;
        }
        s
    }

    /// BLAKE3 digest over the canonical weight bytes (the model identity —
    /// the vessel payload's eventual commitment shape).
    pub fn digest_hex(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        for v in self.scale.lo.iter().chain(self.scale.hi.iter()).chain(self.w.iter()) {
            hasher.update(&v.to_le_bytes());
        }
        hasher.update(&self.lambda_mult.to_le_bytes());
        hasher.update(&self.n.to_le_bytes());
        hasher.finalize().to_hex()[..16].to_string()
    }
}

/// Top-1 agreement + fit-error pieces over grouped samples, against a
/// CALLER-supplied global mean q (so a multi-block corpus aggregates by
/// summing [`AgreeStats`]). The argmax runs over each group's OBSERVED
/// options (the teacher's visited set — the only place the teacher pick
/// is defined).
#[derive(Clone, Copy, Debug, Default)]
pub struct AgreeStats {
    pub hits: u64,
    pub decisions: u64,
    pub sse: f64,
    pub sst: f64,
}

pub fn agreement_stats(model: &CriticModel, samples: &[Sample], groups: &[(usize, usize)], mean_q: f64) -> AgreeStats {
    let mut h = vec![0.0f32; DESIGN_DIM];
    let mut st = AgreeStats::default();
    for &(start, end) in groups {
        let group = &samples[start..end];
        let mut best_i = 0usize;
        let mut best_s = f64::NEG_INFINITY;
        for (i, s) in group.iter().enumerate() {
            let sc = model.score(&s.raw, &mut h);
            if sc > best_s {
                best_s = sc;
                best_i = i;
            }
            st.sse += (sc - s.q) * (sc - s.q);
            st.sst += (s.q - mean_q) * (s.q - mean_q);
        }
        st.decisions += 1;
        if group[best_i].is_pick {
            st.hits += 1;
        }
    }
    st
}

impl AgreeStats {
    /// Mean per-decision top-1 agreement with the teacher pick.
    pub fn agreement(&self) -> f64 {
        self.hits as f64 / self.decisions.max(1) as f64
    }
    /// R² of the q fit (1 = exact; ≤ 0 = no better than the mean).
    pub fn r2(&self) -> f64 {
        if self.sst > 0.0 {
            1.0 - self.sse / self.sst
        } else {
            0.0
        }
    }
}

/// Block aggregation (the bench folds per-regime blocks into one corpus
/// statistic).
impl std::ops::Add for AgreeStats {
    type Output = AgreeStats;
    fn add(self, rhs: AgreeStats) -> AgreeStats {
        AgreeStats {
            hits: self.hits + rhs.hits,
            decisions: self.decisions + rhs.decisions,
            sse: self.sse + rhs.sse,
            sst: self.sst + rhs.sst,
        }
    }
}

/// Convenience single-corpus form of [`agreement_stats`]
/// + [`AgreeStats::agreement`] / [`AgreeStats::r2`].
pub fn agreement_and_r2(model: &CriticModel, samples: &[Sample], groups: &[(usize, usize)]) -> (f64, f64) {
    let mean_q = samples.iter().map(|s| s.q).sum::<f64>() / samples.len().max(1) as f64;
    let st = agreement_stats(model, samples, groups, mean_q);
    (st.agreement(), st.r2())
}

// ── The lane loop (one game, two policies) ───────────────────────────────

/// Which policy drives [`play_lane`].
pub enum LanePolicy<'m> {
    /// Teacher collection: budget-`budget` chance_puct picks; every visited
    /// option's Q recorded through `sink`.
    Teacher { budget: u32, sink: &'m mut Vec<Sample> },
    /// The fitted critic: 1-ply argmax over per-option critic scores
    /// (first strict max — the site's argmax convention).
    Argmax { model: &'m CriticModel },
}

/// One seed, one game, one policy — the T6 game-loop shape (same bag, same
/// garbage start, same teacher-RNG convention) with the pick sourced from
/// the lane policy. Lives here rather than in `tetris_lane` so the frozen
/// T5/T6 harness stays byte-stable while the critic arm lands; the T9
/// adapter promotion consolidates the loops.
pub fn play_lane(genome: &Genome, seed: u64, regime: Regime, cap: usize, policy: &mut LanePolicy<'_>) -> GameStats {
    let mut bag = Bag::new(seed);
    let mut board = start_board(seed, regime);
    let mut next = bag.draw();
    let mut rng = fastrand::Rng::with_seed(seed ^ TEACHER_RNG_SALT);
    let mut st = GameStats { seed, ..GameStats::default() };
    let mut decision_ms: Vec<f64> = Vec::with_capacity(cap.min(4096));
    let mut decision: u32 = 0;
    let mut h_buf: Vec<f32> = vec![0.0; DESIGN_DIM];

    while st.pieces < cap {
        let cur = next;
        next = bag.draw();
        let options = landing_options_with(&board, cur, DropRule::FromTop);
        if options.is_empty() {
            st.topped_out = true;
            break;
        }
        let t0 = Instant::now();
        let picked = match policy {
            LanePolicy::Teacher { budget, sink } => {
                let mode = genome.mode_of(&board);
                let eval = |b: &Board, lines: u32, tetrises: u32| -> f64 {
                    let scan = b.scan();
                    let leaf = Leaf { board: b, heights: scan.heights, lines, tetrises, held: None };
                    genome.eval_with(&leaf, mode, &scan)
                };
                let root = TeacherState::root(&board, cur, &eval);
                let cfg = ChancePuctConfig { budget: *budget, ..ChancePuctConfig::default() };
                let mut search = ChancePuct::new(cfg);
                let picked = match search.search(&root, &mut rng) {
                    Some(p) => p.index,
                    None => usize::MAX, // unreachable: options non-empty ⇒ actions non-empty
                };
                let stats: Vec<RootStat> = search.root_stats().collect();
                for s in stats {
                    if s.index < options.len() && s.visits > 0 {
                        let (b1, l1) = apply(&board, &options[s.index].cells);
                        let raw = raw_opt(&board, &options[s.index], cur, mode, &b1, l1);
                        sink.push(Sample {
                            seed,
                            decision,
                            raw,
                            q: f64::from(s.q),
                            is_pick: s.index == picked,
                        });
                    }
                }
                picked
            }
            LanePolicy::Argmax { model } => {
                let mode = genome.mode_of(&board);
                let mut best = 0usize;
                let mut best_s = f64::NEG_INFINITY;
                for (i, p) in options.iter().enumerate() {
                    let (b1, l1) = apply(&board, &p.cells);
                    let raw = raw_opt(&board, p, cur, mode, &b1, l1);
                    let s = model.score(&raw, &mut h_buf);
                    if s > best_s {
                        best_s = s;
                        best = i;
                    }
                }
                best
            }
        };
        decision_ms.push(t0.elapsed().as_secs_f64() * 1e3);
        st.decisions += 1;
        decision += 1;

        let picked = picked.min(options.len() - 1);
        let (nb, l1) = apply(&board, &options[picked].cells);
        board = nb;
        st.lines += l1;
        st.tetrises += u32::from(l1 == 4);
        st.points += LINES_SCORE[l1.min(4) as usize];
        st.pieces += 1;
    }

    decision_ms.sort_by(f64::total_cmp);
    st.p50_ms = decision_ms.get(decision_ms.len() / 2).copied().unwrap_or(0.0);
    st
}

/// The lane's constant context line (records quote it).
pub fn lane_context() -> String {
    format!("champion {CHAMPION_ID} · KARC chebyshev-{CHEB_M} basis-ridge (Plan 308, no delay ring) · design dim {DESIGN_DIM}")
}

/// Teacher collection over one (regime, seed list), threaded across seeds;
/// per-seed sample blocks merge in seed order (the fit's determinism
/// premise: the Gram then accumulates in (regime, seed, decision, option)
/// order). Shared by the critic bench (fit) and the dataset export bin
/// (riir-train Issue 580's contract).
pub fn collect_teacher(
    genome: &Genome,
    regime: Regime,
    seeds: &[u64],
    budget: u32,
    cap: usize,
    threads: usize,
) -> (Vec<Sample>, Vec<GameStats>) {
    let next_seed = std::sync::atomic::AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<(u64, GameStats, Vec<Sample>)>> =
        std::sync::Mutex::new(Vec::with_capacity(seeds.len()));
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let k = next_seed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let seed = seeds[k];
                    let mut sink: Vec<Sample> = Vec::new();
                    let mut policy = LanePolicy::Teacher { budget, sink: &mut sink };
                    let stats = play_lane(genome, seed, regime, cap, &mut policy);
                    results.lock().expect("results lock").push((seed, stats, sink));
                }
            });
        }
    });
    let mut out = results.into_inner().expect("results");
    out.sort_by_key(|(seed, _, _)| *seed);
    let mut samples = Vec::new();
    let mut stats = Vec::new();
    for (_, st, mut sink) in out {
        stats.push(st);
        samples.append(&mut sink);
    }
    (samples, stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A linear target in two varied coordinates must be recovered to
    /// ranking-relevant accuracy by the closed-form fit (G1-shape sanity:
    /// the fit machinery can represent the class of truth it will need on
    /// real teacher Q).
    #[test]
    fn linear_target_is_recovered() {
        let mk = |i: usize| -> Sample {
            let mut num = [0.0; feat::NUM];
            num[feat::LANDING_HEIGHT] = ((i % 17) as f64 / 8.0 - 1.0).clamp(-1.0, 1.0);
            num[feat::ERODED_CELLS] = ((i % 5) as f64 / 2.0 - 1.0).clamp(-1.0, 1.0);
            let raw = RawOpt { num, piece: (i % 7) as u8, mode: (i % 3) as u8 };
            let q = 2.0 * raw.num[feat::LANDING_HEIGHT] - raw.num[feat::ERODED_CELLS];
            // Groups of 4 consecutive samples share (seed, decision).
            Sample { seed: ((i / 4) % 4) as u64, decision: (i / 4) as u32, raw, q, is_pick: i.is_multiple_of(4) }
        };
        let samples: Vec<Sample> = (0..400).map(mk).collect();
        let groups = decision_groups(&samples);
        assert_eq!(groups.len(), 100, "groups partition by (seed, decision)");
        let scale = FeatureScale::fit(&samples);
        let mut acc = FitAccumulator::new();
        acc.accumulate(&samples, &groups, &scale);
        let model = acc.solve(scale, 0.01);
        let (_, r2) = agreement_and_r2(&model, &samples, &groups);
        assert!(r2 > 0.99, "linear target must fit near-exactly, got R2={r2:.4}");
    }

    /// Same input → same design vector; bias first; declared dimension.
    #[test]
    fn expansion_is_deterministic_and_shaped() {
        let mut num = [0.0; feat::NUM];
        num[feat::HOLES] = 3.0;
        num[feat::HEIGHTS + 4] = 9.0;
        let raw = RawOpt { num, piece: 2, mode: 1 };
        let scale = FeatureScale { lo: [-2.0; feat::NUM], hi: [2.0; feat::NUM] };
        let mut a = vec![0.0f32; DESIGN_DIM];
        let mut b = vec![0.0f32; DESIGN_DIM];
        expand_into(&raw, &scale, &mut a);
        expand_into(&raw, &scale, &mut b);
        assert_eq!(a, b);
        assert_eq!(a[0], 1.0, "bias segment first");
        assert_eq!(a.len(), DESIGN_DIM);
    }
}

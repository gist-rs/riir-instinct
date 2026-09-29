//! The round-5 blended teacher (instinct Issue 009 T7 round 5, pre-registered
//! 2026-09-29): `chance_puct` over (champion evaluator + the trained critic's
//! afterstate values blended at the search's eval seam) — the teacher-side
//! critic-guided search. As a SERVE policy this is dead by arithmetic (the
//! round-4 record priced ~240 evals × 3.76 ms ≈ 0.9 s/spot); as a TEACHER it
//! is unbounded by serve G2, and a teacher stronger than b0 raises the
//! ceiling the 1-ply student distills toward — the mechanism rounds 1–4
//! lacked (every student so far lost to b0 = 907.8 / 532.5).
//!
//! # Where the blend sits (the pre-registration's own words, read literally)
//!
//! "the r4 critic's AFTERSTATE VALUES blended at the search's eval seam" —
//! the VALUE seam, not the prior seam: every leaf evaluation
//! ([`ChanceGame::value`]) blends
//!
//! ```text
//! v = champ(afterstate) + w · gain · (m − m̄)
//! ```
//!
//! where `m` = the trained critic's score of the placement that PRODUCED the
//! afterstate (exactly the `RawOpt` construction the critic was trained on —
//! pre-clear features + post-clear scan, mode at the decision root), and
//! `(m̄, gain)` is a per-decision ROOT frame: over the root's own landing
//! options, `m̄` = mean critic score, `gain = sd(champ)/sd(critic)` maps the
//! critic's deviation into champion-point units. The frame is computed once
//! per decision in [`BlendedState::root`] (~one option-list pass); the root's
//! own `value()` (the search's `value_ref`) stays CHAMPION-ONLY — 0.5 keeps
//! meaning "as good as where we stand now".
//!
//! Action PRIORS stay pure champion: the prior's job is exploration ordering
//! (top-8 truncation), and leaving it champion keeps the critic-call count at
//! ~one per node expansion (~budget per decision) instead of ~one per option
//! per expansion — the 27× cost difference that makes the full-dataset regen
//! feasible on this box (measured: the plain b1600 export is 91 s wall; the
//! round-5 pilot prices the blend overhead in its manifest).
//!
//! # Degenerate cases (all safe by construction)
//!
//! * `weight == 0.0` → NO critic call anywhere, and `value()` returns the
//!   champion score bit-exactly (explicit branch, no `c + 0.0·x` signed-zero
//!   hazard) — the blended teacher is BYTE-IDENTICAL to
//!   [`crate::tetris_lane::TeacherState`] given the same rng (pinned by
//!   [`weight_zero_reproduces_the_plain_teacher_bit_exactly`]).
//! * `sd(critic) < ε` at the root frame (a flat critic) → `gain = 0` → the
//!   blend contributes a constant 0 deviation → pure champion.
//! * Terminal states back up 0 unchanged (the searcher's own law).
//!
//! # Measured verdict (Bench 027, 2026-09-29/30 — the round-5 cheap gate)
//!
//! NULL-TO-HARMFUL across the tested grid, with paired bounds: 18@75 —
//! blend b400 w=0.5 −73.0 vs plain (lb95 −120.4, n=100, cap 600); blend
//! b1600 w=0.5 −0.2 (exact null, ±86 detection limit at n=60, cap 1000);
//! blend b1600 w=1.0 −185.9 (lb95 −283.5). No tested corner has a positive
//! lower bound; the best result is the exact null. The r4 critic is a
//! weaker ranker than the champion leaf (−98.1 vs b0 at 1-ply, ub95 −36.6),
//! so the value-seam blend dilutes at best. The plain teacher's budget
//! ladder is flat in PLAY STRENGTH within the cap windows (t6400 − t1600 =
//! +11.7, lb95 −15.1; the early-death-rate discordance 3-vs-5 is flat too;
//! label accuracy vs budget was not measured). SCOPE: this is the RESIDUAL
//! VALUE-SEAM form (priors excluded by cost, w ∈ {0.5, 1.0}, the r4
//! critic's SIGMOID score) — NOT a falsification of critic-guided search
//! broadly, and NOT of the CONCURRENT sibling lane's z-blend form
//! (`tetris_critic::TeacherBlend` — logit z-interpolation over the root's
//! kept top-8, Bench 026's pre-registered A/B; three form deltas and its
//! cap-5000 16@75 cell are outside this module's measurements). The full
//! residual-form lane was NO-GO on this gate; no eval-seed read spent.
//! Full record + per-seed CSVs: `.benchmarks/027_*`.
//!
//! # Determinism
//!
//! Same convention as the plain teacher: per-(seed, budget, weight, model)
//! deterministic — the rng is seeded `seed ^ TEACHER_RNG_SALT` and consumed
//! only by the searcher's sampling, whose trajectory is a pure function of
//! the (deterministic) blended scores.

use crate::tetris_critic::{raw_opt, Sample, TrainedMlp};
use crate::tetris_lane::{start_board, GameStats, Regime, TeacherMove, TEACHER_RNG_SALT};
use katgpt_core::chance_puct::{ChanceGame, ChancePuct, ChancePuctConfig, RootStat};
use katgpt_tetris::lookahead::{apply, Bag, LINES_SCORE};
use katgpt_tetris::rulebook::{Genome, Leaf, Mode};
use katgpt_tetris::sim::{Board, DropRule, Piece, Placement, landing_options_with};
use std::time::Instant;

/// The champion evaluator + the trained critic + the blend weight, shared by
/// every state of one search.
pub struct BlendCtx<'a> {
    pub genome: &'a Genome,
    pub critic: &'a TrainedMlp,
    pub weight: f64,
}

/// The placement that produced an afterstate (carried so a leaf `value()` can
/// build the critic's `RawOpt` — pre-clear features need the PRE board).
#[derive(Clone)]
struct LastPlace {
    pre: Board,
    cells: [(usize, usize); 4],
    piece: Piece,
    /// Lines the placement itself cleared (not the path-cumulative count the
    /// champion's `Leaf` consumes — `RawOpt` wants the placement's own).
    l1: u32,
}

/// A frame-sd below this reads as "signal absent" (the plain-teacher
/// `MIN_SCALE` convention, applied to the critic's root spread).
const MIN_SD: f64 = 1e-9;

/// The blended chance game — [`crate::tetris_lane::TeacherState`] plus the
/// carried last placement, the decision-root mode, and the root frame.
/// Everything the plain teacher does is preserved verbatim: the same
/// information rule (no preview, fresh bag), the same move encoding, the
/// same terminal law.
#[derive(Clone)]
pub struct BlendedState<'e> {
    board: Board,
    cur: Piece,
    lines: u32,
    tetrises: u32,
    after: bool,
    last: Option<LastPlace>,
    /// The decision root's mode (the champion convention: every leaf of one
    /// decision judged under the root's mode).
    mode: Mode,
    /// `(m̄, gain)` from the decision root's own option set; `None` at the
    /// root itself and whenever `weight == 0` (no critic work at all).
    frame: Option<(f64, f64)>,
    ctx: &'e BlendCtx<'e>,
}

impl<'e> BlendedState<'e> {
    /// The search root for one decision. When `weight > 0`, computes the
    /// alignment frame over the root's landing options (champion + critic
    /// scores); the returned state itself carries `last = None`, so the
    /// root's own `value()` stays champion-only.
    pub fn root(board: &Board, cur: Piece, mode: Mode, ctx: &'e BlendCtx<'_>) -> Self {
        let frame = (ctx.weight > 0.0).then(|| root_frame(board, cur, mode, ctx));
        Self {
            board: board.clone(),
            cur,
            lines: 0,
            tetrises: 0,
            after: false,
            last: None,
            mode,
            frame,
            ctx,
        }
    }

    /// The champion evaluator's raw score (caller units — points), identical
    /// to `TeacherState`'s seam.
    fn champ(&self, b: &Board, lines: u32, tetrises: u32) -> f64 {
        let scan = b.scan();
        let leaf = Leaf { board: b, heights: scan.heights, lines, tetrises, held: None };
        self.ctx.genome.eval_with(&leaf, self.mode, &scan)
    }

    /// The blended leaf value: champion + `w · gain · (m − m̄)`. With no
    /// carried placement (the decision root) or no frame (`weight == 0`),
    /// the champion score unchanged.
    fn blended(&self) -> f64 {
        let c = self.champ(&self.board, self.lines, self.tetrises);
        let Some((m_bar, gain)) = self.frame else {
            return c;
        };
        let Some(last) = &self.last else {
            return c;
        };
        let raw = raw_opt(
            &last.pre,
            &Placement { rot: 0, col: 0, row: 0, cells: last.cells },
            last.piece,
            self.mode,
            &self.board,
            last.l1,
        );
        let mut scratch = vec![0.0f64; self.ctx.critic.scratch_len()];
        let m = self.ctx.critic.score(&raw, &mut scratch);
        c + self.ctx.weight * gain * (m - m_bar)
    }
}

impl ChanceGame for BlendedState<'_> {
    type Action = TeacherMove;
    type Outcome = Piece;

    fn is_terminal(&self) -> bool {
        !self.after && landing_options_with(&self.board, self.cur, DropRule::FromTop).is_empty()
    }

    fn value(&self) -> f32 {
        self.blended() as f32
    }

    fn actions(&self, out: &mut Vec<(Self::Action, f32)>) {
        if self.after {
            return;
        }
        // Priors stay PURE champion (the module doc's cost decision).
        for (i, p) in landing_options_with(&self.board, self.cur, DropRule::FromTop)
            .iter()
            .enumerate()
        {
            let (b1, l1) = apply(&self.board, &p.cells);
            let prior =
                self.champ(&b1, self.lines + l1, self.tetrises + u32::from(l1 == 4)) as f32;
            let mut cells = [(0u8, 0u8); 4];
            for (dst, &(r, c)) in cells.iter_mut().zip(p.cells.iter()) {
                *dst = (r as u8, c as u8);
            }
            out.push((TeacherMove { idx: i as u16, cells }, prior));
        }
    }

    fn apply(&self, m: Self::Action) -> Self {
        let cells: Vec<(usize, usize)> =
            m.cells.iter().map(|&(r, c)| (r as usize, c as usize)).collect();
        let (board, n) = apply(&self.board, &cells);
        let mut last_cells = [(0usize, 0usize); 4];
        for (dst, &(r, c)) in last_cells.iter_mut().zip(m.cells.iter()) {
            *dst = (r as usize, c as usize);
        }
        Self {
            last: Some(LastPlace {
                pre: self.board.clone(),
                cells: last_cells,
                piece: self.cur,
                l1: n,
            }),
            board,
            lines: self.lines + n,
            tetrises: self.tetrises + u32::from(n == 4),
            after: true,
            ..self.clone()
        }
    }

    fn outcomes(&self, out: &mut Vec<(Self::Outcome, f32)>) {
        // Fresh bag: uniform over all seven — serving never sees bag state.
        for &q in &Piece::ALL {
            out.push((q, 1.0 / 7.0));
        }
    }

    fn resolve(&self, drawn: Self::Outcome) -> Self {
        Self { cur: drawn, after: false, ..self.clone() }
    }
}

/// The per-decision alignment frame over the root's own landing options:
/// `(m̄, gain)` with `gain = sd(champ)/sd(critic)` — the critic's deviation
/// from its own mean, re-expressed in champion-point units. A flat critic
/// (`sd < MIN_SD`) yields `gain = 0` (pure champion downstream).
fn root_frame(board: &Board, cur: Piece, mode: Mode, ctx: &BlendCtx<'_>) -> (f64, f64) {
    let options = landing_options_with(board, cur, DropRule::FromTop);
    let mut cs = Vec::with_capacity(options.len());
    let mut ms = Vec::with_capacity(options.len());
    let mut scratch = vec![0.0f64; ctx.critic.scratch_len()];
    for p in &options {
        let (b1, l1) = apply(board, &p.cells);
        let scan = b1.scan();
        let leaf = Leaf { board: &b1, heights: scan.heights, lines: l1, tetrises: u32::from(l1 == 4), held: None };
        cs.push(ctx.genome.eval_with(&leaf, mode, &scan));
        let raw = raw_opt(board, p, cur, mode, &b1, l1);
        ms.push(ctx.critic.score(&raw, &mut scratch));
    }
    let n = cs.len().max(1) as f64;
    let c_bar = cs.iter().sum::<f64>() / n;
    let m_bar = ms.iter().sum::<f64>() / n;
    let sd_c = (cs.iter().map(|c| (c - c_bar) * (c - c_bar)).sum::<f64>() / n).sqrt();
    let sd_m = (ms.iter().map(|m| (m - m_bar) * (m - m_bar)).sum::<f64>() / n).sqrt();
    (m_bar, if sd_m > MIN_SD { sd_c / sd_m } else { 0.0 })
}

/// One blended-teacher game — the `LanePolicy::Teacher` arm of
/// [`crate::tetris_critic::play_lane`] with the search state swapped for
/// [`BlendedState`] (that loop stays byte-stable; this one is the round-5
/// lane's own). Records every visited root option's Q as a [`Sample`],
/// identical shape to the plain collection.
#[allow(clippy::too_many_arguments)]
pub fn play_blended(
    genome: &Genome,
    critic: &TrainedMlp,
    weight: f64,
    seed: u64,
    regime: Regime,
    cap: usize,
    budget: u32,
    sink: &mut Vec<Sample>,
) -> GameStats {
    let ctx = BlendCtx { genome, critic, weight };
    let mut bag = Bag::new(seed);
    let mut board = start_board(seed, regime);
    let mut next = bag.draw();
    let mut rng = fastrand::Rng::with_seed(seed ^ TEACHER_RNG_SALT);
    let mut st = GameStats { seed, ..GameStats::default() };
    let mut decision_ms: Vec<f64> = Vec::with_capacity(cap.min(4096));
    let mut decision: u32 = 0;
    let mut search = ChancePuct::new(ChancePuctConfig { budget, ..ChancePuctConfig::default() });

    while st.pieces < cap {
        let cur = next;
        next = bag.draw();
        let options = landing_options_with(&board, cur, DropRule::FromTop);
        if options.is_empty() {
            st.topped_out = true;
            break;
        }
        let t0 = Instant::now();
        let mode = genome.mode_of(&board);
        let root = BlendedState::root(&board, cur, mode, &ctx);
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

/// Threaded blended collection over one (regime, seed list) — the
/// [`crate::tetris_critic::collect_teacher`] shape: threads across seeds,
/// per-seed blocks merged in seed order (the fit's determinism premise).
#[allow(clippy::too_many_arguments)]
pub fn collect_blended(
    genome: &Genome,
    critic: &TrainedMlp,
    weight: f64,
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
                    let stats = play_blended(genome, critic, weight, seed, regime, cap, budget, &mut sink);
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
    use crate::tetris_critic::LanePolicy;
    use crate::tetris_lane::Regime;

    /// A minimal valid TETMLP1 file (h1 = h2 = 1, tanh hidden, all-zero
    /// weights → σ(b3) everywhere) — hermetic, no sibling data needed. The
    /// w=0 arm never consults it; the w>0 smoke only needs it to be a real
    /// forward pass.
    fn toy_critic() -> TrainedMlp {
        let mut buf = Vec::new();
        let zero_f64s = |buf: &mut Vec<u8>, n: usize| {
            for _ in 0..n {
                buf.extend_from_slice(&0.0f64.to_le_bytes());
            }
        };
        buf.extend_from_slice(b"TETMLP1 ");
        buf.extend_from_slice(&33u32.to_le_bytes());
        buf.extend_from_slice(&7u32.to_le_bytes());
        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes()); // tanh hidden
        zero_f64s(&mut buf, 33); // scale lo
        // scale hi must differ from lo or every feature clips to the same
        // half of the [-1,1] range — zeros/ones keep the forward trivial but
        // VALID, which is all this fixture needs.
        for _ in 0..33 {
            buf.extend_from_slice(&1.0f64.to_le_bytes());
        }
        zero_f64s(&mut buf, 43); // w1 (h1=1 × n_in=43)
        zero_f64s(&mut buf, 1); // b1
        zero_f64s(&mut buf, 1); // w2 (h2=1 × h1=1)
        zero_f64s(&mut buf, 1); // b2
        zero_f64s(&mut buf, 1); // w3
        buf.extend_from_slice(&0.0f64.to_le_bytes()); // b3
        TrainedMlp::from_bytes(&buf).expect("toy critic parses")
    }

    /// THE seam contract: weight 0 must reproduce the plain teacher
    /// BIT-EXACTLY — same picks (pieces/lines/points), same recorded Q bits,
    /// same is_pick sequence — because the blended state's scores are the
    /// champion's scores and the searcher is the same.
    #[test]
    fn weight_zero_reproduces_the_plain_teacher_bit_exactly() {
        let genome = Genome::champion_hybrid();
        let critic = toy_critic();
        for regime in [Regime::garbage(16, 75), Regime::garbage(18, 75)] {
            for seed in 301u64..=306 {
                let mut plain: Vec<Sample> = Vec::new();
                let mut policy = LanePolicy::Teacher { budget: 200, sink: &mut plain };
                let a = crate::tetris_critic::play_lane(&genome, seed, regime, 150, &mut policy);
                let mut blended: Vec<Sample> = Vec::new();
                let b = play_blended(&genome, &critic, 0.0, seed, regime, 150, 200, &mut blended);
                assert_eq!((a.pieces, a.lines, a.points, a.tetrises), (b.pieces, b.lines, b.points, b.tetrises), "regime {regime} seed {seed}: game diverged at weight 0");
                assert_eq!(plain.len(), blended.len(), "regime {regime} seed {seed}: sample count");
                for (p, q) in plain.iter().zip(&blended) {
                    assert_eq!((p.seed, p.decision, p.is_pick), (q.seed, q.decision, q.is_pick));
                    assert_eq!(p.q.to_bits(), q.q.to_bits(), "q bits diverged (regime {regime} seed {seed})");
                    assert_eq!(p.raw.piece, q.raw.piece);
                    assert_eq!(p.raw.mode, q.raw.mode);
                }
            }
        }
    }

    /// w=0.5 with a live critic runs a real game, produces Q in (0,1), and
    /// terminates (top-out or cap) without panicking — the forward-pass smoke
    /// for the blend arithmetic itself.
    #[test]
    fn blended_smoke_runs_and_stays_in_range() {
        let genome = Genome::champion_hybrid();
        let critic = toy_critic();
        let mut sink: Vec<Sample> = Vec::new();
        let st = play_blended(&genome, &critic, 0.5, 301, Regime::garbage(16, 75), 120, 100, &mut sink);
        assert!(st.pieces > 0 && st.pieces <= 120);
        assert!(!sink.is_empty());
        assert!(sink.iter().all(|s| s.q > 0.0 && s.q < 1.0));
        // The toy critic is constant → sd_m = 0 → gain = 0 → the blended
        // teacher must ALSO reproduce the plain teacher's game here (the
        // flat-critic degenerate arm of the module contract).
        let mut plain: Vec<Sample> = Vec::new();
        let mut policy = LanePolicy::Teacher { budget: 100, sink: &mut plain };
        let a = crate::tetris_critic::play_lane(&genome, 301, Regime::garbage(16, 75), 120, &mut policy);
        assert_eq!((a.pieces, a.lines, a.points), (st.pieces, st.lines, st.points));
    }
}

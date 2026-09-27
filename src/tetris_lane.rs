//! The Tetris lane (instinct Issue 009 T5 + T6) — the measurement harness
//! for Instinct playing Tetris on its own path.
//!
//! Three arms share ONE engine (`../katgpt-rs` `katgpt-tetris`: the board
//! sim, the seeded 7-bag, the garbage starts, the rulebook champion genome
//! `68cae9d382014662` — the Bench-892 hybrid the reflexer forward-freeze law
//! pins) and ONE game loop, so a per-seed comparison is a policy comparison
//! and nothing else:
//!
//! - [`ArmKind::ReflexHead`] — free Reflex's board: the corpus-fitted
//!   Tetris head (`riir-reflex::game_heads`, Bench 881 decoded arm, λ=1)
//!   scores each option's rendered spot sentence; first strict max wins.
//!   This is the exact served posture (the head is piece-blind and
//!   state-blind by construction — the spot sentence is its whole input).
//! - [`ArmKind::TeacherPuct`] — the Issue-009 teacher: `katgpt_core::
//!   chance_puct` over the champion's own evaluator, under the information
//!   rule the issue's verdict amendment pins: **no preview, fresh bag** —
//!   exactly what serving sees. The chance model after every placement is a
//!   uniform draw over all seven pieces (a fresh bag; serving never sees
//!   bag state). Budget 0 = the highest-prior action unsearched, i.e. the
//!   1-ply champion-evaluator argmax — the shape of T8's baseline.
//! - [`ArmKind::ChampionSearch`] — the champion genome's own depth-3
//!   beam-6 search in its served posture (preview + true bag support): the
//!   ceiling context row, not a candidate.
//!
//! Every runner fits a `GameHeads` even when its arm never scores with it:
//! the head carries the SERVED GRAMMAR, and the T5 signature is decoded
//! through it (`GameHeads::decode_ok_for_test`) — the reflex side's own
//! decoder, never a re-derived class mapping. Teacher picks never read the
//! head's weights.
//!
//! The no-preview PUCT state lives HERE, not in `katgpt-rs examples/
//! common/tetris_puct.rs` (that adapter's information structure carries the
//! preview and the true bag — the serving-matched posture needs neither).
//! T9's adapter promotion (boundary-guard) consolidates the two; until then
//! this is the one local copy, ~80 lines, and the engine is NOT duplicated.
//!
//! T5 rides the same loop: [`T5Sink`] measures the within-decision value
//! structure the served 5-class input destroys (per-signature spread) and
//! a full-information oracle bound (per-decision max-per-signature pick).
//! ⚠ The oracle bound is NOT a wire-input ceiling — it knows each
//! decision's true per-signature maxima, which no wire decoder can know;
//! the wire-side measurement is katgpt-rs `tetris_10_wire_ceiling` (Bench
//! 006, incl. the demeaned-table decoder class its cross-review added).
//! The two records together: pooling costs little (≤ ~4–6 pick points),
//! so Bench 006's decoder loss is dominated by context loss →
//! WIRE-MUST-WIDEN stands.

use katgpt_core::chance_puct::{ChanceGame, ChancePuct, ChancePuctConfig, RootStat};
use katgpt_tetris::lookahead::{Bag, LINES_SCORE, apply, garbage_board};
use katgpt_tetris::rulebook::{Genome, Leaf, Mode, View, decide};
use katgpt_tetris::sim::{
    Board, DropRule, Piece, Placement, landing_options_with, outcome_features, render_spot_sentence,
};
use riir_reflex::game_heads::GameHeads;
use std::collections::HashMap;
use std::time::Instant;

/// The rulebook-champion genome the whole lane measures against (Bench 892).
pub const CHAMPION_ID: &str = "68cae9d382014662";

/// The teacher-search RNG seed convention, inherited from
/// `katgpt-rs examples/tetris_08_puct_goat` (disclosed in every record).
pub const TEACHER_RNG_SALT: u64 = 0x05EE_D892;

// ── Streams & regimes ────────────────────────────────────────────────────

/// A start position: `rows == 0` is the empty board.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Regime {
    pub rows: usize,
    pub fill: u64,
}

impl Regime {
    pub const fn garbage(rows: usize, fill: u64) -> Self {
        Self { rows, fill }
    }

    pub const EMPTY: Regime = Regime { rows: 0, fill: 0 };
}

impl std::fmt::Display for Regime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rows == 0 {
            write!(f, "empty")
        } else {
            write!(f, "garbage {}@{}%", self.rows, self.fill)
        }
    }
}

/// Start board for a seed under a regime (the garbage mix is seed-derived,
/// `lookahead::garbage_board`'s own convention).
pub fn start_board(seed: u64, regime: Regime) -> Board {
    if regime.rows == 0 {
        Board::empty()
    } else {
        garbage_board(seed, regime.rows, regime.fill)
    }
}

// ── Arms ─────────────────────────────────────────────────────────────────

/// Which policy drives the loop (stateless; per-worker state lives in
/// [`Runner`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ArmKind {
    /// Free Reflex's board (the served Tetris head).
    ReflexHead,
    /// The no-preview chance_puct teacher over the champion evaluator.
    /// `budget == 0` = the 1-ply prior argmax (the T8 baseline shape).
    TeacherPuct { budget: u32 },
    /// The champion genome's own depth-3 beam-6 search (served posture:
    /// preview + true bag support). Ceiling context, never a candidate.
    ChampionSearch,
}

impl ArmKind {
    pub fn name(self) -> String {
        match self {
            Self::ReflexHead => "reflex-head (Bench 881 decoded arm)".into(),
            Self::TeacherPuct { budget } => format!(
                "teacher-puct b{budget} (champion eval, no preview, fresh bag)"
            ),
            Self::ChampionSearch => {
                "champion-search d3 (served posture, preview + bag)".into()
            }
        }
    }

    /// Short slug for record keys.
    pub fn slug(self) -> String {
        match self {
            Self::ReflexHead => "reflex_head".into(),
            Self::TeacherPuct { budget } => format!("teacher_b{budget}"),
            Self::ChampionSearch => "champion_search".into(),
        }
    }
}

/// The fixture texts the reflex head fits from (read once, shared across
/// worker threads; each worker builds its own `GameHeads` — the fit is
/// ~ms and the head is per-worker state).
pub struct Fixtures {
    pub tetris: String,
    pub lanes: String,
    pub flappy: String,
}

/// Per-worker policy state + the shared genome.
pub struct Runner {
    pub kind: ArmKind,
    pub genome: Genome,
    /// Every arm fits the head: the reflex arm scores with it, every arm
    /// decodes the T5 signature through its grammar. Teacher picks never
    /// read the weights.
    heads: GameHeads,
    /// Counted while playing; surfaced in [`GameStats::head_decode_misses`].
    decode_misses: u32,
}

impl Runner {
    /// Panics on anchor drift — the published λ and corpus size are the
    /// contract; a head that misses them is not the served head.
    pub fn new(kind: ArmKind, fixtures: &Fixtures, genome: Genome) -> Self {
        let heads = GameHeads::build(&fixtures.tetris, &fixtures.lanes, &fixtures.flappy);
        assert_eq!(heads.lambda(), 1.0, "tetris head λ drifted (Bench 881 anchor)");
        assert_eq!(heads.n_options(), 2660, "tetris corpus size drifted");
        Self { kind, genome, heads, decode_misses: 0 }
    }

    /// The fitted tetris head's digest (the determinism anchor, recorded).
    pub fn head_digest(&self) -> String {
        self.heads.digest_hex()
    }

    /// One seed, one game. The teacher RNG is per-(seed, budget)
    /// deterministic; the bag and the garbage start are seed-derived, so
    /// every arm faces the identical piece stream and start board.
    pub fn play(
        &mut self,
        seed: u64,
        regime: Regime,
        cap: usize,
        mut sink: Option<&mut T5Sink>,
    ) -> GameStats {
        let mut bag = Bag::new(seed);
        let mut board = start_board(seed, regime);
        let mut next = bag.draw();
        let mut rng = fastrand::Rng::with_seed(seed ^ TEACHER_RNG_SALT);
        let mut st = GameStats { seed, ..GameStats::default() };
        let mut decision_ms: Vec<f64> = Vec::with_capacity(cap.min(4096));

        while st.pieces < cap {
            let cur = next;
            next = bag.draw();
            let support: Vec<Piece> = bag.remaining().to_vec();
            let options = landing_options_with(&board, cur, DropRule::FromTop);
            if options.is_empty() {
                st.topped_out = true;
                break;
            }
            let t0 = Instant::now();
            let (picked, policy_v) = match self.kind {
                ArmKind::ReflexHead => self.pick_reflex(&board, &options),
                ArmKind::TeacherPuct { budget } => {
                    self.pick_teacher(budget, &board, cur, &options, &mut rng)
                }
                ArmKind::ChampionSearch => {
                    let view = View {
                        board: &board,
                        cur,
                        next,
                        held: None,
                        hold_ready: false,
                        bag_remaining: &support,
                    };
                    let d = decide(&self.genome, &view)
                        .expect("options non-empty ⇒ the champion returns a decision");
                    (d.index, Vec::new())
                }
            };
            let ms = t0.elapsed().as_secs_f64() * 1e3;
            decision_ms.push(ms);
            st.decisions += 1;

            if let Some(sink) = sink.as_deref_mut() {
                let root_mode = self.genome.mode_of(&board);
                let rows: Vec<OptRow> = options
                    .iter()
                    .enumerate()
                    .map(|(i, p)| self.opt_row(&board, p, root_mode, policy_v.get(i).copied().flatten()))
                    .collect();
                sink.observe(DecisionRecord { seed, regime, piece: st.pieces, picked, rows });
            }

            let (nb, l1) = apply(&board, &options[picked].cells);
            board = nb;
            st.lines += l1;
            st.tetrises += u32::from(l1 == 4);
            st.points += LINES_SCORE[l1.min(4) as usize];
            st.pieces += 1;
        }

        decision_ms.sort_by(f64::total_cmp);
        st.p50_ms = decision_ms.get(decision_ms.len() / 2).copied().unwrap_or(0.0);
        st.head_decode_misses = self.decode_misses;
        st
    }

    /// The reflex head's pick: per-option spot sentence → head score →
    /// first strict max (the site's argmax convention). Returns the pick
    /// and the per-option head scores (the T5 policy column).
    fn pick_reflex(&mut self, board: &Board, options: &[Placement]) -> (usize, Vec<Option<f64>>) {
        let mut best = 0usize;
        let mut best_s = f64::NEG_INFINITY;
        let mut scores = Vec::with_capacity(options.len());
        for p in options {
            let f = outcome_features(board, p);
            let sentence = render_spot_sentence(board, p, &f);
            let s = match self.heads.score(&sentence) {
                Some(s) => s,
                None => {
                    self.decode_misses += 1;
                    f64::NEG_INFINITY
                }
            };
            scores.push(Some(s));
            if s > best_s {
                best_s = s;
                best = scores.len() - 1;
            }
        }
        (best, scores)
    }

    /// The teacher's pick: no-preview chance_puct over the champion
    /// evaluator, mode judged at the root board (the champion's own
    /// per-decision convention). Returns the pick and the per-option q
    /// (`None` where the search never visited the option).
    fn pick_teacher(
        &mut self,
        budget: u32,
        board: &Board,
        cur: Piece,
        options: &[Placement],
        rng: &mut fastrand::Rng,
    ) -> (usize, Vec<Option<f64>>) {
        // The champion's own evaluator, mode judged at the root board (the
        // champion's per-decision convention — every leaf of one decision
        // is judged under the root's mode).
        let mode = self.genome.mode_of(board);
        let g = &self.genome;
        let eval = move |b: &Board, lines: u32, tetrises: u32| -> f64 {
            let scan = b.scan();
            let leaf = Leaf { board: b, heights: scan.heights, lines, tetrises, held: None };
            g.eval_with(&leaf, mode, &scan)
        };
        let cfg = ChancePuctConfig { budget, ..ChancePuctConfig::default() };
        let root = TeacherState::root(board, cur, &eval);
        let mut search = ChancePuct::new(cfg);
        let picked = match search.search(&root, rng) {
            Some(p) => p.index,
            None => usize::MAX, // unreachable: options non-empty ⇒ actions non-empty
        };
        let stats: Vec<RootStat> = search.root_stats().collect();
        let mut qs = vec![None; options.len()];
        for s in &stats {
            if s.index < qs.len() && s.visits > 0 {
                qs[s.index] = Some(s.q as f64);
            }
        }
        (picked, qs)
    }

    /// One option's T5 row: the 5-class signature decoded through the
    /// served grammar, the champion 1-ply eval of the afterstate (mode at
    /// the decision root), and this arm's per-option policy value.
    fn opt_row(&self, board: &Board, p: &Placement, root_mode: Mode, policy_v: Option<f64>) -> OptRow {
        let f = outcome_features(board, p);
        let sentence = render_spot_sentence(board, p, &f);
        // The sentence is grammar-closed by construction (the sim's
        // renderer is pinned against this grammar), so a decode failure is
        // a hard contract break, not a row to skip.
        let v = GameHeads::decode_ok_for_test(self.heads.grammar(), &sentence);
        let mut fills = [0u8; 5];
        fills.copy_from_slice(&v[..5]);
        let (b1, l1) = apply(board, &p.cells);
        let scan = b1.scan();
        let leaf = Leaf {
            board: &b1,
            heights: scan.heights,
            lines: l1,
            tetrises: u32::from(l1 == 4),
            held: None,
        };
        OptRow {
            fills,
            eval_v: Some(self.genome.eval_with(&leaf, root_mode, &scan)),
            policy_v,
        }
    }
}

// ── The teacher's chance game (no preview, fresh bag) ────────────────────

/// A placement, inline (a tetromino is always 4 cells) — the PUCT adapter's
/// `Move`, local until T9's adapter promotion consolidates the two.
#[derive(Clone, Copy, Debug)]
pub struct TeacherMove {
    /// Index into `landing_options_with(board, cur, FromTop)`.
    pub idx: u16,
    pub cells: [(u8, u8); 4],
}

/// A DECISION state knows the piece to place and NOTHING about the next
/// draw: the chance event after every placement is the next piece, uniform
/// over all seven (a fresh bag). This is the serving information rule.
#[derive(Clone)]
pub struct TeacherState<'e> {
    board: Board,
    cur: Piece,
    lines: u32,
    tetrises: u32,
    after: bool,
    eval: &'e dyn Fn(&Board, u32, u32) -> f64,
}

impl<'e> TeacherState<'e> {
    pub fn root(board: &Board, cur: Piece, eval: &'e dyn Fn(&Board, u32, u32) -> f64) -> Self {
        Self { board: board.clone(), cur, lines: 0, tetrises: 0, after: false, eval }
    }
}

impl ChanceGame for TeacherState<'_> {
    type Action = TeacherMove;
    type Outcome = Piece;

    fn is_terminal(&self) -> bool {
        !self.after && landing_options_with(&self.board, self.cur, DropRule::FromTop).is_empty()
    }

    fn value(&self) -> f32 {
        (self.eval)(&self.board, self.lines, self.tetrises) as f32
    }

    fn actions(&self, out: &mut Vec<(Self::Action, f32)>) {
        if self.after {
            return;
        }
        for (i, p) in landing_options_with(&self.board, self.cur, DropRule::FromTop)
            .iter()
            .enumerate()
        {
            let (b1, l1) = apply(&self.board, &p.cells);
            let prior =
                (self.eval)(&b1, self.lines + l1, self.tetrises + u32::from(l1 == 4)) as f32;
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
        Self {
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

// ── Game stats ───────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct GameStats {
    pub seed: u64,
    pub pieces: usize,
    pub lines: u32,
    pub tetrises: u32,
    pub points: u64,
    pub topped_out: bool,
    pub decisions: usize,
    /// Per-decision p50 wall ms (the T3 disclosure axis).
    pub p50_ms: f64,
    pub head_decode_misses: u32,
}

// ── T5: the serving-input-contract measurement ─────────────────────

/// One option's row in a decision record.
#[derive(Clone, Debug, serde::Serialize)]
pub struct OptRow {
    /// The 5-class signature the served input carries (grammar order:
    /// holes, side, surface, height, clears).
    pub fills: [u8; 5],
    /// Champion 1-ply eval of the afterstate (mode at the decision root).
    pub eval_v: Option<f64>,
    /// This arm's per-option policy value: the reflex head's raw score
    /// (reflex rows) or the teacher's q (teacher rows, visited only).
    pub policy_v: Option<f64>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct DecisionRecord {
    pub seed: u64,
    pub regime: Regime,
    pub piece: usize,
    pub picked: usize,
    pub rows: Vec<OptRow>,
}

/// First strict maximum (lowest index on ties) — the site's argmax and the
/// head's pick convention.
pub fn first_argmax(xs: &[f64]) -> usize {
    let mut best = 0;
    for (i, &x) in xs.iter().enumerate() {
        if x > xs[best] {
            best = i;
        }
    }
    best
}

/// Streaming aggregate for one (source arm, regime) cell.
///
/// The ceiling model: equal-signature options are indistinguishable to ANY
/// decoder on the served input (the head scores per option, so equal
/// signatures get equal scores), so the BEST possible signature-level pick
/// is the lowest-index option whose signature attains the decision's best
/// value. [`Self::ceiling_agree`] counts the decisions where that pick is
/// the eval-optimal one; [`Self::ceiling_loss_*`] prices the rest.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct T5Aggregate {
    pub n_decisions: u64,
    pub n_options: u64,
    /// Σ distinct signatures per decision (vs Σ options ⇒ collision rate).
    pub n_sig_sum: u64,
    /// The best possible signature-level pick is the eval argmax.
    pub ceiling_agree: u64,
    /// Σ/max value regret of that best possible pick (0 ⟺ agreed).
    pub ceiling_loss_sum: f64,
    pub ceiling_loss_max: f64,
    /// The arm's ACTUAL pick equals the eval argmax (reflex rows: how far
    /// the deployed head is from eval-optimal; teacher rows: context).
    pub actual_pick_agree: u64,
    /// Σ/max eval value the actual pick forfeits vs the decision's best.
    pub actual_pick_loss_sum: f64,
    pub actual_pick_loss_max: f64,
    /// Decisions where some signature group carries a nonzero within-group
    /// value spread — information the served input throws away.
    pub wg_spread_rows: u64,
    /// Σ of the per-decision MAX within-group spread (raw value units).
    pub wg_spread_sum: f64,
    /// Decisions whose max within-group spread is ≤ 1% / 10% of the
    /// decision's eval range.
    pub wg_spread_le_1pct: u64,
    pub wg_spread_le_10pct: u64,
    /// Decisions where EVERY option carries a policy value — the
    /// policy-value ceiling is measured on those only, never pooled.
    pub policy_rows: u64,
    pub policy_ceiling_agree: u64,
    pub policy_ceiling_loss_sum: f64,
    /// The first 64 raw records, for the record's examples (bounded).
    #[serde(skip)]
    pub examples: Vec<DecisionRecord>,
}

impl T5Aggregate {
    /// `policy_is_search_q` gates the policy-value ceiling: search q is a
    /// genuine ranking the signatures might not express; per-option head
    /// scores are CONSTANT within a signature group, so the block would be
    /// self-referential there (always agree, always zero loss).
    fn observe(&mut self, rec: &DecisionRecord, policy_is_search_q: bool) {
        self.n_decisions += 1;
        self.n_options += rec.rows.len() as u64;
        if self.examples.len() < 64 {
            self.examples.push(rec.clone());
        }
        let evals: Vec<f64> = rec.rows.iter().filter_map(|r| r.eval_v).collect();
        if evals.len() != rec.rows.len() || evals.is_empty() {
            return;
        }
        let global_max = evals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let global_min = evals.iter().copied().fold(f64::INFINITY, f64::min);
        let range = (global_max - global_min).max(0.0);

        // Per-signature max value (deterministic values; iteration order
        // never decides anything below — lookups only).
        let mut groups: HashMap<[u8; 5], f64> = HashMap::new();
        let mut group_min: HashMap<[u8; 5], f64> = HashMap::new();
        for (r, v) in rec.rows.iter().zip(&evals) {
            let e = groups.entry(r.fills).or_insert(f64::NEG_INFINITY);
            if v > e {
                *e = *v;
            }
            let m = group_min.entry(r.fills).or_insert(f64::INFINITY);
            if v < m {
                *m = *v;
            }
        }
        self.n_sig_sum += groups.len() as u64;

        // The best possible signature-level pick: the lowest-index option
        // whose signature attains the decision's best value.
        let ceiling_pick = rec
            .rows
            .iter()
            .position(|r| groups[&r.fills] == global_max)
            .unwrap_or(usize::MAX);
        let argmax_i = first_argmax(&evals);
        if ceiling_pick == argmax_i {
            self.ceiling_agree += 1;
        }
        let loss = (global_max - evals.get(ceiling_pick).copied().unwrap_or(global_max)).max(0.0);
        self.ceiling_loss_sum += loss;
        self.ceiling_loss_max = self.ceiling_loss_max.max(loss);

        // The arm's actual pick, priced on the eval scale.
        if rec.picked < evals.len() {
            if rec.picked == argmax_i {
                self.actual_pick_agree += 1;
            }
            let pl = (global_max - evals[rec.picked]).max(0.0);
            self.actual_pick_loss_sum += pl;
            self.actual_pick_loss_max = self.actual_pick_loss_max.max(pl);
        }

        // Within-group spread: the value the served input cannot see.
        let mut max_spread = 0.0f64;
        for (sig, gmax) in &groups {
            let gmin = group_min[sig];
            max_spread = max_spread.max(gmax - gmin);
        }
        if max_spread > 0.0 {
            self.wg_spread_rows += 1;
        }
        self.wg_spread_sum += max_spread;
        if range > 0.0 {
            let ns = max_spread / range;
            if ns <= 0.01 {
                self.wg_spread_le_1pct += 1;
            }
            if ns <= 0.10 {
                self.wg_spread_le_10pct += 1;
            }
        } else {
            self.wg_spread_le_1pct += 1;
            self.wg_spread_le_10pct += 1;
        }

        // Policy-value ceiling (all options valued only) — search-q rows
        // only, per the gate above.
        let pv: Vec<Option<f64>> = rec.rows.iter().map(|r| r.policy_v).collect();
        if policy_is_search_q && pv.iter().all(|p| p.is_some()) {
            self.policy_rows += 1;
            let pv: Vec<f64> = pv.into_iter().map(|p| p.expect("all Some")).collect();
            let p_max = pv.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let mut p_groups: HashMap<[u8; 5], f64> = HashMap::new();
            for (r, v) in rec.rows.iter().zip(&pv) {
                let e = p_groups.entry(r.fills).or_insert(f64::NEG_INFINITY);
                if v > e {
                    *e = *v;
                }
            }
            let p_ceiling = rec
                .rows
                .iter()
                .position(|r| p_groups[&r.fills] == p_max)
                .unwrap_or(usize::MAX);
            if p_ceiling == first_argmax(&pv) {
                self.policy_ceiling_agree += 1;
            }
            self.policy_ceiling_loss_sum +=
                (p_max - pv.get(p_ceiling).copied().unwrap_or(p_max)).max(0.0);
        }
    }

    pub fn merge(&mut self, other: &T5Aggregate) {
        self.n_decisions += other.n_decisions;
        self.n_options += other.n_options;
        self.n_sig_sum += other.n_sig_sum;
        self.ceiling_agree += other.ceiling_agree;
        self.ceiling_loss_sum += other.ceiling_loss_sum;
        self.ceiling_loss_max = self.ceiling_loss_max.max(other.ceiling_loss_max);
        self.actual_pick_agree += other.actual_pick_agree;
        self.actual_pick_loss_sum += other.actual_pick_loss_sum;
        self.actual_pick_loss_max = self.actual_pick_loss_max.max(other.actual_pick_loss_max);
        self.wg_spread_rows += other.wg_spread_rows;
        self.wg_spread_sum += other.wg_spread_sum;
        self.wg_spread_le_1pct += other.wg_spread_le_1pct;
        self.wg_spread_le_10pct += other.wg_spread_le_10pct;
        self.policy_rows += other.policy_rows;
        self.policy_ceiling_agree += other.policy_ceiling_agree;
        self.policy_ceiling_loss_sum += other.policy_ceiling_loss_sum;
        for ex in &other.examples {
            if self.examples.len() < 64 {
                self.examples.push(ex.clone());
            }
        }
    }

    pub fn mean_ceiling_loss(&self) -> f64 {
        if self.n_decisions == 0 {
            0.0
        } else {
            self.ceiling_loss_sum / self.n_decisions as f64
        }
    }

    pub fn mean_wg_spread(&self) -> f64 {
        if self.n_decisions == 0 {
            0.0
        } else {
            self.wg_spread_sum / self.n_decisions as f64
        }
    }
}

/// Collects decision records from play, aggregating ONE (source, regime)
/// cell; the bench merges cells across workers.
pub struct T5Sink {
    agg: T5Aggregate,
    policy_is_search_q: bool,
}

impl T5Sink {
    /// `policy_is_search_q`: the arm's policy column carries teacher search
    /// q (gated ceiling analysis) rather than per-option head scores.
    pub fn new(policy_is_search_q: bool) -> Self {
        Self { agg: T5Aggregate::default(), policy_is_search_q }
    }

    pub fn into_agg(self) -> T5Aggregate {
        self.agg
    }

    pub fn agg(&self) -> &T5Aggregate {
        &self.agg
    }

    pub fn merge_agg(&mut self, other: &T5Aggregate) {
        self.agg.merge(other);
    }

    fn observe(&mut self, rec: DecisionRecord) {
        self.agg.observe(&rec, self.policy_is_search_q);
    }
}

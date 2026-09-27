# Issue 009 — Instinct plays Tetris on the arena (a real board, not a text-suite card)

**Status:** OPEN — filed 2026-09-27 (owner direction). The placeholder card is pulled from the arena until this lands. HOW section added 2026-09-27 (training flow: search-distill via expert iteration, chance_puct teacher — verdict AGREE after 2 rounds); T5–T9 gate T7's start. KARC adjudicated same day: refused as drop predictor, added as a T8 modelless readout arm.

## Why

reflex-site `f7a0574` put an "Instinct (hybrid) · trained specialists" card in
every arena game. It was a list of text-suite accuracies (ag_news, emotion, …)
standing where a game board belongs, and its own note admitted it: *"a text-suite
card, not a game board: game spots answer through its Reflex half"*. The owner
expected Instinct playing Tetris. That card was an unrelated patch, so it is
removed (reflex-site, same day) until Instinct has its own play.

There was no plan for this. Proposal 001 A4/A10 names the tetris/flappy/lanes
heads as PUBLIC-RELEASE vessels, which is the substrate, but no task ever made
Instinct DECIDE a Tetris spot.

## Tasks

- [ ] T1 — Instinct `/decide` answers a Tetris spot through its OWN path
      (the tetris head vessel through the Instinct serving composition, not a
      pass-through to Reflex). If the answer is byte-identical to Reflex's,
      the board shows nothing new. Record that as a negative, don't ship it
      as a lane.
- [ ] T2 — Record a seed-607 walk (the arena's `tetris_walk` protocol) and
      chain-verify it with `arena_demo_check.mjs` like the other lanes.
- [ ] T3 — GOAT per Issue 008: Instinct must strictly beat free Reflex's
      board (score / lines / pieces on the same seed), with per-spot p50
      disclosed. A tie sells nothing.
- [ ] T4 — Site half (reflex-site): a `tc-instinct` board card (canvas +
      readout like every other board), raw still last. Arena smoke re-pins
      the grid. `arena_demo_smoke.mjs` now reds on any `*-hybrid` card, so a
      text card can't come back.
- [-] Flappy / lanes: after Tetris.

## How — the training flow (owner question 2026-09-27; verdict ×2 rounds → AGREE)

T1–T4 gate the WHAT (own path, strict beat, walk verify). This section records
the HOW. Verdict provenance: 2026-09-27, two-round verdict ping-pong. Round 1
REVISE moved the recipe off Bench 205 (Moka PUCT is TWO-PLAYER 9×9 GO — negamax
sign flip, no chance nodes; its budget 200 / c_puct 2.5 / top_k 8 are Go
settings and do not transfer; "self-play" is also the wrong term single-player —
this is EXPERT ITERATION) onto the Tetris-native precedent, and round 2 added
the two amendments folded into T6 and T8 below.

**Recipe — search-distill via expert iteration** (not GRPO-from-scratch; GRPO
stays the recorded fallback if the critic's eval cost proves prohibitive). The
lineage that decides it: Bench 892 `chance_puct` passes G1–G4 as a MODELLESS
search yet wins only the hardest regime at 5–150× latency, and Bench 205's own
verdict line says "NOT a modelless gain — the MODEL carries the gain". So the
modelless search is the TEACHER, the trained critic is what serves, and a
modelless-only board does not carry the gain.

- **Teacher:** `katgpt_core::chance_puct` (7-bag chance nodes, sigmoid priors,
  mean backup) over the rulebook-champion evaluator (`68cae9d382014662` — the
  Bench-891/892 champion riir-reflexer's forward-freeze law already pins) on the
  engine parity-checked against the arena's seed-607 walk (60/60 identical
  picks). Budget: Bench 205's budget 200 / c_puct 2.5 are GO settings and do
  NOT transfer — pin from the Bench-892 Tetris regimes
  (`tetris_08_puct_goat`: budgets 100/400/1600) and re-verify the teacher
  margin at T6.
- **History/sequence model: NONE, by construction.** The 7-bag is a uniform
  random permutation — the ONLY structure in drop history is the current bag's
  remaining composition, and `chance_puct` already consumes exactly that
  (samples the remaining-bag distribution at its chance nodes). A learned
  higher-order predictor over recent drops (LEO-style sequence memory,
  `QuestLeoScorer` transplant, …) would be fitting the RNG stream — the
  memorize-the-fixture class; Proposal 042's Phase 0 also closed UNMEASURABLE.
  **KARC included** (Plan 308, `katgpt_core::karc`): its GOAT G1 defines its
  domain of validity — DETERMINISTIC temporal structure (double-scroll
  attractor, 8 Lyapunov times; in riir-engine, HLA belief trajectories, a
  semantic-domain signal with real inertia). The 7-bag is i.i.d.-within-bag:
  `P(next | history) = P(next | bag state)` exactly, so the delay ring carries
  zero predictive bits and KARC would learn the uniform marginal that exact
  enumeration already gives for free (the Research-322 category-confusion
  class). Cheap closing gate if ever contested: measure `H(next | last-k)` —
  it equals `H(uniform over remaining bag)` by construction. If T5's widened
  wire carries the bag state or a preview, consume it DIRECTLY — exact
  enumeration of a known distribution beats any predictor of it. The
  head-to-head ("duel") instrument IS in the design already: paired per-seed
  win/tie/loss + sign test for selection, never memory. **Where KARC DOES
  belong: the value-function readout — see T8's second modelless arm.**
- **Serve:** ONE 1-ply afterstate value critic, argmax over the offered options.
  A placement is deterministic, so V(afterstate) IS the per-option critic
  Q(s,a) — Issue 005's value-head category rule holds. Policy+V at 1-ply is
  redundant: serve one or pre-register a blend, and report the
  student-vs-teacher gap. If 1-ply falls short the next rung is the serve-time
  expectation over the 7 next pieces (~240 head evals, <1 ms) — NEVER
  serve-time PUCT.
- **GOAT posture:** promotion gate = T3 (strict paired-seed beat) with T8 as
  the attribution arm; G2/G4 are inherited from chance_puct's own gate
  (µs latency, alloc-free on a heap-free game) and must be RE-ASSERTED on the
  Instinct serving path at T9. G1 calibration is NOT binding for pure argmax
  play with no confidence output — but if this lane ever serves confidence or
  abstention, the Report-the-Floor rule binds (beat the conformal-naive
  floor, katgpt-rs Plan 340; the repo AGENTS G1 wording).
- **Labels:** record as search-distill. Issue 005's H3 (serve-time PUCT over
  chains) stays open and unmeasured — this is NOT an H3 landing.
- **Elo:** readout/companion only. For a single-player game the direct
  instrument is paired per-seed win/tie/loss + sign test on held-out seeds; the
  selector stays Pareto rank-0 + Beta-LCB (Issue 005's instrument section); the
  gate stays T3.

**The binding constraint (why the order below is rigid):** the arena wire is
text-only — a state sentence plus one spot sentence per option, and each spot
decodes to FIVE coarse class ordinals (holes/side/surface/height/clears;
`riir-reflex/src/game_heads.rs`). No next-piece preview, no raw grid. A trained
head fed only those five categorical inputs is a tiny lookup table and reflex's
ridge fit is already near that ceiling — distilling into it would likely TIE,
which T1 records as a negative. Hence T5 first, and T6's information rule.

Execution order: T5 → T6 → T7 → T8 → T9. T7 must not start before T5 and T6
land.

- [ ] T5 — **Pin the serving input contract.** Measure whether the state
      sentence carries enough for a richer decoder; else widen the wire so the
      Instinct lane receives the raw afterstate grid (protocol change touching
      reflex-site T4 + the reflex contract). If the widened wire is offered to
      Instinct ONLY, record that here as a deliberate owner decision (the
      modelless lane keeps the 5-class text).
- [ ] T6 — **Teacher check, serving-matched, BEFORE any distillation.**
      chance_puct over the rulebook champion must STRICTLY beat free Reflex's
      board on held-out seeds in a separating regime (garbage starts of the
      Bench-892 16:75/18:75 class, or no-cap + score — a saturated
      both-survive-to-cap run ties and sells nothing; keep seed 607 beside it).
      Information rule (verdict amendment): run with NO PREVIEW and a FRESH
      BAG — exactly what serving sees — else the check can pass on information
      the student never gets. (Critic TARGETS may still be mined with the full
      teacher: a board's value averages over the preview anyway.) A teacher
      that cannot win under serving information makes T7 pointless — this
      check fails cheaply before GPU time.
- [ ] T7 — **Expert iteration.** Round-0 SFT prior from
      `tetris_oracle_laya_en_v3` (BLAKE3-pinned; NEVER ships — by construction
      it reproduces Reflex, which is T1's recorded negative). Targets =
      search-root Q or a truncated n-step return, sigmoid-normalised (the
      chance_puct way — never a raw ~1000-piece episode outcome; sparse and
      noisy). 2–3 iterations. Training seeds DISJOINT from seed 607 and the T3
      seeds (a pinned seed is a memorizable piece sequence). lr=0 control arm
      separates a training gain from the fixture prior.
- [ ] T8 — **Ablation — the win must come from the trained part.** Baseline =
      1-ply rulebook-champion evaluator fed the SAME (widened) input as the
      trained head (verdict amendment). The trained critic must strictly beat
      THIS baseline, not just Reflex: preview search + Dellacherie-style rules
      are modelless gains that belong in free Reflex, and shipping them there
      raises Instinct's bar (Issue 008 root cause 1). **Second modelless arm
      (KARC readout, added 2026-09-27):** a closed-form KARC basis-ridge fit of
      V(afterstate) over the same board features (Plan 308 — no delay ring,
      plain basis expansion + ridge; the reflex head's linear micro-fit with a
      nonlinear basis; modelless per Plan 332's own annotation). The trained
      critic must beat BOTH arms; if the KARC readout wins, the modelless path
      wins and that is the honest verdict under the modelless-first mandate
      (same vessel serving, closed-form weights, no expert iteration). Note
      the readout upgrade does NOT escape T5: the reflex head's ceiling is the
      5-coarse-class input, not the ridge shape.
- [ ] T9 — **Mint + serve.** riir-train trains + mints the PUBLIC-RELEASE
      vessel (Proposal 001 A4/A10 — no game-IP content) via `vessel-mint`;
      file the training run as its own riir-train issue. Instinct serves the
      1-ply critic through its own path (T1); per-spot p50 disclosed with T3's
      numbers. Adapter home: promote the katgpt-rs
      `examples/common/tetris_puct.rs` adapter out of examples/ via the
      boundary-guard skill — do NOT write a fourth Tetris engine (three
      already exist: reflex-site JS, katgpt-rs examples, riir-reflex).

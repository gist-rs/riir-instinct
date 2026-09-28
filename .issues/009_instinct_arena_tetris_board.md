# Issue 009 — Instinct plays Tetris on the arena (a real board, not a text-suite card)

**Status:** OPEN — filed 2026-09-27 (owner direction). The placeholder card is pulled from the arena until this lands. HOW section added 2026-09-27 (training flow: search-distill via expert iteration, chance_puct teacher — verdict AGREE after 2 rounds); T5–T9 gate T7's start. KARC adjudicated same day: refused as drop predictor, added as a T8 modelless readout arm. **T5 MEASURED 2026-09-27 (Bench 006): WIRE-MUST-WIDEN — the five-ordinal spot wire carries at most 0.30 of the champion 1-ply evaluator's ranking (maximal decoder, full v2 state sentence included; 0.5470 demeaned) — the widened wire (raw afterstate, Instinct-only sidecar) is a precondition of the serve path, and its two cross-repo halves (reflex contract sidecar field, reflex-site sender) are the next lands. T6 MEASURED (Bench 007): PASS — the serving-matched teacher beats free Reflex by Δ+977.9 lb95 +969.7 pieces (b1600, paired seeds 1..=20); T7 unblocked. T8's second modelless arm MEASURED (Bench 009): NEGATIVE — the closed-form KARC basis-ridge critic (d=202) loses to b0 at every teacher budget (377.4 vs 907.8 pieces at b1600 targets, 2/6/12); b0 stands as the modelless floor and the trained critic's bar is b0 AND that arm. T7 substrate landed (feature contract + collector + fit + eval lane); training seeds 201..=300 pinned, targets at b1600 minimum. T7 EXECUTED 2026-09-28 (Bench 010): NEGATIVE at round-1 capacity — the trained critic beats the ridge arm in both regimes but loses to b0 (554.6 vs 907.8; 427.3 vs 532.5); no mint/serve; re-open levers are tail-targeted loss or critic-guided search.**

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

- [x] T5 — **Pin the serving input contract.** MEASURED 2026-09-27 —
      **WIRE-MUST-WIDEN** (`.benchmarks/006_wire_ceiling/BENCH.md`; the
      instrument = katgpt-rs `examples/tetris_10_wire_ceiling.rs`,
      `required-features = ["template_decode"]`). champ1ply (the champion
      genome with NextPreview OFF + depth 1 — `plies()` clamps to ≥2 while
      the preview rule is on, and depth-2 injects `TOPOUT` values; both
      traps fixed) vs the maximal wire decoder (saturated cell tables over
      the 5-tuple ⊗ piece ⊗ the full v2 state sentence, fallback to the
      reflex-head-shaped ridge): rank agreement **0.3002** (n = 378 301
      held decisions), board ratio **0.001** (90 pts / 19 pieces vs 107 638
      / 3153), all 120 wire games top out, sign p < 0.0001 — both
      pre-declared MUST-WIDEN gates fire. Occupancy ruled out (t1 dense,
      agreement still 0.147). The binding-constraint hypothesis is
      confirmed quantitatively: the wire ceiling is far below tie.
      **Design decision recorded (per this task's own instruction): the
      wire widens INSTINCT-ONLY** — the Instinct `/decide` request carries
      the raw afterstate (grid rows + piece + bag remainder) as a sidecar
      sidecar beside the unchanged 5-class text; the modelless lane, v0.2.x
      serving, and every fixture/pin stay byte-identical. Owed lands:
      the reflex contract's optional sidecar field + the reflex-site
      sender/board-card half (T4's session). T6 (teacher check,
      serving-matched) is now the gate on all GPU spend; note the probe
      already shows the full-feature 1-ply champion is very strong under
      serving information (3153 pieces / 1256 lines avg on held garbage
      seeds), so the T6 separating-regime bar is reachable.
      **REFLEX HALF LANDED 2026-09-28 (riir-reflex `0c9fcab`): the
      `/decide` wire carries the named optional `sidecar` member —
      parse-and-ignore on the modelless lane (byte-identical answers
      pinned with/without + permissive garbage shapes + malformed-body
      still 400s; README wire doc updated). Permissive by contract: this
      half does NOT pin the afterstate schema — the consuming side pins
      it when T9's serving lane lands. Remaining halves: the instinct
      sidecar acceptance/consumption (T1/T9) + the reflex-site sender
      (T4's session).**
      **CROSS-MEASURED same day (`.benchmarks/008_signature_spread`, a
      second instrument, run concurrently — the Issue-825 class, resolved
      by cross-reading both instruments + a Claude verdict round):** the
      5-tuple pools value-different options in ~89% of teacher-play
      decisions (mean max within-group spread ≈ 29 pts), yet the
      per-decision max-oracle still reaches the eval argmax 95% of the
      time — pooling costs ≤ ~4–6 pick points, so 006's decoder loss is
      dominated by CONTEXT loss (the wire cannot tell which signature
      group wins in a new decision). Consistent with 006; the spread
      record's first draft read the oracle as "no-widening" and was
      re-scoped (a ceiling computed WITH the full board is not a wire
      ceiling).
      **Demeaned-decoder addendum (the verdict round's owed run, landed
      katgpt-rs `5961e0991`):** 006's tables averaged ABSOLUTE values —
      a target that tracks position goodness as much as option rank. The
      demeaned-table class (cells bumped with value − decision mean;
      `wire-t1d`/`wire-t3d`) more than doubles the best agreement:
      **0.5470** (t1d; mean rank of the champ pick 0.978, top-3 89.2%) vs
      t3's 0.3002 — **~half of the first-read gap was the ESTIMAND, not
      missing wire information; the information is still the bound** —
      and 0.547 remains ≤ the 0.85 MUST-WIDEN gate, board ratio 0.013,
      sign p < 0.0001. Both decoder classes fail; WIRE-MUST-WIDEN is
      robust across the family.
- [x] T6 — **Teacher check, serving-matched, BEFORE any distillation.**
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
      **LANDED 2026-09-27 (`.benchmarks/007_tetris_teacher_check`;
      harness `src/tetris_lane.rs` + `benches/tetris_teacher_check`,
      feature `tetris_goat`): PASS — T7 UNBLOCKED.** Teacher = chance_puct
      over the champion evaluator, no preview + fresh bag (uniform-7
      chance), budgets 0/100/400/1600 (c1.5 k8 — the 08-goat posture, NOT
      Bench 205's Go settings), paired seeds 1..=20, seed 607 beside
      (excluded from every gate column), teacher rng seed ^ 0x05EE_D892.
      Garbage 16@75 cap 1000: reflex-head 22.1 pieces/g (0/20 survive) vs
      teacher b1600 1 000.0 pieces/g 20/20 — paired Δ+977.9, lb95 +969.7
      (house instrument `stats::paired_upper_bound_f64`). Garbage 18@75:
      Δ+788.2, lb95 +612.2, W/T/L 19/1/0; the budget ladder separates
      (559 → 561 → 753 → 802). Empty-board context (cap 300): reflex
      67.4 pieces/g, 0/20 to cap (seed 607 tops out at 44); teacher b400
      20/20. **Even b0 — the bare 1-ply champion evaluator, T8's baseline
      shape — wins 18–20/0/1**, so the T8 bar is now concrete: the trained
      critic must beat that b0 row fed the WIDENED input. Per-spot p50
      disclosed (b1600 ~5 ms in-process; the T9 serving posture is 1-ply
      ~µs — G2/G4 re-asserted there).
      **Information rule (pinned):** this teacher ran on a FRESH BAG — less
      than the widened wire will carry (grid + piece + bag remainder); the
      T7 teacher consumes the bag remainder once the sidecar ships. A
      teacher that wins on less wins all the more on more.
      **Seed pins for T7:** training seeds must avoid ALL of 1..=40 (006's
      training), 101..=140 (006's held-out), 1..=20 (this bench's eval),
      and 607 — **suggested T7 training set: 201..=300**; the eval/T3 set
      stays 1..=20 + 607-beside.
- [-] T7 — **Expert iteration.** Round-0 SFT prior from
      `tetris_oracle_laya_en_v3` (BLAKE3-pinned; NEVER ships — by construction
      it reproduces Reflex, which is T1's recorded negative). Targets =
      search-root Q or a truncated n-step return, sigmoid-normalised (the
      chance_puct way — never a raw ~1000-piece episode outcome; sparse and
      noisy). 2–3 iterations. Training seeds DISJOINT from seed 607 and the T3
      seeds (a pinned seed is a memorizable piece sequence). lr=0 control arm
      separates a training gain from the fixture prior.
      **T7 MEASURED 2026-09-28 (`.benchmarks/010_tetris_trained_playoff`; trainer
      `riir-train crates/riir-train-engine/examples/tetris_critic_trainer`;
      play-off `benches/tetris_critic_goat --model`, the `LanePolicy::Mlp`
      arm): NEGATIVE at round-1 capacity — b0 stands.** The trained MLP critic
      (43→128→128→1, tanh hidden, f64, BCE on teacher q + 0.3×pairwise rank,
      b1600 dataset, val agree 0.7110 — CLEARS the ridge's 0.66 plateau, the
      linear-in-basis hypothesis confirmed; R² 0.2765; Arm A law held 0.711
      vs control 0.378) beats the ridge arm decisively in BOTH regimes
      (16@75: 554.6 vs 218.6, Δ+304.4 lb95 +96.5; 18@75: 427.3 vs 117.0,
      Δ+325.9 lb95 +104.4 — also above the ridge's own b1600 cell 377.4) but
      LOSES to b0 (907.8 / 532.5; trained-vs-b0 lb95 −609.2 / −359.7). The
      T3 gate (beat b0 AND the ridge, both regimes) FAILS → no mint, no
      serve wiring; round 2 stays gated. The informative finding:
      **imitation top-1 agreement is not play strength** — agreement rose
      0.66→0.71 while play went 218→555, still 350 short of b0; the residual
      disagreement concentrates in catastrophic placements. A re-open needs
      a tail-targeted loss or critic-guided search (not more top-1), or a
      much larger fit (perfect-imitation ceiling ≈ teacher ≈ 985 > 908, so
      the prize exists). Serve G2 measured anyway: trained p50 0.228
      ms/decision (1.3% of the 1 ms bar; ridge 0.006 ms). T5's parity
      contract LANDED + green: `tests/tetris_critic_parity` — the
      `TrainedMlp::score` serve-side forward replays the trainer bit-exactly
      (512/512 fixture rows, f64 op order, format v1 with the activation id
      in the header; model digest `a12f4a69c167b207`).
      **T7 ROUND-2 MEASURED 2026-09-28 (`.benchmarks/017_tetris_round2_playoff`; trainer `riir-train data/tetris_critic_r2/`, digest `66069784fb0fadd1`): NEGATIVE — the tail-targeted loss went BACKWARDS on play; round-1 weights stand.** The lever (pre-registered above): gap-weighted rank pairs (`--rank-gap 1.0`) + tail-weighted BCE (`--bce-tail 4.0 --bce-tail-pow 2.0`, the low-q tail 5×). Val agreement fell 0.7110 → 0.6270 (below the ridge's own 0.657 plateau) and play fell with it: 16@75 173.5 vs round-1 554.6 (vs b0 907.8, vs ridge 218.6 — lost to the RIDGE too, Δ−61.0 lb95 −233.6); 18@75 89.5 vs 427.3. The T3 gate (beat b0 AND the ridge, both regimes) FAILS by a wider margin than round 1. The honest reading: the tail-upweighting over-fit the tail (labels ≈0 everywhere) at the cost of the value surface's shape — round-1's "agreement ≠ strength" lesson cuts both ways: destroying agreement destroyed strength. **Round-1 artifacts (digest `a12f4a69c167b207`) untouched, still the strongest critic. Standing round-3 levers: critic-guided search, or a much larger fit (ceiling ≈ teacher ≈ 985 > b0 908); if the loss is revisited, run the gap and tail levers in ISOLATION first — this round confounded them.**
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
      **Second arm MEASURED 2026-09-27 (`.benchmarks/009_tetris_critic_arm`;
      harness `src/tetris_critic.rs` + `benches/tetris_critic_goat`, feature
      `tetris_goat`): NEGATIVE for the modelless path — b0 stands.** The
      closed-form KARC chebyshev-4 basis-ridge critic (d=202; mode×champion-set
      interactions generalize the FSM; fit on the teacher's sigmoid-normalised
      search-root Q; λ selected on val 281..=300 by top-1 agreement) beats free
      Reflex easily but loses to b0 at both teacher budgets: 16@75 b0 907.8 vs
      critic 218.6 (b400 targets) / 377.4 (b1600 targets), 2/2/16 and 2/6/12;
      18@75 the same shape. b0's row reproduces T6 at 20/0/0 vs reflex
      (Δ+881.0 lb95 +752.1) — a cross-bench consistency pin. Determinism:
      digest + all eval numbers byte-identical across worker counts. The
      trained critic's bar is now concrete: strictly beat b0 AND this arm
      (digest `48b92d26b04f3ecb`). T7 GO — see the notes on T7 above.
- [ ] T9 — **Mint + serve.** riir-train trains + mints the PUBLIC-RELEASE
      vessel (Proposal 001 A4/A10 — no game-IP content) via `vessel-mint`;
      file the training run as its own riir-train issue. Instinct serves the
      1-ply critic through its own path (T1); per-spot p50 disclosed with T3's
      numbers. Adapter home: promote the katgpt-rs
      `examples/common/tetris_puct.rs` adapter out of examples/ via the
      boundary-guard skill — do NOT write a fourth Tetris engine (three
      already exist: reflex-site JS, katgpt-rs examples, riir-reflex).

# Bench 009 — the T8 modelless second arm: a closed-form KARC basis-ridge value critic

**Status:** RECORD — landed 2026-09-27 (riir-instinct Issue 009 T8's second modelless arm + T7's substrate). **VERDICT: the closed-form arm does NOT clear the T8 bar — b0 (the champion 1-ply evaluator) stands as the modelless floor; T7's trained critic must beat b0 AND this arm.** T7 GO with the measured notes below.

## The question

Issue 009 T8's verdict amendment added a MODELLESS endpoint the trained critic must beat: a closed-form KARC basis-ridge fit of the teacher's Q ("no delay ring, plain basis expansion + ridge"). The modelless-first mandate requires measuring it BEFORE any expert-iteration (T7) spend: if it already beats b0, no training is justified; if not, T7 carries the burden with this infrastructure in hand. Either way this arm is on the critical path.

## What landed

- **`src/tetris_critic.rs`** (feature `tetris`): the raw feature contract (33 numeric features: pre-clear `OutcomeFeatures` — Dellacherie's landing/eroded structure the afterstate alone destroys — plus the post-clear afterstate `BoardScan` set and the champion-rulebook height extras wells/deep_well/nine_one/flat_top), the design (bias + `ChebyshevBasis<4>` per scaled coordinate + mode×11 champion-set interactions (generalizing the champion's 3-mode FSM to a smooth blend) + piece×4 placement interactions + 8 curated pairs = **d = 202**), the one-pass Gram/cov accumulator + closed-form f64 ridge solve (`ridge_solve_direct_f64`, the karc re-export), the teacher-sample collection lane loop (`play_lane`, two policies: Teacher / Argmax), and 2 module tests (linear-target recovery, expansion determinism).
- **`benches/tetris_critic_goat.rs`** (feature `tetris_goat`): collect → λ-sweep on val → final refit → the T3-protocol eval with the T8 gate. Per-sample λ multipliers; selection by val top-1 agreement with the teacher pick, tiebreak val R², then lowest λ.
- Cargo: `tetris` feature gains `katgpt-core/karc_forecaster` (the Plan-308 fit substrate: the sealed `KarcBasis` + the ridge re-export; default-on upstream, opt-in here with the lane); `[[bench]] tetris_critic_goat` required-features row (repo-birth gate discipline).

## Protocol

- Teacher: chance_puct over the champion evaluator, **no preview + fresh bag** (the T6 information rule), mode at the decision root, rng seed ^ 0x05EE_D892. Targets = search-root Q ∈ (0,1) — the sigmoid-normalised chance_puct value (the issue's "the chance_puct way").
- Two teacher-budget cells, everything else identical:
  - **b400** (T6's mid posture): train seeds 201..=280, val 281..=300 — 1,010,138 + 264,582 samples, teacher mean 798 pieces.
  - **b1600** (T6's best posture): 1,068,934 + 280,261 samples.
- Seed disjointness: train/val avoid 006's training (1..=40) + held-out (101..=140), the eval 1..=20, and 607.
- Eval: garbage 16@75 + 18@75, cap 1000, paired seeds 1..=20, seed 607 beside (excluded from every gate column). Same fixtures/genome pins as Bench 007.
- Box: M3 Max, AC, load 6.2–12.9 (sibling sessions active — figures are deterministic; only wall_s moves).

## Results

| cell | fit (val) | 16@75 reflex / b0 / critic | critic vs b0 | critic vs reflex |
|---|---|---|---|---|
| b400 | agree 0.657 · R² 0.321 · λ 0.3 | 21.6 / **907.8** / 218.6 | Δ−675.2 lb95 −843.5 (2/2/16) | Δ+205.8 lb95 +84.2 (18/0/2) |
| b1600 | agree 0.661 · R² 0.368 · λ 0.3 | 21.6 / **907.8** / 377.4 | Δ−509.3 lb95 −714.7 (2/6/12) | Δ+371.7 lb95 +185.8 (18/0/2) |

18@75 tells the same story (critic 117.0 / 169.6 vs b0 532.5). Context seed 607 (16@75): b0 tops out at cap 1000 (405 lines); the critic tops out at 33 (b400) / 49 (b1600).

The b0 row **reproduces T6's baseline claim exactly at 20/0/0 vs reflex** (Δ+881.0 lb95 +752.1 on 16@75) — a cross-run consistency pin between the two benches.

## Verdict

**MODELLESS ARM DOES NOT CLEAR THE T8 BAR.** Better targets (b1600) lift the critic 218.6 → 377.4 (fit R² 0.32 → 0.37, agreement 0.657 → 0.661) but the gap to b0 stays ~2.4×. The negative is robust across teacher budgets: a closed-form linear readout in the Chebyshev basis captures the teacher's Q only to ~0.66 top-1 agreement, and the resulting 1-ply policy is far below the champion's hand-tuned evaluator — itself the product of Bench 891/892's rulebook search over this feature family.

**Consequences:**
1. **T7 GO** — the trained critic is justified; its bar is now concrete: strictly beat b0 (the champion 1-ply on the widened features) AND this arm (digests `ec03cb88c56de41d` / `48b92d26b04f3ecb`) on the same protocol.
2. **T7 notes from this run:** (a) target quality is the binding lever — mine targets at **b1600 minimum** (b400's Q noise visibly caps the fit; the T6 ladder shows b1600 is also where the teacher's own play reaches the cap); (b) val agreement plateaus ~0.66 across λ and budgets — the linear-in-basis shape, not regularisation, is the limit; a trained nonlinear critic is exactly the next rung; (c) the serving posture needs NO new capacity on the wire — the critic's per-decision p50 is **~6 µs** (G2 posture trivially holds; G4 alloc-free to assert at T9).
3. The modelless floor for ANY Instinct Tetris lane is b0's evaluator — free Reflex could legally ship the champion 1-ply (preview off) and hold 907.8/532.5; anything Instinct serves must clear that.

## Determinism

Model digest + every eval number byte-identical across worker counts (3 vs 10) — collection merges in seed order, the Gram accumulates in (regime, seed, decision, option) order, the solve is a fixed-order f64 Cholesky.

## Measured traps (paid for here)

- **`katgpt_core::karc::chunked_gram_into` is a PER-BATCH API** — it ZEROES `out_gram` on every call (the "chunking" is the inner loop's 4-unroll, not cross-call accumulation). Per-group calls silently keep only the LAST group (measured: g[bias][bias] = 4 instead of 400, with a perfect normal-equations residual masking it as a consistent solve). The module keeps a local streaming outer-product accumulator (upper triangle + mirror, same f64 order) and documents the divergence at the call site.
- Per-decision grouping must be (seed, decision) — regimes share seeds; a decision-only key silently merges two regimes' options into one group.
- λ must scale with n (per-sample multipliers): a fixed λ=1 at n≈1.3M is effectively no ridge.

Raw JSON: `tetris_critic_arm.json` (b400) · `b1600/tetris_critic_arm.json`.

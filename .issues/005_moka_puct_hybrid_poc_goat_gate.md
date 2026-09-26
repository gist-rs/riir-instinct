# Issue 005 — the Moka+PUCT-style hybrid POC: instinct (model) × reflex (modelless), three compositions + one GOAT gate

**Status:** OPEN — filed 2026-09-26 (Plan 001 P3 design). Blocked on riir-train Issue 576 (a specialist to plug in). The E0 measurement below can run NOW (reflex half only).

## Lineage (what the workspace already measured)

| precedent | prior | value | result |
|---|---|---|---|
| Moka PUCT (katgpt-rs Bench 205) | model policy head | model value head | **98.0%** vs Moka greedy at budget 200, c_puct 2.5, top_k 8 (p≈10⁻⁸). Bench 205:86: "NOT a modelless gain". |
| `engram_puct` (katgpt-rs Proposal 013, Bench 848) | model prior sharpened by a frozen count table, evidence-gated `sigmoid((n − n_min)/τ_n)` | model value, Q initialised from the table mean | G1/G4/G6 PASS, **G5 FAIL 48.1%** (Wilson low 44.8%). Cause: **rumor fraction 100%**: the count table rarely had evidence (gate ≥ 0.5 on 4 / 16,930). |
| `chance_puct` (katgpt-rs Bench 892) | modelless σ board score | modelless σ board score | G1–G4 PASS; wins only in the hardest regime, at 5–150× latency |
| riir-clippy Plan 049 (trained drafter + PUCT) | trained prior | V(passes-verify) | CLOSED-STOP: 0.94 pp < 2.0 pp bar. Its successor went back to modelless work. |
| `successor_density_critic` (katgpt-rs Issue 860, Bench 818) | — | modelless count critic `log p(s⁺=g\|s,a)/p(g)`, argmax 4.3 ns | GOAT PASS, opt-in. The one shipped modelless (s,a) value source. |

Moka's lesson is that the MODEL carries the gain. Proposal 013's lesson is that a
count-table fusion is only as good as its evidence density. Measure that FIRST.

## The arms (one harness run, same test split, read once)

- **A0 reflex alone:** riir-reflex modelless (`nb_scope`, the Bench 051 posture).
- **A1 instinct alone:** the specialist, no modelless term (the attribution
  control Bench 205:86 demands: the hybrid must beat THIS, not just reflex).
- **H1 cascade** (Proposal 047's default): reflex answers when the fused abstain
  gate passes; else the specialist decides among reflex's top-k.
- **H2 prior fusion** (the Proposal 013 shape, transplanted): specialist prior
  `p_i` sharpened by reflex evidence:
  `p'_i = normalize(p_i · exp(g · β · m_i))`, where `m_i` = the nb_scope
  per-token margin for option i and `g = σ((n − n_min)/τ_n)` with `n` = the
  number of the state's tokens the tables have seen. β, n_min and τ_n are
  selected on the held-out TRAIN slice, never on test.
- **H3 PUCT-over-options** (multi-question / sequential decision chains only,
  e.g. typed_decisions cases whose questions condition each other): prior =
  H2's `p'`; value = the specialist value head (Moka's shape), optionally
  initialised from `successor_density_critic` Q. Defaults copied, not tuned:
  c_puct 1.5, top_k 8. Single-shot classification needs no tree, so H3 is
  scored only where a chain exists.

## E0 — measure evidence density before building H2 (runs now)

Per suite, on the stratified selection slice: the distribution of `n` (seen
tokens per state) and the "rumor fraction" (states with n < 4). Proposal 013
died at 100%. If a suite's rumor fraction is > 50%, H2 is pre-declared NOT
armed there, recorded as a measurement rather than tried and refuted.
Expected (unmeasured): low. Bench 051's NB tables see 91–95% of test tokens,
unlike Go transpositions, but the number decides.

## GOAT gate (merged from Proposal 013 + Plan 001)

- **G0 identity:** specialist absent (or gate g ≡ 0) → output byte-identical
  to A0 (Proposal 013 G1).
- **G1 calibration:** calibrated readout ECE beats the conformal-naive floor
  (the riir-reflex G1).
- **G2 latency:** the fusion / cascade overhead < 100 ns per option (Proposal
  013 G2 bar), AND the absolute p99 published per suite. On escalated
  questions the instinct lane must be faster than laya at equal accuracy.
- **G3 no regression:** on the suites reflex already wins (emotion, sst5,
  massive, banking77 at Bench 051), hybrid accuracy ≥ A0 at a Wilson 95% lower
  bound.
- **G4 alloc-free:** the hybrid hot path makes 0 allocations after warmup.
- **G5 quality:** Wilson 95% lower bound of hybrid accuracy > max(A0, A1)
  point accuracy on at least one gap suite (xnli, typed_decisions, ag_news)
  with no G3 breach, **or** equal accuracy to A1 at ≤ 50% escalation (the
  Proposal 013 budget face, where the hybrid's value is the latency it saves).
- **G6 purity:** the reflex half is frozen counts (no runtime gradient); the
  instinct half is a sealed HOSTED-ONLY vessel (Issue 001).

Promote the winning arm, demote the rest. Report every arm, losses included.

## Plan

- [ ] **T1 — E0** evidence-density measurement (reflex-only, now).
- [ ] **T2** — arms A0/A1/H1 once riir-train Issue 576 ships a specialist.
- [ ] **T3** — H2 on the suites E0 clears; β / n_min / τ_n selected on held-out train.
- [ ] **T4** — H3 on chain-shaped cases only; `successor_density_critic` Q-init as a sub-arm.
- [ ] **T5** — GOAT gate G0–G6, record as a Bench; promote the winner as the
      "Reflex · instinct" lane (Issue 003).

## References

katgpt-rs Bench 205, Proposal 013 + Bench 848, Bench 892, Issue 860 / Bench
818, Proposal 014:45 ("PUCT-over-options for sequential decision chains — the
Bench-205 recipe"); riir-clippy Plan 049 + Proposal 009; riir-ai Proposal 047;
riir-reflex Bench 051.

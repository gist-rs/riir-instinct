# Bench 011 — the hybrid lane seats the harness families + code_fixtures (instinct Issue 010 T6; reflex Issue 049 T4)

**Date:** 2026-09-28 · **Box:** AC · powermode high · load1m 5.74 · quotable · **Verdict: the seat seam landed; all six synthetic suites run through the arena and publish `a0_stands` — exactly Issue 049's expected outcome. The eight dataset suites' A0 rows reproduced Bench 005 byte-identically (the seat refactor perturbed nothing).**

## What landed

1. **reflex `03415e5` (Issue 049 T1–T3):** `prepare_seat` accepts the five
   modelless harness families + `code_fixtures`, each seat carrying the
   explicit `synthetic` marker (the posture-fork defence);
   `harness_cache_reuse` stays refused with the decision named. Gates:
   `tests/harness_seat_gates` (5 tests — marker + byte-identity vs
   `synth_by_name` + seat-engine == manual-build bit-exact at the same
   posture + the refusal wording + the dataset posture unchanged).
2. **instinct (this record):** the arena population extends by the six
   synthetic suites (`SUITES` + the arity-8 dispatch row); they carry no
   winner artifacts, so they run the A0/G0-only posture and publish
   `a0_stands` rows. No other instinct-side change (049's design promise,
   kept).

## The seated rows (the single frozen test read, A0 per question)

| suite | questions | A0 accuracy | verdict |
|---|---|---|---|
| harness_visibility | 16 | 0.3750 | a0_stands |
| harness_permissions | 12 | 0.4167 | a0_stands |
| harness_tool_fit | 12 | 0.5000 | a0_stands |
| harness_routing | 16 | 0.4375 | a0_stands |
| harness_sensitivity | 15 | 0.4000 | a0_stands |
| code_fixtures | 32 | 0.3750 | a0_stands |

Authored fixtures with programmatic gold — per Issue 049's scope note,
"A0 stands" IS the valid, publishable answer; no specialist is implied.

**Disclosure (G1 disclosure rows FAIL on the families):** the families'
tiny cal fronts (12–20 cases) put the A0 ECE above the floor (e.g.
visibility raw 0.3723 vs floor 0.25). These rows REGISTER NOTHING — the
G1 gate binds promotable arms claiming calibration, and no family has
one; the disclosure is recorded, never hidden. Latency stays per-case
(p50 8–11 µs on the families — one question per case, seat-composing
arms absent).

## The regression pin (the important negative result)

All eight dataset suites re-ran through the refactored seat: A0 rows
**byte-identical to Bench 005/the published site rows** — ag_news 0.8825,
emotion 0.8850, sst5 0.3967, xnli_en 0.5233, massive_intent_en 0.7800,
banking77 0.8260, typed_decisions 0.4655, prompt_injections 0.7672. The
seat's synthetic branch added nothing to the dataset path.

## Artifacts

- `RESULTS.md` / `predictions.json` / `registration.json` — the full
  arena run (14 suites).
- reflex side: `03415e5` (Issue 049 T1–T3; the 049 close-out cites this
  record's instinct commit per the cross-repo law).

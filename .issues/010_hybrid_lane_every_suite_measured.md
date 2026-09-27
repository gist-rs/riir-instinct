# Issue 010 — the hybrid lane carries a MEASURED row for every suite; "not run" must mean never seated, never "A0 stands"

**Status:** OPEN — filed 2026-09-27 (owner direction: hybrid should produce a bench
result in BOTH cases — trained specialist or not). Root causes verified in code
same day; companion issues: riir-reflex (seat seam refuses the synthetic
families + code_fixtures), riir-train (specialists for the remaining dataset
suites).

## Why

The reflex-site hybrid card renders "Instinct (hybrid) — not run" for 10 suites
(typed_decisions, prompt_injections, xnli_en, code_fixtures,
harness_visibility/permissions/tool_fit/routing/sensitivity/cache_reuse).
Verified in code — three independent causes, none of them a queue:

1. **Population gate** — `src/bin/arena.rs` `SUITES` is a hand-typed const of
   exactly the six specialist suites. Every other suite never enters the run,
   so no hybrid row exists and the site card falls back to "not run".
2. **Fatal specialist join** — `run_suite_n` does
   `load_artifact(winners_dir.join(format!("{name}_winner_v1.bin")))?` and the
   caller `die()`s on the error. A suite WITHOUT a specialist artifact aborts
   the arena instead of publishing its honest A0 posture — even though the
   hybrid lane's G0 kill-switch (`HybridLane::ReflexOnly`) is byte-identical to
   A0 by design (pinned by unit tests). A no-specialist suite CAN carry a
   measured hybrid-lane row whose registered answer is A0; xnli_en already
   proved the shape in Bench 002 (measured A0 0.5233, registered = A0).
3. **Rendering conflation** — `hybrid_lane_doc.json` publishes only suites with
   a registered hybrid arm; measured-but-A0 suites ride
   `skipped_suites_a0_registered: ["xnli_en"]` and the site card renders them
   as "not run". That lies in the worse direction: xnli_en WAS measured
   (Bench 002, single frozen read) and the card says it never ran.

Honest vocabulary the lane must ship: three states, never two —
**hybrid arm** (registered non-A0 arm, gates pass) · **measured — A0 stands**
(seated, single frozen read done, no promotable hybrid arm) · **not run**
(never seated). Today the last two render identically.

## Tasks

- [ ] T1 — Extend the arena population to every reflex DATASET suite: add
      `typed_decisions` + `prompt_injections` (both seatable through
      `prepare_seat` today — they are dataset suites; only this repo's SUITES
      const and the missing specialist artifacts exclude them). Extend the
      label-arity dispatch in `run_suite` if the new suites' label counts fall
      outside {3,4,5,6,59,77}.
- [ ] T2 — No-specialist fallback: when `load_artifact` finds no winner
      artifact, run the A0/G0-only posture (skip A1/H1/H2 construction — do
      not die), publish the measured row with the verdict
      "A0 stands — no specialist", and keep the A0 identity pin green. The
      arena's die-on-missing-specialist becomes die-on-DATASET-missing (a
      suite the seat cannot build is still fatal; a missing specialist is not).
- [ ] T3 — `hybrid_lane_doc.json` grows a per-suite measured verdict: suites
      with a registered hybrid arm keep today's shape; A0-stands suites carry
      their measured A0 row + verdict (replacing the bare
      `skipped_suites_a0_registered` name list — a name list cannot carry the
      measurement); never-seated suites are simply absent.
- [ ] T4 — Reflex-site half (sibling repo, land + cite the SHA here): the
      hybrid card renders the three states above; "measured — A0 stands" is
      NEVER rendered as "not run" (per-card note names the product posture:
      A0 serves, the suite is not sold — Issue 008's gate, visible).
- [ ] T5 — Re-run the arena at the Bench-052 protocol → `predictions.json` +
      lane doc → `republish_bench.sh` → curl-verify the card on
      reflex.gist.rs shows measured rows for every dataset suite.
- [ ] T6 — Harness families + code_fixtures: blocked on the reflex seat seam
      (see companion issue) — when seats exist, they flow through T1/T2 with
      no further instinct-side change. `harness_cache_reuse` stays a recorded
      structural skip for the modelless hybrid (the modelless lane has no KV
      cache; a modelless answer there would be a fake task — reflex Issue 004
      T3), disclosed as such, never silent.

## Honest scope notes

- The A0-stands row does not make a suite "sold": Issue 008's product gate
  still refuses any arm that fails superiority/non-inferiority — the row
  reports the measurement, the serve posture table still decides.
- Families' hybrid rows are measurements of reflex-only posture on synthetic
  decision points; they cannot be specialist compositions until (a) the seat
  exists and (b) anyone argues a specialist is even trainable on authored
  fixtures. Measure first; decide after.
- Pre-registration discipline holds for the new suites: cal-front arms
  registered train-side, ONE frozen test read, same gates (G0/G2/G5 at
  minimum — G1/G3 where an arm exists to gate).

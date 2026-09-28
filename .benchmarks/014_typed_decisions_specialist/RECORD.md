# Bench 014 — the Issue 578 typed_decisions attempt: the artifact is UNSEATABLE (train-pool coverage gap)

**Status:** MEASURED-NEGATIVE (the consumer half refuses) — 2026-09-28.
Train-side lane: riir-train Issue 578 T1 / Bench 610
(`typed_decisions_armA_v1.bin`, BLAKE3 `6f644ccfaf7f7eaf…`); this record is
the consumer verdict: the arena CANNOT seat the artifact for this suite,
and the suite stays `a0_stands`. Records: `RESULTS.md` /
`predictions.json` / `registration.json` (the A0-only posture, Issue 010
T2's shape, with the reason in `a0_note`).

## What the read measured

The 578 T1 holdout read 0.5375 (workflow-level, train-pool rows) against
the published reflex row 0.4655 — a +7 pt train-side edge that did NOT
transfer, because the transfer never got the chance: the specialist cannot
answer 100 of the 500 test+cal cases at all.

**The gap**: typed_decisions' test templates present option keys the
artifact has NO class row for — 9 distinct keys, all in the
`security_incidents` family (5 severity levels
`"Negligible: no access to anything sensitive."` …
`"Critical: active compromise of crown-jewel systems."` + 4 action keys
`close_benign / contain / investigate / monitor`), 100 of 500 cases, ALL
four workflows' seats present (the 25% is one workflow's worth of
questions: 500 of 2000). The train side's distractor-universe law (Bench
610's own fix — extend the universe with train-PRESENTED keys) covers the
TRAIN pool's templates; the reflex harness's test templates are a SUPERSET
the train pool never carried. The seat's own corpus guard says the same
thing from the other side:
`[issue-039 corpus guard] typed_decisions: 1 option label(s) with NO train
docs in the corpus pool → self-doc fallback: security_incidents`.

**The refusal mechanics** (Plan 003's coverage gate, `d644ea5`): a
context-joined suite (the 4 workflow seat labels are the modelless
engine's domain space, disjoint from the 40-label option-key artifact —
`SeatJoin::Context`, sentinel perm rows) answers ONLY by name, so the gate
scans every presented key of every cal+test question BEFORE the join; a
miss takes Issue 010 T2's A0-only posture with the gap named — not a
mid-read panic, not a fished partial read.

## The A0 row (the suite's honest state, re-pinned)

| arm | acc | note |
|---|---|---|
| A0 | **0.4655** | == the published reflex row (A0 pin: arena == reflex `run()` == site ✓) — the OC-armed posture (selected_scale 2.0) |

G1 (A0 disclosure): raw 0.4238 · platt 0.0055 · floor 0.2072 → PASS.

## What unblocks the suite (train-side, filed)

The artifact needs retraining over a train pool that covers the reflex
TEMPLATE key set (or the distractor law extended from the templates, not
the pool): riir-train Issue 580. Until then typed_decisions has no serving
row (`typed_decisions` asserted ABSENT from the manifest in serve-gates,
both directions) and ships nowhere (`deploy.yaml`).

## The lane doc

`.benchmarks/014_typed_decisions_specialist/hybrid_lane_doc.json` — merged
(the 012 legacy rows + the 013 prompt row + this suite's a0_stands cell)
for the reflex-site session; the site's display law keeps the measured
refusal visible, never hidden.

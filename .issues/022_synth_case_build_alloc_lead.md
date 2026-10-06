# Issue 022 — the serve path's remaining 42 allocs/decision: the synth case build is 21 of them

**Status:** LEAD 1 LANDED (instinct `f8ee3a6`, 2026-10-06): decide max **42 → 36** (−14%), `_into` steady/cold **6 → 0**, the served bytes pinned identical; the intake measurement CORRECTED the attribution (see below). LEAD 2 LANDED (instinct `1ed58b7`, 2026-10-06): decide max **36 → 28** (−22%). Remaining leads recorded under Leads.

Filed from the reflex issue 070 lead 2 close-out (instinct `9ac84c8`,
2026-10-05): the bag serve path's allocation pin read **83 → 42** after
adopting reflex's `eval_case_into` scratch-refill face (the eval-path
machinery — engine scratch, wire request, result Vecs, `SeatEval` copy —
is now allocation-reused). The pin is deterministic (×3).

## The measured attribution — CORRECTED by the intake (2026-10-06)

The counting-allocator split the intake step asked for (now a standing
decomposition window in `tests/serve_g4_alloc.rs`) found the 21-of-42
figure was read off the FRESH `synth_served_case` form (the test's
decomposition window measured the fresh construction, a stand-in for the
in-decide cost). The decide path's REUSE form (`synth_served_case_into`)
measured **6 cold / 6 steady** — and all 6 were ONE class:
`rendered_options`' per-request `Vec<String>` (1 Vec + 5 label Strings on
sst5), computed and dropped inside the serve loop — noul's criteria is
Null (the rendered list was never consumed) and choice/score's rendered
list IS the wire slice, so the construction was pure waste there.

## Lead 1 LANDED (`f8ee3a6`)

The serve loop feeds the criteria keep-checks the wire slice (`q.options`)
directly; `rendered_options` left the serve path (it stays for validation
and the receipt's by-design owned once-per-question materialization).
Measured, deterministic ×2: `_into` steady **6 → 0**, cold **6 → 0**, fresh
21 → 15, decide max **42 → 36**; the G4 pin re-pinned 36 with the delta
named per its own protocol. Zero behavior change (the anti-goal holds):
the reuse form is pinned byte-identical (struct + serde bytes) to a fresh
construction across choice-unordered/score/noul shapes and the
steady/set-change/kind-flip/shrink-grow transitions
(`served_case_into_is_byte_identical_to_fresh_across_shapes_and_transitions`);
service_gates 24/24 incl. the data-gated parity face; lib 86/86.

## Leads (instinct-owned classes only)

1. ~~The criteria-map value construction~~ — **LANDED** (above); the case
   build is out of the hot path at steady state. ~~The single-question
   `decide` prelude's template clones (the `&mut self`
   eval borrow forces them — issue 021's note; a take/replace shape
   there is the next measured lead if this lane re-opens)~~ — **LANDED**
   as Lead 2 (below). The remaining per-decision surface (28) is: the
   engine's wire response (`DecisionResponse` +
   `Answer.probabilities` per question — katgpt-core's boundary, by
   design), the receipt's owned `probabilities`/`specialist_scores` clones,
   and the receipt's rendered-options `Vec<String>` (the contract).
2. The receipt's `qo_probs.to_vec()` is the contract (the receipt owns
   its bytes) — NOT a lead.

## Lead 2 LANDED (`1ed58b7`)

The single-question `decide`'s clone prelude left the hot path via the
take/replace window (mirror of the `eval_frame` idiom): the three
template fields (`labels` / `qid` / `q_instructions`) leave `self` for
the `decide_multi` call, the question slice borrows the TAKEN locals,
and all three are restored on ALL paths before the result propagates —
no `?` inside the window. While taken `self.labels` is empty, so
`decide_multi`'s three count reads go through a new `labels_len` boot
mirror (set in `from_parts_opt`, the single constructor tail; those
reads are count-only — never iteration — so the empty Vec is never
walked in the window). A caller-provided option set still pays its Vec
(`o.to_vec()` — the caller's bytes, not self's; not a lead).

Measured, deterministic ×3: decide max **36 → 28** (the −8: the
None-options prelude's 1 Vec + 5 label Strings + 2 Strings);
`_into` steady/cold still **0**, fresh 15 (unchanged); the G4 pin
re-pinned 28 with the delta named per its own protocol. Zero behavior
change (the anti-goal holds): `serve_gates` 24/24 with the dev winners
dir incl. the frozen-picks parity face `served_decisions_are_the_
frozen_goat_picks` with REAL boots (1024s wall); plain run (demo
winners default) 24/24 with the parity face skipping loud (the gate's
own bare-clone disclosure); lib 86/86; clippy `--all-targets -D
warnings` clean. One-off: the first full gates run under dev winners
failed `arsenal_release_refuses_the_eager_posture` (an HTTP
bind/timeout edge test, decide-untouched) — passes in isolation and on
the full rerun; parallel-load flake, not this change. Remaining
surface after it (28): the engine's wire response (`DecisionResponse` +
`Answer.probabilities` per question — katgpt-core's boundary, by
design), the receipt's owned `probabilities`/`specialist_scores`
clones, the receipt's rendered-options `Vec<String>` (the contract),
and the caller-provided-options Vec (caller bytes, not a lead).

**Anti-goal**: zero behavior change — the served case bytes are pinned by
`tests/serve_gates.rs`'s frozen-picks replays and `synth_served_case_into`
was pinned bit-identical to the plain path at its birth (issue 021).

# Issue 022 — the serve path's remaining 42 allocs/decision: the synth case build is 21 of them

**Status:** LEAD 1 LANDED (instinct `f8ee3a6`, 2026-10-06): decide max **42 → 36** (−14%), `_into` steady/cold **6 → 0**, the served bytes pinned identical; the intake measurement CORRECTED the attribution (see below). Remaining leads recorded under Leads.

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
   build is out of the hot path at steady state. The remaining per-decision
   surface (36) is: the engine's wire response (`DecisionResponse` +
   `Answer.probabilities` per question — katgpt-core's boundary, by
   design), the receipt's owned `probabilities`/`specialist_scores` clones,
   the receipt's rendered-options `Vec<String>` (the contract), and the
   single-question `decide` prelude's template clones (the `&mut self`
   eval borrow forces them — issue 021's note; a take/replace shape there
   is the next measured lead if this lane re-opens).
2. The receipt's `qo_probs.to_vec()` is the contract (the receipt owns
   its bytes) — NOT a lead.

**Anti-goal**: zero behavior change — the served case bytes are pinned by
`tests/serve_gates.rs`'s frozen-picks replays and `synth_served_case_into`
was pinned bit-identical to the plain path at its birth (issue 021).

# Issue 022 — the serve path's remaining 42 allocs/decision: the synth case build is 21 of them

**Status:** OPEN (lead intake — measured; no heal applied here)

Filed from the reflex issue 070 lead 2 close-out (instinct `9ac84c8`,
2026-10-05): the bag serve path's allocation pin read **83 → 42** after
adopting reflex's `eval_case_into` scratch-refill face (the eval-path
machinery — engine scratch, wire request, result Vecs, `SeatEval` copy —
is now allocation-reused). The pin is deterministic (×3).

## The measured attribution (sst5 cal front, 20 decides, max taken)

The remaining surface is dominated by **`synth_served_case_into`'s own
construction — 21 of 42**: the synthesized case still materializes the
per-question criteria JSON (`serde_json::json!` / `Map` inserts) and the
rendered option strings on every request. That was already the reuse form
(issue 021 — id/state/Vec buffers persist, the criteria map is kept when
the presented option set is unchanged); the residual is the map's VALUE
construction (fresh `Value::String`/`Array` nodes per request) and the
noul/choice criteria rebuilds when the presented set changes.

The rest: the engine's wire response (`DecisionResponse` +
`Answer.probabilities` per question — katgpt-core's boundary, by design),
the receipt's owned `probabilities`/`specialist_scores` clones, and the
per-question rendered-options `Vec<String>` (moved into the receipt —
by design).

## Leads (instinct-owned classes only)

1. **The criteria-map value construction**: when the presented option set
   is UNCHANGED (the steady-state request shape), the whole criteria
   `Value` could be kept like the map itself is — the same
   keep-when-equal shape one level down. Measured first, never guessed:
   a counting-allocator split of `synth_served_case_into` (cold vs
   steady-state) is the intake step.
2. The receipt's `qo_probs.to_vec()` is the contract (the receipt owns
   its bytes) — NOT a lead.

**Anti-goal**: zero behavior change — the served case bytes are pinned by
`tests/serve_gates.rs`'s frozen-picks replays and `synth_served_case_into`
was pinned bit-identical to the plain path at its birth (issue 021).

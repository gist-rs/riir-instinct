# Issue 025: `EscalateSpec` escalation-precision floor / damp field — one-definition home (OWNER-GATED)

**Status:** Open — OWNER-GATED decision record (widens a public grammar that rethink byte-pins); sequenced AFTER riir-rethink Issue 047 lands

**Provenance:** follow-up consumer-routing pass after katgpt-rs Issue 975 closed (close commit `878526076`; PoC G1–G4 PASS at `48fd0be4d`; consumers (a)–(d) filed per katgpt-rs `HISTORY.md` §Issue 975). Re-scoped under verdict review: the first draft framed this as a "thrash→damp grammar arm," which claimed a dynamics (churn×slope) the ESC gate does not have — the gate is EXACT and stationary (rethink Issues 046/047, verified at `escalating_backend.rs` L1026/L1037). Retracted. The surviving question is static and simpler, below.

## The question this issue holds

Rethink Issue 047 lands a **report-only escalation value-of-information readout** (precision = `P(think ≠ cheap ∧ think better | escalated)`, value per escalation, rate-band position). If that readout shows a suite's escalation precision persistently below a useful bar, the operator's only lever today is a full re-arm (manifest GOAT). The open question: should the ESC **contract** carry a static precision/value knob — an escalation-precision floor or a damp field in the `escalate` table — so the gate itself can sit closer to the value frontier between re-arms, with the readout as its evidence source?

If YES, the one-definition law fixes the home: `riir_instinct::arsenal::EscalateSpec` (the escalation grammar's single definition, per the riir-refine Plan 202 R1 carve) gains the vocabulary HERE, and rethink byte-pins it — the pin-lockstep law (grammar + pin move in ONE family, both repos, one landing).

## Constraints (non-negotiable if armed — unchanged from the first draft, verdict-reviewed as sound)

1. **A5 — the manifest stays the only ARMING surface.** A precision floor / damp field must ride the `escalate` table's shape, never a new env switch. `RIIR_RETHINK_ESC=0` remains the only runtime demote; the rate guard's sticky latch is untouched.
2. **A2 — never a blend.** A damp field means escalate-GATING (suppress/raise the bar for escalation), never score blending across lanes.
3. **Byte-identity when absent:** the grammar addition compiles to the exact pre-arm surface when the new row/field is absent — every existing manifest, pin, and gate byte-identical (the `grpo_chunk_weight=0` contract shape).
4. **Rethink-side gates green at both postures:** `tests/esc_gates.rs` + the escalating-backend suite at armed and demoted postures.

## Sequencing (hard order)

1. riir-rethink Issue 047 lands and separates seeded low-value from high-value postures (its T3). If 047 closes NEGATIVE (cannot separate), this issue closes with it — no evidence source, no knob.
2. Owner GO on THIS issue — a grammar rethink byte-pins is a contract act, not a library edit. Declining is a first-class close path; this file is the decision record either way.
3. One family landing: instinct grammar + type rows, rethink pin + reader, both repos' pin-lockstep tests updated in the same arc.

## Non-goals (unchanged)

- No runtime dampening in rethink's `EscRateGuard` — sticky latch, demote-only (Issue 047 documents why the runtime actuator class cannot occur in-process).
- No arena/serve selection changes — the serve path stays the frozen-GOAT-picks replay (the parity gate pins it); ESC grammar only.
- No change to the escalate-table arming/cross-check laws (digest pin, `RATE_WINDOW` cross-check untouched).

## Gate if armed

Grammar compiles to byte-identity when the field is absent; a grammar self-test asserts the new vocabulary round-trips through the wire shape; rethink's byte-pin test reds on drift (one family); `scripts/ci_feature_guard.sh` green both repos.

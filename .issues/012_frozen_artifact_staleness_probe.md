# Issue 012: Frozen-artifact staleness probe + pre-swap impact report (from arXiv:2609.30652)

**Status:** Open — POC proposal, no GPU/training dependency; pure serving-side forward-pass work.

## Source

[arXiv:2609.30652](https://arxiv.org/abs/2609.30652) "Recursive Self-Improvement via On-Policy Distillation for Reasoning" (Meta AI, 2026) — distillation record: `../riir-train/.research/460_DCE_SRCL_Co_Evolving_Privileged_Teacher.md`.

The paper's fixed-trace probe (their Eq. 8) scores a **pinned set of stored incorrect trajectories** with both the live model and its frozen reference, measuring behavioral divergence: p(EOS | wrong-answer endpoint) and p(reflection-cue | revision prefix). The finding: a frozen reference diverges from the live runtime as the runtime evolves (teacher p(EOS) 90.4%→41.3% across training), and acting on a stale reference costs ~25pp in their setting.

## The modelless residue (no training anywhere)

Every frozen-artifact surface in this repo has the same structural exposure the paper measured:

- arsenal specialists + HOSTED-ONLY vessels (re-minted at bench cadence, served between mints)
- the manifest + PINNED_MANIFEST_DIGEST posture (A5/A6 laws)
- hybrid cal fronts + per-boot threshold derivations (the recorded "frozen-derived-posture seam" follow-up)

Today the only parity instrument is **hard pick parity** (byte-identity on frozen picks, serve_gates) — a 0/1 gate that fires *after* picks flip. The paper's probe is the missing **soft early-warning readout**: it can fire *before* any pick flips.

## Proposed POC (three parts)

1. **Staleness probe:** pin a BLAKE3-digested probe set of known-outcome questions (labels spent once at record time; probe time is gold-free). For a (live runtime, reference artifact) pair, compute per-item divergence on the lane's own calibrated readout (fused sigmoid score, noul propensity, corpus-distance reading — the lane's decline axis stands in for the paper's EOS axis). Gate: bit-identical pairs read **exactly zero** by determinism (free canary); deliberately paired old-artifact/new-runtime fixtures must fire.
2. **Pre-swap impact report:** before a monotonic hot-swap commits (A2), run old-vs-new artifact over the same probe set and emit a per-item Δ changelog on the calibrated readout. Report-only v1 — the T2/LB95 gates still own the promote/demote verdict; this is the early-warning column beside them. Report per-item margin-distance alongside raw divergence so the alarm only trips where a flip is plausible (raw divergence over-weights decision-irrelevant items).
3. **Overlay-refresh policy arm (the paper's schedule ladder, modelless form):** A2 forbids EMA-of-artifacts, but DCE's teacher is *same weights, different conditioning context* — the stack analog is the derived overlay (cal fronts, threshold ladders). Measure: after induced drift, trigger-based closed-form overlay refit (against the pinned cal slice only, never live traffic) vs stale-overlay baseline on the frozen test read; idempotent on fresh pairs (G3 by construction). The paper prices the cliff: ~25pp for zero refresh, ~0pp extra for modest vs continuous refresh — so expect trigger-based ≈ continuous; the arm verifies that at our scale.

## Honest limits (recorded up front)

- Divergence is a symptom, not an adjudication — it names the drifted pair, never which side is wrong.
- The TSD precedent (katgpt-rs Bench 802) killed a similar micro-scale residue DEAD-BY-DOMINATION: our artifacts are re-minted frequently under versioning discipline, so the probe's value may be as a cheap canary + pre-swap report rather than a refresh trigger. Measure before building the policy arm.
- Gold-free at probe time only because labels were spent at record time; the probe set is a maintained fixture, not a live signal.

## Acceptance

- POC writes a verdict table (probe set × {live, reference} × divergence metric) with box-state provenance.
- Zero-divergence canary + induced-staleness firing demonstrated.
- If DEAD-BY-DOMINATION at our cadence: file the negative, close, and the issue's value is the measured defense of the current re-mint discipline.

## Refs

- riir-train Research 460 (distillation record) + Plan 427 (training half — this issue is the serving half)
- katgpt-rs `.research/561` TSD (modelless-residue-at-micro-scale precedent, Bench 802)
- `tests/serve_gates.rs` pick-parity gates (the hard instrument this extends)
- katgpt-rs `.research/122` EDGE-OPD (the diagnostic-tools-kept-when-method-rejected precedent)

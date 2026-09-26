# Plan 001 — the Reflex · instinct lane, riir-clippy flow end to end

**Status:** IN PROGRESS — P1 done (repo born + registered 2026-09-26); P2–P6 open.

Design of record: `../riir-ai/.proposals/047_riir_instinct_hybrid_decision_engine.md`.
Trainer: `../riir-train/.issues/576_arena_specialists_for_riir_instinct.md`.
Shape: `../riir-clippy` (private product engine over public katgpt-rs;
weights are bytes; mining → settle → corpus → retrain).

## The flow

```
            ┌─────────────── riir-instinct (this repo) ───────────────┐
 request ─▶ │ modelless lane (riir-reflex nb_scope) ── confident? ──▶ answer (µs)
            │        │ abstain / top-k prune                             │
            │        ▼                                                   │
            │ specialist forward (riir-infer loaders, HOSTED-ONLY vessel)│
            └────────┬────────────────────────────────────────────────┬┘
                     │ decision outcome (consent-gated, Unset never pushes)
                     ▼                                                  │
     decstat row ─▶ riir-kat wire ─▶ riir-dapps epoch settle + KAT      │
                     │                                                  │
                     ▼                                                  │
     riir-train corpus ─▶ retrain at threshold ─▶ vessel MINT (signed,  │
     HOSTED-ONLY, lineage) ─▶ riir-deployer (cf-container) ─────────────┘
```

## Phases

- [x] **P1 — birth.** Private `gist-rs/riir-instinct`, `develop`, contract
      docs (BOUNDARY `Visibility: private`), numbered dirs; registered as a
      contract repo in katgpt-rs (`repo_set.txt` + AGENTS count + the pin
      files).
- [ ] **P2 — specialists (riir-train Issue 576).** Arm A supervised CE + Arm
      B laya distillation on the arena train splits; held-out train
      selection; sealed frozen artifacts. Blocks P3.
- [ ] **P3a — Moka+PUCT-style hybrid POC (Issue 005):** arms A0 reflex / A1 instinct / H1 cascade / H2 prior fusion (the Proposal 013 shape) / H3 PUCT-over-options (chains only); E0 evidence-density measurement first (runs now, reflex-only); the winner feeds P3.
- [ ] **P3 — hybrid serving + arena lane (Issue 003).** First code: crate
      skeleton, specialist forward via riir-infer, modelless top-k prune +
      escalation on the fused abstain gate; harness lane "Reflex · instinct"
      (accuracy, escalation rate, p50/p99); GOAT gate vs modelless AND laya
      on the same test split, read once.
- [ ] **P4 — vessels (Issue 001).** HOSTED-ONLY reader over the
      `reflexer-vessel` format (verify → decrypt → monotonic apply); minting
      + lineage stay in riir-train; first minting key pinned in the same
      change that ships the first artifact.
- [ ] **P5 — hosted deploy (Issue 002).** `deploy.yaml` for riir-deployer:
      cf-container target (Proposal 014 §hosted plane (b)), stage → verify →
      rolling flip, explicit rollback; secrets via `wrangler secret put` only.
- [ ] **P6 — the flywheel (Issue 004).** decstat contribution rows → riir-kat
      → riir-dapps settle (devnet first) → riir-train corpus → retrain at the
      measured threshold → new vessel → P5 redeploy. The riir-clippy mining
      loop, carried for decisions.

## GOAT gate (every promotion) — the merged Proposal 013 + Plan 001 gate

Full definitions: Issue 005. G0 identity (no specialist → byte-identical to
reflex) · G1 calibration vs the conformal-naive floor · G2 fusion overhead
< 100 ns/option + absolute p99 published, faster than laya at equal accuracy
on escalated questions · G3 no regression on reflex-won suites (Wilson 95%
lower bound ≥ reflex) · G4 alloc-free hot path · G5 Wilson lower bound >
max(reflex, instinct-alone) on a gap suite, or instinct-alone accuracy at ≤ 50%
escalation · G6 purity (frozen counts + sealed vessel, no runtime gradient).
Train rows only; test read once; every arm reported, losses included.

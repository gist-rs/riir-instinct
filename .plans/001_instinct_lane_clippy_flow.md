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

## GOAT gate (every promotion)

G1 calibration vs the conformal-naive floor · G2 p99 within the lane's
budget (the instinct lane must beat laya's latency where it matches its
accuracy) · G3 no regression on suites where the modelless lane already wins
(emotion, sst5, massive, banking77 at Bench 051) · G4 hot path alloc-free
after warmup. Test split read once; train rows only.

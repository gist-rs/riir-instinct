# Plan 001 — the Reflex · instinct lane, riir-clippy flow end to end

**Status:** IN PROGRESS — P1 done (repo born + registered 2026-09-26); P3a's
E0 measured 2026-09-26 (reflex Bench 053: ARMED-PENDING on all 8 dataset
suites — no suite excluded, T3 unblocked by E0); P2's riir-train Issue 576
T1+T2 done 2026-09-26 (Bench 608: Arm A specialist trains all six suites,
artifacts exported) — P2's remainder is 576 T3/T4 (Arm B distillation +
winner pick); P3–P6 open.

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
- [ ] **P2 — specialists (riir-train Issue 576).** Arm A supervised CE
      MEASURED (Bench 608: all six suites, every suite beats its lr=0
      control, artifacts exported); Arm B laya distillation (576 T3) + the
      winner pick (576 T4) open; sealed frozen artifacts. Blocks P3.
- [ ] **P3a — Moka+PUCT-style hybrid POC (Issue 005):** arms A0 reflex / A1 instinct / H1 cascade / H2 prior fusion (the Proposal 013 shape) / H3 PUCT-over-options (chains only); E0 evidence-density measurement MEASURED 2026-09-26 (riir-reflex Bench 053 — ARMED-PENDING on all 8 dataset suites, no suite excluded); the winner feeds P3 (per-suite arms pre-registered on train/cal; one frozen test read — P3a's run IS the read, P3 never re-reads it).
- [ ] **P3 — hybrid serving + arena lane (Issue 003).** First code: crate
      skeleton, specialist forward via riir-infer, modelless top-k prune +
      escalation on the fused abstain gate; harness lane "Reflex · instinct"
      (accuracy, escalation rate, p50/p99); GOAT gate vs modelless AND laya —
      the laya row joined onto P3a's single frozen test read, never a second
      pass.
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

Full definitions: Issue 005. G0 identity (kill switch → byte-identical to
reflex; g ≡ 0 → byte-identical to instinct-alone) · G1 calibration vs the
conformal-naive floor, same metric both sides · G2 fusion overhead < 100
ns/option (per question for H1) + absolute p99 published, faster than laya
(laya − lane 95% UB ≤ δ, paired) on escalated questions · G3 paired non-inferiority on
reflex-won suites (A0 − hybrid 95% upper bound ≤ δ; δ = max(1.0 pp, 2.5·SE)
per suite from the train/cal discordant rate, ≈80% power at parity) · G4
alloc-free hot path · G5 per-suite candidate arm pre-registered on
train/cal (Pareto rank-0 + argmax Beta-LCB, the Proposal 042 seam shape);
its Wilson lower bound > max(reflex, instinct-alone) on a gap
suite, or A1 − arm 95% upper bound ≤ δ (paired, same power rule) at ≤ 50%
escalation · G6 purity (frozen counts + sealed vessel, no runtime gradient).
Train rows only; the
test split read once and its predictions frozen (P3's laya row joins that
run); every arm reported, losses included.

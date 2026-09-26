# Plan 001 — the Reflex · instinct lane, riir-clippy flow end to end

**Status:** IN PROGRESS — P1 ✓; P2 ✓ (576 closed, winners exported);
P3a-E0 ✓ (reflex Bench 053); **P3a T2–T5 ✓ 2026-09-27 (Bench 001: the
hybrid GOAT run — ag_news promotes H2, emotion/sst5 promote A1,
xnli/massive/banking77 keep A0; H3 excluded with reason; the single
test read spent, predictions frozen)**; **P3 T1–T4 ✓ 2026-09-27**
(Issue 003: hybrid composition + the arena lane + the G0–G6 gate faces;
the laya paired face and the reflex-site publish remain open); P4–P6
open; Issue 006 (massive specialist anomaly) open.

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
- [x] **P2 — specialists (riir-train Issue 576).** CLOSED 2026-09-26:
      Arm A supervised CE measured (Bench 608: all six suites, every suite
      beats its lr=0 control); Arm B laya distillation + winner pick
      measured (Bench 609: B wins ag_news .875 / massive .670 /
      xnli .400; A holds emotion/sst5/banking77); winners exported as
      `<suite>_winner_v1.bin` BLAKE3-sealed artifacts
      (`../riir-train/data/instinct_specialists/`, gitignored — P3 mounts
      them via its artifact loader; the format is arm-agnostic). P3
      unblocked.
- [x] **P3a — Moka+PUCT-style hybrid POC (Issue 005):** arms A0 reflex / A1 instinct / H1 cascade / H2 prior fusion (the Proposal 013 shape); H3 PUCT-over-options EXCLUDED with reason (no chain-shaped specialist suite — re-opens with a typed_decisions specialist). E0 MEASURED 2026-09-26 (riir-reflex Bench 053 — ARMED-PENDING on all 8 dataset suites). **T2–T5 MEASURED 2026-09-27 — Bench 001:** the instrument (Pareto rank-0 + argmax Beta-LCB on the cal front) registered per-suite arms; the single frozen test read landed (predictions.json); the GOAT verdict: ag_news → H2, emotion/sst5 → A1, xnli/massive/banking77 → A0. P3's laya row rides `--features arena-laya` (open).
- [x] **P3 — hybrid serving + arena lane (Issue 003).** T1 ✓ 2026-09-26
      (crate skeleton + the RISP artifact reader + the live winner-load
      pin); **T2 ✓ 2026-09-27** (src/hybrid.rs: H1 cascade + H2 prior
      fusion + G0 kill-switch arm + the name-injection join pin);
      **T3 ✓ 2026-09-27** (src/bin/arena.rs over reflex's one-way
      `harness::runner::seat` seam — the boundary check forced this
      shape: a reflex→instinct dep would be a cycle; the A0 drift pin
      holds the seat path to reflex's own run()); **T4 ✓ 2026-09-27**
      (the G0–G6 faces measured in Bench 001). Open: the laya paired
      face, the reflex-site lane-scoped publish, Issue 006 (massive).
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

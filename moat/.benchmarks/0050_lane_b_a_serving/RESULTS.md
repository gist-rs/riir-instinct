# Bench 0050 — Issue 018 Lane B + Lane A: the shared encode worker + the lazy/evict inheritance (the serving-plumbing GOAT record)

**Status:** MEASURED 2026-10-02 — Lane B + Lane A LANDED; `serve-encoder-shared` ships OPT-IN
(promotion to the `serve-encoder` default rides the soak), lazy ENC rows land as a legal
posture with `eager` still the embedded default.

## What landed

- **Lane B** (`src/encoder_serve.rs`): the unit of residency moved from the LANE to the
  WORKER — one worker thread + one agent per registry key. Under `serve-encoder-shared` the
  key is the CHECKPOINT (`ckpt:english` — every ENC lane on it shares one worker); the
  default keys lane+head (`lane:<suite>` — byte-identical per-lane behavior, same code
  path). The registry stores WEAK handles: lanes own the worker's residency, the LAST
  release frees the RAM, and a re-activation pays exactly ONE fresh load (G3 by
  construction — creation holds the registry lock through the ready wait). Per-lane
  in-flight gate (`LANE_INFLIGHT_CAP = 64`) preserves the old bounded-channel back-pressure
  against the shared queue (the fairness bound). Template validation moved to lane ATTACH
  (the worker is template-agnostic; a drift fails the lane, never the shared worker); the
  worker carries the checkpoint digest (streamed BLAKE3 of the resolved weights artifact).
- **Lane A** (`arsenal.rs` + `serve.rs` + the substrate): `budget.load = "lazy"` is now
  LEGAL on ENC rows (L2's lazy-once-then-resident contract inherited; `eager` stays the
  embedded default). L9's residue = G3 (one load per activation) + the sticky-`Failed`
  bound (the slot state machine — a failed load never retries on its own; stronger than
  backoff). The lazy-row BOOT PREFLIGHT refuses config errors at boot: weights presence +
  pin verification (new substrate fn `riir-infer-laya::laya::weights::verify_checkpoint_present`
  — verify-only, never downloads), the ACTIVE-variant sidecar resolution, the memory
  budget counted per SHARED worker (`INSTINCT_ENCODER_MEM_BUDGET_MB`; unset = disclosed,
  not gated — device capacity has no portable query, the operator declares the ceiling),
  and the template-vs-head class count WITHOUT a forward pass. healthz gained `readiness`
  (`ready-cold` for lazy-Unloaded); /decide responses disclose `lane_load`.
- **Fallback (recorded, not built):** share-weights-only (mmap/Arc of the checkpoint) if
  mixed-load p99 ever regresses beyond the envelope. No regression observed (below), so
  the fallback stays dormant.

## Gates

| Gate | Instrument | Result |
|---|---|---|
| Parity (interleaved vs isolated, bit-identical) | `tests/encoder_shared_parity.rs` (env `INSTINCT_ENCODER_SHARED_PARITY=1`), 2 lanes × 24 cases lockstep vs isolated, CPU posture | PASS both modes |
| Cross-build parity (per-lane == shared fingerprints) | two builds, `--nocapture` PARITY lines diffed | **EXACT**: sst5 `c35a50b77602c063`, xnli_en `7c237555b7ab47fd` in BOTH builds |
| RAM (3 lanes, 1 checkpoint) | the serve binary, 3 eager ENC rows (sst5 / xnli_en / ag_news), CPU, private bytes after all-3-ready | **per-lane 4938 MB → shared 1690 MB = 2.92×** |
| Exactly-once load / release-frees / recall-one-load / fairness cap / attach drift / failed-boot retry | `encoder_serve::worker_tests` (weight-free, fake boot/encode) | 6/6 PASS |
| Lazy ENC validates; unknown load word refuses | `serve_gates::enc_posture_grammar_and_validator_rules` (flipped arm) | PASS |
| Full suites | lib at serve-encoder 80/0 · at shared 80/0 · default 67/0 · serve_gates 18/0 · clippy `-D` at default / serve-encoder / serve-encoder-shared all-targets | all green |

## Concurrency / mixed-load (honest scope)

The lockstep interleaved replay IS a mixed-load run (two lanes alternating every case
through the shared worker) and it is bit-identical to isolated. The wall clocks of the two
parity runs — 1216 s (per-lane) vs 1194 s (shared), each including 6 lane boots — are the
recorded same-instrument comparison; no shared-worker head-of-line regression is visible
at this shape. The full mixed-load p50/p99 A/B under concurrent HTTP load is DEFERRED to
the serving soak (the M3 GPU host), with the fairness bound (the cap gate) and the
share-weights-only fallback as the recorded hedges.

## Box state

4090 Windows workstation (shikuwa, i7-13700K), AC power; LAYA_DEVICE=cpu, F16 weights
(848 MB); the CPU lane widened f32 maps dominate the numbers. A sibling agent session ran
concurrently on the box (katgpt-rs work) — the wall-clock rows carry that load class; the
bit-identity and RSS rows are load-insensitive. Weights: `~/.cache/riir-reflex/laya`
(english, F16, no derived variant — `variant f16` disclosed in the boot log).

## Files

- `src/encoder_serve.rs` — worker registry (weak-handle residency), LaneGate, attach
  validation, preflight fns (`preflight_enc_weights` / `preflight_enc_template`), the
  worker_tests module.
- `src/arsenal.rs` — the ENC lazy acceptance (validator).
- `src/bin/serve.rs` — the boot preflight pass (feature-gated with the lane), healthz
  `readiness`, /decide `lane_load`.
- `../riir-infer/crates/riir-infer-laya/src/laya/weights.rs` — `verify_checkpoint_present`
  (the verify-only half of `ensure_checkpoint`; no download, refuses missing).
- `tests/encoder_shared_parity.rs` + the `[[test]]` row; `Cargo.toml` feature
  `serve-encoder-shared`.

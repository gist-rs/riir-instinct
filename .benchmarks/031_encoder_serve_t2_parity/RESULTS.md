# Bench 031 — the serve-side encoder lane's parity gate (issue 016 T2) + the T1 budget derivation

**Status:** RECORD — T2 gate GREEN (2026-09-30, 4090 box, CPU posture); T1 derived at the stand-in posture, re-measures on the D1 GPU host.

## TL;DR

The encoder class now SERVES — behind `posture ENC`, feature `serve-encoder`,
GPU hosts only (issue 016 T2, the staged plan's first rung). The parity gate
reproduces the frozen Bench-029 read **316/600 exactly** through the serve
surface (`AnySuiteServer::boot_bytes` on an ENC manifest row), with the
arena-runner diff leg at **32/32 pick agreement** in-process. The end-to-end
serve-binary smoke answers `200` with `arm: "ENC"` and the per-class sigmoid
scores crossing the wire (the sync boundary: the embedding never does).

## The gate (tests/serve_encoder_parity.rs)

Two legs, both mandatory:

1. **The arena leg** — the frozen arena runner (`eval_encoder_arm`, the code
   that PRODUCED Bench-029) over the same seat, same process, same weights:
   the serve lane's picks must equal its picks exactly (32/32). A serve
   render divergence reds HERE as a lane bug, never as a posture question.
2. **The frozen-count leg** — the full 600-case replay through the serve
   lane; the correct count must equal `accuracy × n` from the Bench-029
   record (the record is the ONE expected-count source; no hand-typed 316).

Plus determinism (the same 32 requests re-answered → identical picks; the
compute-once-and-record law) and the L3 tier bound (p99 < 1 s — the issue's
layer-fit slot ceiling, NOT the modelless µs tier the bag gates assert).

Two bugs the first red run caught (the gate working as designed — both were
lane bugs, caught before they could ship):

- **The canonical options** came from `seat.labels` — the ENGINE label
  universe (`"0".."4"` for sst5), not the presented space the head scored
  (`"very negative".."very positive"`). Fix: the template criteria's own
  order is the canonical presented space (`template_options`).
- **The state form**: the wire state is the suite's serialized-state form;
  the arena encoded the envelope VALUE. Fix: the lane parses the wire
  string back into the Value and encodes THAT, so `serialize_state`
  re-derives the arena's exact prompt bytes on the worker. A wire string
  that does not parse refuses loud (outside the C1 convention), never
  embeds prose the head never scored.

The substrate probe that pinned both (deleted after): token streams
arena-vs-lane 61/61 identical, defs identical, first divergence None.

## The measured record

```
ENC PARITY: suite sst5 · device cpu · correct 316/600 (frozen 316) ·
            p50 228168 µs · p99 437653 µs (tail support 6/600) ·
            head blake3:893db1acf6208e71
ENC DIFF LEG: serve-lane vs arena-runner agreement 32/32
BOX: os windows · arch x86_64 · LAYA_DEVICE unset (lane default → cpu)
```

- **Head**: `E:/git/riir-train/.raw/t599/t6_s0.bin`, BLAKE3 sidecar
  verified, manifest digest drift gate run for real (the winners dir IS the
  head's parent during the gate).
- **Device**: cpu (this build has `serve-encoder` = the CPU laya tree; the
  Metal/CUDA postures compile via `serve-encoder-metal`/`-cuda`). The
  frozen read ran Metal on the M3; the argmax contract is what transfers —
  316/600 exact on a third box, fourth posture (trainer → M3-Metal dump →
  M3-Metal arena → this x86_64 CPU serve).
- **Latency**: the CPU posture is the STAND-IN measurement (~228 ms p50 —
  the C1 M3-CPU class, 157 ms; this box is slower, disclosed). The L3 slot
  bound (1 s) holds even here; the serving GPU host re-measures at D1.
- **Run cost**: ~208 s wall (600 encodes + agent load + two seat preps).

## The e2e serve smoke (the real T2 surface)

`target/release/serve` built `--features serve-encoder`, booted with an ENC
manifest (explicit `file`, eager budget, the head's BLAKE3), `--suites sst5`:

- healthz: `"sst5":{"state":"ready","load":"eager","epoch":0,"arm":"ENC",
  "source":"raw_winner","labels":5,"artifact_labels":5,...,
  "winner_blake3":"893db1acf6208e71"}`
- `POST /decide` on test case 0 → `200`, `"arm":"ENC"`,
  `"pick":"very negative"` (correct — *"no movement , no yuks , not much of
  anything ."*), the 5-wide sigmoid score vector + confidence crossing,
  receipt attached; features stamp `["arena-laya","default","serve-encoder"]`.

## T1 — the budget derivation (the G2 unit + the admission cap)

**The demand model** (verdict-round-2 law): EVENT-ARRIVAL × ADMISSION —
utterance/task arrival rate × the salience-admitted fraction; never the NPC
population. **The G2 unit is GPU-seconds per second**:

```
gpu_s_per_s = admitted_arrival_per_s × per_row_s(host)
cap(host, budget_share) = budget_share / per_row_s(host)   [encodes/s]
```

| Host posture | per-row (measured) | 10/s admitted | 20/s admitted | L3 slot share at 20/s |
|---|---|---|---|---|
| M3-Metal (Bench-029, cited) | 14.7 ms p50 | 0.147 GPU-s/s | 0.294 GPU-s/s | 15–59 % of 0.5–1 s/s |
| This box CPU (031, stand-in) | 228 ms p50 | 2.28 CPU-s/s | 4.56 CPU-s/s | the measurement posture, not a serving posture |
| GPU-posture (8–15 ms, CITED not measured on our boxes) | — | 0.08–0.15 | 0.16–0.30 | 16–60 % |

**The cap is a REQUIRED control** (issue 016's own arithmetic: 50 admitted/s
at the loaded-M3 36 ms figure exceeds a whole GPU). The serving host derives
its cap from ITS measured per-row cost: `cap = budget_share / per_row_s` —
e.g. a Metal host reserving 50 % of the L3 slot at ~15 ms/row caps at
~33 admitted encodes/s; window-batching amortizes the per-row figure above
that. The M5 Ultra is UNMEASURED — the M3 figure is the stand-in premise;
**T1 re-measures on the host's arrival** (the gate prints p50/p99 with the
device label every run, so the re-measure is one gate run).

**G2's assertion for the T5 GOAT gate**: at the host's measured per-row cost
and the configured admission cap, `gpu_s_per_s ≤ the cognition budget's L3
share` — asserted, not cited. This bench records the formula + the stand-in
cells; the serving host's cell lands with D1.

## What this bench does NOT claim

- NOT the D1 trigger: no real GPU serving deploy exists yet (the text lane
  is CPU-only CF; the game prod host has not arrived). The sst5 board cell
  keeps `serve: ✗` until D1 fires (issue 016 T6).
- NOT a promotion: the DEFAULT manifest is unchanged (sst5 still serves A1
  byte-identically; the embedded-manifest byte pin is untouched). ENC rows
  exist only on a GPU-host deployment manifest — and refuse loud at boot on
  a build without `serve-encoder` (the 014 serve refusal governs every
  CPU-only deploy shape, enforced by the default-build gate arm).
- NOT the vessel lane: T4's HOSTED-ONLY head mint rides later; `boot_vessel`
  refuses ENC rows loud for now (the raw sealed head is the T2 posture).

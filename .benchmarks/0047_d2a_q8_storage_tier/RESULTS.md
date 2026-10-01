# Bench 0047 — issue 018 Lane D2a: the Q8 storage tier — ADOPTED, numerics byte-identical to the probe

**Status:** MEASURED + ADOPTED — the D2 branch's storage tier (the device-
resident kernel tier, D2b, remains follow-up work). The fake-quant probe
(Bench 0046) priced the weight-quantization error; this tier lands the
REAL artifact: a Q8_0 storage format, a converter that refuses to emit
anything not byte-identical to the probe's in-memory transform, and the
loader seam that serves it.

## What landed

- **riir-infer `10e33de`**: the reader widens `dtype: "Q8_0"` (34 bytes per
  32 weights + short tail block, f16 scale) through the fake-quant path's
  OWN decode arithmetic; `q8_artifact.rs` (converter + variant resolver);
  the `laya-quant8` bin; `LAYA_WEIGHTS_VARIANT=f16|q8` (the q8 variant is
  sidecar-verified at load, NEVER auto-derived inside a request — a
  missing artifact names the converter command). Double-quantization
  refusals: `--fake-quant` × q8 artifact; ANE × either;
  `WeightPosture::Q8Artifact` is env-selected only (the explicit form
  refuses loud — never a silent F16-under-a-quantized-label).
- **Converters run** (both proofs GREEN on the real bytes):
  - english: 206 tensors (126 quantized / 80 skipped+named 1D),
    421,205,504 elements, **842,587,660 B F16 → 447,707,500 B Q8
    (53.1%)**, blake3 `8f787527…f3be`, 2.16 s;
  - typed: identical geometry, blake3 `ced49dcd…4d901`, 2.20 s.
  - (The 53.1% vs the naive 50%: the 80 skipped 1D norms/biases stay F16
    by the house law — ~24 MB. The issue's "~424 MB" estimate assumed
    whole-file quantization; the honest posture number is 447.7 MB.)

## The adoption re-seat (the issue's law, satisfied by proof + spot-check)

The issue: "Adoption re-seats every cell (frozen reads invalidated by any
numerics change — by design, never grandfathered)." **No numerics changed**:

1. **By construction**: the artifact decode is the fake-quant path's own
   arithmetic (`d · q`, same `q8_scale_bits`/`q8_quant_of` primitives, one
   home in `fake_quant.rs`).
2. **By the converter's proof**: each artifact's read-back was compared
   element-by-element (bit patterns) against `fake_quant_q8_map` of the
   same weights AT CONVERSION TIME, on the real 421M elements — the
   converter refuses to emit otherwise.
3. **By the end-to-end spot-check**: the arena at `LAYA_WEIGHTS_VARIANT=q8`
   (no `--fake-quant`), sst5 over the artifact — **0.5333 (320/600),
   picks byte-identical to Bench 0046's fake-quant run**; the record
   carries `weight_posture: "q8-artifact"` and a zero-error report (the
   artifact IS the quantized form; max_err 0.0 is the honest encoding of
   "nothing was transformed at load").

Therefore Bench 0046's seven-suite gate table IS the adopted numerics'
re-seat — the artifact path reproduces it (spot-check) and the other six
suites' reads are the same bytes by (1)+(2). The per-device determinism
gate (the issue's) concerns D2b's kernels; this tier has no new kernels —
the forwards are byte-identical code at every device, already gated by G5.

## What this tier buys

- Cache/download/storage footprint: 842.6 → 447.7 MB per checkpoint
  (english + typed done; multilingual deliberately NOT converted — DEAD
  BY LAW per riir-train 603).
- Load-time read volume halves (the transient widen peak is unchanged —
  the in-memory map is still F32; the DEVICE-buffer tier is D2b).
- Serve posture: `LAYA_WEIGHTS_VARIANT=q8` (fail-loud, sidecar-verified,
  tamper-evident via the BLAKE3 sidecar).

## Follow-ups (recorded, not started)

- **D2b** — device-resident Q8 buffers + dequant-fused kernels (the real
  memory tier: the M3's unified-memory device buffers are still F32
  here). Per-device determinism re-seats then apply.
- **Q4/PQ2 second** (the issue's) — the same lane, narrower blocks; the
  probe methodology (fake-quant first, artifact second) repeats.
- **Lane C** (the typed head retrain) — per the branch, C trains on the
  ADOPTED encoding; the adoption is now the q8 artifact.

Reproduction: `laya-quant8 <ckpt-dir>` re-derives byte-identical
artifacts (deterministic); the sst5 spot-check command is in
`0046/run_all.sh`'s shape + `LAYA_WEIGHTS_VARIANT=q8`.

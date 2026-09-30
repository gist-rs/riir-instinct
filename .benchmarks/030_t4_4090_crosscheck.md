# Bench 030 — Plan 426 T4/V4 4090 cross-check: the M3's V4 FAIL confirmed by independent execution (teacher dump regenerated, student retrained, arena re-read)

**Status:** MEASURED 2026-09-30 — the cross-check the owner directed ("submit your work as 4090 as a cross check is fine"). Verdict: **the M3's T4 verdict REPRODUCES EXACTLY — V4 FAIL confirmed; the seat is unchanged.** Land: `a625624a`-lineage student verdict re-derived from a fresh dump with DIFFERENT teacher numerics; every frozen pick identical.

## What ran (the 4090 lane, Windows, CPU-fp32 teacher)

1. **Teacher dump regenerated from scratch** (T1 seam, `--distill-teacher openthai`, t20k, CPU fp32 via their `openthai_systemone` service staged at the pinned sha `5d04bcca`, uv venv torch 2.14.0+cpu / transformers 5.17.0, `CUDA_VISIBLE_DEVICES=''`, permutations pinned by the lane): **11514/11514 rows, teacher acc 0.9548** — the M3's dump read 0.9548 from the MPS-fp32 path; the acc agrees to 4 decimals across numerics backends. Dump blake3 `9d098cdf1f31727d` (≠ the M3's `49b16a89…`: header ts + ulps-level prob deltas from the CPU-vs-MPS kernel difference; the trainer's gold-pin join re-verified all 11514 rows against the local pool). Wall ≈ 16 h (box at 90-100% load from a sibling GPU lane the whole run; p50 4.2 s/forward).
2. **The 609 retrain** (`instinct_arm_b`, massive, t20k, mixes 0.0/0.5, holdout 200): **A 0.6600 · B@0 0.6450 · B@0.5 0.6550 → WINNER A (gold)** — the M3's holdout table reproduced exactly (all three arms, same winner). Teacher-side train acc 0.9548 printed by the trainer. Artifact `massive_intent_en_winner_v1.bin` (7,865,749 B, blake3 `1f95e309e5991250…`), exported to the ISOLATED dir `riir-train/data/instinct_specialists_openthai_4090/` — the served winners dir is never touched (and on this box it never had the massive winner; issue 015 records the cross-box digest drift the copy surfaced).
3. **The arena V4 read** (`--winners-dir` at the isolated dir, `--skip-pin-a0`, one frozen test read): **A0 0.7800 (anchor exact) · A1 0.8033 · H1 0.7833 · best arm H2(β=2,nmin=2,τ=2) 0.8067 (T2-certified vs A0, LB95 +0.0007)** — byte-equal to the M3's Bench-023 read at every arm.

## The paired legs (instinct stats law, n=300 frozen questions)

| 4090 arm | vs served H2(β=1,τ=8) 0.8267 | vs M3 best H2(β=2,τ=2) 0.8067 |
|---|---|---|
| A1 0.8033 | mean −0.0233 · lb95 −0.0546 | mean −0.0033 · lb95 −0.0347 |
| H1 0.7833 | mean −0.0433 · lb95 −0.0743 | mean −0.0233 · lb95 −0.0502 |
| **H2(β=2,τ=2) 0.8067** | **mean −0.0200 · lb95 −0.0492** | **mean +0.0000 · lb95 = ub95 = +0.0000** |

The H2-vs-H2 leg is the determinism witness: mean exactly 0.0000 over 300 paired questions — **every pick identical** across two differently-hashed artifacts, two hosts, two teacher-numerics paths. The digest delta is non-semantic (issue 015's hypothesis, now measured at pick granularity).

## Verdict

**V4 FAIL confirmed independently**: best arm 0.8067 < the served 0.8267 (paired mean −0.0200, lb95 −0.0492 — the whole CI below zero). The STRONG teacher's labels still lose to the laya-era artifact through this linear head (the plan's 613-class reading stands: teacher acc is not student acc). The M3's T4 closure (plan 426, commit `f70c6f65`) is cross-checked; the seat is unchanged; the DO-NOT-RERUN coordination stands with an execution witness on the second box.

## What this bench is NOT

Not a seat challenge (V4 refused the student — no seating, no manifest change), not a new arm registration on the served roster (the winner lives in the isolated crosscheck dir), and not the T5 answer (the corpus A/B re-run at the repaired seat-posture read is a separate lane; its veto re-run is in flight on this box at record time).

Records: predictions/registration/RESULTS in this dir; trainer console arms quoted above; dump artifacts gitignored under `riir-train/.raw/t4_teacher/` (digests in-file). Cross-references: plan 426 T4 row (`f70c6f65`, the M3 closure), instinct bench 023 (the M3 read), issue 015 (the digest-drift note this bench resolves at pick granularity), reflex `8f425d1` (the corpus-ab seat-posture repair landed while this lane ran).

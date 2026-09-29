# Bench 026 — the round-5 teacher A/B (issue 009 pre-registration): plain b1600 vs the r4'-critic z-blend — NO-GO

**Status:** MEASURED 2026-09-30 (M3 session, picking the lane up per its pre-registration after the 4090 session moved to riir-train t599). Verdict per the pre-registered GO rule: **NO-GO — the lane stops.**

## The lane (pre-registered in issue 009, commits `37dffa1` + `26fe411` + `eebb64b`; component qualified in Bench 025)

Plain b1600 teacher vs the z-blended teacher (the r4' critic's LOGIT
z-interpolated into the champion eval at the decision-state eval seam, w=0.5,
z-fit over the root's kept top-8, priors pure champion — the pre-registered
form), seeds 401..=420, both arms budget 1600, no preview + fresh bag.
The blend component is the Bench-025 attempt-2 r4' rebuild, digest
`5c7eb6ca4ae3bb3e` (verified by BLAKE3 on this box after copy from the 4090;
`best val_agreement 0.76791 ≥ floor 0.7647`, `warm_epochs 80` present in its
metrics — the amendment's config).

**Provenance note (concurrent-session discipline, the Issue-825 class):** this
bench is the 4090 session's pre-registered gate, executed on the M3 because
(a) the 4090 box was mid-flight on the t599 encoder lane and (b) the A/B is a
CPU play-strength eval — seed-deterministic, box-independent in its verdict.
The component bytes are digest-identical to theirs. The sibling's Bench 027
pilot (the residual form, same seam) is the adjacent-evidence prior: it
measured that form NULL-TO-HARMFUL and predicted nothing better here.

## The cells (the pre-registered three; GO = lb95>0 on ≥1 cell AND no cell ub95<0)

| cell | plain | blended | Δmean (b−p) | lb95 | ub95 | W/T/L | verdict |
|---|---|---|---|---|---|---|---|
| garbage 16@75, cap 5000 | 4535.1 pieces (17 cap) | 4931.1 (19 cap) | **+396.0** | −119.7 | +911.7 | 3/17/0 | positive-lean, NOT significant |
| garbage 18@75, cap 1000 | 654.95 (13 cap) | 605.9 (12 cap) | **−49.05** | −145.8 | +47.7 | 1/16/3 | negative-lean, NOT significant |
| garbage 20@80, cap 1000 | 0.0 (0 cap) | 0.0 (0 cap) | 0.0 | 0.0 | 0.0 | 0/20/0 | DEGENERATE (below) |

**GO RULE: lb95>0 on 0 cells, ub95<0 on 0 cells ⇒ NO-GO.** No-op guards PASS
(blend-path value() calls 177,104,397 · probe picks 110,740 · pick diffs
15,370 — the blend executed and moved picks; the A/B measured a real blend).

## The reading

1. **A wash with mixed signs — the same null Bench 027 predicted.** Cell 1
   leans positive (3 wins, 0 losses, blended survives to cap on 19/20 vs
   17/20), cell 2 leans negative (3 losses, 1 win), neither bound excludes
   zero, and the two lean in OPPOSITE directions. The z-form's three deltas
   over the residual form (logit stretch, z-interpolation, top-8 fit) bought
   at most a sign flip in the surviving cell — not a separation. The r4-class
   critic does not add ranking information the champion evaluator lacks at
   these budgets, in either blend form.
2. **The points column agrees with the NO-GO harder than pieces do.** The
   blended teacher scores FEWER POINTS in both live cells — 219,615 vs
   274,957 at 16@75 (despite +396 pieces) and 23,429 vs 36,542 at 18@75.
   The blend shifts play style from tetris-scoring toward survival; on the
   arena's score metric (T3 judges score/lines/pieces) the blend is worse in
   BOTH cells. A teacher that survives longer while scoring less is not a
   better distillation target for this game.
3. **Cell 3 (20@80) was unplayable — a pre-registration design miss, recorded
   as such.** Both arms died at 0 pieces on all 20 seeds (no legal placement
   for the first piece; `wall_s 0.0002`). The "A/B-only hard cell" was chosen
   without a playability probe and carries zero information; it did not hit
   the both-all-cap censoring rule (the games died, they did not cap), so it
   counted as a separating cell that separated nothing. It changed no
   verdict — even a strongly positive cell 3 could not have met the lb95>0
   rule that cell 1's +396 missed — but any future pre-registration must
   single-probe a new regime's playability before allocating a cell to it.
4. **What this closes:** the z-blend at **w=0.5** (the pre-reg's own scope —
   "a NO-GO closes w=0.5, not the lever"). What it does NOT close: other w
   values (Bench 027 measured the residual form harmful at w=1.0 and null at
   w=0.5 — the two forms together bracket the useful range as thin), and the
   prior-seam blend (never priced in — ~27× the critic calls at the prior
   seam). The honest summary for a future reader: BOTH round-5 forms are now
   measured (z: this bench; residual: Bench 027) and neither produced a
   promotable teacher; re-opening the teacher-blend lever needs a mechanism
   the two benches did not test, not a re-run at a nearby w.
5. **The student never ran.** No blended dataset was built, no r5 student
   trained, no eval seeds spent (still 5 reads — this bench used only the
   disjoint 401..=420 block). The remaining priced lever per the issue:
   the loss/data-shape lane, including the offline label-blend test on the
   existing b1600 dataset (cheap — no teacher runs; low prior, recorded).

## Box state + latency disclosure

Play-strength results are seed-deterministic (load affects wall only).
Launched on the M3 (16-core, AC power, loadavg 6.95/7.86/8.39 at launch with
one sibling `cargo-heal` on katgpt-rs running throughout; the record's
`/proc/loadavg` read is `null` — macOS). Total wall 4234.8 s (~70.6 min),
10 threads. Per-decision p50: plain 7.7–7.8 ms, blended 341–342 ms — these
are WALL figures under load and include the probe (~5 ms/spot per the R2
note); they are disclosed for cost-shape only, never as a latency claim.

## Reproduce

```sh
cargo bench --bench tetris_round5_teacher_ab --features tetris_goat -- \
    --model ../riir-train/data/tetris_critic_r4p2/tetris_mlp_v1.bin \
    --w 0.5 --seeds 401:420 --threads 10 \
    --out .benchmarks/026_tetris_round5_teacher_ab
```

Model: `riir-train/data/tetris_critic_r4p2/tetris_mlp_v1.bin` (BLAKE3
`5c7eb6ca4ae3bb3e…`, the Bench-025 qualified component). Raw record:
`teacher_ab.json` in this directory.

## Addendum — the independent 4090 execution (same gate, second box; verdict CONFIRMED)

The 4090 session ran the same pre-registered gate independently and
concurrently (the Issue-825 duplicate-execution class — adjudicated here as a
cross-box confirmation rather than a duplicate, because the two executions
AGREE on the verdict and DISAGREE on cell 1 in an informative way). Raw
record: `teacher_ab_4090.json` (4090 Windows / i7-13700K, two complete runs
that were byte-identical to each other on every game statistic — same-box
determinism pinned; only wall_s moved, 3040.7 vs 3404.8 s on cell 1 under
different box load).

| cell | 4090 plain | 4090 blended | Δmean | lb95 | ub95 | W/T/L |
|---|---|---|---|---|---|---|
| garbage 16@75, cap 5000 | 4296.7 (16 cap) | 4881.6 (18 cap) | +584.9 | −53.6 | +1223.4 | 4/15/1 |
| garbage 18@75, cap 1000 | 655.0 (13 cap) | 605.9 (12 cap) | −49.0 | −145.8 | +47.7 | 1/16/3 |
| garbage 20@80, cap 1000 | 0.0 | 0.0 | 0.0 | +0.0 | +0.0 | 0/20/0 |

GO rule on the 4090 numbers: identical — **0 GO cells, 0 harm cells ⇒ NO-GO**,
guards pass (175.5M blend value() calls, 15,388/109,750 pick diffs).

**The cross-box pattern is itself the finding — stated at its measured precision:** cell 2's (18@75) VERDICT INPUTS reproduced exactly across macOS ARM and Windows x86 — pieces (654.95/605.90), paired bounds (−49.05, −145.81, +47.71), W/T/L (1/16/3), every digit. Its LINES and POINTS did not (M3 plain 267.4 lines / 36,542 points vs 4090 267.7 / 36,733; blended 23,429 vs 24,154) — **the games themselves diverged cross-arch and reached the same piece counts**; why the counts coincided at cap is unattributed (the earlier "short games stay inside tie-breaks" reading was wrong — the tie-breaks were crossed). Cell 1 (16@75) diverged outright (4535.1 vs 4296.7 plain) — long games compound per-decision ulp divergence past visible statistics. Both cells' VERDICT inputs agree (cell 1 lb95 < 0 on both boxes; cell 2's pieces/bounds identical), so the NO-GO is box-independent — measured, not assumed. The same cross-arch caveat the Bench-025 rebuild recorded for TRAINING data applies to play-strength evals at every horizon: per-arch deterministic, cross-arch divergent in the games' interiors; never compare per-cell numbers across boxes, only the pre-registered verdict inputs and same-box re-runs.

**The points column confirms on this box too, strengthening the NO-GO:** the blended teacher scores FEWER POINTS in both live cells on the 4090 as on the M3 — 209,148 vs 246,104 at 16@75 (despite +584.9 pieces) and 24,154 vs 36,733 at 18@75. The survival-biased style shift is cross-box, not an M3 artifact.

Provenance note for the 4090 runs: the first execution launched via an agent
session that was watchdog-cancelled mid-flight (the detached child survived
and completed the full gate — `tasklist /FI "IMAGENAME eq …"` exact-matches
the bare name and MISSES cargo's hash-suffixed bench binary
`tetris_round5_teacher_ab-<hash>.exe`; filter with a `LIKE` prefix or
`Get-CimInstance` instead). A second complete run then reproduced the first
byte-identically; `teacher_ab_4090.json` is the second run's output.
Launcher provenance: `scripts/r5_launch_ab.ps1` (committed beside this
record).

Session: 4090-r5b, 2026-09-30T03:55+07:00

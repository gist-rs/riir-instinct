# Bench 027 — the round-5 pilot (M3 session): the residual value-seam blended teacher's cheap gate (Issue 009 T7 round 5)

**Status: MEASURED 2026-09-29/30 — the full residual-form lane is NO-GO on
its own cheap-gate logic; the tested blend grid is null-to-harmful and the
plain teacher's budget ladder is measured-flat in play strength.** Revised
after a two-round reviewer pass: the first draft's n=20 means carried no
detection limits (its "b400 +33" was ~1 SE of noise — REFUTED at n=100);
this record states every claim with paired bounds, per-seed CSVs, and
capped fractions beside it.

## CONCURRENT-LANE NOTE (read first — the Issue-825 collision, recorded per
its own discipline)

A sibling session (the 4090 box) landed the round-5 z-blend lane on origin
DURING this session (`37dffa1` pre-reg → `26fe411` impl → `8fd33cd` Bench
025 = their r4' rebuild QUALIFIED). Their form differs from this module's in
exactly three ways — LOGIT vs sigmoid score (theirs stretches the
catastrophic tail this blend's σ compresses), z-INTERPOLATION vs residual
add, top-8 kept fit set vs all-options frame — and their pre-registered
A/B (allocated as Bench 026, seeds 401..=420) carries a cap-5000 16@75 cell
this pilot never measured (our 16@75 cells saturated at cap 600). **This
bench's null is ADJACENT EVIDENCE for that A/B, not a falsification of it;
the A/B proceeds per its pre-registration.** What this bench DOES settle
independently: the residual form (`tetris_blend::BlendedState`, the sigmoid
score) is null-to-harmful at every tested corner, and the teacher budget
ladder is flat in play strength within the cap windows. The two export
flags (`--blend` theirs, `--critic` ours) coexist in `export_tetris_critic`
with the loser retiring when the A/B settles either way.

## Scope of the claim (what was and was not tested)

Tested: the **RESIDUAL VALUE-SEAM blend** — `chance_puct` over (champion
priors + champion leaf values ⊕ `w·gain·(critic_sigmoid − m̄)`), w ∈ {0.5,
1.0}, 18@75 only (16@75 saturates every arm at these caps), the r4 critic
(`7e319da89b7f5a7c`, 512×512, the pre-registration's named critic — the M3
copy; the sibling lane's r4' rebuild is `5c7eb6ca4ae3bb3e`). NOT tested:
the PRIOR-seam blend (excluded by cost — ~27× the critic calls; priced at
days of compute for a full dataset regen), the sibling z-blend form (see
the concurrent-lane note above), other critics, other regimes. A later
reader must not take this bench as falsifying "critic-guided search"
broadly — it falsifies the residual value-seam form at the tested grid.
What the full round-5 lane would have consumed (~3 h: regen ~2 h + retrain
~1 h + playoff) trains a student on a teacher this bench measures as
unchanged-at-best — the lane is refused on that prediction, not run.

## The instrument (landed this session)

- `src/tetris_blend.rs` — `BlendedState`: leaf VALUE evaluations blend
  `champ(afterstate) + w·gain·(m − m̄)`, m = the critic's score of the
  placement producing the afterstate (the exact `RawOpt` construction the
  critic was trained on), `(m̄, gain = sd_champ/sd_critic)` = a per-decision
  frame over the root's options; priors stay pure champion; the root's
  `value_ref` stays champion-only.
- **w=0 is the plain teacher, BIT-EXACT** — pinned by
  `weight_zero_reproduces_the_plain_teacher_bit_exactly` (games + recorded Q
  bits, 12 seed×regime cells); the flat-critic degenerate arm pinned by the
  smoke test. A cross-run determinism pin held too (t400 re-run in probe B ≡
  probe A, per-seed identical).
- `export_tetris_critic --critic <mlp> [--blend-weight w]` — the blended
  collection lane (manifest carries the full blend provenance).
- `examples/tetris_round5_scale_probe.rs` — the per-seed scale probe (all
  arms on one seed set, paired bounds via the house
  `stats::paired_upper_bound_f64`, capped fractions, per-seed CSVs).

## The measurements (18@75 — the discriminating regime; seeds ≥ 401,
NON-eval NON-train; teacher rng `seed ^ 0x05EE_D892`)

**Cap 600, n=100 (seeds 401..=500):**

| arm | mean | capped | paired vs baseline |
|---|---|---|---|
| b0 (1-ply champion) | 348.7 | 56% | — |
| r4 critic 1-ply | 250.6 | 39% | **−98.1** vs b0 (SE 37.4, lb95 −159.7, ub95 −36.6) |
| plain teacher b400 | 459.4 | 76% | +110.6 vs b0 (lb95 +64.3) |
| plain teacher b1600 | 447.2 | 74% | −12.1 vs b400 (lb95 −45.0, ub95 +20.8) — null |
| plain teacher b6400 | 459.0 | 76% | +11.7 vs b1600 (lb95 −15.1, ub95 +38.6) — null |
| blended b400 w=0.5 | 386.4 | 63% | **−73.0** vs b400 (SE 28.8, lb95 −120.4, ub95 −25.5) — HURTS |

**Cap 1000 (the T3 protocol's cap), n=60 (seeds 401..=460):**

| arm | mean | capped | paired vs plain b1600 |
|---|---|---|---|
| plain teacher b1600 | 720.6 | 72% | — |
| blended b1600 w=0.5 | 720.4 | 72% | **−0.2** (SE 52.1, lb95 −85.9, ub95 +85.5) — exact null; sd(diff) 403 |
| blended b1600 w=1.0 | 534.7 | 52% | **−185.9** (SE 59.3, lb95 −283.5, ub95 −88.3) — HURTS badly |

Caveats, stated: both caps censor heavily at this regime (52-76% of games
reach the cap) — compression hides effects LARGER than the cap in principle,
and the detection limits above are the honest floor of what n=60/n=100 buys
on a distribution with per-game sd ≈ 300-460. Six paired comparisons were
run; the b400 harm (z ≈ 2.5) is borderline under a Bonferroni correction for
six — read it as "harmful, lb95 −120", not decisive on its own (the NO-GO
rests on the absence of any positive lower bound across the grid, not on
this one cell). The first pilot's n=20 tables (`pilot_*` manifests beside
this record; val seeds 321..=322 there — the first draft of this record
mis-stated 309..=312) are superseded by the tables above; they remain as the
provenance of the cheap gate's first read. The `pilot_blend_r4_w025_*`
manifest is that first pilot's w=0.25 COLLECTION run (dataset-export shape,
618 s wall) — a cost datapoint only, no strength read beyond the superseded
n=20 mean; the bounded strength tables carry w ∈ {0.5, 1.0} only.

## The reading

1. **The blend is null-to-harmful across the tested grid** — in weight,
   within the cap-1000 setup (b1600: w0.5 −0.2 exact null → w1.0 −185.9);
   and harmful at the shallower budget under the cap-600 setup (w0.5:
   b400 −73.0, harmful, lb95 −120). No tested corner has a positive lower
   bound; the best result is the exact null. (The b400-vs-b1600 pair is a
   CROSS-SETUP comparison — different caps censor differently — so it is
   labeled as such, not as a within-setup gradient.) The shape is what the
   ensemble view predicts: the r4 critic is a weaker ranker than the
   champion leaf (−98.1 vs b0 at 1-ply, ub95 −36.6), so blending it into
   leaf values dilutes at best.
2. **The bootstrapping counter-consideration, honestly noted:** the critic
   distilled the teacher's ROOT Q and is applied at LEAVES, where the plain
   search evaluates only the 1-ply champion — that IS new information at the
   leaf in principle (the AlphaZero mechanism). The measurement says the
   r4-critic's version of it does not overcome its own ranker error at any
   tested corner. A stronger critic (higher val agreement than 0.77) could
   in principle flip the sign — that is a DIFFERENT critic than the
   pre-registration named, and the imitation gap (rounds 1-4) is exactly what
   has so far prevented one from existing.
3. **The budget-ladder successor: no higher rung in teacher PLAY STRENGTH
   within the cap windows — label accuracy vs budget was NOT measured.**
   t6400 − t1600 = +11.7 (lb95 −15.1, ub95 +38.6) and t1600 − t400 = −12.1:
   mean game length is flat in budget at cap 600 (74-76% capped; the ladder
   was not measured at cap 1000 — that table carries a single plain budget,
   b1600, 72% capped, so only the censoring level is comparable there). The
   censoring-robust half of the claim — the EARLY-DEATH rate, which capping
   cannot compress — is also flat: capped-fraction discordance t400 vs t1600
   = 7 seeds capped only under t400 vs 5 only under t1600; t1600 vs t6400 =
   3 vs 5. What stays open in principle: more search budget could still
   make the root-Q LABELS more accurate even with play saturated inside the
   cap — that axis was not measured and is not claimed.
4. **The "better targets" theory does not need round 5.** Blending the
   critic's output into the training labels is testable OFFLINE on the
   EXISTING b1600 dataset (q_teacher + λ·critic-residual, retrain, no regen)
   — it belongs to the issue's "different loss/data shape" lane, where it is
   now recorded as a candidate. Label self-distillation of this shape
   smooths toward the critic the student already cannot beat, so the prior
   is low; it is cheap because no teacher runs at all.
5. **b0 stands.** The T3 bar (beat b0 AND the ridge, both regimes) remains
   unmet by every arm ever trained; this bench adds no student and spends no
   eval-seed read (seeds 1..=20 + 607 remain read only five times:
   009/010/017/018/021).

## What this session landed

- `src/tetris_blend.rs` + lib.rs registration (feature `tetris`) — the
  blended teacher, its two degenerate-arm unit tests, the collection lane.
- `export_tetris_critic --critic/--blend-weight` — manifest blend provenance.
- `examples/tetris_round5_scale_probe.rs` — the one-setup per-seed scale
  probe (reusable for any future teacher-arm question).
- This record + 7 first-pilot manifests + the scale-probe CSVs
  (`seeds_*.csv` beside this record, copied from the probe runs).

Compute spent on the gate: ~65 min of probe runs total (the b1600 blends are
~28/22 min each at n=60 cap 1000) — against ~3 h + GPU for the refused lane.

Box state: M3 Max, loadavg ~4-7 through the probes (sibling sessions active);
the probes are deterministic-per-seed — the piece counts and paired bounds
are the claim, wall columns are pricing only.

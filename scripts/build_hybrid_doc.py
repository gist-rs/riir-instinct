#!/usr/bin/env python3
"""Package the arena's frozen test read into the reflex-site publish doc.

Reads a `.benchmarks/NNN_*/predictions.json` written by the arena
(`cargo run --release --bin arena`) and emits the results-style JSON doc
`publish_bench.py` consumes (the hybrid lane rides the same merge law as
every other lane — riir-instinct .issues/003).

Per suite the lane cell is the SERVING arm's measured read — resolved
from the arsenal manifest (law A5, the ONE selection surface; the
owner's best-measured-arm serving law, 2026-09-27), never the record's
T2-era `registered` field. EVERY seated suite carries a cell (the owner
display law: hiding a measured result reads as "can't handle it" — a
tie or a loss is shown, labeled, and stays visible as the improvement
backlog, Issue 008). The cell carries:
- `serves`  the arm the hosted service answers with (from the manifest)
- `gate`    the T2 certification state of that arm (certified / uncertified
            best-measured / served-by-the-reflex-half)
where the serving arm is A0, the cell IS the measured A0 read (the
product's answer on that suite — the reflex half is part of the
Instinct binary, Plan 001 P5).

Statistics replicate the reflex harness metric laws EXACTLY (the same
functions the arena imports, re-derived here over the frozen picks):
- accuracy           = mean(correct)
- ece                = ece_of's law — 15 bins, edges i/15, left-open
                       right-closed, conf == 0.0 in no bin
- acc_at_50_coverage = stable argsort by conf desc (first index wins
                       ties), k = floor(n*0.5).max(1), hits/k
- mean_confidence    = mean(conf)
- latency_p50/p99_ms = the arena's nearest-rank percentile over
                       total_durs_us / 1000. The SCOPE is per-arm and is
                       published as `latency_scope` (Issue 007):
                       "seat+arm" arms (A0/H1 — contains_seat_solve) time
                       reflex solve + decision, the composed end-to-end;
                       "arm-only" arms (A1/H2) time their own forward
                       alone — they do NOT run reflex, so their latency is
                       NOT end-to-end and must never be read as such.
- consult_rate       = mean(escalated)

Fields the arena does not produce are OMITTED (never zero-filled): the
site renders them as "—" (macro F1 needs a fixed class space the
variable-option suites do not offer; Brier needs the full probability
vector; the abstain block is a Reflex-lane concept; the repeat-check
`determinism_ok` is simply not claimed).

Usage:
    python3 scripts/build_hybrid_doc.py <predictions.json> <out.json>
        [--git-sha <sha>] [--date-utc <iso8601>]
    python3 scripts/build_hybrid_doc.py --self-test
"""

from __future__ import annotations

import argparse
import json
import math
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# Console-safe streams (the console_encoding discipline): non-ASCII arm
# names (β, τ) must survive a cp874-class console.
for _s in (sys.stdout, sys.stderr):
    _s.reconfigure(encoding="utf-8", errors="backslashreplace")

# The lane's machine id — publish_bench.py's LANE_DISPLAY renames it to
# the public "Instinct (hybrid)" at the publish boundary.
LANE_ID = "hybrid"
# The issue-014 C1 encoder arm's lane key (RECORD-ONLY — serve: ✗; the
# encoder class is refused at serve). A SEPARATE lane from the hybrid
# cell: the hybrid cell keeps publishing the SERVING arm (A1 — no serve
# change), the encoder cell publishes the measured-but-refused read.
ENCODER_LANE_ID = "encoder"


def bin_of(conf: float) -> int | None:
    """Index of the `(edges[i], edges[i+1]]` bin, or None — a conf
    exactly ON an edge belongs to the bin it closes; 0.0 in no bin."""
    if conf <= 0.0 or conf > 1.0:
        return None
    for i in range(15):
        lo = i / 15.0
        hi = (i + 1) / 15.0
        if conf > lo and conf <= hi:
            return i
    return None


def ece_of(pairs: list[tuple[float, bool]]) -> float:
    """The reflex harness `ece_of` law: 15 bins over (0,1]."""
    if not pairs:
        return float("nan")
    n = len(pairs)
    cnt = [0] * 15
    conf_sum = [0.0] * 15
    corr_cnt = [0] * 15
    for conf, correct in pairs:
        b = bin_of(conf)
        if b is not None:
            cnt[b] += 1
            conf_sum[b] += conf
            if correct:
                corr_cnt[b] += 1
    ece = 0.0
    for b in range(15):
        if cnt[b] > 0:
            weight = cnt[b] / n
            mean_conf = conf_sum[b] / cnt[b]
            mean_acc = corr_cnt[b] / cnt[b]
            ece += weight * abs(mean_conf - mean_acc)
    return ece


def pct_nearest_rank(sorted_vals: list[float], pct: float) -> float:
    """The arena's `pct`: nearest-rank percentile over a sorted list."""
    n = len(sorted_vals)
    if n == 0:
        return float("nan")
    idx = max(0, math.ceil(pct / 100.0 * n) - 1)
    return sorted_vals[min(idx, n - 1)]


def macro_f1_of(golds: list[int], preds: list[int]) -> float:
    """The reflex harness `hard_metrics` macro-F1 law, verbatim: classes =
    sorted(set(gold) | set(pred)); F1_c = 2tp / max(1, 2tp+fp+fn); the mean
    over classes. (Plan 011 C2: the Rethink cells read on the same axes as
    the lanes — the law is MIRRORED, never re-derived, and the self-test
    pins it against reflex's own known answers.)"""
    classes = sorted(set(golds) | set(preds))
    if not classes:
        return float("nan")
    f1_sum = 0.0
    for c in classes:
        tp = sum(1 for g, p in zip(golds, preds) if g == c and p == c)
        fp = sum(1 for g, p in zip(golds, preds) if g != c and p == c)
        fn = sum(1 for g, p in zip(golds, preds) if g == c and p != c)
        f1_sum += 2.0 * tp / max(1.0, 2.0 * tp + fp + fn)
    return f1_sum / len(classes)


def chance_majority_of(golds: list[int]) -> float:
    """The reflex `chance_majority` law: the largest gold-class share."""
    counts: dict[int, int] = {}
    for g in golds:
        counts[g] = counts.get(g, 0) + 1
    return max(counts.values()) / len(golds)


def chance_corrected_skill_of(score: float, chance: float) -> float:
    """The reflex `chance_corrected_skill` law: (score − chance) /
    (1 − chance), clamped [0, 1]; 0 when the denominator is ~0 (chance → 1
    guards to skill 0 — a degenerate one-class read is no skill)."""
    denom = 1.0 - chance
    if denom <= 1e-9:
        return 0.0
    return min(1.0, max(0.0, (score - chance) / denom))


def jdi_axes(hard: dict, gold: list[int] | None,
             picks: list[int] | None) -> None:
    """Add the plan-011 C2 axes to a cell's `hard` block IN PLACE when the
    gold + picks are available: `macro_f1` (the harness law above) and
    `jdi_chance`/`jdi_skill` (the B3 columns the reflex lanes carry). Fields
    stay absent when either input is missing — never zero-filled (the
    docstring's omission law; the site renders "—"."""
    if not gold or not picks or len(gold) != len(picks):
        return
    hard["macro_f1"] = macro_f1_of(gold, picks)
    chance = chance_majority_of(gold)
    hard["jdi_chance"] = chance
    hard["jdi_skill"] = chance_corrected_skill_of(hard["accuracy"], chance)


def h2_record_name(beta: float, n_min: float, tau_n: float) -> str:
    """The arena's H2 arm-name spelling ("H2(β=0.25,nmin=2,τ=2)") — the
    :g format trims 2.0 → 2 exactly as Rust's f64 Display does."""
    return f"H2(β={beta:g},nmin={n_min:g},τ={tau_n:g})"


def serving_arms(arsenal_path: Path) -> dict[str, str]:
    """suite → serving arm record-name, read off the arsenal manifest.
    A0/A1/H1 map to themselves; H2 renders its params into the record
    spelling. The manifest is the serving truth (law A5) — the doc
    displays what the service answers, plus the T2 gate state."""
    try:
        import tomllib
    except ModuleNotFoundError:  # py3.10
        import tomli as tomllib  # type: ignore
    raw = tomllib.loads(arsenal_path.read_text(encoding="utf-8"))
    out: dict[str, str] = {}
    for row in raw.get("vessel", []):
        suite = row.get("suite")
        posture = row.get("posture") or {}
        arm = posture.get("arm")
        if not suite or not arm:
            continue
        if arm == "H2":
            out[suite] = h2_record_name(posture["beta"], posture["n_min"],
                                        posture["tau_n"])
        else:
            out[suite] = arm
    return out


def gate_note(suite: str, serving: str, run: dict) -> str:
    """The T2 gate state of the serving arm, from the frozen record's
    superiority block — displayed, never hidden (the owner display law):
    a tie or a loss stays visible as the improvement backlog."""
    sup = run.get("superiority") or {}
    if sup.get("pick") == serving and sup.get("passed"):
        return (f"certified (paired LB95 {sup['lb95']:+.4f}, "
                f"mean {sup['mean']:+.4f})")
    if serving == "A0":
        if run.get("registered") == "A0" and run.get("instrument_pick") in (None, "A0"):
            return "served by the reflex half — the best measured arm on this suite"
        return "served by the reflex half (A0 is the argmax)"
    return (f"best measured, T2-uncertified (paired LB95 "
            f"{sup.get('lb95', 0.0):+.4f}) — certification is more "
            f"questions, not a posture rollback")


def lane_cell(arm: dict, gold: list[int] | None = None) -> dict:
    """One registered arm → the lane dict (the published cell). `gold` (the
    run's per-question gold indices, plan 011 C2) adds the macro-F1/JDI
    axes when present."""
    correct: list[bool] = arm["correct"]
    confs: list[float] = arm["confs"]
    durs: list[float] = arm["total_durs_us"]
    escalated: list[bool] = arm["escalated"]
    n = len(correct)
    if n == 0:
        raise ValueError("registered arm has zero questions — nothing to publish")
    # The scope law (instinct Issue 007): the published latency cell must
    # name what it timed. New artifacts carry the arena's typed field;
    # the Bench-002 record predates it, so the arm name infers there —
    # A0/H1 compose the reflex seat, A1/H2 time their own forward alone.
    scope = arm.get("contains_seat_solve")
    if scope is None:
        scope = arm["name"] in ("A0", "H1") or arm["name"].startswith("H1")
    latency_scope = "seat+arm" if scope else "arm-only"

    accuracy = sum(correct) / n
    ece = ece_of(list(zip(confs, correct)))

    # acc@50cov: python's sort is stable, so `key=-conf` keeps first-index
    # order on ties — the harness argsort law; k = floor(n*0.5).max(1).
    order = sorted(range(n), key=lambda i: -confs[i])
    k = max(1, math.floor(n * 0.5))
    acc50 = sum(1 for i in order[:k] if correct[i]) / k

    srt = sorted(durs)
    m = len(srt)
    if m == 0:
        raise ValueError("registered arm has zero latency rows — nothing to publish")
    # The latency percentiles index the DURS rows, never the question
    # count: a multi-question suite's seat-composing arm has one durs row
    # per CASE (n != m — disclosed as latency_rows). The first version
    # indexed with n and IndexError'd on the first real typed_decisions
    # record (Bench 005; invisible while every suite was 1q/case).
    p50 = pct_nearest_rank(srt, 50.0) / 1000.0
    p99 = pct_nearest_rank(srt, 99.0) / 1000.0
    p99_val = srt[min(m - 1, max(0, math.ceil(0.99 * m) - 1))]
    tail_support = sum(1 for d in durs if d >= p99_val)

    cell = {
        "lane": LANE_ID,
        "model": arm["name"],
        "hard": {
            "n": n,
            "accuracy": accuracy,
            "ece": ece,
            "mean_confidence": sum(confs) / n,
            "acc_at_50_coverage": acc50,
        },
        "consult_rate": sum(escalated) / n,
        "latency_scope": latency_scope,
        # Issue 010: a multi-question suite's seat-composing latency rows
        # are PER CASE, not per question — disclosed so a reader never
        # divides one by the other.
        "latency_rows": "cases" if n != len(durs) else "questions",
        "latency_p50_ms": p50,
        "latency_p99_ms": p99,
        "latency_tail_support": tail_support,
    }
    # Plan 011 C2: the JDI axes when the run froze gold (the arena stamps
    # it since the C2 change; legacy records join it via --gold-from under
    # the digest pin).
    jdi_axes(cell["hard"], gold, arm.get("picks"))
    return cell


def encoder_cell(run: dict, gold: list[int] | None = None) -> dict | None:
    """The issue-014 C1 encoder arm → its lane cell (RECORD-ONLY,
    serve: ✗). Built from the record's `encoder_arm` block — per-row
    freezes where the record carries them (the stats laws apply), the
    aggregates otherwise (fields the record lacks are OMITTED, never
    zero-filled — the site renders "—"; the 029 record predates the
    per-row fields and publishes accuracy/latency/T2 only)."""
    enc = run.get("encoder_arm")
    if not enc:
        return None
    n = enc.get("n", 0)
    if not n:
        return None
    cell: dict = {
        "lane": ENCODER_LANE_ID,
        "model": "ENC-t6s0 (laya-english encoder + NLEH v1 head)",
        "hard": {"n": n, "accuracy": enc["accuracy"]},
        "consult_rate": 1.0,
        "latency_scope": "arm-only",
        "latency_rows": "questions",
        "latency_p50_ms": enc["p50_us"] / 1000.0,
        "latency_p99_ms": enc["p99_us"] / 1000.0,
        # The C1 disclosure vocabulary: the cell publishes the MEASURED
        # read with the serve refusal — never a serving posture.
        "serves": "✗ (encoder class refused at serve — A1 serves; instinct issue 014 C1)",
        "gate": (
            f"T2-certified above the incumbent A1 (paired LB95 "
            f"{enc['lb95_vs_a1']:+.4f}, mean {enc['mean_vs_a1']:+.4f}) — "
            f"serve REFUSED on the latency class ({enc['p50_us'] / 1000.0:.1f} "
            f"ms/row vs the ~0.3 ms provisional bar; issue 014 decision 1)"
        ),
        "device": enc.get("device"),
        "record_only": True,
    }
    confs = enc.get("confs")
    correct = enc.get("correct")
    if confs and correct and len(confs) == len(correct) == n:
        cell["hard"]["ece"] = ece_of(list(zip(confs, correct)))
        cell["hard"]["mean_confidence"] = sum(confs) / n
        order = sorted(range(n), key=lambda i: -confs[i])
        k = max(1, math.floor(n * 0.5))
        cell["hard"]["acc_at_50_coverage"] = (
            sum(1 for i in order[:k] if correct[i]) / k
        )
    # Plan 011 C2: the same JDI axes on the encoder cell (gold + the
    # encoder arm's frozen picks — the plan-011 C2 crosswalk compares
    # THIS cell against the Clef row on the same axes).
    jdi_axes(cell["hard"], gold, enc.get("picks"))
    return cell


def _arm(name: str, correct: list[bool], confs: list[float],
         durs: list[float], escalated: list[bool],
         seat: bool | None = None, picks: list[int] | None = None) -> dict:
    arm = {"name": name, "correct": correct, "confs": confs,
           "total_durs_us": durs, "escalated": escalated}
    if picks is not None:
        arm["picks"] = picks
    if seat is not None:
        arm["contains_seat_solve"] = seat
    return arm


def _run(suite: str, registered: str, arms: list[dict], n: int = 4) -> dict:
    return {"suite": suite, "n_questions": n, "n_cases": n,
            "registered": registered, "arms": arms}


# The shared known-answer arm (4 questions, hand-computed):
# correct [T,T,F,T] -> accuracy 0.75; confs [.9,.8,.7,.6] each in its own
# 1/15-wide bin with a 1/4 weight -> ece = .25*(|.9-1|+|.8-1|+|.7-0|+|.6-1|)
# = 0.35; acc@50cov k=2 top-conf = 2/2 = 1.0; durs [10,20,30,40] us ->
# p50 0.02 ms (nearest-rank idx 1), p99 0.04 ms (idx 3), tail_support 1.
KA = dict(correct=[True, True, False, True],
          confs=[0.9, 0.8, 0.7, 0.6],
          durs=[10.0, 20.0, 30.0, 40.0])


def selftest() -> int:
    """Issue 007 T4: the scope law is pinned by known-answer, not by the
    docstring. An A1/H2 arm must read `arm-only` (name inference), an H1
    arm `seat+arm`, the typed `contains_seat_solve` field must OVER RIDE
    the name in both directions, an A0-registered suite must carry its
    MEASURED a0_stands cell + reason (never a bare name list — Issue 010
    T3), and the replicated harness metrics must match their
    hand-computed values on a deterministic fixture."""
    preds = {"frozen_test_predictions": [
        {**_run("ag_news", "A0",
                [_arm("A0", escalated=[], seat=True, **KA),
                 _arm("H2", escalated=[False] * 4, **KA)]),
         "instrument_pick": "H2", "registered": "A0",
         "superiority": {"pick": "H2", "mean": 0.015, "lb95": -0.01,
                         "passed": False}},
        _run("banking77", "A0", [_arm("A0", escalated=[], seat=True,
                                       **KA)]),
        _run("emotion", "H1", [_arm("H1", escalated=[True, False, True,
                                                     False],
                                     **KA)]),
        _run("sst5", "A1", [_arm("A1", escalated=[], seat=False,
                                  **KA)]),
        _run("xnli_en", "H2", [_arm("H2", escalated=[], seat=True,
                                     **KA)]),
        # Issue 010: an a0_stands suite with a reason — its measured A0
        # row must be present, not a bare name in a skip list. The arm is
        # the MULTI-QUESTION shape (8 questions, 4 per-case latency rows):
        # the latency math must index the durs rows, never the question
        # count (the Bench-005 IndexError class).
        {**_run("prompt_injections", "A0",
                [_arm("A0", correct=KA["correct"] * 2,
                      confs=KA["confs"] * 2,
                      durs=KA["durs"],
                      escalated=[], seat=True)]),
         "n_questions": 8, "n_cases": 4,
         "a0_note": "no specialist artifact (Issue 010 T2)"},
    ]}
    # The serving law: ag_news serves its best measured arm (H2 — the
    # T2-refused pick), the rest serve what the record registered.
    serving = {"ag_news": "H2"}
    doc = build_doc_from(preds, git_sha="selftest",
                         date_utc="2026-09-27T00:00:00Z", serving=serving)
    by_name = {s["name"]: s for s in doc["suites"]}
    cells = {name: s["hybrid"] for name, s in by_name.items()
             if s["verdict"] == "hybrid_arm"}

    fails: list[str] = []

    def check(cond: bool, why: str) -> None:
        if not cond:
            fails.append(why)

    # Three-state verdicts: the four hybrid cells + the measured A0 cell.
    check(set(cells) == {"ag_news", "emotion", "sst5", "xnli_en"},
          f"hybrid_arm suite set: {sorted(cells)}")
    # The serving-law display: EVERY seated suite carries a hybrid cell
    # (the a0_stands suite's cell IS its measured A0 read — the product
    # answers everywhere, never a hole), carries `serves` + `gate`, and
    # ag_news displays the SERVING arm (H2, the best measured) even
    # though the record's registered field still reads A0 (T2-era data).
    for s in doc["suites"]:
        check(s["hybrid"] is not None,
              f"{s['name']}: the serving law display requires a cell")
        check(s["hybrid"].get("serves"),
              f"{s['name']}: serves rides the cell")
        check(s["hybrid"].get("gate"),
              f"{s['name']}: gate rides the cell")
    ag = by_name["ag_news"]
    check(ag["verdict"] == "hybrid_arm" and ag["hybrid"]["serves"] == "H2",
          f"ag_news serves the best measured arm: {ag['hybrid'].get('serves')}")
    check("uncertified" in ag["hybrid"]["gate"],
          f"ag_news gate names the T2 state: {ag['hybrid'].get('gate')}")
    pi = by_name.get("prompt_injections")
    check(pi is not None and pi["verdict"] == "a0_stands",
          f"prompt_injections verdict: {pi and pi['verdict']}")
    check(pi is not None and pi["hybrid"] is not None
          and pi["hybrid"]["model"] == "A0"
          and pi["measured_a0"] is not None
          and pi["hybrid"]["hard"] == pi["measured_a0"]["hard"],
          "a0_stands carries its measured A0 read AS the lane cell")
    check(pi is not None and pi["reason"] ==
          "no specialist artifact (Issue 010 T2)",
          f"a0_stands reason: {pi and pi['reason']}")
    if pi is not None and pi["measured_a0"] is not None:
        ma = pi["measured_a0"]
        check(abs(ma["hard"]["accuracy"] - 0.75) < 1e-12,
              f"measured A0 accuracy {ma['hard']['accuracy']}")
        check(ma["latency_scope"] == "seat+arm",
              "measured A0 (A0 arm) reads seat+arm")
        # The multi-question arm: hard.n counts QUESTIONS (8), the
        # latency rows are CASES (4), and the p50/p99 are computed over
        # those 4 durs (the Bench-005 IndexError class, pinned).
        check(ma["hard"]["n"] == 8, f"hard.n counts questions: {ma['hard']['n']}")
        check(ma["latency_rows"] == "cases", f"latency_rows: {ma['latency_rows']}")
        check(abs(ma["latency_p50_ms"] - 0.02) < 1e-12,
              f"multi-q p50 over 4 durs: {ma['latency_p50_ms']}")
        check(abs(ma["latency_p99_ms"] - 0.04) < 1e-12,
              f"multi-q p99 over 4 durs: {ma['latency_p99_ms']}")
        check(ma["latency_tail_support"] == 1,
              f"multi-q tail_support: {ma['latency_tail_support']}")

    # Scope: name inference …
    check(cells["ag_news"]["latency_scope"] == "arm-only",
          f"ag_news (H2, inferred): {cells['ag_news']['latency_scope']}")
    check(cells["emotion"]["latency_scope"] == "seat+arm",
          f"emotion (H1, inferred): {cells['emotion']['latency_scope']}")
    # … and the typed field overriding it in BOTH directions.
    check(cells["sst5"]["latency_scope"] == "arm-only",
          f"sst5 (A1 typed false): {cells['sst5']['latency_scope']}")
    check(cells["xnli_en"]["latency_scope"] == "seat+arm",
          f"xnli_en (H2 typed true): {cells['xnli_en']['latency_scope']}")

    # Known-answer metrics on every cell (all four share KA's numbers).
    for name, consult in (("ag_news", 0.0), ("emotion", 0.5),
                          ("sst5", 0.0), ("xnli_en", 0.0)):
        c = cells[name]
        check(c["model"] in ("H2", "H1", "A1"), f"{name} model {c['model']}")
        check(abs(c["hard"]["accuracy"] - 0.75) < 1e-12,
              f"{name} accuracy {c['hard']['accuracy']}")
        check(abs(c["hard"]["ece"] - 0.35) < 1e-12,
              f"{name} ece {c['hard']['ece']}")
        check(abs(c["hard"]["acc_at_50_coverage"] - 1.0) < 1e-12,
              f"{name} acc50 {c['hard']['acc_at_50_coverage']}")
        check(abs(c["hard"]["mean_confidence"] - 0.75) < 1e-12,
              f"{name} mean_conf {c['hard']['mean_confidence']}")
        check(abs(c["latency_p50_ms"] - 0.02) < 1e-12,
              f"{name} p50 {c['latency_p50_ms']}")
        check(abs(c["latency_p99_ms"] - 0.04) < 1e-12,
              f"{name} p99 {c['latency_p99_ms']}")
        check(c["latency_tail_support"] == 1,
              f"{name} tail_support {c['latency_tail_support']}")
        check(abs(c["consult_rate"] - consult) < 1e-12,
              f"{name} consult_rate {c['consult_rate']} != {consult}")

    if fails:
        for f in fails:
            print(f"FAIL {f}")
        print(f"self-test: {len(fails)} failure(s)")
        return 1

    # ── plan 011 C2: the JDI axes + the gold-from pin ──────────────────
    # Hand-computed: golds [0,0,1,2], preds [0,1,1,2] → correct on q0,q2,q3
    # (accuracy .75). Classes {0,1,2}: F1_0 = 2·1/(2·1+0+1) = 2/3 (fp=0,
    # fn=1 — the wrong pick at q1 is class 1's FP); F1_1 = 2·1/(2·1+1+0) =
    # 2/3; F1_2 = 2·1/(2·1+0+0) = 1 → macro_f1 = (2/3 + 2/3 + 1)/3 = 7/9.
    # chance = max share of gold = 2/4 = .5; skill = (.75 − .5)/(1 − .5) = .5.
    g4 = [0, 0, 1, 2]
    check(abs(macro_f1_of(g4, [0, 1, 1, 2]) - 7.0 / 9.0) < 1e-12,
          f"macro_f1 known answer: {macro_f1_of(g4, [0, 1, 1, 2])}")
    check(abs(macro_f1_of([1, 1], [1, 1]) - 1.0) < 1e-12,
          "macro_f1 perfect single class")
    check(abs(chance_majority_of(g4) - 0.5) < 1e-12,
          f"chance_majority: {chance_majority_of(g4)}")
    check(abs(chance_corrected_skill_of(0.75, 0.5) - 0.5) < 1e-12,
          "skill known answer")
    check(chance_corrected_skill_of(1.0, 1.0) == 0.0,
          "chance → 1 guards skill to 0")
    check(chance_corrected_skill_of(0.2, 0.5) == 0.0,
          "below-chance clamps to 0")

    # The native-gold path: a record whose run carries gold + picks gets
    # the axes on its serving cell; a record without gold keeps them ABSENT
    # (never zero-filled).
    gold_run = _run("xnli_en", "A1",
                    [_arm("A1", escalated=[], seat=False,
                          correct=[True, False, True, True],
                          confs=[0.9, 0.8, 0.7, 0.6],
                          durs=[10.0, 20.0, 30.0, 40.0],
                          picks=[0, 1, 1, 2])])
    gold_run["gold"] = g4
    gold_run["test_digest"] = "fnv1a64-abc"
    gd = build_doc_from({"frozen_test_predictions": [gold_run]},
                        git_sha="selftest", date_utc="2026-10-04T00:00:00Z")
    gs = gd["suites"][0]
    gh = gs["hybrid"]["hard"]
    check(gs["test_digest"] == "fnv1a64-abc", "test_digest forwards")
    check(abs(gh.get("macro_f1", float("nan")) - 7.0 / 9.0) < 1e-12,
          f"native gold macro_f1: {gh.get('macro_f1')}")
    check(abs(gh.get("jdi_skill", float("nan")) - 0.5) < 1e-12,
          f"native gold jdi_skill: {gh.get('jdi_skill')}")
    no_gold = build_doc_from(
        {"frozen_test_predictions": [_run("sst5", "A1",
                                         [_arm("A1", escalated=[], seat=False,
                                               **KA)])]},
        git_sha="selftest", date_utc="2026-10-04T00:00:00Z")
    check("macro_f1" not in no_gold["suites"][0]["hybrid"]["hard"]
          and "jdi_skill" not in no_gold["suites"][0]["hybrid"]["hard"],
          "no gold → axes absent, never zero-filled")

    # The --gold-from pin: digest-gated join on legacy records, with the
    # refusals named in the docstring (count mismatch, digest mismatch,
    # double gold).
    legacy = _run("banking77", "A0",
                  [_arm("A0", escalated=[], seat=True,
                        correct=[True, False, True, True],
                        confs=[0.9, 0.8, 0.7, 0.6],
                        durs=[10.0, 20.0, 30.0, 40.0],
                        picks=[0, 1, 1, 2])])
    ok_dump = {"suite": "banking77", "n_questions": 4,
               "cases_digest": "fnv1a64-dd8", "gold": g4}
    ld = build_doc_from({"frozen_test_predictions": [dict(legacy)]},
                        git_sha="selftest", date_utc="2026-10-04T00:00:00Z",
                        gold_from={"banking77": ok_dump})
    ls = ld["suites"][0]
    check(ls["test_digest"] == "fnv1a64-dd8"
          and "dump" in ls.get("test_digest_source", ""),
          "legacy record takes the dump's digest, disclosed")
    check(abs(ls["hybrid"]["hard"].get("macro_f1", float("nan"))
              - 7.0 / 9.0) < 1e-12,
          "gold-from join computes the axes")
    for bad, why in (
        ({**ok_dump, "n_questions": 5}, "count mismatch"),
        ({**ok_dump, "cases_digest": "fnv1a64-OTHER"}, "digest mismatch"),
    ):
        rec = dict(legacy)
        rec["test_digest"] = "fnv1a64-dd8"
        try:
            build_doc_from({"frozen_test_predictions": [rec]},
                           git_sha="s", date_utc="d",
                           gold_from={"banking77": bad})
            fails.append(f"gold-from {why} must refuse")
        except ValueError:
            pass
    rec = dict(legacy)
    rec["gold"] = g4
    try:
        build_doc_from({"frozen_test_predictions": [rec]},
                       git_sha="s", date_utc="d",
                       gold_from={"banking77": ok_dump})
        fails.append("gold-from on a gold-carrying record must refuse")
    except ValueError:
        pass

    if fails:
        for f in fails:
            print(f"FAIL {f}")
        print(f"self-test: {len(fails)} failure(s)")
        return 1
    print("self-test: PASS (7 fixtures, three-state verdicts + scope law + "
          "known-answer metrics + C2 JDI axes + the gold-from pin)")
    return 0


def build_doc_from(preds: dict, git_sha: str, date_utc: str,
                   serving: dict[str, str] | None = None,
                   box_state: dict | None = None,
                   gold_from: dict[str, dict] | None = None) -> dict:
    """`build_doc` over an already-parsed predictions dict (the self-test
    seam; the file path halves share the body).

    `gold_from` (plan 011 C2): suite → a seat_identity dump ({cases_digest,
    gold, n_questions}). For records frozen BEFORE the arena stamped gold:
    the dump's gold joins the frozen picks ONLY under the identity gates —
    n_questions equality, and digest equality against the record's own
    `test_digest` when it carries one (a record that predates stamping
    takes the dump's digest as its own, disclosed via
    `gold_source`/`test_digest_source` — the seat is the identity
    authority and the dump IS a seat rebuild).

    The SERVING law display (owner verdict 2026-09-27): every seated
    suite carries its serving arm's cell — resolved from the arsenal
    manifest via `serving` (suite → arm record-name; None = the record's
    `registered` field, the T2-era fallback). The verdict vocabulary
    tracks the SERVING state: `hybrid_arm` = a specialist arm serves,
    `a0_stands` = the reflex half serves (its measured cell IS the lane
    cell — the product answers, never a hole). `measured_a0` + `reason`
    stay for the record."""
    suites = []
    for run in preds["frozen_test_predictions"]:
        a0_arm = next((a for a in run["arms"] if a["name"] == "A0"), None)
        n_questions = run.get("n_questions")
        n_cases = run.get("n_cases", n_questions)
        suite = run["suite"]
        # Plan 011 C2: the gold source — the run's own stamp (new records)
        # or a digest-gated seat_identity dump (legacy records).
        gold = run.get("gold")
        gold_source = None
        test_digest = run.get("test_digest")
        test_digest_source = None
        dump = (gold_from or {}).get(suite)
        if dump is not None:
            d_n = dump.get("n_questions")
            if d_n != n_questions:
                raise ValueError(
                    f"{suite}: gold dump carries {d_n} questions, the record "
                    f"froze {n_questions} — the populations differ; refusing "
                    f"the join (re-dump the seat over the record's pool)"
                )
            d_dig = dump.get("cases_digest")
            if test_digest and d_dig != test_digest:
                raise ValueError(
                    f"{suite}: gold dump digest {d_dig} != the record's "
                    f"test_digest {test_digest} — refusing the join"
                )
            if gold:
                raise ValueError(
                    f"{suite}: the record already carries gold; --gold-from "
                    f"is for legacy records (drop the dump for this suite)"
                )
            gold = dump.get("gold")
            gold_source = "seat_identity dump (digest-gated join, plan 011 C2)"
            if not test_digest:
                test_digest = d_dig
                test_digest_source = "seat_identity dump (the record predates stamping)"
        serving_name = (serving or {}).get(suite, run.get("registered"))
        serving_arm = next((a for a in run["arms"] if a["name"] == serving_name),
                           None)
        if serving_arm is None:
            raise ValueError(
                f"{suite}: serving arm {serving_name!r} has no measured "
                f"read in the record — the manifest named an unmeasured arm")
        entry = {
            "name": suite,
            "n_questions": n_questions,
            "n_cases": n_cases,
            "verdict": None,
            # Plan 011 C2: the case-identity pin rides the suite entry — a
            # crosswalk asserts THIS against the reflex-side doc's
            # cases_digest before publishing a cell beside it.
            "test_digest": test_digest,
            "measured_a0": lane_cell(a0_arm, gold) if a0_arm else None,
            "reason": run.get("a0_note"),
        }
        if test_digest_source:
            entry["test_digest_source"] = test_digest_source
        if gold_source:
            entry["gold_source"] = gold_source
        # serves/gate ride ON the cell (the published slot), so they
        # travel through publish_bench's cell-path merge untouched.
        cell = lane_cell(serving_arm, gold)
        cell["serves"] = serving_name
        cell["gate"] = gate_note(suite, serving_name, run)
        entry["hybrid"] = cell
        # The issue-014 C1 encoder cell (record-only, serve: ✗) rides the
        # suite entry BESIDE the serving hybrid cell — the measured read
        # is published without touching the serving posture.
        enc = encoder_cell(run, gold)
        if enc is not None:
            entry["encoder"] = enc
        entry["verdict"] = "a0_stands" if serving_name == "A0" else "hybrid_arm"
        suites.append(entry)
    meta = {
        "host": "m3",
        "git_sha": git_sha,
        "date_utc": date_utc,
        "profile": "release",
        "laya_feature": False,
        "lane_note": (
            "instinct hybrid lane: the SERVING arm's single frozen test "
            "read over the reflex harness seat. Serving law (owner, "
            "2026-09-27): the best measured arm serves — resolved from "
            "the arsenal manifest, A0 a candidate like any other; every "
            "seated suite carries its cell (a tie or a loss is shown, "
            "labeled via serves/gate, and stays visible as the Issue-008 "
            "improvement backlog — never hidden). The T2 strict-"
            "superiority gate remains the ADVERTISING law, not the "
            "serving selector."
        ),
    }
    if box_state is not None:
        # The Issue-021 verdict the run stamped (arena box_state.json) —
        # without it the site can never JUDGE the lane's latency and the
        # frontier refuses to plot the rung. The arena's span rides the
        # doc verbatim; publish_bench stamps per-cell verdicts from it.
        meta["box_state"] = box_state
    return {"meta": meta, "suites": suites}


def build_doc(predictions_path: Path, git_sha: str, date_utc: str,
              serving: dict[str, str] | None = None,
              box_state: dict | None = None,
              gold_from: dict[str, dict] | None = None) -> dict:
    preds = json.loads(predictions_path.read_text(encoding="utf-8"))
    return build_doc_from(preds, git_sha, date_utc, serving, box_state,
                          gold_from)


def load_gold_dump(path: Path) -> dict[str, dict]:
    """One seat_identity dump → {suite: dump}. Refuses a dump without its
    identity fields (a dump that cannot be pinned is not a gold source)."""
    d = json.loads(path.read_text(encoding="utf-8"))
    if not d.get("cases_digest") or not d.get("gold") or not d.get("suite"):
        raise ValueError(
            f"{path}: not a seat_identity dump (needs suite/cases_digest/"
            f"gold — emit one with the seat_identity example)"
        )
    return {d["suite"]: d}


def build_doc_merged(paths: list[Path], git_sha: str, date_utc: str,
                     serving: dict[str, str] | None = None,
                     box_state: dict | None = None,
                     gold_from: dict[str, dict] | None = None) -> dict:
    """`build_doc` over a SUITE-LEVEL UNION of several frozen records —
    for the mixed-pool posture: the serving arm may be measured in a
    per-suite record (typed_decisions' full-pool H2, Bench 020) while the
    other suites ride the default-pool full run (Bench 019). Later files
    override earlier ones per suite (argv order = precedence); every
    suite's cell still comes from ONE frozen record and the manifest
    cross-check runs over the merged view, so an arm named in no record
    still refuses. The meta discloses the merge."""
    merged: dict[str, dict] = {}
    order: list[str] = []
    for p in paths:
        preds = json.loads(p.read_text(encoding="utf-8"))
        for run in preds["frozen_test_predictions"]:
            name = run["suite"]
            if name not in merged:
                order.append(name)
            merged[name] = run
    preds = {"frozen_test_predictions": [merged[n] for n in order]}
    doc = build_doc_from(preds, git_sha, date_utc, serving, box_state,
                         gold_from)
    doc["meta"]["lane_note"] += (
        " Record merge: "
        + ", ".join(p.name for p in paths)
        + " (suite-level union, argv order wins; each suite's cell is "
          "one frozen read)."
    )
    return doc


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("predictions", type=Path, nargs="*",
                    help="the arena's frozen predictions.json (repeat to "
                         "merge records per suite; argv order = precedence)")
    ap.add_argument("--out", "-o", type=Path, required=False,
                    help="the lane doc to write (required unless "
                         "--self-test)")
    ap.add_argument("--git-sha", default=None,
                    help="the arena build's sha (default: git rev-parse HEAD)")
    ap.add_argument("--date-utc", default=None)
    ap.add_argument("--box-state", type=Path, default=None,
                    help="the arena's box_state.json (the Issue-021 "
                         "structured span) — embedded in meta so the site "
                         "can JUDGE the lane's latency; without it every "
                         "hybrid timing cell stays unjudged and the "
                         "frontier refuses to plot the rung")
    ap.add_argument("--arsenal", type=Path, default=None,
                    help="the arsenal manifest (the serving truth; "
                         "default: data/arsenal.toml beside this script, "
                         "falling back to the repo-root arsenal.toml — the "
                         "post-split teaching default)")
    ap.add_argument("--gold-from", type=Path, action="append", default=[],
                    metavar="DUMP",
                    help="seat_identity dump(s) for records frozen before "
                         "gold stamping (plan 011 C2): the dump's gold joins "
                         "the frozen picks ONLY under the identity gates "
                         "(n_questions equality + digest equality when the "
                         "record carries test_digest); emit with the "
                         "seat_identity example")
    ap.add_argument("--self-test", action="store_true",
                    help="run the Issue-007 known-answer arms and exit")
    args = ap.parse_args()
    if args.self_test:
        return selftest()
    if not args.predictions and args.out is None and not args.self_test:
        ap.error("predictions and --out are required (or pass --self-test)")
    if args.predictions and args.out is None:
        ap.error("--out is required with predictions")
    git_sha = args.git_sha
    if git_sha is None:
        git_sha = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, check=True,
        ).stdout.strip()
    date_utc = args.date_utc or datetime.now(timezone.utc).strftime(
        "%Y-%m-%dT%H:%M:%SZ")
    arsenal = args.arsenal
    if arsenal is None:
        root = Path(__file__).resolve().parent.parent
        # Post-split (2026-10-03): the embedded manifest lives at
        # data/arsenal.toml; the repo-root spelling is the pre-split
        # fallback so old checkouts keep working.
        arsenal = next((p for p in (root / "data" / "arsenal.toml",
                                    root / "arsenal.toml") if p.exists()),
                       root / "data" / "arsenal.toml")
    serving = serving_arms(arsenal)
    gold_from: dict[str, dict] = {}
    for dump_path in args.gold_from:
        gold_from.update(load_gold_dump(dump_path))
    box_state = None
    if args.box_state is not None:
        box_state = json.loads(args.box_state.read_text(encoding="utf-8"))
        if not isinstance(box_state.get("start"), dict) or not isinstance(box_state.get("end"), dict):
            ap.error(f"--box-state {args.box_state} carries no start/end span")
    doc = (build_doc(args.predictions[0], git_sha, date_utc, serving, box_state,
                     gold_from or None)
           if len(args.predictions) == 1
           else build_doc_merged(args.predictions, git_sha, date_utc, serving,
                                 box_state, gold_from or None))
    args.out.write_text(json.dumps(doc, indent=1) + "\n", encoding="utf-8")
    names = [s["name"] for s in doc["suites"]]
    a0 = [s["name"] for s in doc["suites"] if s["verdict"] == "a0_stands"]
    print(f"hybrid lane doc: {args.out}")
    print(f"  suites: {', '.join(names)}")
    if a0:
        print(f"  a0_stands (reflex half serves — best measured there): "
              f"{', '.join(a0)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

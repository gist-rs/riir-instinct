#!/usr/bin/env python3
"""Package the arena's frozen test read into the reflex-site publish doc.

Reads a `.benchmarks/NNN_*/predictions.json` written by the arena
(`cargo run --release --bin arena`) and emits the results-style JSON doc
`publish_bench.py` consumes (the hybrid lane rides the same merge law as
every other lane — riir-instinct .issues/003).

Per suite: ONLY the REGISTERED arm becomes the lane cell (the
pre-registration instrument's pick — never a post-hoc best-of). A suite
whose registered arm is `A0` carries NO hybrid lane (the hybrid IS reflex
there; a lane cell would be a duplicate of the modelless row wearing a
new label).

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


def lane_cell(arm: dict) -> dict:
    """One registered arm → the lane dict (the published cell)."""
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

    return {
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


def _arm(name: str, correct: list[bool], confs: list[float],
         durs: list[float], escalated: list[bool],
         seat: bool | None = None) -> dict:
    arm = {"name": name, "correct": correct, "confs": confs,
           "total_durs_us": durs, "escalated": escalated}
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
        _run("ag_news", "H2", [_arm("A0", escalated=[], seat=True,
                                     **KA),
                                _arm("H2", escalated=[False] * 4,
                                     **KA)]),
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
    doc = build_doc_from(preds, git_sha="selftest", date_utc="2026-09-27T00:00:00Z")
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
    check(doc["meta"].get("skipped_suites_a0_registered") is None,
          "the bare skip list must be gone (Issue 010 T3)")
    pi = by_name.get("prompt_injections")
    check(pi is not None and pi["verdict"] == "a0_stands",
          f"prompt_injections verdict: {pi and pi['verdict']}")
    check(pi is not None and pi["hybrid"] is None,
          "a0_stands carries no hybrid cell")
    check(pi is not None and pi["measured_a0"] is not None,
          "a0_stands carries its measured A0 cell")
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
    print("self-test: PASS (6 fixtures, three-state verdicts + scope law + known-answer metrics)")
    return 0


def build_doc_from(preds: dict, git_sha: str, date_utc: str) -> dict:
    """`build_doc` over an already-parsed predictions dict (the self-test
    seam; the file path halves share the body).

    Issue 010 T3 — the doc carries the THREE-STATE vocabulary, per suite:
    `hybrid_arm` (a registered non-A0 arm, gates pass — today's shape) ·
    `a0_stands` (seated, single frozen read done, no promotable hybrid
    arm — carries its MEASURED A0 cell + the reason; the old bare
    `skipped_suites_a0_registered` name list could not carry a
    measurement and the site rendered it as never-run) · absent (never
    seated — the site's `not run`)."""
    suites = []
    for run in preds["frozen_test_predictions"]:
        registered = run["registered"]
        a0_arm = next((a for a in run["arms"] if a["name"] == "A0"), None)
        n_questions = run.get("n_questions")
        n_cases = run.get("n_cases", n_questions)
        entry = {
            "name": run["suite"],
            "n_questions": n_questions,
            "n_cases": n_cases,
            "verdict": None,
            "hybrid": None,
            "measured_a0": None,
            "reason": run.get("a0_note"),
        }
        if registered == "A0":
            # Measured — A0 stands. The A0 arm IS the lane cell here: the
            # hybrid is reflex on this suite, and the measurement is the
            # honest content the old skip list dropped.
            if a0_arm is None:
                raise ValueError(
                    f"{run['suite']}: registered A0 but no A0 arm in the record"
                )
            entry["verdict"] = "a0_stands"
            entry["measured_a0"] = lane_cell(a0_arm)
        else:
            arm = next(a for a in run["arms"] if a["name"] == registered)
            entry["verdict"] = "hybrid_arm"
            entry["hybrid"] = lane_cell(arm)
        suites.append(entry)
    meta = {
        "host": "m3",
        "git_sha": git_sha,
        "date_utc": date_utc,
        "profile": "release",
        "laya_feature": False,
        "lane_note": (
            "instinct hybrid lane: the registered arm's single frozen test "
            "read over the reflex harness seat at the current published "
            "reflex posture (the Bench-004 re-baseline). Three states per "
            "suite (Issue 010): hybrid_arm = a registered non-A0 arm serves; "
            "a0_stands = measured, A0 serves (measured_a0 carries the row, "
            "reason names why nothing is sold); absent from this doc = never "
            "seated. A0-stands is NOT a sale — the superiority gate (Issue "
            "008 T2) refused or no specialist exists."
        ),
    }
    return {"meta": meta, "suites": suites}


def build_doc(predictions_path: Path, git_sha: str, date_utc: str) -> dict:
    preds = json.loads(predictions_path.read_text(encoding="utf-8"))
    return build_doc_from(preds, git_sha, date_utc)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("predictions", type=Path, nargs="?",
                    help="the arena's frozen predictions.json")
    ap.add_argument("out", type=Path, nargs="?")
    ap.add_argument("--git-sha", default=None,
                    help="the arena build's sha (default: git rev-parse HEAD)")
    ap.add_argument("--date-utc", default=None)
    ap.add_argument("--self-test", action="store_true",
                    help="run the Issue-007 known-answer arms and exit")
    args = ap.parse_args()
    if args.self_test:
        return selftest()
    if args.predictions is None or args.out is None:
        ap.error("predictions and out are required (or pass --self-test)")
    git_sha = args.git_sha
    if git_sha is None:
        git_sha = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, check=True,
        ).stdout.strip()
    date_utc = args.date_utc or datetime.now(timezone.utc).strftime(
        "%Y-%m-%dT%H:%M:%SZ")
    doc = build_doc(args.predictions, git_sha, date_utc)
    args.out.write_text(json.dumps(doc, indent=1) + "\n", encoding="utf-8")
    names = [s["name"] for s in doc["suites"]]
    a0 = [s["name"] for s in doc["suites"] if s["verdict"] == "a0_stands"]
    print(f"hybrid lane doc: {args.out}")
    print(f"  suites: {', '.join(names)}")
    if a0:
        print(f"  a0_stands (measured, not sold): {', '.join(a0)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

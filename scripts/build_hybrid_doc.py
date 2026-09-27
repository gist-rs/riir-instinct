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
    p50 = pct_nearest_rank(srt, 50.0) / 1000.0
    p99 = pct_nearest_rank(srt, 99.0) / 1000.0
    p99_val = srt[min(n - 1, max(0, math.ceil(0.99 * n) - 1))]
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
        "latency_p50_ms": p50,
        "latency_p99_ms": p99,
        "latency_tail_support": tail_support,
    }


def build_doc(predictions_path: Path, git_sha: str, date_utc: str) -> dict:
    preds = json.loads(predictions_path.read_text(encoding="utf-8"))
    suites = []
    skipped = []
    for run in preds["frozen_test_predictions"]:
        registered = run["registered"]
        if registered == "A0":
            skipped.append(run["suite"])
            continue
        arm = next(a for a in run["arms"] if a["name"] == registered)
        suites.append({
            "name": run["suite"],
            "n_questions": run["n_questions"],
            # one non-noul question per case — the seat's own assertion
            "n_cases": run["n_questions"],
            "hybrid": lane_cell(arm),
        })
    meta = {
        "host": "m3",
        "git_sha": git_sha,
        "date_utc": date_utc,
        "profile": "release",
        "laya_feature": False,
        "lane_note": (
            "instinct hybrid lane: the registered arm's single frozen test "
            "read over the reflex harness seat (the Bench-052 stratified "
            "protocol); a suite whose registered arm is A0 carries no "
            "hybrid lane"
        ),
    }
    if skipped:
        meta["skipped_suites_a0_registered"] = skipped
    return {"meta": meta, "suites": suites}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("predictions", type=Path)
    ap.add_argument("out", type=Path)
    ap.add_argument("--git-sha", default=None,
                    help="the arena build's sha (default: git rev-parse HEAD)")
    ap.add_argument("--date-utc", default=None)
    args = ap.parse_args()
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
    skipped = doc["meta"].get("skipped_suites_a0_registered", [])
    print(f"hybrid lane doc: {args.out}")
    print(f"  suites: {', '.join(names)}")
    if skipped:
        print(f"  skipped (A0 registered): {', '.join(skipped)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

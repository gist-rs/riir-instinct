#!/usr/bin/env python3
"""Bench 0055 — the D4 Q4 fake-quant gate arithmetic (issue 018 Lane D4).

Mirrors `riir-instinct/src/stats.rs` PairedDiff exactly (z = 1.959963984540054,
se = sqrt(sum2 - sum^2/n)/(n-1)/sqrt(n)) and applies the PRE-REGISTERED
margins in PRE_REGISTRATION.md — declared before any of these 14 reads ran.
The margins are Bench 0046's, verbatim: the serving bar is a product
constant, not a per-format noise estimate, and identical margins keep the
Q8/Q4 verdicts comparable. Stdlib only.
"""
import json
import math
import sys
from pathlib import Path

Z = 1.959963984540054
BENCH = Path(__file__).resolve().parent
RUNS = BENCH / "runs"

# (suite, frozen record dir or None, n>=300?) — margins per pre-registration.
SUITES = [
    ("sst5", ".benchmarks/029_sst5_encoder_c1", True),
    ("xnli_en", ".benchmarks/036_xnli_encoder_cell_seating", True),
    ("ag_news", ".benchmarks/037_ag_news_encoder_cell", True),
    ("massive_intent_en", ".benchmarks/0042_massive_encoder_cell", True),
    ("banking77", ".benchmarks/0043_banking77_encoder_cell", True),
    ("prompt_injections", ".benchmarks/0044_prompt_encoder_cell", False),
    ("typed_decisions", ".benchmarks/041_typed_encoder_cell_seating", True),
]
DELTA_BIG, DELTA_SMALL = 0.02, 0.04
CAL_BIG, CAL_SMALL = 0.01, 0.03


def paired(diffs):
    n = float(len(diffs))
    s = sum(diffs)
    s2 = sum(d * d for d in diffs)
    mean = s / n
    var = max((s2 - s * s / n) / (n - 1.0), 0.0) if len(diffs) > 1 else 0.0
    se = math.sqrt(var) / math.sqrt(n)
    return {"mean": mean, "se": se, "ub95": mean + Z * se, "lb95": mean - Z * se}


def ece(confs, correct, bins=10):
    """10 equal-width bins over (0,1] — the reflex protocol convention
    (a zero-confidence row falls in NO bin; here conf>0 always since the
    sigmoid head's argmax confidence is > 0, but keep the law anyway)."""
    buckets = [[] for _ in range(bins)]
    for c, h in zip(confs, correct):
        if c <= 0.0 or c > 1.0:
            continue
        i = min(bins - 1, int(math.ceil(c * bins)) - 1)
        buckets[i].append((c, h))
    e = 0.0
    n = sum(len(b) for b in buckets)
    for b in buckets:
        if not b:
            continue
        conf = sum(x[0] for x in b) / len(b)
        acc = sum(1 for x in b if x[1]) / len(b)
        e += len(b) / n * abs(conf - acc)
    return e


def load_enc(dirpath):
    p = json.loads((Path(dirpath) / "predictions.json").read_text())
    for suite in p["frozen_test_predictions"]:
        e = suite.get("encoder_arm")
        if e:
            return e
    raise SystemExit(f"no encoder_arm in {dirpath}")


def main():
    out = []
    lane_fail = False
    for suite, frozen_dir, big in SUITES:
        f16 = load_enc(RUNS / f"{suite}_f16")
        q4 = load_enc(RUNS / f"{suite}_q4")
        delta = DELTA_BIG if big else DELTA_SMALL
        cal = CAL_BIG if big else CAL_SMALL
        row = {"suite": suite, "delta_margin": delta, "cal_margin": cal}

        # -- the identity witness ------------------------------------------------
        frz = load_enc(Path(BENCH.parent.parent) / frozen_dir) if frozen_dir else None
        if frz is not None and "picks" in frz:
            row["witness"] = "picks_equal" if frz["picks"] == f16["picks"] else "PICKS_DIVERGED"
            row["witness_acc_frozen"] = frz["accuracy"]
        else:
            row["witness"] = "accuracy_only"
            row["witness_acc_frozen"] = frz["accuracy"] if frz else None
        row["witness_acc_fresh"] = f16["accuracy"]
        if row["witness"] == "PICKS_DIVERGED":
            row["verdict"] = "INVALID_WITNESS"
            lane_fail = True
            out.append(row)
            continue
        if frz is not None and abs(frz["accuracy"] - f16["accuracy"]) > 1e-9:
            row["verdict"] = "INVALID_WITNESS"
            lane_fail = True
            out.append(row)
            continue

        picks_f, picks_q = f16["picks"], q4["picks"]
        corr_f, corr_q = f16["correct"], q4["correct"]
        confs_f, confs_q = f16["confs"], q4["confs"]
        n = len(picks_f)
        if not (n == len(picks_q) == len(corr_f) == len(corr_q) == len(confs_f) == len(confs_q)):
            row["verdict"] = "INVALID_ROW_COUNT"
            lane_fail = True
            out.append(row)
            continue

        # -- retention + flips ---------------------------------------------------
        flips = [(i, picks_f[i], picks_q[i]) for i in range(n) if picks_f[i] != picks_q[i]]
        row["n"] = n
        row["retention"] = (n - len(flips)) / n
        row["flips"] = len(flips)

        # -- accuracy non-inferiority -------------------------------------------
        acc_diffs = [float(cq) - float(cf) for cq, cf in zip(corr_q, corr_f)]
        pd = paired(acc_diffs)
        row["acc"] = {
            "f16": sum(corr_f) / n,
            "q4": sum(corr_q) / n,
            "paired": pd,
        }
        if pd["lb95"] > -delta:
            acc_v = "PASS"
        elif pd["ub95"] < -delta:
            acc_v = "FAIL"
        else:
            acc_v = "UNDECIDED"
        row["acc_verdict"] = acc_v
        row["low_power"] = Z * pd["se"] > delta

        # -- calibration (picked-class Brier) ------------------------------------
        def brier(confs, corr, picks):
            return [
                (confs[i] - (1.0 if corr[i] else 0.0)) ** 2 for i in range(len(confs))
            ]

        bf = brier(confs_f, corr_f, picks_f)
        bq = brier(confs_q, corr_q, picks_q)
        bd = paired([q - f for q, f in zip(bq, bf)])
        row["brier"] = {
            "f16": sum(bf) / n,
            "q4": sum(bq) / n,
            "paired": bd,
        }
        if bd["ub95"] < cal:
            cal_v = "PASS"
        elif bd["lb95"] >= cal:
            cal_v = "FAIL"
        else:
            cal_v = "UNDECIDED"
        row["cal_verdict"] = cal_v
        row["ece_f16"] = ece(confs_f, corr_f)
        row["ece_q4"] = ece(confs_q, corr_q)

        # -- flips by gold class (the family-flip detector) ----------------------
        by_gold = {}
        for i, pf, pq in flips:
            g = f16.get("gold_by_row")
            key = g[i] if g else None
            by_gold[str(key)] = by_gold.get(str(key), 0) + 1
        row["flips_by_gold_known"] = bool(f16.get("gold_by_row"))

        v = "PASS"
        if acc_v == "FAIL" or cal_v == "FAIL":
            v = "FAIL"
        elif acc_v == "UNDECIDED" or cal_v == "UNDECIDED":
            v = "UNDECIDED"
        if v == "FAIL":
            lane_fail = True
        row["verdict"] = v
        out.append(row)

    (BENCH / "d4_verdict.json").write_text(json.dumps(out, indent=2) + "\n")
    print(f"{'suite':22s} {'n':>5s} {'ret':>6s} {'flips':>5s} {'accΔ':>8s} {'LB95':>8s} {'accV':>9s} {'UB95ΔBr':>8s} {'calV':>9s} {'verdict':>9s}")
    for r in out:
        if r["verdict"] in ("INVALID_WITNESS", "INVALID_ROW_COUNT"):
            print(f"{r['suite']:22s} {r['verdict']}")
            continue
        a, b, c = r["acc"]["paired"], r["brier"]["paired"], r
        print(
            f"{r['suite']:22s} {r['n']:5d} {r['retention']:6.4f} {r['flips']:5d} "
            f"{a['mean']:+8.4f} {a['lb95']:+8.4f} {c['acc_verdict']:>9s} "
            f"{b['ub95']:+8.4f} {c['cal_verdict']:>9s} {r['verdict']:>9s}"
        )
    print(f"\nLANE VERDICT: {'FAIL' if lane_fail else 'PASS (no suite FAILED — see UNDECIDED rows)'}")
    return 1 if lane_fail else 0


if __name__ == "__main__":
    sys.exit(main())

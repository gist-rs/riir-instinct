#!/usr/bin/env python3
"""Bench 038 — the emotion confusion read (records-only, reproducible).

Answers "why is emotion ±0.0 vs Reflex, and can any lever move the cell?" from
FROZEN artifacts: Bench-016 predictions.json + the t20k emotion test gold.

Run from this directory with the sibling checkouts beside the workspace
(../riir-reflex/.raw/datasets_t20k/emotion must exist).

Self-check: recomputes A0/A1/H1 accuracy from the join and requires EXACT
agreement with the frozen recorded values — a failing self-check aborts (exit 1),
a wrong join must never produce a verdict.
"""
import json
import glob
import re
import sys
from collections import Counter, defaultdict

BENCH = "../016_cache_reuse_seated/predictions.json"
DATA = "../../../riir-reflex/.raw/datasets_t20k/emotion"
LABELS = ["sadness", "joy", "love", "anger", "fear", "surprise"]


def load_rows(pattern):
    rows = []
    for f in sorted(glob.glob(pattern)):
        with open(f, encoding="utf-8") as fh:
            rows.extend(r["row"] for r in json.load(fh)["rows"])
    return rows


def toks(t):
    return [w for w in re.findall(r"[a-z']+", t.lower()) if len(w) > 3]


def main():
    test = load_rows(f"{DATA}/test-*.json")
    train = load_rows(f"{DATA}/train-*.json")
    gold = [int(r["label"]) for r in test]

    pred = json.load(open(BENCH, encoding="utf-8"))["frozen_test_predictions"]
    e = next(r for r in pred if r["suite"] == "emotion")
    arms = {a["name"]: a for a in e["arms"]}
    a0, a1 = arms["A0"], arms["A1"]

    # -- self-check: the join must reproduce the frozen accuracies exactly
    for name in ("A0", "A1", "H1"):
        acc = sum(p == g for p, g in zip(arms[name]["picks"], gold)) / len(gold)
        if abs(acc - arms[name]["accuracy"]) > 1e-9:
            sys.exit(f"SELF-CHECK FAILED on {name}: {acc} != {arms[name]['accuracy']}")
    print(f"self-check OK (A0 {a0['accuracy']} / A1 {a1['accuracy']} / H1 {arms['H1']['accuracy']})")
    print(f"train pool: {len(train)} rows | test: {len(test)} rows | A0 abstain {sum(a0['abstained'])}")

    # -- confusion
    cm = [[0] * 6 for _ in range(6)]
    errs = []
    for i, (pk, g) in enumerate(zip(a0["picks"], gold)):
        cm[g][pk] += 1
        if pk != g:
            errs.append(i)
    print("\nA0 confusion (gold\\pick):  " + "  ".join(f"{l[:7]:>7}" for l in LABELS))
    for gi, l in enumerate(LABELS):
        n = sum(cm[gi])
        print(f"  {l:>8} (n={n:>3}): " + "  ".join(f"{cm[gi][c]:>7}" for c in range(6))
              + f"   recall {cm[gi][gi]/n:.3f}")
    print(f"\nerrors {len(errs)}/400 | by gold: " + str(Counter(LABELS[gold[i]] for i in errs).most_common()))

    # -- finding 1: confidence knows
    ce = [a0["confs"][i] for i in errs]
    print(f"\n[F1] error conf mean {sum(ce)/len(ce):.3f} | all<0.30: {all(c < 0.30 for c in ce)} "
          f"| abstained among errors {sum(a0['abstained'][i] for i in errs)}/{len(errs)}")

    # -- finding 2: recoverability of love->joy by the train pool's own lexical verdict
    tok_lab = defaultdict(Counter)
    for r in train:
        for w in set(toks(r["text"])):
            tok_lab[w][int(r["label"])] += 1
    lj = [i for i in range(len(gold)) if gold[i] == 2 and a0["picks"][i] == 1]
    rec = 0
    print(f"\n[F2] love->joy errors {len(lj)} — train-pool lexical verdict per text:")
    for i in lj:
        votes = Counter()
        ev = []
        for w in set(toks(test[i]["text"])):
            c = tok_lab.get(w)
            if c and sum(c.values()) >= 3:
                top = c.most_common(1)[0]
                votes[top[0]] += 1
                ev.append(f"{w}->{LABELS[top[0]]}({top[1]}/{sum(c.values())})")
        says = votes.most_common(1)[0][0] if votes else None
        if says == 2:
            rec += 1
        print(f"  [{i:3}] train-top={LABELS[says] if says is not None else 'None':>8} "
              f"{'RECOVERABLE' if says == 2 else 'NOISE':11} | {' '.join(ev[:4])}")
    print(f"  RECOVERABLE: {rec}/{len(lj)}")

    # -- finding 3: the fusion consult subset
    abst = [i for i in range(len(gold)) if a0["abstained"][i]]
    a1_on = sum(a1["picks"][i] == gold[i] for i in abst)
    a0_on = sum(a0["picks"][i] == gold[i] for i in abst)
    rescues = sum(a1["picks"][i] == gold[i] for i in errs)
    print(f"\n[F3] A0-abstained n={len(abst)}: A0 forced {a0_on} | A1 {a1_on} "
          f"(specialist {'WORSE' if a1_on < a0_on else 'better'} on the consult subset)")
    print(f"     oracle-selective = {(len(gold)-len(errs)+rescues)}/{len(gold)} "
          f"= {(len(gold)-len(errs)+rescues)/len(gold):.4f} (requires an oracle gate)")


if __name__ == "__main__":
    main()

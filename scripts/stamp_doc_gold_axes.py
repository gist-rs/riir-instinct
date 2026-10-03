#!/usr/bin/env python3
"""Stamp the plan-011-C2 JDI axes onto a LEGACY lane doc, surgically.

The C2 native paths are landed (the arena stamps gold+digest on every new
instinct record; riir-rethink's emitter stamps its ESC records), but the
legacy docs — emitted by the arena's ENCODER auto-emitter (issue 017 T5,
whose post-split home is riir-rethink's archived hunk) — carry the cell and
its attribution verbatim and must not be re-emitted through
build_hybrid_doc.py (a DIFFERENT shape: it would drop head_kind/ckpt/
shape_desc and the --encoder-note attribution prose). This tool adds ONLY
the C2 fields, in place, under the same join gates the builder's
`--gold-from` path enforces:

  - the dump's `n_questions` must equal the arm's row count;
  - the dump's `cases_digest` must equal the record's own `test_digest`
    when the record carries one (it joins as the disclosed source
    otherwise — the seat is the identity authority and the dump IS a seat
    rebuild);
  - a cell that already carries the axes REFUSES (never re-derived).

The metric laws are IMPORTED from build_hybrid_doc (the pinned mirror of
reflex's `macro_f1_of`/`chance_majority`/`chance_corrected_skill`) — one
mirror, consumed, not a second copy.

Usage:
  scripts/stamp_doc_gold_axes.py <lane_doc.json> <predictions.json> \
      [--suite NAME] [--dump seat_dump.json] [--arm encoder|all] [--write]

Default arm set: `encoder` (the legacy encoder-seating docs' one cell);
`all` stamps every arm the predictions carry picks for, matched by the
doc's cell `model` names. Dry-run by default; `--write` mutates the doc.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from build_hybrid_doc import (  # noqa: E402
    chance_majority_of,
    chance_corrected_skill_of,
    macro_f1_of,
)

AXES = ("macro_f1", "jdi_chance", "jdi_skill")


def load_dump(path: Path) -> dict:
    d = json.loads(path.read_text(encoding="utf-8"))
    for k in ("suite", "cases_digest", "gold", "n_questions"):
        if not d.get(k):
            raise SystemExit(f"{path}: not a seat_identity dump (missing {k})")
    return d


def stamp_cell(cell: dict, gold: list[int], picks: list[int], what: str) -> bool:
    hard = cell.get("hard")
    if not isinstance(hard, dict):
        raise SystemExit(f"{what}: no hard block — not a lane cell")
    have = [a for a in AXES if a in hard]
    if have:
        raise SystemExit(
            f"{what}: already carries {have} — this tool stamps legacy docs, "
            f"never re-derives (drop the fields first if a re-join is wanted)"
        )
    if len(gold) != len(picks):
        raise SystemExit(
            f"{what}: dump gold has {len(gold)} rows, the arm froze "
            f"{len(picks)} picks — the populations differ; refusing the join "
            f"(re-dump the seat over the record's pool)"
        )
    n = len(gold)
    hits = sum(1 for g, p in zip(gold, picks) if g == p)
    acc = hits / n
    hard["macro_f1"] = macro_f1_of(gold, picks)
    chance = chance_majority_of(gold)
    hard["jdi_chance"] = chance
    hard["jdi_skill"] = chance_corrected_skill_of(acc, chance)
    return True


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("doc", type=Path)
    ap.add_argument("predictions", type=Path)
    ap.add_argument("--dump", type=Path, required=True,
                    help="the suite's seat_identity dump (the gold source)")
    ap.add_argument("--arm", default="encoder",
                    help="encoder | all (which cells to stamp)")
    ap.add_argument("--write", action="store_true",
                    help="mutate the doc in place (default: dry-run)")
    args = ap.parse_args()

    doc = json.loads(args.doc.read_text(encoding="utf-8"))
    preds = json.loads(args.predictions.read_text(encoding="utf-8"))
    dump = load_dump(args.dump)

    runs = {r["suite"]: r for r in preds["frozen_test_predictions"]}
    stamped = []
    for s in doc.get("suites", []):
        run = runs.get(s["name"])
        if run is None:
            continue
        if s["name"] != dump["suite"]:
            continue
        # The join gates (the builder's --gold-from law).
        rec_digest = s.get("test_digest")
        if rec_digest and rec_digest != dump["cases_digest"]:
            raise SystemExit(
                f"{s['name']}: the record's test_digest {rec_digest} != the "
                f"dump's {dump['cases_digest']} — refusing the join"
            )
        targets: list[tuple[str, dict, list[int]]] = []
        enc = run.get("encoder_arm")
        if enc and enc.get("picks") and s.get("encoder"):
            targets.append((f"{s['name']}.encoder", s["encoder"], enc["picks"]))
        if args.arm == "all":
            for a in run.get("arms", []):
                name = a.get("name")
                cell = (s.get("hybrid") if name == s["hybrid"].get("model")
                        else s.get("measured_a0")) if s.get("hybrid") else None
                # The hybrid cell's model is the SERVING arm's name; A0 maps
                # to measured_a0. Only stamp when the names line up.
                if name == "A0" and s.get("measured_a0"):
                    targets.append(
                        (f"{s['name']}.measured_a0", s["measured_a0"],
                         a["picks"]))
                elif s.get("hybrid", {}).get("model") == name:
                    targets.append(
                        (f"{s['name']}.hybrid", s["hybrid"], a["picks"]))
        for what, cell, picks in targets:
            if stamp_cell(cell, dump["gold"], picks, what):
                stamped.append(what)
        if targets:
            if not rec_digest:
                s["test_digest"] = dump["cases_digest"]
                s["test_digest_source"] = (
                    "seat_identity dump (the record predates stamping)")
            s.setdefault(
                "gold_source",
                "seat_identity dump (digest-gated join, plan 011 C2)")

    if not stamped:
        raise SystemExit("no target cells found — nothing to stamp")
    for w in stamped:
        print(f"stamped {w}")
    if args.write:
        args.doc.write_text(json.dumps(doc, indent=1) + "\n",
                            encoding="utf-8")
        print(f"wrote {args.doc}")
    else:
        print("dry-run — pass --write to mutate the doc")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""Stamp the confidence-derived metrics onto a LEGACY encoder lane doc.

The board-facing gap (2026-10-09 user report): the reflex bench tables show
accuracy for Rethink's record cells but "—" on acc@50% coverage — and the
landing chart's coverage count read 5/9 because only the TIER-FALLBACK cells
(the serving arm's numbers) carried acc@50 while every suite the encoder arm
measured itself lacked it. The frozen records DO carry the per-row data
(`encoder_arm.confs` / `.correct`) for the post-029 seats — the published
docs were emitted before `encoder_cell` computed the fields, and re-emitting
through build_hybrid_doc would DROP head_kind/ckpt/shape_desc and the
attribution prose (the 0ac9375 lesson). This tool adds ONLY the three
confidence-derived hard fields, in place, under join gates:

  - the record's `encoder_arm` carries `confs`/`correct`, and their lengths
    equal the cell's own `hard.n` (population identity);
  - the accuracy recomputed from `correct` reproduces the cell's published
    `hard.accuracy` (bit-exact — both are mean(correct) over the same rows);
  - a CROSS-RECORD stamp (predictions from another bench dir, via
    --source-doc) additionally requires both docs' suite-row `test_digest`
    to exist and match — the same population pin the C2 stamp enforces;
  - a cell that already carries any of the fields REFUSES (stamps are
    never re-derived; drop the fields first if a re-join is wanted).

The metric laws are IMPORTED from build_hybrid_doc (ece_of + the hoisted
acc_at_50_of) — one law, consumed, not a second copy.

Usage:
  scripts/stamp_doc_conf_metrics.py <lane_doc.json> <predictions.json> \
      [--suite NAME] [--source-doc <lane_doc.json>] [--write]
  scripts/stamp_doc_conf_metrics.py <lane_doc.json> --from-doc <re-read doc> \
      [--write]

Default: dry-run (prints what would land). --write mutates the doc.

The --from-doc mode (reflex-site issue 011): the per-row source is a
FRESH RE-READ's lane doc (arena_encoder_read, which computes the fields
natively at the emit) instead of a frozen predictions file — the gates
adapt: n identity + bit-exact accuracy reproduction against the target's
published cell (the population/posture identity witness — the sst5 read
reproduced 316/600 across four independent postures), plus the digest
match when BOTH rows carry one, plus the re-read's own test_digest
printed for the record. The re-read's LATENCY is never transferred — a
loaded-box re-read's timing is not quotable and the target keeps its
original quiet-run figures.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

# Console-safe streams (the console_encoding discipline): verdicts print
# non-ASCII glyphs and must survive a cp874-class console.
for _stream in (sys.stdout, sys.stderr):
    try:
        _stream.reconfigure(errors="backslashreplace")
    except (AttributeError, ValueError):
        pass

sys.path.insert(0, str(Path(__file__).resolve().parent))
from build_hybrid_doc import acc_at_50_of, ece_of  # noqa: E402

FIELDS = ("acc_at_50_coverage", "ece", "mean_confidence")


def die(msg: str) -> "NoReturn":  # type: ignore[valid-type]
    print(f"⛔ {msg}", file=sys.stderr)
    raise SystemExit(1)


def write_pure_addition(path: Path, doc: dict) -> None:
    """A stamp is a PURE field addition: the doc keeps its own indentation
    (the arena auto-emitter writes indent=2; build_hybrid_doc writes 1) AND
    its own escape posture (auto-emitted docs are raw UTF-8 —, ✗;
    build_hybrid_doc writes ASCII-escaped \u2014). Either dimension
    normalised buries the three new lines in a whole-file diff."""
    raw = path.read_text(encoding="utf-8")
    second = next((l for l in raw.splitlines()[1:] if l.strip()), "")
    indent = len(second) - len(second.lstrip()) if second else 1
    ascii_escaped = bool(re.search(r"\\u[0-9a-fA-F]{4}", raw))
    path.write_text(json.dumps(doc, indent=indent, ensure_ascii=ascii_escaped) + "\n",
                    encoding="utf-8")


def suite_row(doc: dict, suite: str | None, what: str) -> dict:
    rows = doc.get("suites") or []
    if suite is None and len(rows) == 1:
        return rows[0]
    pick = [r for r in rows if r.get("name") == suite] if suite else rows
    if len(pick) != 1:
        die(f"{what}: expected exactly one suite row"
            f" ({suite or 'the single row'}), got {len(pick)} — pass --suite")
    return pick[0]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("doc", type=Path, help="the lane doc to stamp")
    ap.add_argument("predictions", type=Path, nargs="?",
                    help="the arena's frozen predictions.json (omit with --from-doc)")
    ap.add_argument("--from-doc", type=Path, default=None,
                    help="a fresh re-read's lane doc carrying the computed fields "
                         "(arena_encoder_read) — the source instead of predictions.json")
    ap.add_argument("--suite", default=None,
                    help="the suite to stamp (required when the doc or the "
                         "record carries more than one)")
    ap.add_argument("--source-doc", type=Path, default=None,
                    help="the lane doc of the record the predictions came "
                         "from, when it is not this doc's own record — its "
                         "test_digest must match (the cross-record join gate)")
    ap.add_argument("--write", action="store_true",
                    help="mutate the doc (default: dry-run)")
    args = ap.parse_args()

    doc = json.loads(args.doc.read_text(encoding="utf-8"))
    row = suite_row(doc, args.suite, str(args.doc))
    name = row.get("name")
    cell = row.get("encoder")
    if not isinstance(cell, dict) or not isinstance(cell.get("hard"), dict):
        die(f"{name}: no encoder lane cell — this tool stamps encoder docs")
    hard = cell["hard"]

    have = [f for f in FIELDS if f in hard]
    if have:
        die(f"{name}: hard already carries {have} — stamps are never "
            f"re-derived (drop the fields first if a re-join is wanted)")

    if args.from_doc is not None:
        if args.predictions is not None:
            die("pass either <predictions.json> or --from-doc, never both")
        src = json.loads(args.from_doc.read_text(encoding="utf-8"))
        srow = suite_row(src, name, str(args.from_doc))
        scell = srow.get("encoder")
        if not isinstance(scell, dict) or not isinstance(scell.get("hard"), dict):
            die(f"{args.from_doc}: no encoder lane cell — the re-read doc is malformed")
        shard = scell["hard"]
        missing = [f for f in FIELDS if not isinstance(shard.get(f), (int, float))]
        if missing:
            die(f"{name}: the re-read doc's cell lacks {missing} — rebuild the "
                "example at a commit that emits them")
        sn = shard.get("n")
        if sn != hard.get("n"):
            die(f"{name}: n identity failed — the re-read froze {sn} rows vs the "
                f"published {hard.get('n')}")
        sacc, pub = shard.get("accuracy"), hard.get("accuracy")
        if not isinstance(pub, (int, float)) or abs(sacc - pub) > 1e-12:
            die(f"{name}: accuracy reproduction failed — the re-read computes "
                f"{sacc!r} vs the published {pub!r}; the posture/population moved")
        a, b = row.get("test_digest"), srow.get("test_digest")
        if a and b and a != b:
            die(f"{name}: test_digest {a!r} vs the re-read's {b!r} — the "
                "populations differ")
        stamped = {f: shard[f] for f in FIELDS}
        for k, v in stamped.items():
            print(f"  {name}: hard.{k} = {v!r}")
        print(f"  (source: {args.from_doc} · re-read test_digest "
              f"{b or '(none)'} · its latency is NOT transferred)")
        if not args.write:
            print("dry-run — pass --write to stamp")
            return 0
        hard.update(stamped)
        write_pure_addition(args.doc, doc)
        print(f"✓ stamped {args.doc} ({name}) from the re-read doc")
        return 0
        print(f"✓ stamped {args.doc} ({name}) from the re-read doc")
        return 0

    if args.predictions is None:
        ap.error("either <predictions.json> or --from-doc is required")
    recs = pred.get("frozen_test_predictions") or []
    rec = next((r for r in recs if r.get("suite") == name), None)
    if rec is None:
        die(f"{args.predictions}: no frozen record for suite {name!r}")
    enc = rec.get("encoder_arm") or {}
    confs, correct = enc.get("confs"), enc.get("correct")
    n = enc.get("n")
    if not confs or not correct or n != hard.get("n") \
            or len(confs) != n or len(correct) != n:
        die(f"{name}: the record's encoder_arm lacks per-row confs/correct "
            f"at n={hard.get('n')} (n={n}, confs={len(confs or [])}, "
            f"correct={len(correct or [])}) — the record predates the "
            "per-row freeze; the field needs a re-measure, never a stamp")

    acc = sum(1 for c in correct if c) / n
    pub = hard.get("accuracy")
    if not isinstance(pub, (int, float)) or abs(acc - pub) > 1e-12:
        die(f"{name}: accuracy reproduction failed — record rows compute "
            f"{acc!r} vs the published {pub!r}; the populations differ")

    if args.source_doc is not None:
        src = json.loads(args.source_doc.read_text(encoding="utf-8"))
        srow = suite_row(src, name, str(args.source_doc))
        a, b = row.get("test_digest"), srow.get("test_digest")
        if not a or not b or a != b:
            die(f"{name}: cross-record join refused — test_digest "
                f"{a!r} vs {b!r} (both must exist and match)")
        print(f"✓ cross-record digest pin holds: {a}")

    stamped = {
        "ece": ece_of(list(zip(confs, correct))),
        "mean_confidence": sum(confs) / n,
        "acc_at_50_coverage": acc_at_50_of(confs, correct),
    }
    for k, v in stamped.items():
        print(f"  {name}: hard.{k} = {v!r}")
    if not args.write:
        print("dry-run — pass --write to stamp")
        return 0
    hard.update(stamped)
    write_pure_addition(args.doc, doc)
    print(f"✓ stamped {args.doc} ({name})")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/bin/sh
# fence_gate.sh — the Research-003 fence for the Instinct open (riir-ai Proposal 052,
# verdict round 1 correction 1: THE FRESH-ROOT LAW).
#
# Two directions, per the verdict:
#   A. the public tree must not IMPORT the moat (encoder/decstat/hosted-vessel
#      modules, features, deps) and must not default to production data paths;
#   B. the allowlist is the only amnesty (receipt.rs's cfg_attr decstat
#      annotations), and --self-test proves the gate FIRES (a planted import
#      must red — a gate that cannot red certifies nothing).
#
# Verdicts, never interchangeable:
#   PRE-SPLIT  moat files still present in the tree  -> exit 0, loud disclosure
#               (the delete/move family has not landed yet; informational — a
#               gate that reds every day until the wave is a gate nobody runs)
#   RED        moat absent but a moat reference remains      -> exit 1, rows listed
#   GREEN      moat absent, no references, allowlist only    -> exit 0
#
# ENFORCE MODE (--post-split): identical scan, but PRE-SPLIT is RED (exit 1).
# Public CI must use ONLY this mode — the informational exit-0 exists for the
# private pre-split tree and must never guard a public lane (a returning
# deploy.yaml would otherwise pass as PRE-SPLIT).
#
# HISTORY MODE (--history): the fresh-root law's own arm — scans EVERY commit's
# file list of a repo (git log --all --name-only, the union with
# `git rev-list --objects --all` paths) for moat paths (source files + the moat
# records/docs/tests), and requires exactly ONE root commit. This is what the
# owner reads before the visibility flip — the working-tree scan above cannot
# see history. NOTE: on THIS private repo the history audit is expected RED
# (the moat history is whole here by design) — it goes GREEN only on the
# Phase-C fresh-root export.
#
# The allowlist lives beside this gate (`fence_gate_allowlist.txt`, rows
# `path:pattern`) — the ONLY amnesty, and --self-test proves both directions.
#
# Usage:
#   scripts/fence_gate.sh [TREE]            # audit TREE (default: repo root)
#   scripts/fence_gate.sh --post-split [TREE]  # ENFORCE: PRE-SPLIT is RED
#   scripts/fence_gate.sh --history [REPO]  # fresh-root history audit
#   scripts/fence_gate.sh --self-test

set -u

ENFORCE=0
GATE_DIR=$(cd "$(dirname "$0")" && pwd)
ALLOWLIST="$GATE_DIR/fence_gate_allowlist.txt"

# The moat files: their PRESENCE means the tree is pre-split (Proposal 052
# move ledger). Paths relative to the audited tree.
MOAT_FILES="
src/encoder_serve.rs
src/encoder_arm.rs
src/vessel.rs
src/decstat.rs
src/decstat_verify.rs
examples/decstat_floor.rs
deploy.yaml
arsenal.toml
"

# The moat references: CODE surfaces only (*.rs + Cargo.toml) — docs prose is
# free to name the moat (the README explains where the encoder arm lives).
# `feature = "decstat"` is scanned so the receipt.rs amnesty row is
# LOAD-BEARING: the annotation form passes ONLY where allowlisted — any other
# file carrying decstat-gated attrs post-split reds and forces a conscious
# prune-or-allowlist decision (src/lib.rs's `cfg_attr(feature = "decstat",
# pub mod decstat;)` decl and serve.rs's leftover attrs are expected to PRUNE
# at the move family, not to join the allowlist).
MOAT_PATTERNS='
encoder_serve
encoder_arm
EncoderLane
instinct_encoder_lane
instinct_laya_head
load_hosted
crate::decstat
decstat::
decstat_verify
feature = "decstat"
serve-encoder
serve_encoder
arena-laya
arena_laya
dep:riir-kat
instinct_specialists
'

# Files the audit scans, relative to the audited tree.
SCAN_GLOBS="*.rs"

# The moat RECORD/DOC/TEST paths for --history (regex, ERE). The source files
# above are matched exactly; these are the paths whose very PRESENCE in any
# commit leaks the moat (recipes, design records, the encoder A/B instrument,
# the moat gates). `^moat/` is the Rethink SEED itself (Proposal 052's move
# family relocated the moat there) — the export never copies it, so any
# moat/ path in an export's history is a manifest violation.
# ⚠ 2026-10-03 audit: `0(29|…)` matches the `029` spelling only — the four-digit
# `0029_…` record is NOT covered here (and never was). The working-tree records
# arm below carries its own corrected pattern (`0+`), so the gap is closed where
# it still runs. The history arm itself is RETIRED for THIS repo (the owner's
# same-day history sanction — moat content in git HISTORY is owner-sanctioned);
# it stays for any FUTURE fresh-root export.
HISTORY_MOAT_RE='^(\.benchmarks/0(29|3[0-9]|4[0-9]|5[0-6])|\.issues/01[4-8]|\.plans/00[267]|\.proposals/001|\.deploy/|scripts/encoder_load_ab\.py|moat/|tests/(vessel_gates|decstat_gates|serve_encoder_parity|encoder_shared_parity)\.rs)'

# The moat-CLASSED bench RECORDS in the WORKING TREE (the 052 ledger's
# ".benchmarks/029, 031–048, 0050–0056" class). `0+` matches both the `029`
# and `0029` spellings. The working-tree scan (direction R below) runs this on
# <tree>/.benchmarks — after the owner's history sanction the fence is the only
# instrument still watching records, and the 2026-10-03 audit measured that the
# pre-audit GREEN said nothing about them.
RECORD_MOAT_RE='^\.benchmarks/0+(29|3[0-9]|4[0-9]|5[0-6])(_|$)'

# The DELIBERATE STAYS — the published-evidence records that back the live
# reflex.gist.rs cells and the public gate suite, allowed BY NAME (a set, never
# a count — a cardinality-preserving swap must red). Everything else matching
# RECORD_MOAT_RE is a finding: the next moat record that lands in the public
# tree must NOT read GREEN.
# - 0029_massive_synth_seat: the massive serving seat — tests/serve_gates.rs
#   READS its predictions.json (a public test depends on it).
# - 036/037/041: the seated encoder cells (xnli/ag_news/typed) whose
#   hybrid_lane_doc.json is the site's published provenance.
# - 038/039/040: the emotion confusion read, the T8 screens, the v2-head
#   record — md/json only (no heads/binaries; weights never enter repos).
# - 049: the family full-coverage record (the Bench-049 lane doc's input).
# - 0051–0053: the hybrid quotable ladder + synth seat (plan 008 B2's
#   recorded deviation — the open product's published evidence).
RECORDS_ALLOWED="
0029_massive_synth_seat
036_xnli_encoder_cell_seating
037_ag_news_encoder_cell
038_emotion_confusion_read
039_encoder_screens_bank_prompt_typed
040_typed_encoder_v2_head
041_typed_encoder_cell_seating
049_family_full_coverage
0051_families_wide_eval
0052_hybrid_quotable_ladder
0053_massive_synth_0052
"

_records_allowed() {
	for _a in $RECORDS_ALLOWED; do [ "$_a" = "$1" ] && return 0; done
	return 1
}

die() { printf '⛔ %s\n' "$1" >&2; exit 1; }

# Is (file=$_f_rel, pattern=$_pat) allowlisted? Reads the allowlist FILE row by
# row — the query must never be compared against itself (the v1 bug: passing the
# query AS the row let it match itself and allow everything, which the self-test
# arm-2 RED caught before this gate ever certified anything).
allowed() {
	_list=${ALLOWLIST_OVERRIDE:-$ALLOWLIST}
	[ -f "$_list" ] || return 1
	while IFS= read -r _row; do
		case "$_row" in ''|'#'*) continue ;; esac
		_a_path=${_row%%:*}
		_a_pat=${_row#*:}
		[ "$_a_path" = "$_f_rel" ] || continue
		[ "$_a_pat" = "$_pat" ] && return 0
	done < "$_list"
	return 1
}

audit_tree() {
	_tree=$1
	[ -d "$_tree" ] || die "not a directory: $_tree"

	# --- direction 0: PRE-SPLIT disclosure -------------------------------
	_present=0
	for _f in $MOAT_FILES; do
		[ -f "$_tree/$_f" ] && { printf 'PRE-SPLIT moat file present: %s\n' "$_f"; _present=1; }
	done
	if [ "$_present" -eq 1 ]; then
		if [ "$ENFORCE" -eq 1 ]; then
			printf 'VERDICT: RED (enforce) — moat files present in a tree that claims to be post-split:\n'
			for _f in $MOAT_FILES; do [ -f "$_tree/$_f" ] && printf '  %s\n' "$_f"; done
			return 1
		fi
		printf 'VERDICT: PRE-SPLIT — moat files still in the tree (expected until the Proposal-052 move family lands). The fence is NOT yet enforceable here.\n'
		return 0
	fi

	# --- direction A: no moat references in code --------------------------
	# The pattern list is fed through a heredoc (NOT `for in $var`) because a
	# pattern may contain spaces (`feature = "decstat"`) — word-splitting
	# fragments it into three dead patterns, and the amnesty row can never
	# match. The heredoc keeps the loop in this shell, so `_find_hits`
	# accumulates.
	_find_hits=""
	while IFS= read -r _pat; do
		[ -n "$_pat" ] || continue
		_hits=$(cd "$_tree" && find src examples benches tests -name "$SCAN_GLOBS" -type f 2>/dev/null | \
			while IFS= read -r _f_rel; do
				if grep -nF -- "$_pat" "$_f_rel" >/dev/null 2>&1; then
					if allowed; then continue; fi
					printf '  %s: %s\n' "$_f_rel" "$_pat"
				fi
			done)
		[ -n "$_hits" ] && _find_hits="$_find_hits
$_hits"
	done <<EOF
$MOAT_PATTERNS
EOF

	# Cargo.toml: feature/dep surfaces
	_toml_hits=$(grep -nE 'serve-encoder|arena-laya|dep:riir-kat|^decstat' "$_tree/Cargo.toml" 2>/dev/null \
		| sed 's/^/  Cargo.toml: /')
	[ -n "$_toml_hits" ] && _find_hits="$_find_hits
$_toml_hits"

	if [ -n "$(printf '%s' "$_find_hits" | tr -d '[:space:]')" ]; then
		printf 'VERDICT: RED — moat references in a post-split tree:\n%s\n' "$_find_hits"
		return 1
	fi

	# --- direction R: bench RECORDS (the moat-classes set, stays by name) --
	# The 2026-10-03 audit measured that GREEN said nothing about .benchmarks:
	# the history arm that watched records is retired for this repo, so the
	# working-tree scan owns them now. A moat-classed record outside the
	# allowed set is a finding in BOTH modes (its presence is a fact about the
	# tree, not a posture) — RED under enforce, listed in disclosure mode.
	_record_hits=""
	if [ -d "$_tree/.benchmarks" ]; then
		for _d in "$_tree"/.benchmarks/*/; do
			[ -d "$_d" ] || continue
			_d_rel=".benchmarks/$(basename "$_d")"
			printf '%s' "$_d_rel" | grep -qE "$RECORD_MOAT_RE" || continue
			_records_allowed "$(basename "$_d")" && continue
			_record_hits="$_record_hits
  $_d_rel (moat-classed record, not on the deliberate-stays list)"
		done
	fi
	if [ -n "$(printf '%s' "$_record_hits" | tr -d '[:space:]')" ]; then
		if [ "$ENFORCE" -eq 1 ]; then
			printf 'VERDICT: RED (enforce) — moat-classed bench record(s) outside the deliberate-stays set:%s\n' "$_record_hits"
			return 1
		fi
		printf 'VERDICT: RED — moat-classed bench record(s) outside the deliberate-stays set:%s\n' "$_record_hits"
		return 1
	fi

	printf 'VERDICT: GREEN — no moat files, no moat references, records within the deliberate-stays set.\n'
	return 0
}

# --history: the fresh-root law's own audit (see header). Every path ANY commit
# ever carried, plus exactly-one-root. This is the owner's pre-flip read.
history_audit() {
	_repo=$1
	[ -d "$_repo/.git" ] || die "not a git repo: $_repo"
	_paths=$(git -C "$_repo" log --all --name-only --format= 2>/dev/null | sort -u)
	[ -n "$_paths" ] || die "no readable history in $_repo"
	_roots=$(git -C "$_repo" rev-list --max-parents=0 --all 2>/dev/null | wc -l | tr -d ' ')
	_hits=""
	for _f in $MOAT_FILES; do
		printf '%s\n' "$_paths" | grep -qxF -- "$_f" && _hits="$_hits
  $_f"
	done
	_rhits=$(printf '%s\n' "$_paths" | grep -E "$HISTORY_MOAT_RE" || :)
	[ -n "$_rhits" ] && _hits="$_hits
$_rhits"
	if [ "$_roots" -ne 1 ]; then
		printf 'VERDICT: RED (history) — %s root commits (the fresh root must be ONE):\n%s\n' "$_roots" "$_hits"
		return 1
	fi
	if [ -n "$_hits" ]; then
		printf 'VERDICT: RED (history) — moat paths present in some commit:\n%s\n' "$_hits"
		return 1
	fi
	printf 'VERDICT: GREEN (history) — single root commit, zero moat paths across ALL commits.\n'
	return 0
}

self_test() {
	_tmp=$(mktemp -d) || die "mktemp failed"
	trap 'rm -rf "$_tmp"' EXIT
	mkdir -p "$_tmp/fx/src" "$_tmp/fx/examples" "$_tmp/fx/benches" "$_tmp/fx/tests"
	cp "$ALLOWLIST" "$_tmp/fx-allowlist.txt" 2>/dev/null || : # fixture uses the real allowlist via env

	# arm 1: a clean teaching tree must be GREEN (post-split posture: no moat files)
	printf 'use crate::specialist;\n' > "$_tmp/fx/src/lib.rs"
	printf 'pub fn noop() {}\n' > "$_tmp/fx/src/specialist.rs"
	_out=$(ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") || \
		die "self-test arm 1: clean tree did not read GREEN: $_out"
	printf '%s' "$_out" | grep -q 'VERDICT: GREEN' || die "self-test arm 1: verdict not GREEN: $_out"

	# arm 2: a planted moat import must RED
	printf 'use crate::encoder_serve::EncoderLane;\n' > "$_tmp/fx/src/lib.rs"
	_out=$(ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") && \
		die "self-test arm 2: planted import did NOT red (gate cannot fire = certifies nothing): $_out"
	printf '%s' "$_out" | grep -q RED || die "self-test arm 2: verdict not RED: $_out"

	# arm 3: a planted moat FILE must read PRE-SPLIT
	printf 'pub struct EncoderLane;\n' > "$_tmp/fx/src/encoder_arm.rs"
	_out=$(ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") || \
		die "self-test arm 3: moat-file presence should be PRE-SPLIT (exit 0), not an error: $_out"
	printf '%s' "$_out" | grep -q PRE-SPLIT || die "self-test arm 3: verdict not PRE-SPLIT: $_out"

	# arm 4: the SAME moat file under ENFORCE must RED (public CI's mode —
	# a returning moat file can never pass as PRE-SPLIT there)
	_out=$(ENFORCE=1 ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") && \
		die "self-test arm 4: enforce mode did NOT red on a moat file: $_out"
	printf '%s' "$_out" | grep -q RED || die "self-test arm 4: verdict not RED under enforce: $_out"

	# arm 5: --history on a repo whose history carried a moat path must RED ...
	_h=$(mktemp -d)
	git -C "$_h" init -q
	git -C "$_h" -c user.email=t@t -c user.name=t commit -q --allow-empty -m root
	mkdir -p "$_h/src" && printf 'x\n' > "$_h/src/encoder_arm.rs"
	git -C "$_h" add -A
	git -C "$_h" -c user.email=t@t -c user.name=t commit -q -m moat
	_out=$(history_audit "$_h") && die "self-test arm 5a: moat history did NOT red: $_out"
	printf '%s' "$_out" | grep -q RED || die "self-test arm 5a: verdict not RED: $_out"

	# ... and on a clean single-root repo must GREEN
	_h2=$(mktemp -d)
	git -C "$_h2" init -q
	git -C "$_h2" -c user.email=t@t -c user.name=t commit -q --allow-empty -m root
	printf 'pub fn noop() {}\n' > "$_h2/lib.rs"
	git -C "$_h2" add -A
	git -C "$_h2" -c user.email=t@t -c user.name=t commit -q -m clean
	_out=$(history_audit "$_h2") || die "self-test arm 5b: clean history did not read GREEN: $_out"
	printf '%s' "$_out" | grep -q 'GREEN (history)' || die "self-test arm 5b: verdict not GREEN: $_out"
	rm -rf "$_h" "$_h2"

	# arm 6: a planted moat-classed RECORD outside the stays must RED ...
	# (the fixture must be post-split-clean here — arm 2's planted import and
	# arm 3's moat file would fire directions 0/A first)
	rm -f "$_tmp/fx/src/encoder_arm.rs"
	printf 'use crate::specialist;\n' > "$_tmp/fx/src/lib.rs"
	mkdir -p "$_tmp/fx/.benchmarks/0029_massive_synth_seat" "$_tmp/fx/.benchmarks/045_planted_moat_record"
	_out=$(ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") && \
		die "self-test arm 6: planted moat record did NOT red: $_out"
	printf '%s' "$_out" | grep -q RED || die "self-test arm 6: verdict not RED: $_out"
	printf '%s' "$_out" | grep -q '045_planted_moat_record' || die "self-test arm 6: red does not name the planted record: $_out"
	printf '%s' "$_out" | grep -q '0029_massive_synth_seat' && die "self-test arm 6: the ALLOWED stay 0029 was flagged"

	# arm 7: the allowed stays alone must GREEN (the set, not a count — the
	# v1 allowlist query-matches-itself bug is the precedent for arming this)
	rm -rf "$_tmp/fx/.benchmarks/045_planted_moat_record"
	_out=$(ALLOWLIST_OVERRIDE="$_tmp/fx-allowlist.txt" audit_tree "$_tmp/fx") || \
		die "self-test arm 7: allowed stays did not read GREEN: $_out"
	printf '%s' "$_out" | grep -q 'VERDICT: GREEN' || die "self-test arm 7: verdict not GREEN: $_out"

	printf '✅ fence_gate self-test: 7/7 arms fired (GREEN / RED / PRE-SPLIT / enforce-RED / history RED+GREEN / record-RED / stays-GREEN).\n'
}

main() {
	case "${1:-}" in
	--self-test) self_test; return $? ;;
	--post-split) ENFORCE=1; shift; audit_tree "${1:-$GATE_DIR/..}" ;;
	--history) shift; history_audit "${1:-$GATE_DIR/..}" ;;
	*) audit_tree "${1:-$GATE_DIR/..}" ;;
	esac
}

main "$@"

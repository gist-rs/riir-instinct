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
# The allowlist lives beside this gate (`fence_gate_allowlist.txt`, rows
# `path:pattern`) — the ONLY amnesty, and --self-test proves both directions.
#
# Usage:
#   scripts/fence_gate.sh [TREE]     # audit TREE (default: repo root)
#   scripts/fence_gate.sh --self-test

set -u

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

	printf 'VERDICT: GREEN — no moat files, no moat references (allowlist honored).\n'
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

	printf '✅ fence_gate self-test: 3/3 arms fired (GREEN / RED / PRE-SPLIT).\n'
}

main() {
	if [ "${1:-}" = "--self-test" ]; then self_test; return $?; fi
	audit_tree "${1:-$GATE_DIR/..}"
}

main "$@"

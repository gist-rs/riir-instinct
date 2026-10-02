#!/usr/bin/env bash
# cross-build.sh — the Issue 008 remedy: cross-compile a cargo bin for a
# target triple from ANY host, then RE-CHECK the product's ELF header before
# it can ride a container stage.
#
# Why this exists: the container providers (cf-container / docker) stage the
# built binary into a `debian:bookworm-slim` image. A host-built binary on a
# mac is Mach-O — the image builds fine and the container dies at start with
# `exec format error` (on Cloudflare, after a full remote build). The
# deployer's stage-time gate `ensure_container_elf` refuses that; this script
# is the BUILD-TIME half — it produces the right binary in the first place
# and verifies its own product (defense in depth: build-time check here,
# stage-time re-check there).
#
# Tool ladder (first that works, force with RIIR_CROSS_TOOL=cargo|zigbuild|cross):
#   1. cargo     — triple == rustc host (native build; also the Linux-host case)
#   2. zigbuild  — cargo-zigbuild + zig on PATH (mac→linux gnu/musl; C/C++ deps
#                  cross via zig cc — the sentencepiece-sys class)
#   3. cross     — cross-rs (docker-based)
#
# Usage (the deployer's plan emits exactly this shape):
#   sh cross-build.sh [--expect <binary-path>] -- build [--release] [-p pkg]
#          [--bin b] [--features f,...] --target <triple>[.<glibc>]
#
# - `--target` is REQUIRED (this wrapper exists for triple builds).
# - A `.<glibc>` suffix (zigbuild's pinned-glibc form, e.g.
#   x86_64-unknown-linux-gnu.2.36 — bookworm is 2.36) is KEPT for zigbuild
#   and STRIPPED for the other tools.
# - `--expect` names the binary the build must produce; it is verified
#   (existence + ELF magic + e_machine vs the triple's arch) — the same
#   bytes `ensure_container_elf` re-checks at stage time.
# - Exit 0 iff the build ran AND (when given) the expect-bin verified.
#
# Self-test: `sh cross-build.sh --self-test` — fixture arms over the ELF
# verifier + triple parsing; needs no toolchain.

set -euo pipefail

# Completion sentinel — an aborting `set -e` script on macOS bash 3.2 can
# enter the EXIT trap with $? already 0, and a bare cleanup trap launders
# the abort to exit 0 (the katgpt-rs Issue 734 trap-launder lesson, applied
# from day one here).
COMPLETE=0
trap 'rc=$?; if [ "$COMPLETE" -ne 1 ]; then
        echo "[cross-build] ABORTED before completion (rc forced to 1)" >&2
        rc=1
      fi
      exit $rc' EXIT

die() { echo "cross-build.sh: $*" >&2; COMPLETE=1; exit 1; }
note() { echo "[cross-build] $*"; }

# ── triple helpers ─────────────────────────────────────────────────────

# arch token = everything before the first '-'
triple_arch() { printf '%s' "${1%%-*}"; }

# strip a `.<glibc>` suffix: x86_64-unknown-linux-gnu.2.36 → x86_64-unknown-linux-gnu
strip_glibc() { printf '%s' "${1%%.*}"; }

# ELF-producing target OS? (non-ELF targets get an existence-only check)
triple_is_elf() {
    case "$1" in
        *linux*|*freebsd*|*netbsd*|*openbsd*|*illumos*|*android*) return 0 ;;
        *) return 1 ;;
    esac
}

# e_machine for an arch token, as the little-endian od -tx1 byte string the
# verifier compares against — "" when unknown (magic-only check then).
machine_bytes_for_arch() {
    case "$1" in
        x86_64)              printf '3e00' ;; # EM_X86_64 = 62
        aarch64)             printf 'b700' ;; # EM_AARCH64 = 183
        arm|armv7)           printf '2800' ;; # EM_ARM = 40
        i386|i486|i586|i686) printf '0300' ;; # EM_386 = 3
        riscv64)             printf 'f300' ;; # EM_RISCV = 243
        *)                   printf '' ;;
    esac
}

# ── product verification (the re-check) ────────────────────────────────
# verify_bin <path> <triple> → 0 ok; 1 not-ok (detail on stderr). Mirrors
# `ensure_container_elf` byte-for-byte in intent: magic first, then
# e_machine at bytes 18-19 against the triple's arch.
verify_bin() {
    v_path="$1"; v_triple="$2"
    if [ ! -f "$v_path" ]; then
        echo "verify: expected binary missing: $v_path (nothing built at --expect)" >&2
        return 1
    fi
    v_clean="$(strip_glibc "$v_triple")"
    if ! triple_is_elf "$v_clean"; then
        note "OK (non-ELF target triple — existence check only): $v_path"
        return 0
    fi
    v_magic="$(dd if="$v_path" bs=1 count=4 2>/dev/null | od -An -tx1 | tr -d ' \n')"
    if [ "$v_magic" != "7f454c46" ]; then
        echo "verify: not an ELF: $v_path (first bytes: ${v_magic:-<empty>}) — a container would build and die at start ('exec format error'; Issue 008)" >&2
        return 1
    fi
    v_mach="$(dd if="$v_path" bs=1 skip=18 count=2 2>/dev/null | od -An -tx1 | tr -d ' \n')"
    v_want="$(machine_bytes_for_arch "$(triple_arch "$v_clean")")"
    if [ -n "$v_want" ] && [ "$v_mach" != "$v_want" ]; then
        echo "verify: ELF e_machine mismatch: $v_path is ${v_mach} but triple ${v_triple} wants ${v_want} — wrong-arch binary (Issue 008's cf-container class)" >&2
        return 1
    fi
    if [ -z "$v_want" ]; then
        note "WARN: arch token unknown to the verifier — magic verified, e_machine unchecked: $v_path"
    else
        note "OK: $v_path (ELF magic + e_machine $v_mach match $v_clean)"
    fi
    return 0
}

# ── self-test ──────────────────────────────────────────────────────────
selftest() {
    tmp="$(mktemp -d "${TMPDIR:-/tmp}/cross-build-selftest.XXXXXX")"
    fails=0; arms=0
    ok()   { arms=$((arms+1)); }
    bad()  { arms=$((arms+1)); fails=$((fails+1)); echo "selftest FAIL: $*" >&2; }
    expect_rc0() { ok_name="$1"; shift; if "$@" >/dev/null 2>&1; then ok; else bad "$ok_name (wanted rc 0)"; fi; }
    expect_rc1() { ok_name="$1"; shift; if "$@" >/dev/null 2>&1; then bad "$ok_name (wanted rc 1)"; else ok; fi; }

    # fixture binaries — a 20-byte header is all the verifier reads
    { printf '\177ELF'; head -c 14 /dev/zero; printf '\076\000'; } > "$tmp/x86_64.elf"  # e_machine 0x003E
    { printf '\177ELF'; head -c 14 /dev/zero; printf '\267\000'; } > "$tmp/aarch64.elf" # e_machine 0x00B7
    { printf '\377\376\000'; }                                      > "$tmp/machO.bin"  # fat/Mach-O-ish magic
    { printf '\177E'; }                                             > "$tmp/trunc.bin"
    { printf '#!/bin/sh\n'; }                                       > "$tmp/text.sh"

    X="x86_64-unknown-linux-gnu"
    A="aarch64-unknown-linux-gnu"

    expect_rc0 "x86_64 elf passes x86_64 triple"   verify_bin "$tmp/x86_64.elf" "$X"
    expect_rc0 "aarch64 elf passes aarch64 triple" verify_bin "$tmp/aarch64.elf" "$A"
    expect_rc1 "mach-O refuses against linux"      verify_bin "$tmp/machO.bin" "$X"
    expect_rc1 "text refuses against linux"        verify_bin "$tmp/text.sh" "$X"
    expect_rc1 "truncated header refuses"          verify_bin "$tmp/trunc.bin" "$X"
    expect_rc1 "arch mismatch refuses (aarch64 bin as x86_64)" verify_bin "$tmp/aarch64.elf" "$X"
    expect_rc1 "arch mismatch refuses (x86_64 bin as aarch64)" verify_bin "$tmp/x86_64.elf" "$A"
    expect_rc1 "missing file refuses"              verify_bin "$tmp/nope.bin" "$X"
    expect_rc0 "non-ELF triple (darwin) is existence-only" verify_bin "$tmp/text.sh" "x86_64-apple-darwin"

    if [ "$(strip_glibc x86_64-unknown-linux-gnu.2.36)" = "x86_64-unknown-linux-gnu" ]; then ok; else bad "strip_glibc suffix"; fi
    if [ "$(strip_glibc aarch64-unknown-linux-musl)" = "aarch64-unknown-linux-musl" ]; then ok; else bad "strip_glibc no-suffix passthrough"; fi
    if [ "$(triple_arch x86_64-unknown-linux-gnu)" = "x86_64" ]; then ok; else bad "triple_arch x86_64"; fi
    if [ "$(triple_arch aarch64-unknown-linux-gnu)" = "aarch64" ]; then ok; else bad "triple_arch aarch64"; fi
    if [ "$(machine_bytes_for_arch x86_64)" = "3e00" ]; then ok; else bad "machine x86_64"; fi
    if [ "$(machine_bytes_for_arch aarch64)" = "b700" ]; then ok; else bad "machine aarch64"; fi
    if triple_is_elf x86_64-unknown-linux-gnu; then ok; else bad "is_elf linux"; fi
    if triple_is_elf x86_64-apple-darwin; then bad "is_elf darwin must be false"; else ok; fi

    rm -rf "$tmp"
    if [ "$fails" -gt 0 ]; then
        echo "selftest: $arms arms, $fails FAILURES" >&2
        COMPLETE=1; exit 1
    fi
    echo "selftest: $arms arms, 0 failures"
    COMPLETE=1; exit 0
}

# ── arg parse ──────────────────────────────────────────────────────────

EXPECT=""
ARGS=()
while [ $# -gt 0 ]; do
    case "$1" in
        --expect)    [ $# -ge 2 ] || die "--expect needs a value"; EXPECT="$2"; shift 2 ;;
        --self-test) selftest ;;
        --)          shift; while [ $# -gt 0 ]; do ARGS+=("$1"); shift; done ;;
        *)           die "unknown wrapper arg: '$1' (wrapper args: --expect <path>, then -- and the cargo args)" ;;
    esac
done

[ ${#ARGS[@]} -gt 0 ] || die "no cargo args after -- (need: build ... --target <triple>)"
[ "${ARGS[0]}" = "build" ] || die "this wrapper only fronts 'cargo build' (first arg: '${ARGS[0]}')"

TRIPLE=""
i=0
while [ $i -lt ${#ARGS[@]} ]; do
    a="${ARGS[$i]}"
    case "$a" in
        --target=*) TRIPLE="${a#--target=}"; break ;;
    esac
    if [ "$a" = "--target" ] && [ $((i + 1)) -lt ${#ARGS[@]} ]; then
        TRIPLE="${ARGS[$((i + 1))]}"
        break
    fi
    i=$((i + 1))
done
[ -n "$TRIPLE" ] || die "no --target <triple> in the cargo args — this wrapper exists for triple builds"
TRIPLE_CLEAN="$(strip_glibc "$TRIPLE")"

# NOTE: no `exit` in the awk — an early-exiting consumer SIGPIPEs rustc
# (141) and pipefail aborts the script (the pipefail_discard class); the
# `|| true` routes a missing rustc into the friendly die below
HOST="$(rustc -vV 2>/dev/null | awk '/^host: /{print $2}' || true)"
[ -n "$HOST" ] || die "cannot read the rustc host triple (rustc -vV)"

# ── tool ladder ────────────────────────────────────────────────────────

have_zig() { command -v zig >/dev/null 2>&1 || python3 -m ziglang --version >/dev/null 2>&1; }

TOOL="${RIIR_CROSS_TOOL:-}"
if [ -z "$TOOL" ]; then
    if [ "$TRIPLE_CLEAN" = "$HOST" ]; then
        TOOL=cargo
    elif command -v cargo-zigbuild >/dev/null 2>&1 && have_zig; then
        TOOL=zigbuild
    elif command -v cross >/dev/null 2>&1; then
        TOOL=cross
    else
        die "cross-compiling $TRIPLE_CLEAN from host $HOST needs a toolchain — install cargo-zigbuild + zig (cargo install cargo-zigbuild && brew install zig) or cross (cargo install cross; needs docker). Or set RIIR_CROSS_TOOL."
    fi
fi
case "$TOOL" in
    cargo|zigbuild|cross) ;;
    *) die "RIIR_CROSS_TOOL must be one of cargo|zigbuild|cross (got: '$TOOL')" ;;
esac

# std for the target (idempotent; absent rustup → the build itself will say)
if command -v rustup >/dev/null 2>&1; then
    rustup target add "$TRIPLE_CLEAN" >/dev/null 2>&1 || true
fi

# ── rebuild args (glibc suffix: kept for zigbuild, stripped elsewhere) ──

FINAL=()
i=0
while [ $i -lt ${#ARGS[@]} ]; do
    a="${ARGS[$i]}"
    if [ "$a" = "--target" ] && [ $((i + 1)) -lt ${#ARGS[@]} ]; then
        if [ "$TOOL" = "zigbuild" ]; then
            FINAL+=("$a" "${ARGS[$((i + 1))]}")
        else
            FINAL+=("$a" "$TRIPLE_CLEAN")
        fi
        i=$((i + 2))
    else
        case "$a" in
            --target=*)
                if [ "$TOOL" = "zigbuild" ]; then FINAL+=("$a"); else FINAL+=("--target=$TRIPLE_CLEAN"); fi ;;
            *) FINAL+=("$a") ;;
        esac
        i=$((i + 1))
    fi
done

# ── run ────────────────────────────────────────────────────────────────

note "host=$HOST target=$TRIPLE (clean: $TRIPLE_CLEAN) tool=$TOOL"
case "$TOOL" in
    cargo)    set -- cargo "${FINAL[@]}" ;;
    zigbuild) set -- cargo zigbuild "${FINAL[@]:1}" ;; # zigbuild IS the build subcommand
    cross)    set -- cross "${FINAL[@]}" ;;
esac
note "run: $*"
if ! "$@"; then
    COMPLETE=1
    exit 1
fi

# ── verify the product (the re-check) ──────────────────────────────────

if [ -n "$EXPECT" ]; then
    if ! verify_bin "$EXPECT" "$TRIPLE"; then
        COMPLETE=1
        exit 1
    fi
else
    note "no --expect given — product not verified here (the deployer's stage-time gate still re-checks container binaries)"
fi

COMPLETE=1
exit 0

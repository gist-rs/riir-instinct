#!/usr/bin/env python3
"""encoder_load_ab.py — issue 018 Update 8's follow-up: a runnable
concurrent-load A/B instrument on the LANDED Lane B implementation,
public surface only (the HTTP serve binary — the worker registry is
private by design, so the soak cell re-measures without re-deriving).

What it does, per posture (per-lane `serve-encoder` — demoted from the
promoted default by `RIIR_INSTINCT_ENCODER_SHARED=0` when the build
implies the shared worker — vs shared `serve-encoder-shared`):

  1. cargo-build the serve binary at the posture's feature set.
  2. Generate an ENC manifest (explicit head files + their .blake3
     sidecars — the boot's manifest↔file drift gate validates for real).
  3. Boot the serve binary, poll /healthz until every suite reads ready.
  4. Warm each lane once (excluded from stats; recorded separately), then
     drive N clients × M decisions round-robining the suites over real
     test states pulled from the frozen datasets pool.
  5. Read RSS (ps) before load and after load; kill the server.
  6. Compare the two postures: p50/p99 ratios, RSS ratio, and the warmup
     decisions' receipt.decision digests (a byte-level pick-parity
     witness ACROSS builds — a divergence is a posture finding and exits
     2).

It RECORDS; it does not gate. The promotion decision (serve-encoder-shared
→ serve-encoder default) is the soak's owner call; the 1.217× loaded-box
envelope from Update 8 is printed as provisional context, never a verdict.

Preflight: `../riir-reflex/scripts/bench_preflight.sh` runs first and its
PROVENANCE line lands in the record (the Issue-021 law). Default `strict`
refuses on a failed preflight; `--preflight record` downgrades to
record-and-disclose (numbers carry the load class, the soak re-measures).

Usage:
  python3 scripts/encoder_load_ab.py                       # the default A/B
  python3 scripts/encoder_load_ab.py --clients 12 --rounds 25
  python3 scripts/encoder_load_ab.py --device cpu          # CPU posture
  python3 scripts/encoder_load_ab.py --postures shared     # one arm only

Exit codes: 0 = both postures measured, parity held · 1 = a phase failed
(boot/build/load) · 2 = parity DIVERGED (a posture finding — the record is
still written) · 3 = preflight refused in strict mode.
"""

from __future__ import annotations

import argparse
import hashlib
import http.client
import json
import os
import shutil
import socket
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PREFLIGHT = REPO.parent / "riir-reflex" / "scripts" / "bench_preflight.sh"

DEFAULT_SUITES = ["sst5", "xnli_en", "ag_news"]
DEFAULT_HEADS = {
    "sst5": "t6_s0.bin",
    "xnli_en": "xnli_en_encoder_v1.bin",
    "ag_news": "ag_news_encoder_v1.bin",
}
# Update 8's loaded-box envelope (the duplicate session's Metal A/B) —
# printed as provisional context beside the ratio, never a gate.
ENVELOPE_P99 = 1.217

POSTURE_FEATURES = {
    "per-lane": "serve-encoder",
    "shared": "serve-encoder-shared",
}


def die(msg: str, code: int = 1) -> "None":
    print(f"⛔ {msg}", flush=True)
    sys.exit(code)


def sh(cmd: list[str], cwd: Path | None = None, timeout: int | None = None) -> str:
    print(f"$ {' '.join(cmd)}", flush=True)
    proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout)
    if proc.returncode != 0:
        sys.stdout.write(proc.stdout[-4000:])
        sys.stderr.write(proc.stderr[-4000:])
        die(f"command failed rc={proc.returncode}: {cmd[0]} …")
    return proc.stdout


def run_preflight(mode: str) -> str:
    """The Issue-021 law: quote the PROVENANCE line beside any latency number."""
    if mode == "skip":
        return "PROVENANCE: skipped by flag (NOT a measurement posture — record-only runs only)"
    if not PREFLIGHT.is_file():
        die(f"bench_preflight.sh not found at {PREFLIGHT} — the sibling checkout must stand beside this repo")
    proc = subprocess.run(["bash", str(PREFLIGHT)], capture_output=True, text=True)
    provenance = ""
    for line in (proc.stdout + proc.stderr).splitlines():
        if line.startswith("PROVENANCE:"):
            provenance = line.strip()
    if proc.returncode != 0:
        print(proc.stdout, proc.stderr)
        if mode == "strict":
            die(f"preflight REFUSED (rc={proc.returncode}) — the box is not fit for a latency number; "
                f"re-run when quiet, or pass --preflight record to disclose-and-continue", code=3)
        print(f"⚠ preflight REFUSED (rc={proc.returncode}) — recording under a disclosed load class "
              f"(--preflight record); the soak re-measures")
    if not provenance:
        die("preflight produced no PROVENANCE line — read its output above")
    print(f"ok preflight {provenance}")
    return provenance


def load_avg() -> float:
    try:
        return os.getloadavg()[0]
    except OSError:
        return -1.0


def build_posture(posture: str, device: str, profile: str, timeout: int) -> Path:
    features = POSTURE_FEATURES[posture]
    if device == "metal":
        features += ",riir-reflex/laya-riir-metal"
    elif device == "cuda":
        features += ",riir-reflex/laya-riir-cuda"
    sh(["cargo", "build", f"--{profile}", "--bin", "serve", "--features", features],
       cwd=REPO, timeout=timeout)
    built = REPO / "target" / profile / "serve"
    if not built.is_file():
        die(f"cargo produced no binary at {built}")
    return built


def make_manifest(suites: list[str], heads_dir: Path, out_path: Path,
                  head_overrides: dict[str, str]) -> Path:
    rows: list[str] = []
    for s in suites:
        name = head_overrides.get(s, DEFAULT_HEADS.get(s))
        if not name:
            die(f"suite {s}: no default head file known — pass --head {s}=<file.bin>")
        head = heads_dir / name
        if not head.is_file():
            die(f"head artifact {head} absent — pass --head {s}=<file.bin> (relative to {heads_dir})")
        sidecar = head.with_name(head.name + ".blake3")
        if not sidecar.is_file():
            die(f"digest sidecar {sidecar} absent — the manifest needs the mint-time blake3")
        digest = sidecar.read_text(encoding="utf-8").strip()
        if len(digest) != 64:
            die(f"sidecar {sidecar} does not carry a 64-hex digest: {digest[:20]!r}…")
        rows.append(
            f'[[vessel]]\nsuite   = "{s}"\ndigest  = "blake3:{digest}"\n'
            f'class   = "hosted_only"\nfile    = "{name}"\n'
            f'posture = {{ arm = "ENC" }}\npin_keys = []\n'
            f'budget  = {{ load = "eager", max_payload_mb = 16 }}\n'
        )
    out_path.write_text("\n".join(rows), encoding="utf-8")
    print(f"ok manifest {out_path} ({len(suites)} ENC rows)")
    return out_path


# The serialized-state ENVELOPE per suite — the reflex harness's builders
# (suites.rs) wrap every row in a keyed object and `serialize_state` renders
# it with PYTHON separators (`", "` / `": "`). The ENC lane consumes the
def state_text(suite: str, row: dict) -> str | None:
    """The suite's serialized-state envelope for one pool row, pyjson-shaped
    # (the reflex harness's builders in suites.rs: sst5 `{"text"}`, ag_news
    # `{"article"}`, xnli_en `{"premise", "hypothesis"}`; the ENC lane consumes
    # the suite's serialized-state form — a raw text string 422s at the edge).
    row fields (`text` / `premise`+`hypothesis`) differ from the ENVELOPE
    keys (`{"text"}` / `{"article"}` / the NLI pair) — extract the fields,
    then wrap."""
    r = row.get("row", {})
    if r.get("premise") is not None and r.get("hypothesis") is not None:
        envelope = {"premise": r["premise"], "hypothesis": r["hypothesis"]}
    elif r.get("text"):
        envelope = {"article": r["text"]} if suite == "ag_news" else {"text": r["text"]}
    else:
        return None
    return json.dumps(envelope, separators=(", ", ": "), ensure_ascii=False)


def load_states(datasets_dir: Path, suites: list[str], n: int) -> dict[str, list[str]]:
    """Real test rows from the frozen pool (test-000.json, first n states),
    rendered as the suite's serialized-state envelope."""
    states: dict[str, list[str]] = {}
    for s in suites:
        path = datasets_dir / s / "test-000.json"
        if not path.is_file():
            die(f"datasets pool file {path} absent — pass --datasets-dir")
        doc = json.loads(path.read_text(encoding="utf-8"))
        texts = [t for t in (state_text(s, r) for r in doc.get("rows", [])) if t]
        if len(texts) < n:
            die(f"{path} carries {len(texts)} usable rows < {n} requested")
        states[s] = texts[:n]
        print(f"ok states {s}: {n} rows from {path.name} "
              f"(len med {statistics.median(len(t) for t in states[s]):.0f} chars)")
    return states


def free_port(bind: str) -> None:
    host, port = bind.rsplit(":", 1)
    try:
        with socket.create_connection((host, int(port)), timeout=0.3):
            die(f"something already listens on {bind} — pass --bind; never share the port")
    except OSError:
        return


def request_json(bind: str, method: str, path: str, body: dict | None, timeout: float) -> tuple[int, dict]:
    host, port = bind.rsplit(":", 1)
    conn = http.client.HTTPConnection(host, int(port), timeout=timeout)
    payload = json.dumps(body) if body is not None else None
    headers = {"Content-Type": "application/json"} if payload else {}
    t0 = time.perf_counter_ns()
    conn.request(method, path, body=payload, headers=headers)
    resp = conn.getresponse()
    raw = resp.read()
    wall_us = (time.perf_counter_ns() - t0) // 1000
    conn.close()
    try:
        doc = json.loads(raw)
    except json.JSONDecodeError:
        doc = {"_raw": raw[:400].decode("utf-8", "replace")}
    return wall_us, doc  # wall_us only meaningful for POST /decide


def wait_ready(bind: str, suites: list[str], timeout_s: int) -> dict:
    t0 = time.time()
    last = {}
    while time.time() - t0 < timeout_s:
        try:
            _, doc = request_json(bind, "GET", "/healthz", None, 5.0)
            last = doc
            suites_doc = doc.get("suites", {})
            words = [suites_doc.get(s, {}).get("readiness") for s in suites]
            if all(w == "ready" for w in words):
                return doc
            if any(w == "failed" for w in words):
                failed = {s: suites_doc.get(s) for s in suites
                          if suites_doc.get(s, {}).get("readiness") == "failed"}
                die(f"lane boot FAILED: {json.dumps(failed)[:1200]}")
        except Exception:  # boot window: connect refused AND half-open reads
            pass
        time.sleep(2.0)
    die(f"lanes never reached ready within {timeout_s}s; last healthz: {json.dumps(last)[:1200]}")


def rss_mib(pid: int) -> float:
    out = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)],
                         capture_output=True, text=True)
    kib = int(out.stdout.strip() or 0)
    return kib / 1024.0


def percentile(sorted_vals: list[int], p: float) -> tuple[int, int]:
    """House percentile law: report the index AND its tail support."""
    n = len(sorted_vals)
    idx = min(int(n * p), n - 1)
    return sorted_vals[idx], n - idx


def run_load(bind: str, suites: list[str], states: dict[str, list[str]],
             clients: int, rounds: int, req_timeout: float) -> tuple[list[dict], list[dict]]:
    """Each client cycles the suites (offset by client id), one connection per
    decision (the server closes per request — the real serving shape).
    Each client's FIRST decision is a thread-start-contaminated throwaway
    (measured: the heavy cell's 4.3 s outlier sat entirely in round 0 —
    thread spawn + connect storm, both postures) — excluded from the stats,
    returned as the per-client warm rows."""
    errors: list[str] = []

    def client(c: int) -> tuple[list[dict], list[dict]]:
        warm: list[dict] = []
        rows: list[dict] = []
        # The throwaway: same shape as a measured row, never counted.
        s0 = suites[c % len(suites)]
        state0 = states[s0][(c * rounds) % len(states[s0])]
        wall_us, doc = request_json(bind, "POST", "/decide",
                                    {"suite": s0, "state": state0}, req_timeout)
        warm.append({"client": c, "suite": s0, "wall_us": wall_us,
                     "server_us": doc.get("us")})
        for r in range(rounds):
            s = suites[(c + r) % len(suites)]
            state = states[s][(c * rounds + r) % len(states[s])]
            wall_us, doc = request_json(bind, "POST", "/decide",
                                        {"suite": s, "state": state}, req_timeout)
            if doc.get("pick_index") is None or "us" not in doc:
                errors.append(f"client {c} round {r}: bad reply {json.dumps(doc)[:300]}")
                continue
            rows.append({
                "client": c, "round": r, "suite": s, "wall_us": wall_us,
                "server_us": doc["us"], "pick_index": doc["pick_index"],
                "decision_b3": doc.get("receipt", {}).get("decision"),
                "lane_load": doc.get("lane_load"),
            })
        return rows, warm

    import threading
    results: list = [None] * clients
    warms: list = [None] * clients

    def run_client(i: int) -> None:
        rows, warm = client(i)
        results[i] = rows
        warms[i] = warm

    t0 = time.time()
    threads = [threading.Thread(target=run_client, args=(i,)) for i in range(clients)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    wall_s = time.time() - t0
    if errors:
        die(f"{len(errors)} load errors, first: {errors[0]}")
    flat = [row for chunk in results for row in chunk]
    flat_warm = [row for chunk in warms for row in chunk]
    print(f"ok load {len(flat)} decisions in {wall_s:.1f}s "
          f"({sum(r['wall_us'] for r in flat) / 1e6:.1f}s client-cumulative; "
          f"{len(flat_warm)} per-client throwaways excluded)")
    return flat, flat_warm


def warm_probes(bind: str, suites: list[str], states: dict[str, list[str]],
                req_timeout: float) -> list[dict]:
    probes = []
    for s in suites:
        wall_us, doc = request_json(bind, "POST", "/decide",
                                    {"suite": s, "state": states[s][0]}, req_timeout)
        if doc.get("pick_index") is None:
            die(f"warmup probe {s}: bad reply {json.dumps(doc)[:300]}")
        probes.append({"suite": s, "wall_us": wall_us, "server_us": doc["us"],
                       "pick_index": doc["pick_index"],
                       "decision_b3": doc.get("receipt", {}).get("decision")})
        print(f"ok warm {s}: pick {doc['pick']!r} server {doc['us']}µs wall {wall_us}µs")
    return probes


def stop_server(proc: subprocess.Popen, log_path: Path) -> None:
    if proc.poll() is None:
        proc.terminate()
        try:
            proc.wait(timeout=20)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=10)
    tail = Path(log_path).read_text(encoding="utf-8", errors="replace").splitlines()[-8:]
    print("server log tail:\n  " + "\n  ".join(tail))


def posture_fingerprint(bind: str) -> dict:
    _, doc = request_json(bind, "GET", "/healthz", None, 5.0)
    return {"build": doc.get("build"), "features": doc.get("features"),
            "version": doc.get("version")}


def describe(vals: list[int]) -> dict:
    vals_sorted = sorted(vals)
    p50, tail50 = percentile(vals_sorted, 0.50)
    p99, tail99 = percentile(vals_sorted, 0.99)
    return {
        "n": len(vals), "mean_us": round(statistics.mean(vals), 1),
        "p50_us": p50, "p99_us": p99, "max_us": vals_sorted[-1],
        "p99_tail_support": tail99,
    }


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--postures", default="per-lane,shared")
    ap.add_argument("--suites", default=",".join(DEFAULT_SUITES))
    ap.add_argument("--clients", type=int, default=6)
    ap.add_argument("--rounds", type=int, default=12)
    ap.add_argument("--states-per-suite", type=int, default=8)
    ap.add_argument("--device", choices=["metal", "cpu", "cuda"], default="metal")
    ap.add_argument("--profile", choices=["release", "debug"], default="release")
    ap.add_argument("--bind", default="127.0.0.1:18091")
    ap.add_argument("--ready-timeout", type=int, default=900)
    ap.add_argument("--build-timeout", type=int, default=2700)
    ap.add_argument("--request-timeout", type=float, default=120.0)
    ap.add_argument("--heads-dir", default=str(REPO.parent / "riir-train" / ".raw" / "t599"))
    ap.add_argument("--datasets-dir", default=str(REPO.parent / "riir-reflex" / ".raw" / "datasets_t20k"))
    ap.add_argument("--head", action="append", default=[],
                    help="suite=path override, e.g. --head sst5=t6_s0.bin")
    ap.add_argument("--out", default=str(REPO / ".benchmarks" / "0054_encoder_load_ab"))
    ap.add_argument("--preflight", choices=["strict", "record", "skip"], default="strict")
    ap.add_argument("--skip-build", action="store_true",
                    help="reuse target/<profile>/serve as-is (posture = whatever is built — "
                    "the fingerprint discloses it; only valid with --postures shared,per-lane "
                    "re-runs where you manage the build yourself)")
    args = ap.parse_args()

    postures = [p.strip() for p in args.postures.split(",") if p.strip()]
    for p in postures:
        if p not in POSTURE_FEATURES:
            die(f"unknown posture {p!r} — known: {list(POSTURE_FEATURES)}")
    if args.skip_build and len(postures) != 1:
        die("--skip-build measures ONE posture per invocation (the binary is single-posture)")
    suites = [s.strip() for s in args.suites.split(",") if s.strip()]
    head_overrides = {}
    for row in args.head:
        if "=" not in row:
            die(f"--head expects suite=file, got {row!r}")
        k, v = row.split("=", 1)
        head_overrides[k] = v

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    heads_dir = Path(args.heads_dir)
    datasets_dir = Path(args.datasets_dir)

    print(f"== encoder_load_ab: postures={postures} suites={suites} "
          f"clients={args.clients} rounds={args.rounds} device={args.device} "
          f"profile={args.profile} bind={args.bind}")
    provenance = run_preflight(args.preflight)
    provenance += f" load_at_launch={load_avg():.2f}"

    free_port(args.bind)
    manifest = make_manifest(suites, heads_dir, out / "arsenal_enc.toml", head_overrides)
    states = load_states(datasets_dir, suites, args.states_per_suite)

    runs: dict[str, dict] = {}
    for posture in postures:
        print(f"\n== posture {posture} ==")
        if args.skip_build:
            binary = REPO / "target" / args.profile / "serve"
            if not binary.is_file():
                die(f"--skip-build: no binary at {binary}")
        else:
            binary = build_posture(posture, args.device, args.profile, args.build_timeout)
        bin_sha = hashlib.sha256(binary.read_bytes()).hexdigest()

        state_dir = out / f"state-{posture}"
        state_dir.mkdir(exist_ok=True)
        log_path = out / f"serve-{posture}.log"
        log = log_path.open("w", encoding="utf-8")
        env = dict(os.environ)
        env.update({
            "INSTINCT_ARSENAL": str(manifest),
            "INSTINCT_WINNERS_DIR": str(heads_dir),
            "INSTINCT_DATASETS_DIR": str(datasets_dir),
            "INSTINCT_STATE_DIR": str(state_dir),
            "LAYA_DEVICE": args.device,
        })
        if posture == "per-lane":
            # The promotion demote switch (issue 018): pre-promotion this is
            # a no-op (the per-lane build keys by lane regardless); once
            # `serve-encoder` implies the shared worker, the env is what
            # bit-restores the per-lane keys for this arm (byte-identical
            # serving decisions — the 0050/0054 parity law).
            env["RIIR_INSTINCT_ENCODER_SHARED"] = "0"
        print(f"$ {binary} --bind {args.bind} --suites {','.join(suites)} "
              f"(log {log_path})")
        proc = subprocess.Popen([str(binary), "--bind", args.bind,
                                 "--suites", ",".join(suites)],
                                stdout=log, stderr=log, env=env)
        try:
            healthz = wait_ready(args.bind, suites, args.ready_timeout)
            fp = posture_fingerprint(args.bind)
            rss_boot = rss_mib(proc.pid)
            probes = warm_probes(args.bind, suites, states, args.request_timeout)
            rows, client_warm = run_load(args.bind, suites, states,
                                         args.clients, args.rounds, args.request_timeout)
            time.sleep(2.0)
            rss_samples = []
            for _ in range(3):
                time.sleep(2.0)
                rss_samples.append(rss_mib(proc.pid))
            rss_load = statistics.median(rss_samples)
            runs[posture] = {
                "binary_sha256": bin_sha,
                "fingerprint": fp,
                "healthz_suites": healthz.get("suites"),
                "rss_boot_mib": round(rss_boot, 1),
                "rss_load_mib": round(rss_load, 1),
                "warmup": probes,
                "client_warm": client_warm,
                "decisions": rows,
                "summary": describe([r["wall_us"] for r in rows]),
                "server_wall_s": None,
            }
        finally:
            stop_server(proc, log_path)
            log.close()
        time.sleep(3.0)  # let the port free before the next posture

    # ── the A/B ──
    record: dict = {
        "instrument": "scripts/encoder_load_ab.py",
        "date": time.strftime("%Y-%m-%d %H:%M:%S %z"),
        "provenance": provenance,
        "postures": postures,
        "suites": suites,
        "clients": args.clients, "rounds": args.rounds,
        "n": args.clients * args.rounds,
        "device": args.device, "profile": args.profile,
        "states_per_suite": args.states_per_suite,
        "states_source": str(datasets_dir),
        "heads_dir": str(heads_dir),
        "envelope_p99_provisional": ENVELOPE_P99,
        "runs": runs,
    }

    parity_ok = True
    if len(postures) == 2:
        a, b = postures
        pa = {p["suite"]: p["decision_b3"] for p in runs[a]["warmup"]}
        pb = {p["suite"]: p["decision_b3"] for p in runs[b]["warmup"]}
        diverged = [s for s in suites if pa.get(s) != pb.get(s)]
        record["decision_parity"] = {
            "witness": "warmup receipt.decision digests across postures",
            "ok": not diverged, "diverged_suites": diverged,
            "per_suite": {s: {"a": pa.get(s), "b": pb.get(s)} for s in suites},
        }
        if diverged:
            parity_ok = False

    write_results(out, record)

    print("\n== A/B ==")
    for posture in postures:
        s = runs[posture]["summary"]
        print(f"  {posture:9s} p50 {s['p50_us']:>9,} µs · p99 {s['p99_us']:>9,} µs "
              f"(tail {s['p99_tail_support']}) · mean {s['mean_us']:>10,.1f} · "
              f"RSS boot {runs[posture]['rss_boot_mib']:>7,.1f} → load {runs[posture]['rss_load_mib']:>7,.1f} MiB")
    if len(postures) == 2:
        a, b = postures
        sa, sb = runs[a]["summary"], runs[b]["summary"]
        # The ratio is BY NAME (shared / per-lane), never by list position —
        # --postures shared,per-lane must not invert the line's label.
        num = runs.get("shared", runs[b])
        den = runs.get("per-lane", runs[a])
        if "shared" not in runs or "per-lane" not in runs:
            num, den = runs[b], runs[a]
        r50 = num["summary"]["p50_us"] / den["summary"]["p50_us"]
        r99 = num["summary"]["p99_us"] / den["summary"]["p99_us"]
        rrss = den["rss_load_mib"] / num["rss_load_mib"]
        print(f"  shared/per-lane: p50 {r50:.3f}× · p99 {r99:.3f}× "
              f"(Update-8 provisional envelope {ENVELOPE_P99}×) · RSS {rrss:.2f}×")
        print(f"  decision parity across postures: "
              f"{'OK (receipt digests equal)' if parity_ok else '⛔ DIVERGED — a posture finding'}")

    if not parity_ok:
        sys.exit(2)


def write_results(out: Path, record: dict) -> None:
    (out / "results.json").write_text(
        json.dumps(record, indent=2, ensure_ascii=False), encoding="utf-8")

    postures = record["postures"]
    lines = [
        "# Encoder concurrent-load A/B — the LANDED Lane B implementation, public surface",
        "",
        f"Instrument: `scripts/encoder_load_ab.py` · {record['date']}  ",
        f"{record['provenance']}  ",
        f"Postures: {', '.join(postures)} · suites: {', '.join(record['suites'])} · "
        f"{record['clients']} clients × {record['rounds']} rounds = n {record['n']} · "
        f"device {record['device']} · profile {record['profile']} · "
        f"states: first {record['states_per_suite']} test rows per suite from the frozen pool "
        f"(`{record['states_source']}`) · heads: `{record['heads_dir']}`",
        "",
        "Warmup decisions (one per suite) are EXCLUDED from the stats and recorded separately; "
        "their `receipt.decision` digests are the cross-posture pick-parity witness.",
        "",
        "| posture | n | p50 µs | p99 µs (tail) | mean µs | max µs | RSS ready MiB | RSS steady MiB |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for posture in postures:
        s = record["runs"][posture]["summary"]
        r = record["runs"][posture]
        lines.append(
            f"| {posture} | {s['n']} | {s['p50_us']:,} | {s['p99_us']:,} ({s['p99_tail_support']}) "
            f"| {s['mean_us']:,.1f} | {s['max_us']:,} | {r['rss_boot_mib']:,.1f} | {r['rss_load_mib']:,.1f} |")
    if len(postures) == 2:
        a, b = postures
        runs = record["runs"]
        num = runs.get("shared", runs[b])
        den = runs.get("per-lane", runs[a])
        if "shared" not in runs or "per-lane" not in runs:
            num, den = runs[b], runs[a]
        r50 = num["summary"]["p50_us"] / den["summary"]["p50_us"]
        r99 = num["summary"]["p99_us"] / den["summary"]["p99_us"]
        rrss = den["rss_load_mib"] / num["rss_load_mib"]
        parity = record.get("decision_parity", {})
        lines += [
            "",
            f"**shared/per-lane:** p50 **{r50:.3f}×** · p99 **{r99:.3f}×** · RSS **{rrss:.2f}×**",
            f"",
            f"Context (NOT a gate): Update 8's provisional loaded-box p99 envelope {record['envelope_p99_provisional']}× "
            f"— the soak owns the decision cell; this instrument makes that cell re-measurable "
            f"without re-deriving.",
            "",
            f"Decision parity across postures: "
            + ("**OK** — warmup `receipt.decision` digests equal on every suite (byte-level witness across builds)"
               if parity.get("ok") else
               f"**⛔ DIVERGED** on {parity.get('diverged_suites')} — a posture finding, exit 2"),
            "",
            "## Posture fingerprints",
            "",
            "| posture | binary sha256 | build | features |",
            "|---|---|---|---|",
        ]
        for posture in postures:
            r = record["runs"][posture]
            lines.append(f"| {posture} | `{r['binary_sha256'][:16]}…` | `{r['fingerprint'].get('build')}` "
                         f"| `{json.dumps(r['fingerprint'].get('features'))}` |")
        lines += ["", "## Warmup probes (excluded from stats)", "",
                  "| posture | suite | pick | server µs | wall µs | decision b3 |", "|---|---|---|---|---|---|"]
        for posture in postures:
            for p in record["runs"][posture]["warmup"]:
                lines.append(f"| {posture} | {p['suite']} | {p['pick_index']} | {p['server_us']:,} "
                             f"| {p['wall_us']:,} | `{str(p['decision_b3'])[:16]}…` |")
    lines += [
        "",
        "## Honest scope",
        "",
        "- Client-observed wall includes connection setup (the std-only server closes per request) — "
        "identical shape on both postures; the server-side `us` fields are in `results.json`.",
        "- Each client's FIRST decision is an excluded throwaway (thread-start + connect-storm "
        "contamination — the heavy cell measured a 4.3 s outlier sitting entirely in round 0); "
        "the per-client throwaway walls are in `results.json` (`client_warm`).",
        "- `RSS ready` is read the moment every lane reports ready; `RSS steady` is the median of "
        "three reads after the load phase — eager lanes are resident from boot, so both columns "
        "are post-residency; the per-lane numbers carry 3 full checkpoint residencies, the shared "
        "numbers one (the RSS ratio is Lane B's whole point, the latency ratio is its cost).",
        "- A one-session reading on a shared box; the soak re-measures at the serving posture. "
        "Quote the PROVENANCE line, never this file's numbers alone.",
        "",
    ]
    (out / "RESULTS.md").write_text("\n".join(lines), encoding="utf-8")
    print(f"ok wrote {out/'RESULTS.md'} + {out/'results.json'}")


if __name__ == "__main__":
    main()

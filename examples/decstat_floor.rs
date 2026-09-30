//! The decstat receipt FLOOR runner (Plan 043 Phase C2): the submitter
//! and verifier roles in ONE process over the SAME booted seat, playing
//! the full lane against a kat-service deployment — receipts from ACTUAL
//! engine runs (honest by construction: the submitter role runs the
//! engine), lease, judge, verdicts — and the FLOOR print:
//!
//! - **reproduction rate** `verified / (verified + mismatch)` — D5's
//!   arming quantity: the same-release replay faithfulness.
//! - **end-to-end yield** `verified / leased` — the same number through
//!   the whole lane, skew included.
//!
//! Plus the skew breakdown (toolchain / manifest / corpus / mismatch /
//! verified + duplicate acks) and the box state (the G2 provenance law:
//! this is a RATE, but the box it was measured on is part of the claim).
//!
//! The record lands under `.benchmarks/<NNN>_decstat_receipt_floor/`
//! (`floor.json` + a reproduce-me `README.md`; the runner allocates the
//! number from `.benchmarks/.highwater` — never a hand-typed number).
//!
//! CLI / env contract:
//!
//! ```text
//! cargo run --release --features decstat --example decstat_floor [--n 40] [--rounds 8]
//!     [--limit 64] [--suites a,b] [--benchmarks-dir .benchmarks]
//!
//! INSTINCT_ACCOUNT_KEY      required — the SUBMITTER's 64-hex seed
//!                           (file path or inline hex; the consent key).
//! INSTINCT_VERIFY_KEY       required — the VERIFIER's 64-hex seed (file
//!                           path or inline hex). A distinct account: the
//!                           lane excludes the submitter from verifying
//!                           its own receipts (riir-dapps Plan 043 B3).
//! INSTINCT_KAT_SERVICE_URL  the service (default
//!                           https://devnet.ai.gist.rs — the devnet-first
//!                           ladder).
//! INSTINCT_DATASETS_DIR / INSTINCT_WINNERS_DIR /
//! INSTINCT_SYNTH_CORPUS_DIR / INSTINCT_ARSENAL — the boot env, the
//!                           serve binary's exact contract.
//! INSTINCT_MACHINE_LABEL    the receipt run-id label (default the host name).
//! ```
//!
//! Loud exits (2): a missing key, a 503 (the inert-until-armed
//! posture — printed, then refused), or zero suites booted.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use riir_instinct::decstat_verify::{BootConfig, Breakdown, Verdict, Verifier};
use riir_kat::kat_decstat_receipt_client::{
    HttpDecStatReceiptTransport, request_decstat_lease, sign_receipt, submit_decstat_verdict,
    submit_receipts,
};
use riir_kat::kat_protocol_decstat_receipt::DecStatReceiptSigned;
use riir_kat::kat_protocol_decstat_verdict::DECSTAT_ACCEPT_DUPLICATE;

/// The submitter generates receipts from this many engine decisions
/// (round-robin across the booted suites).
const DEFAULT_N: usize = 40;
/// Lease rounds before giving up on coverage.
const DEFAULT_ROUNDS: usize = 8;
/// The lease pull's size (the server caps at 64).
const DEFAULT_LIMIT: u32 = 64;
/// The submit→lease settle gap (the ingest is synchronous; this only
/// spaces the two roles' traffic).
const SETTLE_SLEEP_SECS: u64 = 2;
/// The HTTP budget for one lane call (an explicit command surface).
const LANE_TIMEOUT_SECS: u64 = 30;

struct Args {
    n: usize,
    rounds: usize,
    limit: u32,
    suites: Option<Vec<String>>,
    benchmarks_dir: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        n: DEFAULT_N,
        rounds: DEFAULT_ROUNDS,
        limit: DEFAULT_LIMIT,
        suites: None,
        benchmarks_dir: PathBuf::from(".benchmarks"),
    };
    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        let value = |name: &str| -> Result<String, String> {
            argv.get(i + 1)
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match argv[i].as_str() {
            "--n" => {
                a.n = value("--n")?.parse().map_err(|_| "--n wants a number")?;
                i += 1;
            }
            "--rounds" => {
                a.rounds = value("--rounds")?
                    .parse()
                    .map_err(|_| "--rounds wants a number")?;
                i += 1;
            }
            "--limit" => {
                a.limit = value("--limit")?
                    .parse()
                    .map_err(|_| "--limit wants a number")?;
                i += 1;
            }
            "--suites" => {
                let names: Vec<String> = value("--suites")?
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect();
                if names.is_empty() {
                    return Err("--suites named no suite".into());
                }
                a.suites = Some(names);
                i += 1;
            }
            "--benchmarks-dir" => {
                a.benchmarks_dir = value("--benchmarks-dir")?.into();
                i += 1;
            }
            other => return Err(format!("unknown arg {other}")),
        }
        i += 1;
    }
    Ok(a)
}

/// A key value is EITHER 64 inline hex chars (a dev/test runner
/// convenience) OR a file path holding the seed — the file path
/// delegates to the consent key loader (the exact same decode).
fn load_key_flexible(value: &str, what: &str) -> Result<ed25519_dalek::SigningKey, String> {
    let trimmed = value.trim();
    if trimmed.len() == 64 && trimmed.bytes().all(|b| b.is_ascii_hexdigit()) {
        let mut seed = [0u8; 32];
        for (i, byte) in seed.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&trimmed[i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("{what}: bad hex at byte {i}: {e}"))?;
        }
        return Ok(ed25519_dalek::SigningKey::from_bytes(&seed));
    }
    riir_instinct::decstat::load_signing_key(trimmed).map_err(|e| format!("{what}: {e}"))
}

fn env_key(name: &str, what: &str) -> Result<ed25519_dalek::SigningKey, String> {
    match std::env::var(name) {
        Ok(v) if !v.trim().is_empty() => load_key_flexible(&v, what),
        _ => Err(format!(
            "refusing: {name} is not set — {what} must sign (set it to a 64-hex seed file \
             path or an inline 64-hex seed; a missing key never runs the lane)"
        )),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("⛔ decstat_floor: {e}");
        std::process::exit(2);
    }
}

struct Floor {
    breakdown: Breakdown,
    duplicate_acks: u64,
    leased_distinct: usize,
    rounds_run: usize,
    receipts_generated: usize,
    stored: u32,
    submit_duplicate: u32,
    submit_refused: u32,
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let submit_key = env_key("INSTINCT_ACCOUNT_KEY", "the submitter (INSTINCT_ACCOUNT_KEY)")?;
    let verify_key = env_key("INSTINCT_VERIFY_KEY", "the verifier (INSTINCT_VERIFY_KEY)")?;
    let service_url = std::env::var("INSTINCT_KAT_SERVICE_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| riir_instinct::decstat::DEFAULT_SERVICE_URL.into());
    let transport = HttpDecStatReceiptTransport::new(&service_url, LANE_TIMEOUT_SECS);

    // ── the boot env: the serve binary's exact contract ─────────────
    let datasets_dir = std::env::var("INSTINCT_DATASETS_DIR")
        .unwrap_or_else(|_| "../riir-reflex/.raw/datasets_t20k".into());
    let winners_dir = std::env::var("INSTINCT_WINNERS_DIR")
        .unwrap_or_else(|_| "../riir-train/data/instinct_specialists".into());
    let synth_corpus_dir = std::env::var("INSTINCT_SYNTH_CORPUS_DIR").ok();
    let arsenal_path = std::env::var("INSTINCT_ARSENAL").ok();
    let loaded_manifest: Option<String> = match &arsenal_path {
        Some(p) => Some(std::fs::read_to_string(p).map_err(|e| format!("read arsenal manifest {p}: {e}"))?),
        None => None,
    };
    let manifest = match &loaded_manifest {
        Some(text) => riir_instinct::arsenal::ArsenalManifest::parse(text)?,
        None => riir_instinct::arsenal::ArsenalManifest::embedded_default()
            .map_err(|e| format!("embedded arsenal manifest: {e}"))?,
    };

    // ── boot the seat engine once (both roles share it) ─────────────
    eprintln!(
        "[decstat_floor] booting the seat engine (datasets {datasets_dir}, winners {winners_dir}, \
         arsenal {})",
        arsenal_path.as_deref().unwrap_or("embedded default"),
    );
    let mut verifier = Verifier::boot(
        BootConfig {
            datasets_dir: datasets_dir.clone(),
            winners_dir: winners_dir.clone(),
            synth_corpus_dir: synth_corpus_dir.clone(),
            manifest,
            suite_filter: args.suites.clone(),
        },
        loaded_manifest.clone(),
    )
    .map_err(|e| format!("boot refused: {e}"))?;
    for (name, reason) in &verifier.skipped_suites {
        eprintln!("[decstat_floor] suite {name} skipped at boot: {reason}");
    }
    let build_fp_hex = verifier.build_fp_hex();
    let manifest_fp_hex = verifier.manifest_fp_hex();
    eprintln!(
        "[decstat_floor] booted {} suite(s) · build_fp {build_fp_hex} · manifest_fp {manifest_fp_hex}",
        verifier.suites().len()
    );

    // ── the SUBMITTER role: receipts from ACTUAL engine runs ────────
    let machine = std::env::var("INSTINCT_MACHINE_LABEL").unwrap_or_else(|_| host_name());
    let run_id = riir_kat::kat_fixstat_client::fresh_run_id(&machine);
    let build_fp =
        riir_instinct::receipt::fp8_from_hex(&build_fp_hex).expect("build fingerprint is 16 hex");
    let manifest_fp = riir_instinct::receipt::manifest_fingerprint(loaded_manifest.as_deref());

    // Snapshot the walk plan (name + case count per suite) so the
    // round-robin never holds a borrow across `drive_case`'s &mut.
    let plan: Vec<(&'static str, usize)> = verifier
        .suites()
        .iter()
        .map(|h| (h.name, h.n_cases))
        .collect();
    let mut receipts: Vec<DecStatReceiptSigned> = Vec::new();
    let mut cursors = vec![0usize; plan.len()];
    let mut made = 0usize;
    while made < args.n {
        let mut progressed = false;
        for (si, (name, n_cases)) in plan.iter().enumerate() {
            if made >= args.n {
                break;
            }
            let ci = cursors[si];
            if ci >= *n_cases {
                continue; // this suite is exhausted
            }
            cursors[si] = ci + 1;
            progressed = true;
            let Ok(decisions) = verifier.drive_case(si, ci) else {
                eprintln!("[decstat_floor] {name} case {ci}: decide refused — skipped");
                continue;
            };
            for d in &decisions {
                if made >= args.n {
                    break;
                }
                let input_hash =
                    riir_instinct::receipt::input_blake3(verifier.state_str(si, ci), &d.options);
                let decision_hash = riir_instinct::receipt::decision_blake3(d);
                receipts.push(sign_receipt(
                    &submit_key,
                    &run_id,
                    name,
                    &riir_instinct::receipt::hash32_from_hex(&input_hash)
                        .expect("input hash is 64 hex"),
                    &riir_instinct::receipt::hash32_from_hex(&decision_hash)
                        .expect("decision hash is 64 hex"),
                    &build_fp,
                    &manifest_fp,
                ));
                made += 1;
            }
        }
        if !progressed {
            break; // every suite exhausted below N
        }
    }
    if receipts.is_empty() {
        return Err(
            "no receipts generated (every suite's decide refused) — nothing to measure".into(),
        );
    }
    eprintln!(
        "[decstat_floor] submitter: signing {} receipt(s) (run_id {}) and pushing to {service_url}",
        receipts.len(),
        hex_prefix(&run_id),
    );
    let submit_ack = submit_receipts(&transport, &submit_key, &receipts).map_err(|e| {
        format!(
            "submit refused: {e}{}",
            if e.contains("503") {
                " — the decstat receipt lane is INERT on this deployment (unconfigured / \
                 cut=0): the floor cannot be measured against it (the recorded posture)"
            } else {
                ""
            }
        )
    })?;
    eprintln!(
        "[decstat_floor] submit ack: stored {} · duplicate {} · refused {}",
        submit_ack.stored, submit_ack.duplicate, submit_ack.refused
    );
    std::thread::sleep(std::time::Duration::from_secs(SETTLE_SLEEP_SECS));

    // ── the VERIFIER role: lease → judge → verdict, until coverage ──
    let mut breakdown = Breakdown::default();
    let mut duplicate_acks = 0u64;
    let mut leased_distinct: HashSet<String> = HashSet::new();
    let mut judged: HashMap<String, Verdict> = HashMap::new();
    let mut rounds_run = 0usize;
    for _round in 0..args.rounds {
        if leased_distinct.len() >= args.n {
            break;
        }
        let lease = request_decstat_lease(&transport, &verify_key, args.limit).map_err(|e| {
            format!(
                "lease refused: {e}{}",
                if e.contains("503") {
                    " — the lane is INERT on this deployment (the recorded posture)"
                } else {
                    ""
                }
            )
        })?;
        rounds_run += 1;
        if lease.items.is_empty() {
            eprintln!(
                "[decstat_floor] round {rounds_run}: the lease came back EMPTY ({} distinct \
                 inputs covered) — stopping (the pool has nothing leasable for this verifier)",
                leased_distinct.len()
            );
            break;
        }
        eprintln!(
            "[decstat_floor] round {rounds_run}: leased {} item(s) (coverage {}/{})",
            lease.items.len(),
            leased_distinct.len(),
            args.n
        );
        for item in &lease.items {
            let key = item.input_hash_hex.to_ascii_lowercase();
            let verdict = match judged.get(&key) {
                // Already judged this run: push the CACHED verdict (the
                // server answers `duplicate` — counted, never re-judged).
                Some(v) => {
                    breakdown.record(v);
                    v.clone()
                }
                None => {
                    let v = verifier.judge(item);
                    breakdown.record(&v);
                    if !v.is_skipped() {
                        judged.insert(key.clone(), v.clone());
                    }
                    v
                }
            };
            if verdict.is_skipped() {
                continue; // loud skip — no verdict crosses the wire
            }
            let input_hash = riir_instinct::receipt::hash32_from_hex(&item.input_hash_hex)
                .unwrap_or([0u8; 32]);
            match submit_decstat_verdict(
                &transport,
                &verify_key,
                &input_hash,
                verdict.wire_outcome().unwrap_or(riir_kat::kat_protocol_decstat_verdict::DECSTAT_VERDICT_CORPUS_SKEW),
                &verdict.computed_hash(),
                &build_fp,
            ) {
                Ok(a) => {
                    if a.accept == DECSTAT_ACCEPT_DUPLICATE {
                        duplicate_acks += 1;
                    }
                    eprintln!("[decstat_floor] verdict: {} ({})", a.accept, item.suite);
                }
                Err(e) => eprintln!("[decstat_floor] verdict push failed: {e}"),
            }
            if judged.contains_key(&key) {
                leased_distinct.insert(key);
            }
        }
    }

    // ── the FLOOR print + the record ─────────────────────────────────
    let reproduction = breakdown.reproduction_rate();
    let yield_e2e = if leased_distinct.is_empty() {
        0.0
    } else {
        breakdown.verified as f64 / leased_distinct.len() as f64
    };
    println!();
    println!("════════════════════════════════════════════════════════");
    println!("decstat receipt FLOOR (Plan 043 C2 — the D5 arming quantity)");
    println!("  reproduction rate  verified/(verified+mismatch) = {reproduction:.4}");
    println!(
        "    ({} verified / {} re-ran)",
        breakdown.verified,
        breakdown.verified + breakdown.mismatch
    );
    println!("  end-to-end yield   verified/leased              = {yield_e2e:.4}");
    println!(
        "    ({} verified / {} distinct inputs leased)",
        breakdown.verified,
        leased_distinct.len()
    );
    println!(
        "  breakdown: verified {} · mismatch {} · corpus_skew {} · toolchain_skipped {} · \
         manifest_skipped {} · duplicate_acks {duplicate_acks}",
        breakdown.verified,
        breakdown.mismatch,
        breakdown.corpus_skew,
        breakdown.toolchain_skipped,
        breakdown.manifest_skipped
    );
    println!(
        "  rounds {rounds_run} · receipts {} (stored {} · duplicate {} · refused {})",
        receipts.len(),
        submit_ack.stored,
        submit_ack.duplicate,
        submit_ack.refused
    );
    println!(
        "  box: host {} · {} · rustc {} · features {:?}",
        host_name(),
        cargo_version(),
        riir_instinct::receipt::RUSTC_RELEASE,
        riir_instinct::receipt::COMPILED_FEATURES,
    );
    println!(
        "  box: datasets {datasets_dir} · winners {winners_dir} · synth {:?} · manifest {}…",
        synth_corpus_dir,
        manifest_digest(loaded_manifest.as_deref()),
    );
    println!("  service {service_url}");
    println!("════════════════════════════════════════════════════════");

    write_record(
        &args,
        &RecordCtx {
            service_url: &service_url,
            datasets_dir: &datasets_dir,
            winners_dir: &winners_dir,
            synth_dir: synth_corpus_dir.as_deref(),
            loaded_manifest: loaded_manifest.as_deref(),
        },
        &Floor {
            breakdown,
            duplicate_acks,
            leased_distinct: leased_distinct.len(),
            rounds_run,
            receipts_generated: receipts.len(),
            stored: submit_ack.stored,
            submit_duplicate: submit_ack.duplicate,
            submit_refused: submit_ack.refused,
        },
        &verifier,
    )?;
    Ok(())
}

fn manifest_digest(loaded: Option<&str>) -> String {
    let full = match loaded {
        Some(text) => riir_instinct::arsenal::ArsenalManifest::digest_of(text),
        None => riir_instinct::arsenal::ArsenalManifest::embedded_manifest_digest(),
    };
    full.chars().take(16).collect()
}

fn hex_prefix(bytes: &[u8]) -> String {
    bytes.iter().take(4).map(|b| format!("{b:02x}")).collect()
}

fn host_name() -> String {
    std::process::Command::new("hostname")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown-host".into())
}

fn cargo_version() -> String {
    std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("rustc {}", riir_instinct::receipt::RUSTC_RELEASE))
}

/// The run's environment the record discloses (the box state + the
/// service posture) — one struct so the writer's arity stays small.
struct RecordCtx<'a> {
    service_url: &'a str,
    datasets_dir: &'a str,
    winners_dir: &'a str,
    synth_dir: Option<&'a str>,
    loaded_manifest: Option<&'a str>,
}

/// The record write: read `.benchmarks/.highwater`, allocate value+1,
/// write it back — in the same run that writes the record (the
/// numbering discipline; never a hand-typed number).
fn write_record(
    args: &Args,
    ctx: &RecordCtx<'_>,
    floor: &Floor,
    verifier: &Verifier,
) -> Result<(), String> {
    let service_url = ctx.service_url;
    let datasets_dir = ctx.datasets_dir;
    let winners_dir = ctx.winners_dir;
    let synth_dir = ctx.synth_dir;
    let loaded_manifest = ctx.loaded_manifest;
    let hw_path = args.benchmarks_dir.join(".highwater");
    let current: u32 = std::fs::read_to_string(&hw_path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let next = current + 1;
    let dir = args.benchmarks_dir.join(format!("{next:04}_decstat_receipt_floor"));
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
    std::fs::write(&hw_path, format!("{next:04}"))
        .map_err(|e| format!("bump {}: {e}", hw_path.display()))?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let record = serde_json::json!({
        "record": format!("{next:04}_decstat_receipt_floor"),
        "ts_unix": ts,
        "service_url": service_url,
        "params": {
            "n_target": args.n,
            "rounds_max": args.rounds,
            "rounds_run": floor.rounds_run,
            "lease_limit": args.limit,
            "suite_filter": args.suites,
        },
        "submit": {
            "receipts_generated": floor.receipts_generated,
            "stored": floor.stored,
            "duplicate": floor.submit_duplicate,
            "refused": floor.submit_refused,
        },
        "breakdown": {
            "verified": floor.breakdown.verified,
            "mismatch": floor.breakdown.mismatch,
            "corpus_skew": floor.breakdown.corpus_skew,
            "toolchain_skipped": floor.breakdown.toolchain_skipped,
            "manifest_skipped": floor.breakdown.manifest_skipped,
            "duplicate_acks": floor.duplicate_acks,
            "leased_distinct": floor.leased_distinct,
        },
        "rates": {
            "reproduction_rate_verified_over_rerun": floor.breakdown.reproduction_rate(),
            "end_to_end_yield_verified_over_leased": if floor.leased_distinct == 0 { 0.0 }
                else { floor.breakdown.verified as f64 / floor.leased_distinct as f64 },
        },
        "suites_booted": verifier
            .suites()
            .iter()
            .map(|h| serde_json::json!({
                "name": h.name,
                "cases": h.n_cases,
                "arm": h.arm_name(),
            }))
            .collect::<Vec<_>>(),
        "suites_skipped": verifier.skipped_suites,
        "box": {
            "host": host_name(),
            "cargo": cargo_version(),
            "rustc_release": riir_instinct::receipt::RUSTC_RELEASE,
            "rustc_commit": riir_instinct::receipt::RUSTC_COMMIT,
            "rustc_host": riir_instinct::receipt::RUSTC_HOST,
            "features": riir_instinct::receipt::COMPILED_FEATURES,
            "package_version": env!("CARGO_PKG_VERSION"),
            "datasets_dir": datasets_dir,
            "winners_dir": winners_dir,
            "synth_corpus_dir": synth_dir,
            "manifest_digest_prefix": manifest_digest(loaded_manifest),
            "build_fp_hex": riir_instinct::receipt::fingerprint(),
            "manifest_fp_hex": riir_instinct::receipt::manifest_fingerprint_hex(loaded_manifest),
        },
    });
    std::fs::write(
        dir.join("floor.json"),
        serde_json::to_string_pretty(&record).unwrap_or_default(),
    )
    .map_err(|e| format!("write floor.json: {e}"))?;
    std::fs::write(
        dir.join("README.md"),
        record_readme(&dir, service_url, args),
    )
    .map_err(|e| format!("write README.md: {e}"))?;
    eprintln!("[decstat_floor] record written: {}", dir.display());
    Ok(())
}

/// The reproduce-me README — how the floor was produced, never a typed
/// number (the runner's `floor.json` is the record of record).
fn record_readme(dir: &std::path::Path, service_url: &str, args: &Args) -> String {
    let name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("record")
        .to_string();
    let (n, rounds) = (args.n, args.rounds);
    format!(
        r#"# decstat receipt FLOOR ({name})

The measured same-release reproduction rate of the decstat receipt
reward class (riir-dapps Plan 043 Phase C / D5): the number that gates
arming — a RATE, never a latency claim. The numbers live in
[`floor.json`](floor.json) (the runner wrote them; nothing here is
hand-typed).

## What was measured

The submitter and verifier roles, ONE process over the SAME booted seat
engine (`--example decstat_floor --features decstat`):

1. **submit** — N engine decisions sampled round-robin across the
   manifest's booted suites; each receipt's input/decision hashes are
   computed from the ACTUAL decide runs (honest by construction), signed
   with the submitter key, pushed to `{service_url}`.
2. **lease + judge + verdict** — the verifier key leases items, judges
   each per the C1 loop (toolchain screen → manifest screen → corpus
   resolution → re-run → hash compare), pushes verdicts, until ≥ N
   distinct inputs are covered or the round cap hits.

The FLOOR quantities: `reproduction_rate = verified/(verified+mismatch)`
(the D5 arming quantity) and `end_to_end_yield = verified/leased`, plus
the skew breakdown (toolchain / manifest / corpus / mismatch / verified
/ duplicate acks) and the box state (the G2 provenance law).

## Reproduce

```sh
INSTINCT_ACCOUNT_KEY=<64-hex seed file> \
INSTINCT_VERIFY_KEY=<64-hex seed file — a DIFFERENT account> \
INSTINCT_KAT_SERVICE_URL={service_url} \
cargo run --release --features decstat --example decstat_floor -- --n {n} --rounds {rounds}
```

Boot env (the serve binary's contract): `INSTINCT_DATASETS_DIR`,
`INSTINCT_WINNERS_DIR`, `INSTINCT_SYNTH_CORPUS_DIR`, `INSTINCT_ARSENAL`.
Flags: `--n <decisions>` · `--rounds <max>` · `--limit <lease size>` ·
`--suites a,b` · `--benchmarks-dir <dir>`.

Refusals (exit 2): a missing key, a 503 (the lane is inert until the
reward class arms — the recorded posture), or zero suites booted.
"#
    )
}

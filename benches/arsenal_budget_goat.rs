//! `arsenal_budget_goat` — Proposal 001 T8, the arsenal budget legs' GOAT
//! gate (opt-in: `cargo bench --bench arsenal_budget_goat --features
//! arsenal_goat`).
//!
//! Four legs, the proposal's own words made measurable:
//!
//! 1. **Init p99 under a full manifest (lazy bounds held)** — the
//!    embedded six-row manifest parses and validates (incl. the six
//!    artifact digest reads + BLAKE3 hashes) under a pre-declared bound,
//!    and the lazy slot registry constructs in µs — boot NEVER pays the
//!    lane loads. The full six-lane eager boot is timed and DISCLOSED
//!    (per-lane + total): that is the cost the lazy posture defers, not
//!    a gate.
//! 2. **Routing ns–µs at k=dozens** — the hoarding gate's per-admission
//!    check at the production shape (k=5 loaded, the real six centroids)
//!    and at k=64 (deterministic synthetic directions) stays µs-class
//!    (law A3: coarse vessels, cheap routing, never on the decision
//!    path). The decision-path slot read (lock + Ready match) is
//!    ns-class.
//! 3. **Swap atomicity under concurrent decisions** — REAL servers: 3
//!    reader threads decide() under the slot lock while a swapper
//!    installs strict epoch advances built the production way (fresh
//!    seat + artifact bytes → whole-snapshot install). Every observed
//!    decision must be whole and monotone-epoch; zero torn reads is the
//!    gate; swap install latency is disclosed.
//! 4. **Armed-off = bit-identical** — the kill-switch admits the exact
//!    literal only (re-pinned here); the armed gate admits all six real
//!    centroids (the default set passes — measured, twice, bit-identical
//!    verdicts); disarmed, the caller admits unconditionally — the
//!    serving outcome set (all six up) is identical either way.
//!
//! Data: `INSTINCT_DATASETS_DIR` (default
//! `../riir-reflex/.raw/datasets_t20k`) + `INSTINCT_WINNERS_DIR` (default
//! `../riir-train/data/instinct_specialists`). Absent data REFUSES (exit
//! 1) naming the env — never a green zero.

use std::hint::black_box;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use katgpt_core::set_admission::SetAdmissionConfig;
use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::arsenal_ops::{
    EpochTag, HoardReport, InstallOutcome, LaneSlot, LaneState, hoard_check, hoard_gate_armed_for,
};
use riir_instinct::server::AnySuiteServer;

// ── pre-declared bars ───────────────────────────────────────────────────

/// Init: the full-manifest parse+validate (six digest reads + hashes)
/// must hold this p99 — boot never pays the lane loads.
const BAR_VALIDATE_P99: Duration = Duration::from_millis(1_000);
/// Init: the lazy slot registry (six slots) is bookkeeping, not loading.
const BAR_REGISTRY_P99: Duration = Duration::from_millis(1);
/// Routing: the hoard check is off the decision path and must stay
/// µs-class at any k (law A3).
const HOARD_BAR_US: u128 = 100;
/// The decision-path slot read (lock + Ready match, no decide) is
/// ns-class.
const SLOT_READ_BAR_US: u128 = 10;
/// Swap hammer liveness: the readers must actually serve during churn.
const MIN_HAMMER_DECISIONS: usize = 1_000;

const HAMMER_SWAPS: u64 = 12;
const HAMMER_READERS: usize = 3;
const VALIDATE_ITERS: usize = 20;
const REGISTRY_ITERS: usize = 1_000;
const HOARD_REAL_ITERS: usize = 600;
const HOARD_K64_ITERS: usize = 600;
const HOARD_SYNTH_K: usize = 64;
const SLOT_READ_ITERS: usize = 20_000;
const QUIESCENT_ITERS: usize = 200;

type Centroids = Vec<(&'static str, [f32; riir_instinct::arsenal_ops::DIM])>;

fn main() {
    provenance();
    let datasets =
        std::path::PathBuf::from(env_or_default("INSTINCT_DATASETS_DIR", "../riir-reflex/.raw/datasets_t20k"));
    let winners =
        std::path::PathBuf::from(env_or_default("INSTINCT_WINNERS_DIR", "../riir-train/data/instinct_specialists"));
    if !datasets.is_dir() {
        fail(&format!(
            "datasets dir {} missing — set INSTINCT_DATASETS_DIR (the GOAT gate refuses an \
             absent population, it never prints a green zero)",
            datasets.display()
        ));
    }
    if !winners.join("ag_news_winner_v1.bin").is_file() {
        fail(&format!(
            "winner artifact ag_news_winner_v1.bin missing under {} — set INSTINCT_WINNERS_DIR",
            winners.display()
        ));
    }

    let mut gates: Vec<(&'static str, bool, String)> = Vec::new();

    // ── G-A: init under the full manifest ───────────────────────────────
    let manifest = Arc::new(
        ArsenalManifest::embedded_default()
            .unwrap_or_else(|e| fail(&format!("embedded manifest: {e}"))),
    );
    let suites: Vec<&'static str> = manifest
        .suites()
        .map(|s| -> &'static str { Box::leak(s.to_string().into_boxed_str()) })
        .collect();
    assert_eq!(suites.len(), 6, "the default manifest carries six rows");

    let mut validate_us = Vec::with_capacity(VALIDATE_ITERS);
    for _ in 0..VALIDATE_ITERS {
        let t = Instant::now();
        let fresh = ArsenalManifest::embedded_default()
            .unwrap_or_else(|e| fail(&format!("embedded manifest: {e}")));
        fresh
            .validate(&riir_instinct::arsenal::ValidateCtx::raw(&winners))
            .unwrap_or_else(|e| fail(&format!("manifest validate: {e}")));
        black_box(&fresh);
        validate_us.push(t.elapsed().as_micros());
    }
    validate_us.sort_unstable();
    let (vp50, vp99) = pct_us(&validate_us);
    let g_validate = vp99 <= BAR_VALIDATE_P99.as_micros();
    println!(
        "G-A init: manifest parse+validate p50 {vp50} µs / p99 {vp99} µs (bar p99 ≤ {} ms) — {}",
        BAR_VALIDATE_P99.as_millis(),
        verdict_word(g_validate)
    );
    gates.push(("init_validate_p99", g_validate, format!("p99 {vp99} µs")));

    let digest0 = [0u8; 32];
    let mut registry_us = Vec::with_capacity(REGISTRY_ITERS);
    for _ in 0..REGISTRY_ITERS {
        let t = Instant::now();
        let slots: Vec<LaneSlot<u64>> = suites
            .iter()
            .map(|s| {
                LaneSlot::new(s, LaneState::Unloaded { applied: EpochTag { epoch: 0, digest: digest0 } })
            })
            .collect();
        black_box(&slots);
        registry_us.push(t.elapsed().as_micros());
    }
    registry_us.sort_unstable();
    let (rp50, rp99) = pct_us(&registry_us);
    let g_registry = rp99 <= BAR_REGISTRY_P99.as_micros();
    println!(
        "G-A init: lazy registry (6 slots) p50 {rp50} µs / p99 {rp99} µs (bar p99 ≤ 1 ms) — {}",
        verdict_word(g_registry)
    );
    gates.push(("lazy_registry_p99", g_registry, format!("p99 {rp99} µs")));

    // The eager six-lane boot: the cost the lazy posture DEFERS —
    // disclosed per lane, never a gate.
    let mut servers: Vec<(&'static str, AnySuiteServer)> = Vec::with_capacity(6);
    let mut per_lane: Vec<(&'static str, u128)> = Vec::with_capacity(6);
    let eager_t = Instant::now();
    for s in &suites {
        let t = Instant::now();
        let server = AnySuiteServer::boot(s, &datasets, &winners, &manifest)
            .unwrap_or_else(|e| fail(&format!("eager boot {s}: {e}")));
        per_lane.push((s, t.elapsed().as_millis()));
        servers.push((s, server));
    }
    println!(
        "G-A init: EAGER six-lane boot total {} ms (the lazy posture defers this) — per lane: {}",
        eager_t.elapsed().as_millis(),
        per_lane
            .iter()
            .map(|(s, ms)| format!("{s} {ms} ms"))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Real centroids (the hoarding gate's production vectors).
    let centroids: Centroids = servers.iter().map(|(s, srv)| (*s, srv.centroid())).collect();

    // ── G-B: routing ns–µs ──────────────────────────────────────────────
    let cfg = SetAdmissionConfig::default();
    let mut real_us = Vec::with_capacity(HOARD_REAL_ITERS);
    let mut real_admits = 0usize;
    for i in 0..HOARD_REAL_ITERS {
        let (cand, cand_vec) = &centroids[i % centroids.len()];
        let loaded: Centroids = centroids
            .iter()
            .filter(|(s, _)| s != cand)
            .map(|(s, v)| (*s, *v))
            .collect();
        let t = Instant::now();
        let r = black_box(hoard_check(cand, *cand_vec, &loaded, &cfg));
        real_us.push(t.elapsed().as_nanos());
        if r.is_ok() {
            real_admits += 1;
        }
    }
    real_us.sort_unstable();
    let (hp50, hp99) = pct_ns(&real_us);
    let g_hoard = hp99 <= HOARD_BAR_US * 1_000;
    println!(
        "G-B routing: hoard_check k=5 (real six, round-robin) p50 {} ns / p99 {hp99} ns (bar \
         p99 ≤ {HOARD_BAR_US} µs, admits {real_admits}/{HOARD_REAL_ITERS}) — {}",
        hp50,
        verdict_word(g_hoard)
    );
    gates.push(("hoard_k5_p99", g_hoard, format!("p99 {hp99} ns")));

    // k=dozens headroom: deterministic synthetic directions (a fixture,
    // never entropy).
    let synth = synth_directions(HOARD_SYNTH_K);
    let mut k64_us = Vec::with_capacity(HOARD_K64_ITERS);
    let mut k64_ok = 0usize;
    for i in 0..HOARD_K64_ITERS {
        let cand = synth[i % HOARD_SYNTH_K];
        let loaded: Centroids = synth
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i % HOARD_SYNTH_K)
            .map(|(j, v)| (leak_name(j), *v))
            .collect();
        let t = Instant::now();
        let r = black_box(hoard_check("synthetic", cand, &loaded, &cfg));
        k64_us.push(t.elapsed().as_nanos());
        if r.is_ok() {
            k64_ok += 1;
        }
    }
    k64_us.sort_unstable();
    let (hp50k, hp99k) = pct_ns(&k64_us);
    let g_hoard64 = hp99k <= HOARD_BAR_US * 1_000;
    println!(
        "G-B routing: hoard_check k={HOARD_SYNTH_K} (synthetic; {k64_ok}/{HOARD_K64_ITERS} \
         admitted, the rest certificate-collapsed — the gate answered either way) p50 {hp50k} \
         ns / p99 {hp99k} ns — {}",
        verdict_word(g_hoard64)
    );
    gates.push(("hoard_k64_p99", g_hoard64, format!("p99 {hp99k} ns")));

    // The decision-path slot read: lock + Ready match, no decide.
    let slot_read: Arc<LaneSlot<u64>> = Arc::new(LaneSlot::new(
        "read",
        LaneState::Ready {
            server: 1,
            applied: EpochTag { epoch: 1, digest: [1; 32] },
            centroid: [1.0; riir_instinct::arsenal_ops::DIM],
        },
    ));
    let mut sr_us = Vec::with_capacity(SLOT_READ_ITERS);
    for _ in 0..SLOT_READ_ITERS {
        let t = Instant::now();
        let st = slot_read.state.lock().expect("slot lock");
        let epoch = match &*st {
            LaneState::Ready { applied, .. } => applied.epoch,
            _ => unreachable!("fixture is Ready"),
        };
        black_box(epoch);
        drop(st);
        sr_us.push(t.elapsed().as_nanos());
    }
    sr_us.sort_unstable();
    let (sp50, sp99) = pct_ns(&sr_us);
    let g_slot = sp99 <= SLOT_READ_BAR_US * 1_000;
    println!(
        "G-B routing: slot Ready-read p50 {sp50} ns / p99 {sp99} ns (bar p99 ≤ \
         {SLOT_READ_BAR_US} µs) — {}",
        verdict_word(g_slot)
    );
    gates.push(("slot_read_p99", g_slot, format!("p99 {sp99} ns")));

    // ── G-C: swap atomicity under concurrent decisions ──────────────────────
    let (g_swap, swap_report) = swap_hammer(&datasets, &winners, Arc::clone(&manifest));
    println!("{swap_report}");
    gates.push(("swap_no_torn_reads", g_swap, swap_report));

    // Quiescent worst-case disclosure: the 77-label banking77 suite,
    // uncontended (the hammer's contended p50/p99 is the serving shape;
    // this is the per-decision ceiling). Uses ag_news's real state text —
    // the eval path (bag → scores → fusion) is state-content-shape
    // dependent, not domain dependent.
    {
        let idx = servers
            .iter()
            .position(|(s, _)| *s == "banking77")
            .expect("banking77 in the default manifest");
        let (_, mut b77) = servers.swap_remove(idx);
        let state = real_state_text("banking77", &datasets, &winners, &manifest);
        let mut durs = Vec::with_capacity(QUIESCENT_ITERS);
        for _ in 0..QUIESCENT_ITERS {
            let t = Instant::now();
            let d = b77.decide(&state, None).unwrap_or_else(|e| fail(&format!("b77 decide: {e}")));
            black_box(&d);
            durs.push(t.elapsed().as_nanos());
        }
        durs.sort_unstable();
        let (qp50, qp99) = pct_ns(&durs);
        println!(
            "G-C decide: banking77 (77 labels) quiescent p50 {} ns ({:.2} µs) / p99 {qp99} ns \
             ({:.2} µs) over {QUIESCENT_ITERS} — disclosed (Bench 002 gated the fusion axis; \
             this is the served ceiling)",
            qp50,
            qp50 as f64 / 1_000.0,
            qp99 as f64 / 1_000.0
        );
    }

    // ── G-D: armed-off identity ─────────────────────────────────────────
    let mut g_identity = true;
    let mut id_notes: Vec<String> = Vec::new();
    g_identity &= hoard_gate_armed_for(None);
    g_identity &= !hoard_gate_armed_for(Some("0"));
    for v in ["1", "off", "0 ", " 0", "false"] {
        g_identity &= hoard_gate_armed_for(Some(v));
    }
    id_notes.push("kill-switch literal re-pinned (only exactly \"0\" disarms)".into());
    // The armed gate admits all six real centroids — measured twice,
    // bit-identical verdicts.
    let mut pass1: Vec<Result<HoardReport, String>> = Vec::new();
    let mut pass2: Vec<Result<HoardReport, String>> = Vec::new();
    for (cand, cand_vec) in &centroids {
        let loaded: Centroids = centroids
            .iter()
            .filter(|(s, _)| s != cand)
            .map(|(s, v)| (*s, *v))
            .collect();
        pass1.push(hoard_check(cand, *cand_vec, &loaded, &cfg).map_err(|e| e.to_string()));
        pass2.push(hoard_check(cand, *cand_vec, &loaded, &cfg).map_err(|e| e.to_string()));
    }
    let admitted1 = pass1.iter().filter(|r| r.is_ok()).count();
    if admitted1 != 6 {
        g_identity = false;
        id_notes.push(format!("armed gate admitted {admitted1}/6 — the default set must pass"));
    }
    // Per-suite verdicts with the gate's own numbers — the diagnostic
    // record when a suite refuses (the Bench-003 finding surfaced here).
    for ((cand, _), r) in centroids.iter().zip(pass1.iter()) {
        match r {
            Ok(rep) => println!(
                "G-D identity:   {cand}: admitted (vendi {:.3} / floor {:.3}, k {}, judged {})",
                rep.vendi, rep.floor, rep.k, rep.judged
            ),
            Err(e) => println!("G-D identity:   {cand}: REFUSED — {e}"),
        }
    }
    for (a, b) in pass1.iter().zip(pass2.iter()) {
        let same = match (a, b) {
            (Ok(x), Ok(y)) => {
                x.k == y.k
                    && x.saturated == y.saturated
                    && x.judged == y.judged
                    && x.vendi.to_bits() == y.vendi.to_bits()
                    && x.floor.to_bits() == y.floor.to_bits()
                    && x.participation_ratio.to_bits() == y.participation_ratio.to_bits()
            }
            (Err(x), Err(y)) => x == y,
            _ => false,
        };
        if !same {
            g_identity = false;
            id_notes.push("armed gate verdicts not bit-identical across two passes".into());
            break;
        }
    }
    id_notes.push(format!(
        "armed admits {admitted1}/6 real centroids (bit-identical verdicts ×2); disarmed admits \
         unconditionally — the serving outcome set (all six up) is identical either way"
    ));
    println!("G-D identity: {} — {}", verdict_word(g_identity), id_notes.join("; "));
    gates.push(("armed_off_identity", g_identity, id_notes.join("; ")));

    // ── verdict ─────────────────────────────────────────────────────────
    let failed: Vec<&(&'static str, bool, String)> =
        gates.iter().filter(|(_, ok, _)| !ok).collect();
    println!();
    if failed.is_empty() {
        println!("✓ arsenal_budget_goat: GOAT PASS — {} gates", gates.len());
    } else {
        for (name, _, note) in &failed {
            println!("✗ {name}: {note}");
        }
        println!(
            "⛔ arsenal_budget_goat: GOAT FAIL — {} of {}",
            failed.len(),
            gates.len()
        );
        std::process::exit(1);
    }
}

/// The swap hammer: REAL servers under the slot lock. Returns
/// (no-torn-reads gate, report line).
fn swap_hammer(
    datasets: &Path,
    winners: &Path,
    manifest: Arc<ArsenalManifest>,
) -> (bool, String) {
    let suite: &'static str = "ag_news";
    // The seat is consumed by the boot; extract a real state string first,
    // and install the booted server as the epoch-0 artifact (the boot
    // shape — no wasted extra boot).
    let seat = riir_reflex::harness::runner::seat::prepare_seat(suite, datasets)
        .unwrap_or_else(|e| fail(&format!("prepare_seat {suite}: {e}")));
    let state: String = match seat.suite.cases[0].state.clone() {
        serde_json::Value::String(s) => s,
        v => v.to_string(),
    };
    let srv0 = AnySuiteServer::boot_from_seat(suite, seat, winners, &manifest)
        .unwrap_or_else(|e| fail(&format!("boot {suite}: {e}")));
    let centroid0 = srv0.centroid();
    let winner_path = winners.join(format!("{suite}_winner_v1.bin"));
    let digest0 = blake3_bytes(&winner_path);

    let slot: Arc<LaneSlot<AnySuiteServer>> = Arc::new(LaneSlot::new(
        suite,
        LaneState::Unloaded {
            applied: EpochTag { epoch: 0, digest: digest0 },
        },
    ));
    slot.install_ready(srv0, EpochTag { epoch: 0, digest: digest0 }, centroid0, false)
        .unwrap_or_else(|e| fail(&format!("initial install: {e}")));

    let stop = Arc::new(AtomicBool::new(false));
    let errors: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let torn: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let mut readers = Vec::new();
    for _ in 0..HAMMER_READERS {
        let slot = Arc::clone(&slot);
        let stop = Arc::clone(&stop);
        let errors = Arc::clone(&errors);
        let torn = Arc::clone(&torn);
        let state = state.clone();
        readers.push(std::thread::spawn(move || {
            let mut last_epoch = 0u64;
            let mut durs: Vec<u128> = Vec::new();
            while !stop.load(Ordering::Relaxed) {
                let t = Instant::now();
                // The production shape: decide() runs UNDER the slot lock
                // — a swap serializes against it, a decision observes one
                // whole server.
                let mut st = slot.state.lock().expect("slot lock");
                if let LaneState::Ready { server, applied, .. } = &mut *st {
                    let epoch = applied.epoch;
                    if epoch < last_epoch {
                        torn.fetch_add(1, Ordering::Relaxed);
                    }
                    last_epoch = epoch;
                    match server.decide(&state, None) {
                        Ok(d) => {
                            black_box(&d);
                            durs.push(t.elapsed().as_nanos());
                        }
                        Err(_) => {
                            errors.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            }
            durs.sort_unstable();
            durs
        }));
    }

    // The swapper: strict epoch advances, each built the production way
    // (fresh seat + artifact bytes → whole-snapshot install).
    let swapper_slot = Arc::clone(&slot);
    let swapper_datasets = datasets.to_path_buf();
    let swapper = std::thread::spawn(move || {
        let mut install_ms: Vec<u128> = Vec::new();
        for e in 1..=HAMMER_SWAPS {
            let t = Instant::now();
            let bytes =
                std::fs::read(&winner_path).unwrap_or_else(|e| fail(&format!("read winner: {e}")));
            let digest = *blake3::hash(&bytes).as_bytes();
            let seat = riir_reflex::harness::runner::seat::prepare_seat(suite, &swapper_datasets)
                .unwrap_or_else(|e| fail(&format!("prepare_seat (swapper): {e}")));
            let srv = AnySuiteServer::boot_bytes(suite, seat, &bytes, &manifest)
                .unwrap_or_else(|e| fail(&format!("boot_bytes (swapper): {e}")));
            let outcome = swapper_slot
                .install_ready(srv, EpochTag { epoch: e, digest }, centroid0, false)
                .unwrap_or_else(|e| fail(&format!("strict advance must pass the gate: {e}")));
            assert!(matches!(outcome, InstallOutcome::Advanced));
            install_ms.push(t.elapsed().as_millis());
        }
        install_ms
    });

    let mut install_ms = swapper.join().expect("swapper");
    stop.store(true, Ordering::Relaxed);
    let mut all_durs: Vec<u128> = Vec::new();
    for r in readers {
        all_durs.extend(r.join().expect("reader"));
    }
    all_durs.sort_unstable();

    let e = errors.load(Ordering::Relaxed);
    let t = torn.load(Ordering::Relaxed);
    let n = all_durs.len();
    let (dp50, dp99) = pct_ns(&all_durs);
    install_ms.sort_unstable();
    let (ip50, ip99) = pct_us(&install_ms);
    let g = e == 0 && t == 0 && n >= MIN_HAMMER_DECISIONS;
    let report = format!(
        "G-C swap: {n} decisions served across {HAMMER_SWAPS} installs, {e} decision errors, \
         {t} torn reads (bar: 0/0, ≥{MIN_HAMMER_DECISIONS} served) — {}; contended decide p50 \
         {dp50} ns / p99 {dp99} ns; swap install p50 {ip50} ms / p99 {ip99} ms (disclosed)",
        verdict_word(g)
    );
    (g, report)
}

/// One suite's first test case's state text, via a throwaway seat (the
/// quiescent disclosure only; the hammer reuses the hammer seat's text).
fn real_state_text(
    suite: &'static str,
    datasets: &Path,
    winners: &Path,
    manifest: &ArsenalManifest,
) -> String {
    let _ = (winners, manifest);
    let seat = riir_reflex::harness::runner::seat::prepare_seat(suite, datasets)
        .unwrap_or_else(|e| fail(&format!("prepare_seat {suite}: {e}")));
    match seat.suite.cases[0].state.clone() {
        serde_json::Value::String(s) => s,
        v => v.to_string(),
    }
}

// ── helpers ─────────────────────────────────────────────────────────────

fn fail(msg: &str) -> ! {
    eprintln!("⛔ arsenal_budget_goat: {msg}");
    std::process::exit(1);
}

fn env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn verdict_word(ok: bool) -> &'static str {
    if ok { "PASS" } else { "FAIL" }
}

/// Nearest-rank percentiles over a sorted-ascending ns table.
fn pct_ns(sorted: &[u128]) -> (u128, u128) {
    (pct(sorted, 50), pct(sorted, 99))
}

/// Nearest-rank percentiles over a sorted-ascending µs/ms table.
fn pct_us(sorted: &[u128]) -> (u128, u128) {
    (pct(sorted, 50), pct(sorted, 99))
}

fn pct(sorted: &[u128], p: u64) -> u128 {
    let n = sorted.len();
    if n == 0 {
        return 0;
    }
    let idx = ((p as f64 / 100.0 * n as f64).ceil() as usize).saturating_sub(1);
    sorted[idx.min(n - 1)]
}

/// Deterministic pseudo-random unit directions in the 8-dim admission
/// space (xorshift64, fixed seed — the k-scaling population is a
/// fixture, never entropy).
fn synth_directions(k: usize) -> Vec<[f32; riir_instinct::arsenal_ops::DIM]> {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut out = Vec::with_capacity(k);
    while out.len() < k {
        let mut v = [0.0_f32; riir_instinct::arsenal_ops::DIM];
        for x in &mut v {
            let bits = next();
            *x = ((bits >> 11) as f32 / (1u64 << 53) as f32) * 2.0 - 1.0;
        }
        let n: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        if n <= 0.0 || !n.is_finite() {
            continue;
        }
        for x in &mut v {
            *x /= n;
        }
        out.push(v);
    }
    out
}

/// Leaked slot names for the synthetic k-set (bench-only; leaked once).
fn leak_name(i: usize) -> &'static str {
    use std::sync::OnceLock;
    static NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();
    let names = NAMES.get_or_init(|| {
        (0..HOARD_SYNTH_K)
            .map(|i| {
                let s: &'static str = Box::leak(format!("d{i}").into_boxed_str());
                s
            })
            .collect()
    });
    names[i % names.len()]
}

fn blake3_bytes(path: &Path) -> [u8; 32] {
    let bytes =
        std::fs::read(path).unwrap_or_else(|e| fail(&format!("read {}: {e}", path.display())));
    *blake3::hash(&bytes).as_bytes()
}

/// The box state beside every latency figure (the house law): load
/// average + power source, printed, never gating. Best-effort — a spawn
/// failure degrades to a missing line, never a crash.
fn provenance() {
    println!("PROVENANCE: host m3 (Apple M3 Max, 16 cores)");
    if let Ok(out) = std::process::Command::new("uptime").output() {
        println!("PROVENANCE: {}", String::from_utf8_lossy(&out.stdout).trim());
    }
    if let Ok(out) = std::process::Command::new("pmset").arg("-g").arg("batt").output() {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines().take(2) {
            println!("PROVENANCE: power: {}", line.trim());
        }
    }
    println!(
        "PROVENANCE: profile release · iters: {VALIDATE_ITERS} validate · {REGISTRY_ITERS} \
         registry · {HOARD_REAL_ITERS} hoard k5 · {HOARD_K64_ITERS} hoard \
         k{HOARD_SYNTH_K} · {SLOT_READ_ITERS} slot reads · {QUIESCENT_ITERS} quiescent \
         decides · {HAMMER_SWAPS}-swap hammer × {HAMMER_READERS} readers"
    );
}

//! The staleness-probe gates (Issue 012 / Plan 005 T4). The verdict rows are
//! PINNED AS MEASURED (Bench 022, 2026-09-29) — a re-pin is a measured
//! decision, never a silent loosening. Skips LOUD without the on-disk
//! material (the serve_gates convention): fixture + winners dir.

use riir_instinct::staleness::{
    self, PairSide, ProbeSet, SuiteProbe, artifact_digest, compare_pair,
};
use riir_instinct::specialist::{load_artifact, winner_bridge, BagConvention};
use std::path::{Path, PathBuf};

const FIXTURE_PATH: &str = "tests/fixtures/staleness_probe_set.json";
/// The measured pins. **RE-PINNED 2026-10-05 (riir-rethink Issue 023 T4):**
/// ag_news (unknown licence) and emotion (research-only) raw texts left
/// the PUBLIC fixture — massive_intent_en (Apache-2.0) and prompt_injections
/// (Apache-2.0) took their slots. The banking77 drift row measured
/// EXACTLY its old pins across the rebuild (the slice is byte-identical —
/// the deterministic round-robin law); the candidate row is now the MASSIVE
/// winner-vs-armA pair (14 flips — richer than ag_news's 1); the
/// byte-identical control role moved from emotion to prompt_injections
/// (winner == armA bytes, verified sha256).
const PIN_FIXTURE_DIGEST: &str = "3d332d6c816d8c7162fe7f127e51ac3749417b6f321be0e5f0750b2efe5c2caa";
const PIN_DRIFT_FLIPS: usize = 13;
const PIN_DRIFT_MEAN_GOLD: f64 = 0.24003;
const PIN_CANDIDATE_FLIPS: usize = 14;

fn winners_dir() -> PathBuf {
    std::env::var("INSTINCT_WINNERS_DIR")
        .map(PathBuf::from)
        // The artifact lane's pull target (instinct issue 020 / Plan 623 T4).
        .unwrap_or_else(|_| PathBuf::from("artifacts/cache"))
}

fn side<'a>(
    spec: &'a riir_instinct::Specialist,
    conv: BagConvention,
    name: &'a str,
    digest: &'a str,
) -> PairSide<'a> {
    PairSide { spec, conv, name, digest }
}

fn fixture() -> Option<ProbeSet> {
    let path = Path::new(FIXTURE_PATH);
    if !path.exists() {
        eprintln!("SKIP (loud): no probe fixture at {FIXTURE_PATH} — run the example's --build-probe");
        return None;
    }
    ProbeSet::load(path).ok()
}

fn slice<'a>(set: &'a ProbeSet, suite: &str) -> Option<&'a SuiteProbe> {
    let s = set.suites.iter().find(|s| s.suite == suite);
    if s.is_none() {
        eprintln!("SKIP (loud): fixture carries no {suite} slice");
    }
    s
}

fn artifact(dir: &Path, file: &str) -> Option<(riir_instinct::Specialist, String)> {
    let path = dir.join(file);
    if !path.exists() {
        eprintln!("SKIP (loud): no artifact {} — set INSTINCT_WINNERS_DIR", path.display());
        return None;
    }
    let digest = artifact_digest(&path).ok()?;
    let spec = load_artifact(&path).ok()?;
    Some((spec, digest))
}

/// The fixture is pinned: a changed fixture moves every digest it rides, so
/// the change must be a measured re-pin (Plan 005 T1's stability law).
#[test]
fn fixture_digest_matches_the_pinned_bench_022_record() {
    let Some(set) = fixture() else { return };
    assert_eq!(
        set.digest_hex(),
        PIN_FIXTURE_DIGEST,
        "probe fixture changed without a measured re-pin — rebuild + re-run Bench 022 + re-pin"
    );
}

/// The free canary (structural): a byte-identical pair reads EXACTLY zero —
/// every delta, every item, no tolerance.
#[test]
fn identical_pair_reads_exactly_zero() {
    let Some(set) = fixture() else { return };
    let Some(probe) = slice(&set, "banking77") else { return };
    let dir = winners_dir();
    let Some((spec, digest)) = artifact(&dir, "banking77_nbsvm_v2.bin") else {
        return;
    };
    let conv = winner_bridge("banking77").convention;
    let r = compare_pair(probe, side(&spec, conv, "v2", &digest), side(&spec, conv, "v2", &digest), "fx")
        .expect("canary compares");
    assert_eq!(r.flips, 0);
    assert_eq!(r.mean_abs_gold_delta, 0.0, "canary must be EXACT zero");
    assert_eq!(r.max_abs_gold_delta, 0.0);
    assert!(r.divergences.iter().all(|d| d.gold_delta == 0.0));
    assert!(r.divergences.iter().all(|d| d.margin_delta == 0.0));
    assert!(!r.fired);
}

/// The drift row (Bench 022's measured verdict): the real Bench-012 bridge
/// event (v1 count-bag → v2 presence-bag) FIRES with 13/64 flips and mean
/// |Δgold| 0.240 — the probe detects a real historical drift with wide
/// margin. NOT dead-by-domination; the T6 swap-path hook is therefore live
/// work (issue 012), and a re-pin here is a measured decision.
#[test]
fn banking77_v1_to_v2_bridge_drift_fires_as_measured() {
    let Some(set) = fixture() else { return };
    let Some(probe) = slice(&set, "banking77") else { return };
    let dir = winners_dir();
    let Some((live, live_digest)) = artifact(&dir, "banking77_nbsvm_v2.bin") else {
        return;
    };
    let Some((reference, ref_digest)) = artifact(&dir, "banking77_winner_v1.bin") else {
        return;
    };
    let r = compare_pair(
        probe,
        side(&live, winner_bridge("banking77").convention, "v2 (bridged winner)", &live_digest),
        side(&reference, BagConvention::Count, "v1 (pre-bridge)", &ref_digest),
        "fx",
    )
    .expect("drift row compares");
    assert!(r.fired, "the bridge drift must fire");
    assert_eq!(r.fire_reason, "pick flip");
    assert_eq!(r.flips, PIN_DRIFT_FLIPS, "flip count moved — re-measure + re-pin");
    assert!(
        (r.mean_abs_gold_delta - PIN_DRIFT_MEAN_GOLD).abs() < 5e-6,
        "mean |Δgold| moved: {} vs pinned {} — re-measure + re-pin",
        r.mean_abs_gold_delta,
        PIN_DRIFT_MEAN_GOLD
    );
    assert!(r.label_universe.identical, "v1/v2 universes were identical at the pin");
}

/// The candidate row: two same-generation massive_intent_en artifacts
/// read divergent (14 flips, mean |Δgold| 0.107 — re-measured 2026-10-05
/// at the licence swap; the pre-swap ag_news row was 1 flip / 0.041) —
/// the pre-swap report's exact use case: a candidate swap is visible
/// before it is served.
#[test]
fn massive_candidate_pair_fires_as_measured() {
    let Some(set) = fixture() else { return };
    let Some(probe) = slice(&set, "massive_intent_en") else { return };
    let dir = winners_dir();
    let Some((live, live_digest)) = artifact(&dir, "massive_intent_en_winner_v1.bin") else {
        return;
    };
    let Some((reference, ref_digest)) = artifact(&dir, "massive_intent_en_armA_v1.bin") else {
        return;
    };
    let conv = winner_bridge("massive_intent_en").convention;
    let r = compare_pair(
        probe,
        side(&live, conv, "winner v1", &live_digest),
        side(&reference, conv, "armA v1", &ref_digest),
        "fx",
    )
    .expect("candidate row compares");
    assert!(r.fired);
    assert_eq!(r.flips, PIN_CANDIDATE_FLIPS, "flip count moved — re-measure + re-pin");
}

/// Determinism: a second full run of the drift row is byte-identical (the
/// divergence values, not just the aggregate — a bit flip anywhere is a
/// defect).
#[test]
fn drift_row_is_bit_identical_across_runs() {
    let Some(set) = fixture() else { return };
    let Some(probe) = slice(&set, "banking77") else { return };
    let dir = winners_dir();
    let Some((live, live_digest)) = artifact(&dir, "banking77_nbsvm_v2.bin") else {
        return;
    };
    let Some((reference, ref_digest)) = artifact(&dir, "banking77_winner_v1.bin") else {
        return;
    };
    let run = || {
        compare_pair(
            probe,
            side(&live, winner_bridge("banking77").convention, "v2", &live_digest),
            side(&reference, BagConvention::Count, "v1", &ref_digest),
            "fx",
        )
        .expect("drift row compares")
    };
    let a = run();
    let b = run();
    let ja = serde_json::to_string(&a).expect("serializes");
    let jb = serde_json::to_string(&b).expect("serializes");
    assert_eq!(ja, jb, "two runs of the same pair must be byte-identical");
}

/// The reporter's own summary line matches the fired-set arithmetic (the
/// fire rule is pre-registered: any flip OR mean |Δgold| ≥ the bar).
#[test]
fn fire_rule_is_the_pre_registered_disjunction() {
    let Some(set) = fixture() else { return };
    let dir = winners_dir();
    let Some(probe) = slice(&set, "massive_intent_en") else { return };
    let Some((live, live_digest)) = artifact(&dir, "massive_intent_en_winner_v1.bin") else {
        return;
    };
    let Some((reference, ref_digest)) = artifact(&dir, "massive_intent_en_armA_v1.bin") else {
        return;
    };
    let conv = winner_bridge("massive_intent_en").convention;
    let r = compare_pair(
        probe,
        side(&live, conv, "w", &live_digest),
        side(&reference, conv, "a", &ref_digest),
        "fx",
    )
    .expect("compares");
    let expected = r.flips > 0 || r.mean_abs_gold_delta >= staleness::FIRE_GOLD_DELTA;
    assert_eq!(r.fired, expected);
}

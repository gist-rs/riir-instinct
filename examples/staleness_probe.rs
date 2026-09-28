//! The staleness-probe runner (Issue 012 / Plan 005) — builds the pinned
//! probe fixture and runs the report rows over the on-disk artifact pairs.
//!
//! Modes:
//! - `--build-probe` — read the datasets (`INSTINCT_DATASETS_DIR`, default
//!   `../riir-reflex/.raw/datasets_t20k`) and write the committed fixture
//!   `tests/fixtures/staleness_probe_set.json` (labels spent at record time;
//!   probe time needs no dataset dir). One-time + on fixture change.
//! - default — load the fixture, verify its digest, run the report rows over
//!   the artifact pairs (`INSTINCT_WINNERS_DIR`, default
//!   `../riir-train/data/instinct_specialists`), print the verdict table and
//!   write `.benchmarks/022_staleness_probe/` (results.json + REPORT.md).
//!
//! REPORT-ONLY: nothing here serves, swaps, or writes any serving state.

use riir_instinct::staleness::{
    self, PairSide, ProbeItem, ProbeSet, SuiteProbe, artifact_digest, compare_pair,
};
use riir_instinct::specialist::{load_artifact, winner_bridge, BagConvention};
use std::path::{Path, PathBuf};

const FIXTURE_PATH: &str = "tests/fixtures/staleness_probe_set.json";
const BENCH_DIR: &str = ".benchmarks/022_staleness_probe";
const ITEMS_PER_SUITE: usize = 64;

fn datasets_dir() -> PathBuf {
    std::env::var("INSTINCT_DATASETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../riir-reflex/.raw/datasets_t20k"))
}

fn winners_dir() -> PathBuf {
    std::env::var("INSTINCT_WINNERS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../riir-train/data/instinct_specialists"))
}

/// The suite builders' text+gold extraction for the three probed suites —
/// the same row fields the reflex builders read (banking77 mteb: `text` +
/// `label_text`; ag_news/emotion: `text` + int `label`), written ONCE into
/// the fixture so probe time never re-reads datasets.
fn row_text_gold(suite: &str, row: &serde_json::Value) -> Option<(String, String)> {
    let text = row.get("text")?.as_str()?.to_string();
    let gold = match suite {
        "banking77" => row.get("label_text")?.as_str()?.to_string(),
        _ => row.get("label")?.as_i64()?.to_string(),
    };
    Some((text, gold))
}

/// Label-stratified round-robin over the suite's TEST rows (the mirror's
/// first rows are label-clustered — first-N would read one label).
fn build_suite_probe(
    datasets: &Path,
    suite: &str,
    presented_keys: &[String],
) -> Result<SuiteProbe, String> {
    let dir = datasets.join(suite);
    let mut rows: Vec<serde_json::Value> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("test-") && name.ends_with(".json") {
            names.push(name);
        }
    }
    names.sort();
    if names.is_empty() {
        return Err(format!("{suite}: no test-*.json under {}", dir.display()));
    }
    for name in &names {
        let text =
            std::fs::read_to_string(dir.join(name)).map_err(|e| format!("read {name}: {e}"))?;
        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("parse {name}: {e}"))?;
        if let Some(rs) = v.get("rows").and_then(|r| r.as_array()) {
            rows.extend(rs.iter().filter_map(|r| r.get("row").cloned()));
        }
    }
    // Round-robin one row per label per pass, in first-seen label order —
    // deterministic, no RNG, label-coverage-first.
    let mut per_label: Vec<(String, Vec<&serde_json::Value>)> = Vec::new();
    for row in &rows {
        let Some((_, gold)) = row_text_gold(suite, row) else {
            continue;
        };
        match per_label.iter_mut().find(|(l, _)| *l == gold) {
            Some((_, bucket)) => bucket.push(row),
            None => per_label.push((gold, vec![row])),
        }
    }
    let mut items = Vec::with_capacity(ITEMS_PER_SUITE);
    'outer: for round in 0.. {
        for (label, bucket) in &per_label {
            let Some(row) = bucket.get(round) else {
                continue;
            };
            let Some((text, _)) = row_text_gold(suite, row) else {
                continue;
            };
            items.push(ProbeItem {
                id: format!("{suite}:{round}:{}", items.len()),
                text,
                gold_key: label.clone(),
                gold_pos: presented_keys
                    .iter()
                    .position(|k| k == label)
                    .unwrap_or(items.len().min(presented_keys.len().saturating_sub(1))),
                });
            if items.len() >= ITEMS_PER_SUITE {
                break 'outer;
            }
        }
        if round > 10_000 {
            return Err(format!("{suite}: round-robin did not converge"));
        }
    }
    Ok(SuiteProbe {
        suite: suite.to_string(),
        presented_keys: presented_keys.to_vec(),
        items,
    })
}

/// The presented-option universes (the builders' fixed key sets; banking77's
/// keys are the artifact labels in the artifact's own spelling — resolved by
/// name at probe time, so the fixture carries the mteb spelling and the
/// resolver's fallbacks bridge it).
fn presented_keys_for(suite: &str, datasets: &Path) -> Result<Vec<String>, String> {
    match suite {
        "banking77" => {
            // Sorted unique label_text over ALL test rows (the mteb
            // builder's rule), underscore form → the artifact label form is
            // the resolver's business.
            let mut labels: Vec<String> = Vec::new();
            let dir = datasets.join(suite);
            let mut names: Vec<String> = std::fs::read_dir(&dir)
                .map_err(|e| format!("read_dir {}: {e}", dir.display()))?
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| n.starts_with("test-") && n.ends_with(".json"))
                .collect();
            names.sort();
            for name in &names {
                let text = std::fs::read_to_string(dir.join(name)).map_err(|e| e.to_string())?;
                let v: serde_json::Value =
                    serde_json::from_str(&text).map_err(|e| format!("parse {name}: {e}"))?;
                if let Some(rs) = v.get("rows").and_then(|r| r.as_array()) {
                    for r in rs {
                        if let Some(t) = r
                            .get("row")
                            .and_then(|r| r.get("label_text"))
                            .and_then(|t| t.as_str())
                        {
                            if !labels.iter().any(|l| l == t) {
                                labels.push(t.to_string());
                            }
                        }
                    }
                }
            }
            labels.sort();
            Ok(labels
                .into_iter()
                .map(|l| l.replace('_', " "))
                .collect())
        }
        "ag_news" => Ok(vec![
            "world".into(),
            "sports".into(),
            "business".into(),
            "sci_tech".into(),
        ]),
        "emotion" => Ok(vec![
            "sadness".into(),
            "joy".into(),
            "love".into(),
            "anger".into(),
            "fear".into(),
            "surprise".into(),
        ]),
        other => Err(format!("no presented-key universe for suite {other:?}")),
    }
}

fn build_probe() -> Result<(), String> {
    let datasets = datasets_dir();
    let mut suites = Vec::new();
    for suite in ["banking77", "ag_news", "emotion"] {
        let keys = presented_keys_for(suite, &datasets)?;
        let probe = build_suite_probe(&datasets, suite, &keys)?;
        println!(
            "  {suite}: {} items over {} presented keys",
            probe.items.len(),
            keys.len()
        );
        suites.push(probe);
    }
    let set = ProbeSet {
        generated: "2026-09-29".into(),
        source: format!("{} (test splits, label-stratified round-robin)", datasets.display()),
        suites,
    };
    let json = set.canonical_json();
    let path = Path::new(FIXTURE_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, &json).map_err(|e| format!("write {FIXTURE_PATH}: {e}"))?;
    println!("wrote {FIXTURE_PATH} (digest {})", set.digest_hex());
    Ok(())
}

struct Side {
    name: &'static str,
    file: &'static str,
    convention: BagConvention,
}

fn side_of<'a>(spec: &'a riir_instinct::Specialist, side: &Side, digest: &'a str) -> PairSide<'a> {
    PairSide {
        spec,
        conv: side.convention,
        name: side.name,
        digest,
    }
}

/// Run the report rows. Each suite slice names its own pair; the live side
/// is ALWAYS the currently-bridged winner (what serving would carry), the
/// reference side the stale/candidate artifact.
fn run_report() -> Result<(), String> {
    let fixture = Path::new(FIXTURE_PATH);
    let set = staleness::ProbeSet::load(fixture)?;
    let digest = set.digest_hex();
    println!("fixture digest {digest}");
    let winners = winners_dir();
    let rows: Vec<(&str, Side, Side)> = vec![
        (
            "banking77",
            Side {
                name: "v2 (bridged winner)",
                file: "banking77_nbsvm_v2.bin",
                convention: winner_bridge("banking77").convention,
            },
            Side {
                name: "v2 (self — canary)",
                file: "banking77_nbsvm_v2.bin",
                convention: winner_bridge("banking77").convention,
            },
        ),
        (
            "banking77",
            Side {
                name: "v2 (bridged winner)",
                file: "banking77_nbsvm_v2.bin",
                convention: winner_bridge("banking77").convention,
            },
            Side {
                name: "v1 (pre-bridge)",
                file: "banking77_winner_v1.bin",
                convention: BagConvention::Count,
            },
        ),
        (
            "ag_news",
            Side {
                name: "winner v1",
                file: "ag_news_winner_v1.bin",
                convention: winner_bridge("ag_news").convention,
            },
            Side {
                name: "armA v1",
                file: "ag_news_armA_v1.bin",
                convention: winner_bridge("ag_news").convention,
            },
        ),
        (
            "emotion",
            Side {
                name: "winner v1",
                file: "emotion_winner_v1.bin",
                convention: winner_bridge("emotion").convention,
            },
            Side {
                name: "armA v1 (byte-identical)",
                file: "emotion_armA_v1.bin",
                convention: winner_bridge("emotion").convention,
            },
        ),
    ];
    let mut reports = Vec::new();
    for (suite, live, reference) in &rows {
        let probe = set
            .suites
            .iter()
            .find(|s| &s.suite == suite)
            .ok_or_else(|| format!("fixture carries no {suite} slice"))?;
        let live_path = winners.join(live.file);
        let ref_path = winners.join(reference.file);
        let live_spec = load_artifact(&live_path)?;
        let ref_spec = load_artifact(&ref_path)?;
        let live_digest = artifact_digest(&live_path)?;
        let ref_digest = artifact_digest(&ref_path)?;
        let r = compare_pair(
            probe,
            side_of(&live_spec, live, &live_digest),
            side_of(&ref_spec, reference, &ref_digest),
            &digest,
        )?;
        println!(
            "{:>9} {:>24} vs {:<24} items {} flips {} mean|Δgold| {:.5} max {:.5} mean|Δmargin| {:.5} → {}",
            r.suite,
            r.live_name,
            r.ref_name,
            r.items,
            r.flips,
            r.mean_abs_gold_delta,
            r.max_abs_gold_delta,
            r.mean_abs_margin_delta,
            if r.fired { "FIRED" } else { "quiet" },
        );
        reports.push(r);
    }
    write_bench(&reports, &digest)?;
    Ok(())
}

fn write_bench(reports: &[staleness::PairReport], fixture_digest: &str) -> Result<(), String> {
    let dir = Path::new(BENCH_DIR);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let results = serde_json::json!({
        "fixture_digest": fixture_digest,
        "fire_gold_delta_bar": staleness::FIRE_GOLD_DELTA,
        "box_state": box_state(),
        "rows": reports,
    });
    let path = dir.join("results.json");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&results).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("write {}: {e}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}

fn box_state() -> serde_json::Value {
    let load = std::fs::read_to_string("/proc/loadavg")
        .map(|t| t.split_whitespace().next().unwrap_or("?").to_string())
        .unwrap_or_else(|_| "n/a (macOS)".into());
    serde_json::json!({
        "recorded_at_unix": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        "loadavg_1m_or_n_a": load,
        "note": "divergence is a correctness/determinism readout, not a latency claim — \
                 the load figure is provenance, not a gate",
    })
}

fn main() {
    let build = std::env::args().any(|a| a == "--build-probe");
    let result = if build {
        build_probe()
    } else {
        run_report()
    };
    if let Err(e) = result {
        eprintln!("staleness_probe: {e}");
        std::process::exit(1);
    }
}

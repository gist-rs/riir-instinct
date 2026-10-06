//! Issue 006 T1 probe — WHERE does the massive specialist collapse?
//!
//! Scores one winner artifact through the SERVING path (the reader + the
//! reflex tokenizer law) over every text source and form, so the collapse
//! point is identified by differential rather than by hypothesis:
//!
//! - source: t20k train rows (Bench 609's own distribution), t20k test
//!   rows, datasets train rows, datasets test rows (the seat's rows);
//! - form: RAW row text vs the seat's pyjson state string
//!   (`{'utterance': …}`);
//! - measure: argmax over the artifact's full universe vs the row's gold
//!   label_text (the train-core measure), PLUS the pick distribution
//!   (a collapsed model shows as a one-class pick distribution).
//!
//! ```text
//! cargo run --release --example massive_anomaly_probe -- \
//!     [artifacts/cache]
//! ```
//!
//! The default dir is the artifact lane's pull target (instinct issue
//! 020 / Plan 623 T4).

use std::collections::BTreeMap;
use std::path::PathBuf;

use riir_instinct::specialist::{bag_into, load_artifact};

struct Row {
    label: String,
    text: String,
}

fn load_rows(dir: &std::path::Path, prefix: &str) -> Result<Vec<Row>, String> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read_dir {}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(prefix) && n.ends_with(".json"))
        })
        .collect();
    files.sort();
    let mut rows = Vec::new();
    for f in &files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("read {}: {e}", f.display()))?;
        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", f.display()))?;
        let Some(list) = v.get("rows").and_then(|r| r.as_array()) else {
            return Err(format!("{}: no rows array", f.display()));
        };
        for r in list {
            let row = match r.get("row") {
                Some(row) => row,
                None => r,
            };
            let (Some(label), Some(text)) = (
                row.get("label_text").and_then(|v| v.as_str()),
                row.get("text").and_then(|v| v.as_str()),
            ) else {
                continue;
            };
            rows.push(Row {
                label: label.to_string(),
                text: text.to_string(),
            });
        }
    }
    Ok(rows)
}

/// Argmax over the artifact's full universe (the train-core measure),
/// scoring through the serving bag law. Returns (pick, correct).
fn score(form_text: &[u8], m: &riir_instinct::specialist::Specialist, gold: &str) -> (usize, bool) {
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    bag_into(form_text, &mut bag, &mut scratch);
    let pick = m.pick(&bag);
    let correct = m.labels.get(pick).is_some_and(|l| l == gold);
    (pick, correct)
}

fn eval_source(name: &str, rows: &[Row], m: &riir_instinct::specialist::Specialist) {
    for (form, render) in [
        (
            "raw",
            Box::new(|t: &str| t.as_bytes().to_vec()) as Box<dyn Fn(&str) -> Vec<u8>>,
        ),
        (
            "state",
            Box::new(|t: &str| {
                let v = serde_json::json!({ "utterance": t });
                riir_reflex::pyjson::serialize_state(&v).into_bytes()
            }) as Box<dyn Fn(&str) -> Vec<u8>>,
        ),
    ] {
        let mut hits = 0usize;
        let mut n = 0usize;
        let mut picks: BTreeMap<&str, usize> = BTreeMap::new();
        for r in rows {
            let bytes = render(&r.text);
            let (pick, correct) = score(&bytes, m, &r.label);
            hits += usize::from(correct);
            n += 1;
            if let Some(l) = m.labels.get(pick) {
                *picks.entry(l.as_str()).or_default() += 1;
            }
        }
        let mut top: Vec<(&str, usize)> = picks.into_iter().collect();
        top.sort_by_key(|&(_, c)| std::cmp::Reverse(c));
        let tops: Vec<String> = top
            .iter()
            .take(3)
            .map(|(l, c)| format!("{l}:{c}"))
            .collect();
        println!(
            "  {name:>14} / {form:>5}: {hits}/{n} = {:.4} · distinct picks {} · top [{}]",
            hits as f64 / n.max(1) as f64,
            top.len(),
            tops.join(", ")
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let winners = args
        .first()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/cache"));
    let winner_path = winners.join("massive_intent_en_winner_v1.bin");
    let m = load_artifact(&winner_path).unwrap_or_else(|e| {
        eprintln!("load {}: {e}", winner_path.display());
        std::process::exit(1);
    });
    println!(
        "artifact: {} · {} labels · seal verified",
        winner_path.display(),
        m.labels.len()
    );

    let sources: [(&str, PathBuf, &str); 4] = [
        (
            "t20k train",
            PathBuf::from("../riir-reflex/.raw/datasets_t20k/massive_intent_en"),
            "train-",
        ),
        (
            "t20k test",
            PathBuf::from("../riir-reflex/.raw/datasets_t20k/massive_intent_en"),
            "test-",
        ),
        (
            "datasets train",
            PathBuf::from("../riir-reflex/.raw/datasets/massive_intent_en"),
            "train-",
        ),
        (
            "datasets test",
            PathBuf::from("../riir-reflex/.raw/datasets/massive_intent_en"),
            "test-",
        ),
    ];
    for (name, dir, prefix) in sources {
        match load_rows(&dir, prefix) {
            Ok(rows) if !rows.is_empty() => {
                println!("{name}: {} rows", rows.len());
                eval_source(name, &rows, &m);
            }
            Ok(_) => println!("{name}: NO ROWS"),
            Err(e) => println!("{name}: LOAD FAILED — {e}"),
        }
    }
}

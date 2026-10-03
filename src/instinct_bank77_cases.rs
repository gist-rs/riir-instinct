//! Plan 425 T2 — the banking77 teacher-pass case law, reimplemented
//! train-side (the established law-reimplementation pattern; riir-train
//! has no cargo dep on reflex, BOUNDARY.md). The v2 teacher dump this lane
//! writes must join the SAME rows in the SAME order as the collapsed
//! teacher's dump, or Arm B's student join silently trains on a different
//! population — so the law is pinned per-row against the existing dump's
//! gold column before any spend.

use std::path::Path;

use serde_json::{json, Map, Value};

use crate::instinct_specialist::read_train_envelope;

/// One teacher-pass case: the wire form the laya agent consumes plus the
/// student-side join facts.
#[derive(Debug, Clone)]
pub struct TeacherCase {
    /// The case id (`banking77:<pos>`, the reflex builder's spelling).
    pub id: String,
    /// The state object — `{"message": <text>}`.
    pub state: Value,
    /// The question wire form — `{"type":"choice","instructions":…,
    /// "criteria":{<key>:null,…}}` (keys in sorted-label order; insertion
    /// order IS the option order).
    pub question: Value,
    /// Student-side gold class index (the row's position in the sorted
    /// class list).
    pub gold: usize,
    /// The row's label (the space-joined `label_text` spelling, the
    /// student-side `SuiteRow::label` law).
    pub label: String,
}

/// The banking77 (mteb mirror) teacher-pass population:
///
/// - rows: the sorted `train-*.json` envelope files, `.rows[].row` in
///   load order;
/// - doc label: `label_text` with `_` → ` ` (the int-label fallback of
///   the reflex/train `train_row_label` law; mteb rows carry label_text);
/// - survivor: text AND label present (the `train_docs` filter_map);
/// - option universe: sorted unique `label_text` over ALL rows on disk,
///   keys = the same `_` → ` ` transform (the `build_banking77_mteb`
///   law — the universe comes from the whole split, the cases from the
///   survivor order);
/// - gold: the row's key position in the sorted key list;
/// - state: `{"message": text}`; question: qid `intent`, instructions
///   "Which banking intent does `message` express?", criteria keys in
///   sorted order, all values null (the `choice_q` law).
///
/// Returns `(cases, classes)` — classes = the sorted label universe.
///
/// # Panics
/// Panics when a surviving row's label is absent from the key set (the
/// reflex builder's own panic law — a universe/join drift is a build
/// bug, not a data condition).
#[must_use]
pub fn banking77_teacher_cases(datasets_dir: &Path) -> (Vec<TeacherCase>, Vec<String>) {
    let raw = read_train_envelope(datasets_dir, "banking77").expect("banking77 train envelope");
    // Option universe over ALL rows: sorted unique label_text (raw form),
    // keyed by the space-joined transform.
    let mut raw_labels: Vec<String> = Vec::new();
    for row in &raw {
        if let Some(t) = row.get("label_text").and_then(Value::as_str) {
            if !raw_labels.iter().any(|l| l == t) {
                raw_labels.push(t.to_string());
            }
        }
    }
    raw_labels.sort();
    let keys: Vec<String> = raw_labels.iter().map(|n| n.replace('_', " ")).collect();

    let mut cases = Vec::new();
    for (pos, row) in raw.iter().enumerate() {
        let (Some(text), Some(label_text)) = (
            row.get("text").and_then(Value::as_str),
            row.get("label_text").and_then(Value::as_str),
        ) else {
            continue;
        };
        let label = label_text.replace('_', " ");
        let gold = keys
            .iter()
            .position(|k| *k == label)
            .unwrap_or_else(|| panic!("banking77 mteb: row label_text {label_text:?} not in key set"));
        // The criteria object's INSERTION order is the option order.
        let mut crit = Map::new();
        for k in &keys {
            crit.insert(k.clone(), Value::Null);
        }
        cases.push(TeacherCase {
            id: format!("banking77:{pos}"),
            state: json!({ "message": text }),
            question: json!({
                "type": "choice",
                "instructions": "Which banking intent does `message` express?",
                "criteria": Value::Object(crit),
            }),
            gold,
            label,
        });
    }
    (cases, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn envelope(rows: Vec<Value>) -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        let suite = dir.path().join("banking77");
        std::fs::create_dir_all(&suite).expect("mkdir");
        let wrapped: Vec<Value> = rows
            .iter()
            .enumerate()
            .map(|(i, r)| json!({ "row_idx": i, "row": r }))
            .collect();
        std::fs::write(
            suite.join("train-00000-of-00001.json"),
            json!({ "rows": wrapped }).to_string(),
        )
        .expect("write");
        dir
    }

    #[test]
    fn case_law_keys_sorted_gold_and_wire_form() {
        let rows = vec![
            json!({ "text": "card blocked", "label": 3, "label_text": "top_up_failed" }),
            json!({ "text": "atm ate my card", "label": 0, "label_text": "atm_support" }),
            json!({ "text": "refund missing", "label": 1, "label_text": "Refund_not_showing_up" }),
        ];
        let dir = envelope(rows);
        let (cases, classes) = banking77_teacher_cases(dir.path());
        // Universe = sorted unique label_text, space-joined (ASCII sort:
        // uppercase before lowercase).
        assert_eq!(
            classes,
            vec!["Refund not showing up", "atm support", "top up failed"]
        );
        assert_eq!(cases.len(), 3);
        // Row order preserved; gold = key position; ids = bank77:<pos>.
        assert_eq!(cases[0].id, "banking77:0");
        assert_eq!(cases[0].label, "top up failed");
        assert_eq!(cases[0].gold, 2);
        assert_eq!(cases[1].gold, 1);
        assert_eq!(cases[2].gold, 0);
        // Wire form: criteria insertion order IS the sorted key order.
        let crit = cases[0].question["criteria"].as_object().expect("crit");
        let key_order: Vec<&String> = crit.keys().collect();
        assert_eq!(key_order, classes.iter().collect::<Vec<_>>());
        assert!(crit.values().all(|v| v.is_null()));
        assert_eq!(
            cases[0].question["instructions"],
            "Which banking intent does `message` express?"
        );
        assert_eq!(cases[0].question["type"], "choice");
        assert_eq!(cases[0].state["message"], "card blocked");
    }

    #[test]
    fn survivor_predicate_skips_rows_without_text_or_label_text() {
        let rows = vec![
            json!({ "text": "ok row", "label": 0, "label_text": "atm_support" }),
            json!({ "label": 1, "label_text": "top_up_failed" }),
            json!({ "text": "no label text", "label": 2 }),
        ];
        let dir = envelope(rows);
        let (cases, classes) = banking77_teacher_cases(dir.path());
        // The universe STILL covers all rows on disk (the builder law),
        // but only the survivor builds a case.
        assert_eq!(classes.len(), 2);
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].label, "atm support");
    }
}

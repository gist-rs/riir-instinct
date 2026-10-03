//! Issue 578 T1 — the `typed_decisions` option-conditioned specialist rows.
//!
//! The five fixed-criteria suites train one SuiteRow per TRAIN row with
//! the class label; a typed_decisions row instead carries a QUESTION SET
//! (choice / score / noul, each with its own option keys) over one
//! structured `state` JSON object. The serving bridge (riir-instinct
//! `arena.rs` `fill_positions`) scores a case's FIRST question among its
//! PRESENTED option keys, resolved to artifact class rows BY NAME. So the
//! training rows here are option-conditioned: one [`SuiteRow`] per
//! scorable (question, gold option) pair —
//!
//! - **label** = the gold OPTION KEY (the reflex `typed_gold_events`
//!   law): choice keys verbatim in criteria-object insertion order, score
//!   levels via the bridge's `Value → String` rendering, noul as the
//!   internal `"no"` / `"yes"` pair (gold `true` → `"yes"`);
//! - **text** = the RAW `state` string.
//!
//! ## The raw-state premise (the feature-law agreement with serving)
//!
//! The arena bags `serialize_state(parse(state))` — reflex's
//! `riir_infer_laya::pyjson::serialize_state`, the Python-`json.dumps`
//! port (default `", "` / `": "` separators, `ensure_ascii=False`,
//! shortest-round-trip float repr). The dataset rows were EMITTED by
//! Python `json.dumps`, so for these bytes the raw string and the
//! re-rendering agree by provenance (same separators, same float spellings,
//! same insertion order) — the trainer consumes the raw string and never
//! re-parses it. The catastrophic-fork class (a VOCAB/salt mismatch)
//! cannot arise here: the bag law is [`crate::instinct_specialist::
//! bag_into`] itself, shared verbatim.
//!
//! ## Parse laws (the reflex `build_typed_decisions` + `typed_gold_events`
//! rules, mirrored)
//!
//! - a malformed `questions`/`gold` string skips the ROW; a question with
//!   no gold entry, an unknown `type`, or an unresolvable gold mapping
//!   skips that QUESTION only;
//! - questions iterate in the parsed dict's INSERTION order (serde_json
//!   `preserve_order` — train-engine now enables it; the first surviving
//!   event is the case's question[0], the bridge's scored question);
//! - the row-level `workflow` filter is deliberately NOT applied (a train
//!   row that would not become a case is still training data — the
//!   `typed_gold_events` precedent).
//!
//! ## Consumer coupling (read before minting a winner)
//!
//! The instinct arena's full-arm path REFUSES suites whose cases carry
//! multi-question sets or noul questions (the shape-law assertion), and
//! `SpecialistLane::join` requires every SEAT label (for typed_decisions:
//! the workflow names) to appear in the artifact universe. An
//! option-key artifact therefore needs the typed consumer bridge before
//! `<suite>_winner_v1.bin` lands — minting the winner name first would
//! error the arena's typed_decisions run entirely (the shape gate is a
//! `return Err`, not an A0 fallback). This module ships the
//! `typed_decisions_armA_v1.bin` artifact; the winner rename belongs to
//! the consumer landing (riir-train Issue 578 T3 + the riir-instinct
//! bridge).

use std::collections::HashMap;
use std::path::Path;

use serde_json::Value;

use crate::instinct_specialist::{
    argmax_lowest_tie, read_train_envelope, bag_into, stratified_holdout, Specialist, SuiteRow,
};

// ── the parse laws ──────────────────────────────────────────────────────

/// The question kinds the reflex `qdef_kind` law admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedKind {
    Choice,
    Score,
    Noul,
}

/// One scorable question of a typed_decisions row: the question's
/// presented option keys in template order (empty for noul — its two
/// positions are the fixed `"no"` / `"yes"` pair) and the gold position.
#[derive(Debug, Clone)]
pub struct TypedEvent {
    pub qid: String,
    pub kind: TypedKind,
    pub keys: Vec<String>,
    pub gold_idx: usize,
}

impl TypedEvent {
    /// The training label — the gold option key (the `typed_gold_events`
    /// law; noul spells `"no"` / `"yes"`, gold `true` → `"yes"`).
    #[must_use]
    pub fn gold_option(&self) -> &str {
        match self.kind {
            TypedKind::Noul => {
                if self.gold_idx == 1 {
                    "yes"
                } else {
                    "no"
                }
            }
            _ => &self.keys[self.gold_idx],
        }
    }

    /// The presented option keys in gold-position space — the bridge's
    /// `fill_positions` law for this question. Noul presents the fixed
    /// `["no", "yes"]` pair (gold idx 0/1 = false/true — the positions the
    /// arena's `gold.idx` speaks; the reflex engine renders the text as
    /// `[false, true]`, and the bridge maps those names onto these class
    /// rows).
    #[must_use]
    pub fn positions(&self) -> Vec<String> {
        match self.kind {
            TypedKind::Noul => vec!["no".to_string(), "yes".to_string()],
            _ => self.keys.clone(),
        }
    }
}

/// One typed_decisions TRAIN row: the workflow (the seat's label space —
/// context only; the specialist's classes are option keys), the RAW state
/// string (the feature text), and the row's scorable events in insertion
/// order.
#[derive(Debug, Clone)]
pub struct TypedRow {
    pub workflow: String,
    pub state: String,
    pub events: Vec<TypedEvent>,
}

impl TypedRow {
    /// One [`SuiteRow`] per scorable event (label = the gold option key,
    /// text = the raw state string).
    #[must_use]
    pub fn event_rows(&self) -> Vec<SuiteRow> {
        self.events
            .iter()
            .map(|e| SuiteRow {
                label: e.gold_option().to_string(),
                text: self.state.clone(),
                presented: None,
            })
            .collect()
    }
}

/// Python `str()` for JSON values (the reflex `py_str` law): strings
/// pass through; bools are short literals; numbers the serde rendering;
/// null the Python `None`.
fn py_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::Null => "None".to_string(),
        other => other.to_string(),
    }
}

/// Python `int()`-style tolerant integer read (the reflex
/// `value_as_i64` law): JSON number or numeric string.
fn value_as_i64(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

/// A dataset column that arrives as a JSON string (or, tolerantly, already
/// parsed); parse failures read Null (the reflex
/// `parse_json_string_or_value` law).
fn parse_json_string_or_value(v: &Value) -> Value {
    match v {
        Value::String(s) => serde_json::from_str(s).unwrap_or(Value::Null),
        other => other.clone(),
    }
}

/// Criteria KEY order for a choice question: object → keys in insertion
/// order; array → elements via the bridge's `Value → String` rendering
/// (`String` verbatim, anything else the compact JSON form) — the
/// reflex `choice_keys` + `fill_positions` laws.
fn choice_keys(crit: &Value) -> Option<Vec<String>> {
    match crit {
        Value::Object(m) => Some(m.keys().cloned().collect()),
        Value::Array(a) => Some(
            a.iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect(),
        ),
        _ => None,
    }
}

/// Score-question levels: the criteria ARRAY via the same rendering.
fn score_levels(crit: &Value) -> Option<Vec<String>> {
    let a = crit.as_array()?;
    Some(
        a.iter()
            .map(|v| match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect(),
    )
}

/// The gold mapping for one question (the reflex `gold_answer` law,
/// position half only): `Option` = the gold position into `keys` — `None`
/// = the question is skipped (no gold entry / unresolvable mapping).
fn gold_idx_of(kind: TypedKind, keys: &[String], g: &Value) -> Option<usize> {
    match kind {
        TypedKind::Choice => {
            let label = py_str(g.get("label")?);
            keys.iter().position(|k| *k == label)
        }
        TypedKind::Noul => {
            let is_true = py_str(g.get("label")?).to_lowercase() == "true";
            Some(usize::from(is_true))
        }
        TypedKind::Score => {
            let label = value_as_i64(g.get("label")?)?;
            let idx = usize::try_from(label).ok()?;
            if idx >= keys.len() {
                None
            } else {
                Some(idx)
            }
        }
    }
}

/// Parse one envelope row into a [`TypedRow`] — `None` = the row is
/// skipped (no `state` string, or malformed `questions`/`gold`).
#[must_use]
pub fn typed_row(row: &Value) -> Option<TypedRow> {
    let workflow = row.get("workflow")?.as_str()?.to_string();
    let state = row.get("state")?.as_str()?.to_string();
    let questions_v = row.get("questions").map(parse_json_string_or_value);
    let gold_v = row.get("gold").map(parse_json_string_or_value);
    let (qmap, gmap) = match (questions_v.as_ref().and_then(Value::as_object), gold_v.as_ref().and_then(Value::as_object)) {
        (Some(q), Some(g)) => (q, g),
        _ => return None,
    };
    let mut events = Vec::new();
    for (qid, qdef) in qmap {
        let Some(kind) = qdef.get("type").and_then(Value::as_str).and_then(|t| match t {
            "choice" => Some(TypedKind::Choice),
            "score" => Some(TypedKind::Score),
            "noul" => Some(TypedKind::Noul),
            _ => None,
        }) else {
            continue;
        };
        let Some(g) = gmap.get(qid) else {
            continue;
        };
        let keys = match kind {
            TypedKind::Noul => Vec::new(),
            TypedKind::Choice => choice_keys(qdef.get("criteria")?)?,
            TypedKind::Score => score_levels(qdef.get("criteria")?)?,
        };
        let Some(idx) = gold_idx_of(kind, &keys, g) else {
            continue;
        };
        events.push(TypedEvent {
            qid: qid.clone(),
            kind,
            keys,
            gold_idx: idx,
        });
    }
    Some(TypedRow {
        workflow,
        state,
        events,
    })
}

/// Load typed_decisions' train rows from a datasets dir — the shared
/// `train-*.json` envelope walk, each raw row through [`typed_row`].
///
/// # Errors
/// The envelope walk's failures (missing dir, unparsable page) and an
/// empty survivor set.
pub fn load_typed_rows(dir: &Path) -> Result<Vec<TypedRow>, String> {
    let raw = read_train_envelope(dir, "typed_decisions")?;
    let mut rows = Vec::new();
    for row in &raw {
        if let Some(r) = typed_row(row) {
            rows.push(r);
        }
    }
    if rows.is_empty() {
        return Err("suite typed_decisions: no rows survived the parse laws".into());
    }
    Ok(rows)
}

/// The label-stratified ROW-level holdout: round-robin rows by WORKFLOW
/// (the reflex `stratified_split` law over the workflow as the bucket
/// label), so every event of a holdout row stays holdout — the serve-time
/// generalization direction (unseen state text), never event-level mixing
/// of one row's questions across the split.
/// Returns `(holdout_row_idxs, train_row_idxs)`.
#[must_use]
pub fn typed_row_holdout(rows: &[TypedRow], budget: usize) -> (Vec<usize>, Vec<usize>) {
    let buckets: Vec<SuiteRow> = rows
        .iter()
        .map(|r| SuiteRow {
            label: r.workflow.clone(),
            text: String::new(),
            presented: None,
        })
        .collect();
    stratified_holdout(&buckets, budget)
}

// ── the evals (both in the serving bridge's option-restricted space) ────

/// One position-space eval's outcome. `unanswered` cases had a presented
/// key (or noul half) the artifact never trained — the serving bridge
/// REFUSES those loudly; here they count as incorrect and are counted.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RestrictedEval {
    pub total: usize,
    pub correct: usize,
    pub unanswered: usize,
}

impl RestrictedEval {
    /// The accuracy over ALL cases (unanswered counts as wrong) — the
    /// number the arena's correctness vector would read.
    #[must_use]
    pub fn accuracy(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            f64::from(u32::try_from(self.correct).unwrap_or(u32::MAX))
                / f64::from(u32::try_from(self.total).unwrap_or(u32::MAX))
        }
    }
}

/// The ARENA-space metric: per row, the FIRST surviving event
/// (question[0]), scored among its presented positions by name, argmax
/// with ties to the lowest POSITION (the bridge's tie law — positions
/// follow the template order, so "lowest position" is "lowest artifact
/// class" only by the mapping, not numerically; the bridge argmaxes
/// position scores, which is what this mirrors).
#[must_use]
pub fn typed_case_eval(model: &Specialist, rows: &[TypedRow], idxs: &[usize]) -> RestrictedEval {
    let mut ev = RestrictedEval {
        total: idxs.len(),
        correct: 0,
        unanswered: 0,
    };
    let mut bag: Vec<(u32, f32)> = Vec::new();
    let mut scratch: Vec<u32> = Vec::new();
    let mut scores = vec![0.0f32; model.labels.len()];
    for &ri in idxs {
        let Some(first) = rows[ri].events.first() else {
            ev.unanswered += 1;
            continue;
        };
        bag_into(rows[ri].state.as_bytes(), &mut bag, &mut scratch);
        model.scores_into(&bag, &mut scores);
        let pick = position_pick(model, first, &scores);
        if pick == usize::MAX {
            ev.unanswered += 1;
        } else if pick == first.gold_idx {
            ev.correct += 1;
        }
    }
    ev
}

/// The event-level restricted metric: every scorable holdout event,
/// scored among its own presented positions.
#[must_use]
pub fn typed_event_eval(model: &Specialist, rows: &[TypedRow], idxs: &[usize]) -> RestrictedEval {
    let mut ev = RestrictedEval {
        total: 0,
        correct: 0,
        unanswered: 0,
    };
    let mut bag: Vec<(u32, f32)> = Vec::new();
    let mut scratch: Vec<u32> = Vec::new();
    let mut scores = vec![0.0f32; model.labels.len()];
    for &ri in idxs {
        bag_into(rows[ri].state.as_bytes(), &mut bag, &mut scratch);
        model.scores_into(&bag, &mut scores);
        for e in &rows[ri].events {
            ev.total += 1;
            let pick = position_pick(model, e, &scores);
            if pick == usize::MAX {
                ev.unanswered += 1;
            } else if pick == e.gold_idx {
                ev.correct += 1;
            }
        }
    }
    ev
}

/// Restrict the model's class scores to the event's presented positions
/// and argmax with ties to the lowest POSITION. Returns `usize::MAX`
/// when a position has no artifact class row (unanswered — counted, never
/// silently dropped).
fn position_pick(model: &Specialist, e: &TypedEvent, scores: &[f32]) -> usize {
    let positions = e.positions();
    let mut pos_scores: Vec<f32> = Vec::with_capacity(positions.len());
    let mut all_known = true;
    for key in &positions {
        match model.labels.iter().position(|l| l == key) {
            Some(ci) => pos_scores.push(scores[ci]),
            None => {
                all_known = false;
                break;
            }
        }
    }
    if !all_known {
        return usize::MAX;
    }
    argmax_lowest_tie(&pos_scores)
}

/// The workflow-majority baseline in the case space: per workflow, the
/// majority gold option of question[0] over the TRAIN rows, predicted for
/// every holdout row of that workflow. Context for the case metric — the
/// dumb-but-honest floor a pure prior reaches without reading any state.
#[must_use]
pub fn typed_case_majority_baseline(
    rows: &[TypedRow],
    train_idxs: &[usize],
    holdout_idxs: &[usize],
) -> RestrictedEval {
    let mut tally: HashMap<(&str, &str), usize> = HashMap::new();
    for &ri in train_idxs {
        if let Some(first) = rows[ri].events.first() {
            *tally
                .entry((rows[ri].workflow.as_str(), first.gold_option()))
                .or_insert(0) += 1;
        }
    }
    let mut majority: HashMap<&str, &str> = HashMap::new();
    for ((wf, opt), n) in &tally {
        match majority.get(wf) {
            Some(cur) if tally.get(&(wf, cur)).copied().unwrap_or(0) >= *n => {}
            _ => {
                majority.insert(wf, opt);
            }
        }
    }
    let mut ev = RestrictedEval {
        total: holdout_idxs.len(),
        correct: 0,
        unanswered: 0,
    };
    for &ri in holdout_idxs {
        let Some(first) = rows[ri].events.first() else {
            ev.unanswered += 1;
            continue;
        };
        if majority.get(rows[ri].workflow.as_str()).copied() == Some(first.gold_option()) {
            ev.correct += 1;
        }
    }
    ev
}

// ── tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instinct_specialist::ArmAConfig;

    fn row_json(state: &str, questions: &str, gold: &str) -> Value {
        serde_json::json!({
            "workflow": "wf",
            "state": state,
            "questions": questions,
            "gold": gold,
        })
    }

    /// The option-spelling law: choice keys verbatim in criteria-object
    /// INSERTION order (NOT sorted order — the pinned defect class),
    /// score levels via the bridge rendering, noul as the no/yes pair.
    #[test]
    fn option_spelling_law() {
        // insertion order deliberately NOT alphabetical: zeta first
        let r = typed_row(&row_json(
            r#"{"x": 1}"#,
            r#"{"pick": {"type": "choice", "instructions": "i", "criteria": {"zeta": "d1", "mid": "d2", "alpha": "d3"}},
                "level": {"type": "score", "instructions": "i", "criteria": ["low", "high"]},
                "flag": {"type": "noul", "instructions": "i"}}"#,
            r#"{"pick": {"label": "mid"}, "level": {"label": 1}, "flag": {"label": true}}"#,
        ))
        .expect("row parses");
        assert_eq!(r.events.len(), 3);
        assert_eq!(r.events[0].keys, vec!["zeta", "mid", "alpha"]);
        assert_eq!(r.events[0].gold_option(), "mid");
        assert_eq!(r.events[1].keys, vec!["low", "high"]);
        assert_eq!(r.events[1].gold_option(), "high");
        assert_eq!(r.events[2].gold_option(), "yes");
        assert_eq!(r.events[2].positions(), vec!["no", "yes"]);
        // gold FALSE noul → "no"
        let r2 = typed_row(&row_json(
            r#"{"x": 1}"#,
            r#"{"flag": {"type": "noul", "instructions": "i"}}"#,
            r#"{"flag": {"label": false}}"#,
        ))
        .unwrap();
        assert_eq!(r2.events[0].gold_option(), "no");
    }

    /// Insertion order IS the label order: a gold label resolving by
    /// position must follow the criteria object's written order, not the
    /// sorted one (the preserve_order premise — this test is the reason
    /// train-engine enables the feature).
    #[test]
    fn insertion_order_is_the_label_order() {
        let r = typed_row(&row_json(
            r#"{"x": 1}"#,
            r#"{"pick": {"type": "choice", "instructions": "i", "criteria": {"zulu": "d", "alpha": "d", "mike": "d"}}}"#,
            r#"{"pick": {"label": "alpha"}}"#, // position 1 in written order
        ))
        .unwrap();
        assert_eq!(r.events[0].gold_idx, 1);
        assert_eq!(r.events[0].gold_option(), "alpha");
    }

    /// The skip laws: malformed `questions` skips the ROW; a question with
    /// no gold entry / unknown type / out-of-range score skips that
    /// QUESTION only.
    #[test]
    fn skip_laws() {
        // malformed questions string → whole row skipped
        assert!(typed_row(&row_json(
            r#"{"x": 1}"#,
            "{not json",
            r#"{}"#,
        ))
        .is_none());
        // missing state → row skipped
        let v = serde_json::json!({"workflow": "wf", "questions": "{}", "gold": "{}"});
        assert!(typed_row(&v).is_none());
        // mixed: one good question + one with no gold + one unknown type
        let r = typed_row(&row_json(
            r#"{"x": 1}"#,
            r#"{"good": {"type": "choice", "instructions": "i", "criteria": {"a": "d", "b": "d"}},
                "nogold": {"type": "choice", "instructions": "i", "criteria": {"a": "d", "b": "d"}},
                "weird": {"type": "essay", "instructions": "i"}}"#,
            r#"{"good": {"label": "b"}}"#,
        ))
        .unwrap();
        assert_eq!(r.events.len(), 1);
        assert_eq!(r.events[0].qid, "good");
        // score out of range → question skipped
        let r2 = typed_row(&row_json(
            r#"{"x": 1}"#,
            r#"{"lvl": {"type": "score", "instructions": "i", "criteria": ["a", "b"]}}"#,
            r#"{"lvl": {"label": 5}}"#,
        ))
        .unwrap();
        assert!(r2.events.is_empty());
    }

    /// A zero-weight specialist ties every position to 0 → the argmax law
    /// picks the LOWEST position; a bias on one class moves the pick; a
    /// presented key outside the artifact universe reads unanswered
    /// (`usize::MAX` → counted incorrect).
    #[test]
    fn case_eval_restriction_and_tie_law() {
        let labels = vec!["alpha".to_string(), "zeta".to_string()];
        let model = Specialist::zero(&labels);
        let rows = vec![
            typed_row(&row_json(
                r#"{"x": 1}"#,
                r#"{"pick": {"type": "choice", "instructions": "i", "criteria": {"zeta": "d", "alpha": "d"}}}"#,
                r#"{"pick": {"label": "alpha"}}"#,
            ))
            .unwrap(),
        ];
        // all-zero scores → tie → position 0 ("zeta") ≠ gold (position 1)
        let ev = typed_case_eval(&model, &rows, &[0]);
        assert_eq!((ev.total, ev.correct), (1, 0));
        // bias alpha's class row (labels[0]) → its position (1) wins == gold
        let mut biased = Specialist::zero(&labels);
        biased.b[0] = 0.5;
        let ev = typed_case_eval(&biased, &rows, &[0]);
        assert_eq!((ev.total, ev.correct, ev.unanswered), (1, 1, 0));
        // an unknown presented key → unanswered
        let rows_unknown = vec![
            typed_row(&row_json(
                r#"{"x": 1}"#,
                r#"{"pick": {"type": "choice", "instructions": "i", "criteria": {"omega": "d", "alpha": "d"}}}"#,
                r#"{"pick": {"label": "alpha"}}"#,
            ))
            .unwrap(),
        ];
        let ev = typed_case_eval(&biased, &rows_unknown, &[0]);
        assert_eq!((ev.total, ev.correct, ev.unanswered), (1, 0, 1));
    }

    /// The row-level holdout keeps every event of a holdout row on the
    /// holdout side (the serve-time generalization direction).
    #[test]
    fn holdout_is_row_level() {
        let mk = |wf: &str, i: usize| TypedRow {
            workflow: wf.to_string(),
            state: format!(r#"{{"i": {i}}}"#),
            events: vec![TypedEvent {
                qid: "pick".into(),
                kind: TypedKind::Choice,
                keys: vec!["a".into(), "b".into()],
                gold_idx: i % 2,
            }],
        };
        let rows: Vec<TypedRow> = (0..12)
            .map(|i| mk(if i % 2 == 0 { "w0" } else { "w1" }, i))
            .collect();
        let (holdout, train) = typed_row_holdout(&rows, 4);
        assert_eq!(holdout.len() + train.len(), 12);
        assert!(!holdout.is_empty());
        // disjoint and complete
        let mut seen = [false; 12];
        for &i in holdout.iter().chain(train.iter()) {
            seen[i] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    /// End-to-end smoke through the real trainer: a three-workflow toy
    /// corpus with linearly separable option structure must beat the lr0
    /// control on both restricted metrics.
    #[test]
    fn trainer_learns_separable_options_and_beats_the_control() {
        // workflow A: gold "alpha" carries the SHARED marker "aa";
        // workflow B: gold "zeta" carries "zz". The marker token is the
        // only signal and is shared across rows — the holdout generalizes
        // through it (unique per-row markers would be unlearnable by
        // construction and only tie the control).
        let mk = |wf: &str, i: usize| {
            let (marker, gold) = if wf == "w0" {
                ("aa", "alpha")
            } else {
                ("zz", "zeta")
            };
            TypedRow {
                workflow: wf.to_string(),
                state: format!(r#"{{"m": "{marker}", "i": {i}}}"#),
                events: vec![TypedEvent {
                    qid: "pick".into(),
                    kind: TypedKind::Choice,
                    keys: vec!["alpha".into(), "zeta".into()],
                    gold_idx: usize::from(gold == "zeta"),
                }],
            }
        };
        let rows: Vec<TypedRow> = (0..24)
            .map(|i| mk(if i % 2 == 0 { "w0" } else { "w1" }, i))
            .collect();
        let (holdout_rows, train_rows) = typed_row_holdout(&rows, 8);
        let mut all: Vec<SuiteRow> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        for &i in train_rows.iter().chain(holdout_rows.iter()) {
            for sr in rows[i].event_rows() {
                if !labels.contains(&sr.label) {
                    labels.push(sr.label.clone());
                }
                all.push(sr);
            }
        }
        labels.sort();
        let n_train = train_rows.len();
        let train_idx: Vec<usize> = (0..n_train).collect();
        let holdout_idx: Vec<usize> = (n_train..all.len()).collect();
        let cfg = ArmAConfig {
            epochs: 5,
            ..ArmAConfig::default()
        };
        let run = crate::instinct_specialist::train_arm_a(
            &all,
            &labels,
            &train_idx,
            &holdout_idx,
            &cfg,
        )
        .expect("trains");
        assert!(run.holdout_accuracy > run.control_accuracy);
        let case = typed_case_eval(&run.model, &rows, &holdout_rows);
        assert!(case.accuracy() > 0.5, "case acc {}", case.accuracy());
    }

    /// The distractor-universe law (the measured customer_service
    /// `close_no_action` class): a key that is PRESENTED but never gold
    /// in train still joins the label universe (the example extends it
    /// with presented keys), and after training it must NOT dominate —
    /// the dense per-class negative exposure pushes its bias below the
    /// gold classes'. A never-trained filler would instead sit at
    /// zero-init and win every case.
    #[test]
    fn never_gold_presented_keys_train_negative_and_cannot_dominate() {
        // w0 rows: gold "alpha" marked by "aa"; every question ALSO
        // presents "dead", which is never gold anywhere.
        let mk = |i: usize| TypedRow {
            workflow: "w0".to_string(),
            state: format!(r#"{{"m": "aa", "i": {i}}}"#),
            events: vec![TypedEvent {
                qid: "pick".into(),
                kind: TypedKind::Choice,
                keys: vec!["dead".into(), "alpha".into()],
                gold_idx: 1,
            }],
        };
        let rows: Vec<TypedRow> = (0..16).map(mk).collect();
        let (holdout_rows, train_rows) = typed_row_holdout(&rows, 4);
        let mut train_events: Vec<SuiteRow> = Vec::new();
        for &i in &train_rows {
            train_events.extend(rows[i].event_rows());
        }
        let mut labels = vec!["dead".to_string(), "alpha".to_string()];
        labels.sort();
        let n_train = train_events.len();
        let mut all = train_events;
        for &i in &holdout_rows {
            all.extend(rows[i].event_rows());
        }
        let train_idx: Vec<usize> = (0..n_train).collect();
        let holdout_idx: Vec<usize> = (n_train..all.len()).collect();
        let cfg = ArmAConfig {
            epochs: 4,
            ..ArmAConfig::default()
        };
        let run = crate::instinct_specialist::train_arm_a(
            &all,
            &labels,
            &train_idx,
            &holdout_idx,
            &cfg,
        )
        .expect("trains");
        // The dead class trained negative: its bias sits below the gold
        // class's, so every holdout case answers "alpha" (the gold).
        let dead_idx = run.model.labels.iter().position(|l| l == "dead").unwrap();
        let alpha_idx = run.model.labels.iter().position(|l| l == "alpha").unwrap();
        assert!(run.model.b[dead_idx] < run.model.b[alpha_idx]);
        let case = typed_case_eval(&run.model, &rows, &holdout_rows);
        assert_eq!(
            case.unanswered, 0,
            "the presented-but-never-gold key left cases unanswered"
        );
        assert!(
            case.accuracy() > 0.5,
            "dead key dominated: case acc {}",
            case.accuracy()
        );
    }

    /// The majority baseline on a known answer: workflow w0's majority
    /// gold is "alpha" (2 of 3), so a holdout row of w0 with gold
    /// "alpha" scores and one with "zeta" does not.
    #[test]
    fn majority_baseline_follows_the_train_tally() {
        let mk = |wf: &str, gold: &str| TypedRow {
            workflow: wf.to_string(),
            state: r#"{"x": 1}"#.into(),
            events: vec![TypedEvent {
                qid: "pick".into(),
                kind: TypedKind::Choice,
                keys: vec!["alpha".into(), "zeta".into()],
                gold_idx: usize::from(gold == "zeta"),
            }],
        };
        let rows = vec![
            mk("w0", "alpha"),
            mk("w0", "alpha"),
            mk("w0", "zeta"),
            mk("w0", "alpha"),
        ];
        let ev = typed_case_majority_baseline(&rows, &[0, 1, 2], &[3]);
        assert_eq!((ev.total, ev.correct), (1, 1));
        let rows2 = vec![mk("w0", "alpha"), mk("w0", "zeta"), mk("w0", "zeta"), mk("w0", "alpha")];
        let ev2 = typed_case_majority_baseline(&rows2, &[0, 1, 2], &[3]);
        assert_eq!((ev2.total, ev2.correct), (1, 0));
    }
}

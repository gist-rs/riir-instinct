//! riir-train 632 T3 — the Eq-9 typed head lane: the dual-shuffle
//! distilled choice head plus the hard-label noul / per-qid score /
//! coverage heads, trained over `typed_decisions` (Issue 629 pattern).
//!
//! Data laws (each refused loud on drift, never normalized away):
//! - **choice** — rows are ALL typed train choice events (the test seat's
//!   choice keys are a subset of the train universe, measured 2026-10-16).
//!   The 1200 coverage-corpus present rows carry per-row soft targets from
//!   the rho artifact's keyed conditionals (projected onto the class
//!   universe, non-presented classes exactly 0.0) blended with gold under
//!   the DUAL-SHUFFLE qualification (`gold_mix` [`GOLD_MIX_QUALIFIED`]
//!   qualified / 1.0 rejected — the 7:3 teacher:gold blend); every other
//!   choice event trains pure gold. `gold_mix == 1.0` is bit-identical to
//!   the `Targets::Gold` row (pinned by test).
//! - **noul** — one head over every noul event, class space the fixed
//!   `["no", "yes"]` pair, pure Brier `λ = 1.0` (the `(p−y)²` shape; the
//!   paper's margin hinge is skipped, disclosed in the read doc).
//! - **score** — one head per score qid (the level-description universes
//!   differ per qid: risk/urgency/churn_risk/discrepancy_severity 4,
//!   severity 5), plain BCE. Aux eval: normalized-expectation MAE +
//!   gold-level sorted-cumulative-mass (reported, never a training loss).
//! - **coverage** — the Eq-9 coverage half on the corpus's c_y gold-
//!   presence labels (road (c) — the ρ column is analysis-only per T2b),
//!   text = state + instructions + rendered criteria (the only arms-
//!   distinguishing signal), 2-class, AUROC on a row-stratified holdout.
//!
//! Text law for every PICK head: the raw dataset state string VERBATIM
//! (the [`crate::instinct_specialist_typed`] `event_rows` law) — train and
//! read see byte-identical features, no fixture re-serialization skew.
//!
//! Trainer: the ONE shared [`crate::instinct_specialist::train_core`]
//! (lazy-AdamW one-vs-all logistic BCE) — never a second copy of the
//! schedule/Adam laws.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use serde_json::Value;

use crate::instinct_specialist::{
    bag_into, read_train_envelope, stratified_holdout, train_core, ArmAConfig, ArmARun, Specialist,
    SuiteRow, Targets, TeacherDump,
};
use crate::instinct_specialist_typed::{choice_keys, py_str, typed_row, TypedKind, TypedRow};

/// The qualified row's gold share of the 7:3 teacher:gold blend (the
/// issue's "teacher soft targets blended 7:3 with hard gold").
pub const GOLD_MIX_QUALIFIED: f32 = 0.3;

/// The fixed 2-class coverage vocabulary (sorted — the class-index law).
pub const COVERAGE_LABELS: [&str; 2] = ["absent", "covered"];

// ── sidecar-verified JSON artifact loading ───────────────────────────────

/// Read one BLAKE3-sealed JSON artifact: the sidecar (`<path>.blake3`,
/// hex text) is verified against the exact file bytes BEFORE any parse —
/// a drifted artifact refuses loud. Returns the parsed doc + the digest
/// (callers pin cross-artifact generations against it).
fn read_sealed_json(path: &Path) -> Result<(Value, String), String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let side = std::path::PathBuf::from(format!("{}.blake3", path.display()));
    let expected = std::fs::read_to_string(&side)
        .map_err(|e| format!("read {}: {e} — the artifact is unsealed", side.display()))?
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string();
    let actual = blake3::hash(&bytes).to_hex().to_string();
    if actual != expected {
        return Err(format!(
            "{}: BLAKE3 seal mismatch (expected {expected}, got {actual})",
            path.display()
        ));
    }
    let doc = serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok((doc, actual))
}

// ── the typed pool with row ids ─────────────────────────────────

/// The typed train pool WITH the envelope row id — `typed_row` parses each
/// envelope row (the one parse law) and the row's `id` field rides beside
/// it (the coverage corpus's `row_id` IS this id; `TypedRow` itself drops
/// it, so the join pairs here before the skip filter can misalign a zip).
fn load_typed_rows_with_ids(dir: &Path) -> Result<Vec<(String, TypedRow)>, String> {
    let raw = read_train_envelope(dir, "typed_decisions")?;
    let mut rows = Vec::new();
    for row in &raw {
        let Some(id) = row.get("id").and_then(Value::as_str) else {
            continue;
        };
        if let Some(r) = typed_row(row) {
            rows.push((id.to_string(), r));
        }
    }
    if rows.is_empty() {
        return Err("suite typed_decisions: no rows survived the parse laws".into());
    }
    Ok(rows)
}

// ── the choice half (Eq-9 distilled) ────────────────────────────────────

/// One choice training row's identity — the join surface between the
/// corpus artifacts and the typed train pool, kept for the read doc.
#[derive(Debug, Clone)]
pub struct ChoiceRowMeta {
    /// The coverage-corpus row id (`Some` iff this row IS a corpus row —
    /// the only rows that carry teacher targets).
    pub corpus_id: Option<String>,
    pub row_id: String,
    pub qid: String,
    /// Presented keys in template (insertion) order — the pick space.
    pub presented: Vec<String>,
    pub gold_label: String,
}

/// The Eq-9 choice training set: every typed train choice event plus the
/// corpus rows' projected teacher targets and per-row gold mixes.
#[derive(Debug)]
pub struct Eq9ChoiceData {
    pub rows: Vec<SuiteRow>,
    /// Dense row-major soft targets over [`Eq9ChoiceData::universe`];
    /// non-corpus rows are all-zero (their target is pure gold via
    /// `gold_mix == 1.0`, so the zero slice is never read with weight).
    pub teacher: TeacherDump,
    pub gold: Vec<usize>,
    pub gold_mix: Vec<f32>,
    pub meta: Vec<ChoiceRowMeta>,
    /// Sorted distinct choice keys over the train pool (the class space).
    pub universe: Vec<String>,
    pub n_corpus_rows: usize,
    pub n_qualified: usize,
}

/// Load the Eq-9 choice data.
///
/// * `datasets` — the typed dataset dir (`typed_decisions/train-*.json`).
/// * `coverage` / `rho` / `dual_shuffle` — the sealed corpus artifacts.
///   `dual_shuffle = None` is the PRE-CAPTURE dev posture: every corpus
///   row trains pure gold (`gold_mix 1.0`) — an upper-bound arm, disclosed
///   by the caller; the qualification file is the real lane.
pub fn load_choice_eq9(
    datasets: &Path,
    coverage: &Path,
    rho: &Path,
    dual_shuffle: Option<&Path>,
) -> Result<Eq9ChoiceData, String> {
    let (cov_doc, cov_digest) = read_sealed_json(coverage)?;

    // The corpus: present-arm rows keyed by (row_id, qid).
    let mut corpus: HashMap<(String, String), &Value> = HashMap::new();
    for r in cov_doc
        .get("rows")
        .and_then(Value::as_array)
        .ok_or("coverage artifact: no rows array")?
    {
        if r.get("arm").and_then(Value::as_str) != Some("present") {
            continue;
        }
        let key = (
            r.get("row_id")
                .and_then(Value::as_str)
                .ok_or("corpus row: no row_id")?
                .to_string(),
            r.get("qid")
                .and_then(Value::as_str)
                .ok_or("corpus row: no qid")?
                .to_string(),
        );
        if corpus.insert(key.clone(), r).is_some() {
            return Err(format!("coverage corpus: duplicate present row {key:?}"));
        }
    }
    let n_corpus_expected = corpus.len();

    // The rho artifact: keyed conditionals per corpus id + the generation
    // pin (its input_blake3 must equal the coverage rows' digest).
    let (rho_doc, _) = read_sealed_json(rho)?;
    let rho_input = rho_doc
        .get("summary")
        .and_then(|s| s.get("input_blake3"))
        .and_then(Value::as_str)
        .ok_or("rho artifact: summary.input_blake3 missing")?;
    if rho_input != cov_digest {
        return Err(format!(
            "rho artifact was captured against a DIFFERENT coverage generation \
             ({rho_input} != {cov_digest})"
        ));
    }
    let mut rho_by_id: HashMap<String, BTreeMap<String, f64>> = HashMap::new();
    for r in rho_doc
        .get("rows")
        .and_then(Value::as_array)
        .ok_or("rho artifact: no rows array")?
    {
        if r.get("arm").and_then(Value::as_str) != Some("present") {
            continue;
        }
        let id = r
            .get("id")
            .and_then(Value::as_str)
            .ok_or("rho row: no id")?;
        let cond = r
            .get("conditional_probabilities")
            .and_then(Value::as_object)
            .ok_or_else(|| format!("rho row {id}: no conditional_probabilities"))?;
        let mut m = BTreeMap::new();
        for (k, v) in cond {
            let v = v
                .as_f64()
                .ok_or_else(|| format!("rho row {id}: non-numeric p({k})"))?;
            m.insert(k.clone(), v);
        }
        if rho_by_id.insert(id.to_string(), m).is_some() {
            return Err(format!("rho artifact: duplicate present row {id}"));
        }
    }
    if rho_by_id.len() != n_corpus_expected {
        return Err(format!(
            "rho artifact: {} present rows vs corpus {} — set equality is the join law",
            rho_by_id.len(),
            n_corpus_expected
        ));
    }

    // The dual-shuffle qualification (optional — the pre-capture posture).
    let mut qualified_by_id: Option<HashMap<String, bool>> = None;
    if let Some(ds) = dual_shuffle {
        let (ds_doc, _) = read_sealed_json(ds)?;
        let s = ds_doc
            .get("summary")
            .ok_or("dual-shuffle artifact: no summary")?;
        let pinned = s
            .get("inputs")
            .and_then(|i| i.get("coverage_rows_blake3"))
            .and_then(Value::as_str)
            .ok_or("dual-shuffle summary: no coverage_rows_blake3")?;
        if pinned != cov_digest {
            return Err(format!(
                "dual-shuffle artifact pins a DIFFERENT coverage generation \
                 ({pinned} != {cov_digest})"
            ));
        }
        // The rule string names the blend the capture ran under; this lane
        // consumes 7:3 teacher:gold. A different shape means the artifact
        // was written by a different gate generation — refuse, never guess.
        let rule = s
            .get("blend_rule")
            .and_then(Value::as_str)
            .ok_or("dual-shuffle summary: no blend_rule")?;
        if !rule.contains("7:3") {
            return Err("dual-shuffle summary: blend_rule is not the 7:3 shape".into());
        }
        let mut q = HashMap::new();
        for r in ds_doc
            .get("rows")
            .and_then(Value::as_array)
            .ok_or("dual-shuffle artifact: no rows array")?
        {
            let id = r.get("id").and_then(Value::as_str).ok_or("ds row: no id")?;
            let is_q = r
                .get("qualified")
                .and_then(Value::as_bool)
                .ok_or("ds row: no qualified")?;
            if q.insert(id.to_string(), is_q).is_some() {
                return Err(format!("dual-shuffle artifact: duplicate row {id}"));
            }
        }
        if q.len() != n_corpus_expected {
            return Err(format!(
                "dual-shuffle artifact: {} rows vs corpus {} — every corpus row must \
                 be qualified or rejected, none missing",
                q.len(),
                n_corpus_expected
            ));
        }
        qualified_by_id = Some(q);
    }

    // The typed train pool: every choice event is a row; corpus rows join
    // by (row_id = the envelope id, qid) and carry the teacher + mix.
    let pool = load_typed_rows_with_ids(datasets)?;
    let mut keys: BTreeSet<String> = BTreeSet::new();
    let mut events: Vec<(String, String, String, Vec<String>, String)> = Vec::new();
    for (row_id, tr) in &pool {
        for e in &tr.events {
            if e.kind != TypedKind::Choice {
                continue;
            }
            keys.extend(e.keys.iter().cloned());
            events.push((
                tr.state.clone(),
                row_id.clone(),
                e.qid.clone(),
                e.keys.clone(),
                e.gold_option().to_string(),
            ));
        }
    }
    let universe: Vec<String> = keys.into_iter().collect();
    let mut class_of: HashMap<&str, usize> = HashMap::with_capacity(universe.len());
    for (i, k) in universe.iter().enumerate() {
        class_of.insert(k.as_str(), i);
    }
    let n_classes = universe.len();

    let mut rows = Vec::with_capacity(events.len());
    let mut meta = Vec::with_capacity(events.len());
    let mut gold = Vec::with_capacity(events.len());
    let mut gold_mix = Vec::with_capacity(events.len());
    let mut q_mat = Vec::with_capacity(events.len() * n_classes);
    let mut picks = Vec::with_capacity(events.len());
    let mut n_corpus = 0usize;
    let mut n_qualified = 0usize;
    for (state, row_id, qid, presented, gold_label) in &events {
        let mut mix = 1.0f32;
        let mut corpus_id = None;
        if let Some(cr) = corpus.get(&(row_id.clone(), qid.clone())) {
            let id = cr
                .get("id")
                .and_then(Value::as_str)
                .ok_or("corpus row: no id")?
                .to_string();
            // The corpus gold must equal the pool's own parse — a drift is
            // a generation bug, never a silent relabel.
            let corpus_gold = cr
                .get("gold_label")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("corpus row {id}: no gold_label"))?;
            if corpus_gold != gold_label {
                return Err(format!(
                    "corpus row {id}: gold {corpus_gold:?} != the pool's {gold_label:?}"
                ));
            }
            // Corpus presented keys must equal the pool's — the corpus was
            // synthesized FROM this pool; a divergence is a regeneration.
            let corpus_keys = choice_keys(
                cr.get("criteria")
                    .ok_or_else(|| format!("corpus row {id}: no criteria"))?,
            )
            .ok_or_else(|| format!("corpus row {id}: bad criteria"))?;
            if corpus_keys != *presented {
                return Err(format!(
                    "corpus row {id}: presented keys drifted from the pool row"
                ));
            }
            let cond = rho_by_id
                .get(&id)
                .ok_or_else(|| format!("corpus row {id}: no teacher conditionals"))?;
            let mut prow: Vec<f32> = vec![0.0f32; n_classes];
            let mut mass = 0.0f32;
            for k in presented {
                let Some(&c) = class_of.get(k.as_str()) else {
                    return Err(format!(
                        "corpus row {id}: presented key {k:?} outside the universe"
                    ));
                };
                let v = *cond.get(k).ok_or_else(|| {
                    format!("corpus row {id}: teacher missing presented key {k:?}")
                })?;
                let v = v as f32;
                mass += v;
                prow[c] = v;
            }
            if !(0.999..=1.001).contains(&mass) {
                return Err(format!(
                    "corpus row {id}: projected teacher mass {mass} outside [0.999, 1.001]"
                ));
            }
            let best = prow
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0);
            picks.push(best as u32);
            q_mat.extend_from_slice(&prow);
            if let Some(qmap) = &qualified_by_id {
                if *qmap
                    .get(&id)
                    .ok_or_else(|| format!("corpus row {id}: no qualification"))?
                {
                    mix = GOLD_MIX_QUALIFIED;
                    n_qualified += 1;
                }
            }
            corpus_id = Some(id);
            n_corpus += 1;
        } else {
            picks.push(0);
            q_mat.extend(std::iter::repeat_n(0.0f32, n_classes));
        }
        let c = *class_of.get(gold_label.as_str()).ok_or_else(|| {
            format!("row {row_id}/{qid}: gold {gold_label:?} outside the universe")
        })?;
        rows.push(SuiteRow {
            label: gold_label.clone(),
            text: state.clone(),
            presented: Some(presented.clone()),
        });
        meta.push(ChoiceRowMeta {
            corpus_id,
            row_id: row_id.clone(),
            qid: qid.clone(),
            presented: presented.clone(),
            gold_label: gold_label.clone(),
        });
        gold.push(c);
        gold_mix.push(mix);
    }
    if n_corpus != n_corpus_expected {
        return Err(format!(
            "join miss: {n_corpus} of {n_corpus_expected} corpus rows matched a train event — \
             the corpus and the pool are different generations"
        ));
    }
    if q_mat.len() != rows.len() * n_classes {
        return Err("teacher matrix shape mismatch".into());
    }
    Ok(Eq9ChoiceData {
        rows,
        teacher: TeacherDump {
            classes: universe.clone(),
            gold: gold.iter().map(|&g| g as u32).collect(),
            picks,
            q: q_mat,
        },
        gold,
        gold_mix,
        meta,
        universe,
        n_corpus_rows: n_corpus,
        n_qualified,
    })
}

/// Train the Eq-9 choice head: the shared trainer core over the per-row
/// mix targets.
pub fn train_choice_eq9(
    data: &Eq9ChoiceData,
    train_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &ArmAConfig,
) -> Result<ArmARun, String> {
    train_core(
        &data.rows,
        &data.universe,
        train_idx,
        holdout_idx,
        cfg,
        Targets::DistillRowMix {
            q: &data.teacher,
            gold: data.gold.clone(),
            gold_mix: data.gold_mix.clone(),
        },
    )
}

/// The presented-restricted pick (the serving bridge's option-restricted
/// space): argmax over the presented classes only, ties to the LOWEST
/// PRESENTED position (the engine argmax law in pick space). `None` = no
/// presented key is in the class universe (unanswerable — counted
/// incorrect by the evals, never silently dropped).
pub fn pick_restricted(
    model: &Specialist,
    bag: &[(u32, f32)],
    universe: &[String],
    presented: &[String],
) -> Option<(usize, f32)> {
    if bag.is_empty() || universe.is_empty() {
        return None;
    }
    let mut scores = vec![0.0f32; universe.len()];
    model.scores_into(bag, &mut scores);
    let mut best: Option<(usize, f32)> = None;
    for k in presented {
        let Some(ci) = universe.iter().position(|u| u == k) else {
            continue;
        };
        let s = scores[ci];
        match best {
            // Ties keep the EARLIER presented position (the engine argmax
            // law in pick space); a strictly higher score replaces.
            Some((_, bs)) if s <= bs => {}
            _ => best = Some((ci, s)),
        }
    }
    best
}

/// Restricted question-level eval over row indices: hit = the restricted
/// pick's label equals the row's gold label. Unanswerable rows count as
/// misses and are counted (`unanswered`). Returns `(accuracy, unanswered)`.
pub fn restricted_eval(
    model: &Specialist,
    rows: &[SuiteRow],
    universe: &[String],
    idxs: &[usize],
) -> (f64, usize) {
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    let mut hit = 0usize;
    let mut unanswered = 0usize;
    for &i in idxs {
        bag.clear();
        bag_into(rows[i].text.as_bytes(), &mut bag, &mut scratch);
        let Some((ci, _)) = pick_restricted(
            model,
            &bag,
            universe,
            rows[i].presented.as_deref().unwrap_or(&[]),
        ) else {
            unanswered += 1;
            continue;
        };
        if universe[ci] == rows[i].label {
            hit += 1;
        }
    }
    (hit as f64 / idxs.len().max(1) as f64, unanswered)
}

// ── the coverage half (Eq-9 BCE on c_y, road (c)) ────────────────────────

/// The coverage training set: both arms of the corpus, `c_y` as the hard
/// label, text = state + instructions + rendered criteria.
pub struct CoverageData {
    pub rows: Vec<SuiteRow>,
    pub labels: Vec<String>,
    pub gold: Vec<usize>,
    /// (id, c_y) per row, corpus order — the read doc's provenance.
    pub meta: Vec<(String, u8)>,
}

/// Load the coverage arm from the sealed corpus. Text = state, then the
/// instructions, then one `key: description` line per criteria key in
/// presented order — the only arms-distinguishing signal (absent rows
/// drop the gold key).
pub fn load_coverage_eq9(coverage: &Path) -> Result<CoverageData, String> {
    let (doc, _) = read_sealed_json(coverage)?;
    let arr = doc
        .get("rows")
        .and_then(Value::as_array)
        .ok_or("coverage artifact: no rows array")?;
    let mut rows = Vec::with_capacity(arr.len());
    let mut gold = Vec::with_capacity(arr.len());
    let mut meta = Vec::with_capacity(arr.len());
    for r in arr {
        let id = r
            .get("id")
            .and_then(Value::as_str)
            .ok_or("corpus row: no id")?;
        let c_y = r
            .get("c_y")
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("row {id}: no c_y"))?;
        if c_y > 1 {
            return Err(format!("row {id}: c_y {c_y} not binary"));
        }
        let state = r
            .get("state")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("row {id}: no state"))?;
        let instructions = r
            .get("instructions")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("row {id}: no instructions"))?;
        let crit = r
            .get("criteria")
            .ok_or_else(|| format!("row {id}: no criteria"))?;
        let keys = choice_keys(crit).ok_or_else(|| format!("row {id}: criteria not keyable"))?;
        let mut text = String::with_capacity(state.len() + instructions.len() + 64 * keys.len());
        text.push_str(state);
        text.push('\n');
        text.push_str(instructions);
        for k in &keys {
            let desc = match crit.get(k) {
                Some(Value::String(s)) => s.clone(),
                Some(other) => py_str(other),
                None => String::new(),
            };
            text.push('\n');
            text.push_str(k);
            text.push_str(": ");
            text.push_str(&desc);
        }
        rows.push(SuiteRow {
            label: COVERAGE_LABELS[c_y as usize].to_string(),
            text,
            presented: None,
        });
        gold.push(c_y as usize);
        meta.push((id.to_string(), c_y as u8));
    }
    if rows.is_empty() {
        return Err("coverage corpus: zero rows".into());
    }
    Ok(CoverageData {
        labels: COVERAGE_LABELS.iter().map(|s| (*s).to_string()).collect(),
        rows,
        gold,
        meta,
    })
}

/// AUROC via the pairwise Mann-Whitney statistic with 0.5 ties — the
/// coverage head's holdout metric (the T2b floor's own metric shape).
pub fn auroc(pos: &[f32], neg: &[f32]) -> Option<f64> {
    if pos.is_empty() || neg.is_empty() {
        return None;
    }
    let mut sum = 0.0f64;
    for &p in pos {
        for &n in neg {
            sum += match p.partial_cmp(&n) {
                Some(std::cmp::Ordering::Greater) => 1.0,
                Some(std::cmp::Ordering::Equal) => 0.5,
                _ => 0.0,
            };
        }
    }
    Some(sum / (pos.len() * neg.len()) as f64)
}

/// The coverage head's holdout AUROC: score = the `covered` class's
/// sigmoid (gold 1), label = the row's c_y.
pub fn coverage_holdout_auroc(
    model: &Specialist,
    data: &CoverageData,
    idxs: &[usize],
) -> Option<f64> {
    let cov_ci = data
        .labels
        .iter()
        .position(|l| l == "covered")
        .expect("coverage vocabulary carries 'covered'");
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    let mut scores_buf = vec![0.0f32; data.labels.len()];
    let mut pos = Vec::new();
    let mut neg = Vec::new();
    for &i in idxs {
        bag.clear();
        bag_into(data.rows[i].text.as_bytes(), &mut bag, &mut scratch);
        if bag.is_empty() {
            continue;
        }
        model.scores_into(&bag, &mut scores_buf);
        if data.gold[i] == 1 {
            pos.push(scores_buf[cov_ci]);
        } else {
            neg.push(scores_buf[cov_ci]);
        }
    }
    auroc(&pos, &neg)
}

// ── the noul + score halves (hard-label) ────────────────────────────────

/// The noul + score heads' training data: rows plus the per-row case
/// stratum (the envelope row id — a case's questions never split across a
/// holdout, the house row-level law).
#[derive(Debug, Clone, Default)]
pub struct ScoreHeadRows {
    pub rows: Vec<SuiteRow>,
    pub strata: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct HardLabelHeads {
    pub noul_rows: Vec<SuiteRow>,
    pub noul_strata: Vec<String>,
    pub score: BTreeMap<String, ScoreHeadRows>,
}

/// Every typed train event flattened to the [`SuiteRow`] shape a
/// [`crate::instinct_specialist::train_arm_a`] run consumes, keyed by
/// kind. Text = the raw state string (the house `event_rows` law);
/// `presented` = the event's pick space (noul carries the fixed
/// `"no" / "yes"` pair via `TypedEvent::positions`).
pub fn hard_label_rows(datasets: &Path) -> Result<HardLabelHeads, String> {
    let pool = load_typed_rows_with_ids(datasets)?;
    let mut out = HardLabelHeads::default();
    for (row_id, tr) in &pool {
        for e in &tr.events {
            let row = SuiteRow {
                label: e.gold_option().to_string(),
                text: tr.state.clone(),
                presented: Some(e.positions()),
            };
            match e.kind {
                TypedKind::Noul => {
                    out.noul_rows.push(row);
                    out.noul_strata.push(row_id.clone());
                }
                TypedKind::Score => {
                    let head = out.score.entry(e.qid.clone()).or_default();
                    head.rows.push(row);
                    head.strata.push(row_id.clone());
                }
                TypedKind::Choice => {}
            }
        }
    }
    Ok(out)
}

/// The score head's aux evals on holdout rows: the presented scores are
/// L1-normalized into a distribution over the level positions;
/// `(normalized-expectation MAE, mean CDM at the gold level)` — the CDM
/// (sorted-cumulative-mass) calibration readout. `NaN` when no row
/// evaluates.
pub fn score_aux_eval(
    model: &Specialist,
    rows: &[SuiteRow],
    universe: &[String],
    idxs: &[usize],
) -> (f64, f64) {
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    let mut scores_buf = vec![0.0f32; universe.len()];
    let mut mae_sum = 0.0f64;
    let mut cdm_sum = 0.0f64;
    let mut n = 0usize;
    for &i in idxs {
        bag.clear();
        bag_into(rows[i].text.as_bytes(), &mut bag, &mut scratch);
        if bag.is_empty() {
            continue;
        }
        let presented = rows[i].presented.as_deref().unwrap_or(&[]);
        model.scores_into(&bag, &mut scores_buf);
        let mut mass = 0.0f64;
        for k in presented {
            if let Some(ci) = universe.iter().position(|u| u == k) {
                mass += scores_buf[ci] as f64;
            }
        }
        if mass <= 0.0 {
            continue;
        }
        let gold_pos = presented.iter().position(|k| *k == rows[i].label);
        let mut expectation = 0.0f64;
        let mut cum = 0.0f64;
        let mut gold_cdm = None;
        for (pos, k) in presented.iter().enumerate() {
            let Some(ci) = universe.iter().position(|u| u == k) else {
                continue;
            };
            let p = scores_buf[ci] as f64 / mass;
            expectation += p * pos as f64;
            cum += p;
            if Some(pos) == gold_pos {
                gold_cdm = Some(cum);
            }
        }
        if let Some(g) = gold_pos {
            mae_sum += (expectation - g as f64).abs();
        }
        cdm_sum += gold_cdm.unwrap_or(0.0);
        n += 1;
    }
    if n == 0 {
        (f64::NAN, f64::NAN)
    } else {
        (mae_sum / n as f64, cdm_sum / n as f64)
    }
}

// ── the holdout law (row-stratified, one shape for all four heads) ──────

/// The row-stratified holdout over the given per-row strata (a case's
/// questions never split across the holdout — the house
/// `typed_row_holdout` law). Returns `(holdout, train)`.
pub fn row_holdout(strata: &[String], budget: usize) -> (Vec<usize>, Vec<usize>) {
    let buckets: Vec<SuiteRow> = strata
        .iter()
        .map(|s| SuiteRow {
            label: s.clone(),
            text: String::new(),
            presented: None,
        })
        .collect();
    stratified_holdout(&buckets, budget)
}

/// The PAIRED-case holdout for two-arm rows (present/absent): the
/// round-robin selects WHOLE strata — every row of a selected case crosses
/// together — so the holdout carries both arms of each selected case (the
/// plain [`row_holdout`] would take one row per case and, with the corpus
/// ordered present-first, an all-`c_y=1` holdout whose AUROC is
/// undefined). Budget = holdout ROWS (a case contributes all its rows).
pub fn paired_holdout(strata: &[String], budget: usize) -> (Vec<usize>, Vec<usize>) {
    let mut order: Vec<String> = Vec::new();
    let mut by_case: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, s) in strata.iter().enumerate() {
        if !by_case.contains_key(s.as_str()) {
            order.push(s.clone());
        }
        by_case.entry(s.as_str()).or_default().push(i);
    }
    let mut holdout = Vec::new();
    let mut in_holdout = vec![false; strata.len()];
    for case in &order {
        if holdout.len() >= budget {
            break;
        }
        for &i in &by_case[case.as_str()] {
            holdout.push(i);
            in_holdout[i] = true;
        }
    }
    let rest: Vec<usize> = (0..strata.len()).filter(|i| !in_holdout[*i]).collect();
    (holdout, rest)
}

// ── the tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instinct_specialist::{train_arm_a, ArmAConfig};

    fn demo_rows() -> Vec<SuiteRow> {
        (0..4)
            .map(|i| SuiteRow {
                label: ["alpha", "beta", "gamma"][i % 3].to_string(),
                text: format!("state text {i} with discriminative token{i}"),
                presented: Some(vec!["alpha".into(), "beta".into(), "gamma".into()]),
            })
            .collect()
    }

    fn demo_universe() -> Vec<String> {
        vec!["alpha".into(), "beta".into(), "gamma".into()]
    }

    /// `gold_mix == 1.0` is bit-identical to `Targets::Gold` for every row
    /// (the `0.0 · finite q` law) — the dual-shuffle-rejected posture is
    /// exactly the incumbent Arm A, pinned byte-for-byte on the weights.
    #[test]
    fn rejected_rows_train_bit_identical_to_gold() {
        let rows = demo_rows();
        let universe = demo_universe();
        let gold = vec![0usize, 1, 2, 0];
        let n = universe.len();
        let teacher = TeacherDump {
            classes: universe.clone(),
            gold: gold.iter().map(|&g| g as u32).collect(),
            picks: vec![1; 4],
            q: vec![0.25f32; 4 * n], // garbage the mix must keep at weight 0
        };
        let gold_mix = vec![1.0f32; 4];
        let cfg = ArmAConfig {
            epochs: 2,
            lr: 0.05,
            warmup_steps: 0,
            ..ArmAConfig::default()
        };
        let all: Vec<usize> = (0..4).collect();
        let via_mix = train_core(
            &rows,
            &universe,
            &all,
            &[],
            &cfg,
            Targets::DistillRowMix {
                q: &teacher,
                gold: gold.clone(),
                gold_mix: gold_mix.clone(),
            },
        )
        .expect("row-mix run trains");
        let via_gold = train_core(&rows, &universe, &all, &[], &cfg, Targets::Gold(gold))
            .expect("gold run trains");
        assert_eq!(
            via_mix.model.w, via_gold.model.w,
            "weights must be bit-identical"
        );
        assert_eq!(
            via_mix.model.b, via_gold.model.b,
            "biases must be bit-identical"
        );
    }

    /// The qualified 7:3 blend must actually move the model toward the
    /// teacher: with the teacher pointing OFF gold, the qualified run's
    /// weights differ from the pure-gold run (the mix carries signal).
    #[test]
    fn qualified_mix_blends_the_teacher_in() {
        let rows = demo_rows();
        let universe = demo_universe();
        let gold = vec![0usize, 1, 2, 0];
        let n = universe.len();
        // Teacher puts ALL its mass on the LAST class (never the gold).
        let mut q = vec![0.0f32; 4 * n];
        for r in 0..4 {
            q[r * n + 2] = 1.0;
        }
        let teacher = TeacherDump {
            classes: universe.clone(),
            gold: vec![0; 4],
            picks: vec![2; 4],
            q,
        };
        let cfg = ArmAConfig {
            epochs: 2,
            lr: 0.05,
            warmup_steps: 0,
            ..ArmAConfig::default()
        };
        let all: Vec<usize> = (0..4).collect();
        let qualified = train_core(
            &rows,
            &universe,
            &all,
            &[],
            &cfg,
            Targets::DistillRowMix {
                q: &teacher,
                gold: gold.clone(),
                gold_mix: vec![GOLD_MIX_QUALIFIED; 4],
            },
        )
        .expect("qualified run trains");
        let pure_gold =
            train_core(&rows, &universe, &all, &[], &cfg, Targets::Gold(gold)).expect("gold run");
        assert_ne!(
            qualified.model.w, pure_gold.model.w,
            "a 0.3-gold / 0.7-teacher blend must differ from pure gold"
        );
    }

    #[test]
    fn auroc_basics() {
        assert_eq!(auroc(&[0.9, 0.8], &[0.2, 0.1]), Some(1.0));
        assert_eq!(auroc(&[0.1, 0.2], &[0.8, 0.9]), Some(0.0));
        assert_eq!(auroc(&[0.5], &[0.5]), Some(0.5));
        assert_eq!(auroc(&[], &[0.5]), None);
    }

    /// The restricted pick: ties fall to the LOWEST PRESENTED position, a
    /// key outside the class universe is unanswerable, and an empty class
    /// space never picks.
    #[test]
    fn pick_restricted_tie_and_exclusion_laws() {
        let model = Specialist::zero(&["a".to_string(), "b".to_string(), "c".to_string()]);
        let universe = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let bag = vec![(0u32, 1.0f32)];
        // Zero model → all scores equal → the FIRST PRESENTED key wins
        // (the tie law in pick space).
        let (ci, _) = pick_restricted(&model, &bag, &universe, &["c".to_string(), "a".to_string()])
            .expect("presented keys answer");
        assert_eq!(
            universe[ci], "c",
            "tie must keep the first presented position"
        );
        // A presented key outside the universe is skipped, not fatal.
        let (ci, _) = pick_restricted(
            &model,
            &bag,
            &universe,
            &["zz".to_string(), "b".to_string()],
        )
        .expect("the in-universe key answers");
        assert_eq!(universe[ci], "b");
        // No presented key in the universe → unanswerable.
        assert!(pick_restricted(&model, &bag, &universe, &["zz".to_string()]).is_none());
        assert!(pick_restricted(&model, &[], &universe, &["a".to_string()]).is_none());
    }

    // ── the sealed-artifact join laws (pid-scoped temp dir) ─────────────

    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new(tag: &str) -> Self {
            let p =
                std::env::temp_dir().join(format!("typed_eq9_test_{}_{}", tag, std::process::id()));
            let _ = std::fs::remove_dir_all(&p);
            std::fs::create_dir_all(p.join("typed_decisions")).expect("temp dir");
            Self(p)
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn seal(path: &std::path::Path, body: &str) {
        std::fs::write(path, body).expect("write artifact");
        std::fs::write(
            format!("{}.blake3", path.display()),
            format!("{}\n", blake3::hash(body.as_bytes()).to_hex()),
        )
        .expect("write sidecar");
    }

    /// One typed train page with TWO choice rows (row 0 = the corpus row;
    /// row 1 = a non-corpus event) — the minimal join fixture.
    fn write_dataset(dir: &std::path::Path) {
        let page = serde_json::json!({
            "rows": [
                {"row": {
                    "id": "tr_wf_000000", "workflow": "wf",
                    "state": "state one alpha",
                    "questions": "{\"act\": {\"type\": \"choice\", \"instructions\": \"i\", \"criteria\": {\"alpha\": \"A d\", \"beta\": \"B d\", \"gamma\": \"G d\"}}}",
                    "gold": "{\"act\": {\"label\": \"alpha\"}}"
                }},
                {"row": {
                    "id": "tr_wf_000001", "workflow": "wf",
                    "state": "state two beta",
                    "questions": "{\"act\": {\"type\": \"choice\", \"instructions\": \"i\", \"criteria\": {\"beta\": \"B d\", \"gamma\": \"G d\"}}, \"level\": {\"type\": \"score\", \"instructions\": \"how much\", \"criteria\": [\"low l\", \"mid l\", \"high l\"]}}",
                    "gold": "{\"act\": {\"label\": \"beta\"}, \"level\": {\"label\": 2}}"
                }}
            ]
        });
        let ds = dir.join("typed_decisions").join("train-000.json");
        std::fs::write(&ds, serde_json::to_string(&page).unwrap()).expect("write dataset");
    }

    fn corpus_row(id: &str, row_id: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id, "arm": "present",
            "criteria": {"alpha": "A d", "beta": "B d", "gamma": "G d"},
            "n_options": 3, "c_y": 1,
            "row_id": row_id, "qid": "act",
            "state": "state one alpha",
            "instructions": "i", "gold_label": "alpha"
        })
    }

    fn write_corpus(dir: &std::path::Path) -> std::path::PathBuf {
        let cov = dir.join("cov.json");
        let doc = serde_json::json!({
            "meta": {"kind": "jeva_t3_coverage_rows"},
            "rows": [
                corpus_row("tr_wf_000000#act", "tr_wf_000000"),
                {"id": "tr_wf_000000#act@absent", "arm": "absent",
                 "criteria": {"beta": "B d", "gamma": "G d"}, "n_options": 2,
                 "c_y": 0, "row_id": "tr_wf_000000", "qid": "act",
                 "state": "state one alpha", "instructions": "i", "gold_label": null}
            ]
        });
        seal(&cov, &serde_json::to_string(&doc).unwrap());
        cov
    }

    fn write_rho(dir: &std::path::Path, cov_digest: &str) -> std::path::PathBuf {
        let rho = dir.join("rho.json");
        let doc = serde_json::json!({
            "summary": {"input_blake3": cov_digest},
            "rows": [
                {"id": "tr_wf_000000#act", "arm": "present", "c_y": 1,
                 "rho": 0.5, "latency_ms": 1.0, "choice": null,
                 "conditional_probabilities": {"alpha": 0.2, "beta": 0.3, "gamma": 0.5}}
            ]
        });
        seal(&rho, &serde_json::to_string(&doc).unwrap());
        rho
    }

    fn write_ds(dir: &std::path::Path, cov_digest: &str, qualified: bool) -> std::path::PathBuf {
        let ds = dir.join("ds.json");
        let doc = serde_json::json!({
            "summary": {
                "inputs": {"coverage_rows_blake3": cov_digest},
                "blend_rule": "qualified -> targets = 0.7*teacher_conditionals + 0.3*gold_onehot (the 7:3 teacher:gold blend); rejected -> pure gold (mix 1.0)."
            },
            "rows": [{"id": "tr_wf_000000#act", "qualified": qualified}]
        });
        seal(&ds, &serde_json::to_string(&doc).unwrap());
        ds
    }

    fn cov_digest_of(cov: &std::path::Path) -> String {
        blake3::hash(&std::fs::read(cov).expect("read cov"))
            .to_hex()
            .to_string()
    }

    fn zero_score_model(labels: &[String]) -> Specialist {
        Specialist::zero(labels)
    }

    #[test]
    fn join_laws_happy_path_and_projections() {
        let t = TempDir::new("happy");
        write_dataset(&t.0);
        let cov = write_corpus(&t.0);
        let rho = write_rho(&t.0, &cov_digest_of(&cov));
        let ds = write_ds(&t.0, &cov_digest_of(&cov), true);
        let data = load_choice_eq9(&t.0, &cov, &rho, Some(&ds)).expect("the join assembles");
        // Universe = the train pool's choice keys (row 1 presents beta+gamma).
        assert_eq!(data.universe, vec!["alpha", "beta", "gamma"]);
        assert_eq!(data.rows.len(), 2);
        assert_eq!(data.n_corpus_rows, 1);
        assert_eq!(data.n_qualified, 1);
        // Corpus row: teacher projected (0.2, 0.3, 0.5); non-corpus row: all
        // zero (its mix is 1.0, so the slice is never read with weight).
        let n = data.universe.len();
        assert_eq!(&data.teacher.q[0..n], &[0.2, 0.3, 0.5]);
        assert!(data.teacher.q[n..2 * n].iter().all(|&v| v == 0.0));
        assert_eq!(data.gold_mix, vec![GOLD_MIX_QUALIFIED, 1.0]);
        // The unqualified posture flips the mix, nothing else.
        let ds2 = write_ds(&t.0, &cov_digest_of(&cov), false);
        let data2 = load_choice_eq9(&t.0, &cov, &rho, Some(&ds2)).expect("rejected posture loads");
        assert_eq!(data2.gold_mix, vec![1.0, 1.0]);
        assert_eq!(data2.n_qualified, 0);
        // The dev posture (no qualification file) is pure gold everywhere.
        let data3 = load_choice_eq9(&t.0, &cov, &rho, None).expect("dev posture loads");
        assert_eq!(data3.gold_mix, vec![1.0, 1.0]);
    }

    #[test]
    fn join_laws_refuse_drift() {
        let t = TempDir::new("drift");
        write_dataset(&t.0);
        let cov = write_corpus(&t.0);
        let digest = cov_digest_of(&cov);
        let rho = write_rho(&t.0, &digest);

        // A qualification file missing a corpus row refuses (every corpus
        // row must be qualified or rejected, none missing).
        let ds_missing = {
            let p = t.0.join("ds_missing.json");
            let doc = serde_json::json!({
                "summary": {
                    "inputs": {"coverage_rows_blake3": digest},
                    "blend_rule": "the 7:3 teacher:gold blend"
                },
                "rows": []
            });
            seal(&p, &serde_json::to_string(&doc).unwrap());
            p
        };
        let err = load_choice_eq9(&t.0, &cov, &rho, Some(&ds_missing)).unwrap_err();
        assert!(err.contains("every corpus row"), "got: {err}");

        // A qualification file pinned to a DIFFERENT coverage generation.
        let ds_forgen = write_ds(&t.0, &"0".repeat(64), true);
        let err = load_choice_eq9(&t.0, &cov, &rho, Some(&ds_forgen)).unwrap_err();
        assert!(err.contains("DIFFERENT coverage generation"), "got: {err}");

        // A rho artifact from another generation refuses.
        let rho_bad = write_rho(&t.0, &"f".repeat(64));
        let err = load_choice_eq9(&t.0, &cov, &rho_bad, None).unwrap_err();
        assert!(err.contains("DIFFERENT coverage generation"), "got: {err}");

        // Corpus presented keys drifted from the pool row refuse.
        let cov_drift = {
            let p = t.0.join("cov_drift.json");
            let mut row = corpus_row("tr_wf_000000#act", "tr_wf_000000");
            row["criteria"] = serde_json::json!({"alpha": "A d", "beta": "B d"});
            let doc = serde_json::json!({"meta": {}, "rows": [row]});
            seal(&p, &serde_json::to_string(&doc).unwrap());
            p
        };
        let rho_drift = write_rho(&t.0, &cov_digest_of(&cov_drift));
        let err = load_choice_eq9(&t.0, &cov_drift, &rho_drift, None).unwrap_err();
        assert!(
            err.contains("drifted") || err.contains("outside") || err.contains("mass"),
            "got: {err}"
        );
    }

    /// The coverage arm: c_y → label mapping, the text law (state +
    /// instructions + one `key: desc` line per presented key), and the
    /// sealed-sidecar refusal.
    #[test]
    fn coverage_arm_labels_text_and_seal() {
        let t = TempDir::new("coverage");
        let cov = write_corpus(&t.0);
        let data = load_coverage_eq9(&cov).expect("coverage arm loads");
        assert_eq!(data.rows.len(), 2);
        assert_eq!(data.rows[0].label, "covered");
        assert_eq!(data.rows[1].label, "absent");
        assert_eq!(data.gold, vec![1, 0]);
        assert!(
            data.rows[0]
                .text
                .starts_with("state one alpha\ni\nalpha: A d\nbeta: B d\ngamma: G d"),
            "text law: got {:?}",
            data.rows[0].text
        );
        // An unsealed artifact refuses.
        let unsealed = t.0.join("unsealed.json");
        std::fs::write(
            &unsealed,
            serde_json::to_string(&serde_json::json!({"rows": []})).unwrap(),
        )
        .unwrap();
        assert!(load_coverage_eq9(&unsealed).is_err());
    }

    /// The full trained lane assembles end-to-end on the fixture:
    /// choice (Eq-9), noul/score flattening, coverage AUROC.
    #[test]
    fn end_to_end_fixture_lane() {
        let t = TempDir::new("e2e");
        write_dataset(&t.0);
        let cov = write_corpus(&t.0);
        let rho = write_rho(&t.0, &cov_digest_of(&cov));
        let ds = write_ds(&t.0, &cov_digest_of(&cov), true);
        let data = load_choice_eq9(&t.0, &cov, &rho, Some(&ds)).expect("join assembles");
        let cfg = ArmAConfig {
            epochs: 1,
            warmup_steps: 0,
            ..ArmAConfig::default()
        };
        let run = train_choice_eq9(&data, &[0, 1], &[], &cfg).expect("choice trains");
        let (acc, unanswered) = restricted_eval(&run.model, &data.rows, &data.universe, &[0]);
        assert_eq!(unanswered, 0);
        let _ = acc;
        // Coverage: one covered + one absent row; a zero model (all scores
        // equal) must read AUROC exactly 0.5 (the tie law).
        let covd = load_coverage_eq9(&cov).expect("coverage loads");
        let zero = Specialist::zero(&covd.labels);
        assert_eq!(coverage_holdout_auroc(&zero, &covd, &[0, 1]), Some(0.5));
        // Noul/score flattening runs and the score aux reads NaN-safe.
        let heads = hard_label_rows(&t.0).expect("hard-label rows");
        assert!(heads.noul_rows.is_empty());
        let score_rows = &heads.score["level"].rows;
        assert_eq!(score_rows.len(), 1);
        assert_eq!(score_rows[0].label, "high l");
        assert_eq!(heads.score["level"].strata, vec!["tr_wf_000001"]);
        assert_eq!(
            score_rows[0].presented.as_deref(),
            Some(
                &[
                    "low l".to_string(),
                    "mid l".to_string(),
                    "high l".to_string()
                ][..]
            )
        );
        let uni = crate::instinct_specialist::label_universe(score_rows);
        let (mae, cdm) = score_aux_eval(&zero_score_model(&uni), score_rows, &uni, &[0]);
        assert!(!mae.is_nan() && !cdm.is_nan());
        // Arm A over the flattened rows still trains (the hard-label path).
        let _ = train_arm_a(score_rows, &uni, &[0], &[], &cfg).expect("arm a trains");
    }
    /// The paired-case holdout selects WHOLE strata, so a two-arm corpus's
    /// holdout carries both arms (the plain row_holdout would read
    /// single-arm and an AUROC over it is undefined).
    #[test]
    fn paired_holdout_selects_whole_cases() {
        let strata: Vec<String> = ["c1", "c1", "c2", "c2", "c3", "c3"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let (h, t) = paired_holdout(&strata, 2);
        assert_eq!(h, vec![0, 1], "c1 crosses whole (both rows)");
        assert_eq!(t, vec![2, 3, 4, 5]);
        let strata_single: Vec<String> = ["c1", "c2", "c3"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let (h1, t1) = paired_holdout(&strata_single, 2);
        assert_eq!(h1, vec![0, 1]);
        assert_eq!(t1, vec![2]);
    }
}

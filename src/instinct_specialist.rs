//! Issue 576 — the arena decision specialists for the `Reflex · instinct`
//! lane (riir-instinct Plan 001 P2). Arm A: supervised one-vs-all logistic
//! over the reflex event lexicon, trained on the arena suites' TRAIN rows
//! only.
//!
//! WHY THIS SHAPE (the issue's T1: "small encoder or ternary head, sized
//! for a p50 well under laya"): the serving cost of a sparse logistic is
//! one sparse dot per class — µs-class, three orders under the laya lane's
//! ~26–34 ms p50 — and the model class is the measured one: Bench 051's
//! issue-038 POC read plain NB on these train rows at ag_news .868 /
//! emotion .570 / sst5 .387, all far above the shipped modelless lane.
//! A discriminatively trained logistic over the SAME events is the
//! smallest step up from that measurement.
//!
//! FEATURE LAW (the cross-repo contract): events are the reflex
//! `hashed_tokens_into` law — lowercased ASCII words (punctuation-trimmed)
//! plus word bigrams, FNV-1a 64, `mod VOCAB` — at the reflex `NB_VOCAB`
//! width of 2^17, so the specialist and the nb_scope tables read ONE
//! lexicon and H2's `m_i` stays in the same feature space. The law is
//! REIMPLEMENTED here (riir-train deps no riir-reflex — boundary); the
//! structure pins in the tests are the sync pin, and a change on either
//! side must move both + the artifacts (`corpus_version` discipline).
//!
//! PROTOCOL (the issue's no-cheat law): TRAIN rows only; the held-out
//! selection slice is a label-stratified round-robin front of the TRAIN
//! split (the reflex `stratified_split` law — deterministic, no RNG); the
//! test split is never touched here (the arena reads it once, in
//! riir-instinct). The lr=0 control is the house training-gate rule: with
//! every parameter at zero the pick is class 0, so the control accuracy is
//! exactly the holdout share of class 0 — the trained model must beat it.
//!
//! SERVING READOUT: per-class sigmoid scores (never softmax — the house
//! law), argmax with ties to the lowest class index (the engine argmax
//! law). The artifact carries the class NAMES, so P3 maps options by name.
//!
//! OPTIMIZER: lazy AdamW — dense state, updates touched only (per row the
//! touched set is `nnz × L`), decoupled weight decay on touched params
//! (the standard lazy-Adam caveat: untouched params decay only when
//! touched; with 3 epochs of dense text every feature is touched many
//! times). Zero init + fixed row order + fixed per-row op order ⇒ the run
//! is bit-deterministic single-threaded, no seed, no RNG.

use std::collections::HashMap;
use std::path::Path;

// ── the feature law (the reflex embed.rs law, reimplemented) ────────────

/// Event vocabulary width — the reflex `nb_scope::NB_VOCAB` value. The two
/// sides MUST stay equal or the lexicons fork.
pub const VOCAB: usize = 1 << 17;

/// Word-hash salt (the reflex `WORD_SALT`).
pub const WORD_SALT: u64 = 0x3456_7890_1234_5678;
/// Bigram-hash salt (the reflex `BIGRAM_SALT`).
pub const BIGRAM_SALT: u64 = 0x0f1e_2d3c_4b5a_6978;

/// FNV-1a 64 over ASCII-lowercased bytes (the reflex `fnv1a_word`).
#[inline]
pub fn fnv1a_word(bytes: &[u8], salt: u64) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64 ^ salt;
    for &b in bytes {
        let b = if b.is_ascii_uppercase() { b + 32 } else { b };
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Trim non-alphanumeric ASCII edges (the reflex `token`).
#[inline]
fn token(raw: &[u8]) -> &[u8] {
    let is_sep = |b: u8| !b.is_ascii_alphanumeric();
    let mut a = 0usize;
    let mut b = raw.len();
    while a < b && is_sep(raw[a]) {
        a += 1;
    }
    while b > a && is_sep(raw[b - 1]) {
        b -= 1;
    }
    &raw[a..b]
}

/// Event ids for `text`: unigrams + bigrams, `mod VOCAB`, in emission
/// order (duplicates kept — a count model wants them). The reflex
/// `hashed_tokens_into` law.
pub fn events_into(text: &[u8], out: &mut Vec<u32>) {
    out.clear();
    let mut prev: Option<u64> = None;
    for raw in text.split(|b: &u8| b.is_ascii_whitespace()) {
        let t = token(raw);
        if t.is_empty() {
            continue;
        }
        let h = fnv1a_word(t, WORD_SALT);
        out.push((h % VOCAB as u64) as u32);
        if let Some(p) = prev {
            let mut pair = [0u8; 16];
            pair[..8].copy_from_slice(&p.to_le_bytes());
            pair[8..].copy_from_slice(&h.to_le_bytes());
            out.push((fnv1a_word(&pair, BIGRAM_SALT) % VOCAB as u64) as u32);
        }
        prev = Some(h);
    }
}

/// The L2-normalized sparse count bag: `(bucket, weight)` pairs SORTED by
/// bucket (deterministic reduction order — a HashMap iteration would make
/// the gradient sum order machine-dependent). Collisions merge with their
/// weights summed, so `Σ weights` is the event count whatever the hashes
/// do. Cleared first.
pub fn bag_into(text: &[u8], out: &mut Vec<(u32, f32)>, scratch: &mut Vec<u32>) {
    events_into(text, scratch);
    out.clear();
    if scratch.is_empty() {
        return;
    }
    scratch.sort_unstable();
    let mut i = 0usize;
    while i < scratch.len() {
        let bucket = scratch[i];
        let start = i;
        while i < scratch.len() && scratch[i] == bucket {
            i += 1;
        }
        out.push((bucket, (i - start) as f32));
    }
    let n2 = out.iter().map(|(_, w)| w * w).sum::<f32>().sqrt();
    if n2 > 0.0 {
        let inv = 1.0 / n2;
        for w in out.iter_mut() {
            w.1 *= inv;
        }
    }
}

// ── rows + the stratified holdout (the reflex label/text laws) ──────────

/// One labeled train row.
#[derive(Debug, Clone)]
pub struct SuiteRow {
    pub label: String,
    pub text: String,
    /// Plan 010 S1MB: the case's PRESENTED option keys in the pick-space
    /// spelling (choice = the criteria keys; noul = the fixed
    /// ["false", "true"]; score = the level INDEX strings) — what the
    /// presented-constrained holdout read restricts its argmax to.
    /// `None` = fixed-label suite (presented == the class universe).
    pub presented: Option<Vec<String>>,
}

/// The per-row label/text rules — the reflex `train_row_label` +
/// `train_docs` laws for the six above-gate suites, plus the Issue 578
/// T2 below-gate suite (prompt_injections, 546 train rows). Its label
/// uses the UNIFIED noul spelling — `"no"` / `"yes"` (gold 1 = injection
/// = yes), the same law the typed_decisions artifact uses, so the
/// consumer noul bridge maps presented `[false, true]` positions onto
/// `no`/`yes` class rows for BOTH suites with one rule. (Reflex's own
/// corpus convention for this suite is `"0"`/`"1"` — that is the
/// the modelless lane's train-doc law, a different consumer.)
/// Issue 593: `code_fixtures` joins the label_text passthrough — its
/// train envelopes are EXPORTED by the consumer repo (riir-instinct
/// `examples/export_code_fixtures`, the frozen fixture's cal+docs slices;
/// riir-train gains no reflex dep) with the union label universe: the 8
/// frozen module names + the unified NOUL pair (`no`/`yes`), one row per
/// (span, question). Unknown suite → None (the loader fails loud on an
/// empty result).
pub fn suite_row(suite: &str, row: &serde_json::Value) -> Option<SuiteRow> {
    // Plan 010 S1MB: one judgment per row (the converter's flatten). The
    // label speaks the ARENA's pick-space spelling — the seat labels the
    // presented-option bridge resolves by name: choice = the gold KEY at
    // the gold position ("A", ...), noul = "false"/"true", score = the
    // level INDEX string ("0".."9" — reflex's option_key_union spelling
    // for array criteria, NOT the criteria value at that position). The
    // text law (below) mirrors the arena's query bag exactly — the s1mb
    // task signal lives in the instructions, so the trainer's text and
    // the arena's bag_text_into carry the same bytes.
    if matches!(suite, "s1mb_choice" | "s1mb_noul" | "s1mb_score") {
        let q = row.get("question")?;
        let keys = q.get("keys")?.as_array()?;
        let gold_idx = q.get("gold_idx")?.as_u64()? as usize;
        let state = row.get("state")?.as_str()?;
        let presented: Vec<String> = match suite {
            "s1mb_choice" => keys
                .iter()
                .map(|k| k.as_str().map(str::to_string))
                .collect::<Option<Vec<_>>>()?,
            // The unified noul pair (the producer artifact law the
            // arena's bridge resolves by name — NOUL_PAIR), in presented
            // order: position 0 = "no" = gold idx 0 ("false").
            "s1mb_noul" => vec!["no".to_string(), "yes".to_string()],
            _ => (0..keys.len()).map(|i| i.to_string()).collect(),
        };
        let label = match suite {
            "s1mb_score" => presented.get(gold_idx)?.clone(),
            "s1mb_noul" => match keys.get(gold_idx)?.as_str()? {
                "false" => "no".to_string(),
                "true" => "yes".to_string(),
                other => return Some(SuiteRow { // unreachable — loud skip
                    label: format!("__bad_noul_key_{other}"),
                    text: String::new(),
                    presented: None,
                }),
            },
            _ => keys.get(gold_idx)?.as_str()?.to_string(),
        };
        // The text law: state + "\n" + instructions — the s1mb task signal
        // (the noul statement, the folded false/true definitions, the
        // question phrasing) lives in the instructions, and the arena's
        // query bag for these suites carries the same text
        // (SuiteCtx::bag_text_into). An empty instructions folds to the
        // state-only bag (a trailing separator emits zero tokens).
        let instructions = q.get("instructions").and_then(|v| v.as_str()).unwrap_or("");
        return Some(SuiteRow {
            label,
            text: format!("{state}\n{instructions}"),
            presented: Some(presented),
        });
    }
    let label = match suite {
        "massive_intent_en" | "code_fixtures" => row.get("label_text")?.as_str()?.to_string(),
        "banking77" => match row.get("label_text").and_then(|v| v.as_str()) {
            Some(t) => t.replace('_', " "),
            None => row.get("label")?.as_i64()?.to_string(),
        },
        "ag_news" | "emotion" | "sst5" | "xnli_en" => row.get("label")?.as_i64()?.to_string(),
        "prompt_injections" => match row.get("label")?.as_i64()? {
            1 => "yes".to_string(),
            _ => "no".to_string(),
        },
        _ => return None,
    };
    let text = match suite {
        "xnli_en" => format!(
            "{}\n{}",
            row.get("premise")?.as_str()?,
            row.get("hypothesis")?.as_str()?
        ),
        _ => row.get("text")?.as_str()?.to_string(),
    };
    Some(SuiteRow {
        label,
        text,
        presented: None,
    })
}

/// Load one suite's train rows from a datasets dir
/// (`<dir>/<suite>/train-*.json`, the datasets-server envelope
/// `{rows: [{row_idx, row}]}`). Row order is the load order (sorted file
/// names) — part of the determinism contract.
pub fn load_suite_rows(dir: &Path, suite: &str) -> Result<Vec<SuiteRow>, String> {
    let raw = read_train_envelope(dir, suite)?;
    let mut rows = Vec::new();
    for row in &raw {
        if let Some(sr) = suite_row(suite, row) {
            rows.push(sr);
        }
    }
    if rows.is_empty() {
        return Err(format!("suite {suite}: no rows survived the label rule"));
    }
    Ok(rows)
}

/// The raw `row` Values of a suite's `train-*.json` envelope files, in
/// load order (sorted file names) — shared by [`load_suite_rows`] and the
/// typed_decisions loader ([`instinct_specialist_typed`]).
pub(crate) fn read_train_envelope(dir: &Path, suite: &str) -> Result<Vec<serde_json::Value>, String> {
    read_split_envelope(dir, suite, "train")
}

/// The generalized reader behind [`read_train_envelope`]: the raw `row`
/// Values of a suite's `<split>-*.json` envelope files, in load order.
/// The issue-579 T1 lane reads the TEST split through this (the one
/// sanctioned validation read) without forking the envelope law.
pub(crate) fn read_split_envelope(
    dir: &Path,
    suite: &str,
    split: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let suite_dir = dir.join(suite);
    let prefix = format!("{split}-");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&suite_dir)
        .map_err(|e| format!("read_dir {}: {e}", suite_dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&prefix) && n.ends_with(".json"))
        })
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("no {split}-*.json under {}", suite_dir.display()));
    }
    let mut rows = Vec::new();
    for f in &files {
        let text = std::fs::read_to_string(f).map_err(|e| format!("read {}: {e}", f.display()))?;
        let v: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", f.display()))?;
        let page = v
            .get("rows")
            .and_then(|r| r.as_array())
            .ok_or_else(|| format!("{}: no rows array", f.display()))?;
        for r in page {
            if let Some(row) = r.get("row") {
                rows.push(row.clone());
            }
        }
    }
    Ok(rows)
}

/// The class universe: distinct train labels, SORTED (stable domain order;
/// the artifact carries the names, so P3 maps options by name).
pub fn label_universe(rows: &[SuiteRow]) -> Vec<String> {
    let mut l: Vec<String> = Vec::new();
    for r in rows {
        if !l.contains(&r.label) {
            l.push(r.label.clone());
        }
    }
    l.sort();
    l
}

/// Plan 010 T4 — the s1mb specialist's CLASS SPACE: the sorted union of
/// every PRESENTED key across BOTH halves (train + test), in the
/// pick-space spelling [`suite_row`] labels speak. Gold is never read
/// here (the no-cheat law: the test split's labels stay unread — only
/// the option-key space crosses, the same fact reflex's engine
/// `option_key_union` is built from). The presented-option bridge
/// resolves by name, so the artifact must cover every key the seat can
/// present — cal cases included, which are carved from the train half.
pub fn s1mb_presented_union(dir: &Path, suite: &str) -> Result<Vec<String>, String> {
    if !matches!(suite, "s1mb_choice" | "s1mb_noul" | "s1mb_score") {
        return Err(format!("suite {suite}: not an s1mb suite"));
    }
    let sdir = dir.join(suite);
    let mut seen = std::collections::HashSet::new();
    for split in ["test", "train"] {
        let prefix = format!("{split}-");
        let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(&sdir)
            .map_err(|e| format!("read {}: {e}", sdir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.starts_with(&prefix) && n.ends_with(".json"))
                    .unwrap_or(false)
            })
            .collect();
        paths.sort();
        if paths.is_empty() {
            return Err(format!("suite {suite}: no {split}-*.json under {}", sdir.display()));
        }
        for p in paths {
            let bytes = std::fs::read(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
            let v: serde_json::Value = serde_json::from_slice(&bytes)
                .map_err(|e| format!("parse {}: {e}", p.display()))?;
            for entry in v.get("rows").and_then(|r| r.as_array()).into_iter().flatten() {
                let Some(keys) = entry
                    .get("row")
                    .and_then(|r| r.get("question"))
                    .and_then(|q| q.get("keys"))
                    .and_then(|k| k.as_array())
                else {
                    continue;
                };
                match suite {
                    "s1mb_choice" => {
                        for k in keys {
                            if let Some(s) = k.as_str() {
                                seen.insert(s.to_string());
                            }
                        }
                    }
                    "s1mb_noul" => {
                        // The unified pair — the artifact spelling the
                        // arena's noul bridge resolves by name.
                        seen.insert("no".to_string());
                        seen.insert("yes".to_string());
                    }
                    _ => {
                        for i in 0..keys.len() {
                            seen.insert(i.to_string());
                        }
                    }
                }
            }
        }
    }
    let mut names: Vec<String> = seen.into_iter().collect();
    names.sort();
    Ok(names)
}

/// The label-stratified round-robin holdout — the reflex `stratified_split`
/// law: bucket by label (first-appearance order), one row per label per
/// round, up to `budget`. Returns `(holdout_in_pick_order, train_rest_in_
/// row_order)`. Deterministic, no RNG.
pub fn stratified_holdout(rows: &[SuiteRow], budget: usize) -> (Vec<usize>, Vec<usize>) {
    let mut by_label: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        if !by_label.contains_key(r.label.as_str()) {
            order.push(&r.label);
        }
        by_label.entry(r.label.as_str()).or_default().push(i);
    }
    let buckets: Vec<&Vec<usize>> = order.iter().map(|l| &by_label[l]).collect();
    let mut cursors = vec![0usize; buckets.len()];
    let mut holdout = Vec::with_capacity(budget.min(rows.len()));
    let mut in_holdout = vec![false; rows.len()];
    if budget > 0 {
        loop {
            let mut took = false;
            for (b, bucket) in buckets.iter().enumerate() {
                if cursors[b] < bucket.len() {
                    let idx = bucket[cursors[b]];
                    cursors[b] += 1;
                    holdout.push(idx);
                    in_holdout[idx] = true;
                    took = true;
                    if holdout.len() >= budget {
                        break;
                    }
                }
            }
            if holdout.len() >= budget || !took {
                break;
            }
        }
    }
    let rest: Vec<usize> = (0..rows.len()).filter(|i| !in_holdout[*i]).collect();
    (holdout, rest)
}

// ── the model + Arm A trainer ───────────────────────────────────────────

/// Trainer knobs. Zero-init training: no seed, no RNG anywhere.
#[derive(Debug, Clone, Copy)]
pub struct ArmAConfig {
    pub epochs: usize,
    pub lr: f32,
    pub weight_decay: f32,
    pub warmup_steps: u64,
    /// Cosine floor as a fraction of `lr`.
    pub min_lr_ratio: f32,
}

impl Default for ArmAConfig {
    fn default() -> Self {
        Self {
            epochs: 3,
            lr: 0.05,
            weight_decay: 0.01,
            warmup_steps: 500,
            min_lr_ratio: 0.05,
        }
    }
}

/// The trained one-vs-all logistic: `score_c = b_c + w_c · x` over the
/// L2-normalized bag. Row-major `w` (`class * VOCAB + bucket`).
#[derive(Debug, Clone)]
pub struct Specialist {
    pub labels: Vec<String>,
    pub w: Vec<f32>,
    pub b: Vec<f32>,
}

impl Specialist {
    pub fn zero(labels: &[String]) -> Self {
        Self {
            labels: labels.to_vec(),
            w: vec![0.0; labels.len() * VOCAB],
            b: vec![0.0; labels.len()],
        }
    }

    /// Sigmoid scores per class for one sparse bag (never softmax).
    pub fn scores_into(&self, bag: &[(u32, f32)], out: &mut [f32]) {
        for (c, o) in out.iter_mut().enumerate() {
            let mut s = self.b[c];
            for &(j, v) in bag {
                s += self.w[c * VOCAB + j as usize] * v;
            }
            *o = katgpt_core::exact_sigmoid(s);
        }
    }

    /// The serving pick: argmax score, ties to the lowest class index.
    pub fn pick(&self, bag: &[(u32, f32)]) -> usize {
        let mut scores = vec![0.0f32; self.labels.len()];
        self.scores_into(bag, &mut scores);
        argmax_lowest_tie(&scores)
    }
}

/// Argmax with ties resolved to the LOWEST index (the engine argmax law).
/// argmax with ties to the LOWEST index — crate-visible so the typed
/// specialist's position-space evals mirror the serving bridge's tie law
/// exactly ([`instinct_specialist_typed`]).
pub(crate) fn argmax_lowest_tie(scores: &[f32]) -> usize {
    let mut best = 0usize;
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > scores[best] {
            best = i;
        }
    }
    best
}

/// Warmup + cosine schedule (the `AdamW::lr_at` shape). Crate-visible so
/// the issue-579 NBSVM logistic counterpart shares the one schedule law.
pub(crate) fn lr_at(base: f32, step: u64, total_steps: u64, warmup: u64, min_ratio: f32) -> f32 {
    if warmup > 0 && step < warmup {
        return base * (step as f32 + 1.0) / warmup as f32;
    }
    let t = if total_steps > warmup {
        ((step - warmup) as f32 / (total_steps - warmup) as f32).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let cos = 0.5 * (1.0 + (std::f32::consts::PI * t).cos());
    base * (min_ratio + (1.0 - min_ratio) * cos)
}

/// One Arm A training run summary.
#[derive(Debug, Clone)]
pub struct ArmARun {
    pub model: Specialist,
    pub steps: usize,
    pub final_loss: f32,
    pub holdout_accuracy: f64,
    /// The lr=0 control: every parameter stays 0 → pick = class 0 → exactly
    /// the holdout share of class 0. The trained run must beat this.
    pub control_accuracy: f64,
    /// Rows skipped for zero events (pure punctuation / empty text) — loud,
    /// never silent.
    pub skipped_empty: usize,
}

/// Train Arm A: supervised one-vs-all logistic BCE over `train` rows
/// (indices into `rows`), scored on `holdout` indices. Bit-deterministic:
/// fixed row order, fixed op order, single thread, zero init.
pub fn train_arm_a(
    rows: &[SuiteRow],
    labels: &[String],
    train_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &ArmAConfig,
) -> Result<ArmARun, String> {
    let mut class_of: HashMap<&str, usize> = HashMap::with_capacity(labels.len());
    for (i, l) in labels.iter().enumerate() {
        class_of.insert(l.as_str(), i);
    }
    // Row-space gold targets (skipped-empty rows carry an unused entry).
    let mut gold: Vec<usize> = vec![0; rows.len()];
    for &i in train_idx {
        let c = class_of
            .get(rows[i].label.as_str())
            .ok_or_else(|| format!("row label {:?} outside the universe", rows[i].label))?;
        gold[i] = *c;
    }
    train_core(rows, labels, train_idx, holdout_idx, cfg, Targets::Gold(gold))
}

/// The per-row training target. Both variants are looked up in ROW space
/// (the original `rows` index) so the distilled rows read their teacher
/// vector directly.
enum Targets<'a> {
    /// One-hot gold (Arm A) — also `Distill` with `mix == 1.0` exactly.
    Gold(Vec<usize>),
    /// `t = mix·onehot(gold) + (1−mix)·q_row` (Arm B). `mix == 1.0` reduces
    /// to the exact one-hot (0.0 · finite q is exactly 0.0), so
    /// [`train_arm_b`] at `gold_mix = 1.0` must be bit-identical to
    /// [`train_arm_a`] — pinned by test.
    Distill {
        q: &'a TeacherDump,
        gold: Vec<usize>,
        mix: f32,
    },
}

impl Targets<'_> {
    #[inline]
    fn at(&self, row: usize, c: usize) -> f32 {
        match self {
            Targets::Gold(g) => {
                if c == g[row] {
                    1.0
                } else {
                    0.0
                }
            }
            Targets::Distill { q, gold, mix } => {
                let onehot = if c == gold[row] { 1.0 } else { 0.0 };
                *mix * onehot + (1.0 - *mix) * q.row(row)[c]
            }
        }
    }
}

/// The shared trainer core (Arm A gold / Arm B distilled soft targets):
/// lazy-AdamW one-vs-all logistic BCE, bit-deterministic (fixed row order,
/// fixed op order, single thread, zero init).
#[allow(clippy::too_many_arguments)]
fn train_core(
    rows: &[SuiteRow],
    labels: &[String],
    train_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &ArmAConfig,
    targets: Targets<'_>,
) -> Result<ArmARun, String> {
    let mut class_of: HashMap<&str, usize> = HashMap::with_capacity(labels.len());
    for (i, l) in labels.iter().enumerate() {
        class_of.insert(l.as_str(), i);
    }
    let l = labels.len();
    // Pre-tokenize once (memory: rows × nnz × 8 B — ag_news 20k × ~75 × 8 ≈ 12 MB).
    let mut bags: Vec<Vec<(u32, f32)>> = Vec::with_capacity(train_idx.len());
    let mut row_of: Vec<usize> = Vec::with_capacity(train_idx.len());
    let mut scratch = Vec::new();
    let mut skipped_empty = 0usize;
    for &i in train_idx {
        let mut bag = Vec::new();
        bag_into(rows[i].text.as_bytes(), &mut bag, &mut scratch);
        if bag.is_empty() {
            skipped_empty += 1;
            continue;
        }
        bags.push(bag);
        row_of.push(i);
    }
    if bags.is_empty() {
        return Err("every train row tokenized to zero events".into());
    }

    let mut model = Specialist::zero(labels);
    let mut opt_w: Vec<(f32, f32)> = vec![(0.0, 0.0); l * VOCAB]; // (m, v)
    let mut b_state: Vec<(f32, f32)> = vec![(0.0, 0.0); l];
    let total_steps = (cfg.epochs * bags.len()) as u64;
    let mut scores = vec![0.0f32; l];
    let mut step: u64 = 0;
    let mut final_loss = 0.0f32;
    let (beta1, beta2, eps) = (0.9f32, 0.999f32, 1e-8f32);
    for _epoch in 0..cfg.epochs {
        for (r, bag) in bags.iter().enumerate() {
            let row = row_of[r];
            // Forward: scores + the mean BCE over classes (for the summary).
            let mut loss = 0.0f32;
            for (c, o) in scores.iter_mut().enumerate() {
                let mut s = model.b[c];
                for &(j, val) in bag {
                    s += model.w[c * VOCAB + j as usize] * val;
                }
                *o = katgpt_core::exact_sigmoid(s);
                let y = targets.at(row, c);
                let p = *o;
                loss -= (y * p.max(f32::MIN_POSITIVE)
                    + (1.0 - y) * (1.0 - p).max(f32::MIN_POSITIVE))
                .ln();
            }
            final_loss = loss / l as f32;
            let lr = lr_at(cfg.lr, step, total_steps, cfg.warmup_steps, cfg.min_lr_ratio);
            step += 1;
            let bc1 = 1.0 - beta1.powf(step as f32);
            let bc2 = 1.0 - beta2.powf(step as f32);
            // Lazy AdamW per touched (class, bucket); decay decoupled on
            // touched params only (documented lazy-Adam caveat).
            for (c, &s) in scores.iter().enumerate() {
                let ds = (s - targets.at(row, c)) / l as f32;
                for &(j, val) in bag {
                    let p = c * VOCAB + j as usize;
                    let g = ds * val;
                    let st = &mut opt_w[p];
                    st.0 = beta1 * st.0 + (1.0 - beta1) * g;
                    st.1 = beta2 * st.1 + (1.0 - beta2) * g * g;
                    let mhat = st.0 / bc1;
                    let vhat = st.1 / bc2;
                    let decay = cfg.weight_decay * model.w[p];
                    model.w[p] -= lr * (mhat / (vhat.sqrt() + eps) + decay);
                }
            }
            // Biases: dense Adam (every class touched every row).
            for (c, &s) in scores.iter().enumerate() {
                let ds = (s - targets.at(row, c)) / l as f32;
                let st = &mut b_state[c];
                st.0 = beta1 * st.0 + (1.0 - beta1) * ds;
                st.1 = beta2 * st.1 + (1.0 - beta2) * ds * ds;
                let mhat = st.0 / bc1;
                let vhat = st.1 / bc2;
                model.b[c] -= lr * (mhat / (vhat.sqrt() + eps));
            }
        }
    }
    let mut bag = Vec::new();
    let mut scratch2 = Vec::new();
    let mut hit = 0usize;
    for &i in holdout_idx {
        bag_into(rows[i].text.as_bytes(), &mut bag, &mut scratch2);
        if !bag.is_empty() && model.pick(&bag) == class_of[rows[i].label.as_str()] {
            hit += 1;
        }
    }
    let holdout_accuracy = hit as f64 / holdout_idx.len().max(1) as f64;
    let class0 = holdout_idx
        .iter()
        .filter(|&&i| class_of[rows[i].label.as_str()] == 0)
        .count();
    let control_accuracy = class0 as f64 / holdout_idx.len().max(1) as f64;
    Ok(ArmARun {
        model,
        steps: step as usize,
        final_loss,
        holdout_accuracy,
        control_accuracy,
        skipped_empty,
    })
}

// ── the Arm-B teacher dump (riir-reflex `harness --distill`) ───────────

/// The teacher-pass file magic — "RIDT": Riir-train Instinct Distill
/// Teacher (written by riir-reflex `src/harness/runner/distill.rs`, the
/// same bytes both sides).
pub const TEACHER_MAGIC: [u8; 4] = *b"RIDT";
pub const TEACHER_VERSION: u32 = 1;

/// One suite's laya teacher probabilities over its train rows, in ROW
/// order (the survivor sequence both sides derive identically from the
/// sorted `train-*.json` pages).
#[derive(Debug, Clone)]
pub struct TeacherDump {
    /// The label universe, SORTED (the riir-train `label_universe` law —
    /// the student asserts exact equality, order included).
    pub classes: Vec<String>,
    /// Per row: gold class index (`< classes.len()`).
    pub gold: Vec<u32>,
    /// Per row: the teacher's argmax class (the sanity stat only).
    pub picks: Vec<u32>,
    /// Flat row-major `n_rows × n_classes` soft targets.
    pub q: Vec<f32>,
}

impl TeacherDump {
    #[must_use]
    pub fn n_rows(&self) -> usize {
        self.gold.len()
    }

    #[must_use]
    pub fn n_classes(&self) -> usize {
        self.classes.len()
    }

    /// Row `i`'s soft-target slice.
    #[must_use]
    pub fn row(&self, i: usize) -> &[f32] {
        let c = self.n_classes();
        &self.q[i * c..(i + 1) * c]
    }
}

/// Load + fully verify one suite's teacher dump: BLAKE3 sidecar first
/// (the seal is over the exact file bytes), then the structural checks
/// (magic, version, universe, per-row ranges, finite q). `path` is the
/// `.bin`; the sidecar is `path.blake3` (hex text).
pub fn load_teacher_dump(path: &Path) -> Result<TeacherDump, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let sidecar = std::path::PathBuf::from(format!("{}.blake3", path.display()));
    let expected = std::fs::read_to_string(&sidecar)
        .map_err(|e| format!("read {}: {e} — run the teacher pass first", sidecar.display()))?
        .trim()
        .to_string();
    let actual = blake3::hash(&bytes).to_hex().to_string();
    if actual != expected {
        return Err(format!(
            "{}: BLAKE3 seal mismatch — expected {expected}, got {actual}",
            path.display()
        ));
    }

    let err = |m: String| format!("{}: {m}", path.display());
    if bytes.len() < 12 || bytes[..4] != TEACHER_MAGIC {
        return Err(err(format!("bad magic (expected {TEACHER_MAGIC:?})")));
    }
    let version = u32::from_le_bytes(
        bytes[4..8].try_into().map_err(|_| err("truncated version".into()))?,
    );
    if version != TEACHER_VERSION {
        return Err(err(format!("unsupported version {version}")));
    }
    let header_len = u32::from_le_bytes(
        bytes[8..12].try_into().map_err(|_| err("truncated header len".into()))?,
    ) as usize;
    let body_start = 12usize
        .checked_add(header_len)
        .ok_or_else(|| err("header len overflow".into()))?;
    if bytes.len() < body_start {
        return Err(err("truncated header".into()));
    }
    let header: serde_json::Value = serde_json::from_slice(&bytes[12..body_start])
        .map_err(|e| err(format!("bad header json: {e}")))?;
    let classes: Vec<String> = serde_json::from_value(
        header.get("classes").cloned().ok_or_else(|| err("header: no classes".into()))?,
    )
    .map_err(|e| err(format!("header classes: {e}")))?;
    if classes.is_empty() {
        return Err(err("empty class universe".into()));
    }
    let mut sorted = classes.clone();
    sorted.sort();
    sorted.dedup();
    if sorted.len() != classes.len() {
        return Err(err("duplicate class in the universe".into()));
    }
    let n_classes = classes.len();
    let n_rows = (bytes.len() - body_start) / (8 + 4 * n_classes);
    if (bytes.len() - body_start) % (8 + 4 * n_classes) != 0 {
        return Err(err(format!(
            "body is not a whole number of {}-byte row(s) for {n_classes} class(es)",
            8 + 4 * n_classes
        )));
    }
    if header
        .get("n_rows")
        .and_then(serde_json::Value::as_u64)
        .is_some_and(|n| n as usize != n_rows)
    {
        return Err(err("header n_rows disagrees with the body size".into()));
    }

    let mut gold = Vec::with_capacity(n_rows);
    let mut picks = Vec::with_capacity(n_rows);
    let mut q = Vec::with_capacity(n_rows * n_classes);
    let mut off = body_start;
    for r in 0..n_rows {
        let g = u32::from_le_bytes(
            bytes[off..off + 4].try_into().map_err(|_| err(format!("row {r}: truncated")))?,
        );
        let p = u32::from_le_bytes(
            bytes[off + 4..off + 8]
                .try_into()
                .map_err(|_| err(format!("row {r}: truncated")))?,
        );
        if g as usize >= n_classes {
            return Err(err(format!("row {r}: gold {g} out of range")));
        }
        if p as usize >= n_classes {
            return Err(err(format!("row {r}: pick {p} out of range")));
        }
        off += 8;
        for c in 0..n_classes {
            let v = f32::from_le_bytes(
                bytes[off..off + 4]
                    .try_into()
                    .map_err(|_| err(format!("row {r} class {c}: truncated")))?,
            );
            if !v.is_finite() {
                return Err(err(format!("row {r} class {c}: non-finite q")));
            }
            q.push(v);
            off += 4;
        }
        gold.push(g);
        picks.push(p);
    }
    Ok(TeacherDump { classes, gold, picks, q })
}

/// Arm B knobs: the [`ArmAConfig`] schedule + the gold mix. `gold_mix =
/// 0.0` is pure distillation (the issue's protocol); `1.0` reduces the
/// loss to Arm A's exactly (pinned by test).
#[derive(Debug, Clone, Copy)]
pub struct ArmBConfig {
    pub base: ArmAConfig,
    pub gold_mix: f32,
}

impl Default for ArmBConfig {
    fn default() -> Self {
        Self { base: ArmAConfig::default(), gold_mix: 0.0 }
    }
}

/// Train Arm B: the one-vs-all logistic distilled from the laya teacher's
/// probabilities (`t = mix·onehot + (1−mix)·q`) over the same train rows
/// Arm A sees. Validates the JOIN before training: the dump's class
/// universe must equal `labels` (order included), every dump row must
/// carry this suite's row label, and the dump must cover every row.
pub fn train_arm_b(
    rows: &[SuiteRow],
    labels: &[String],
    teacher: &TeacherDump,
    train_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &ArmBConfig,
) -> Result<ArmARun, String> {
    if teacher.classes.len() != labels.len() || teacher.classes.iter().zip(labels).any(|(a, b)| a != b) {
        return Err(format!(
            "teacher/universe join failed: dump classes {:?} vs labels {:?} — order and \
             spelling must match exactly",
            teacher.classes.first().map(String::as_str),
            labels.first().map(String::as_str)
        ));
    }
    if teacher.n_rows() != rows.len() {
        return Err(format!(
            "teacher/rows join failed: dump covers {} row(s), the suite has {}",
            teacher.n_rows(),
            rows.len()
        ));
    }
    for (i, row) in rows.iter().enumerate() {
        let g = teacher.gold[i] as usize;
        if labels[g] != row.label {
            return Err(format!(
                "teacher/rows join failed at row {i}: dump gold {:?} vs row label {:?}",
                labels[g], row.label
            ));
        }
    }
    let gold: Vec<usize> = teacher.gold.iter().map(|&g| g as usize).collect();
    train_core(
        rows,
        labels,
        train_idx,
        holdout_idx,
        &cfg.base,
        Targets::Distill { q: teacher, gold, mix: cfg.gold_mix },
    )
}

// ── the frozen artifact (the rule_embed_frozen law) ─────────────────────

/// Artifact magic — "RISP": Riir-train Instinct SPecialist.
pub const ARTIFACT_MAGIC: [u8; 4] = *b"RISP";
pub const ARTIFACT_VERSION: u8 = 1;

fn quantize_row(w: &[f32]) -> (f32, Vec<i8>) {
    let max_abs = w.iter().fold(0.0f32, |m, &x| m.max(x.abs()));
    if max_abs == 0.0 {
        return (1.0, vec![0i8; w.len()]);
    }
    let scale = max_abs / 127.0;
    let q = w
        .iter()
        .map(|&x| (x / scale).round().clamp(-127.0, 127.0) as i8)
        .collect();
    (scale, q)
}

/// Canonical frozen bytes: magic + version + payload + BLAKE3 seal over
/// the payload. A pure function of the inputs (fixed order, no RNG).
/// WEIGHts ARE BYTES — the artifact is written to disk, never committed.
pub fn encode_artifact(suite: &str, model: &Specialist) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(suite.as_bytes());
    payload.push(0);
    payload.extend_from_slice(&(model.labels.len() as u32).to_le_bytes());
    for label in &model.labels {
        payload.extend_from_slice(label.as_bytes());
        payload.push(0);
    }
    payload.extend_from_slice(&(VOCAB as u32).to_le_bytes());
    payload.extend_from_slice(&model.b.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
    for c in 0..model.labels.len() {
        let (scale, q) = quantize_row(&model.w[c * VOCAB..(c + 1) * VOCAB]);
        payload.extend_from_slice(&scale.to_le_bytes());
        payload.extend(q.iter().map(|&x| x as u8));
    }
    let mut out = Vec::with_capacity(payload.len() + 5 + 32);
    out.extend_from_slice(&ARTIFACT_MAGIC);
    out.push(ARTIFACT_VERSION);
    out.extend_from_slice(&payload);
    out.extend_from_slice(blake3::hash(&payload).as_bytes());
    out
}

/// Decode + verify the seal. Returns `(suite, dequantized_model)`.
pub fn decode_artifact(bytes: &[u8]) -> Result<(String, Specialist), String> {
    if bytes.len() < 5 + 32 {
        return Err("artifact too short".into());
    }
    if bytes[..4] != ARTIFACT_MAGIC {
        return Err("bad magic".into());
    }
    if bytes[4] != ARTIFACT_VERSION {
        return Err(format!("unsupported version {}", bytes[4]));
    }
    let body = &bytes[5..bytes.len() - 32];
    let seal = &bytes[bytes.len() - 32..];
    if blake3::hash(body).as_bytes() != seal {
        return Err("BLAKE3 seal mismatch — the bytes are not the sealed payload".into());
    }
    let mut cur = 0usize;
    let suite = {
        let start = cur;
        while cur < body.len() && body[cur] != 0 {
            cur += 1;
        }
        if cur >= body.len() {
            return Err("unterminated suite name".into());
        }
        let s = std::str::from_utf8(&body[start..cur]).map_err(|e| e.to_string())?;
        cur += 1;
        s.to_string()
    };
    if body.len() < cur + 4 {
        return Err("truncated before class count".into());
    }
    let l = u32::from_le_bytes(body[cur..cur + 4].try_into().unwrap()) as usize;
    cur += 4;
    let mut labels = Vec::with_capacity(l);
    for _ in 0..l {
        let start = cur;
        while cur < body.len() && body[cur] != 0 {
            cur += 1;
        }
        if cur >= body.len() {
            return Err("unterminated label".into());
        }
        labels.push(
            std::str::from_utf8(&body[start..cur])
                .map_err(|e| e.to_string())?
                .to_string(),
        );
        cur += 1;
    }
    if body.len() < cur + 4 {
        return Err("truncated before vocab".into());
    }
    let v = u32::from_le_bytes(body[cur..cur + 4].try_into().unwrap()) as usize;
    cur += 4;
    if v != VOCAB {
        return Err(format!(
            "artifact vocab {v} != this build's {VOCAB} — lexicon fork"
        ));
    }
    if body.len() < cur + l * 4 + l * (4 + v) {
        return Err("truncated weight block".into());
    }
    let mut b = Vec::with_capacity(l);
    for c in 0..l {
        b.push(f32::from_le_bytes(
            body[cur + c * 4..cur + c * 4 + 4].try_into().unwrap(),
        ));
    }
    cur += l * 4;
    let mut w = vec![0.0f32; l * v];
    for c in 0..l {
        let scale = f32::from_le_bytes(body[cur..cur + 4].try_into().unwrap());
        cur += 4;
        for (j, q) in body[cur..cur + v].iter().enumerate() {
            w[c * v + j] = (*q as i8) as f32 * scale;
        }
        cur += v;
    }
    Ok((suite, Specialist { labels, w, b }))
}

// ── tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_law_structure() {
        // Structural pins: the hash function's total spread is pinned by
        // the live cross-check against the serving side at P3; here the
        // LAW is pinned — lowercasing in-hash, trimming, bigram pairing,
        // duplicate preservation.
        assert_eq!(
            fnv1a_word(b"Hello", WORD_SALT),
            fnv1a_word(b"hello", WORD_SALT),
            "lowered in-hash"
        );
        let mut ev = Vec::new();
        events_into(b"Hello, world hello", &mut ev);
        assert_eq!(ev.len(), 5, "3 unigrams + 2 bigrams, duplicates kept");
        // Emission order (the reflex law): per token the UNIGRAM then its
        // bigram with the previous word — [u1, u2, b12, u3, b23].
        let (w1, w2, w3) = (ev[0], ev[1], ev[3]);
        assert_ne!(w1, w2);
        assert_ne!(w2, w3);
        assert_eq!(w1, w3, "hello twice → same unigram id");
        events_into(b"!!! ...", &mut ev);
        assert!(ev.is_empty(), "pure punctuation yields no events");
    }

    #[test]
    fn bag_is_sorted_and_l2_normalized() {
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        bag_into(b"b a a", &mut bag, &mut scratch);
        // 5 events (b, a, bi(b,a), a, bi(a,a)); collisions would merge
        // buckets with weights summed, so only invariants are asserted.
        assert!(!bag.is_empty());
        assert!(bag.windows(2).all(|w| w[0].0 < w[1].0), "strictly sorted");
        assert!(bag.iter().all(|(_, w)| *w > 0.0));
        let n2: f32 = bag.iter().map(|(_, w)| w * w).sum::<f32>().sqrt();
        assert!((n2 - 1.0).abs() < 1e-5, "L2 normalized: {n2}");
    }

    #[test]
    fn holdout_round_robins_and_partitions() {
        let rows: Vec<SuiteRow> = ["a", "a", "a", "b", "b", "a", "b"]
            .into_iter()
            .enumerate()
            .map(|(i, l)| SuiteRow {
                label: l.to_string(),
                text: format!("row {i}"),
                presented: None,
            })
            .collect();
        let (holdout, rest) = stratified_holdout(&rows, 4);
        // Round robin a,b,a,b over the first-appearance buckets, pick order.
        assert_eq!(holdout, vec![0, 3, 1, 4]);
        assert_eq!(holdout.len() + rest.len(), rows.len());
        assert!(holdout.iter().all(|h| !rest.contains(h)), "disjoint");
        let mut all = holdout.clone();
        all.extend_from_slice(&rest);
        all.sort_unstable();
        assert_eq!(all, (0..rows.len()).collect::<Vec<_>>(), "exact partition");
    }

    #[test]
    fn trainer_learns_a_two_class_toy_and_beats_the_lr0_control() {
        let rows: Vec<SuiteRow> = (0..40)
            .map(|i| {
                if i % 2 == 0 {
                    SuiteRow {
                        label: "pos".into(),
                        text: "excellent wonderful fantastic superb".into(),
                        presented: None,
                    }
                } else {
                    SuiteRow {
                        label: "neg".into(),
                        text: "terrible awful horrid dreadful".into(),
                        presented: None,
                    }
                }
            })
            .collect();
        let labels = label_universe(&rows);
        assert_eq!(labels, vec!["neg", "pos"]);
        let (holdout, train) = stratified_holdout(&rows, 8);
        let cfg = ArmAConfig { epochs: 3, ..ArmAConfig::default() };
        let run = train_arm_a(&rows, &labels, &train, &holdout, &cfg).unwrap();
        assert!(
            run.holdout_accuracy > run.control_accuracy,
            "trained {} must beat the lr=0 control {}",
            run.holdout_accuracy,
            run.control_accuracy
        );
        assert!(run.holdout_accuracy > 0.5, "toy must be learnable: {}", run.holdout_accuracy);
        // Determinism: a second identical run is bit-identical.
        let again = train_arm_a(&rows, &labels, &train, &holdout, &cfg).unwrap();
        assert_eq!(run.model.w, again.model.w, "bit-deterministic training");
        assert_eq!(run.model.b, again.model.b);
    }

    // ── Arm B / teacher dump ─────────────────────────────────────

    /// Hand-build one teacher-dump file + its BLAKE3 sidecar (the reflex
    /// writer's exact bytes; no shared encoder to drift).
    fn write_teacher_dump(
        path: &std::path::Path,
        classes: &[&str],
        rows: &[(u32, u32, Vec<f32>)],
    ) {
        let mut body = Vec::new();
        body.extend_from_slice(b"RIDT");
        body.extend_from_slice(&1u32.to_le_bytes());
        let header = serde_json::json!({ "classes": classes, "n_rows": rows.len() });
        let hb = serde_json::to_string(&header).unwrap().into_bytes();
        body.extend_from_slice(&(hb.len() as u32).to_le_bytes());
        body.extend_from_slice(&hb);
        for (g, p, q) in rows {
            body.extend_from_slice(&g.to_le_bytes());
            body.extend_from_slice(&p.to_le_bytes());
            for v in q {
                body.extend_from_slice(&v.to_le_bytes());
            }
        }
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, &body).unwrap();
        std::fs::write(
            format!("{}.blake3", path.display()),
            format!("{}\n", blake3::hash(&body).to_hex()),
        )
        .unwrap();
    }

    fn toy_rows() -> Vec<SuiteRow> {
        (0..40)
            .map(|i| {
                if i % 2 == 0 {
                    SuiteRow {
                        label: "pos".into(),
                        text: "excellent wonderful fantastic superb".into(),
                        presented: None,
                    }
                } else {
                    SuiteRow {
                        label: "neg".into(),
                        text: "terrible awful horrid dreadful".into(),
                        presented: None,
                    }
                }
            })
            .collect()
    }

    /// The toy teacher: 0.9 on gold, 0.1 on the other class, in
    /// first-appearance class order [neg, pos].
    fn toy_teacher_rows(rows: &[SuiteRow]) -> Vec<(u32, u32, Vec<f32>)> {
        rows.iter()
            .map(|r| {
                if r.label == "pos" {
                    (1, 1, vec![0.1, 0.9])
                } else {
                    (0, 0, vec![0.9, 0.1])
                }
            })
            .collect()
    }

    #[test]
    fn teacher_dump_round_trips_and_refuses_corruption() {
        let dir = std::env::temp_dir().join(format!("instinct_teacher_test_rt_{}", std::process::id()));
        let path = dir.join("toy_teacher.bin");
        let rows = toy_teacher_rows(&toy_rows());
        write_teacher_dump(&path, &["neg", "pos"], &rows);

        let dump = load_teacher_dump(&path).expect("load");
        assert_eq!(dump.classes, vec!["neg".to_string(), "pos".to_string()]);
        assert_eq!(dump.n_rows(), 40);
        assert_eq!(dump.n_classes(), 2);
        assert_eq!(dump.gold[0], 1);
        assert_eq!(dump.row(0), &[0.1, 0.9]);
        assert_eq!(dump.row(1), &[0.9, 0.1]);

        // Corruption anywhere is refused by the seal.
        let mut corrupt = std::fs::read(&path).unwrap();
        let mid = 12 + (corrupt.len() - 12) / 2;
        corrupt[mid] ^= 0xff;
        std::fs::write(&path, &corrupt).unwrap();
        let e = load_teacher_dump(&path).unwrap_err();
        assert!(e.contains("BLAKE3 seal mismatch"), "{e}");

        // A missing sidecar refuses loud (never a silent pass).
        let body = std::fs::read(&path).unwrap();
        std::fs::remove_file(format!("{}.blake3", path.display())).unwrap();
        std::fs::write(&path, &body).unwrap(); // restore the body, NOT the sidecar
        let e = load_teacher_dump(&path).unwrap_err();
        assert!(e.contains("run the teacher pass first"), "{e}");

        // Bad magic refuses (consistent seal — the magic check is the one
        // under test; the seal check runs first).
        write_teacher_dump(&path, &["neg", "pos"], &rows);
        let mut bad = std::fs::read(&path).unwrap();
        bad[0] = b'X';
        std::fs::write(&path, &bad).unwrap();
        std::fs::write(
            format!("{}.blake3", path.display()),
            format!("{}\n", blake3::hash(&bad).to_hex()),
        )
        .unwrap();
        let e = load_teacher_dump(&path).unwrap_err();
        assert!(e.contains("bad magic"), "{e}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn arm_b_gold_mix_one_is_bit_identical_to_arm_a() {
        let rows = toy_rows();
        let labels = label_universe(&rows);
        let (holdout, train) = stratified_holdout(&rows, 8);
        let dir = std::env::temp_dir().join(format!("instinct_teacher_test_mix1_{}", std::process::id()));
        let path = dir.join("mix1_teacher.bin");
        write_teacher_dump(&path, &labels.iter().map(String::as_str).collect::<Vec<_>>(), &toy_teacher_rows(&rows));
        let dump = load_teacher_dump(&path).unwrap();

        let cfg = ArmAConfig { epochs: 3, ..ArmAConfig::default() };
        let a = train_arm_a(&rows, &labels, &train, &holdout, &cfg).unwrap();
        let b = train_arm_b(
            &rows,
            &labels,
            &dump,
            &train,
            &holdout,
            &ArmBConfig { base: cfg, gold_mix: 1.0 },
        )
        .unwrap();
        assert_eq!(a.model.w, b.model.w, "gold_mix 1.0 must reduce to Arm A bit-exactly");
        assert_eq!(a.model.b, b.model.b);
        assert_eq!(a.final_loss, b.final_loss);
        assert_eq!(a.holdout_accuracy, b.holdout_accuracy);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn arm_b_learns_a_smooth_teacher_and_beats_the_lr0_control() {
        let rows = toy_rows();
        let labels = label_universe(&rows);
        let (holdout, train) = stratified_holdout(&rows, 8);
        let dir = std::env::temp_dir().join(format!("instinct_teacher_test_pure_{}", std::process::id()));
        let path = dir.join("pure_teacher.bin");
        write_teacher_dump(&path, &labels.iter().map(String::as_str).collect::<Vec<_>>(), &toy_teacher_rows(&rows));
        let dump = load_teacher_dump(&path).unwrap();

        let cfg = ArmAConfig { epochs: 3, ..ArmAConfig::default() };
        let b = train_arm_b(
            &rows,
            &labels,
            &dump,
            &train,
            &holdout,
            &ArmBConfig { base: cfg, gold_mix: 0.0 },
        )
        .unwrap();
        assert!(
            b.holdout_accuracy > b.control_accuracy,
            "distilled {} must beat the lr=0 control {}",
            b.holdout_accuracy,
            b.control_accuracy
        );
        assert!(b.holdout_accuracy > 0.5, "smooth-teacher toy must be learnable");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn teacher_join_is_pinned() {
        let rows = toy_rows();
        let labels = label_universe(&rows);
        let (holdout, train) = stratified_holdout(&rows, 8);
        let dump_ok = TeacherDump {
            classes: labels.clone(),
            gold: toy_teacher_rows(&rows).into_iter().map(|(g, _, _)| g).collect(),
            picks: vec![0; rows.len()],
            q: toy_teacher_rows(&rows).into_iter().flat_map(|(_, _, q)| q).collect(),
        };
        let cfg = ArmBConfig::default();

        // Class order flipped = a different universe → refused.
        let flipped = TeacherDump { classes: vec!["pos".into(), "neg".into()], ..dump_ok.clone() };
        assert!(train_arm_b(&rows, &labels, &flipped, &train, &holdout, &cfg).is_err());

        // A mislabeled row → refused (row 0 is "pos", claim "neg").
        let mut bad_gold = dump_ok.clone();
        bad_gold.gold[0] = 0;
        assert!(train_arm_b(&rows, &labels, &bad_gold, &train, &holdout, &cfg).is_err());

        // Coverage short by one → refused.
        let mut short = dump_ok;
        short.gold.pop();
        short.q.truncate(short.q.len() - 2);
        short.picks.pop();
        assert!(train_arm_b(&rows, &labels, &short, &train, &holdout, &cfg).is_err());
    }

    #[test]
    fn artifact_round_trips_and_refuses_corruption() {
        let h = fnv1a_word(b"good", WORD_SALT) as usize % VOCAB;
        let model = Specialist {
            labels: vec!["neg".into(), "pos".into()],
            w: {
                let mut w = vec![0.0f32; 2 * VOCAB];
                w[h] = 3.0; // class 0 ("neg") likes "good" — the round trip moves it
                w
            },
            b: vec![0.25, -0.5],
        };
        let bytes = encode_artifact("toy", &model);
        let (suite, decoded) = decode_artifact(&bytes).unwrap();
        assert_eq!(suite, "toy");
        assert_eq!(decoded.labels, model.labels);
        assert_eq!(decoded.b, model.b);
        // Quantization is lossy but bounded: |q·scale − w| ≤ scale/2.
        let scale = 3.0f32 / 127.0;
        assert!((decoded.w[h] - 3.0).abs() <= scale * 0.51);
        // Corruption anywhere in the body is refused by the seal.
        let mut corrupt = bytes.clone();
        let mid = 5 + (bytes.len() - 5 - 32) / 2;
        corrupt[mid] ^= 0xff;
        assert!(decode_artifact(&corrupt).is_err());
        // Truncation is refused.
        assert!(decode_artifact(&bytes[..bytes.len() - 8]).is_err());
    }
}

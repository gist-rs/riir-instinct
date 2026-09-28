//! The frozen-artifact staleness probe (Issue 012 / Plan 005) — the soft
//! early-warning readout beside the hard pick-parity gate
//! (`tests/serve_gates.rs`). arXiv:2609.30652's fixed-trace probe, modelless
//! form: score a pinned probe set with the LIVE artifact and a REFERENCE
//! artifact and measure per-item divergence on the lane's own readout.
//!
//! Scope (recorded, not accidental): ARTIFACT-space. The specialist sigmoid
//! score IS the artifact-owned surface serving inherits (the position scores
//! are a subset; the reflex priors do not drift with the artifact), so an
//! artifact-pair divergence measured here is exactly the drift a swap would
//! carry into serving. Position-space (presented-option) metrics are a
//! follow-up if the report earns a swap-path hook.
//!
//! Everything here is REPORT-ONLY: nothing serves, nothing swaps, nothing
//! writes state. Determinism is a gate — the same pair + fixture produces a
//! byte-identical report, and a byte-identical pair produces EXACT-ZERO
//! divergence (the free canary).

use crate::specialist::{BagConvention, Specialist, VOCAB};
use serde::Serialize;
use std::path::Path;

/// The pre-registered fire rule (Plan 005 T1, pinned before any run): any
/// pick flip fires hard; mean |Δgold| ≥ this threshold fires soft. The
/// threshold is a 2-point sigmoid shift on the gold class — material at
/// serving margins, far under any plausible flip band.
pub const FIRE_GOLD_DELTA: f64 = 0.02;

/// One pinned probe question: the raw serving text (what `bag_into` hashes),
/// the presentation-space gold key, and the gold's position in the
/// presented-option order (the Issue-006 gold space).
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct ProbeItem {
    pub id: String,
    pub text: String,
    pub gold_key: String,
    pub gold_pos: usize,
}

/// One suite's slice of the probe set: the fixed presented-option universe
/// (the builders present the same keys for every case — "never shuffled" is
/// each builder's own contract) and the stratified item list.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct SuiteProbe {
    pub suite: String,
    pub presented_keys: Vec<String>,
    pub items: Vec<ProbeItem>,
}

/// The pinned probe set. Labels were spent ONCE at record time (the fixture
/// build); probe time is gold-free in the paper's sense — nothing here
/// consults the datasets.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct ProbeSet {
    pub generated: String,
    pub source: String,
    pub suites: Vec<SuiteProbe>,
}

impl ProbeSet {
    /// The canonical serialization (struct field order — serde_json keeps
    /// declaration order; `preserve_order` holds the value maps too) and its
    /// BLAKE3 digest. The digest IS the fixture's identity: a report records
    /// it, and a changed fixture changes every digest it rides.
    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("probe set serializes")
    }

    pub fn digest_hex(&self) -> String {
        let json = self.canonical_json();
        let mut out = String::with_capacity(64);
        for b in blake3::hash(json.as_bytes()).as_bytes() {
            use std::fmt::Write as _;
            let _ = write!(out, "{b:02x}");
        }
        out
    }

    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        let set: ProbeSet =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))?;
        Ok(set)
    }
}

/// The artifact-space readout for one probe item under one artifact: the
/// gold class's sigmoid score (when the gold resolved), the top-1 class row,
/// its score, and the runner-up's (margin = top1 − runner; 0.0 for a
/// single-class universe).
#[derive(Debug, Clone, PartialEq)]
pub struct Readout {
    pub gold_score: Option<f32>,
    pub top1_class: usize,
    pub top1_score: f32,
    pub runner_score: f32,
}

/// The per-item divergence between two readouts of the same question.
#[derive(Debug, Clone, Serialize)]
pub struct ItemDivergence {
    pub id: String,
    pub gold_delta: f32,
    pub pick_flip: bool,
    pub margin_delta: f32,
}

/// One artifact pair's verdict row over one suite's probe slice.
#[derive(Debug, Clone, Serialize)]
pub struct PairReport {
    pub suite: String,
    pub live_name: String,
    pub live_digest: String,
    pub live_convention: &'static str,
    pub ref_name: String,
    pub ref_digest: String,
    pub ref_convention: &'static str,
    pub fixture_digest: String,
    pub items: usize,
    pub label_universe: LabelUniverse,
    /// Per-item divergences, sorted by |gold_delta| descending — the read
    /// order (worst first).
    pub divergences: Vec<ItemDivergence>,
    pub flips: usize,
    pub mean_abs_gold_delta: f64,
    pub max_abs_gold_delta: f64,
    pub mean_abs_margin_delta: f64,
    pub fired: bool,
    pub fire_reason: &'static str,
}

/// The two artifacts' label universes, compared by name. A universe delta is
/// itself a probe finding (an artifact whose labels the seat cannot present).
#[derive(Debug, Clone, Serialize)]
pub struct LabelUniverse {
    pub live_labels: usize,
    pub ref_labels: usize,
    pub identical: bool,
    /// Labels carried by one side only (capped — a malformed artifact could
    /// carry hundreds; the count is the finding, a few examples the context).
    pub live_only: Vec<String>,
    pub ref_only: Vec<String>,
}

/// Score one artifact over one probe item in ARTIFACT space: full-universe
/// scores, the engine argmax law (strictly greater, ties → lowest class —
/// the same walk [`crate::specialist::Specialist::pick`] runs), the gold
/// class resolved by name against THIS artifact's universe.
fn score_item(
    spec: &Specialist,
    conv: BagConvention,
    text: &str,
    gold_row: usize,
    bag: &mut Vec<(u32, f32)>,
    scratch: &mut Vec<u32>,
    scores: &mut Vec<f32>,
) -> Readout {
    conv.bag_into(text.as_bytes(), bag, scratch);
    scores.clear();
    scores.resize(spec.labels.len(), 0.0);
    spec.scores_into(bag, scores);
    let mut best = 0usize;
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > scores[best] {
            best = i;
        }
    }
    let runner = scores
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != best)
        .map(|(_, &s)| s)
        .fold(f32::NEG_INFINITY, f32::max);
    Readout {
        gold_score: Some(scores[gold_row]),
        top1_class: best,
        top1_score: scores[best],
        runner_score: if spec.labels.len() > 1 { runner } else { 0.0 },
    }
}

/// The gold key's class row in ONE artifact's universe: by name, trying the
/// three spellings the seat↔artifact join actually carries (raw, spaces→
/// underscores, underscores→spaces); then the Issue-006 identity-by-count
/// rule (the presented-key count equals the label count → the gold POSITION
/// is the class row). Refuses loud when unresolvable — a silently wrong gold
/// row would poison every gold delta downstream.
fn resolve_gold(spec: &Specialist, item: &ProbeItem, presented: &[String]) -> Result<usize, String> {
    // Bound first — the two `.replace()` spellings are owned temporaries and
    // a slice literal borrowing them inside the loop head is E0716.
    let spaced_to_under = item.gold_key.replace(' ', "_");
    let under_to_spaced = item.gold_key.replace('_', " ");
    for key in [
        item.gold_key.as_str(),
        spaced_to_under.as_str(),
        under_to_spaced.as_str(),
    ] {
        if let Some(pos) = spec.labels.iter().position(|l| l == key) {
            return Ok(pos);
        }
    }
    if presented.len() == spec.labels.len() {
        return Ok(item.gold_pos);
    }
    Err(format!(
        "item {}: gold key {:?} resolves to no class row of {:?} ({} labels; {} presented; \
         spellings tried: raw / '_' / ' ') and the identity-by-count rule does not apply",
        item.id,
        item.gold_key,
        spec.suite,
        spec.labels.len(),
        presented.len()
    ))
}

/// The label-universe comparison (by name).
fn label_universe(live: &Specialist, reference: &Specialist) -> LabelUniverse {
    let live_only: Vec<String> = live
        .labels
        .iter()
        .filter(|l| !reference.labels.contains(l))
        .cloned()
        .collect();
    let ref_only: Vec<String> = reference
        .labels
        .iter()
        .filter(|l| !live.labels.contains(l))
        .cloned()
        .collect();
    LabelUniverse {
        live_labels: live.labels.len(),
        ref_labels: reference.labels.len(),
        identical: live_only.is_empty() && ref_only.is_empty(),
        live_only,
        ref_only,
    }
}

/// The artifact digest (bytes → BLAKE3 hex). The report records both sides'
/// digests so a row is reproducible from the bytes alone.
pub fn artifact_digest(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut out = String::with_capacity(64);
    for b in blake3::hash(&bytes).as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(out, "{b:02x}");
    }
    Ok(out)
}

/// One side of a comparison: the loaded artifact, its OWN bag convention
/// (a v1 count-bag artifact read through a presence bag is a model that was
/// never trained), its display name, and its byte digest.
#[derive(Clone, Copy)]
pub struct PairSide<'a> {
    pub spec: &'a Specialist,
    pub conv: BagConvention,
    pub name: &'a str,
    pub digest: &'a str,
}

/// Compare LIVE vs REFERENCE over one suite's probe slice — the staleness
/// probe (a frozen reference against what serving carries) AND the pre-swap
/// impact report (a candidate against what serving carries; Issue 012 part
/// 2, REPORT-ONLY by law: the T2/LB95 gates own the promote/demote verdict,
/// this is the early-warning column beside them).
pub fn compare_pair(
    suite_probe: &SuiteProbe,
    live: PairSide<'_>,
    reference: PairSide<'_>,
    fixture_digest: &str,
) -> Result<PairReport, String> {
    let PairSide {
        spec: live_spec,
        conv: live_conv,
        name: live_name,
        digest: live_digest,
    } = live;
    let PairSide {
        spec: reference,
        conv: ref_conv,
        name: ref_name,
        digest: ref_digest,
    } = reference;
    if live_spec.suite != suite_probe.suite || reference.suite != suite_probe.suite {
        return Err(format!(
            "compare_pair: suite mismatch — probe {:?}, live {:?}, reference {:?}",
            suite_probe.suite, live_spec.suite, reference.suite
        ));
    }
    let universe = label_universe(live_spec, reference);
    let mut divergences = Vec::with_capacity(suite_probe.items.len());
    let mut bag = Vec::with_capacity(256);
    let mut scratch = Vec::with_capacity(VOCAB / 8);
    let mut scores_live = Vec::new();
    let mut scores_ref = Vec::new();
    let mut flips = 0usize;
    let mut gold_sum = 0.0f64;
    let mut gold_max = 0.0f64;
    let mut margin_sum = 0.0f64;
    for item in &suite_probe.items {
        let gold_live = resolve_gold(live_spec, item, &suite_probe.presented_keys)?;
        let gold_ref = resolve_gold(reference, item, &suite_probe.presented_keys)?;
        let a = score_item(
            live_spec,
            live_conv,
            &item.text,
            gold_live,
            &mut bag,
            &mut scratch,
            &mut scores_live,
        );
        let b = score_item(
            reference,
            ref_conv,
            &item.text,
            gold_ref,
            &mut bag,
            &mut scratch,
            &mut scores_ref,
        );
        let gold_delta = match (a.gold_score, b.gold_score) {
            (Some(x), Some(y)) => x - y,
            _ => 0.0,
        };
        // A flip is a flip in LIVE CLASS ROWS; with identical universes the
        // rows are the same labels, so a row flip IS a label flip. With a
        // divergent universe the row comparison is still well-defined (the
        // report's universe block names the delta) but flips there speak
        // rows, not labels — the row says which.
        let flip = a.top1_class != b.top1_class;
        if flip {
            flips += 1;
        }
        let margin_a = a.top1_score - a.runner_score;
        let margin_b = b.top1_score - b.runner_score;
        margin_sum += (margin_a - margin_b).abs() as f64;
        let abs_gold = (gold_delta as f64).abs();
        gold_sum += abs_gold;
        gold_max = gold_max.max(abs_gold);
        divergences.push(ItemDivergence {
            id: item.id.clone(),
            gold_delta,
            pick_flip: flip,
            margin_delta: margin_a - margin_b,
        });
    }
    divergences.sort_by(|a, b| {
        b.gold_delta
            .abs()
            .partial_cmp(&a.gold_delta.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let n = divergences.len().max(1) as f64;
    let mean_gold = gold_sum / n;
    let (fired, fire_reason) = if flips > 0 {
        (true, "pick flip")
    } else if mean_gold >= FIRE_GOLD_DELTA {
        (true, "mean |gold delta| over the pre-registered bar")
    } else {
        (false, "quiet")
    };
    Ok(PairReport {
        suite: suite_probe.suite.clone(),
        live_name: live_name.to_string(),
        live_digest: live_digest.to_string(),
        live_convention: live_conv.name(),
        ref_name: ref_name.to_string(),
        ref_digest: ref_digest.to_string(),
        ref_convention: ref_conv.name(),
        fixture_digest: fixture_digest.to_string(),
        items: divergences.len(),
        label_universe: universe,
        divergences,
        flips,
        mean_abs_gold_delta: mean_gold,
        max_abs_gold_delta: gold_max,
        mean_abs_margin_delta: margin_sum / n,
        fired,
        fire_reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny synthetic artifact (2 labels, deterministic weights) so the
    /// divergence maths are pinned without any real file on disk. Each
    /// class's FULL VOCAB-width row is set to its weight, so the score is
    /// sigmoid(w_c · Σbag + b_c) whatever bucket the text hashes to — the
    /// row-major `class * VOCAB` layout [`Specialist::score_class`] reads.
    fn synthetic(labels: &[&str], w0: f32, w1: f32) -> Specialist {
        Specialist {
            suite: "synthetic".into(),
            labels: labels.iter().map(|s| s.to_string()).collect(),
            w: {
                let mut w = vec![w0; VOCAB];
                w.extend(std::iter::repeat_n(w1, VOCAB));
                w
            },
            b: vec![0.0, 0.0],
        }
    }

    fn probe() -> SuiteProbe {
        SuiteProbe {
            suite: "synthetic".into(),
            presented_keys: vec!["alpha".into(), "beta".into()],
            items: vec![ProbeItem {
                id: "s:0".into(),
                text: "alpha".into(),
                gold_key: "alpha".into(),
                gold_pos: 0,
            }],
        }
    }

    fn side<'a>(spec: &'a Specialist, conv: BagConvention, name: &'a str, digest: &'a str) -> PairSide<'a> {
        PairSide { spec, conv, name, digest }
    }

    #[test]
    fn identical_artifacts_read_exactly_zero() {
        let a = synthetic(&["alpha", "beta"], 1.0, 1.0);
        let b = synthetic(&["alpha", "beta"], 1.0, 1.0);
        let p = probe();
        let r = compare_pair(
            &p,
            side(&a, BagConvention::Count, "a", "da"),
            side(&b, BagConvention::Count, "b", "db"),
            "fx",
        )
        .expect("compare");
        assert!(!r.fired, "identical pair must stay quiet");
        assert_eq!(r.flips, 0);
        assert_eq!(r.mean_abs_gold_delta, 0.0);
        assert!(r.divergences.iter().all(|d| d.gold_delta == 0.0));
        assert!(r.divergences.iter().all(|d| d.margin_delta == 0.0));
    }

    #[test]
    fn weight_delta_moves_gold_and_the_fire_rule_holds() {
        let live = synthetic(&["alpha", "beta"], 4.0, 1.0);
        let reference = synthetic(&["alpha", "beta"], 1.0, 1.0);
        let p = probe();
        let r = compare_pair(
            &p,
            side(&live, BagConvention::Count, "live", "dl"),
            side(&reference, BagConvention::Count, "ref", "dr"),
            "fx",
        )
        .expect("compare");
        assert!(r.mean_abs_gold_delta.abs() > 0.0, "a weight delta must move gold");
        // The fire rule is pre-registered: flips OR mean |Δgold| ≥ 0.02.
        assert_eq!(r.fired, r.flips > 0 || r.mean_abs_gold_delta >= FIRE_GOLD_DELTA);
    }

    #[test]
    fn unresolvable_gold_refuses_loud() {
        let live = synthetic(&["alpha", "beta"], 1.0, 1.0);
        let reference = synthetic(&["alpha", "beta"], 1.0, 1.0);
        let mut p = probe();
        p.presented_keys = vec!["alpha".into()]; // 1 presented vs 2 labels → no identity rule
        p.items[0].gold_key = "gamma".into();
        assert!(compare_pair(
            &p,
            side(&live, BagConvention::Count, "l", "d"),
            side(&reference, BagConvention::Count, "r", "d2"),
            "fx"
        )
        .is_err());
    }

    #[test]
    fn fixture_digest_is_stable_across_round_trip() {
        let mut set = ProbeSet {
            generated: "t".into(),
            source: "t".into(),
            suites: vec![probe()],
        };
        let d1 = set.digest_hex();
        let json = set.canonical_json();
        let back: ProbeSet = serde_json::from_str(&json).expect("round trip");
        let d2 = back.digest_hex();
        assert_eq!(d1, d2, "digest must survive a serialize round trip");
        set.suites[0].items[0].text = "changed".into();
        assert_ne!(set.digest_hex(), d1, "a content change must move the digest");
    }
}

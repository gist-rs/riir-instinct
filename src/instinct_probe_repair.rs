//! Issue 028 T1 — the G0 collision census (Research 653 §5, the class-2
//! leg of the twinarm probe-repair PoC): bucket member tables over the
//! trainer's OWN tokenizer law ([`instinct_specialist::source_events_into`],
//! the same walk [`instinct_specialist::events_into`] speaks), colliding
//! pairs flagged when two high-support members disagree on their dominant
//! class, and the collision cost priced by the **un-collided oracle** —
//! ONE diagnostic retrain per suite that splits every flagged member into
//! its own fresh bucket (a different feature map, NEVER shipped).
//!
//! # Pre-registered reading (pinned BEFORE the first run — no re-tuning)
//!
//! - **Member support** = the number of TRAIN rows whose event stream
//!   contains the source (row presence, not event count — one long document
//!   cannot swamp the class profile). Per-class support: the same count per
//!   gold class.
//! - **High-support** = support ≥ [`S_MIN`] (5).
//! - **Dominant class** = argmax of the per-class profile, ties to the
//!   LOWEST class index (the engine argmax law).
//! - **Qualifying pair** = two distinct sources sharing a bucket, both
//!   high-support, dominants differing. A bucket is flagged when ≥ 1
//!   qualifying pair exists; the flagged set is the connected components
//!   over qualifying pairs, and EVERY member of a multi-member component
//!   is remapped (so every qualifying pair is split, and split members
//!   cannot re-collide with each other).
//! - **Fresh buckets** come from the suite's UNUSED pool — buckets no
//!   source of ANY loaded row maps to (train + holdout both feed the
//!   oracle's tokenization) — assigned in ascending bucket order to
//!   members in ascending `(bigram, hash)` order. Deterministic, and a
//!   remapped source can never collide with anything.
//! - **Arms** = [`instinct_specialist::train_arm_a`] (incumbent map) vs
//!   [`instinct_specialist::train_arm_a_remap`] (oracle map), SAME rows,
//!   SAME [`ArmAConfig::default()`] (the Bench-608 recipe), SAME
//!   [`instinct_specialist::stratified_holdout`] split (budget
//!   [`HOLDOUT_BUDGET`], the Bench-608 law). The trainer is
//!   bit-deterministic, so the only difference between arms is the
//!   flagged members' buckets.
//! - **Materiality** = paired holdout delta (oracle − incumbent) ≥
//!   [`MATERIAL_PT`] points AND the one-sided 95% LCB of the per-row
//!   delta > 0. Per-suite `≈ 0` (immaterial) = anything else — a NEGATIVE
//!   delta is also immaterial (a merged column acting as a useful feature
//!   is not a collision cost). **Fold law**: every censused suite
//!   immaterial ⇒ class 2 folds as a recorded negative.
//! - **Level readout** — the census prices a total ceiling per suite and
//!   makes no dynamics claim; per-pair retrains are a FOLLOW-ON gated on
//!   a material total (Research 653 §5), not part of T1.
//!
//! Source identity is the full 64-bit law hash plus the kind bit; two
//! distinct sources colliding in all 64 bits merge into one member (the
//! incumbent bucket law already merges them implicitly; probability
//! ~n²/2⁶⁴ — disclosed, not guarded).
//!
//! NOT in T1: the tune/eval proportional-alternation split (binds G1/G2,
//! arrives with the repair pass), the grammar sidecar (T5), any repair
//! emission. This module PRICES; it never edits weights.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use serde::Serialize;

use crate::instinct_specialist::{
    ArmAConfig, ArmARun, BucketRemap, SourceEvent, SuiteRow, VOCAB, load_suite_rows,
    source_events_into, stratified_holdout, train_arm_a, train_arm_a_remap,
};

/// The pinned high-support threshold (rows).
pub const S_MIN: usize = 5;
/// The pinned holdout budget — the Bench-608 stratified-front law.
pub const HOLDOUT_BUDGET: usize = 200;
/// The pinned materiality bar, in holdout-accuracy POINTS (×100).
pub const MATERIAL_PT: f64 = 1.0;
/// Two-sided 95% normal multiplier for the paired-delta CI.
const Z975: f64 = 1.959_964;
/// One-sided 95% normal multiplier for the paired-delta LCB.
const Z95: f64 = 1.644_854;

/// The census population — the six Bench-608 suites plus the Issue-578
/// prompt_injections suite (the same loader + feature law).
/// `typed_decisions` is a DISCLOSED EXCLUSION: its row space rides the
/// typed loader (`instinct_specialist_typed`), a separate feature law;
/// it can join a follow-up cell if the lane survives T1.
pub const DEFAULT_SUITES: [&str; 7] = [
    "ag_news",
    "emotion",
    "sst5",
    "massive_intent_en",
    "banking77",
    "xnli_en",
    "prompt_injections",
];

/// The pinned census knobs. No CLI overrides — the pre-registration is
/// the contract; a changed threshold is a NEW pre-registration.
#[derive(Debug, Clone, Copy)]
pub struct CensusConfig {
    pub s_min: usize,
    pub holdout_budget: usize,
}

impl Default for CensusConfig {
    fn default() -> Self {
        Self {
            s_min: S_MIN,
            holdout_budget: HOLDOUT_BUDGET,
        }
    }
}

/// Per-member train-side accumulation (row-presence law).
#[derive(Debug, Clone)]
struct MemberAcc {
    support: u32,
    per_class: Vec<u32>,
}

/// One high-support censused member.
#[derive(Debug, Clone, Serialize)]
pub struct MemberStat {
    /// The full 64-bit law hash.
    pub hash: u64,
    /// Unigram (false) or bigram (true).
    pub bigram: bool,
    /// Train rows containing the source.
    pub support: u32,
    /// Per-class train-row presence (label = first-appearance index).
    pub per_class: Vec<u32>,
    /// argmax of `per_class`, ties to the LOWEST index.
    pub dominant: usize,
}

/// One flagged bucket: its qualifying high-support members (a connected
/// component over qualifying pairs) and the pair count it was flagged by.
#[derive(Debug, Clone, Serialize)]
pub struct FlaggedBucket {
    pub bucket: u32,
    pub qualifying_pairs: usize,
    /// In source-key `(hash, bigram)` ascending order — the remap
    /// assignment order.
    pub members: Vec<MemberStat>,
}

/// One trained arm's readout (the census-relevant projection of
/// [`ArmARun`]).
#[derive(Debug, Clone, Serialize)]
pub struct ArmReadout {
    pub holdout_acc: f64,
    pub control_acc: f64,
    pub final_loss: f32,
    pub pick_brier: f64,
}

/// The paired per-row readout over the shared holdout (same order both
/// arms — the same `holdout_idx` drives both runs).
#[derive(Debug, Clone, Serialize)]
pub struct PairedDelta {
    /// Mean per-row delta (oracle − incumbent), accuracy fraction.
    pub delta: f64,
    /// Two-sided 95% CI of the mean delta (normal approx, n ≥ 2).
    pub ci95: [f64; 2],
    /// One-sided 95% LCB of the mean delta.
    pub lcb95: f64,
    /// Rows where the oracle picked right and the incumbent did not.
    pub oracle_better: usize,
    /// Rows where the incumbent picked right and the oracle did not.
    pub incumbent_better: usize,
    /// Holdout rows scored (incl. empty-bag rows, which both arms skip
    /// identically and therefore never enter the discordant counts).
    pub n: usize,
}

/// The per-suite verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SuiteVerdict {
    Material,
    Immaterial,
}

/// One suite's full census.
#[derive(Debug, Clone, Serialize)]
pub struct SuiteCensus {
    pub suite: String,
    pub rows: usize,
    pub labels: usize,
    pub train_rows: usize,
    pub holdout_rows: usize,
    /// Train-side events scanned (duplicates included).
    pub events_train: u64,
    /// Train-side DISTINCT sources (the member table population).
    pub distinct_sources: usize,
    /// Buckets used by ANY loaded row (the complement is the fresh pool).
    pub buckets_used: usize,
    /// The fresh-pool capacity (`VOCAB − buckets_used`).
    pub pool_size: usize,
    pub flagged_buckets: Vec<FlaggedBucket>,
    pub qualifying_pairs: usize,
    pub remapped_members: usize,
    pub incumbent: ArmReadout,
    pub oracle: ArmReadout,
    pub paired: PairedDelta,
    pub verdict: SuiteVerdict,
}

/// Record-level provenance (machine-emitted, never hand-typed).
#[derive(Debug, Clone, Serialize)]
pub struct RecordMeta {
    pub datasets_dir: String,
    pub s_min: usize,
    pub holdout_budget: usize,
    pub material_pt: f64,
    pub trainer: String,
}

/// Label universe in FIRST-APPEARANCE order (the trainer's convention —
/// [`crate::instinct_specialist`] builds `class_of` from the labels slice
/// it is handed, so the census owns the order and passes it to both arms).
fn label_universe(rows: &[SuiteRow]) -> Vec<String> {
    let mut labels: Vec<String> = Vec::new();
    let mut seen: HashSet<&str> = HashSet::new();
    for r in rows {
        if seen.insert(r.label.as_str()) {
            labels.push(r.label.clone());
        }
    }
    labels
}

/// The pure flag half: group high-support members by bucket, find
/// qualifying pairs, take connected components. Returns buckets ascending,
/// members within a bucket in source-key `(hash, bigram)` ascending order
/// — the remap assignment order.
fn flag_buckets(member_acc: &HashMap<(u64, bool), MemberAcc>, s_min: usize) -> Vec<FlaggedBucket> {
    let dominant = |acc: &MemberAcc| -> usize {
        let mut best = 0usize;
        for (c, &s) in acc.per_class.iter().enumerate().skip(1) {
            if s > acc.per_class[best] {
                best = c;
            }
        }
        best
    };
    let mut by_bucket: HashMap<u32, Vec<(u64, bool)>> = HashMap::new();
    for (&key, acc) in member_acc {
        if acc.support as usize >= s_min {
            let bucket = (key.0 % VOCAB as u64) as u32;
            by_bucket.entry(bucket).or_default().push(key);
        }
    }
    let mut out = Vec::new();
    for (bucket, mut keys) in by_bucket {
        keys.sort_unstable(); // (hash, bigram) ascending
        let n = keys.len();
        if n < 2 {
            continue;
        }
        let doms: Vec<usize> = keys.iter().map(|k| dominant(&member_acc[k])).collect();
        // Union-find over qualifying pairs; roots join toward the lower
        // root (deterministic component identity).
        let mut parent: Vec<usize> = (0..n).collect();
        let mut pairs = 0usize;
        for (i, &di) in doms.iter().enumerate() {
            for (j, &dj) in doms.iter().enumerate().skip(i + 1) {
                if di != dj {
                    pairs += 1;
                    let mut a = i;
                    while parent[a] != a {
                        a = parent[a];
                    }
                    let mut b = j;
                    while parent[b] != b {
                        b = parent[b];
                    }
                    if a != b {
                        parent[a.max(b)] = a.min(b);
                    }
                }
            }
        }
        if pairs == 0 {
            continue;
        }
        // Component sizes: singletons never joined a pair — only members
        // of a size ≥ 2 component are flagged.
        let mut size = vec![0usize; n];
        for i in 0..n {
            let mut a = i;
            while parent[a] != a {
                a = parent[a];
            }
            size[a] += 1;
        }
        let members: Vec<MemberStat> = keys
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                let mut a = *i;
                while parent[a] != a {
                    a = parent[a];
                }
                size[a] >= 2
            })
            .map(|(_, k)| {
                let acc = &member_acc[k];
                MemberStat {
                    hash: k.0,
                    bigram: k.1,
                    support: acc.support,
                    per_class: acc.per_class.clone(),
                    dominant: dominant(acc),
                }
            })
            .collect();
        out.push(FlaggedBucket {
            bucket,
            qualifying_pairs: pairs,
            members,
        });
    }
    out.sort_unstable_by_key(|b| b.bucket);
    out
}

/// The pure assignment half: every member of every flagged bucket takes
/// its own fresh bucket from the UNUSED pool (ascending pool order; the
/// members move in `(bucket asc, source-key asc)` order — the order the
/// flagger already produced). Loud error when the pool is too small
/// (impossible at 2¹⁷ — the guard is structural).
fn build_remap(
    flagged: &[FlaggedBucket],
    used_buckets: &HashSet<u32>,
) -> Result<(BucketRemap, usize), String> {
    let ordered: Vec<(u64, bool)> = flagged
        .iter()
        .flat_map(|b| b.members.iter().map(|m| (m.hash, m.bigram)))
        .collect();
    let need = ordered.len();
    let mut remap: BucketRemap = BucketRemap::with_capacity(need);
    if need == 0 {
        return Ok((remap, 0));
    }
    let mut next = 0usize;
    for fresh in 0..VOCAB as u32 {
        if used_buckets.contains(&fresh) {
            continue;
        }
        remap.insert(ordered[next], fresh);
        next += 1;
        if next == need {
            return Ok((remap, need));
        }
    }
    Err(format!(
        "unused-bucket pool exhausted: {next}/{need} assigned — the pool law is broken"
    ))
}

/// The paired per-row readout (normal-approximation paired stats).
/// Empty-bag holdout rows carry `hit = 0` in BOTH arms (the trainer's NaN
/// law) — they contribute 0 to the delta and never enter the discordants.
fn paired_delta(incumbent_hits: &[u8], oracle_hits: &[u8]) -> Result<PairedDelta, String> {
    let n = incumbent_hits.len();
    if n != oracle_hits.len() {
        return Err(format!(
            "paired vectors misaligned: {n} vs {}",
            oracle_hits.len()
        ));
    }
    if n < 2 {
        return Err("paired stats need n ≥ 2 holdout rows".into());
    }
    let diffs: Vec<f64> = oracle_hits
        .iter()
        .zip(incumbent_hits)
        .map(|(&o, &i)| f64::from(o) - f64::from(i))
        .collect();
    let mean = diffs.iter().sum::<f64>() / n as f64;
    let var = diffs.iter().map(|d| (d - mean) * (d - mean)).sum::<f64>() / (n - 1) as f64;
    let se = var.sqrt() / (n as f64).sqrt();
    let oracle_better = oracle_hits
        .iter()
        .zip(incumbent_hits)
        .filter(|(&o, &i)| o == 1 && i == 0)
        .count();
    let incumbent_better = oracle_hits
        .iter()
        .zip(incumbent_hits)
        .filter(|(&o, &i)| o == 0 && i == 1)
        .count();
    Ok(PairedDelta {
        delta: mean,
        ci95: [mean - Z975 * se, mean + Z975 * se],
        lcb95: mean - Z95 * se,
        oracle_better,
        incumbent_better,
        n,
    })
}

fn readout(run: &ArmARun) -> ArmReadout {
    ArmReadout {
        holdout_acc: run.holdout_accuracy,
        control_acc: run.control_accuracy,
        final_loss: run.final_loss,
        pick_brier: run.calibration.brier,
    }
}

/// The census over ALREADY-LOADED rows (the pure half — the e2e test
/// drives this directly; [`census_suite`] is the loader wrapper).
pub fn census_rows(
    suite: &str,
    rows: &[SuiteRow],
    cfg: &CensusConfig,
) -> Result<SuiteCensus, String> {
    let labels = label_universe(rows);
    let (holdout, train) = stratified_holdout(rows, cfg.holdout_budget);
    let mut in_train = vec![false; rows.len()];
    for &i in &train {
        in_train[i] = true;
    }
    let class_of: Vec<usize> = rows
        .iter()
        .map(|r| {
            labels
                .iter()
                .position(|l| l == &r.label)
                .expect("label from the universe")
        })
        .collect();

    // ── the scan: member table (train-side row presence) + used buckets ──
    let mut member_acc: HashMap<(u64, bool), MemberAcc> = HashMap::new();
    let mut used_buckets: HashSet<u32> = HashSet::new();
    let mut src: Vec<SourceEvent> = Vec::new();
    let mut row_seen: HashSet<(u64, bool)> = HashSet::new();
    let mut events_train = 0u64;
    for (i, row) in rows.iter().enumerate() {
        source_events_into(row.text.as_bytes(), &mut src);
        let mut row_events = 0usize;
        for e in &src {
            used_buckets.insert((e.hash % VOCAB as u64) as u32);
            row_events += 1;
            if in_train[i] && row_seen.insert((e.hash, e.bigram)) {
                let acc = member_acc
                    .entry((e.hash, e.bigram))
                    .or_insert_with(|| MemberAcc {
                        support: 0,
                        per_class: vec![0; labels.len()],
                    });
                acc.support += 1;
                acc.per_class[class_of[i]] += 1;
            }
        }
        if in_train[i] {
            events_train += row_events as u64;
        }
        row_seen.clear();
    }
    let distinct_sources = member_acc.len();

    // ── flag + remap ──
    let flagged = flag_buckets(&member_acc, cfg.s_min);
    let qualifying_pairs: usize = flagged.iter().map(|b| b.qualifying_pairs).sum();
    let (remap, remapped_members) = build_remap(&flagged, &used_buckets)?;
    let pool_size = VOCAB - used_buckets.len();

    // ── the two arms: ONE trainer, ONE split, ONE config; only the map moves ──
    let cfg_a = ArmAConfig::default();
    let incumbent = train_arm_a(rows, &labels, &train, &holdout, &cfg_a)?;
    let oracle = train_arm_a_remap(rows, &labels, &train, &holdout, &cfg_a, &remap)?;

    let paired = paired_delta(&incumbent.holdout_pick_hit, &oracle.holdout_pick_hit)?;
    let delta_pt = paired.delta * 100.0;
    let verdict = if delta_pt >= MATERIAL_PT && paired.lcb95 > 0.0 {
        SuiteVerdict::Material
    } else {
        SuiteVerdict::Immaterial
    };

    Ok(SuiteCensus {
        suite: suite.to_string(),
        rows: rows.len(),
        labels: labels.len(),
        train_rows: train.len(),
        holdout_rows: holdout.len(),
        events_train,
        distinct_sources,
        buckets_used: used_buckets.len(),
        pool_size,
        qualifying_pairs,
        remapped_members,
        flagged_buckets: flagged,
        incumbent: readout(&incumbent),
        oracle: readout(&oracle),
        paired,
        verdict,
    })
}

/// The census over one suite's train pages in a datasets dir.
pub fn census_suite(
    datasets_dir: &Path,
    suite: &str,
    cfg: &CensusConfig,
) -> Result<SuiteCensus, String> {
    let rows = load_suite_rows(datasets_dir, suite)?;
    census_rows(suite, &rows, cfg)
}

/// The markdown census record (machine-rendered from the measured
/// structs — numbers are never hand-typed into a record).
pub fn render_record(censuses: &[SuiteCensus], meta: &RecordMeta) -> String {
    let mut md = String::new();
    md.push_str("# G0 collision census — the un-collided oracle ceiling (issue 028 T1)\n\n");
    md.push_str(
        "**Pre-registration (pinned before the first run):** member support = train-row \
presence; high-support = support ≥ S_MIN; dominant = per-class argmax ties-to-lowest; \
qualifying pair = two high-support same-bucket members with differing dominants; the oracle \
retrain splits EVERY member of every flagged component into its own fresh bucket from the \
suite's UNUSED pool; arms = train_arm_a vs train_arm_a_remap over ArmAConfig::default() \
(the Bench-608 recipe) on the SAME stratified_holdout split; material = paired delta ≥ \
1.0 pt AND one-sided LCB95 > 0; every-suite-immaterial ⇒ class 2 folds (the issue's fold \
law). Level readout — no dynamics claim; per-pair retrains are a follow-on gated on a \
material total.\n\n",
    );
    md.push_str(&format!(
        "| datasets | S_MIN | holdout budget | material bar | trainer |\n|---|---|---|---|---|\n| {} | {} | {} | {} pt | {} |\n\n",
        meta.datasets_dir,
        meta.s_min,
        meta.holdout_budget,
        meta.material_pt,
        meta.trainer,
    ));
    md.push_str("## Per-suite totals\n\n");
    md.push_str("| suite | rows | labels | train/holdout | distinct src | buckets used (pool) | flagged buckets | pairs | remapped | incumbent acc | oracle acc | Δ pt | verdict |\n");
    md.push_str("|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for c in censuses {
        md.push_str(&format!(
            "| {} | {} | {} | {}/{} | {} | {} ({}) | {} | {} | {} | {:.4} | {:.4} | {:+.2} | {:?} |\n",
            c.suite,
            c.rows,
            c.labels,
            c.train_rows,
            c.holdout_rows,
            c.distinct_sources,
            c.buckets_used,
            c.pool_size,
            c.flagged_buckets.len(),
            c.qualifying_pairs,
            c.remapped_members,
            c.incumbent.holdout_acc,
            c.oracle.holdout_acc,
            c.paired.delta * 100.0,
            c.verdict,
        ));
    }
    md.push_str("\n## Paired reads (shared holdout, row-aligned)\n\n");
    md.push_str("| suite | Δ mean | 95% CI | one-sided LCB95 | oracle-only hits | incumbent-only hits | n |\n");
    md.push_str("|---|---|---|---|---|---|---|\n");
    for c in censuses {
        md.push_str(&format!(
            "| {} | {:+.4} | [{:+.4}, {:+.4}] | {:+.4} | {} | {} | {} |\n",
            c.suite,
            c.paired.delta,
            c.paired.ci95[0],
            c.paired.ci95[1],
            c.paired.lcb95,
            c.paired.oracle_better,
            c.paired.incumbent_better,
            c.paired.n,
        ));
    }
    md.push_str("\n## Flagged-bucket detail (top members per suite, by support)\n\n");
    for c in censuses {
        if c.flagged_buckets.is_empty() {
            md.push_str(&format!(
                "- **{}**: no flagged bucket at S_MIN — no qualifying pair exists.\n",
                c.suite
            ));
            continue;
        }
        md.push_str(&format!(
            "- **{}**: {} flagged bucket(s), {} remapped member(s). Largest:\n",
            c.suite,
            c.flagged_buckets.len(),
            c.remapped_members,
        ));
        let mut tops: Vec<&FlaggedBucket> = c.flagged_buckets.iter().collect();
        tops.sort_by_key(|b| {
            std::cmp::Reverse(b.members.iter().map(|m| m.support).max().unwrap_or(0))
        });
        for b in tops.iter().take(5) {
            let members: Vec<String> = b
                .members
                .iter()
                .map(|m| {
                    format!(
                        "{}#{:016x} sup={} dom={}",
                        if m.bigram { "bi" } else { "uni" },
                        m.hash,
                        m.support,
                        m.dominant
                    )
                })
                .collect();
            md.push_str(&format!(
                "  - bucket {} ({} pairs): {}\n",
                b.bucket,
                b.qualifying_pairs,
                members.join(" | "),
            ));
        }
    }
    let material: Vec<&str> = censuses
        .iter()
        .filter(|c| c.verdict == SuiteVerdict::Material)
        .map(|c| c.suite.as_str())
        .collect();
    md.push_str("\n## Fold verdict\n\n");
    if material.is_empty() {
        md.push_str(
            "**Every suite immaterial at 2¹⁷ ⇒ class 2 FOLDS as a recorded negative** \
(the collision cost of the frozen feature law is immaterial on these corpora). Class 1 is \
unaffected.\n",
        );
    } else {
        md.push_str(&format!(
            "**Class 2 STAYS OPEN** — material suite(s): {}. The census documents a structurally \
unremovable cost of the frozen feature law (input for the lexicon-fork-guard's cost \
accounting, never a weight-edit claim); per-pair retrains are the follow-on.\n",
            material.join(", "),
        ));
    }
    md
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instinct_specialist::{SuiteRow, bag_into, bag_into_remap, events_into};

    fn row(label: &str, text: &str) -> SuiteRow {
        SuiteRow {
            label: label.to_string(),
            text: text.to_string(),
            presented: None,
        }
    }

    /// The source projection's buckets must equal the bucket projection,
    /// event for event — one walk, two views.
    #[test]
    fn source_projection_matches_events_into() {
        let text = b"Alpha, beta! alpha beta gamma... gamma.";
        let mut buckets = Vec::new();
        events_into(text, &mut buckets);
        let mut sources = Vec::new();
        source_events_into(text, &mut sources);
        let mapped: Vec<u32> = sources
            .iter()
            .map(|e| (e.hash % VOCAB as u64) as u32)
            .collect();
        assert_eq!(buckets, mapped);
        assert!(
            sources.iter().any(|e| e.bigram),
            "bigrams exist in the sample"
        );
    }

    /// An EMPTY remap is the incumbent map — bit-identical bags.
    #[test]
    fn empty_remap_is_bag_into() {
        let text = b"the quick brown fox jumps over the lazy dog";
        let mut a = Vec::new();
        let mut s1 = Vec::new();
        bag_into(text, &mut a, &mut s1);
        let mut b = Vec::new();
        let mut s2 = Vec::new();
        let mut src = Vec::new();
        let remap = BucketRemap::new();
        bag_into_remap(text, &mut b, &mut s2, &mut src, &remap);
        assert_eq!(a, b);
    }

    /// A named source moves to its forced bucket; unnamed neighbors stay.
    #[test]
    fn remap_moves_only_named_sources() {
        let text = b"alpha beta";
        let mut sources = Vec::new();
        source_events_into(text, &mut sources);
        let alpha = sources
            .iter()
            .find(|e| !e.bigram)
            .expect("unigram exists")
            .hash;
        let beta = sources
            .iter()
            .filter(|e| !e.bigram)
            .find(|e| e.hash != alpha)
            .expect("second unigram exists")
            .hash;
        let alpha_bucket = (alpha % VOCAB as u64) as u32;
        let beta_bucket = (beta % VOCAB as u64) as u32;
        // A fresh bucket neither source uses (first candidate is
        // near-certain; the walk is the guard).
        let fresh = (0..VOCAB as u32)
            .find(|&c| c != alpha_bucket && c != beta_bucket)
            .expect("VOCAB has room");
        let mut remap = BucketRemap::new();
        remap.insert((alpha, false), fresh);
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        bag_into_remap(text, &mut bag, &mut scratch, &mut sources, &remap);
        assert!(
            bag.iter().any(|&(b, _)| b == fresh),
            "alpha sits at the fresh bucket"
        );
        assert!(
            !bag.iter().any(|&(b, _)| b == alpha_bucket),
            "old bucket abandoned"
        );
        // L2 law intact: the remapped bag is still unit-norm.
        let sumsq: f32 = bag.iter().map(|(_, w)| w * w).sum();
        assert!((sumsq - 1.0).abs() < 1e-5);
    }

    fn acc(support: u32, per_class: &[u32]) -> MemberAcc {
        MemberAcc {
            support,
            per_class: per_class.to_vec(),
        }
    }

    /// The flag law's known answer: two high-support same-bucket members
    /// with differing dominants qualify; a low-support neighbor never
    /// joins; same-dominant pairs never qualify.
    #[test]
    fn flag_law_known_answer() {
        let b0 = VOCAB as u64; // % VOCAB == 0
        let b0b = 2 * VOCAB as u64; // % VOCAB == 0
        let other = 3 * VOCAB as u64 + 1; // % VOCAB == 1
        let mut m: HashMap<(u64, bool), MemberAcc> = HashMap::new();
        m.insert((b0, false), acc(10, &[10, 0])); // dom 0
        m.insert((b0b, false), acc(8, &[0, 8])); // dom 1 — disagrees
        m.insert((b0b, true), acc(1, &[1, 0])); // LOW support — never flags
        m.insert((other, false), acc(9, &[9, 0])); // dom 0, own bucket
        let flagged = flag_buckets(&m, S_MIN);
        assert_eq!(flagged.len(), 1, "only bucket 0 flags");
        let f = &flagged[0];
        assert_eq!(f.bucket, 0);
        assert_eq!(f.qualifying_pairs, 1);
        assert_eq!(f.members.len(), 2, "low-support neighbor stays out");
        assert!(f.members.iter().all(|x| x.hash != b0b || !x.bigram));
        // Same-dominant pair never qualifies.
        let mut m2: HashMap<(u64, bool), MemberAcc> = HashMap::new();
        m2.insert((b0, false), acc(10, &[6, 4])); // dom 0
        m2.insert((b0b, false), acc(9, &[7, 2])); // dom 0 — agrees
        assert!(flag_buckets(&m2, S_MIN).is_empty());
    }

    /// The assignment law: deterministic, from the unused pool, one fresh
    /// bucket per member, ascending in (bucket, bigram, hash) order.
    #[test]
    fn remap_assignment_deterministic_and_sound() {
        let b0 = VOCAB as u64;
        let b0b = 2 * VOCAB as u64;
        let mut m: HashMap<(u64, bool), MemberAcc> = HashMap::new();
        m.insert((b0, false), acc(10, &[10, 0]));
        m.insert((b0b, true), acc(8, &[0, 8]));
        let flagged = flag_buckets(&m, S_MIN);
        let mut used = HashSet::new();
        used.insert((b0 % VOCAB as u64) as u32);
        used.insert(((b0b % VOCAB as u64) as u32).wrapping_add(1)); // neighbor bucket
        let (r1, n1) = build_remap(&flagged, &used).unwrap();
        let (r2, n2) = build_remap(&flagged, &used).unwrap();
        assert_eq!(n1, n2);
        assert_eq!(r1, r2, "assignment is deterministic");
        assert_eq!(n1, 2);
        let targets: Vec<u32> = r1.values().copied().collect();
        assert!(
            targets.iter().all(|t| !used.contains(t)),
            "fresh buckets are unused"
        );
        assert!(targets[0] != targets[1], "members split apart");
        // Ascending pool order: the LOWER-KEY member (b0 < b0b in hash
        // order, which is the flagger's member order) takes the smaller
        // fresh bucket.
        let first_target = r1[&(b0, false)];
        let second_target = r1[&(b0b, true)];
        assert!(first_target < second_target);
    }

    /// Zero flags ⇒ the oracle arm is bit-identical to the incumbent —
    /// the plumbing cannot move an unflagged source.
    #[test]
    fn zero_flags_oracle_bit_identical() {
        let rows: Vec<SuiteRow> = (0..8)
            .map(|i| {
                row(
                    if i % 2 == 0 { "a" } else { "b" },
                    &format!("doc {i} about topic {} with words w{i} common words", i % 2),
                )
            })
            .collect();
        let labels = vec!["a".to_string(), "b".to_string()];
        let (holdout, train) = stratified_holdout(&rows, 4);
        let cfg = crate::instinct_specialist::ArmAConfig {
            epochs: 1,
            warmup_steps: 0,
            ..crate::instinct_specialist::ArmAConfig::default()
        };
        let base = train_arm_a(&rows, &labels, &train, &holdout, &cfg).unwrap();
        let oracle =
            train_arm_a_remap(&rows, &labels, &train, &holdout, &cfg, &BucketRemap::new()).unwrap();
        assert_eq!(base.model.w, oracle.model.w);
        assert_eq!(base.model.b, oracle.model.b);
        assert_eq!(base.holdout_pick_hit, oracle.holdout_pick_hit);
    }

    /// End-to-end over synthetic rows: the census completes, the paired
    /// vectors align, and an unflaggable corpus reads immaterial.
    #[test]
    fn census_rows_end_to_end_immaterial_on_clean_corpus() {
        let rows: Vec<SuiteRow> = (0..40)
            .map(|i| {
                let label = ["x", "y", "z"][i % 3];
                row(
                    label,
                    &format!("sample {i} token{i}A token{i}B common {}", i % 3),
                )
            })
            .collect();
        // The corpus is tiny, so the SPLIT needs the small budget — the
        // default 200 would pull every row into the holdout and leave the
        // trainer nothing to train on (the census fails loud there).
        let cfg = CensusConfig {
            holdout_budget: 20,
            ..CensusConfig::default()
        };
        let c = census_rows("synthetic", &rows, &cfg).unwrap();
        assert_eq!(c.rows, 40);
        assert_eq!(c.labels, 3);
        assert_eq!(c.holdout_rows, 20);
        assert_eq!(c.train_rows, 20);
        assert_eq!(c.paired.n, 20);
        assert_eq!(
            c.verdict,
            SuiteVerdict::Immaterial,
            "no flags ⇒ delta 0 ⇒ immaterial"
        );
        assert_eq!(c.paired.delta, 0.0);
        assert_eq!(c.remapped_members, 0);
        let md = render_record(
            &[c],
            &RecordMeta {
                datasets_dir: "synthetic".into(),
                s_min: S_MIN,
                holdout_budget: HOLDOUT_BUDGET,
                material_pt: MATERIAL_PT,
                trainer: "test".into(),
            },
        );
        assert!(md.contains("FOLDS as a recorded negative"));
    }

    /// The paired-stats known answer.
    #[test]
    fn paired_delta_known_answer() {
        let i = [1u8, 0, 1, 1];
        let o = [1u8, 1, 1, 0];
        let p = paired_delta(&i, &o).unwrap();
        assert_eq!(p.delta, 0.0);
        assert_eq!(p.oracle_better, 1);
        assert_eq!(p.incumbent_better, 1);
        assert_eq!(p.n, 4);
        assert!(p.ci95[0] <= 0.0 && 0.0 <= p.ci95[1]);
        // A strict win reads positive: diffs [1,1,0,0] → mean 0.5, sd
        // 0.577, se 0.289, one-sided LCB 0.5 − 1.645·0.289 ≈ 0.025 > 0.
        let i2 = [0u8, 0, 1, 1];
        let o2 = [1u8, 1, 1, 1];
        let p2 = paired_delta(&i2, &o2).unwrap();
        assert!((p2.delta - 0.5).abs() < 1e-12);
        assert_eq!(p2.oracle_better, 2);
        assert!(
            p2.lcb95 > 0.0,
            "2/4 net wins at n=4 clears the one-sided bar"
        );
        // And a single flip does NOT (0.25 mean, LCB −0.161): the bar is
        // real, not decoration.
        let p3 = paired_delta(&i, &o2).unwrap();
        assert!(p3.lcb95 < 0.0);
    }
}

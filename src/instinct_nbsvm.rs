//! Issue 579 T1 — the emotion v2 lane: the reflex NBSVM ridge baseline
//! (`riir-reflex/src/nb_ridge.rs`, reimplemented law-for-law) plus the Arm
//! A logistic counterpart trained in the SAME NBSVM feature space — the
//! issue's named main lever.
//!
//! Two systems over one feature stream
//! ([`instinct_specialist::events_into`], the reflex `embed.rs` law — the
//! lexicon provably cannot fork from the modelless lane's):
//!
//! - **The baseline** ([`NbsvmRidge`]): binary presence features, top-k by
//!   (df desc, bucket asc), the α=1 NB log-count-ratio transform WITH its
//!   documented probe quirk verbatim (the probe's Python `tot[lab]` was a
//!   string-label missing-key read — so `nin = αV` and
//!   `nout = ndocs + αV`, CONSTANTS across classes; reflex measured the
//!   "fixed" per-class-total form pushing the rest-side denominator
//!   negative on this pool), shared Gram + per-class damped closed-form
//!   solve via `katgpt_core::linalg::ridge_solve_direct_f32`. Validation
//!   anchor (issue 579 T1 step 1): emotion test **0.8925** pure-argmax at
//!   k=2048/λ=10 — the `issue038_t7_probe.py` record.
//! - **The v2 candidate** ([`train_nbsvm`]): the 576 Arm A trainer (lazy
//!   AdamW one-vs-all BCE, zero-init, fixed order, bit-deterministic)
//!   with the bag scaled per class by the class's OWN ratio vector — the
//!   closed-form ridge IS exactly this shape, so the logistic variant is
//!   its stochastic counterpart (issue 579 T1 step 2). The fixed scaling
//!   FOLDS into the weights after training (`w′[c,b] = w[c,b]·r_c(b)`),
//!   so the returned model is a plain
//!   [`instinct_specialist::Specialist`] scoring PRESENCE bags — the T3
//!   artifact format is unchanged; only the input-bag convention moves
//!   (counts → presence), which [`NbsvmRun`] discloses.
//!
//! The gate (issue 579 T1 step 3): paired LB95 (v2 − baseline, per-row
//! correctness over the held-out TRAIN slice) > 0 — the
//! `stats::PairedDiff::lb95` law (riir-instinct, the Issue-008 T2 product
//! gate), reimplemented in [`paired_lb95`]; riir-train cannot dep
//! riir-instinct (the dependency flows the other way). The published
//! 0.8850 row is the ridge@8 BLEND posture; this baseline is the
//! pure-argmax system, which measured HIGHER on test (0.8925) — so the
//! holdout bar enforced here is the STRICTER of the two.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::instinct_specialist::{
    argmax_lowest_tie, events_into, lr_at, read_split_envelope, suite_row, Specialist, SuiteRow,
    VOCAB,
};

/// Ratio smoothing — the probe-validated add-one (α = 1.0), deliberately
/// NOT an observed-Laplace resolution (the reflex `fit` doc's recorded
/// refusal: the αV mass shrinks the rest-side denominator negative on
/// real pools).
pub const ALPHA: f32 = 1.0;

/// The one-sided 95% normal quantile — mirrored from riir-instinct
/// `stats.rs` (the law the Issue-008 T2 gate runs under).
pub const Z95: f64 = 1.959_963_984_540_054;

// ── paired stats (the riir-instinct `stats::PairedDiff` law) ─────────────

/// The paired difference A − B over per-row correctness, with the
/// ONE-SIDED 95% bounds: `lb95` certifies a strict win (the 579 holdout
/// gate), `ub95` caps a regression (reported for shape parity with the
/// cited law).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PairedDiff {
    pub mean: f64,
    pub se: f64,
    pub ub95: f64,
    pub lb95: f64,
}

/// Paired lower bound over two arms' per-row correctness (same rows both
/// arms — the outcomes are direct). `None` on length mismatch or empty.
#[must_use]
pub fn paired_lb95(a: &[bool], b: &[bool]) -> Option<PairedDiff> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    let n = a.len() as f64;
    let mut sum = 0.0f64;
    let mut sum2 = 0.0f64;
    for (x, y) in a.iter().zip(b.iter()) {
        let d = f64::from(i8::from(*x) - i8::from(*y));
        sum += d;
        sum2 += d * d;
    }
    let mean = sum / n;
    let var = if a.len() > 1 {
        ((sum2 - sum * sum / n) / (n - 1.0)).max(0.0)
    } else {
        0.0
    };
    let se = var.sqrt() / n.sqrt();
    Some(PairedDiff {
        mean,
        se,
        ub95: mean + Z95 * se,
        lb95: mean - Z95 * se,
    })
}

// ── bags ─────────────────────────────────────────────────────────────────

/// The presence bag: `(bucket, value)` SORTED by bucket, one entry per
/// distinct event (duplicates collapsed — binary presence, the NBSVM
/// feature convention), `value = 1.0` or, with `norm`, `1/√nnz` (the
/// L2-normalized presence variant). Cleared first; `scratch` reused.
pub fn presence_bag_into(
    text: &[u8],
    norm: bool,
    out: &mut Vec<(u32, f32)>,
    scratch: &mut Vec<u32>,
) {
    events_into(text, scratch);
    out.clear();
    scratch.sort_unstable();
    scratch.dedup();
    let v = if norm && !scratch.is_empty() {
        1.0 / (scratch.len() as f32).sqrt()
    } else {
        1.0
    };
    out.extend(scratch.iter().map(|&b| (b, v)));
}

/// Sorted unique event ids (the [`NbsvmRidge`] eval input: the doc's
/// presence set, ascending for the ridge's own binary search).
pub fn presence_ids_into(text: &[u8], out: &mut Vec<u32>, scratch: &mut Vec<u32>) {
    events_into(text, scratch);
    out.clear();
    out.extend_from_slice(scratch);
    out.sort_unstable();
    out.dedup();
}

// ── the closed-form baseline (the reflex `NbRidge` law) ─────────────────

/// The fitted one-vs-rest NBSVM ridge (immutable after the build;
/// read-only on the eval path). Pure-argmax system — no blend term: the
/// blend is reflex's SERVING posture (ridge σ blended with the nb/oc
/// terms), not this baseline's shape.
#[derive(Debug)]
pub struct NbsvmRidge {
    n_dom: usize,
    /// Selected buckets, ASCENDING (the eval path binary-searches).
    feats: Vec<u32>,
    /// Per-class ratio per selected feature (flat, feature-major `k × N`).
    ratios: Vec<f32>,
    /// Per-class weight rows over `[k features, bias]`, in the SCALED
    /// space (flat, class-major `(k + 1) × N`).
    weights: Vec<f32>,
    /// The λ the weights were solved with.
    lambda: f32,
}

impl NbsvmRidge {
    /// Feature count (k).
    #[must_use]
    pub fn feats_len(&self) -> usize {
        self.feats.len()
    }

    /// The λ the weights were solved with.
    #[must_use]
    pub fn lambda(&self) -> f32 {
        self.lambda
    }

    /// Debug/eval surface: the selected bucket ids (ascending).
    #[must_use]
    pub fn feats_dbg(&self) -> &[u32] {
        &self.feats
    }

    /// Class `d`'s ratio for one bucket (test/differential surface).
    #[must_use]
    pub fn ratio_at(&self, d: usize, bucket: u32) -> Option<f32> {
        let i = self.feats.binary_search(&bucket).ok()?;
        Some(self.ratios[i * self.n_dom + d])
    }

    /// Per-class ridge scores for one doc's PRESENCE ids (sorted unique):
    /// `w_d · (presence ⊙ r_d) + bias_d`. Positive magnitudes carry no
    /// calibrated meaning — this baseline is consumed by pure argmax.
    pub fn scores_into(&self, presence_sorted: &[u32], out: &mut [f32]) {
        let k = self.feats.len();
        let stride = k + 1;
        for (d, o) in out.iter_mut().enumerate() {
            let row = d * stride;
            let mut s = self.weights[row + k];
            for &w in presence_sorted {
                let Ok(i) = self.feats.binary_search(&w) else {
                    continue;
                };
                s += self.weights[row + i] * self.ratios[i * self.n_dom + d];
            }
            *o = s;
        }
    }

    /// The pick: argmax score, ties to the LOWEST class index (the engine
    /// argmax law — numpy's first-max, the probe's convention).
    #[must_use]
    pub fn pick(&self, presence_sorted: &[u32]) -> usize {
        let mut scores = vec![0.0f32; self.n_dom];
        self.scores_into(presence_sorted, &mut scores);
        argmax_lowest_tie(&scores)
    }

    /// Fit one-vs-rest ridge rows from labeled rows (grouped internally by
    /// class, row order preserved within class). λ is the per-class damped
    /// solve's ridge (`lambda + I` on ALL stride diagonal entries,
    /// bias included — the probe's `lam * np.eye(k+1)`).
    ///
    /// Errors when a row's label is outside `labels`, or (loud assert) when
    /// the pool shape drives a log-count ratio non-finite.
    pub fn fit(
        rows: &[SuiteRow],
        labels: &[String],
        lambda: f32,
        top_k: usize,
    ) -> Result<NbsvmRidge, String> {
        let n_dom = labels.len();
        let mut class_of: HashMap<&str, usize> = HashMap::with_capacity(n_dom);
        for (i, l) in labels.iter().enumerate() {
            class_of.insert(l.as_str(), i);
        }
        // Pass 1: tokenize once; per-class + global counts, document
        // frequencies, all over the raw 2^17 stream (the reflex shape).
        let mut tok_cnt = vec![0u32; n_dom * VOCAB];
        let mut glob_cnt = vec![0u32; VOCAB];
        let mut df = vec![0u32; VOCAB];
        let mut doc_count = 0usize;
        let mut rows_per_class: Vec<Vec<Vec<u32>>> = vec![Vec::new(); n_dom];
        let mut buf = Vec::new();
        for row in rows {
            let d = *class_of
                .get(row.label.as_str())
                .ok_or_else(|| format!("row label {:?} outside the universe", row.label))?;
            events_into(row.text.as_bytes(), &mut buf);
            for &w in &buf {
                tok_cnt[d * VOCAB + w as usize] += 1;
                glob_cnt[w as usize] += 1;
            }
            let mut seen = buf.clone();
            seen.sort_unstable();
            seen.dedup();
            for &w in &seen {
                df[w as usize] += 1;
            }
            doc_count += 1;
            rows_per_class[d].push(std::mem::take(&mut buf));
        }

        // Feature selection: (df desc, bucket asc) over touched buckets,
        // truncate to top_k, then RE-INDEX ascending (the eval path
        // binary-searches — the reflex measured 0.29-class bug when this
        // sort was missing).
        let mut order: Vec<u32> =
            (0..VOCAB as u32).filter(|&w| df[w as usize] > 0).collect();
        order.sort_unstable_by(|&a, &b| {
            df[b as usize].cmp(&df[a as usize]).then(a.cmp(&b))
        });
        order.truncate(top_k);
        order.sort_unstable();
        let k = order.len();

        // Per-class ratios over the selected features. ⚠ MIRROR THE PROBE
        // VERBATIM — quirk included: `nin = αV` and `nout = ndocs + αV`,
        // NO per-class totals anywhere (a string-label missing-key read,
        // not a design choice; the 0.8925 emotion reading was produced by
        // exactly this arithmetic and "fixing" it breaks the premise).
        let av = ALPHA * VOCAB as f32;
        let nin = av;
        let nout = doc_count as f32 + av;
        let mut ratios = vec![0f32; k * n_dom];
        for j in 0..n_dom {
            for (i, &w) in order.iter().enumerate() {
                let cin = tok_cnt[j * VOCAB + w as usize] as f32 + ALPHA;
                let cout =
                    (glob_cnt[w as usize] - tok_cnt[j * VOCAB + w as usize]) as f32 + ALPHA;
                let r = (cin / nin).ln() - (cout / nout).ln();
                assert!(
                    r.is_finite(),
                    "nbsvm ridge: non-finite log-count ratio (pool shape broke \
                     the probed arithmetic)"
                );
                ratios[i * n_dom + j] = r;
            }
        }

        // Shared Gram (XᵀX over presence rows, bias column k) + per-class
        // doc-frequency columns (XᵀY with one-hot y — an integer count
        // per feature, exact in f32 whatever the accumulation order).
        let stride = k + 1;
        let mut gram = vec![0f32; stride * stride];
        let mut dfj = vec![0u32; k * n_dom];
        let mut n_docs_per_class = vec![0f32; n_dom];
        {
            let mut present: Vec<usize> = Vec::with_capacity(512);
            for (j, rws) in rows_per_class.iter().enumerate() {
                for row in rws {
                    n_docs_per_class[j] += 1.0;
                    present.clear();
                    let mut seen = row.clone();
                    seen.sort_unstable();
                    seen.dedup();
                    for &w in &seen {
                        if let Ok(i) = order.binary_search(&w) {
                            present.push(i);
                            dfj[i * n_dom + j] += 1;
                        }
                    }
                    for &a in &present {
                        let arow = a * stride;
                        for &b in &present {
                            gram[arow + b] += 1.0;
                        }
                        gram[arow + k] += 1.0;
                        gram[k * stride + a] += 1.0;
                    }
                }
            }
            gram[k * stride + k] = doc_count as f32;
        }

        // Per-class damped solve IN THE SCALED SPACE (bias row/col
        // unscaled): `(R_j G R_j + λI) w = R_j Xᵀ y_j` — exactly
        // `ridge_solve_direct_f32`'s shape.
        let mut weights = vec![0f32; stride * n_dom];
        let mut gram_reg = vec![0f32; stride * stride];
        let mut cov = vec![0f32; stride];
        let mut l_scratch = vec![0f32; stride * stride];
        let mut z_scratch = vec![0f32; stride];
        for j in 0..n_dom {
            for i in 0..k {
                let r_i = ratios[i * n_dom + j];
                let grow = i * stride;
                for m in 0..k {
                    gram_reg[grow + m] = r_i * ratios[m * n_dom + j] * gram[grow + m];
                }
                gram_reg[grow + k] = r_i * gram[grow + k];
                gram_reg[k * stride + i] = gram[k * stride + i] * r_i;
                cov[i] = r_i * dfj[i * n_dom + j] as f32;
            }
            gram_reg[k * stride + k] = gram[k * stride + k];
            cov[k] = n_docs_per_class[j];
            for i in 0..stride {
                gram_reg[i * stride + i] += lambda;
            }
            katgpt_core::linalg::ridge_solve_direct_f32(
                &mut weights[j * stride..(j + 1) * stride],
                &mut l_scratch,
                &mut z_scratch,
                &gram_reg,
                &cov,
                stride,
                1,
            );
        }

        Ok(NbsvmRidge {
            n_dom,
            feats: order,
            ratios,
            weights,
            lambda,
        })
    }
}

// ── the v2 candidate (the NBSVM-transformed Arm A logistic) ──────────────

/// Trainer knobs for [`train_nbsvm`]. The schedule fields mirror
/// [`instinct_specialist::ArmAConfig`] (one schedule law, [`lr_at`]); the
/// two NBSVM levers are `presence_norm` and `top_k`.
#[derive(Debug, Clone, Copy)]
pub struct NbsvmConfig {
    pub epochs: usize,
    pub lr: f32,
    pub weight_decay: f32,
    pub warmup_steps: u64,
    /// Cosine floor as a fraction of `lr`.
    pub min_lr_ratio: f32,
    /// L2-normalize the presence bag (the Arm A bag convention adapted)
    /// instead of raw 1.0 presence (the ridge's own space).
    pub presence_norm: bool,
    /// Feature budget: top-k by (df desc, bucket asc) over touched
    /// buckets — the ridge's own selection law. `None` = every touched
    /// bucket.
    pub top_k: Option<usize>,
    /// Global constant multiplied into the (normalized) presence values
    /// before the ratio scaling. The ridge's squared loss is unsaturated
    /// by construction; the logistic's is not — ratio-scaled presence
    /// bags drive |s| past 30 and starve the BCE gradient. The constant
    /// FOLDS into the weights (`w′ = w·r_c·c`), so serving stays a plain
    /// linear scorer over the UNSCALED presence bag.
    pub input_scale: f32,
}

impl Default for NbsvmConfig {
    fn default() -> Self {
        Self {
            epochs: 3,
            lr: 0.05,
            weight_decay: 0.01,
            warmup_steps: 500,
            min_lr_ratio: 0.05,
            presence_norm: false,
            top_k: None,
            input_scale: 1.0,
        }
    }
}

/// One [`train_nbsvm`] run.
#[derive(Debug, Clone)]
pub struct NbsvmRun {
    /// The FOLDED model: plain linear weights over presence bags
    /// (`score_c = b_c + Σ w′[c,b]·x_b`). Serve it with the SAME
    /// `presence_norm` convention the run trained under — [`NbsvmRun`]
    /// carries the flag because the artifact format cannot (T3's hand-off
    /// spec must).
    pub model: Specialist,
    pub steps: usize,
    pub final_loss: f32,
    pub holdout_accuracy: f64,
    /// The lr=0 control: exactly the holdout share of class 0 (the 576
    /// law — the trained run must beat it).
    pub control_accuracy: f64,
    /// Train rows skipped for zero presence (empty text / pure
    /// punctuation) — loud, never silent.
    pub skipped_empty: usize,
    /// The serving input convention for the folded model.
    pub presence_norm: bool,
    /// The resolved selected-feature count (`top_k` or every touched
    /// bucket).
    pub feats_used: usize,
}

/// Train the NBSVM-transformed Arm A: supervised one-vs-all logistic BCE
/// where class `c`'s forward reads the bag scaled by ITS OWN ratio vector
/// (`s_c = b_c + Σ_b w[c,b]·x_b·r_c(b)`), over `train` row indices,
/// scored on `holdout` indices. Zero-init, fixed row order, fixed per-row
/// op order, single thread — bit-deterministic, no seed, no RNG.
///
/// The ratio tables live dense over the VOCAB with UNSELECTED buckets
/// zeroed: a zero ratio makes the gradient exactly zero, and untouched
/// zero-init weights therefore stay exactly zero through AdamW (decay
/// included: `lr·wd·0 = 0`) — so `top_k` restriction needs no index
/// indirection and the final fold (`w′ = w ⊙ r_c`) preserves it.
pub fn train_nbsvm(
    rows: &[SuiteRow],
    labels: &[String],
    train_idx: &[usize],
    holdout_idx: &[usize],
    cfg: &NbsvmConfig,
) -> Result<NbsvmRun, String> {
    let n = labels.len();
    let mut class_of: HashMap<&str, usize> = HashMap::with_capacity(n);
    for (i, l) in labels.iter().enumerate() {
        class_of.insert(l.as_str(), i);
    }
    // Counts over the TRAIN rows only (the ratio's own fitting pool — the
    // ridge fits its counts on the same rows it solves).
    let mut tok_cnt = vec![0u32; n * VOCAB];
    let mut glob_cnt = vec![0u32; VOCAB];
    let mut df = vec![0u32; VOCAB];
    let mut ndocs = 0usize;
    let mut scratch = Vec::new();
    for &i in train_idx {
        let c = *class_of
            .get(rows[i].label.as_str())
            .ok_or_else(|| format!("row label {:?} outside the universe", rows[i].label))?;
        events_into(rows[i].text.as_bytes(), &mut scratch);
        for &b in &scratch {
            tok_cnt[c * VOCAB + b as usize] += 1;
            glob_cnt[b as usize] += 1;
        }
        let mut seen = scratch.clone();
        seen.sort_unstable();
        seen.dedup();
        for &b in &seen {
            df[b as usize] += 1;
        }
        ndocs += 1;
    }
    if ndocs == 0 {
        return Err("no train rows".into());
    }

    // Feature set: the ridge's own selection law (df desc, bucket asc),
    // ascending for the fold walk.
    let mut touched: Vec<u32> =
        (0..VOCAB as u32).filter(|&b| df[b as usize] > 0).collect();
    if let Some(k) = cfg.top_k {
        touched.sort_unstable_by(|&a, &b| {
            df[b as usize].cmp(&df[a as usize]).then(a.cmp(&b))
        });
        touched.truncate(k);
    }
    touched.sort_unstable();
    let feats_used = touched.len();

    // Per-class ratio tables, class-major, unselected buckets zeroed.
    let av = ALPHA * VOCAB as f32;
    let nin = av;
    let nout = ndocs as f32 + av;
    let mut ratios = vec![0f32; n * VOCAB];
    for (c, rrow) in ratios.as_chunks_mut::<VOCAB>().0.iter_mut().enumerate() {
        for &b in &touched {
            let cin = tok_cnt[c * VOCAB + b as usize] as f32 + ALPHA;
            let cout = (glob_cnt[b as usize] - tok_cnt[c * VOCAB + b as usize]) as f32
                + ALPHA;
            let r = (cin / nin).ln() - (cout / nout).ln();
            assert!(
                r.is_finite(),
                "nbsvm v2: non-finite log-count ratio (pool shape broke the \
                 probed arithmetic)"
            );
            rrow[b as usize] = r;
        }
    }
    drop(tok_cnt);

    // Presence bags (values 1.0 or L2-normalized), fixed train order,
    // empties skipped loud.
    let mut bags: Vec<Vec<(u32, f32)>> = Vec::with_capacity(train_idx.len());
    let mut bag_gold: Vec<usize> = Vec::with_capacity(train_idx.len());
    let mut bag = Vec::new();
    let mut skipped_empty = 0usize;
    for &i in train_idx {
        presence_bag_into(rows[i].text.as_bytes(), cfg.presence_norm, &mut bag, &mut scratch);
        if bag.is_empty() {
            skipped_empty += 1;
            continue;
        }
        if cfg.input_scale != 1.0 {
            for v in bag.iter_mut() {
                v.1 *= cfg.input_scale;
            }
        }
        bags.push(std::mem::take(&mut bag));
        bag_gold.push(class_of[rows[i].label.as_str()]);
    }
    if bags.is_empty() {
        return Err("every train row tokenized to zero events".into());
    }

    let mut model = Specialist::zero(labels);
    let mut opt_w: Vec<(f32, f32)> = vec![(0.0, 0.0); n * VOCAB];
    let mut b_state: Vec<(f32, f32)> = vec![(0.0, 0.0); n];
    let total_steps = (cfg.epochs * bags.len()) as u64;
    let mut scores = vec![0.0f32; n];
    let mut step: u64 = 0;
    let mut final_loss = 0.0f32;
    let (beta1, beta2, eps) = (0.9f32, 0.999f32, 1e-8f32);
    for _epoch in 0..cfg.epochs {
        for (r, bag) in bags.iter().enumerate() {
            let gold = bag_gold[r];
            // Forward + the mean BCE over classes (the 576 summary shape).
            let mut loss = 0.0f32;
            for (c, o) in scores.iter_mut().enumerate() {
                let rrow = &ratios[c * VOCAB..(c + 1) * VOCAB];
                let mut s = model.b[c];
                for &(b, val) in bag {
                    let rv = rrow[b as usize];
                    if rv != 0.0 {
                        s += model.w[c * VOCAB + b as usize] * val * rv;
                    }
                }
                *o = katgpt_core::exact_sigmoid(s);
                let y = if gold == c { 1.0 } else { 0.0 };
                let p = *o;
                loss -= (y * p.max(f32::MIN_POSITIVE)
                    + (1.0 - y) * (1.0 - p).max(f32::MIN_POSITIVE))
                .ln();
            }
            final_loss = loss / n as f32;
            let lr = lr_at(cfg.lr, step, total_steps, cfg.warmup_steps, cfg.min_lr_ratio);
            step += 1;
            let bc1 = 1.0 - beta1.powf(step as f32);
            let bc2 = 1.0 - beta2.powf(step as f32);
            // Lazy AdamW per touched (class, bucket) — the 576 op order,
            // with the gradient carrying the class's own ratio. A zero
            // ratio makes every update an exact no-op on a zero-init
            // weight, so skipping it is bit-identical and skips ~all
            // touches under `top_k`.
            for (c, &s) in scores.iter().enumerate() {
                let y = if gold == c { 1.0 } else { 0.0 };
                let ds = (s - y) / n as f32;
                let rrow = &ratios[c * VOCAB..(c + 1) * VOCAB];
                for &(b, val) in bag {
                    let rv = rrow[b as usize];
                    if rv == 0.0 {
                        continue;
                    }
                    let g = ds * val * rv;
                    let p = c * VOCAB + b as usize;
                    let st = &mut opt_w[p];
                    st.0 = beta1 * st.0 + (1.0 - beta1) * g;
                    st.1 = beta2 * st.1 + (1.0 - beta2) * g * g;
                    let mhat = st.0 / bc1;
                    let vhat = st.1 / bc2;
                    let decay = cfg.weight_decay * model.w[p];
                    model.w[p] -= lr * (mhat / (vhat.sqrt() + eps) + decay);
                }
                // Bias: dense Adam (every class touched every row).
                let st = &mut b_state[c];
                st.0 = beta1 * st.0 + (1.0 - beta1) * ds;
                st.1 = beta2 * st.1 + (1.0 - beta2) * ds * ds;
                let mhat = st.0 / bc1;
                let vhat = st.1 / bc2;
                model.b[c] -= lr * (mhat / (vhat.sqrt() + eps));
            }
        }
    }

    // Fold the fixed scaling into the weights: `w′[c,b] = w[c,b]·r_c(b)`
    // (plus `input_scale` c — trained `Σ w·(c·x)·r` ≡ folded `Σ w·r·c·x`).
    // From here the model is a plain linear scorer over UNSCALED presence
    // bags.
    for c in 0..n {
        let rrow = &ratios[c * VOCAB..(c + 1) * VOCAB];
        for &b in &touched {
            model.w[c * VOCAB + b as usize] *= rrow[b as usize];
        }
    }

    // Holdout eval over presence bags (the SAME convention the model
    // serves under); empties stay in the denominator as misses (the 576
    // eval shape).
    let mut bag = Vec::new();
    let mut scratch2 = Vec::new();
    let mut hit = 0usize;
    for &i in holdout_idx {
        presence_bag_into(rows[i].text.as_bytes(), cfg.presence_norm, &mut bag, &mut scratch2);
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

    Ok(NbsvmRun {
        model,
        steps: step as usize,
        final_loss,
        holdout_accuracy,
        control_accuracy,
        skipped_empty,
        presence_norm: cfg.presence_norm,
        feats_used,
    })
}

// ── the 579 protocol helpers ─────────────────────────────────────────────

/// The probe's front/pool split (`issue038_t7_probe.py::stratified_front`,
/// with the text-set exclusion): `per_label` first rows of EACH label in
/// sorted-label order form the front; the pool excludes every row whose
/// TEXT appears in the front (duplicate texts excluded with it — the
/// probe's set semantics, not index semantics).
#[must_use]
pub fn probe_front_split(
    rows: &[SuiteRow],
    labels: &[String],
    per_label: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut by_label: HashMap<&str, Vec<usize>> = HashMap::new();
    for (i, r) in rows.iter().enumerate() {
        by_label.entry(r.label.as_str()).or_default().push(i);
    }
    let mut front = Vec::new();
    for lab in labels {
        if let Some(idxs) = by_label.get(lab.as_str()) {
            front.extend(idxs.iter().take(per_label).copied());
        }
    }
    let front_texts: HashSet<&str> =
        front.iter().map(|&i| rows[i].text.as_str()).collect();
    let pool: Vec<usize> = (0..rows.len())
        .filter(|&i| !front_texts.contains(rows[i].text.as_str()))
        .collect();
    (front, pool)
}

/// Load one suite's rows from a datasets dir for ANY split
/// (`<dir>/<suite>/<split>-*.json`, the datasets-server envelope) — the
/// generalized loader behind [`instinct_specialist::load_suite_rows`].
/// Issue 579 T1 reads the TEST split through this exactly once (the
/// sanctioned baseline-validation read); every v2 decision stays on the
/// train side.
pub fn load_suite_split(
    dir: &Path,
    suite: &str,
    split: &str,
) -> Result<Vec<SuiteRow>, String> {
    let raw = read_split_envelope(dir, suite, split)?;
    let mut rows = Vec::new();
    for row in &raw {
        if let Some(sr) = suite_row(suite, row) {
            rows.push(sr);
        }
    }
    if rows.is_empty() {
        return Err(format!(
            "suite {suite} split {split}: no rows survived the label rule"
        ));
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(label: &str, text: &str) -> SuiteRow {
        SuiteRow {
            label: label.to_string(),
            text: text.to_string(),
            presented: None,
        }
    }

    /// The quirk pin: the ratio's `nin`/`nout` are the CONSTANTS αV and
    /// ndocs + αV — the probe's string-label missing-key read — NOT the
    /// per-class totals. The corpus carries ~2100 class-a events so the
    /// two forms differ by ~1.6e-2 (≫ f32 noise at ratio magnitude ~10);
    /// "fixing" the quirk reds the first assertion.
    #[test]
    fn ratio_quirk_ignores_per_class_totals() {
        let filler = "one two three four five six seven eight nine ten";
        let a_text = format!("alpha {filler}");
        let mut rows: Vec<SuiteRow> = (0..100).map(|_| row("a", &a_text)).collect();
        rows.push(row("b", "bravo"));
        let labels = vec!["a".to_string(), "b".to_string()];
        let m = NbsvmRidge::fit(&rows, &labels, 10.0, 4096).expect("fit");
        let mut ids = Vec::new();
        events_into(b"alpha", &mut ids);
        let alpha_b = ids[0];
        // Replay the SAME event stream the fit read: per-class token
        // total, the alpha count, the global alpha count.
        let mut ev = Vec::new();
        let mut cin = 0f64;
        let mut tot_a = 0f64;
        for r in rows.iter().filter(|r| r.label == "a") {
            events_into(r.text.as_bytes(), &mut ev);
            tot_a += ev.len() as f64;
            cin += ev.iter().filter(|&&b| b == alpha_b).count() as f64;
        }
        let mut glob = cin;
        events_into("bravo".as_bytes(), &mut ev);
        glob += ev.iter().filter(|&&b| b == alpha_b).count() as f64;
        let ndocs = rows.len() as f64;
        let got = m.ratio_at(0, alpha_b).expect("alpha selected") as f64;
        // QUIRK: nin = αV, nout = ndocs + αV (class-a totals never enter).
        let quirk = ((cin + 1.0) / VOCAB as f64).ln()
            - ((glob - cin + 1.0) / (ndocs + VOCAB as f64)).ln();
        assert!(
            (got - quirk).abs() < 1e-4,
            "ratio {got} vs quirk expectation {quirk}"
        );
        // The per-class-total form (nin = tot_a + αV) differs materially.
        let fixed = ((cin + 1.0) / (tot_a + VOCAB as f64)).ln()
            - ((glob - cin + 1.0) / (ndocs + VOCAB as f64)).ln();
        assert!(
            (got - fixed).abs() > 1e-2,
            "quirk and fixed forms must disagree on this pool ({got} vs {fixed})"
        );
    }

    /// Feature selection: (df desc, bucket asc) — the excluded-low-df arm
    /// and the equal-df tie-break both pinned.
    #[test]
    fn feature_selection_df_desc_bucket_asc() {
        let mut rows = Vec::new();
        for _ in 0..5 {
            rows.push(row("a", "zulu"));
        }
        for _ in 0..5 {
            rows.push(row("a", "yankee"));
        }
        rows.push(row("b", "xray"));
        let labels = vec!["a".to_string(), "b".to_string()];
        let mut ids = Vec::new();
        events_into(b"zulu", &mut ids);
        let z = ids[0];
        events_into(b"yankee", &mut ids);
        let y = ids[0];
        events_into(b"xray", &mut ids);
        let x = ids[0];
        // top-2: the df-5 pair stays, the df-1 bucket is excluded.
        let m = NbsvmRidge::fit(&rows, &labels, 10.0, 2).expect("fit");
        assert_eq!(m.feats_dbg().len(), 2);
        assert!(!m.feats_dbg().contains(&x));
        assert!(m.feats_dbg().contains(&z) && m.feats_dbg().contains(&y));
        // Equal df, one slot: the LOWER bucket id wins the tie.
        let m1 = NbsvmRidge::fit(&rows, &labels, 10.0, 1).expect("fit");
        assert_eq!(m1.feats_dbg(), &[z.min(y)]);
    }

    #[test]
    fn ridge_solves_a_toy_and_picks_gold() {
        let mut rows = Vec::new();
        for _ in 0..8 {
            rows.push(row("a", "alpha charlie"));
        }
        for _ in 0..8 {
            rows.push(row("b", "bravo delta"));
        }
        let labels = vec!["a".to_string(), "b".to_string()];
        let m = NbsvmRidge::fit(&rows, &labels, 1.0, 4096).expect("fit");
        let mut ids = Vec::new();
        let mut out = Vec::new();
        presence_ids_into(b"alpha charlie", &mut out, &mut ids);
        assert_eq!(m.pick(&out), 0);
        presence_ids_into(b"bravo delta", &mut out, &mut ids);
        assert_eq!(m.pick(&out), 1);
    }

    #[test]
    fn fit_is_deterministic() {
        let rows = vec![
            row("a", "alpha charlie"),
            row("a", "alpha echo"),
            row("b", "bravo delta"),
            row("b", "bravo foxtrot"),
        ];
        let labels = vec!["a".to_string(), "b".to_string()];
        let a = NbsvmRidge::fit(&rows, &labels, 10.0, 4096).expect("fit");
        let b = NbsvmRidge::fit(&rows, &labels, 10.0, 4096).expect("fit");
        assert_eq!(a.feats_dbg(), b.feats_dbg());
        assert_eq!(a.ratios.len(), b.ratios.len());
        for (x, y) in a.ratios.iter().zip(b.ratios.iter()) {
            assert_eq!(x.to_bits(), y.to_bits());
        }
        assert_eq!(a.weights.len(), b.weights.len());
        for (x, y) in a.weights.iter().zip(b.weights.iter()) {
            assert_eq!(x.to_bits(), y.to_bits());
        }
    }

    /// The paired law, hand-checked: diffs [0, 1, −1, 0] → mean 0,
    /// se = √(2/3)/2, lb95 = −Z95·se.
    #[test]
    fn paired_lb95_known_values() {
        let a = [true, true, false, false];
        let b = [true, false, true, false];
        let d = paired_lb95(&a, &b).expect("paired");
        assert_eq!(d.mean, 0.0);
        let se = (2.0f64 / 3.0).sqrt() / 2.0;
        assert!((d.se - se).abs() < 1e-12);
        assert!((d.lb95 - (-Z95 * se)).abs() < 1e-12);
        assert!((d.ub95 - (Z95 * se)).abs() < 1e-12);
        assert!(paired_lb95(&a[..1], &b).is_none());
        assert!(paired_lb95(&[], &[]).is_none());
    }

    #[test]
    fn presence_bag_is_sorted_unique_and_normed() {
        let mut out = Vec::new();
        let mut s = Vec::new();
        presence_bag_into(b"alpha bravo alpha", false, &mut out, &mut s);
        assert!(out.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(!out.is_empty());
        assert!(out.iter().all(|(_, v)| *v == 1.0));
        presence_bag_into(b"alpha bravo alpha", true, &mut out, &mut s);
        let inv = 1.0 / (out.len() as f32).sqrt();
        assert!(out.iter().all(|(_, v)| (*v - inv).abs() < 1e-7));
        presence_bag_into(b"!!! ...", false, &mut out, &mut s);
        assert!(out.is_empty());
    }

    #[test]
    fn v2_learns_a_toy_and_beats_the_lr0_control() {
        let mut rows = Vec::new();
        for _ in 0..8 {
            rows.push(row("a", "alpha charlie"));
        }
        for _ in 0..8 {
            rows.push(row("b", "bravo delta"));
        }
        rows.push(row("a", "alpha charlie echo"));
        rows.push(row("b", "bravo delta foxtrot"));
        let labels = vec!["a".to_string(), "b".to_string()];
        let train_idx: Vec<usize> = (0..16).collect();
        let holdout_idx = vec![16, 17];
        let run =
            train_nbsvm(&rows, &labels, &train_idx, &holdout_idx, &NbsvmConfig::default())
                .expect("train");
        assert!(run.holdout_accuracy > run.control_accuracy);
        assert_eq!(run.holdout_accuracy, 1.0);
        assert_eq!(run.control_accuracy, 0.5);
        assert_eq!(run.skipped_empty, 0);
    }

    /// `top_k` restriction: the fold preserves the zeroing — every
    /// unselected bucket's weight stays EXACTLY zero (zero ratio → zero
    /// gradient → untouched zero-init weight), and the selected bucket
    /// carries the learned signal.
    #[test]
    fn v2_top_k_restriction_zeroes_unselected_weights() {
        let mut rows = Vec::new();
        for _ in 0..8 {
            rows.push(row("a", "alpha charlie"));
        }
        for _ in 0..8 {
            rows.push(row("b", "bravo delta"));
        }
        let labels = vec!["a".to_string(), "b".to_string()];
        let train_idx: Vec<usize> = (0..16).collect();
        // The trainer's own selection rule, replayed for the expectation:
        // df over train rows, (df desc, bucket ASC — the LOWEST id wins a
        // df tie; max_by returns the LAST max, so min over the max-df
        // set), top-1.
        let mut df: HashMap<u32, u32> = HashMap::new();
        let mut scratch = Vec::new();
        for &i in &train_idx {
            events_into(rows[i].text.as_bytes(), &mut scratch);
            let mut seen = scratch.clone();
            seen.sort_unstable();
            seen.dedup();
            for b in seen {
                *df.entry(b).or_default() += 1;
            }
        }
        let maxdf = df.values().copied().max().expect("df");
        let sel = *df
            .iter()
            .filter(|&(_, &d)| d == maxdf)
            .map(|(b, _)| b)
            .min()
            .expect("df");
        let cfg = NbsvmConfig {
            top_k: Some(1),
            ..NbsvmConfig::default()
        };
        let run = train_nbsvm(&rows, &labels, &train_idx, &[], &cfg).expect("train");
        assert_eq!(run.feats_used, 1);
        let w = &run.model.w;
        for b in 0..VOCAB {
            let live = w[b] != 0.0 || w[VOCAB + b] != 0.0;
            assert_eq!(live, b == sel as usize, "bucket {b} live={live}");
        }
    }

    /// The lexicon law pins (drift tripwire — the stream must stay the
    /// reflex `embed.rs` constants, or every ratio below forks).
    #[test]
    fn lexicon_constants_are_the_reflex_law() {
        use crate::instinct_specialist::{BIGRAM_SALT, WORD_SALT};
        assert_eq!(WORD_SALT, 0x3456_7890_1234_5678);
        assert_eq!(BIGRAM_SALT, 0x0f1e_2d3c_4b5a_6978);
        assert_eq!(VOCAB, 1 << 17);
        assert_eq!(ALPHA, 1.0);
    }
}

//! The GOAT-gate statistics (riir-instinct Issue 005): Wilson bounds,
//! paired non-inferiority, the δ power rule, the Pareto rank-0 filter and
//! the Beta-LCB selector — the pre-registration instrument's whole
//! arithmetic, pure and known-answer-tested. No RNG anywhere: the
//! instrument ranks deterministic train-side outcomes.

/// The two-sided 95% normal quantile.
pub const Z95: f64 = 1.959_963_984_540_054;

/// Wilson 95% lower bound of a binomial proportion (the G5 face:
/// `accuracy Wilson LB > max(A0, A1)`). Zero n → 0.0 (no evidence, no
/// bound — the caller refuses, never reads it as a pass).
pub fn wilson_lb(k: usize, n: usize) -> f64 {
    wilson_bound(k, n, Z95, false)
}

/// Wilson 95% bound; `upper` picks the side. Pure Wilson score interval
/// with the continuity-free (standard) form.
pub fn wilson_bound(k: usize, n: usize, z: f64, upper: bool) -> f64 {
    if n == 0 {
        return 0.0;
    }
    let nf = n as f64;
    let p = k as f64 / nf;
    let z2 = z * z;
    let denom = 1.0 + z2 / nf;
    let center = p + z2 / (2.0 * nf);
    let radius = z * (p * (1.0 - p) / nf + z2 / (4.0 * nf * nf)).sqrt();
    let b = (center + if upper { radius } else { -radius }) / denom;
    b.clamp(0.0, 1.0)
}

/// The paired difference A − B over per-question correctness (both arms
/// answer the SAME questions — the paired outcomes are direct). The G3
/// statistic: mean, SE, and the one-sided 95% UPPER bound (regression =
/// A0 − hybrid; the bound caps the regression).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PairedDiff {
    pub mean: f64,
    pub se: f64,
    pub ub95: f64,
}

pub fn paired_upper_bound(a: &[bool], b: &[bool]) -> Option<PairedDiff> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    let n = a.len() as f64;
    let mut sum = 0.0f64;
    let mut sum2 = 0.0f64;
    for (x, y) in a.iter().zip(b) {
        let d = i8::from(*x) - i8::from(*y);
        let d = f64::from(d);
        sum += d;
        sum2 += d * d;
    }
    let mean = sum / n;
    // Sample variance of the diffs (n−1); the diffs are ±1/0, so this is
    // exact, never a proportion shortcut.
    let var = ((sum2 - sum * sum / n) / (n - 1.0)).max(0.0);
    let se = var.sqrt() / n.sqrt();
    Some(PairedDiff {
        mean,
        se,
        ub95: mean + Z95 * se,
    })
}

/// The G3/δ power rule: `δ = max(1.0 pp, 2.5·SE)` where SE is the paired
/// SE computed on the held-out train/cal discordant rate AT THE TEST n
/// (pre-declared from train-side data, evaluated on test). 2.5·SE ≈ 80%
/// power at parity (≈20% parity-fail, stated in the gate, not hidden).
pub fn delta_suite(p_discordant_train: f64, n_test: usize) -> f64 {
    let p = p_discordant_train.clamp(0.0, 1.0);
    let se = if n_test == 0 {
        f64::INFINITY
    } else {
        (p * (1.0 - p) / n_test as f64).sqrt()
    };
    (0.01f64).max(2.5 * se)
}

/// One arm's train/cal reading, as the instrument's three axes (all
/// MINIMIZED — accuracy enters negated; the caller computes the LCB).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArmStat {
    /// −Beta_LCB(accuracy): lower is better (higher accuracy).
    pub neg_acc_lcb: f64,
    /// Escalation / consult rate: lower is better (the latency face).
    pub cost_rate: f64,
    /// p99 latency (µs): lower is better.
    pub p99_us: f64,
}

/// `a` dominates `b` when it is ≤ on every axis and < on at least one
/// (the `dominates` shape — riir-clippy `ruliology_search.rs`, Bench 572
/// lineage; the same seam Proposal 042 prescribes).
pub fn dominates(a: &ArmStat, b: &ArmStat) -> bool {
    let le = a.neg_acc_lcb <= b.neg_acc_lcb
        && a.cost_rate <= b.cost_rate
        && a.p99_us <= b.p99_us;
    let lt = a.neg_acc_lcb < b.neg_acc_lcb
        || a.cost_rate < b.cost_rate
        || a.p99_us < b.p99_us;
    le && lt
}

/// The Pareto rank-0 indices of `arms` (non-dominated candidates). A
/// candidate tied with a clone of itself stays rank-0 (mutual
/// non-domination) — the filter removes dominated candidates, never ties.
pub fn pareto_rank0(arms: &[ArmStat]) -> Vec<usize> {
    let mut keep = Vec::with_capacity(arms.len());
    for (i, a) in arms.iter().enumerate() {
        let dominated = arms
            .iter()
            .enumerate()
            .any(|(j, b)| j != i && dominates(b, a));
        if !dominated {
            keep.push(i);
        }
    }
    keep
}

/// The gated arm per suite: argmax Beta-LCB (ε = 0.05 quantile — the
/// conservative idiom) among the Pareto rank-0 candidates
/// ([`katgpt_core::best_belief_score`], the best_belief ε-quantile Beta
/// lower bound). Returns the winning candidate index.
pub fn select_arm(rank0: &[usize], successes: &[u32], failures: &[u32]) -> usize {
    assert_eq!(rank0.len(), successes.len());
    assert_eq!(rank0.len(), failures.len());
    assert!(!rank0.is_empty(), "empty rank-0 set — the instrument refused");
    let mut best_pos = 0usize;
    let mut best_lcb = f32::NEG_INFINITY;
    for (pos, (&s, &f)) in successes.iter().zip(failures).enumerate() {
        let lcb = katgpt_core::best_belief_score(s, f, 0.05);
        if lcb > best_lcb {
            best_lcb = lcb;
            best_pos = pos;
        }
    }
    rank0[best_pos]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    /// Known-answer Wilson bounds — the standard formula hand-derived
    /// (95/100 at z=1.96 reads [0.888, 0.978] in the usual tables;
    /// 100/100 reads [0.963, 1]).
    #[test]
    fn wilson_known_answers() {
        assert!(close(wilson_lb(95, 100), 0.8882, 5e-4), "{}", wilson_lb(95, 100));
        assert!(
            close(wilson_bound(95, 100, Z95, true), 0.9785, 5e-4),
            "{}",
            wilson_bound(95, 100, Z95, true)
        );
        // Degenerate counts stay inside [0,1] and keep their side; the
        // zero-count lower bound is a rounding residue above 0.
        assert!(wilson_lb(0, 100) < 1e-12, "{}", wilson_lb(0, 100));
        assert!(close(wilson_lb(100, 100), 0.9630, 5e-4), "{}", wilson_lb(100, 100));
        assert_eq!(wilson_lb(5, 0), 0.0, "no evidence, no bound");
    }

    /// The paired bound on constructed data: A0 wrong on 6 (the hybrid
    /// right there), the hybrid wrong on 2 → mean = (2−6)/100 = −0.04 and
    /// the UB is mean + z·SE over the ±1/0 diffs (exact variance).
    #[test]
    fn paired_bound_known_answer() {
        let a: Vec<bool> = (0..100).map(|i| !(i < 6)).collect();
        // The hybrid errs on a DISJOINT pair of questions (6..8) — all 6
        // A0-wrong questions have the hybrid right, 2 the reverse.
        let b: Vec<bool> = (0..100).map(|i| !(6..8).contains(&i)).collect();
        let pd = paired_upper_bound(&a, &b).expect("pairs");
        assert!(close(pd.mean, -0.04, 1e-12));
        let sum = 2.0f64 - 6.0;
        let sum2 = 8.0f64;
        let n = 100.0f64;
        let var = (sum2 - sum * sum / n) / (n - 1.0);
        let se = var.sqrt() / n.sqrt();
        assert!(close(pd.ub95, pd.mean + Z95 * se, 1e-12));
        assert!(pd.ub95 > 0.0, "a regressing hybrid's bound must admit the regression");
        assert!(paired_upper_bound(&a[..3], &b).is_none(), "length mismatch refuses");
    }

    /// δ = max(1.0 pp, 2.5·SE) — the floor and the scaling both pinned.
    #[test]
    fn delta_rule() {
        // Tiny SE → the 1.0 pp floor.
        assert!(close(delta_suite(0.01, 100_000), 0.01, 1e-12));
        // Bigger discordance at small n → 2.5·SE wins.
        let d = delta_suite(0.2, 400);
        let se = (0.2f64 * 0.8 / 400.0).sqrt();
        assert!(close(d, 2.5 * se, 1e-12));
        assert!(d > 0.01);
    }

    /// The rank-0 filter: a dominated arm leaves; mutual ties stay.
    #[test]
    fn pareto_filter() {
        let arms = [
            ArmStat { neg_acc_lcb: 0.10, cost_rate: 0.0, p99_us: 10.0 }, // best acc
            ArmStat { neg_acc_lcb: 0.12, cost_rate: 0.3, p99_us: 12.0 }, // dominated by 0
            ArmStat { neg_acc_lcb: 0.15, cost_rate: 0.0, p99_us: 5.0 },  // fastest — rank-0
        ];
        let mut r0 = pareto_rank0(&arms);
        r0.sort_unstable();
        assert_eq!(r0, vec![0, 2], "the all-worse arm leaves; the frontier stays");
        let tied = [
            ArmStat { neg_acc_lcb: 0.1, cost_rate: 0.2, p99_us: 9.0 },
            ArmStat { neg_acc_lcb: 0.1, cost_rate: 0.2, p99_us: 9.0 },
        ];
        assert_eq!(pareto_rank0(&tied).len(), 2, "mutual non-domination keeps ties");
    }

    /// The selector: argmax Beta-LCB among rank-0 — ordering-monotone
    /// cases only (equal rates: more evidence wins; a strictly better
    /// rate at equal n wins; a zero-evidence arm never wins).
    #[test]
    fn selector_prefers_the_conservative_lcb() {
        // Equal rate 0.5: Beta(4,4)'s 5% quantile sits above Beta(2,2)'s
        // — MORE evidence wins the conservative bound.
        assert_eq!(select_arm(&[0, 1], &[3, 1], &[3, 1]), 0);
        // Higher rate at equal n wins outright.
        assert_eq!(select_arm(&[0, 1], &[9, 6], &[1, 4]), 0);
        // Zero-evidence never beats any evidence at a plausible rate.
        assert_eq!(select_arm(&[0, 1], &[0, 1], &[2, 1]), 1);
    }
}

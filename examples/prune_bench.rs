//! Timing probe for the threshold prune (Issue 003 G2's O(n·k) remedy):
//! old insertion-scan prune vs the threshold prune at the arena's real
//! shapes (n = option count, top_k 8). Report-only; the ARENA's
//! fusion-overhead micro is the canonical instrument — this just proves
//! the direction before a G2 re-measure.
//!
//! ```text
//! cargo run --release --example prune_bench
//! ```

use std::hint::black_box;
use riir_instinct::{Cascade, MAX_TOP_K};

/// The pre-remedy shape: full insertion walk for every option.
fn prune_old(
    top_k: usize,
    probs: &[f64],
    survivors: &mut [(usize, f64); MAX_TOP_K],
) -> usize {
    let mut n = 0usize;
    for (i, &p) in probs.iter().enumerate() {
        let mut pos = n.min(top_k);
        while pos > 0 && survivors[pos - 1].1 < p {
            pos -= 1;
        }
        if pos >= top_k {
            continue;
        }
        let last = n.min(top_k - 1);
        let mut s = last;
        while s > pos {
            survivors[s] = survivors[s - 1];
            s -= 1;
        }
        survivors[pos] = (i, p);
        if n < top_k {
            n += 1;
        }
    }
    n
}

fn main() {
    let iters = 200_000;
    for (n, k) in [(4usize, 4usize), (6, 6), (20, 8), (59, 8), (77, 8), (400, 8)] {
        let probs: Vec<f64> = (0..n)
            .map(|i| ((i as f64 * 0.618_033_988_7).sin() * 1e4).fract().abs())
            .collect();
        let cas = Cascade { top_k: k };
        let mut surv: [(usize, f64); MAX_TOP_K] = [(0, 0.0); MAX_TOP_K];
        // Warm + equivalence witness.
        let a = cas.prune(&probs, &mut surv).to_vec();
        let b = prune_old(k, &probs, &mut surv);
        assert_eq!(a.len(), b, "equivalence n={n} k={k}");
        assert_eq!(&a, &surv[..b], "byte-equal n={n} k={k}");

        let t = std::time::Instant::now();
        let mut sum = 0usize;
        for _ in 0..iters {
            let kept = cas.prune(&probs, &mut surv);
            sum = sum.wrapping_add(kept[0].0);
        }
        let new_ns = t.elapsed().as_nanos() as f64 / iters as f64;
        black_box(sum);

        let t = std::time::Instant::now();
        let mut sum = 0usize;
        for _ in 0..iters {
            let kept = prune_old(k, &probs, &mut surv);
            sum = sum.wrapping_add(black_box(kept));
        }
        let old_ns = t.elapsed().as_nanos() as f64 / iters as f64;
        black_box(sum);
        println!(
            "n {n:>3} · k {k}: old {old_ns:>6.0} ns · new {new_ns:>6.0} ns · {ratio:.2}x",
            ratio = old_ns / new_ns
        );
    }
}

// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 2097-2140

        .enumerate()
        .filter(|(_, &e)| e)
        .map(|(i, _)| i)
        .collect();
    if escalated.is_empty() {
        return None;
    }
    let cases: Vec<SuiteCase> = escalated.iter().map(|&i| seat.suite.cases[i].clone()).collect();
    let durs = riir_reflex::harness::runner::seat::laya_escalation_latency_us(&cases).ok()?;
    let lane_durs: Vec<f64> = escalated.iter().map(|&i| h1.own_durs_us[i]).collect();
    let laya_durs: Vec<f64> = durs.iter().map(|&d| d as f64).collect();
    // The non-inferiority form: the violation direction is lane − laya
    // (positive = the lane SLOWER = bad); the gate passes when its 95%
    // upper bound ≤ 0 — i.e. the lane is faster with 95% confidence.
    // (The first recording inverted this: ub95(laya − lane) ≤ 0 fails
    // exactly when the lane is much faster — the data was favorable, the
    // sign was not.)
    let diffs: Vec<f64> = lane_durs
        .iter()
        .zip(&laya_durs)
        .map(|(m, l)| m - l)
        .collect();
    let pd = paired_upper_bound_f64(&diffs)?;
    Some(LayaFace {
        escalated_n: escalated.len(),
        lane_p50_us: pct(&lane_durs, 0.5),
        laya_p50_us: pct(&laya_durs, 0.5),
        paired_ub95_us: pd.ub95,
        pass: pd.ub95 <= 0.0,
    })
}

#[cfg(not(feature = "arena-laya"))]
fn laya_face(_seat: &Seat, _test_arms: &[ArmOut]) -> Option<LayaFace> {
    None
}

/// The A0 drift pin: reflex's own `run()` over xnli_en at the same
/// posture must reproduce the arena's A0 test accuracy exactly (the same
/// engine, the same questions — a divergence is a seat-path defect).
fn pin_a0_identity(datasets_dir: &Path, runs: &[SuiteRun], suites: &[&str]) -> Result<(), String> {
    let opts = riir_reflex::harness::runner::RunOptions {
        datasets_dir: datasets_dir.to_path_buf(),
        suites: suites.iter().map(|s| (*s).to_string()).collect(),

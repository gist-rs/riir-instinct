//! The G1 calibrator contract pin (Bench 001's first-run lesson): the
//! sigmoid-gate calibrator's `apply()` is the IDENTITY until `refit()` is
//! called — `observe()` only fills the evidence ring. A calibration face
//! that observes but never refits reads Platt == raw to full precision
//! and fails the gate for the wrong reason. This pin fails if that
//! contract ever changes silently in either direction.

use katgpt_core::sigmoid_calibration::SigmoidGateCalibrator;

fn overconfident_pairs() -> SigmoidGateCalibrator {
    // Everything scored 0.95, only 80% correct — a genuinely overconfident
    // readout the Platt fit must deflate.
    let mut cal = SigmoidGateCalibrator::new(512, 64);
    for i in 0..200 {
        cal.observe(0.95, i % 5 != 0);
    }
    cal
}

#[test]
fn apply_is_identity_until_refit() {
    let cal = overconfident_pairs();
    assert_eq!(
        cal.apply(0.95),
        0.95,
        "apply must be the identity before refit (the documented contract)"
    );
}

#[test]
fn refit_deflates_an_overconfident_readout() {
    let mut cal = overconfident_pairs();
    assert!(cal.refit(), "200 pairs at min_obs 64 must fit");
    let out = cal.apply(0.95);
    assert!(
        out < 0.9,
        "the fit should deflate an overconfident 0.95 toward the base rate 0.8, got {out}"
    );
}

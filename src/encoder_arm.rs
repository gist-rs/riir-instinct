//! The encoder-feature arm reader (instinct issue 014 C1): replay the
//! riir-train NLEH v1 frozen head over the laya-english encoder's own
//! output states, live-encoded per seat case — the RECORD-ONLY arm of the
//! arena (the encoder class is REFUSED at serve per issue 014 decision 1;
//! this module exists to put the measured number on the board with
//! `serve: ✗`, never to seat).
//!
//! ## Why a law copy, not a dependency
//!
//! The codec + forward are byte-lawful re-implementations of riir-train's
//! `instinct_encoder_lane` (the lane's ONE home upstream). The instinct
//! law is the mirror: "riir-train trains, this repo consumes bytes" — a
//! cargo dep on the trainer would invert that, so the consumer side
//! re-implements the ~120-line read path. The pin is FUNCTIONAL: the
//! sealed artifact `t6_s0.bin` (BLAKE3 `…`, the riir-train 600 T6 read)
//! must reproduce its frozen confusion matrix through THIS reader on the
//! same encode bytes (cell-identical, the M3 replay's posture) — the
//! C1 bench record is that witness.
//!
//! ## The feature law (the exact `lenc_features` order)
//!
//! `x = marker_rows (n_markers·d) ++ mean-over-seq(hidden) (d)` — the
//! mean accumulated per token IN ORDER (bit-order-identical to the
//! trainer), then the artifact's fixed affine `x' = (x − mu)·inv_std`
//! clamped ±30, then the MLP (ReLU hidden, per-class sigmoid) with the
//! trainer's accumulation order (`w1` row-major over `x` in order).

// ── the NLEH v1 codec (read half only — the trainer writes) ───────────

/// A parsed NLEH v1 artifact (the exact field layout riir-train's
/// `write_nleh_v1` exported at issue 599 T3).
pub struct NlehHead {
    pub feat_dim: usize,
    pub hidden: usize,
    pub n_classes: usize,
    pub mu: Vec<f32>,
    pub inv_std: Vec<f32>,
    /// `[feat_dim × hidden]` row-major.
    pub w1: Vec<f32>,
    pub b1: Vec<f32>,
    /// `[hidden × n_classes]` row-major.
    pub w2: Vec<f32>,
    pub b2: Vec<f32>,
}

/// Parse + size-validate an NLEH v1 artifact (`n_classes` decoded from the
/// tail, exactly the trainer's law: the remainder must be a whole multiple
/// of `hidden + 1`).
pub fn read_nleh_v1(bytes: &[u8]) -> Result<NlehHead, String> {
    let err = |m: &str| format!("NLEH: {m}");
    if bytes.len() < 16 || &bytes[0..4] != b"NLEH" {
        return Err(err("not an NLEH artifact"));
    }
    let ver = u32::from_le_bytes(bytes[4..8].try_into().map_err(|_| err("short"))?);
    if ver != 1 {
        return Err(err(format!("version {ver} != 1").as_str()));
    }
    let feat_dim = u32::from_le_bytes(bytes[8..12].try_into().map_err(|_| err("short"))?) as usize;
    let hidden = u32::from_le_bytes(bytes[12..16].try_into().map_err(|_| err("short"))?) as usize;
    if feat_dim == 0 || hidden == 0 {
        return Err(err("zero dim"));
    }
    let mut pos = 16usize;
    let take = |pos: &mut usize, n: usize| -> Result<Vec<f32>, String> {
        let end = pos
            .checked_add(n.checked_mul(4).ok_or_else(|| err("overflow"))?)
            .ok_or_else(|| err("overflow"))?;
        if end > bytes.len() {
            return Err(err("truncated"));
        }
        let out = bytes[*pos..end]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|c| f32::from_le_bytes(*c))
            .collect();
        *pos = end;
        Ok(out)
    };
    let mu = take(&mut pos, feat_dim)?;
    let inv_std = take(&mut pos, feat_dim)?;
    let w1 = take(&mut pos, feat_dim.checked_mul(hidden).ok_or_else(|| err("overflow"))?)?;
    let b1 = take(&mut pos, hidden)?;
    // n_classes is only decodable from the tail: what remains is w2 + b2,
    // i.e. hidden·k + k floats = k·(hidden + 1) — the trainer's law.
    let rem = (bytes.len() - pos) / 4;
    if rem == 0 || !rem.is_multiple_of(hidden + 1) {
        return Err(err("cannot decode n_classes from the tail"));
    }
    let n_classes = rem / (hidden + 1);
    let w2 = take(&mut pos, hidden.checked_mul(n_classes).ok_or_else(|| err("overflow"))?)?;
    let b2 = take(&mut pos, n_classes)?;
    if pos != bytes.len() {
        return Err(err("trailing bytes"));
    }
    Ok(NlehHead {
        feat_dim,
        hidden,
        n_classes,
        mu,
        inv_std,
        w1,
        b1,
        w2,
        b2,
    })
}

/// Load + BLAKE3-verify one sealed artifact (sidecar `<path>.blake3`
/// unless `blake3_path` names it explicitly — the trainer eval's law).
pub fn load_sealed(path: &std::path::Path, blake3_path: Option<&std::path::Path>) -> Result<NlehHead, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let side = blake3_path
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| {
            let mut p: std::ffi::OsString = path.as_os_str().to_os_string();
            p.push(".blake3");
            std::path::PathBuf::from(p)
        });
    let want = std::fs::read_to_string(&side)
        .map_err(|e| format!("read {}: {e}", side.display()))?
        .trim()
        .to_string();
    let got = blake3::hash(&bytes).to_hex().to_string();
    if got != want {
        return Err(format!(
            "BLAKE3 mismatch: {} is not the sealed artifact (want {want}, got {got})",
            path.display()
        ));
    }
    read_nleh_v1(&bytes)
}

/// The artifact's fixed standardization affine, in place:
/// `x' = (x − mu)·inv_std` then the ±30 inf-guard (the trainer's feat_std
/// law — the exact transform the holdout read scored under).
pub fn nleh_standardize(a: &NlehHead, x: &mut [f32]) {
    debug_assert_eq!(x.len(), a.feat_dim);
    for (j, v) in x.iter_mut().enumerate() {
        *v = ((*v - a.mu[j]) * a.inv_std[j]).clamp(-30.0, 30.0);
    }
}

/// The eval forward: ReLU hidden layer, per-class sigmoid outputs. Same
/// accumulation order as the trainer's forward (h computed first, then
/// y[c] accumulated PER CLASS over j in order — float addition is not
/// associative, and the frozen read's cell identity is the pin) so the
/// read is bit-faithful to what the holdout scored.
pub fn nleh_forward(a: &NlehHead, xs: &[f32], out: &mut Vec<f32>) {
    debug_assert_eq!(xs.len(), a.feat_dim);
    let k = a.n_classes;
    out.clear();
    out.resize(k, 0.0);
    let mut h = vec![0f32; a.hidden];
    for (hj, hs) in h.iter_mut().enumerate() {
        let mut s = a.b1[hj];
        let base = hj * a.feat_dim;
        for (i, &xi) in xs.iter().enumerate() {
            s += a.w1[base + i] * xi;
        }
        *hs = s.max(0.0);
    }
    for (c, yc) in out.iter_mut().enumerate() {
        let mut s = a.b2[c];
        for (j, &hv) in h.iter().enumerate() {
            s += a.w2[j * k + c] * hv;
        }
        *yc = 1.0 / (1.0 + (-s).exp());
    }
}

// ── the feature law ───────────────────────────────────────────────────

/// `x = marker_rows ++ mean-over-seq(hidden_state)` — the exact
/// `lenc_features` accumulation (per-token in order; `out` cleared and
/// refilled). The encode's `marker_rows`/`hidden` arrive as the live
/// encoder's output (no LENC cache in this repo — the arena encodes).
pub fn encode_features(
    marker_rows: &[f32],
    hidden: &[f32],
    d: usize,
    n_markers: usize,
    out: &mut Vec<f32>,
) -> Result<(), String> {
    if marker_rows.len() != n_markers * d {
        return Err(format!(
            "marker_rows len {} != n_markers {n_markers} × d {d}",
            marker_rows.len()
        ));
    }
    if d == 0 || !hidden.len().is_multiple_of(d) {
        return Err(format!("hidden len {} not a multiple of d {d}", hidden.len()));
    }
    let seq = hidden.len() / d;
    out.clear();
    out.reserve(n_markers * d + d);
    out.extend_from_slice(marker_rows);
    let mut mean = vec![0f32; d];
    for t in 0..seq {
        let base = t * d;
        for (j, m) in mean.iter_mut().enumerate() {
            *m += hidden[base + j] / seq as f32;
        }
    }
    out.extend_from_slice(&mean);
    debug_assert_eq!(out.len(), n_markers * d + d);
    Ok(())
}

/// The trainer's pick law: argmax by `total_cmp`, ties to the LATER index
/// (`Iterator::max_by` semantics — the exact law the frozen read scored
/// under; do not "fix" this to lowest-tie without re-freezing).
pub fn nleh_pick(y: &[f32]) -> usize {
    (0..y.len())
        .max_by(|p, r| y[*p].total_cmp(&y[*r]))
        .unwrap_or(0)
}

// ── the seat-side runner (the arena's encoder arm — cfg arena-laya) ────

/// The record-only encoder arm over a seat's test cases (issue 014 C1):
/// live-encode every case through the laya-english checkpoint (the SAME
/// agent + wire form the harness laya lane and the riir-train dump use),
/// replay the sealed NLEH head, and return per-question pick/correct/
/// confidence/µs rows. ONE question per case — the C1 scope (sst5's shape);
/// a multi-question suite is refused loud, not silently mis-scored.
///
/// Latency is the FULL arm cost per question (encode + feature + head
/// forward), the number the serve refusal is priced against. The first
/// case is an UNMEASURED warmup (the Metal pipeline-compile law — the
/// harness lane's pre-ramp).
#[cfg(feature = "arena-laya")]
pub struct EncoderArmOut {
    pub device: &'static str,
    pub picks: Vec<usize>,
    pub correct: Vec<bool>,
    pub confs: Vec<f64>,
    pub durs_us: Vec<f64>,
    /// The head's class count (== the question's criteria count, asserted
    /// per case — the class space IS the presented-option space).
    pub n_classes: usize,
    pub feat_dim: usize,
}

#[cfg(feature = "arena-laya")]
pub fn eval_encoder_arm(
    cases: &[riir_reflex::harness::suites::SuiteCase],
    art_path: &std::path::Path,
) -> Result<EncoderArmOut, String> {
    use riir_reflex::harness::runner::case_questions;
    use riir_reflex::laya::config::Checkpoint;
    use riir_reflex::laya::riir::RiirAgent;
    use riir_reflex::laya::weights::weights_root;

    let art = super::encoder_arm::load_sealed(art_path, None)?;
    let agent = RiirAgent::load(&weights_root(), Checkpoint::English)
        .map_err(|e| format!("laya english load: {e}"))?;
    let device: &'static str = agent.device();

    let mut picks = Vec::with_capacity(cases.len());
    let mut correct = Vec::with_capacity(cases.len());
    let mut confs = Vec::with_capacity(cases.len());
    let mut durs_us = Vec::with_capacity(cases.len());

    let mut x: Vec<f32> = Vec::with_capacity(art.feat_dim);
    let mut y: Vec<f32> = Vec::new();

    let run_one =
        |case: &riir_reflex::harness::suites::SuiteCase,
         x: &mut Vec<f32>,
         y: &mut Vec<f32>|
         -> Result<(usize, bool, f64, f64), String> {
            let qs = case_questions(case);
            if qs.len() != 1 {
                return Err(format!(
                    "case {}: the C1 encoder arm scores ONE question per case — got {} \
                     (multi-question suites are out of the C1 scope, refuse loud)",
                    case.id,
                    qs.len()
                ));
            }
            let t = std::time::Instant::now();
            let enc = agent
                .encode_question(&case.state, &qs[0].1)
                .map_err(|e| format!("encode ({}): {e}", case.id))?;
            if enc.markers.len() != art.n_classes {
                return Err(format!(
                    "case {}: encode presents {} options, the head has {} classes — \
                     the class space is the presented-option space; a mismatch is \
                     template drift, refuse loud",
                    case.id,
                    enc.markers.len(),
                    art.n_classes
                ));
            }
            super::encoder_arm::encode_features(
                &enc.marker_rows,
                &enc.hidden,
                enc.d,
                enc.markers.len(),
                x,
            )?;
            if x.len() != art.feat_dim {
                return Err(format!(
                    "case {}: feature dim {} != the head's {} — a different \
                     checkpoint width? refuse loud",
                    case.id,
                    x.len(),
                    art.feat_dim
                ));
            }
            super::encoder_arm::nleh_standardize(&art, x);
            super::encoder_arm::nleh_forward(&art, x, y);
            let dt_us = t.elapsed().as_nanos() as f64 / 1000.0;
            let pick = super::encoder_arm::nleh_pick(y);
            Ok((pick, pick == case.gold[0].idx, f64::from(y[pick]), dt_us))
        };

    // The unmeasured warmup (the harness lane's pre-ramp law: the first
    // Metal forward pays the pipeline compile — never a timed row).
    if let Some(c0) = cases.first() {
        run_one(c0, &mut x, &mut y)?;
    }
    for case in cases {
        let (pick, hit, conf, dt_us) = run_one(case, &mut x, &mut y)?;
        picks.push(pick);
        correct.push(hit);
        confs.push(conf);
        durs_us.push(dt_us);
    }
    Ok(EncoderArmOut {
        device,
        picks,
        correct,
        confs,
        durs_us,
        n_classes: art.n_classes,
        feat_dim: art.feat_dim,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny 2-class artifact (feat_dim 4 = 2 markers × d 2; hidden 3).
    fn tiny_art() -> NlehHead {
        NlehHead {
            feat_dim: 4,
            hidden: 3,
            n_classes: 2,
            mu: vec![0.0; 4],
            inv_std: vec![1.0; 4],
            w1: vec![
                1.0, 0.0, 0.0, 0.0, // h0: x0
                0.0, 1.0, 0.0, 0.0, // h1: x1
                0.0, 0.0, 1.0, 0.0, // h2: x2
            ],
            b1: vec![0.0; 3],
            // class0 reads h0; class1 reads h1 (positive weights) — the
            // pick follows whichever input is larger. Row-major
            // [hidden × n_classes]: j=0 → [2,0], j=1 → [0,2], j=2 → [0,0].
            w2: vec![2.0, 0.0, 0.0, 2.0, 0.0, 0.0],
            b2: vec![0.0; 2],
        }
    }

    fn write_nleh(a: &NlehHead) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"NLEH");
        b.extend_from_slice(&1u32.to_le_bytes());
        b.extend_from_slice(&(a.feat_dim as u32).to_le_bytes());
        b.extend_from_slice(&(a.hidden as u32).to_le_bytes());
        for v in a.mu.iter().chain(&a.inv_std) {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in &a.w1 {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in a.b1.iter().chain(&a.w2).chain(&a.b2) {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    }

    #[test]
    fn round_trips_and_sizes_validate() {
        let art = tiny_art();
        let bytes = write_nleh(&art);
        let back = read_nleh_v1(&bytes).expect("parse");
        assert_eq!(back.feat_dim, 4);
        assert_eq!(back.hidden, 3);
        assert_eq!(back.n_classes, 2);
        assert_eq!(back.w2, art.w2);
    }

    #[test]
    fn refuses_corruption() {
        assert!(read_nleh_v1(b"NOPE____").is_err(), "bad magic");
        assert!(read_nleh_v1(b"NLEH\x02\x00\x00\x00").is_err(), "version");
        // Truncated mid-weights: header asks feat 4 × hidden 3 = 12 w1
        // floats but the bytes stop early.
        let mut b = write_nleh(&tiny_art());
        b.truncate(b.len() - 20);
        assert!(read_nleh_v1(&b).is_err(), "truncated");
        // Trailing junk breaks the n_classes tail decode.
        let mut b2 = write_nleh(&tiny_art());
        b2.extend_from_slice(&0f32.to_le_bytes());
        assert!(read_nleh_v1(&b2).is_err(), "trailing");
    }

    #[test]
    fn forward_matches_the_hand_computed_pick() {
        let art = tiny_art();
        let mut x = vec![0.5, 2.0, 0.0, 0.0]; // x1 > x0 → class 1
        nleh_standardize(&art, &mut x);
        let mut y = Vec::new();
        nleh_forward(&art, &x, &mut y);
        assert_eq!(y.len(), 2);
        assert_eq!(nleh_pick(&y), 1);
        // Flip the inputs → class 0.
        let mut x2 = vec![2.0, 0.5, 0.0, 0.0];
        nleh_standardize(&art, &mut x2);
        let mut y2 = Vec::new();
        nleh_forward(&art, &x2, &mut y2);
        assert_eq!(nleh_pick(&y2), 0);
        // Sigmoid range.
        for v in y.iter().chain(&y2) {
            assert!((0.0..=1.0).contains(v), "sigmoid out of range: {v}");
        }
    }

    #[test]
    fn pick_ties_to_the_later_index() {
        // The trainer's exact law: Iterator::max_by keeps the LAST maximal
        // element. A behavioral pin — changing this to lowest-tie silently
        // re-freezes every read.
        assert_eq!(nleh_pick(&[0.5, 0.5, 0.1]), 1);
        assert_eq!(nleh_pick(&[0.1, 0.5, 0.5]), 2);
    }

    #[test]
    fn features_are_markers_then_pooled_mean() {
        let marker_rows = [1.0f32, 2.0, 3.0, 4.0]; // 2 markers × d 2
        // seq 2: token0 = [10, 20], token1 = [30, 40] → mean [20, 30]
        let hidden = [10.0f32, 20.0, 30.0, 40.0];
        let mut out = Vec::new();
        encode_features(&marker_rows, &hidden, 2, 2, &mut out).expect("feat");
        assert_eq!(out, vec![1.0, 2.0, 3.0, 4.0, 20.0, 30.0]);
        // Shape refusals.
        assert!(encode_features(&marker_rows, &hidden, 3, 2, &mut out).is_err());
        assert!(encode_features(&[1.0], &hidden, 2, 2, &mut out).is_err());
    }
}

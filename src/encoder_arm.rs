//! The encoder-feature arm reader (instinct issue 014 C1): replay the
//! riir-train NLEH frozen head over the laya encoder's own output states,
//! live-encoded per seat case — the RECORD-ONLY arm of the arena (the
//! encoder class is REFUSED at serve per issue 014 decision 1; this module
//! exists to put the measured number on the board with `serve: ✗`, never
//! to seat).
//!
//! Two artifact shapes (the seal carries the version):
//! - **NLEH v1** (issue 014 C1, sst5's shape): fixed class space —
//!   `feat_dim = n_markers·d + d`, ONE question per case, the C1 scope.
//! - **NLEH v2** (issue 016 T9, typed_decisions' shape): PER-OPTION
//!   scoring — `score_i = MLP([marker_i ; pooled]) → sigmoid`, any
//!   presented width, and the MULTI-QUESTION per-case flattening the A0
//!   arms already use (typed carries 5 questions/case).
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
//! C1 bench record is that witness. The v2 half carries the same law: the
//! arena replay must reproduce the trainer's frozen read cell-for-cell
//! (the third-posture witness — 016 T9's 1510/2000).
//!
//! ## The feature law (the exact `lenc_features` order)
//!
//! `x = marker_rows (n_markers·d) ++ mean-over-seq(hidden) (d)` — the
//! mean accumulated per token IN ORDER (bit-order-identical to the
//! trainer), then the artifact's fixed affine `x' = (x − mu)·inv_std`
//! clamped ±30, then the MLP (ReLU hidden, per-class sigmoid) with the
//! trainer's accumulation order (`w1` row-major over `x` in order). The
//! v2 feature law is the same frozen substrate per OPTION:
//! `x_i = [marker_i (d) ; pooled (d)]` — only the grouping changes.

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

/// Read + BLAKE3-verify one sealed artifact's bytes (sidecar
/// `<path>.blake3` unless `blake3_path` names it explicitly — the trainer
/// eval's law). Shared by both version loaders.
fn read_sealed_bytes(
    path: &std::path::Path,
    blake3_path: Option<&std::path::Path>,
) -> Result<Vec<u8>, String> {
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
    Ok(bytes)
}

/// Load + BLAKE3-verify one sealed NLEH **v1** artifact (sidecar
/// `<path>.blake3` unless `blake3_path` names it explicitly — the trainer
/// eval's law). Serve-side v1 posture: the ENC serve lane pins v1 today.
pub fn load_sealed(path: &std::path::Path, blake3_path: Option<&std::path::Path>) -> Result<NlehHead, String> {
    read_nleh_v1(&read_sealed_bytes(path, blake3_path)?)
}

/// A sealed NLEH artifact of either shape — the version rides the bytes'
/// own header (magic + u32 version), never the filename.
pub enum SealedHead {
    V1(NlehHead),
    V2(NlehV2),
}

/// Parse a BLAKE3-verified buffer as the sealed NLEH artifact of EITHER
/// version, dispatching on the header's version word — a v2 artifact must
/// never be parsed as v1 or refused as alien.
fn parse_sealed_head(bytes: &[u8]) -> Result<SealedHead, String> {
    let ver = if bytes.len() >= 8 && &bytes[0..4] == b"NLEH" {
        Some(u32::from_le_bytes(bytes[4..8].try_into().expect("u32")))
    } else {
        None
    };
    match ver {
        Some(1) => Ok(SealedHead::V1(read_nleh_v1(bytes)?)),
        Some(2) => Ok(SealedHead::V2(read_nleh_v2(bytes)?)),
        _ => Err("not an NLEH artifact of a known version (magic/version word)".to_string()),
    }
}

/// Load + BLAKE3-verify a sealed NLEH artifact of EITHER version (sidecar
/// `<path>.blake3` unless `blake3_path` names it explicitly — the trainer
/// eval's law; the version rides the bytes' own header, never the
/// filename).
pub fn load_sealed_head(
    path: &std::path::Path,
    blake3_path: Option<&std::path::Path>,
) -> Result<SealedHead, String> {
    parse_sealed_head(&read_sealed_bytes(path, blake3_path)?)
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

/// The mean-over-seq(hidden) half of BOTH feature laws — `d` wide, the
/// mean accumulated per token IN ORDER (`out[t] contributes hidden[t]/seq`),
/// bit-order-identical to the trainer's `pooled_hidden`. `out` is cleared
/// and resized to `d`.
pub fn pooled_mean(hidden: &[f32], d: usize, out: &mut Vec<f32>) -> Result<(), String> {
    if d == 0 || !hidden.len().is_multiple_of(d) {
        return Err(format!("hidden len {} not a multiple of d {d}", hidden.len()));
    }
    let seq = hidden.len() / d;
    out.clear();
    out.resize(d, 0.0);
    for t in 0..seq {
        let base = t * d;
        for (j, m) in out.iter_mut().enumerate() {
            *m += hidden[base + j] / seq as f32;
        }
    }
    Ok(())
}

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
    let mut mean = Vec::with_capacity(d);
    pooled_mean(hidden, d, &mut mean)?;
    out.clear();
    out.reserve(n_markers * d + d);
    out.extend_from_slice(marker_rows);
    out.extend_from_slice(&mean);
    debug_assert_eq!(out.len(), n_markers * d + d);
    Ok(())
}

/// The trainer's pick law: argmax by `total_cmp`, ties to the LATER index
/// (`Iterator::max_by` semantics — the exact law the frozen read scored
/// under; do not "fix" this to lowest-tie without re-freezing). Shared by
/// BOTH versions — the v2 eval that scored the typed frozen read used the
/// same `max_by` (the lane's "first-argmax" prose describes the no-tie
/// case; the code law is last-tie, and the replay must be bit-faithful to
/// the code).
pub fn nleh_pick(y: &[f32]) -> usize {
    (0..y.len())
        .max_by(|p, r| y[*p].total_cmp(&y[*r]))
        .unwrap_or(0)
}

// ── the NLEH v2 codec (read half only — the trainer writes) ───────────

/// A parsed NLEH v2 artifact — the PER-OPTION scoring shape (instinct
/// issue 016 T9; the law-copy mirror of riir-train's `read_nleh_v2`).
/// `score_i = sigmoid(w2 · ReLU(W1 · std([marker_i ; pooled]) + b1) + b2)`
/// — a single sigmoid per PRESENTED option, any width.
pub struct NlehV2 {
    /// The encoder's hidden width (the marker row width).
    pub d: usize,
    pub hidden: usize,
    /// `[2d]` standardization affine.
    pub mu: Vec<f32>,
    /// `[2d]`.
    pub inv_std: Vec<f32>,
    /// `[2d × hidden]` row-major.
    pub w1: Vec<f32>,
    /// `[hidden]`.
    pub b1: Vec<f32>,
    /// `[hidden]` (the single sigmoid output's weights).
    pub w2: Vec<f32>,
    pub b2: f32,
}

/// Parse + size-validate an NLEH v2 artifact. The layout is FIXED-LENGTH
/// (unlike v1's tail-decoded `n_classes`): `16 + (2d·2 + 2d·hidden +
/// hidden + hidden + 1)·4` bytes — the exact law the trainer's
/// `write_nleh_v2` emits.
pub fn read_nleh_v2(bytes: &[u8]) -> Result<NlehV2, String> {
    let err = |m: &str| format!("NLEH v2: {m}");
    if bytes.len() < 16 || &bytes[0..4] != b"NLEH" {
        return Err(err("not an NLEH artifact"));
    }
    let ver = u32::from_le_bytes(bytes[4..8].try_into().map_err(|_| err("short"))?);
    if ver != 2 {
        return Err(err(
            format!("version {ver} != 2 (a v1 artifact cannot be read as v2 — the shapes differ)").as_str(),
        ));
    }
    let d = u32::from_le_bytes(bytes[8..12].try_into().map_err(|_| err("short"))?) as usize;
    let hidden =
        u32::from_le_bytes(bytes[12..16].try_into().map_err(|_| err("short"))?) as usize;
    if d == 0 || hidden == 0 {
        return Err(err("zero dim"));
    }
    let want = 16 + (2 * d * 2 + 2 * d * hidden + hidden + hidden + 1) * 4;
    if bytes.len() != want {
        return Err(err(
            format!(
                "length {} != the v2 layout's {} (d {d} · hidden {hidden})",
                bytes.len(),
                want
            )
            .as_str(),
        ));
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
    let mu = take(&mut pos, 2 * d)?;
    let inv_std = take(&mut pos, 2 * d)?;
    let w1 = take(&mut pos, 2 * d * hidden)?;
    let b1 = take(&mut pos, hidden)?;
    let w2 = take(&mut pos, hidden)?;
    let b2 = f32::from_le_bytes(
        bytes[pos..pos + 4]
            .try_into()
            .map_err(|_| err("b2 short"))?,
    );
    Ok(NlehV2 {
        d,
        hidden,
        mu,
        inv_std,
        w1,
        b1,
        w2,
        b2,
    })
}

/// The v2 eval forward over ONE live encode: pooled mean over the
/// sequence axis, then per PRESENTED option `x = [marker_i ; pooled]` →
/// the artifact's affine (±30 clamp) → ReLU hidden → single sigmoid.
/// Scores land in `out` in presented order; `x2d` (2d) and `pooled` (d)
/// are caller scratch, reused across options. Same accumulation order as
/// the trainer's forward (`w1` row-major over `x` in order, the output
/// sum over `j` in order) so the read is bit-faithful to what the holdout
/// and the frozen test read scored.
pub fn nleh_v2_forward(
    a: &NlehV2,
    marker_rows: &[f32],
    hidden_state: &[f32],
    x2d: &mut Vec<f32>,
    pooled: &mut Vec<f32>,
    h: &mut Vec<f32>,
    out: &mut Vec<f32>,
) -> Result<(), String> {
    let d = a.d;
    pooled_mean(hidden_state, d, pooled)?;
    if !marker_rows.len().is_multiple_of(d) {
        return Err(format!(
            "marker_rows len {} not a multiple of d {d}",
            marker_rows.len()
        ));
    }
    h.clear();
    h.resize(a.hidden, 0.0);
    out.clear();
    out.reserve(marker_rows.len() / d);
    for marker in marker_rows.chunks_exact(d) {
        x2d.clear();
        x2d.extend_from_slice(marker);
        x2d.extend_from_slice(pooled);
        debug_assert_eq!(x2d.len(), 2 * d);
        for (j, v) in x2d.iter_mut().enumerate() {
            *v = ((*v - a.mu[j]) * a.inv_std[j]).clamp(-30.0, 30.0);
        }
        for (hj, hs) in h.iter_mut().enumerate() {
            let mut s = a.b1[hj];
            let base = hj * 2 * d;
            for (off, &xi) in x2d.iter().enumerate() {
                s += a.w1[base + off] * xi;
            }
            *hs = s.max(0.0);
        }
        let mut s = a.b2;
        for (j, &hv) in h.iter().enumerate() {
            s += a.w2[j] * hv;
        }
        out.push(1.0 / (1.0 + (-s).exp()));
    }
    Ok(())
}

// ── the seat-side runner (the arena's encoder arm — cfg arena-laya) ────

/// The record-only encoder arm over a seat's test cases (issue 014 C1,
/// widened to the v2 per-option shape by issue 016 T9): live-encode
/// through the named laya checkpoint (the SAME agent + wire form the
/// harness laya lane and the riir-train dump use), replay the sealed NLEH
/// head, and return per-row pick/correct/confidence/µs rows.
///
/// - **v1**: ONE question per case — the C1 scope (sst5's shape); a
///   multi-question suite is refused loud, not silently mis-scored.
/// - **v2**: per-option scoring over EVERY question per case — the A0
///   arms' per-question flattening (typed carries 5 questions/case; the
///   row denominator is the question count, the board's own convention).
///   Any presented width reads (the class space is per-question).
///
/// The checkpoint is the CALLER's choice (the arena's `--encoder-ckpt`):
/// the artifact does not carry its training checkpoint, and a v2 head
/// trained on the typed cache is meaningless over the english encoder —
/// the pairing is a real functional fact, never inferred from version or
/// filename.
///
/// Latency is the FULL arm cost per row (encode + feature + head
/// forward), the number the serve refusal is priced against. The first
/// ROW is an UNMEASURED warmup (the Metal pipeline-compile law — the
/// harness lane's pre-ramp).
#[cfg(feature = "arena-laya")]
pub struct EncoderArmOut {
    pub device: &'static str,
    pub picks: Vec<usize>,
    pub correct: Vec<bool>,
    pub confs: Vec<f64>,
    pub durs_us: Vec<f64>,
    /// "v1" / "v2 per-option" — the artifact's own shape word.
    pub head_kind: &'static str,
    /// The laya checkpoint's subfolder ("english" / "typed" /
    /// "multilingual") — disclosed beside every number.
    pub ckpt: &'static str,
    /// The honest shape description for the record prose (v1: "4 classes ·
    /// feat 5120"; v2: "d 1024 · hidden 128 · widths 2/4/5").
    pub shape_desc: String,
}

#[cfg(feature = "arena-laya")]
pub fn eval_encoder_arm(
    cases: &[riir_reflex::harness::suites::SuiteCase],
    art_path: &std::path::Path,
    ckpt: riir_reflex::laya::config::Checkpoint,
) -> Result<EncoderArmOut, String> {
    use riir_reflex::harness::runner::case_questions;
    use riir_reflex::laya::riir::RiirAgent;
    use riir_reflex::laya::weights::weights_root;

    let head = super::encoder_arm::load_sealed_head(art_path, None)?;
    let agent = RiirAgent::load(&weights_root(), ckpt)
        .map_err(|e| format!("laya {} load: {e}", ckpt.subfolder()))?;
    let device: &'static str = agent.device();

    let mut picks: Vec<usize> = Vec::new();
    let mut correct: Vec<bool> = Vec::new();
    let mut confs: Vec<f64> = Vec::new();
    let mut durs_us: Vec<f64> = Vec::new();

    // The unmeasured warmup (the harness lane's pre-ramp law: the first
    // Metal forward pays the pipeline compile — never a timed row). The
    // warmup row is then re-run measured inside the loops, so every row
    // scores exactly once on the record.
    let shape_desc: String;
    match &head {
        SealedHead::V1(art) => {
            shape_desc = format!("{} classes · feat {}", art.n_classes, art.feat_dim);
            let mut x: Vec<f32> = Vec::with_capacity(art.feat_dim);
            let mut y: Vec<f32> = Vec::new();
            if let Some(c0) = cases.first() {
                let qdef = one_question_def(c0)?;
                score_row_v1(&agent, art, c0, &qdef, &mut x, &mut y)?;
            }
            for case in cases {
                let qdef = one_question_def(case)?;
                let t = std::time::Instant::now();
                let (pick, hit, conf) = score_row_v1(&agent, art, case, &qdef, &mut x, &mut y)?;
                let dt_us = t.elapsed().as_nanos() as f64 / 1000.0;
                picks.push(pick);
                correct.push(hit);
                confs.push(conf);
                durs_us.push(dt_us);
            }
        }
        SealedHead::V2(art) => {
            if art.d == 0 {
                return Err("NLEH v2: zero marker width".to_string());
            }
            let mut scratch = V2Scratch {
                x2d: Vec::with_capacity(2 * art.d),
                pooled: Vec::with_capacity(art.d),
                h: vec![0f32; art.hidden],
                y: Vec::new(),
            };
            if let Some(c0) = cases.first() {
                let qs = case_questions(c0);
                let qdef = &qs
                    .first()
                    .ok_or_else(|| format!("case {}: zero questions", c0.id))?
                    .1;
                score_row_v2(&agent, art, c0, 0, qdef, &mut scratch)?;
            }
            // The T9 widening: every question per case is its own row (the
            // A0 arms' flattening — the board's typed denominator is 2000
            // questions, not 400 cases). Widths are collected for the
            // record prose (typed reads 2/4/5).
            let mut widths: Vec<usize> = Vec::new();
            for case in cases {
                let qs = case_questions(case);
                if qs.is_empty() {
                    return Err(format!(
                        "case {}: zero questions — the seat builder's own refusal shape",
                        case.id
                    ));
                }
                if qs.len() != case.gold.len() {
                    return Err(format!(
                        "case {}: {} questions but {} gold rows — the seat builds them in lockstep",
                        case.id,
                        qs.len(),
                        case.gold.len()
                    ));
                }
                for (qi, (_, qdef)) in qs.iter().enumerate() {
                    let t = std::time::Instant::now();
                    let (pick, hit, conf, width) =
                        score_row_v2(&agent, art, case, qi, qdef, &mut scratch)?;
                    let dt_us = t.elapsed().as_nanos() as f64 / 1000.0;
                    widths.push(width);
                    picks.push(pick);
                    correct.push(hit);
                    confs.push(conf);
                    durs_us.push(dt_us);
                }
            }
            widths.sort_unstable();
            widths.dedup();
            let w: Vec<String> = widths.iter().map(|x| x.to_string()).collect();
            shape_desc = format!("d {} · hidden {} · widths {}", art.d, art.hidden, w.join("/"));
        }
    }

    let head_kind = match &head {
        SealedHead::V1(_) => "v1",
        SealedHead::V2(_) => "v2 per-option",
    };
    Ok(EncoderArmOut {
        device,
        picks,
        correct,
        confs,
        durs_us,
        head_kind,
        ckpt: ckpt.subfolder(),
        shape_desc,
    })
}

/// The v1 C1 shape check + the single question's def: ONE question per
/// case (sst5's scope); a multi-question suite is refused loud, never
/// silently mis-scored.
#[cfg(feature = "arena-laya")]
fn one_question_def(
    case: &riir_reflex::harness::suites::SuiteCase,
) -> Result<serde_json::Value, String> {
    use riir_reflex::harness::runner::case_questions;
    let qs = case_questions(case);
    if qs.len() != 1 {
        return Err(format!(
            "case {}: the C1 v1 encoder arm scores ONE question per case — got {} \
             (multi-question suites need the v2 per-option artifact; issue 016 T9)",
            case.id,
            qs.len()
        ));
    }
    Ok(qs.into_iter().next().expect("len checked").1)
}

/// The v1 per-case scorer (the C1 one-question law): encode → the fixed
/// feature law → the head. Returns (pick, hit, conf).
#[cfg(feature = "arena-laya")]
fn score_row_v1(
    agent: &riir_reflex::laya::riir::RiirAgent,
    art: &NlehHead,
    case: &riir_reflex::harness::suites::SuiteCase,
    qdef: &serde_json::Value,
    x: &mut Vec<f32>,
    y: &mut Vec<f32>,
) -> Result<(usize, bool, f64), String> {
    let enc = agent
        .encode_question(&case.state, qdef)
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
    super::encoder_arm::nleh_standardize(art, x);
    super::encoder_arm::nleh_forward(art, x, y);
    let pick = super::encoder_arm::nleh_pick(y);
    Ok((pick, pick == case.gold[0].idx, f64::from(y[pick])))
}

/// The v2 per-question scorer (the T9 widening): encode the case's qi-th
/// question → per-option `[marker_i ; pooled]` → the head → argmax (the
/// shared `nleh_pick` last-tie law) over the presented options. Returns
/// (pick, hit, conf, width). The gold idx speaks the SAME presented space
/// (the seat builder's law, mirrored by the t608 case-maker — the
/// floor-reproduction witness).
/// The v2 per-question scratch (the four buffers travel together — one
/// allocation site, reused across every row of the run).
#[cfg(feature = "arena-laya")]
struct V2Scratch {
    x2d: Vec<f32>,
    pooled: Vec<f32>,
    h: Vec<f32>,
    y: Vec<f32>,
}

#[cfg(feature = "arena-laya")]
fn score_row_v2(
    agent: &riir_reflex::laya::riir::RiirAgent,
    art: &NlehV2,
    case: &riir_reflex::harness::suites::SuiteCase,
    qi: usize,
    qdef: &serde_json::Value,
    s: &mut V2Scratch,
) -> Result<(usize, bool, f64, usize), String> {
    let enc = agent
        .encode_question(&case.state, qdef)
        .map_err(|e| format!("encode ({} q{qi}): {e}", case.id))?;
    if enc.d != art.d {
        return Err(format!(
            "case {}: encode width {} != the head's d {} — a different \
             checkpoint? refuse loud",
            case.id,
            enc.d,
            art.d
        ));
    }
    let gold = case
        .gold
        .get(qi)
        .ok_or_else(|| format!("case {}: gold {qi} missing", case.id))?;
    if gold.idx >= enc.markers.len() {
        return Err(format!(
            "case {} q{qi}: gold idx {} outside the presented space ({} options)",
            case.id,
            gold.idx,
            enc.markers.len()
        ));
    }
    super::encoder_arm::nleh_v2_forward(
        art,
        &enc.marker_rows,
        &enc.hidden,
        &mut s.x2d,
        &mut s.pooled,
        &mut s.h,
        &mut s.y,
    )?;
    let pick = super::encoder_arm::nleh_pick(&s.y);
    Ok((
        pick,
        pick == gold.idx,
        f64::from(s.y[pick]),
        enc.markers.len(),
    ))
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

    // ── the v2 half (issue 016 T9 — the per-option shape) ──────────

    /// A tiny v2 artifact (d 2 · hidden 2): the score reads marker[0]
    /// through h0 (positive) and the pooled tail through h1 (zeroed by
    /// weights) — the pick follows the option whose FIRST marker
    /// coordinate is larger.
    fn tiny_v2() -> NlehV2 {
        NlehV2 {
            d: 2,
            hidden: 2,
            mu: vec![0.0; 4],
            inv_std: vec![1.0; 4],
            // x = [marker_i (2) ; pooled (2)]: h0 reads x0, h1 reads x1.
            w1: vec![
                1.0, 0.0, 0.0, 0.0, // h0: x0
                0.0, 1.0, 0.0, 0.0, // h1: x1 (unused by the output)
            ],
            b1: vec![0.0; 2],
            w2: vec![2.0, 0.0], // score reads h0 only
            b2: 0.0,
        }
    }

    fn write_nleh_v2(a: &NlehV2) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"NLEH");
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&(a.d as u32).to_le_bytes());
        b.extend_from_slice(&(a.hidden as u32).to_le_bytes());
        for v in a.mu.iter().chain(&a.inv_std) {
            b.extend_from_slice(&v.to_le_bytes());
        }
        for v in a.w1.iter().chain(&a.b1).chain(&a.w2) {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b.extend_from_slice(&a.b2.to_le_bytes());
        b
    }

    #[test]
    fn v2_round_trips_and_sizes_validate() {
        let art = tiny_v2();
        let bytes = write_nleh_v2(&art);
        let back = read_nleh_v2(&bytes).expect("parse");
        assert_eq!(back.d, 2);
        assert_eq!(back.hidden, 2);
        assert_eq!(back.w2, art.w2);
        assert_eq!(back.b2, art.b2);
        // A v1 artifact cannot be read as v2 (the shapes differ).
        assert!(read_nleh_v2(&write_nleh(&tiny_art())).is_err(), "v1 as v2");
        assert!(read_nleh_v1(&write_nleh_v2(&tiny_v2())).is_err(), "v2 as v1");
    }

    #[test]
    fn v2_refuses_corruption() {
        assert!(read_nleh_v2(b"NOPE____").is_err(), "bad magic");
        assert!(read_nleh_v2(b"NLEH\x03\x00\x00\x00").is_err(), "version");
        let mut b = write_nleh_v2(&tiny_v2());
        b.truncate(b.len() - 8);
        assert!(read_nleh_v2(&b).is_err(), "truncated");
        let mut b2 = write_nleh_v2(&tiny_v2());
        b2.extend_from_slice(&0f32.to_le_bytes());
        assert!(read_nleh_v2(&b2).is_err(), "trailing (fixed-length layout)");
    }

    #[test]
    fn v2_forward_scores_each_option_on_its_own_marker() {
        let art = tiny_v2();
        // 3 presented options (any width reads — the v2 law): markers
        // [0.1, _], [0.9, _], [0.5, _]; pooled identical for all. The
        // score reads marker[0] → option 1 wins.
        let marker_rows = vec![0.1f32, 0.0, 0.9, 0.0, 0.5, 0.0];
        let hidden = vec![7.0f32, 7.0, 7.0, 7.0]; // pooled [7, 7], identical
        let mut x2d = Vec::new();
        let mut pooled = Vec::new();
        let mut h = vec![0f32; art.hidden];
        let mut y = Vec::new();
        nleh_v2_forward(&art, &marker_rows, &hidden, &mut x2d, &mut pooled, &mut h, &mut y)
            .expect("forward");
        assert_eq!(y.len(), 3, "one score per presented option");
        assert_eq!(nleh_pick(&y), 1);
        assert!(y[1] > y[0] && y[1] > y[2], "monotone in the marker");
        for v in &y {
            assert!((0.0..=1.0).contains(v), "sigmoid out of range: {v}");
        }
        // Widths may MIX (2-wide and 3-wide rows in one call sequence —
        // the presented space is per-row, never global).
        let marker_rows_2 = vec![0.2f32, 0.0, 0.8, 0.0];
        nleh_v2_forward(&art, &marker_rows_2, &hidden, &mut x2d, &mut pooled, &mut h, &mut y)
            .expect("forward 2-wide");
        assert_eq!(y.len(), 2);
        assert_eq!(nleh_pick(&y), 1);
        // A marker_rows length not a multiple of d refuses.
        assert!(nleh_v2_forward(&art, &marker_rows[..5], &hidden, &mut x2d, &mut pooled, &mut h, &mut y).is_err());
    }

    #[test]
    fn sealed_head_dispatches_on_the_version_word() {
        // The dispatch is on the header's version word, never the filename:
        // a v2 buffer through the same parse path the loader uses reads as
        // V2 (parse_sealed_head IS load_sealed_head minus the file read).
        assert!(matches!(
            parse_sealed_head(&write_nleh_v2(&tiny_v2())),
            Ok(SealedHead::V2(_))
        ));
        assert!(matches!(
            parse_sealed_head(&write_nleh(&tiny_art())),
            Ok(SealedHead::V1(_))
        ));
        assert!(parse_sealed_head(b"JUNK").is_err());
    }
}

// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 206-251

/// The issue-014 C1 encoder-arm face (RECORD-ONLY, `serve: ✗`): the
/// frozen seat read of the NLEH head over the live laya-english encode,
/// paired against the INCUMBENT A1 bag arm on the same rows (the T2
/// gate's comparator — the cross-pool claim the C1 read settles). The
/// per-row latency is the full arm cost (encode + head), the number the
/// serve refusal is priced against (the provisional text-lane bar
/// ≤ ~300 µs; the encoder reads ms-class — the evidence, not a pass).
struct EncoderFace {
    /// Path of the sealed NLEH artifact this read replayed.
    art: String,
    accuracy: f64,
    n: usize,
    /// Paired (ENC − A1) mean accuracy delta on the same frozen rows.
    mean_vs_a1: f64,
    /// Its 95% lower bound — > 0 certifies the encoder arm strictly
    /// above the incumbent (the T2 form, A1 the comparator).
    lb95_vs_a1: f64,
    p50_us: f64,
    p99_us: f64,
    /// The laya device posture the encode ran on ("metal" on the M3
    /// read — disclosed beside every latency figure).
    device: &'static str,
    /// "v1" / "v2 per-option" — the artifact's own shape word (016 T9).
    head_kind: &'static str,
    /// The laya checkpoint's subfolder the encode ran on ("english" /
    /// "typed" / "multilingual") — the caller's `--encoder-ckpt`.
    ckpt: &'static str,
    /// The honest shape description (v1: "4 classes · feat 5120"; v2:
    /// "d 1024 · hidden 128 · widths 2/4/5").
    shape_desc: String,
    /// Per-row freezes (predictions.json): the picks/correct/confs/µs
    /// arrays — the doc builder's stats laws consume them where present.
    picks: Vec<usize>,
    correct: Vec<bool>,
    confs: Vec<f64>,
    durs_us: Vec<f64>,
    /// The WEIGHT posture the encode ran under (issue 018 Lane D1/D4):
    /// "f16" (the shipped posture — every seated cell), "fake-quant-q8"
    /// (the D1 probe read) or "fake-quant-q4" (the D4 probe read).
    #[cfg(feature = "arena-laya")]
    weight_posture: &'static str,
    /// The fake-quant report (Some iff the posture is a fake-quant),
    /// serialized verbatim — the disclosure of what was quantized.
    #[cfg(feature = "arena-laya")]
    fake_quant_report: Option<riir_reflex::laya::riir::fake_quant::FakeQuantReport>,
}

// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 291-297

    laya: Option<LayaFace>,
    /// The issue-014 C1 encoder arm (Some iff `--encoder-art` named a
    /// sealed NLEH head and the feature armed it): a RECORD-ONLY row —
    /// never a registration candidate, never served (the encoder class
    /// is refused at serve, issue 014 decision 1).
    encoder: Option<EncoderFace>,
}

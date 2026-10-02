// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 559-570

    if !md.is_file() {
        return Err(format!(
            "{} is not a regular file (artifacts are files)",
            path.display()
        ));
    }
    if md.len() > cap as u64 + 1 {
        return Err(format!(
            "{} is {} bytes, over the {cap} byte cap",
            path.display(),
            md.len()
        ));

// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 579-649

        return Err(format!(
            "{} is {} bytes, over the {cap} byte cap",
            path.display(),
            buf.len()
        ));
    }
    Ok(buf)
}

/// What a successful load hands the installer: the serving server, the
/// artifact bytes' BLAKE3 (the epoch tag's digest half — the same
/// quantity the manifest row pins), and (vessel mode) the vessel facts
/// the host persists after a full install.
struct LoadedLane {
    server: AnySuiteServer,
    artifact_digest: [u8; 32],
    #[cfg(feature = "vessel")]
    facts: Option<riir_instinct::server::VesselFacts>,
}

/// Load one suite's lane: the seat (datasets) + the artifact (the row's
/// file, or the swap's explicit artifact), read ONCE and built from those
/// bytes. Vessel mode when configured (fail-closed — no raw-winner
/// fallback inside vessel mode), else the raw sealed winners.
fn load_lane(
    ctx: &BootCtx,
    suite: &'static str,
    artifact: Option<&str>,
) -> Result<LoadedLane, String> {
    let Some(row) = ctx.manifest.row(suite) else {
        return Err(format!("suite {suite} is not in the arsenal manifest"));
    };
    let cap = (row.budget.max_payload_mb << 20) as usize;
    // The seated synth corpus (Plan 426 T6): env dir + a present artifact
    // for THIS suite seats it; anything else keeps the gold seat. A present
    // artifact that FAILS verification is fatal (a corrupt corpus must
    // never degrade into a quiet gold seat — the vessel reader's law).
    let seat = match ctx.synth_corpus_dir.as_ref() {
        Some(dir) => {
            let synth = Path::new(dir).join(format!("{suite}_synth.jsonl"));
            if synth.is_file() {
                let s = riir_reflex::harness::runner::seat::prepare_seat_with_synth(
                    suite,
                    Path::new(&ctx.datasets_dir),
                    &synth,
                    SYNTH_EXTRA_CAP,
                )?;
                let meta = s.synth.as_ref().expect("with_synth seated the corpus");
                eprintln!(
                    "  lane {suite}: synth corpus seated — {} rows ({} dropped) · digest {}…",
                    meta.docs.len(),
                    meta.rows_dropped,
                    &meta.digest_hex[..16.min(meta.digest_hex.len())]
                );
                s
            } else {
                riir_reflex::harness::runner::seat::prepare_seat(
                    suite,
                    Path::new(&ctx.datasets_dir),
                )?
            }
        }
        None => {
            riir_reflex::harness::runner::seat::prepare_seat(suite, Path::new(&ctx.datasets_dir))?
        }
    };
    #[cfg(feature = "vessel")]
    if let Some(cfg) = ctx.vessel.as_ref() {
        if row.digest.is_none() {
            return Err(format!(
                "suite {suite}: the row carries no artifact digest — the vessel lane loads \

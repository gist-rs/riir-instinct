// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 141-246

                suite_filter = Some(names);
            }
            other => die(&format!("unknown arg {other}")),
        }
        i += 1;
    }

    // Vessel mode: INSTINCT_VESSEL_DIR set → the specialists load from
    // HOSTED-ONLY vessels. Fail-closed configuration: the key and at
    // least one operator pin are REQUIRED the moment the dir is set — a
    // half-configured vessel boot must refuse, never fall back to raw
    // winners (that would be the moat leak the class discipline exists
    // to prevent, wearing a fallback's clothes). Runs BEFORE the bind:
    // a misconfigured host never opens a port.
    #[cfg(feature = "vessel")]
    let vessel = match (&vessel_dir, &vessel_key_hex) {
        (Some(dir), Some(key_hex)) => {
            let key_bytes = decode_hex32(key_hex)
                .unwrap_or_else(|| die("INSTINCT_VESSEL_KEY_HEX must be 64 hex chars (32 bytes)"));
            let pins_hex = vessel_pins_hex.as_deref().unwrap_or("");
            if pins_hex.trim().is_empty() {
                die("INSTINCT_VESSEL_PINS_HEX is required in vessel mode (id:pubkey hex, comma-\n                    separated) — no compiled-in pins exist, the operator pins the mint key");
            }
            let mut keys: Vec<(u32, [u8; 32])> = Vec::new();
            for pin in pins_hex.split(',') {
                let pin = pin.trim();
                if pin.is_empty() {
                    continue;
                }
                let Some((id, hex)) = pin.split_once(':') else {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX entry {pin:?} is not id:hex"));
                };
                let id: u32 = id.trim().parse().unwrap_or_else(|_| {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX key-id {id:?} is not a number"))
                });
                let bytes = decode_hex32(hex.trim()).unwrap_or_else(|| {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX entry {id} is not 64 hex chars"))
                });
                keys.push((id, bytes));
            }
            eprintln!(
                "[riir-instinct] vessel mode: dir {dir} · {} pin(s) · state dir {state_dir}",
                keys.len()
            );
            Some(VesselConfig {
                dir: dir.into(),
                key: key_bytes,
                pins: reflexer_vessel::pins_from_bytes(&keys),
                state_dir: state_dir.into(),
            })
        }
        (Some(_), None) => die(
            "INSTINCT_VESSEL_DIR is set without INSTINCT_VESSEL_KEY_HEX — vessel mode is a\n             whole configuration (dir + key + pins), never a partial one",
        ),
        (None, Some(_)) => die(
            "INSTINCT_VESSEL_KEY_HEX is set without INSTINCT_VESSEL_DIR — vessel mode is a\n             whole configuration (dir + key + pins), never a partial one",
        ),
        (None, None) => None,
    };
    #[cfg(not(feature = "vessel"))]
    if vessel_dir.is_some() || vessel_key_hex.is_some() {
        die(
            "vessel env vars are set but this binary was built WITHOUT the `vessel` feature —\n             rebuild with --features vessel (the reader is opt-in by design)",
        );
    }
    #[cfg(not(feature = "vessel"))]
    struct VesselConfig;
    #[cfg(not(feature = "vessel"))]
    let vessel: Option<VesselConfig> = None;
    #[cfg(not(feature = "vessel"))]
    let _ = &vessel;

    // The arsenal manifest (Proposal 001, law A5 — the ONE selection
    // surface): parse, then validate BEFORE lanes resolve AND before the
    // bind — a manifest that refuses never opens a port. Drift (artifact
    // bytes ≠ the pinned digest), an unknown posture arm, a class the
    // reader cannot honor: all loud here, naming the row + field.
    let (manifest, arsenal_digest, arsenal_desc): (ArsenalManifest, String, String) =
        match &arsenal_path {
            Some(p) => {
                let text = std::fs::read_to_string(p)
                    .unwrap_or_else(|e| die(&format!("read arsenal manifest {p}: {e}")));
                let digest = ArsenalManifest::digest_of(&text);
                let m = ArsenalManifest::parse(&text)
                    .unwrap_or_else(|e| die(&format!("arsenal manifest {p}: {e}")));
                (m, digest, p.clone())
            }
            None => (
                ArsenalManifest::embedded_default()
                    .unwrap_or_else(|e| die(&format!("embedded arsenal manifest: {e}"))),
                ArsenalManifest::embedded_manifest_digest(),
                "embedded default".into(),
            ),
        };
    let manifest = std::sync::Arc::new(manifest);
    #[cfg(feature = "vessel")]
    let vctx = match vessel.as_ref() {
        Some(cfg) => riir_instinct::arsenal::ValidateCtx::vessel(
            std::path::Path::new(&winners_dir),
            &cfg.dir,
            &cfg.pins,
        ),
        None => riir_instinct::arsenal::ValidateCtx::raw(std::path::Path::new(&winners_dir)),
    };
    #[cfg(not(feature = "vessel"))]
    let vctx = riir_instinct::arsenal::ValidateCtx::raw(std::path::Path::new(&winners_dir));

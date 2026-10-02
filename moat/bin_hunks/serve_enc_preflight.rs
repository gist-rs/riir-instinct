// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 286-383

    // The lazy-ENC boot preflight (issue 018 Lane A): a lazy encoder row
    // must refuse its config errors HERE, at boot — never on the first
    // request. Three halves, all loud: the weights (pinned files present
    // + digest-matching, the ACTIVE variant sidecar), the memory budget
    // (counted per SHARED worker — distinct checkpoints × artifact bytes,
    // never per lane), and per-row the template-vs-head class count
    // WITHOUT a forward pass (the seat is rebuilt at the row's first
    // load — disclosed, boot-time cost only). Compiled with the lane —
    // the default build has no ENC lane to preflight.
    #[cfg(feature = "serve-encoder")]
    {
        let enc_lazy_suites: Vec<&'static str> = suites
            .iter()
            .copied()
            .filter(|s| {
                let row = manifest.row(s).unwrap_or_else(|| {
                    die(&format!("suite {s}: vanished from the validated manifest"))
                });
                row.to_arm() == Ok(riir_instinct::server::Arm::Enc) && row.budget.load == "lazy"
            })
            .collect();
        if !enc_lazy_suites.is_empty() {
            let facts = riir_instinct::encoder_serve::preflight_enc_weights()
                .unwrap_or_else(|e| die(&format!("ENC lazy preflight: {e}")));
            eprintln!(
                "[riir-instinct] ENC preflight: weights ok — {} MiB at {} (variant {}, digest {})",
                facts.bytes >> 20,
                facts.weights_path.display(),
                facts.variant.unwrap_or("f16"),
                facts.digest16
            );
            match std::env::var("INSTINCT_ENCODER_MEM_BUDGET_MB") {
                Ok(raw) => {
                    let budget_mb: u64 = raw.trim().parse().unwrap_or_else(|_| {
                        die(&format!(
                            "INSTINCT_ENCODER_MEM_BUDGET_MB={raw:?} is not a number"
                        ))
                    });
                    // Per SHARED worker: the distinct-checkpoint charge.
                    // ONE checkpoint serves today (`ENC_CHECKPOINT_LABEL`),
                    // so the sum is that artifact once — three english
                    // lanes must never charge it three times.
                    let total = facts.bytes;
                    if total > budget_mb << 20 {
                        die(&format!(
                            "ENC memory budget: {} distinct checkpoint worker(s) charge {} MiB, \
                             over INSTINCT_ENCODER_MEM_BUDGET_MB={budget_mb} — shrink the seated \
                             ENC rows or raise the budget (device capacity is the operator's \
                             declared ceiling; this box does not expose a portable GPU-memory \
                             query)",
                            1,
                            total >> 20
                        ));
                    }
                    eprintln!(
                        "[riir-instinct] ENC preflight: memory budget ok — {} MiB charged (1 \
                         distinct checkpoint worker) of {budget_mb} MiB",
                        total >> 20
                    );
                }
                Err(_) => {
                    eprintln!(
                        "[riir-instinct] ENC preflight: no INSTINCT_ENCODER_MEM_BUDGET_MB — \
                         residency is disclosed ({} MiB/worker), not gated",
                        facts.bytes >> 20
                    );
                }
            }
            for s in &enc_lazy_suites {
                let row = manifest.row(s).expect("row from the validated manifest");
                #[cfg(feature = "vessel")]
                if vessel.is_some() {
                    eprintln!(
                        "[riir-instinct] ENC preflight: {s} — vessel mode: the template half \
                         rides the first load (the head ships inside the signed vessel; \
                         authenticity + template checks run there) — disclosed, not skipped"
                    );
                    continue;
                }
                let head_name = row.artifact_file(format!("{s}_encoder_head_v1.bin"));
                let head_path = std::path::Path::new(&winners_dir).join(&head_name);
                let bytes = std::fs::read(&head_path)
                    .unwrap_or_else(|e| die(&format!("ENC preflight: read head {head_name}: {e}")));
                let seat = riir_reflex::harness::runner::seat::prepare_seat(
                    s,
                    std::path::Path::new(&datasets_dir),
                )
                .unwrap_or_else(|e| die(&format!("ENC preflight: prepare {s} seat: {e}")));
                let n_classes =
                    riir_instinct::encoder_serve::preflight_enc_template(s, &bytes, &seat)
                        .unwrap_or_else(|e| die(&format!("ENC preflight: {s}: {e}")));
                eprintln!(
                    "[riir-instinct] ENC preflight: {s} template ok (head {n_classes} classes) \
                     — the seat rebuilds at the row's first load (lazy)"
                );
            }
        }
    }

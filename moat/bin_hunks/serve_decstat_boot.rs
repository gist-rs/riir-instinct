// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 325-376

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

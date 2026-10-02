// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1357-1374

            &format!(
                "suite {:?} is lazy and not loaded — load triggered by this request; retry shortly",
                req.suite
            ),
            cors,
        );
        return;
    }
    let mut guard = slot.state.lock().expect("slot lock");
    let (server, lane_load): (&mut AnySuiteServer, &str) = match &mut *guard {
        LaneState::Ready { server, .. } => (server, "ready"),
        LaneState::Loading { .. } => {
            json_error(
                stream,
                "503 Service Unavailable",
                "loading",
                &format!(
                    "suite {:?} is still loading (seat boot or lazy load) — retry shortly",

// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1384-1391

                "503 Service Unavailable",
                "lane_failed",
                &format!("suite {:?} failed to load: {error}", req.suite),
                cors,
            );
            return;
        }
        LaneState::Unloaded { .. } => unreachable!("the lazy trigger above consumed Unloaded"),

// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1456-1464

            // The capture is AFTER the response is written — the decide
            // path's latency never sees it, and a capture panic (none
            // expected; the sink is infallible) could not eat a reply.
            #[cfg(feature = "decstat")]
            riir_instinct::decstat::record(d.suite, &d.arm, d.abstained);
        }
        Ok(MultiDecision::Many(decisions)) => {
            let wire = wire_questions.as_deref().unwrap_or_default();
            let docs: Vec<serde_json::Value> = decisions

// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 1485-1496

                            "features": COMPILED_FEATURES,
                            "input": input_blake3(&req.state, &d.options),
                            "decision": decision_blake3(d),
                            "lane": "hybrid",
                        },
                    })
                })
                .collect();
            let doc = serde_json::json!({
                "suite": req.suite,
                "lane": "hybrid",
                "n_decisions": decisions.len(),

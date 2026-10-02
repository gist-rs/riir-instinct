// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 2423-2444

            "encoder_arm": run.encoder.as_ref().map(encoder_arm_json),
            "arms": arms,
        }));
    }
    let doc = serde_json::json!({ "frozen_test_predictions": suites });
    let path = out_dir.join("predictions.json");
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&doc).expect("predictions json"),
    )
    .expect("write predictions");
    eprintln!("frozen predictions: {}", path.display());

    // The full pre-registration table (the RESULTS.md text has promised
    // this file since Bench 001; write it for real).
    let regs: serde_json::Value = serde_json::to_value(
        runs.iter()
            .map(|r| {
                serde_json::json!({
                    "suite": r.suite,
                    "registered": r.registered.name(),
                    "rows": r.registration.iter().map(|row| serde_json::json!({

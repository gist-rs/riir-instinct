// origin: src/bin/serve.rs @ B1 seam commit 22ae291, lines 410-439

    {
        let consent = riir_instinct::decstat::consent_enabled(
            std::env::var("RIIR_INSTINCT_STATS").ok().as_deref(),
        );
        if consent {
            let key_path = std::env::var("INSTINCT_ACCOUNT_KEY").unwrap_or_else(|_| {
                die(
                    "RIIR_INSTINCT_STATS=on requires INSTINCT_ACCOUNT_KEY=<64-hex seed file> — \
                     the row is signed with the account key; refusing is the honest posture",
                )
            });
            let key =
                riir_instinct::decstat::load_signing_key(&key_path).unwrap_or_else(|e| die(&e));
            let url = std::env::var("INSTINCT_KAT_SERVICE_URL")
                .unwrap_or_else(|_| riir_instinct::decstat::DEFAULT_SERVICE_URL.into());
            let machine = std::env::var("INSTINCT_MACHINE_LABEL").unwrap_or_default();
            let toolchain = format!("rustc {RUSTC_RELEASE}");
            let sink = Arc::new(riir_instinct::decstat::DecStatSink::new());
            riir_instinct::decstat::install(Some(Arc::clone(&sink)));
            match riir_instinct::decstat::spawn_flusher(
                sink,
                key,
                riir_instinct::decstat::FlushConfig {
                    service_url: url.clone(),
                    machine,
                    toolchain,
                },
            ) {
                Ok(_) => eprintln!(
                    "[riir-instinct] decstat: on — flushing decision stats to {url} every {}s/{} decisions",

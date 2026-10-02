// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 460-484

        die("--top-k out of range (1..=32)");
    }
    // The focused-read filter (Plan 003's T5 frozen reads): run a subset
    // of the population; the A0 pin covers exactly what ran. Repeatable.
    let suites: Vec<&'static str> = if only_suites.is_empty() {
        SUITES.to_vec()
    } else {
        only_suites
            .iter()
            .map(|s| {
                SUITES
                    .iter()
                    .find(|x| **x == s.as_str())
                    .copied()
                    .unwrap_or_else(|| {
                        die(&format!(
                            "unknown suite {s} — the arena seats {SUITES:?}"
                        ))
                    })
            })
            .collect()
    };

    eprintln!("=== riir-instinct arena — the Reflex · instinct GOAT run ===");
    eprintln!(

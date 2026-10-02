//! P3 T1 live pin: load the real riir-train winner artifacts and score a
//! sample text through the serving forward — the cross-repo codec +
//! feature-law check the unit-test fixtures cannot carry (they pin the
//! FORMAT; this pins the BYTES the trainer actually shipped).
//!
//! ```text
//! cargo run --release --example load_winners -- \
//!     data/demo_specialists [ag_news,emotion,...]
//! ```
//!
//! Suite → winner arm per Bench 609: A on emotion/sst5/banking77,
//! B(gold_mix=0) on ag_news/xnli_en, B(gold_mix=0.5) on massive_intent_en
//! — the artifact format is arm-agnostic, so the loader does not care.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = args
        .first()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("data/demo_specialists"));
    let suites: Vec<String> = args
        .get(1)
        .map(|s| s.split(',').map(str::to_string).collect())
        .unwrap_or_else(|| {
            ["ag_news", "emotion", "sst5", "massive_intent_en", "banking77", "xnli_en"]
                .iter()
                .map(|s| (*s).to_string())
                .collect()
        });

    let samples: [(&str, &str); 3] = [
        ("ag_news", "Wall St. beats expectations as tech shares rally"),
        ("emotion", "i am so happy and grateful today"),
        ("banking77", "where is my card? it never arrived"),
    ];

    let mut failures = 0usize;
    for suite in &suites {
        // File + bag convention resolve through the winner bridge (Issue
        // 579) — the same resolution the arena and the serve lane use.
        let bridge = riir_instinct::specialist::winner_bridge(suite);
        let path = dir.join(
            bridge
                .file
                .map(str::to_string)
                .unwrap_or_else(|| format!("{suite}_winner_v1.bin")),
        );
        if !path.exists() {
            eprintln!("  {suite}: no winner artifact at {} — export first (riir-train instinct_arm_b)", path.display());
            failures += 1;
            continue;
        }
        match riir_instinct::specialist::load_artifact(&path) {
            Ok(m) => {
                let sample = samples
                    .iter()
                    .find(|(s, _)| s == suite)
                    .map(|(_, t)| *t)
                    .unwrap_or("sample text");
                let mut bag = Vec::new();
                let mut scratch = Vec::new();
                bridge
                    .convention
                    .bag_into(sample.as_bytes(), &mut bag, &mut scratch);
                let pick = m.pick(&bag);
                println!(
                    "  {suite}: labels {} · vocab-pinned · {}-bag · pick {pick} = {:?} for {sample:?}",
                    m.labels.len(),
                    bridge.convention.name(),
                    m.labels.get(pick).map(String::as_str)
                );
            }
            Err(e) => {
                eprintln!("  {suite}: LOAD FAILED — {e}");
                failures += 1;
            }
        }
    }
    if failures > 0 {
        eprintln!("load_winners: {failures} failure(s) — LOUD");
        std::process::exit(1);
    }
    println!("load_winners: PASSED — every artifact decoded through the serving reader");
}

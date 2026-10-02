// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 2461-2568

    )
    .expect("write registration");
    eprintln!("registration table: {}", rpath.display());
}

fn write_results(out_dir: &Path, runs: &[SuiteRun], box_state: &str) {
    // The record number comes from the OUT DIR the caller chose — a
    // hand-typed title here would have kept saying "Bench 001" when the
    // 052-protocol rerun landed in 002_ (it did).
    let bench_tag = out_dir
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| match n.split_once('_') {
            Some((tag, _)) if tag.chars().all(|c| c.is_ascii_digit()) && !tag.is_empty() => {
                format!("Bench {tag}")
            }
            _ => format!("Bench record ({n})"),
        })
        .unwrap_or_else(|| "Bench record".to_string());
    let mut md = String::with_capacity(16 << 10);
    md.push_str(&format!(
        "# {bench_tag} — the Reflex · instinct hybrid GOAT run\n\n"
    ));
    md.push_str("**Status:** MEASURED — the single frozen test read (Issue 005 T5 / \
Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + \
argmax Beta-LCB instrument; predictions frozen in `predictions.json`.\n\n");
    md.push_str(&format!(
        "Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's \
re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome \
off — oc and ridge arm only where their cal-slice selection clears the bar: oc on \
typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit \
through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin \
asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and \
— when the reflex-site checkout stands beside the workspace — that reflex's rows equal the \
PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — \
a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a \
crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ {BETA_GRID:?} × n_min ∈ {N_MIN_GRID:?} × \
τ ∈ {TAU_GRID:?} — 45 candidates, train-side only. Product gate (Issue 008 T2): the \
registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 \
on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 \
instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; \
A1/H1/H2 resolve each presented option to its specialist class row (by name for the \
suites whose keys are the label strings — massive/banking77 — by index under k == N for \
the fixed-criteria suites), and every hybrid pick is a position, directly comparable with \
gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case \
for seat-composing arms, `n_cases` disclosed)."
    ));
    md.push_str(&format!("Box state: {box_state}\n\n"));

    for run in runs {
        md.push_str(&format!("## {}\n\n", run.suite));
        md.push_str(&format!(
            "Verdict: **{}**{}.\n\n",
            run.verdict,
            run.a0_note
                .as_ref()
                .map(|n| format!(" — {n}"))
                .unwrap_or_default()
        ));
        md.push_str(&format!(
            "Posture: cap {} · head {:.2} · nb {:.2} · ridge {:.2} ({}) · fused-gate thresholds \\n{:.3}/{:.3} · specialist bag {}. Questions: {} over {} cases.{}\n\n",
            run.posture.effective_cap,
            run.posture.head_scale,
            run.posture.nb_scale,
            run.posture.ridge_scale,
            run.posture.nb_view,
            run.posture.score_threshold,
            run.posture.distance_threshold,
            run.posture.bag_convention,
            run.n_questions,
            run.n_cases,
            if !run.specialist_present {
                " A0/G0-only posture — no specialist artifact (Issue 010 T2)."
            } else {
                ""
            }
        ));
        if let Some(synth) = &run.posture.synth_corpus {
            md.push_str(&format!(
                "Synth corpus seated: {synth} (Plan 426 T5/T6 — blake3-verified at seat \\\nload; the posture is gold-fit, the corpus is the only difference).\n\n"
            ));
        }

        // The registration table (train/cal — the instrument's output).
        // The refused pick stays visible beside the served arm.
        md.push_str("### Pre-registration (cal front, train-side only)\n\n");
        md.push_str("| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |\n|---|---|---|---|---|---|\n");
        let pick = run.superiority.as_ref().map(|s| &s.pick);
        for r in &run.registration {
            if r.rank0 || r.cand == run.registered || pick.is_some_and(|p| *p == r.cand) {
                md.push_str(&format!(
                    "| {} | {} | {:.4} | {:.4} | {:.3} | {:.0} |\n",
                    if r.rank0 { "✓" } else { "" },
                    r.arm,
                    r.cal_acc,
                    r.acc_lcb,
                    r.consult,
                    r.p99_us
                ));
            }
        }
        md.push_str(&format!(
            "\nServed arm: **{}** (rank-0 {}/{} candidates; the full table rides \
`registration.json`).{}\n\n",
            run.registered.name(),
            run.registration.iter().filter(|r| r.rank0).count(),
            run.registration.len(),
            match &run.superiority {

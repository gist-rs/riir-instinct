// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 1430-1521

        .iter()
        .map(|&i| (cal_arms[i].correct.len() as u32) - cal_arms[i].n_correct())
        .collect();
    let win_pos = select_arm(&rank0, &successes, &failures);
    // select_arm returns the CANDIDATE index (the rank-0 element), never
    // a position within the rank-0 set — the massive run's rank0 [0, 2,
    // 16] made the old rank0_sorted[win_pos] re-index OOB (Issue 006's
    // position-vs-index class, one level up in the instrument). rank0 is
    // already in ascending candidate order (pareto_rank0 keeps index
    // order), so argmax ties resolve identically with or without the
    // sort that used to sit here.
    let instrument_pick = cands[win_pos].clone();
    eprintln!(
        "  instrument: rank-0 {}/{} · registered {}",
        rank0.len(),
        registration.len(),
        instrument_pick.name()
    );

    // ── THE TEST READ (once; predictions frozen below) ─────────────
    let mut test_arms: Vec<ArmOut> = Vec::new();
    {
        let (a0, test_se) = SuiteCtx::<N>::eval_a0(&mut ctx.engine, &seat.suite.cases, &seat.state_strs)?;
        let h1 = ctx.eval_h1(&test_se, &seat.suite.cases, &seat.state_strs);
        let (a1, h2s) = ctx.eval_a1_h2(&seat.suite.cases, &seat.state_strs);
        test_arms.push(a0);
        test_arms.push(a1);
        test_arms.push(h1);
        test_arms.extend(h2s);
    }
    let test_acc = |c: &Cand| -> f64 {
        test_arms
            .iter()
            .find(|a| a.name == c.name())
            .map(ArmOut::accuracy)
            .unwrap_or(f64::NAN)
    };
    eprintln!(
        "  test: A0 {:.4} · A1 {:.4} · H1 {:.4} · pick {} → {:.4}",
        test_acc(&Cand::A0),
        test_acc(&Cand::A1),
        test_acc(&Cand::H1),
        instrument_pick.name(),
        test_acc(&instrument_pick)
    );

    // ── the product gate (Issue 008 T2): STRICTLY above the current ────
    // Reflex row, or the registration refuses. Reflex is free; a tie (or
    // an edge the paired data cannot certify) sells nothing. The paired
    // superiority bound (stats::PairedDiff::lb95) reads the SAME frozen
    // test read the predictions freeze — no extra read is spent.
    let mut registered = instrument_pick.clone();
    let superiority = if instrument_pick != Cand::A0 {
        let face = (|| {
            let reg = test_arms.iter().find(|a| a.name == instrument_pick.name())?;
            let a0 = test_arms.iter().find(|a| a.name == "A0")?;
            paired_upper_bound(&reg.correct, &a0.correct)
        })();
        let Some(pd) = face else {
            die(&format!(
                "{}: the superiority gate could not pair the pick with A0 \
                 on the test read — the arm record is missing",
                name
            ));
        };
        let passed = pd.lb95 > 0.0;
        eprintln!(
            "  T2 superiority: {} − A0 mean {:+.4} · LB95 {:+.4} > 0 → {}",
            instrument_pick.name(),
            pd.mean,
            pd.lb95,
            if passed { "PASS" } else { "REFUSED → A0 serves" }
        );
        if !passed {
            registered = Cand::A0;
        }
        Some(SuperiorityFace {
            pick: instrument_pick.clone(),
            mean: pd.mean,
            lb95: pd.lb95,
            passed,
        })
    } else {
        None
    };

    // ── the fusion-overhead micro (G2's <100 ns bar) ─────────────
    let (h1_ns, h1_fusion_ns, h2_ns_opt) = fusion_overhead_micro(&mut ctx, N);
    eprintln!(
        "  fusion overhead: H1 full decision {:.0} ns/q (fusion-only {:.0} ns/q) · H2 fusion {:.2} ns/option",
        h1_ns, h1_fusion_ns, h2_ns_opt
    );

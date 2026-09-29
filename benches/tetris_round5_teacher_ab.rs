//! Issue 009 round 5 — the teacher A/B (the GO/NO-GO before any blended
//! dataset or retrain spend). Plain b1600 teacher vs critic-z-blended
//! b1600 teacher (w pre-registered 0.5), pre-registration recorded in
//! `.issues/009_instinct_arena_tetris_board.md` (verdicts R1 REVISE + R2
//! AGREE, committed at `7060deb` BEFORE any compute).
//!
//! Cells (the censoring rules are declared BEFORE any number is read):
//! - garbage 16@75, cap 5000 (b1600 saturates cap 1000 — Bench 007)
//! - garbage 18@75, cap 1000 (b1600 survived only 16/20 there — separates)
//! - garbage 20@80, cap 1000 (A/B-only hard cell)
//!
//! Seeds 401..=420 — deliberately disjoint from the T6/T3 eval set
//! (1..=20 + 607) and every training block: the A/B picks a teacher, and
//! picking it on the student's eval seeds would leak the choice into the
//! playoff.
//!
//! NO-OP GUARDS (read first, before any cell): blend-path value() calls
//! greater than 0 AND blended-vs-plain root-pick diffs greater than 0 —
//! else the blend never ran or never moved a pick and every cell is void.
//!
//! GO RULE (the pre-registration): blended−plain paired lb95 > 0 on ≥ 1
//! cell AND no cell with ub95 < 0 (the no-harm guard). A cell where BOTH
//! arms hit the cap on every seed is declared non-separating and its
//! bounds are not evidence either way.
//!
//! ```sh
//! cargo bench --bench tetris_round5_teacher_ab --features tetris_goat -- \
//!     --model <r4-weights.bin> [--w 0.5] [--seeds 401:420] \
//!     [--threads 10] [--out .benchmarks/025_tetris_round5_teacher_ab]
//! ```

use riir_instinct::stats::paired_upper_bound_f64;
use riir_instinct::tetris_critic::{LanePolicy, TrainedMlp, play_lane, teacher_probe_telemetry, teacher_probe_telemetry_reset};
use riir_instinct::tetris_lane::{
    CHAMPION_ID, Regime, blend_telemetry, blend_telemetry_reset,
};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

struct Args {
    model: Option<PathBuf>,
    w: f64,
    seeds: (u64, u64),
    budget: u32,
    threads: usize,
    out: PathBuf,
}

fn parse_args() -> Args {
    let mut a = Args {
        model: None,
        w: 0.5,
        seeds: (401, 420),
        budget: 1600,
        threads: 10,
        out: PathBuf::from(".benchmarks/025_tetris_round5_teacher_ab"),
    };
    let mut argv: Vec<String> = std::env::args().collect();
    while argv.last().is_some_and(|x| x == "--bench" || x == "tetris_round5_teacher_ab") {
        argv.pop();
    }
    let start = argv.iter().rposition(|x| x == "--").map(|p| p + 1).unwrap_or(1);
    let argv: Vec<String> = argv[start..].to_vec();
    let mut i = 0;
    while i < argv.len() {
        let val = || argv.get(i + 1).cloned().unwrap_or_default();
        let range = |s: &str| -> (u64, u64) {
            let (lo, hi) = s.split_once(':').expect("seed range lo:hi");
            (lo.parse().expect("lo"), hi.parse().expect("hi"))
        };
        match argv[i].as_str() {
            "--model" => a.model = Some(PathBuf::from(val())),
            "--w" => a.w = val().parse().expect("--w f64"),
            "--seeds" => a.seeds = range(&val()),
            "--teacher-budget" => a.budget = val().parse().expect("--teacher-budget u32"),
            "--threads" => a.threads = val().parse().expect("--threads usize"),
            "--out" => a.out = PathBuf::from(val()),
            other => panic!("unknown arg {other:?}"),
        }
        i += 2;
    }
    a
}

/// One A/B cell: (regime, cap) — the pre-registered three.
const CELLS: [(Regime, usize); 3] = [
    (Regime::garbage(16, 75), 5000),
    (Regime::garbage(18, 75), 1000),
    (Regime::garbage(20, 80), 1000),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    Plain,
    Blended,
}

/// All seeds' games for one (cell, arm), threaded over seeds.
#[allow(clippy::too_many_arguments)] // mirrors the cell/arm/threaded runner shape
fn run_cell(
    genome: &katgpt_tetris::rulebook::Genome,
    arm: Arm,
    model: &TrainedMlp,
    w: f64,
    budget: u32,
    regime: Regime,
    cap: usize,
    seeds: &[u64],
    threads: usize,
) -> Vec<(u64, riir_instinct::tetris_lane::GameStats)> {
    let next_seed = AtomicUsize::new(0);
    let results: Mutex<Vec<(u64, riir_instinct::tetris_lane::GameStats)>> =
        Mutex::new(Vec::with_capacity(seeds.len()));
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let k = next_seed.fetch_add(1, Ordering::Relaxed);
                if k >= seeds.len() {
                    break;
                }
                let seed = seeds[k];
                let st = match arm {
                    Arm::Plain => {
                        let mut sink = Vec::new();
                        let mut policy = LanePolicy::Teacher { budget, sink: &mut sink };
                        play_lane(genome, seed, regime, cap, &mut policy)
                    }
                    Arm::Blended => {
                        let mut sink = Vec::new();
                        let mut policy =
                            LanePolicy::TeacherBlended { budget, model, w, sink: &mut sink };
                        play_lane(genome, seed, regime, cap, &mut policy)
                    }
                };
                results.lock().expect("results lock").push((seed, st));
            });
        }
    });
    let mut out = results.into_inner().expect("results");
    out.sort_by_key(|(seed, _)| *seed);
    out
}

fn summarize(res: &[(u64, riir_instinct::tetris_lane::GameStats)], cap: usize) -> Value {
    let n = res.len().max(1);
    let pieces: f64 = res.iter().map(|(_, s)| s.pieces as f64).sum::<f64>() / n as f64;
    let lines: f64 = res.iter().map(|(_, s)| s.lines as f64).sum::<f64>() / n as f64;
    let points: f64 = res.iter().map(|(_, s)| s.points as f64).sum::<f64>() / n as f64;
    let survived = res.iter().filter(|(_, s)| !s.topped_out).count();
    let capped = res.iter().filter(|(_, s)| s.pieces >= cap).count();
    let mut durs: Vec<f64> = res.iter().map(|(_, s)| s.p50_ms).collect();
    durs.sort_by(f64::total_cmp);
    json!({
        "mean_pieces": pieces,
        "mean_lines": lines,
        "mean_points": points,
        "survived": survived,
        "cap_hits": capped,
        "of": res.len(),
        "p50_decision_ms": durs.get(durs.len() / 2).copied().unwrap_or(0.0),
    })
}

/// Paired (blended − plain) pieces over same-seed games.
fn paired(res: &[(u64, riir_instinct::tetris_lane::GameStats)], base: &[(u64, riir_instinct::tetris_lane::GameStats)]) -> Value {
    let diffs: Vec<f64> = res
        .iter()
        .zip(base)
        .filter(|((s, _), (b, _))| s == b)
        .map(|((_, s), (_, b))| s.pieces as f64 - b.pieces as f64)
        .collect();
    let mut w = 0usize;
    let (mut t, mut l) = (0usize, 0usize);
    for d in &diffs {
        match d.total_cmp(&0.0) {
            std::cmp::Ordering::Greater => w += 1,
            std::cmp::Ordering::Equal => t += 1,
            std::cmp::Ordering::Less => l += 1,
        }
    }
    let (mean, lb95, ub95) = match paired_upper_bound_f64(&diffs) {
        Some(d) => (d.mean, d.lb95, d.ub95),
        None => (0.0, f64::NEG_INFINITY, f64::INFINITY),
    };
    json!({
        "n_paired": diffs.len(),
        "paired_mean": mean,
        "paired_lb95": lb95,
        "paired_ub95": ub95,
        "wtl": format!("{w}/{t}/{l}"),
    })
}

fn iso_now() -> String {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock");
    let secs = d.as_secs();
    let days = secs / 86400;
    let (y, m, dd) = {
        // civil-from-days (Howard Hinnant's algorithm)
        let z = days as i64 + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y, m, d)
    };
    let tod = secs % 86400;
    format!("{y:04}-{m:02}-{dd:02}T{:02}:{:02}:{:02}Z", tod / 3600, (tod % 3600) / 60, tod % 60)
}

fn main() {
    let args = parse_args();
    let model_path = args.model.clone().expect("--model <r4-weights.bin> is required (the blend component)");
    let raw = std::fs::read(&model_path).expect("read --model weights file");
    let model = TrainedMlp::from_bytes(&raw)
        .unwrap_or_else(|e| panic!("parse {}: {e}", model_path.display()));
    let model_digest = TrainedMlp::digest_hex(&raw);
    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    assert_eq!(genome.id(), CHAMPION_ID, "champion genome drifted from the pinned digest");
    let seeds: Vec<u64> = (args.seeds.0..=args.seeds.1).collect();
    assert!(!seeds.iter().any(|s| (1..=40).contains(s) || (101..=140).contains(s)
        || (201..=300).contains(s) || *s == 607 || (1..=20).contains(s)),
        "A/B seeds must stay disjoint from the training and eval blocks (pre-registration)");

    println!(
        "== tetris_round5_teacher_ab — plain b{b} vs critic z-blend (w={w}, model {d}) b{b} ==",
        b = args.budget,
        w = args.w,
        d = model_digest
    );
    println!(
        "seeds {}..={} · champion {CHAMPION_ID} · model dims {:?} · threads {}",
        args.seeds.0,
        args.seeds.1,
        model.dims(),
        args.threads
    );
    blend_telemetry_reset();
    teacher_probe_telemetry_reset();

    let started = Instant::now();
    let mut cells_json = Vec::<Value>::new();
    let (mut go_cells, mut harm_cells, mut nonsep_cells) = (0usize, 0usize, 0usize);
    for &(regime, cap) in &CELLS {
        let t0 = Instant::now();
        let plain = run_cell(&genome, Arm::Plain, &model, args.w, args.budget, regime, cap, &seeds, args.threads);
        let blended = run_cell(&genome, Arm::Blended, &model, args.w, args.budget, regime, cap, &seeds, args.threads);
        let cell_s = t0.elapsed().as_secs_f64();
        let ps = summarize(&plain, cap);
        let bs = summarize(&blended, cap);
        let paired = paired(&blended, &plain);
        // Censoring rule (declared up front): both arms all-cap ⇒ the cell
        // cannot separate the teachers, and its bounds are not evidence.
        let both_all_cap = ps["cap_hits"].as_u64().unwrap_or(0) == seeds.len() as u64
            && bs["cap_hits"].as_u64().unwrap_or(0) == seeds.len() as u64;
        let lb = paired["paired_lb95"].as_f64().unwrap_or(f64::NEG_INFINITY);
        let ub = paired["paired_ub95"].as_f64().unwrap_or(f64::INFINITY);
        if both_all_cap {
            nonsep_cells += 1;
        } else {
            if lb > 0.0 {
                go_cells += 1;
            }
            if ub < 0.0 {
                harm_cells += 1;
            }
        }
        let pp = ps["mean_pieces"].as_f64().unwrap_or(0.0);
        let bp = bs["mean_pieces"].as_f64().unwrap_or(0.0);
        let dm = paired["paired_mean"].as_f64().unwrap_or(0.0);
        let wtl = paired["wtl"].as_str().unwrap_or("?");
        let plain_caps = &ps["cap_hits"];
        let blended_caps = &bs["cap_hits"];
        let nonsep_note = if both_all_cap { " · NON-SEPARATING (both all-cap)" } else { "" };
        println!(
            "{regime} cap {cap}: plain {pp:.1} pieces (cap {plain_caps}) · blended {bp:.1} (cap {blended_caps}) · Δmean {dm:+.1} lb95 {lb:+.1} ub95 {ub:+.1} wtl {wtl} · {cell_s:.1}s{nonsep_note}"
        );
        cells_json.push(json!({
            "regime": regime.to_string(),
            "cap": cap,
            "plain": ps,
            "blended": bs,
            "paired_blended_minus_plain": paired,
            "non_separating_both_all_cap": both_all_cap,
            "wall_s": cell_s,
        }));
    }

    // ── The no-op guards (verdict R1) — read BEFORE the cells mean anything.
    let blend_calls = blend_telemetry();
    let (picks, diffs, probe_micros) = teacher_probe_telemetry();
    let (guards_ok, guard_note) = if blend_calls == 0 {
        (false, "blend path NEVER executed — every cell is void (build/no-op defect)")
    } else if picks == 0 || diffs == 0 {
        (false, "blend never moved a root pick — the A/B would measure nothing")
    } else {
        (true, "blend executed and moved picks — guards pass")
    };
    println!(
        "guards: blend-path value() calls {blend_calls} · probe picks {picks} · pick diffs {diffs} · probe wall {:.1}s — {}",
        probe_micros as f64 / 1e6,
        guard_note
    );

    // The pre-registered GO rule.
    let go = guards_ok && go_cells >= 1 && harm_cells == 0;
    println!(
        "GO RULE: lb95>0 on {go_cells} cell(s), ub95<0 on {harm_cells}, non-separating {nonsep_cells} ⇒ {}",
        if go { "GO — proceed to the blended dataset + r5 train" } else { "NO-GO — record the negative and stop" }
    );

    let record = json!({
        "meta": {
            "issue": "riir-instinct 009 round 5 teacher A/B (pre-registration committed 7060deb BEFORE compute)",
            "recorded_at": iso_now(),
            "box_load": {
                "avg1": std::fs::read_to_string("/proc/loadavg").ok(),
            },
            "champion_genome_id": CHAMPION_ID,
            "tetris_head_unused": true,
            "model_path": model_path.display().to_string(),
            "model_blake3_16": model_digest,
            "model_dims": { "h1": model.dims().0, "h2": model.dims().1, "tanh_hidden": model.dims().2 },
            "w": args.w,
            "budget": args.budget,
            "seeds": format!("{}..={}", args.seeds.0, args.seeds.1),
            "teacher_information_rule": "no preview, fresh bag (uniform-7 chance); mode at the decision root",
            "cells": "16@75 cap 5000; 18@75 cap 1000; 20@80 cap 1000 (A/B-only)",
            "censoring_rule": "both arms all-cap on every seed ⇒ non-separating, bounds not evidence",
            "go_rule": "blend guards pass AND lb95>0 on >=1 cell AND no cell ub95<0",
            "no_op_guards": {
                "blend_value_calls": blend_calls,
                "probe_picks": picks,
                "probe_pick_diffs": diffs,
                "probe_wall_s": probe_micros as f64 / 1e6,
                "note": "probe wall is DISCLOSED SEPARATELY from per-spot latency (verdict R2); blended p50_decision_ms includes the probe (~5 ms/spot of the ~180 ms) — subtract conceptually, never compare to the plain arm's p50 as a latency claim",
            },
            "threads": args.threads,
            "wall_s": started.elapsed().as_secs_f64(),
        },
        "guards_ok": guards_ok,
        "go": go,
        "go_cells": go_cells,
        "harm_cells": harm_cells,
        "non_separating_cells": nonsep_cells,
        "cells": cells_json,
    });
    std::fs::create_dir_all(&args.out).expect("create out dir");
    let path = args.out.join("teacher_ab.json");
    std::fs::write(&path, serde_json::to_string_pretty(&record).expect("serialize"))
        .expect("write record");
    println!("wrote {}", path.display());
    if !guards_ok {
        std::process::exit(2);
    }
}

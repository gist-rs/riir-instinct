//! tetris_teacher_check — instinct Issue 009 T6 (serving-matched teacher
//! check) + T5 (the serving-input-contract measurement), one harness run.
//!
//! The question T6 answers CHEAPLY, before any GPU time: can the teacher
//! (`katgpt_core::chance_puct` over the rulebook-champion evaluator, no
//! preview, fresh bag — exactly what serving sees) STRICTLY beat free
//! Reflex's board on held-out seeds in a separating regime? A teacher that
//! cannot win under serving information makes T7 (expert iteration)
//! pointless.
//!
//! The question T5 answers: does the served 5-class input carry enough for
//! a richer decoder, or is it a hard ceiling (the wire-widening trigger)?
//!
//! Run:
//! ```sh
//! cargo bench --test tetris_teacher_check --features tetris_goat -- \
//!     [--paired-seeds 20] [--regimes 16:75,18:75] [--cap 1000] \
//!     [--budgets 0,100,400,1600] [--threads 8] [--out .benchmarks/007_tetris_teacher_check]
//! ```
//! Seed 607 always runs beside the paired set (context, excluded from the
//! paired gate), on every regime, plus one empty-board context cell
//! (cap 300, reflex + champion + b400).

use riir_instinct::stats::paired_upper_bound_f64;
use riir_instinct::tetris_lane::{
    ArmKind, CHAMPION_ID, Fixtures, Regime, Runner, T5Sink,
};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const CONTEXT_SEED: u64 = 607;

struct Args {
    paired_seeds: usize,
    regimes: Vec<Regime>,
    cap: usize,
    budgets: Vec<u32>,
    threads: usize,
    out: PathBuf,
}

fn parse_args() -> Args {
    let mut a = Args {
        paired_seeds: 20,
        regimes: vec![Regime::garbage(16, 75), Regime::garbage(18, 75)],
        cap: 1000,
        budgets: vec![0, 100, 400, 1600],
        threads: 8,
        out: PathBuf::from(".benchmarks/007_tetris_teacher_check"),
    };
    // cargo bench forwards our args but APPENDS its own target-selection
    // echo (a trailing `--bench` [+ target name]) with no `--` separator;
    // drop that tail, parse the rest, and still panic on anything else.
    let mut argv: Vec<String> = std::env::args().collect();
    while argv
        .last()
        .is_some_and(|a| a == "--bench" || a == "tetris_teacher_check")
    {
        argv.pop();
    }
    let start = argv.iter().rposition(|a| a == "--").map(|p| p + 1).unwrap_or(1);
    let argv: Vec<String> = argv[start..].to_vec();
    let mut i = 0;
    while i < argv.len() {
        let val = || argv.get(i + 1).cloned().unwrap_or_default();
        match argv[i].as_str() {
            "--paired-seeds" => a.paired_seeds = val().parse().expect("--paired-seeds usize"),
            "--cap" => a.cap = val().parse().expect("--cap usize"),
            "--threads" => a.threads = val().parse().expect("--threads usize"),
            "--out" => a.out = PathBuf::from(val()),
            "--regimes" => {
                a.regimes = val()
                    .split(',')
                    .map(|s| {
                        let (r, f) = s.split_once(':').expect("regime rows:fill");
                        Regime::garbage(r.parse().expect("rows"), f.parse().expect("fill"))
                    })
                    .collect();
            }
            "--budgets" => {
                a.budgets = val()
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();
            }
            other => panic!("unknown arg {other:?}"),
        }
        i += 2;
    }
    a
}

fn iso_now() -> String {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock");
    let secs = d.as_secs() as i64;
    let (days, sod) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + 2 * yoe / 1460 + (4 * yoe + 3) / 146_096) / 365;
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        sod / 3600,
        sod % 3600 / 60,
        sod % 60,
        d.subsec_millis()
    )
}

fn box_load() -> String {
    std::process::Command::new("uptime")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            s.split("load average")
                .nth(1)
                .map(|t| t.trim_start_matches(['s', ':', ' ']).trim().to_string())
        })
        .unwrap_or_else(|| "unknown".into())
}

fn load_fixtures() -> Fixtures {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir = std::env::var("TETRIS_LANE_FIXTURE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            manifest
                .parent()
                .expect("manifest has a parent")
                .join("riir-reflex/assets/game_heads")
        });
    let read = |name: &str| -> String {
        std::fs::read_to_string(dir.join(name))
            .unwrap_or_else(|e| panic!("read {name} from {}: {e}", dir.display()))
    };
    let fixtures = Fixtures {
        tetris: read("tetris_oracle_laya_en_v3.jsonl"),
        lanes: read("lanes_oracle_laya_en_v1.jsonl"),
        flappy: read("flappy_oracle_laya_en_v3.jsonl"),
    };
    // The fixture pins are the contract — a drifted copy is not the head
    // the arena serves.
    let pins = riir_reflex::game_heads::fixture_pins(
        &fixtures.tetris,
        &fixtures.lanes,
        &fixtures.flappy,
    );
    for (name, got, pin) in pins {
        assert_eq!(got, pin, "{name}: fixture blake3 != pinned blake3");
    }
    fixtures
}

/// Arms under test, in table order (reflex first — the paired baseline).
fn arms(budgets: &[u32]) -> Vec<ArmKind> {
    let mut arms = vec![ArmKind::ReflexHead];
    arms.extend(budgets.iter().map(|&b| ArmKind::TeacherPuct { budget: b }));
    arms.push(ArmKind::ChampionSearch);
    arms
}

/// All seeds' games for one (arm, regime) cell, threaded over seeds.
/// `paired_only = false` adds the context seed 607 to the returned map.
fn run_cell(
    kind: ArmKind,
    regime: Regime,
    seeds: &[u64],
    cap: usize,
    threads: usize,
    fixtures: &Fixtures,
    collect_t5: bool,
) -> Vec<(u64, riir_instinct::tetris_lane::GameStats, Option<riir_instinct::tetris_lane::T5Aggregate>)> {
    let next_seed = AtomicUsize::new(0);
    let results: std::sync::Mutex<
        Vec<(u64, riir_instinct::tetris_lane::GameStats, Option<riir_instinct::tetris_lane::T5Aggregate>)>,
    > = std::sync::Mutex::new(Vec::with_capacity(seeds.len()));
    let fixture_ref = fixtures;
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
                let mut runner = Runner::new(kind, fixture_ref, genome);
                let mut sink = T5Sink::new(matches!(kind, ArmKind::TeacherPuct { .. }));
                loop {
                    let k = next_seed.fetch_add(1, Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let seed = seeds[k];
                    let (stats, agg) = if collect_t5 {
                        let cell = Some(&mut sink);
                        let stats = runner.play(seed, regime, cap, cell);
                        (stats, Some(sink.agg().clone()))
                    } else {
                        let stats = runner.play(seed, regime, cap, None);
                        (stats, None)
                    };
                    results.lock().expect("results lock").push((seed, stats, agg));
                }
            });
        }
    });
    let mut out = results.into_inner().expect("results");
    out.sort_by_key(|(seed, _, _)| *seed);
    out
}

fn main() {
    let a = parse_args();
    let started = Instant::now();

    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    assert_eq!(genome.id(), CHAMPION_ID, "champion genome drifted from the pinned digest");

    let fixtures = load_fixtures();
    // The served head's determinism anchor, from one runner.
    let head_digest = Runner::new(ArmKind::ReflexHead, &fixtures, genome).head_digest();

    let paired_seeds: Vec<u64> = (1..=a.paired_seeds as u64).collect();
    let all_seeds = || -> Vec<u64> {
        let mut v = paired_seeds.clone();
        v.push(CONTEXT_SEED);
        v
    };

    println!(
        "== tetris_teacher_check — Issue 009 T6 (teacher vs free Reflex, serving-matched) + T5 =="
    );
    println!(
        "paired seeds 1..={}, context seed {CONTEXT_SEED} beside · regimes {:?} · cap {} · budgets {:?} · {} workers",
        a.paired_seeds, a.regimes, a.cap, a.budgets, a.threads
    );
    println!(
        "teacher info rule: NO PREVIEW, FRESH BAG (uniform-7 chance) · champion eval · genome {CHAMPION_ID} · head digest {head_digest}"
    );
    println!("box: load {}", box_load());

    let arm_list = arms(&a.budgets);

    // One summary row per (regime, arm).
    #[derive(Clone)]
    struct Cell {
        regime: Regime,
        kind: ArmKind,
        per_seed: Vec<(u64, Value)>,
        mean_pieces: f64,
        mean_lines: f64,
        mean_points: f64,
        survived: usize,
        p50_ms: f64,
        wtl: (usize, usize, usize),
        paired_mean: f64,
        paired_lb95: f64,
    }

    let mut cells: Vec<Cell> = Vec::new();
    let mut t5_cells: Vec<(String, String, Value)> = Vec::new();

    for &regime in &a.regimes {
        let seeds = all_seeds();
        let games = a.paired_seeds; // summary columns are paired-seeds only
        // Baseline first: the reflex head's per-seed pieces.
        let base = run_cell(ArmKind::ReflexHead, regime, &seeds, a.cap, a.threads, &fixtures, true);
        let base_pieces: Vec<f64> = base.iter().map(|(_, s, _)| s.pieces as f64).collect();

        // T5 from the reflex source.
        let mut reflex_agg = riir_instinct::tetris_lane::T5Aggregate::default();
        for (_, _, agg) in &base {
            if let Some(agg) = agg {
                reflex_agg.merge(agg);
            }
        }
        t5_cells.push((
            "reflex_head".into(),
            regime.to_string(),
            serde_json::to_value(&reflex_agg).expect("t5 reflex agg"),
        ));

        for &kind in &arm_list {
            if kind == ArmKind::ReflexHead {
                continue;
            }
            let collect = matches!(kind, ArmKind::TeacherPuct { budget: 400 });
            let res = run_cell(kind, regime, &seeds, a.cap, a.threads, &fixtures, collect);
            if collect {
                let mut agg = riir_instinct::tetris_lane::T5Aggregate::default();
                for (_, _, a) in &res {
                    if let Some(a) = a {
                        agg.merge(a);
                    }
                }
                t5_cells.push((
                    "teacher_b400".into(),
                    regime.to_string(),
                    serde_json::to_value(&agg).expect("t5 teacher agg"),
                ));
            }
            // Every summary column is PAIRED-SEEDS ONLY (1..=N); the
            // context seed 607 rides in the JSON's per_seed rows and the
            // separate context cells, never inside a gate column.
            let paired: Vec<&(u64, riir_instinct::tetris_lane::GameStats, Option<riir_instinct::tetris_lane::T5Aggregate>)> =
                res.iter().filter(|(s, _, _)| (*s as usize) <= a.paired_seeds).collect();
            let pieces: Vec<f64> = paired.iter().map(|(_, s, _)| s.pieces as f64).collect();
            let lines: f64 = paired.iter().map(|(_, s, _)| s.lines as f64).sum::<f64>() / paired.len() as f64;
            let points: f64 = paired.iter().map(|(_, s, _)| s.points as f64).sum::<f64>() / paired.len() as f64;
            let survived = paired.iter().filter(|(_, s, _)| s.pieces == a.cap).count();
            let mut durs: Vec<f64> = paired.iter().map(|(_, s, _)| s.p50_ms).collect();
            durs.sort_by(f64::total_cmp);
            let p50 = durs.get(durs.len() / 2).copied().unwrap_or(0.0);
            let mut w = 0;
            let (mut tie, mut l) = (0usize, 0usize);
            let mut diffs: Vec<f64> = Vec::new();
            for ((_, s, _), b) in paired.iter().zip(&base_pieces) {
                diffs.push(s.pieces as f64 - b);
                match (s.pieces as f64).total_cmp(b) {
                    std::cmp::Ordering::Greater => w += 1,
                    std::cmp::Ordering::Equal => tie += 1,
                    std::cmp::Ordering::Less => l += 1,
                }
            }
            let (pm, plb) = match paired_upper_bound_f64(&diffs) {
                Some(d) => (d.mean, d.lb95),
                None => (0.0, f64::NEG_INFINITY),
            };
            println!(
                "{:<64} regime {regime}: pieces {:>7.1} lines {:>7.1} pts {:>9.0} surv {survived:>2}/{games} p50 {:>7.2}ms W/T/L {w}/{tie}/{l} Δ{pm:>+7.1} lb95 {plb:>+7.1}",
                kind.name(), pieces.iter().sum::<f64>() / pieces.len() as f64,
                lines, points, p50
            );
            cells.push(Cell {
                regime,
                kind,
                per_seed: res
                    .iter()
                    .map(|(seed, s, _)| {
                        (*seed, serde_json::to_value(s).expect("stats json"))
                    })
                    .collect(),
                mean_pieces: pieces.iter().sum::<f64>() / pieces.len() as f64,
                mean_lines: lines,
                mean_points: points,
                survived,
                p50_ms: p50,
                wtl: (w, tie, l),
                paired_mean: pm,
                paired_lb95: plb,
            });
        }

        // The baseline row itself, for the record (paired seeds only,
        // like every gate column above).
        let paired_base: Vec<&(u64, riir_instinct::tetris_lane::GameStats, Option<riir_instinct::tetris_lane::T5Aggregate>)> =
            base.iter().filter(|(s, _, _)| (*s as usize) <= a.paired_seeds).collect();
        let pieces: Vec<f64> = paired_base.iter().map(|(_, s, _)| s.pieces as f64).collect();
        let lines: f64 = paired_base.iter().map(|(_, s, _)| s.lines as f64).sum::<f64>() / paired_base.len() as f64;
        let points: f64 = paired_base.iter().map(|(_, s, _)| s.points as f64).sum::<f64>() / paired_base.len() as f64;
        let survived = paired_base.iter().filter(|(_, s, _)| s.pieces == a.cap).count();
        let mut durs: Vec<f64> = paired_base.iter().map(|(_, s, _)| s.p50_ms).collect();
        durs.sort_by(f64::total_cmp);
        println!(
            "{:<64} regime {regime}: pieces {:>7.1} lines {:>7.1} pts {:>9.0} surv {survived:>2}/{games} p50 {:>7.2}ms (baseline)",
            ArmKind::ReflexHead.name(),
            pieces.iter().sum::<f64>() / pieces.len() as f64,
            lines, points, durs.get(durs.len() / 2).copied().unwrap_or(0.0)
        );
        cells.push(Cell {
            regime,
            kind: ArmKind::ReflexHead,
            per_seed: base
                .iter()
                .map(|(seed, s, _)| (*seed, serde_json::to_value(s).expect("stats json")))
                .collect(),
            mean_pieces: pieces.iter().sum::<f64>() / pieces.len() as f64,
            mean_lines: lines,
            mean_points: points,
            survived,
            p50_ms: durs.get(durs.len() / 2).copied().unwrap_or(0.0),
            wtl: (0, 0, 0),
            paired_mean: 0.0,
            paired_lb95: 0.0,
        });
    }

    // The empty-board context cell (seed 607 beside the live arena's shape).
    let ctx_regime = Regime::EMPTY;
    let ctx_seeds = vec![CONTEXT_SEED];
    let mut ctx = serde_json::Map::new();
    for kind in [
        ArmKind::ReflexHead,
        ArmKind::TeacherPuct { budget: 400 },
        ArmKind::ChampionSearch,
    ] {
        let res = run_cell(kind, ctx_regime, &ctx_seeds, 300, 1, &fixtures, false);
        if let Some((_, s, _)) = res.first() {
            println!(
                "context {ctx_regime} seed {CONTEXT_SEED} cap 300: {} pieces {} lines {} pts {} topped_out {}",
                kind.slug(),
                s.pieces,
                s.lines,
                s.points,
                s.topped_out
            );
            ctx.insert(kind.slug(), serde_json::to_value(s).expect("ctx stats"));
        }
    }

    let doc = json!({
        "meta": {
            "issue": "riir-instinct 009 T6 (+T5 collection)",
            "recorded_at": iso_now(),
            "box_load": box_load(),
            "champion_genome_id": CHAMPION_ID,
            "tetris_head_digest": head_digest,
            "teacher_information_rule": "no preview, fresh bag (uniform-7 chance model); chance_puct over the champion evaluator (mode at the decision root); teacher rng = seed ^ 0x05EE_D892",
            "paired_seeds": format!("1..={}", a.paired_seeds),
            "context_seed": CONTEXT_SEED,
            "regimes": a.regimes.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
            "cap": a.cap,
            "budgets": a.budgets,
            "threads": a.threads,
            "stream": "katgpt_tetris::lookahead::Bag (seeded guideline 7-bag; harness-internal — the arena's JS PieceBag pops the same shuffle from the other end, so harness seeds are not JS-walk seeds)",
            "gate": "T6 PASS iff some teacher budget's paired pieces diff vs reflex-head has lb95 > 0",
            "wall_s": started.elapsed().as_secs_f64(),
        },
        "cells": cells.iter().map(|c| json!({
            "regime": c.regime.to_string(),
            "arm": c.kind.slug(),
            "arm_name": c.kind.name(),
            "mean_pieces": c.mean_pieces,
            "mean_lines": c.mean_lines,
            "mean_points": c.mean_points,
            "survived": c.survived,
            "p50_ms": c.p50_ms,
            "wtl_vs_reflex": format!("{}/{}/{}", c.wtl.0, c.wtl.1, c.wtl.2),
            "paired_mean": c.paired_mean,
            "paired_lb95": c.paired_lb95,
            "per_seed": c.per_seed,
        })).collect::<Vec<_>>(),
        "t5_input_contract": t5_cells.iter().map(|(src, regime, agg)| json!({
            "source": src,
            "regime": regime,
            "aggregate": agg,
        })).collect::<Vec<_>>(),
        "context_empty_seed607_cap300": ctx,
    });

    std::fs::create_dir_all(&a.out).expect("create out dir");
    let path = a.out.join("tetris_teacher_check.json");
    std::fs::write(&path, serde_json::to_string_pretty(&doc).expect("serialize"))
        .expect("write results");
    println!("wrote {}", path.display());

    // The T6 verdict line.
    let best = cells
        .iter()
        .filter(|c| matches!(c.kind, ArmKind::TeacherPuct { .. }))
        .max_by(|a, b| a.paired_lb95.total_cmp(&b.paired_lb95));
    match best {
        Some(c) if c.paired_lb95 > 0.0 => println!(
            "T6 VERDICT: PASS — {} regime {}: paired Δ{:+.1} pieces, lb95 {:+.1} > 0 (W/T/L {}/{}/{}) — the teacher beats free Reflex under serving information; T7 is unblocked.",
            c.kind.name(),
            c.regime,
            c.paired_mean,
            c.paired_lb95,
            c.wtl.0,
            c.wtl.1,
            c.wtl.2
        ),
        Some(c) => println!(
            "T6 VERDICT: FAIL — best teacher budget {} regime {}: paired Δ{:+.1}, lb95 {:+.1} ≤ 0 — no promotable teacher posture; T7 stays blocked.",
            c.kind.name(),
            c.regime,
            c.paired_mean,
            c.paired_lb95
        ),
        None => println!("T6 VERDICT: FAIL — no teacher budget produced a row."),
    }
}

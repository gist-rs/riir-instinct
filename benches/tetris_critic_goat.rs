//! tetris_critic_goat — instinct Issue 009 T8's MODELLESS second arm: the
//! closed-form KARC basis-ridge value critic, fit on the T6 teacher's
//! search-root Q, played against the T8 bar.
//!
//! The question this answers CHEAPLY, before any expert-iteration (T7)
//! spend: can a closed-form basis-ridge fit of the teacher's Q over the
//! widened serving features already beat
//! (a) free Reflex's board AND
//! (b) the T8 baseline b0 — the 1-ply champion evaluator (budget-0 PUCT
//!     prior, the exact shape the issue's verdict amendment pins)?
//! If (b) holds, the modelless path wins and that is the honest verdict
//! under the modelless-first mandate; if not, the trained critic's bar is
//! BOTH rows and T7 is justified with this infrastructure in hand.
//!
//! Phases: collect (teacher b400 on training seeds 201..=300, disjoint
//! from 006's 1..=40/101..=140, this bench's eval 1..=20, and 607) → fit
//! (train 201..=280, λ swept on val 281..=300 by top-1 agreement with the
//! teacher pick, final refit on train+val at the chosen λ) → eval (the T3
//! protocol: garbage 16@75 + 18@75, cap 1000, paired seeds 1..=20, seed
//! 607 beside, excluded from every gate column).
//!
//! Run:
//! ```sh
//! cargo bench --test tetris_critic_goat --features tetris_goat -- \
//!     [--train-seeds 201:280] [--val-seeds 281:300] [--teacher-budget 400] \
//!     [--regimes 16:75,18:75] [--cap 1000] [--threads 6] \
//!     [--lambdas 0.01,0.03,0.1,0.3,1,3,10] [--out .benchmarks/009_tetris_critic_arm]
//! ```

use riir_instinct::stats::paired_upper_bound_f64;
use riir_instinct::tetris_critic::{
    AgreeStats, CriticModel, FeatureScale, FitAccumulator, LanePolicy, Sample, agreement_stats,
    decision_groups, lane_context, play_lane,
};
use riir_instinct::tetris_lane::{ArmKind, CHAMPION_ID, Fixtures, GameStats, Regime, Runner};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const CONTEXT_SEED: u64 = 607;
const PAIRED_SEEDS: usize = 20;

struct Args {
    train: (u64, u64),
    val: (u64, u64),
    teacher_budget: u32,
    regimes: Vec<Regime>,
    cap: usize,
    threads: usize,
    lambdas: Vec<f64>,
    out: PathBuf,
}

fn parse_args() -> Args {
    let mut a = Args {
        train: (201, 280),
        val: (281, 300),
        teacher_budget: 400,
        regimes: vec![Regime::garbage(16, 75), Regime::garbage(18, 75)],
        cap: 1000,
        threads: 6,
        lambdas: vec![0.01, 0.03, 0.1, 0.3, 1.0, 3.0, 10.0],
        out: PathBuf::from(".benchmarks/009_tetris_critic_arm"),
    };
    // cargo bench forwards our args but APPENDS its own target-selection
    // echo (a trailing `--bench` [+ target name]) with no `--` separator;
    // drop that tail, parse the rest, and still panic on anything else
    // (the 007 bench's convention).
    let mut argv: Vec<String> = std::env::args().collect();
    while argv
        .last()
        .is_some_and(|x| x == "--bench" || x == "tetris_critic_goat")
    {
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
            "--train-seeds" => a.train = range(&val()),
            "--val-seeds" => a.val = range(&val()),
            "--teacher-budget" => a.teacher_budget = val().parse().expect("--teacher-budget u32"),
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
            "--lambdas" => {
                a.lambdas = val()
                    .split(',')
                    .map(|s| s.parse().expect("lambda f64"))
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
    // the arena serves (the 007 bench's convention).
    let pins =
        riir_reflex::game_heads::fixture_pins(&fixtures.tetris, &fixtures.lanes, &fixtures.flappy);
    for (name, got, pin) in pins {
        assert_eq!(got, pin, "{name}: fixture blake3 != pinned blake3");
    }
    fixtures
}

/// One (regime, split) sample block: the samples and their contiguous
/// (seed, decision) groups stay together — indices are only valid against
/// the block's own list (regimes share seeds; a global index space would
/// silently merge two regimes' decisions into one group). Block ORDER is
/// the regime order (the collection rows carry the names).
struct Block {
    samples: Vec<Sample>,
    groups: Vec<(usize, usize)>,
}

/// Teacher collection over one (regime, seed list), threaded across seeds;
/// per-seed sample blocks merge in seed order (the fit's determinism
/// premise: the Gram then accumulates in (regime, seed, decision, option)
/// order).
fn collect(
    genome: &katgpt_tetris::rulebook::Genome,
    regime: Regime,
    seeds: &[u64],
    budget: u32,
    cap: usize,
    threads: usize,
) -> (Vec<Sample>, Vec<GameStats>) {
    let next_seed = AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<(u64, GameStats, Vec<Sample>)>> =
        std::sync::Mutex::new(Vec::with_capacity(seeds.len()));
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let k = next_seed.fetch_add(1, Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let seed = seeds[k];
                    let mut sink: Vec<Sample> = Vec::new();
                    let mut policy = LanePolicy::Teacher { budget, sink: &mut sink };
                    let stats = play_lane(genome, seed, regime, cap, &mut policy);
                    results.lock().expect("results lock").push((seed, stats, sink));
                }
            });
        }
    });
    let mut out = results.into_inner().expect("results");
    out.sort_by_key(|(seed, _, _)| *seed);
    let mut samples = Vec::new();
    let mut stats = Vec::new();
    for (_, st, mut sink) in out {
        stats.push(st);
        samples.append(&mut sink);
    }
    (samples, stats)
}

/// The eval arms (T3 protocol + the T8 gate). JSON keys spell the names
/// (`reflex_head` / `champion_b0` / `critic_ridge`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EvalArm {
    Reflex,
    B0,
    Critic,
}

/// Everything one eval cell needs beyond the arm and the seed list.
struct EvalCtx<'a> {
    model: &'a CriticModel,
    genome: &'a katgpt_tetris::rulebook::Genome,
    fixtures: &'a Fixtures,
    regime: Regime,
    cap: usize,
    threads: usize,
}

/// All seeds' games for one eval arm, threaded over seeds.
fn run_cell(arm: EvalArm, ctx: &EvalCtx<'_>, seeds: &[u64]) -> Vec<(u64, GameStats)> {
    let model = ctx.model;
    let regime = ctx.regime;
    let cap = ctx.cap;
    let next_seed = AtomicUsize::new(0);
    let results: std::sync::Mutex<Vec<(u64, GameStats)>> =
        std::sync::Mutex::new(Vec::with_capacity(seeds.len()));
    std::thread::scope(|s| {
        for _ in 0..ctx.threads.max(1) {
            s.spawn(|| match arm {
                EvalArm::Critic => {
                    loop {
                        let k = next_seed.fetch_add(1, Ordering::Relaxed);
                        if k >= seeds.len() {
                            break;
                        }
                        let seed = seeds[k];
                        let mut policy = LanePolicy::Argmax { model };
                        let st = play_lane(ctx.genome, seed, regime, cap, &mut policy);
                        results.lock().expect("results lock").push((seed, st));
                    }
                }
                EvalArm::Reflex | EvalArm::B0 => {
                    let kind = if arm == EvalArm::Reflex {
                        ArmKind::ReflexHead
                    } else {
                        ArmKind::TeacherPuct { budget: 0 }
                    };
                    let mut runner = Runner::new(kind, ctx.fixtures, ctx.genome.clone());
                    loop {
                        let k = next_seed.fetch_add(1, Ordering::Relaxed);
                        if k >= seeds.len() {
                            break;
                        }
                        let seed = seeds[k];
                        let st = runner.play(seed, regime, cap, None);
                        results.lock().expect("results lock").push((seed, st));
                    }
                }
            });
        }
    });
    let mut out = results.into_inner().expect("results");
    out.sort_by_key(|(seed, _)| *seed);
    out
}

fn summarize(res: &[(u64, GameStats)]) -> Value {
    let pieces: f64 = res.iter().map(|(_, s)| s.pieces as f64).sum::<f64>() / res.len().max(1) as f64;
    let lines: f64 = res.iter().map(|(_, s)| s.lines as f64).sum::<f64>() / res.len().max(1) as f64;
    let points: f64 = res.iter().map(|(_, s)| s.points as f64).sum::<f64>() / res.len().max(1) as f64;
    let survived = res.iter().filter(|(_, s)| !s.topped_out).count();
    let mut durs: Vec<f64> = res.iter().map(|(_, s)| s.p50_ms).collect();
    durs.sort_by(f64::total_cmp);
    json!({
        "mean_pieces": pieces,
        "mean_lines": lines,
        "mean_points": points,
        "survived": survived,
        "of": res.len(),
        "p50_decision_ms": durs.get(durs.len() / 2).copied().unwrap_or(0.0),
    })
}

/// Paired pieces diff of `res` vs `base` (both seed-sorted, same seed
/// sets) restricted to the paired seeds (1..=PAIRED_SEEDS; 607 excluded).
fn paired_vs(res: &[(u64, GameStats)], base: &[(u64, GameStats)]) -> Value {
    let diffs: Vec<f64> = res
        .iter()
        .zip(base)
        .filter(|((s, _), (b, _))| s == b && (*s as usize) <= PAIRED_SEEDS)
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
    let (pm, plb) = match paired_upper_bound_f64(&diffs) {
        Some(d) => (d.mean, d.lb95),
        None => (0.0, f64::NEG_INFINITY),
    };
    json!({
        "n_paired": diffs.len(),
        "paired_mean": pm,
        "paired_lb95": plb,
        "wtl": format!("{w}/{t}/{l}"),
    })
}

/// Agreement/R² aggregated over blocks against ONE global mean q.
fn agg_agree(model: &CriticModel, blocks: &[&Block]) -> (f64, f64) {
    let n: usize = blocks.iter().map(|b| b.samples.len()).sum();
    let mean_q = blocks
        .iter()
        .flat_map(|b| b.samples.iter())
        .map(|s| s.q)
        .sum::<f64>()
        / n.max(1) as f64;
    let mut total = AgreeStats::default();
    for b in blocks {
        total = total + agreement_stats(model, &b.samples, &b.groups, mean_q);
    }
    (total.agreement(), total.r2())
}

fn main() {
    let a = parse_args();
    let started = Instant::now();

    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    assert_eq!(genome.id(), CHAMPION_ID, "champion genome drifted from the pinned digest");
    let fixtures = load_fixtures();

    let train_seeds: Vec<u64> = (a.train.0..=a.train.1).collect();
    let val_seeds: Vec<u64> = (a.val.0..=a.val.1).collect();
    let mut eval_seeds: Vec<u64> = (1..=PAIRED_SEEDS as u64).collect();
    eval_seeds.push(CONTEXT_SEED);

    println!("== tetris_critic_goat — Issue 009 T8's modelless second arm (closed-form KARC basis-ridge critic) ==");
    println!("{}", lane_context());
    println!(
        "teacher b{} (T6 posture: no preview, fresh bag) · train {}..={} · val {}..={} · regimes {:?} · cap {} · λ multipliers {:?} · {} workers",
        a.teacher_budget, a.train.0, a.train.1, a.val.0, a.val.1, a.regimes, a.cap, a.lambdas, a.threads
    );
    println!(
        "seed disjointness: train/val avoid 006's 1..=40 + 101..=140, the eval 1..=20, and 607 · box: load {}",
        box_load()
    );

    // ── Phase A: collect ────────────────────────────────────────────────
    let mut train_blocks: Vec<Block> = Vec::new();
    let mut val_blocks: Vec<Block> = Vec::new();
    let mut collection: Vec<Value> = Vec::new();
    let mut teacher_pieces: Vec<f64> = Vec::new();
    for &regime in &a.regimes {
        let t0 = Instant::now();
        let (ts, tstats) = collect(&genome, regime, &train_seeds, a.teacher_budget, a.cap, a.threads);
        let (vs, vstats) = collect(&genome, regime, &val_seeds, a.teacher_budget, a.cap, a.threads);
        teacher_pieces.extend(tstats.iter().chain(vstats.iter()).map(|s| s.pieces as f64));
        collection.push(json!({
            "regime": regime.to_string(),
            "train_samples": ts.len(),
            "val_samples": vs.len(),
            "teacher_mean_pieces": {
                "train": tstats.iter().map(|s| s.pieces).sum::<usize>() as f64 / tstats.len().max(1) as f64,
                "val": vstats.iter().map(|s| s.pieces).sum::<usize>() as f64 / vstats.len().max(1) as f64,
            },
            "wall_s": t0.elapsed().as_secs_f64(),
        }));
        train_blocks.push(Block { groups: decision_groups(&ts), samples: ts });
        val_blocks.push(Block { groups: decision_groups(&vs), samples: vs });
    }
    let n_train: usize = train_blocks.iter().map(|b| b.samples.len()).sum();
    let n_val: usize = val_blocks.iter().map(|b| b.samples.len()).sum();
    println!(
        "collected {n_train} train + {n_val} val samples · teacher mean pieces {:.0}",
        teacher_pieces.iter().sum::<f64>() / teacher_pieces.len().max(1) as f64
    );

    // ── Phase B: fit (accumulate ONCE per corpus, solve per λ) ─────────
    let scale = FeatureScale::fit_iter(train_blocks.iter().flat_map(|b| b.samples.iter()));
    let mut train_acc = FitAccumulator::new();
    for b in &train_blocks {
        train_acc.accumulate(&b.samples, &b.groups, &scale);
    }
    let mut val_acc = FitAccumulator::new();
    for b in &val_blocks {
        val_acc.accumulate(&b.samples, &b.groups, &scale);
    }

    println!("λ sweep (per-sample multipliers; selection = val top-1 agreement with the teacher pick, tiebreak R²):");
    let mut sweep: Vec<(f64, f64, f64, f64)> = Vec::new(); // (λ, val_agree, val_r2, train_r2)
    let train_refs: Vec<&Block> = train_blocks.iter().collect();
    let val_refs: Vec<&Block> = val_blocks.iter().collect();
    for &lam in &a.lambdas {
        let m = train_acc.solve(scale.clone(), lam);
        let (va, vr2) = agg_agree(&m, &val_refs);
        let (_, tr2) = agg_agree(&m, &train_refs);
        sweep.push((lam, va, vr2, tr2));
        println!("  λ×n {lam:>6}: val agree {va:.3} val R² {vr2:.4} train R² {tr2:.4}");
    }
    let chosen = sweep
        .iter()
        .max_by(|x, y| x.1.total_cmp(&y.1).then(x.2.total_cmp(&y.2)).then(y.0.total_cmp(&x.0)))
        .map(|x| x.0)
        .expect("λ grid non-empty");
    println!("chosen λ multiplier {chosen}");

    // Final model: refit on train+val at the chosen λ (val was spent on
    // selection only; the refit is the standard practice and deterministic).
    let mut final_acc = FitAccumulator::new();
    for b in train_blocks.iter().chain(val_blocks.iter()) {
        final_acc.accumulate(&b.samples, &b.groups, &scale);
    }
    let model = final_acc.solve(scale, chosen);
    let (final_val_agree, final_val_r2) = agg_agree(&model, &val_refs);
    let (final_train_agree, final_train_r2) = agg_agree(&model, &train_refs);
    println!(
        "final model digest {} · design dim {} · n {} · train agree {:.3} R² {:.4} · val agree {:.3} R² {:.4}",
        model.digest_hex(),
        riir_instinct::tetris_critic::DESIGN_DIM,
        model.n,
        final_train_agree,
        final_train_r2,
        final_val_agree,
        final_val_r2
    );

    // ── Phase C: the T3 protocol eval + the T8 gate ────────────────────
    let mut eval: Vec<Value> = Vec::new();
    let mut gate_hit: Option<Value> = None;
    for &regime in &a.regimes {
        let ctx = EvalCtx {
            model: &model,
            genome: &genome,
            fixtures: &fixtures,
            regime,
            cap: a.cap,
            threads: a.threads,
        };
        let base = run_cell(EvalArm::Reflex, &ctx, &eval_seeds);
        let b0 = run_cell(EvalArm::B0, &ctx, &eval_seeds);
        let crit = run_cell(EvalArm::Critic, &ctx, &eval_seeds);
        let vs_reflex = paired_vs(&crit, &base);
        let vs_b0 = paired_vs(&crit, &b0);
        let b0_vs_reflex = paired_vs(&b0, &base);
        let crit_sum = summarize(&crit);
        let lb_crit_b0 = vs_b0["paired_lb95"].as_f64().unwrap_or(f64::NEG_INFINITY);
        println!(
            "{:<14} reflex {:.1} pcs · b0 {:.1} pcs · critic {:.1} pcs · critic p50 {:.3}ms · vs reflex Δ{:+.1} lb95 {:+.1} ({}) · vs b0 Δ{:+.1} lb95 {:+.1} ({})",
            regime.to_string(),
            base.iter().map(|(_, s)| s.pieces as f64).sum::<f64>() / base.len() as f64,
            b0.iter().map(|(_, s)| s.pieces as f64).sum::<f64>() / b0.len() as f64,
            crit_sum["mean_pieces"].as_f64().unwrap_or(0.0),
            crit_sum["p50_decision_ms"].as_f64().unwrap_or(0.0),
            vs_reflex["paired_mean"].as_f64().unwrap_or(0.0),
            vs_reflex["paired_lb95"].as_f64().unwrap_or(0.0),
            vs_reflex["wtl"].as_str().unwrap_or(""),
            vs_b0["paired_mean"].as_f64().unwrap_or(0.0),
            lb_crit_b0,
            vs_b0["wtl"].as_str().unwrap_or(""),
        );
        // The context seed's own row (excluded from every gate column).
        let ctx_row = |res: &[(u64, GameStats)]| -> Value {
            match res.iter().find(|(s, _)| *s == CONTEXT_SEED) {
                Some((_, st)) => json!({
                    "pieces": st.pieces, "lines": st.lines, "points": st.points,
                    "topped_out": st.topped_out,
                }),
                None => json!(null),
            }
        };
        if lb_crit_b0 > 0.0 && gate_hit.is_none() {
            gate_hit = Some(json!({
                "regime": regime.to_string(),
                "vs_b0": vs_b0,
                "vs_reflex": vs_reflex,
            }));
        }
        eval.push(json!({
            "regime": regime.to_string(),
            "arms": {
                "reflex_head": summarize(&base),
                "champion_b0": summarize(&b0),
                "critic_ridge": crit_sum,
            },
            "critic_vs_reflex": vs_reflex,
            "critic_vs_champion_b0": vs_b0,
            "b0_vs_reflex": b0_vs_reflex,
            "context_seed607": {
                "reflex_head": ctx_row(&base),
                "champion_b0": ctx_row(&b0),
                "critic_ridge": ctx_row(&crit),
            },
        }));
    }

    // ── Verdict + record ────────────────────────────────────────────────
    let verdict = if gate_hit.is_some() {
        "MODELLESS CANDIDATE — the closed-form critic strictly beats the b0 baseline \
         (paired lb95 > 0): the modelless path may carry the gain; the trained critic \
         (T7) must beat THIS arm, not just b0, to justify its spend."
    } else {
        "MODELLESS ARM DOES NOT CLEAR THE T8 BAR — b0 stands; T7's trained critic must \
         beat b0 AND this arm (the closed-form readout is the arm to beat)."
    };
    println!("T8-second-arm VERDICT: {verdict}");

    let doc = json!({
        "meta": {
            "issue": "riir-instinct 009 T8 second modelless arm (KARC basis-ridge critic) + T7 substrate",
            "recorded_at": iso_now(),
            "box_load": box_load(),
            "champion_genome_id": CHAMPION_ID,
            "fit": lane_context(),
            "teacher": format!(
                "chance_puct budget {} (T6 posture: no preview, fresh bag, mode at the decision root, rng = seed ^ {:#x})",
                a.teacher_budget,
                riir_instinct::tetris_lane::TEACHER_RNG_SALT
            ),
            "train_seeds": format!("{}..={}", a.train.0, a.train.1),
            "val_seeds": format!("{}..={}", a.val.0, a.val.1),
            "eval_seeds": format!("1..={PAIRED_SEEDS} paired + {CONTEXT_SEED} beside (excluded from every gate column)"),
            "seed_disjointness": "train/val avoid 006's training (1..=40) + held-out (101..=140), the eval 1..=20, and 607",
            "regimes": a.regimes.iter().map(|r| r.to_string()).collect::<Vec<_>>(),
            "cap": a.cap,
            "threads": a.threads,
            "selection": "λ per-sample multiplier swept; chosen by val top-1 agreement with the teacher pick, tiebreak val R², then lowest λ",
            "wall_s": started.elapsed().as_secs_f64(),
        },
        "collection": collection,
        "n_train_samples": n_train,
        "n_val_samples": n_val,
        "model": {
            "digest": model.digest_hex(),
            "design_dim": riir_instinct::tetris_critic::DESIGN_DIM,
            "chosen_lambda_mult": chosen,
            "n_final": model.n,
            "train_agreement": final_train_agree,
            "train_r2": final_train_r2,
            "val_agreement": final_val_agree,
            "val_r2": final_val_r2,
        },
        "lambda_sweep": sweep.iter().map(|(lam, va, vr2, tr2)| json!({
            "lambda_mult": lam, "val_agreement": va, "val_r2": vr2, "train_r2": tr2,
        })).collect::<Vec<_>>(),
        "eval": eval,
        "gate_hit": gate_hit,
        "gate": "T8-second-arm modelless candidate iff critic vs champion_b0 paired pieces lb95 > 0 on some regime",
        "verdict": verdict,
    });
    std::fs::create_dir_all(&a.out).expect("create out dir");
    let path = a.out.join("tetris_critic_arm.json");
    std::fs::write(&path, serde_json::to_string_pretty(&doc).expect("serialize")).expect("write results");
    println!("wrote {}", path.display());
}

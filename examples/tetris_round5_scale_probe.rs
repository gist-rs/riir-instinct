//! tetris_round5_scale_probe — the Bench-025 follow-up (Issue 009 T7 round
//! 5, reviewer round 1): per-seed piece counts for every arm of the round-5
//! question on ONE seed set / ONE setup, so the pilot's claims carry
//! detection limits instead of bare means.
//!
//! What the n=20 first pilot could not do (the REVISE round's finding):
//! each arm is 20 GAMES (SE ≈ 30-40 pieces), so "+33 at b400" was ~1 SE and
//! the b1600 null ruled out nothing smaller than ±60. This probe prints a
//! PER-SEED CSV (`<out>/seeds_<arm>.csv`: arm,seed,pieces,capped) + a
//! paired summary vs the first requested arm (house instrument
//! `stats::paired_upper_bound_f64`), and records the capped fraction —
//! censoring at the cap compresses differences and must be visible.
//!
//! Arms (one scale, no eval-seed spend — use seeds ≥ 401):
//! - `b0` — the 1-ply champion evaluator (budget-0 PUCT prior), the
//!   T3 bar's own baseline.
//! - `mlp1ply` — the r4 trained critic as a 1-ply policy (`--critic`),
//!   the "weaker ranker" claim's own measurement.
//! - `t400`/`t1600`/`t6400` — the plain teacher ladder (b6400 prices the
//!   ladder successor's ceiling directly).
//! - `blendNNN` — the value-seam blended teacher at budget NNN (weight
//!   from `--weight`, default 0.5).
//!
//! ```sh
//! cargo run --release --features tetris --example tetris_round5_scale_probe -- \
//!     --seeds 401:500 --regime 18:75 --cap 600 --threads 10 \
//!     [--critic ../riir-train/data/tetris_critic_r4/tetris_mlp_v1.bin] \
//!     [--weight 0.5] [--arms b0,mlp1ply,t400,t1600,t6400] \
//!     [--out /tmp/r5_scale]
//! ```

use riir_instinct::stats::paired_upper_bound_f64;
use riir_instinct::tetris_blend::play_blended;
use riir_instinct::tetris_critic::{LanePolicy, Sample, TrainedMlp};
use riir_instinct::tetris_lane::Regime;
use std::path::PathBuf;

fn parse_regime(s: &str) -> Regime {
    if s == "empty" {
        return Regime::EMPTY;
    }
    let (r, f) = s.split_once(':').expect("regime rows:fill or empty");
    Regime::garbage(r.parse().expect("rows"), f.parse().expect("fill"))
}

fn main() {
    let mut seeds = (401u64..=500).collect::<Vec<_>>();
    let mut regime = Regime::garbage(18, 75);
    let mut cap = 600usize;
    let mut threads = 10usize;
    let mut critic_path: Option<PathBuf> = None;
    let mut weight = 0.5f64;
    let mut arms = "b0,mlp1ply,t400,t1600,t6400".to_string();
    let mut out = PathBuf::from("/tmp/r5_scale");
    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < argv.len() {
        let val = || argv.get(i + 1).cloned().unwrap_or_default();
        match argv[i].as_str() {
            "--seeds" => {
                let v = val();
                let (lo, hi) = v.split_once(':').expect("seeds lo:hi");
                seeds = (lo.parse().expect("lo")..=hi.parse().expect("hi")).collect();
            }
            "--regime" => regime = parse_regime(&val()),
            "--cap" => cap = val().parse().expect("cap"),
            "--threads" => threads = val().parse().expect("threads"),
            "--critic" => critic_path = Some(PathBuf::from(val())),
            "--weight" => weight = val().parse().expect("weight f64"),
            "--arms" => arms = val(),
            "--out" => out = PathBuf::from(val()),
            other => panic!("unknown arg {other:?}"),
        }
        i += 2;
    }

    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    let critic = critic_path.map(|p| {
        let raw = std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
        TrainedMlp::from_bytes(&raw)
            .unwrap_or_else(|e| panic!("--critic {}: {e}", p.display()))
    });
    std::fs::create_dir_all(&out).expect("create out");
    println!(
        "== round-5 scale probe · regime {regime} · cap {cap} · {} seeds {}..={} · threads {threads} ==",
        seeds.len(),
        seeds.first().copied().unwrap_or(0),
        seeds.last().copied().unwrap_or(0)
    );

    #[allow(clippy::type_complexity)]
    let mut results: Vec<(String, Vec<(u64, f64, bool)>, f64)> = Vec::new();
    for arm in arms.split(',').filter(|s| !s.is_empty()) {
        let needs_critic = arm.starts_with("blend") || arm == "mlp1ply";
        if needs_critic && critic.is_none() {
            panic!("arm {arm} needs --critic");
        }
        let t0 = std::time::Instant::now();
        let rows: std::sync::Mutex<Vec<(u64, f64, bool)>> =
            std::sync::Mutex::new(Vec::with_capacity(seeds.len()));
        let next_seed = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|s| {
            for _ in 0..threads.max(1) {
                s.spawn(|| loop {
                    let k = next_seed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if k >= seeds.len() {
                        break;
                    }
                    let seed = seeds[k];
                    let st = match arm {
                        "b0" | "t400" | "t1600" | "t6400" => {
                            let budget = match arm {
                                "b0" => 0u32,
                                "t400" => 400,
                                "t1600" => 1600,
                                _ => 6400,
                            };
                            let mut sink: Vec<Sample> = Vec::new();
                            let mut policy = LanePolicy::Teacher { budget, sink: &mut sink };
                            riir_instinct::tetris_critic::play_lane(
                                &genome, seed, regime, cap, &mut policy,
                            )
                        }
                        "mlp1ply" => {
                            let m = critic.as_ref().expect("checked");
                            let mut policy = LanePolicy::Mlp { model: m };
                            riir_instinct::tetris_critic::play_lane(
                                &genome, seed, regime, cap, &mut policy,
                            )
                        }
                        _ if arm.starts_with("blend") => {
                            let m = critic.as_ref().expect("checked");
                            let budget: u32 = arm
                                .strip_prefix("blend")
                                .expect("blendNNN (e.g. blend400)")
                                .parse()
                                .expect("budget u32");
                            let mut sink: Vec<Sample> = Vec::new();
                            play_blended(&genome, m, weight, seed, regime, cap, budget, &mut sink)
                        }
                        other => panic!("unknown arm {other:?}"),
                    };
                    rows.lock()
                        .expect("rows")
                        .push((seed, st.pieces as f64, st.pieces >= cap));
                });
            }
        });
        let mut rows = rows.into_inner().expect("rows");
        rows.sort_by_key(|(seed, _, _)| *seed);
        let n = rows.len();
        let mean = rows.iter().map(|(_, p, _)| p).sum::<f64>() / n.max(1) as f64;
        let capped = rows.iter().filter(|(_, _, c)| *c).count();
        let mut sorted: Vec<f64> = rows.iter().map(|(_, p, _)| *p).collect();
        sorted.sort_by(f64::total_cmp);
        let wall = t0.elapsed().as_secs_f64();
        println!(
            "{arm:>12}: n {n} · mean {mean:7.1} · p50 {p50:7.1} · capped {capped}/{n} ({:.0}%) · {wall:.0}s",
            100.0 * capped as f64 / n as f64,
            p50 = sorted[n / 2],
        );
        let mut csv = String::from("arm,seed,pieces,capped\n");
        for (seed, p, c) in &rows {
            csv.push_str(&format!("{arm},{seed},{p},{}\n", u8::from(*c)));
        }
        std::fs::write(out.join(format!("seeds_{arm}.csv")), csv).expect("write per-seed csv");
        results.push((arm.to_string(), rows, wall));
    }

    // Paired summary: every arm after the first vs the first (same seeds,
    // same order — rows are seed-sorted).
    if results.len() >= 2 {
        let (base_name, base_rows, _) = &results[0];
        println!("\npaired Δ vs {base_name} (mean Δ, SE, lb95, ub95):");
        for (name, rows, _) in &results[1..] {
            let base_by_seed: std::collections::HashMap<u64, f64> =
                base_rows.iter().map(|(s, p, _)| (*s, *p)).collect();
            let diffs: Vec<f64> = rows
                .iter()
                .filter_map(|(s, p, _)| base_by_seed.get(s).map(|b| p - b))
                .collect();
            match paired_upper_bound_f64(&diffs) {
                Some(d) => println!(
                    "  {name:>12} − {base_name:<8}: Δ{:+7.1} · SE {:5.1} · lb95 {:+7.1} · ub95 {:+7.1} (n {})",
                    d.mean, d.se, d.lb95, d.ub95, diffs.len()
                ),
                None => println!("  {name:>12}: pairing unavailable"),
            }
        }
    }
}

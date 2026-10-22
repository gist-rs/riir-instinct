//! export_tetris_critic_cont — the `TETCRIT2` multi-continuation dataset
//! (riir-train Plan 429 T1.1, with Plan 460 T1.1's per-step format
//! requirement). Layout and continuation law: [`riir_instinct::tetris_cont`].
//!
//! The plain teacher (`export_tetris_critic`'s default arm, same seeds,
//! same budget, same game loop) plays each seed; at every
//! `--stride`-th decision its `--top-k` most-visited options each get
//! `--m` continuation rollouts under the champion 1-ply policy and a fresh
//! bag, to TERMINAL (Plan 429's law): the default `--cont-cap` is the u16
//! maximum, measured above every observed game length (survival pilot,
//! 2026-10-23: 100% of 11,328 rollouts topped out, mean ~5.5k placements);
//! a rollout that still reaches the cap is FLAGGED truncated, never
//! silently read as terminal. Output: gitignored split files
//! under `--out` + a BLAKE3 manifest (the pin).
//!
//! Run:
//! ```sh
//! cargo run --release --features tetris --bin export_tetris_critic_cont -- \
//!     [--out .raw/tetris_critic_cont] [--teacher-budget 1600] \
//!     [--train-seeds 201:280] [--val-seeds 281:300] \
//!     [--regimes 16:75,18:75] [--cap 1000] [--threads 10] \
//!     [--m 8] [--cont-cap 65535] [--stride 8] [--top-k 4]
//! ```

use katgpt_tetris::lookahead::LINES_SCORE;
use riir_instinct::tetris_cont::{
    decode_tetcrit2, encode_tetcrit2, ContConfig, CONT_SALT, HEADER_V2_BYTES, SAMPLE_V1_BYTES,
};
use riir_instinct::tetris_critic::collect_teacher_cont;
use riir_instinct::tetris_lane::{Regime, CHAMPION_ID, TEACHER_RNG_SALT};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

struct Args {
    out: PathBuf,
    budget: u32,
    train: (u64, u64),
    val: (u64, u64),
    regimes: Vec<Regime>,
    cap: usize,
    threads: usize,
    cont: ContConfig,
}

fn parse_args() -> Args {
    let mut a = Args {
        out: PathBuf::from(".raw/tetris_critic_cont"),
        budget: 1600,
        train: (201, 280),
        val: (281, 300),
        regimes: vec![Regime::garbage(16, 75), Regime::garbage(18, 75)],
        cap: 1000,
        threads: 10,
        cont: ContConfig {
            m: 8,
            cap: u16::MAX,
            decision_stride: 8,
            top_k: 4,
        },
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let range = |s: &str| -> (u64, u64) {
        let (lo, hi) = s.split_once(':').expect("seed range lo:hi");
        (lo.parse().expect("lo"), hi.parse().expect("hi"))
    };
    let mut i = 0;
    while i < argv.len() {
        let v = argv
            .get(i + 1)
            .unwrap_or_else(|| panic!("{} needs a value", argv[i]))
            .as_str();
        match argv[i].as_str() {
            "--out" => a.out = PathBuf::from(v),
            "--teacher-budget" => a.budget = v.parse().expect("budget u32"),
            "--train-seeds" => a.train = range(v),
            "--val-seeds" => a.val = range(v),
            "--cap" => a.cap = v.parse().expect("cap usize"),
            "--threads" => a.threads = v.parse().expect("threads usize"),
            "--m" => a.cont.m = v.parse().expect("m u8"),
            "--cont-cap" => a.cont.cap = v.parse().expect("cont-cap u16"),
            "--stride" => a.cont.decision_stride = v.parse().expect("stride u32"),
            "--top-k" => a.cont.top_k = v.parse().expect("top-k usize"),
            "--regimes" => {
                a.regimes = v
                    .split(',')
                    .map(|s| {
                        let (r, f) = s.split_once(':').expect("regime rows:fill");
                        Regime::garbage(r.parse().expect("rows"), f.parse().expect("fill"))
                    })
                    .collect();
            }
            other => panic!("unknown arg {other:?}"),
        }
        i += 2;
    }
    a.cont.validate();
    a
}

fn main() {
    let a = parse_args();
    let started = Instant::now();
    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    assert_eq!(
        genome.id(),
        CHAMPION_ID,
        "champion genome drifted from the pinned digest"
    );
    println!(
        "== export_tetris_critic_cont — TETCRIT2 (teacher b{}, M={}, cont-cap {}, stride {}, top-k {}) ==",
        a.budget, a.cont.m, a.cont.cap, a.cont.decision_stride, a.cont.top_k
    );
    std::fs::create_dir_all(&a.out).expect("create out dir");

    let mut files = Vec::<Value>::new();
    for &regime in &a.regimes {
        let tag = if regime.rows == 0 {
            "empty".to_string()
        } else {
            format!("{}x{}", regime.rows, regime.fill)
        };
        for (split, (lo, hi)) in [("train", a.train), ("val", a.val)] {
            let seeds: Vec<u64> = (lo..=hi).collect();
            let t0 = Instant::now();
            let (samples, stats) =
                collect_teacher_cont(&genome, &a.cont, regime, &seeds, a.budget, a.cap, a.threads);
            let buf = encode_tetcrit2(&samples, &a.cont);
            // Read-back: the file on disk must decode to what was collected.
            let back = decode_tetcrit2(&buf).expect("TETCRIT2 read-back");
            assert_eq!(back.samples.len(), samples.len(), "read-back sample count");
            assert!(
                back.samples
                    .iter()
                    .zip(&samples)
                    .all(|(x, y)| x.rollouts == y.rollouts
                        && x.sample.q.to_bits() == y.sample.q.to_bits()),
                "read-back content drift"
            );
            let name = format!("{split}-{tag}.bin");
            std::fs::write(a.out.join(&name), &buf).expect("write split file");

            let rolls = samples.len() * usize::from(a.cont.m);
            let steps: usize = samples
                .iter()
                .flat_map(|s| s.rollouts.iter().map(|r| r.lines.len()))
                .sum();
            let topped = samples
                .iter()
                .flat_map(|s| &s.rollouts)
                .filter(|r| r.topped_out)
                .count();
            let mean_pieces =
                stats.iter().map(|s| s.pieces).sum::<usize>() as f64 / stats.len().max(1) as f64;
            println!(
                "{name}: {} samples · {rolls} rollouts · mean len {:.1} · topped {:.3} · {:.1} MB · teacher mean pieces {mean_pieces:.0} · {}s",
                samples.len(),
                steps as f64 / rolls.max(1) as f64,
                topped as f64 / rolls.max(1) as f64,
                buf.len() as f64 / 1e6,
                t0.elapsed().as_secs()
            );
            files.push(json!({
                "name": name,
                "split": split,
                "regime": regime.to_string(),
                "seeds": format!("{lo}..={hi}"),
                "samples": samples.len(),
                "rollouts": rolls,
                "steps": steps,
                "mean_rollout_len": steps as f64 / rolls.max(1) as f64,
                "topped_out_fraction": topped as f64 / rolls.max(1) as f64,
                "bytes": buf.len(),
                "bytes_per_sample": buf.len() as f64 / samples.len().max(1) as f64,
                "blake3": blake3::hash(&buf).to_hex().to_string(),
                "teacher_mean_pieces": mean_pieces,
            }));
        }
    }

    let manifest = json!({
        "magic": "TETCRIT2",
        "layout": format!(
            "header {HEADER_V2_BYTES} B (magic, n:u32, m:u8, cap:u16, pad:u8); per sample: the TETCRIT1 {SAMPLE_V1_BYTES} B record, then per rollout len:u16 flags:u8 (bit0 topped_out) lines:[u8; len]"
        ),
        "generated_at": SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_secs(),
        "contract": "riir-instinct src/tetris_cont.rs (layout + continuation law); src/tetris_critic.rs feat (the 33-feature order)",
        "champion_genome_id": CHAMPION_ID,
        "teacher": {
            "engine": "katgpt_core::chance_puct over the champion evaluator (the export_tetris_critic default arm)",
            "budget": a.budget,
            "information_rule": "no preview, fresh bag (uniform-7 chance); mode at the decision root",
            "rng_salt": format!("{TEACHER_RNG_SALT:#x}"),
        },
        "continuations": {
            "m": a.cont.m,
            "cap": a.cont.cap,
            "decision_stride": a.cont.decision_stride,
            "top_k": a.cont.top_k,
            "selection": "decisions d with d % stride == 0; the top_k teacher-visited options by visits (ties -> lower index), 0 = all visited",
            "policy": "champion genome 1-ply argmax over the champion evaluator (f64, first strict max), per-step mode, no preview, no hold, fresh 7-bag",
            "piece_seed": format!("splitmix64(game_seed ^ {CONT_SALT:#x} ^ decision<<24 ^ option<<8 ^ j)"),
            "reward_channel": "lines cleared per placement (u8 0..=4); step 0 = the sampled option itself",
            "points_table": LINES_SCORE,
            "terminal": "topped_out flag set when no landing option exists before cap; otherwise truncated at cap",
        },
        "train_seeds": format!("{}..={}", a.train.0, a.train.1),
        "val_seeds": format!("{}..={}", a.val.0, a.val.1),
        "seed_disjointness": "avoids 006's 1..=40 + 101..=140, the eval 1..=20, and 607",
        "cap": a.cap,
        "target": "q = teacher search-root value (TETCRIT1 law); continuation values = Σ points_table[lines] per rollout",
        "files": files,
        "wall_s": started.elapsed().as_secs_f64(),
    });
    let mpath = a.out.join("manifest.json");
    std::fs::write(
        &mpath,
        serde_json::to_string_pretty(&manifest).expect("serialize"),
    )
    .expect("write manifest");
    println!("wrote {}", mpath.display());
}

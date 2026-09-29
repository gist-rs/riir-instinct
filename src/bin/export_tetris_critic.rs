//! export_tetris_critic — the Issue 009 T7 training dataset export
//! (riir-train Issue 580's input contract).
//!
//! Re-collects the teacher samples deterministically (the same
//! `collect_teacher` the critic bench fits on) and writes the RAW
//! (features, teacher-Q) pairs as fixed-width little-endian binary files
//! + a BLAKE3 manifest — the `distill_teacher` precedent (reflex Issue
//!   576 T3): gitignored data under `.raw/`, the manifest is the pin.
//!
//! File layout (magic `TETCRIT1`, then per sample, fixed width 299 B):
//! `seed:u64 decision:u32 piece:u8 mode:u8 is_pick:u8 pad:u8 q:f64
//! num:[f64;33]`. The feature ORDER is the contract documented in
//! `riir_instinct::tetris_critic::feat` (the manifest names it).
//!
//! Run:
//! ```sh
//! cargo run --release --features tetris --bin export_tetris_critic -- \
//!     [--out .raw/tetris_critic] [--teacher-budget 1600] \
//!     [--train-seeds 201:280] [--val-seeds 281:300] \
//!     [--regimes 16:75,18:75] [--cap 1000] [--threads 10] \
//!     [--blend <model.bin>:<w>]
//! ```
//!
//! `--blend` (issue 009 round 5): the teacher becomes the CRITIC-Z-BLENDED
//! search (the model's logit z-blended into the champion eval at the
//! decision-state eval seam at weight w) — the blended-dataset lane. The
//! manifest records the blend (model digest + w) beside the teacher block.

use riir_instinct::tetris_critic::{TrainedMlp, collect_teacher, collect_teacher_blended, feat};
use riir_instinct::tetris_lane::{CHAMPION_ID, Regime, TEACHER_PROBE_SALT, TEACHER_RNG_SALT};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Parsed CLI: (out dir, teacher budget, train range, val range, regimes, cap, threads, blend).
type ExportArgs = (PathBuf, u32, (u64, u64), (u64, u64), Vec<Regime>, usize, usize, Option<(PathBuf, f64)>);

fn parse_args() -> ExportArgs {
    let mut out = PathBuf::from(".raw/tetris_critic");
    let mut budget = 1600u32;
    let mut train = (201u64, 280u64);
    let mut val = (281u64, 300u64);
    let mut regimes = vec![Regime::garbage(16, 75), Regime::garbage(18, 75)];
    let mut cap = 1000usize;
    let mut threads = 10usize;
    let mut blend: Option<(PathBuf, f64)> = None;
    let mut argv: Vec<String> = std::env::args().collect();
    while argv.last().is_some_and(|x| x == "export_tetris_critic") {
        argv.pop();
    }
    let start = argv.iter().rposition(|x| x == "--").map(|p| p + 1).unwrap_or(1);
    let argv: Vec<String> = argv[start..].to_vec();
    let mut i = 0;
    while i < argv.len() {
        let arg = || argv.get(i + 1).cloned().unwrap_or_default();
        let range = |s: &str| -> (u64, u64) {
            let (lo, hi) = s.split_once(':').expect("seed range lo:hi");
            (lo.parse().expect("lo"), hi.parse().expect("hi"))
        };
        match argv[i].as_str() {
            "--out" => out = PathBuf::from(arg()),
            "--teacher-budget" => budget = arg().parse().expect("budget u32"),
            "--train-seeds" => train = range(&arg()),
            "--val-seeds" => val = range(&arg()),
            "--cap" => cap = arg().parse().expect("cap usize"),
            "--threads" => threads = arg().parse().expect("threads usize"),
            "--blend" => {
                let spec = arg();
                let (path, w) = spec.rsplit_once(':').expect("--blend <model.bin>:<w>");
                blend = Some((PathBuf::from(path), w.parse().expect("blend w f64")));
            }
            "--regimes" => {
                regimes = arg()
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
    (out, budget, train, val, regimes, cap, threads, blend)
}

const SAMPLE_BYTES: usize = 8 + 4 + 1 + 1 + 1 + 1 + 8 + feat::NUM * 8;

fn write_split(
    path: &PathBuf,
    samples: &[riir_instinct::tetris_critic::Sample],
) -> (String, usize) {
    let mut buf = Vec::with_capacity(12 + samples.len() * SAMPLE_BYTES);
    buf.extend_from_slice(b"TETCRIT1");
    buf.extend_from_slice(&(samples.len() as u32).to_le_bytes());
    for s in samples {
        buf.extend_from_slice(&s.seed.to_le_bytes());
        buf.extend_from_slice(&s.decision.to_le_bytes());
        buf.push(s.raw.piece);
        buf.push(s.raw.mode);
        buf.push(u8::from(s.is_pick));
        buf.push(0); // pad to the documented 299 B width
        buf.extend_from_slice(&s.q.to_le_bytes());
        for v in &s.raw.num {
            buf.extend_from_slice(&v.to_le_bytes());
        }
    }
    std::fs::write(path, &buf).expect("write split file");
    (blake3::hash(&buf).to_hex()[..16].to_string(), samples.len())
}

fn main() {
    let (out, budget, train, val, regimes, cap, threads, blend) = parse_args();
    let started = Instant::now();
    let genome = katgpt_tetris::rulebook::Genome::champion_hybrid();
    assert_eq!(genome.id(), CHAMPION_ID, "champion genome drifted from the pinned digest");

    // The blend component (round 5): the trained critic whose logit the
    // teacher z-blends. The digest rides the manifest beside the teacher
    // block (the provenance law).
    let model_path = blend.as_ref().map(|(p, _)| p.clone());
    let blend_w = blend.as_ref().map(|(_, w)| *w).unwrap_or(0.0);
    let model_bytes = model_path.as_ref().map(|p| std::fs::read(p).expect("read blend model"));
    let model = model_bytes
        .as_deref()
        .map(|raw| TrainedMlp::from_bytes(raw).expect("parse blend model"));
    let model_digest = model_bytes.as_deref().map(TrainedMlp::digest_hex);

    println!(
        "== export_tetris_critic — riir-train Issue 580's dataset (teacher b{budget}, no preview + fresh bag{})==",
        match &blend {
            Some((p, w)) => format!(", critic z-blend w={w} @ {})", p.display()),
            None => String::new(),
        }
    );
    std::fs::create_dir_all(&out).expect("create out dir");

    let mut files = Vec::<Value>::new();
    for &regime in &regimes {
        let tag = if regime.rows == 0 {
            "empty".to_string()
        } else {
            format!("{}x{}", regime.rows, regime.fill)
        };
        for (split, seeds) in [
            ("train", (train.0..=train.1).collect::<Vec<_>>()),
            ("val", (val.0..=val.1).collect::<Vec<_>>()),
        ] {
            let t0 = Instant::now();
            let (samples, stats) = match &model {
                Some(m) => collect_teacher_blended(
                    &genome, m, blend_w, regime, &seeds, budget, cap, threads,
                ),
                None => collect_teacher(&genome, regime, &seeds, budget, cap, threads),
            };
            let name = format!("{split}-{tag}.bin");
            let path = out.join(&name);
            let (digest, n) = write_split(&path, &samples);
            let mean_pieces =
                stats.iter().map(|s| s.pieces).sum::<usize>() as f64 / stats.len().max(1) as f64;
            println!(
                "{name}: {n} samples ({:.1} MB) · teacher mean pieces {mean_pieces:.0} · {}s",
                n as f64 * SAMPLE_BYTES as f64 / 1e6,
                t0.elapsed().as_secs()
            );
            files.push(json!({
                "name": name,
                "split": split,
                "regime": regime.to_string(),
                "samples": n,
                "bytes": n * SAMPLE_BYTES + 12,
                "blake3_16": digest,
                "teacher_mean_pieces": mean_pieces,
            }));
        }
    }

    let manifest = json!({
        "magic": "TETCRIT1",
        "sample_bytes": SAMPLE_BYTES,
        "generated_at": SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_secs(),
        "contract": "riir-instinct src/tetris_critic.rs (feat module: the 33-feature order; RawOpt.piece = Piece::index(), mode = Genome::mode_of as u8)",
        "champion_genome_id": CHAMPION_ID,
        "teacher": {
            "engine": if model.is_some() {
                "katgpt_core::chance_puct over the champion evaluator + the trained critic's logit z-blended at the decision-state eval seam (issue 009 round 5)"
            } else {
                "katgpt_core::chance_puct over the champion evaluator"
            },
            "budget": budget,
            "information_rule": "no preview, fresh bag (uniform-7 chance); mode at the decision root",
            "rng_salt": format!("{TEACHER_RNG_SALT:#x}"),
            "blend": model.as_ref().map(|_| json!({
                "w": blend_w,
                "model_blake3_16": model_digest,
                "form": "v = mean_e + ((1-w)*(e-mean_e)/std_e + w*(c-mean_c)/std_c)*std_e; z-fit over the root's kept top-8 by champion eval; priors pure champion; root plain",
                "probe_salt": format!("{TEACHER_PROBE_SALT:#x}"),
            })),
        },
        "train_seeds": format!("{}..={}", train.0, train.1),
        "val_seeds": format!("{}..={}", val.0, val.1),
        "seed_disjointness": "avoids 006's 1..=40 + 101..=140, the eval 1..=20, and 607",
        "cap": cap,
        "target": "q = teacher search-root value ∈ (0,1) (sigmoid-normalised, the chance_puct way); is_pick = the teacher's chosen option",
        "files": files,
        "wall_s": started.elapsed().as_secs_f64(),
    });
    let mpath = out.join("manifest.json");
    std::fs::write(&mpath, serde_json::to_string_pretty(&manifest).expect("serialize")).expect("write manifest");
    println!("wrote {}", mpath.display());
}

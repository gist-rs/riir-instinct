//! Issue 580 T5's G5-style parity gate: the TRAIN-side scores (written by
//! riir-train's `tetris_critic_trainer` into `parity_fixture.json`) must be
//! reproduced BIT-EXACTLY by the serve-side forward
//! ([`riir_instinct::tetris_critic::TrainedMlp::score`]) — the shipped head
//! is the trained head, same bytes, same op order.
//!
//! Skips loud (exit 0, named lines) when the environment is absent:
//! - `TET_CRITIC_MODEL` — the format-v1 weights file (`tetris_mlp_v1.bin`);
//! - `TET_CRITIC_PARITY` — the matching `parity_fixture.json`
//!   (defaults to `<model dir>/parity_fixture.json`).
//! `TET_CRITIC_PARITY_REQUIRE=1` turns the missing-data skip into a
//! failure (the slice_leak pattern) — never a silent green zero.

use serde_json::Value;

#[test]
fn serve_side_replays_train_side_scores_bit_exactly() {
    let model_path = match std::env::var("TET_CRITIC_MODEL") {
        Ok(p) => std::path::PathBuf::from(p),
        Err(_) => {
            let require = std::env::var("TET_CRITIC_PARITY_REQUIRE").is_ok_and(|v| v == "1");
            println!("SKIP: TET_CRITIC_MODEL unset — no trained head to parity-check");
            if require {
                panic!("TET_CRITIC_PARITY_REQUIRE=1 but TET_CRITIC_MODEL is unset");
            }
            return;
        }
    };
    let fixture_path = std::env::var("TET_CRITIC_PARITY")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let mut p = model_path.clone();
            p.set_file_name("parity_fixture.json");
            p
        });
    let raw = std::fs::read(&model_path).expect("read weights file");
    let model = riir_instinct::tetris_critic::TrainedMlp::from_bytes(&raw)
        .unwrap_or_else(|e| panic!("parse {}: {e}", model_path.display()));
    println!(
        "model {} · from {}",
        riir_instinct::tetris_critic::TrainedMlp::digest_hex(&raw),
        model_path.display()
    );

    let fixture: Vec<Value> =
        serde_json::from_str(&std::fs::read_to_string(&fixture_path).expect("read fixture"))
            .expect("parse fixture");
    assert!(fixture.len() >= 32, "parity fixture too thin: {} rows", fixture.len());

    let mut scratch = vec![0.0f64; model.scratch_len()];
    let mut mismatches = 0usize;
    for (i, row) in fixture.iter().enumerate() {
        let mut num = [0.0f64; 33];
        for (k, v) in row["num"].as_array().expect("num array").iter().enumerate() {
            num[k] = v.as_f64().expect("f64");
        }
        let raw_opt = riir_instinct::tetris_critic::RawOpt {
            num,
            piece: row["piece"].as_u64().expect("piece") as u8,
            mode: row["mode"].as_u64().expect("mode") as u8,
        };
        let served = model.score(&raw_opt, &mut scratch);
        let want = u64::from_str_radix(
            row["score_bits"].as_str().expect("score_bits").trim_start_matches("0x"),
            16,
        )
        .expect("hex bits");
        if served.to_bits() != want {
            mismatches += 1;
            println!(
                "row {i}: served {:016x} != train {:016x}",
                served.to_bits(),
                want
            );
        }
    }
    assert_eq!(mismatches, 0, "parity: {mismatches} of {} rows diverge", fixture.len());
    println!("PARITY: {} of {} rows bit-exact", fixture.len(), fixture.len());
}

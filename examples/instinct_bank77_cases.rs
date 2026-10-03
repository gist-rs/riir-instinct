//! Plan 425 T2 — build the banking77 teacher-pass cases.jsonl for the
//! riir-infer `dump_encoder_states` extraction, and CROSS-CHECK the
//! rebuilt population against the collapsed teacher's existing dump
//! (per-row gold equality over all 9993 rows — the join proof).
//!
//! ```text
//! cargo run --release -p riir-instinct --example instinct_bank77_cases \
//!     -- --datasets ../riir-reflex/.raw/datasets_t20k \
//!       --teacher ../riir-reflex/.raw/distill_teacher/banking77_teacher.bin \
//!       --out /tmp/p425/cases.jsonl
//! ```

use std::path::PathBuf;

use riir_instinct::instinct_bank77_cases::banking77_teacher_cases;
use riir_instinct::instinct_specialist::load_teacher_dump;

struct Args {
    datasets: String,
    teacher: String,
    out: String,
}

fn parse_args() -> Args {
    let mut a = Args {
        datasets: "../riir-reflex/.raw/datasets_t20k".into(),
        teacher: "../riir-reflex/.raw/distill_teacher/banking77_teacher.bin".into(),
        out: "/tmp/p425/cases.jsonl".into(),
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        let val = |i: &mut usize, name: &str| -> String {
            *i += 1;
            args.get(*i)
                .unwrap_or_else(|| panic!("{name} needs a value"))
                .clone()
        };
        match args[i].as_str() {
            "--datasets" => a.datasets = val(&mut i, "--datasets"),
            "--teacher" => a.teacher = val(&mut i, "--teacher"),
            "--out" => a.out = val(&mut i, "--out"),
            other => panic!("unknown flag {other}"),
        }
        i += 1;
    }
    a
}

fn main() {
    let args = parse_args();
    let (cases, classes) = banking77_teacher_cases(std::path::Path::new(&args.datasets));
    println!(
        "cases: {} rows · {} classes (first: {:?})",
        cases.len(),
        classes.len(),
        classes.first().map(String::as_str)
    );

    // The join proof: per-row gold equality against the collapsed
    // teacher's dump (which the teacher pass built from reflex's own case
    // law — the two laws must agree on every row or this lane's dumps
    // describe a different population).
    let dump = load_teacher_dump(std::path::Path::new(&args.teacher)).expect("teacher dump");
    assert_eq!(
        dump.classes, classes,
        "class universe drift vs the existing dump"
    );
    assert_eq!(
        dump.n_rows(),
        cases.len(),
        "row-count drift vs the existing dump"
    );
    let mut mismatches = 0usize;
    for (i, c) in cases.iter().enumerate() {
        if dump.gold[i] as usize != c.gold {
            mismatches += 1;
            if mismatches <= 5 {
                eprintln!("gold mismatch at row {i}: dump {} vs rebuilt {}", dump.gold[i], c.gold);
            }
        }
    }
    assert_eq!(mismatches, 0, "gold join broken at {mismatches} row(s)");
    println!("gold join: {}/{} EXACT vs the existing dump", cases.len(), cases.len());

    // Write cases.jsonl for the extraction bin.
    let path = PathBuf::from(&args.out);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdir out parent");
    }
    let mut out = String::with_capacity(1 << 20);
    for c in &cases {
        out.push_str(
            &serde_json::json!({
                "id": c.id,
                "state": c.state,
                "question": c.question,
            })
            .to_string(),
        );
        out.push('\n');
    }
    std::fs::write(&path, out).expect("write cases.jsonl");
    println!("wrote {}", path.display());
}

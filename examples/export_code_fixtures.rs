//! Issue 593 (riir-train) / riir-instinct Issue 008 T7 — export the frozen
//! code_fixtures spans as train envelopes the specialist trainer reads.
//!
//! The suite has NO datasets dir: it is the committed BLAKE3-pinned
//! fixture (`riir-reflex/src/harness/code_fixtures_frozen.json`), sliced
//! per module into eval / cal / docs (`code_frozen::slice_module`). The
//! eval slices ARE the frozen 32-question test — they are NEVER exported.
//! The trainable pool is cal + docs (112 spans), each exported as TWO rows
//! so one artifact carries the union label universe the seat's bridge
//! resolves by name:
//! - `(module_label, src)` — the 8-way "module" question;
//! - `("no" | "yes", src)` — the binary "is_pub" question, spelled with
//!   the producer's unified NOUL pair (the same law the typed_decisions
//!   artifact uses; the seat's noul rendering `[false, true]` maps by
//!   position onto this pair through the existing bridge).
//!
//! Output: `<datasets>/code_fixtures/train-0000.json` in the
//! datasets-server envelope shape `{rows: [{row_idx, row}]}` — the exact
//! bytes `instinct_specialist::read_train_envelope` parses. Deterministic
//! order: frozen module order, cal before docs, module row before is_pub
//! row. Rerunning overwrites byte-identically (asserted).
//!
//! ```text
//! cargo run --release --example export_code_fixtures -- \
//!     [--datasets ../riir-reflex/.raw/datasets_t20k]
//! ```

#[derive(serde::Serialize)]
struct EnvelopeRow<'a> {
    row_idx: usize,
    row: Row<'a>,
}

#[derive(serde::Serialize)]
struct Row<'a> {
    label_text: &'a str,
    text: &'a str,
}

fn main() {
    let mut datasets = "../riir-reflex/.raw/datasets_t20k".to_string();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;
    while i < args.len() {
        if args[i] == "--datasets" {
            i += 1;
            datasets = args.get(i).cloned().expect("--datasets needs a value");
        } else {
            panic!("unknown flag {}", args[i]);
        }
        i += 1;
    }
    let modules = riir_reflex::harness::code_frozen::frozen_modules();
    let mut rows: Vec<EnvelopeRow> = Vec::new();
    for m in modules {
        // cal then docs — the two TRAINABLE slices. eval is the frozen
        // test and is deliberately never touched here.
        for f in m.cal.iter().chain(&m.docs) {
            rows.push(EnvelopeRow {
                row_idx: rows.len(),
                row: Row { label_text: &m.label, text: &f.src },
            });
            rows.push(EnvelopeRow {
                row_idx: rows.len(),
                row: Row {
                    label_text: if f.is_pub { "yes" } else { "no" },
                    text: &f.src,
                },
            });
        }
    }
    let n_pub = rows
        .iter()
        .filter(|r| r.row.label_text == "yes")
        .count();
    let n_mod: usize = rows.iter().filter(|r| r.row.label_text != "no" && r.row.label_text != "yes").count();
    println!(
        "export_code_fixtures: {} modules · {} spans (cal+docs) · {} rows ({} module + \
         {} is_pub [{} yes / {} no])",
        modules.len(),
        rows.len() / 2,
        rows.len(),
        n_mod,
        rows.len() - n_mod,
        n_pub,
        rows.len() - n_mod - n_pub,
    );

    let doc = serde_json::json!({ "rows": rows });
    let dir = std::path::PathBuf::from(&datasets).join("code_fixtures");
    std::fs::create_dir_all(&dir).expect("create suite dir");
    let path = dir.join("train-0000.json");
    let bytes = serde_json::to_vec_pretty(&doc).expect("serialize envelope");
    // Idempotence: a rerun overwrites byte-identically (deterministic
    // order + pretty shapes) — assert it when a file already exists.
    if let Ok(prev) = std::fs::read(&path) {
        assert_eq!(
            prev, bytes,
            "{path:?} already exists and a rerun would change it — the export must be \
             deterministic; investigate before overwriting"
        );
    }
    std::fs::write(&path, &bytes).expect("write envelope");
    println!("export_code_fixtures: wrote {} ({} bytes)", path.display(), bytes.len());
}

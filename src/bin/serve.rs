//! The hosted instinct decision server binary — the thin env/arg shell
//! over the shared HTTP edge (`riir_instinct::serve_edge`, plan 009 T1):
//! parse the process surface, build a
//! [`riir_instinct::serve_edge::ServeConfig`], call
//! [`riir_instinct::serve_edge::run`]. The endpoint contract, the
//! lane-loading postures, the hoarding gate, the monotonic swap and
//! every refusal shape live on the edge module; this bin owns the env
//! vocabulary, the args, and the exit discipline (unknown arg / empty
//! suites filter → exit 1 loud; every edge boot refusal arrives as
//! `run`'s `Err` and dies the same way). The Rethink serve bin builds
//! the same `ServeConfig` from its own surface and calls the SAME
//! `run` — the edge is re-shared, never forked (riir-ai Proposal 052).

use riir_instinct::serve_edge::{run, ServeConfig};

fn main() {
    let mut datasets_dir = std::env::var("INSTINCT_DATASETS_DIR")
        .unwrap_or_else(|_| "../riir-reflex/.raw/datasets_t20k".into());
    let mut winners_dir =
        std::env::var("INSTINCT_WINNERS_DIR").unwrap_or_else(|_| "data/demo_specialists".into());
    let mut bind = std::env::var("INSTINCT_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    // Plan 426 T6's serve posture: unset = the gold corpus everywhere
    // (byte-identical boots); set = suites with a present
    // `<dir>/<suite>_synth.jsonl` seat it.
    let synth_corpus_dir = std::env::var("INSTINCT_SYNTH_CORPUS_DIR").ok();
    // The arsenal manifest: `INSTINCT_ARSENAL` or `--arsenal <path>`;
    // absent both, the embedded default (Proposal 001, law A5 — one
    // manifest per HOST/deployment).
    let mut arsenal_path: Option<String> = std::env::var("INSTINCT_ARSENAL").ok();
    let mut suite_filter: Option<Vec<String>> = None;
    let mut i = 1;
    let args: Vec<String> = std::env::args().collect();
    while i < args.len() {
        match args[i].as_str() {
            "--datasets-dir" => {
                i += 1;
                datasets_dir = args[i].clone();
            }
            "--winners-dir" => {
                i += 1;
                winners_dir = args[i].clone();
            }
            "--bind" => {
                i += 1;
                bind = args[i].clone();
            }
            "--arsenal" => {
                i += 1;
                arsenal_path = Some(args[i].clone());
            }
            "--suites" => {
                i += 1;
                // Raw names here; resolved against the manifest at the
                // edge (an unknown name refuses loud — the manifest is
                // the only selection surface, there is no table behind
                // it).
                let names: Vec<String> = args[i]
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect();
                if names.is_empty() {
                    die("--suites named no suite");
                }
                suite_filter = Some(names);
            }
            other => die(&format!("unknown arg {other}")),
        }
        i += 1;
    }

    let cfg = ServeConfig {
        bind,
        datasets_dir,
        winners_dir,
        synth_corpus_dir,
        arsenal_path,
        suite_filter,
        // The CORS posture stays env-driven by default — the edge reads
        // RIIR_INSTINCT_ALLOWED_ORIGIN itself when the config omits it.
        cors_origin: None,
        // The raw posture: the edge parses + validates the manifest and
        // loads lanes with its own raw loader (plan 009 T2's defaults).
        prevalidated_manifest: None,
        lane_loader: None,
    };
    if let Err(e) = run(cfg) {
        die(&e);
    }
}

fn die(msg: &str) -> ! {
    eprintln!("⛔ serve: {msg}");
    std::process::exit(1);
}

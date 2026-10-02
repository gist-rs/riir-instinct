// origin: src/bin/arena.rs @ B1 seam commit 22ae291, lines 371-406

    let mut synth_corpus: Option<PathBuf> = None;
    let mut synth_extra_cap = 128usize;
    // Issue 014 C1: the RECORD-ONLY encoder arm — a sealed NLEH head
    // replayed over the live laya encode of the seat's test cases. Needs
    // `arena-laya` (the laya tree); the read publishes `serve: ✗` — never
    // a serve change (the encoder class is refused at serve, issue 014
    // §1). Issue 016 T9: the artifact may be v2 (per-option, multi-
    // question) and the CHECKPOINT is the caller's — the artifact does
    // not carry its training checkpoint, so `--encoder-ckpt` names it.
    let mut encoder = EncoderOpts::default();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--datasets-dir" => {
                i += 1;
                datasets_dir = PathBuf::from(&args[i]);
            }
            "--winners-dir" => {
                i += 1;
                winners_dir = PathBuf::from(&args[i]);
            }
            "--out" => {
                i += 1;
                out_dir = PathBuf::from(&args[i]);
            }
            "--top-k" => {
                i += 1;
                top_k = args[i].parse().expect("--top-k needs a number");
            }
            "--suite" => {
                i += 1;
                only_suites.push(args[i].clone());
            }
            "--skip-pin-a0" => pin_a0 = false,
            "--synth-corpus" => {
                i += 1;

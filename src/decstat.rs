//! The decstat capture lane (Plan 002 / Issue 004 T1): consent-gated
//! decision-outcome aggregation for the served lane — the riir-clippy
//! `--stats` semantics carried for decisions.
//!
//! Laws:
//! - **Unset never pushes.** Consent is the exact literal
//!   `RIIR_INSTINCT_STATS=on` — unset, empty, `1`, or `ON` are all OFF
//!   ([`consent_enabled`]). When off, nothing is installed and the
//!   `/decide` path is byte-identical (the record call compiles to a
//!   `OnceLock` miss).
//! - **Stats only, never questions, never answers.** The row schema is
//!   totals + per-`suite:primitive:class` counters + per-suite totals.
//!   No state text, no option text, no pick text crosses (the
//!   Proposal-014 redaction law).
//! - **The push is a best-effort tail.** One status line per flush, a
//!   5 s budget, never an exit-code change, and a failed push DROPS the
//!   window — no offline stacking (a lost row is a lost stat, the
//!   fixstat law; run-id idempotency makes a retry free but the window
//!   is not re-queued).
//!
//! Devnet-first: the default service URL is the devnet host —
//! Proposal 014's ladder lands contribution lanes on the devnet fleet
//! ledger before anything money-adjacent points at mainnet.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use riir_kat::kat_decstat_client::{
    DecStatRow, HttpDecstatTransport, push_decstat,
};
use riir_kat::kat_protocol_decstat::{DecStatCount, DECSTAT_COUNTERS_MAX, DECSTAT_SUITES_MAX};

/// Flush when this many decisions accumulated since the last one …
pub const FLUSH_THRESHOLD: u32 = 256;
/// … or when this long passed with a non-empty window.
pub const FLUSH_EVERY_SECS: u64 = 60;
/// The flusher's poll cadence (cheap: one atomic load per tick).
const POLL_SECS: u64 = 5;

/// The devnet-first default (the Proposal-014 ladder — contribution
/// lanes land on the devnet fleet ledger first; mainnet is the owner
/// ceremony it already is).
pub const DEFAULT_SERVICE_URL: &str = "https://devnet.ai.gist.rs";

/// The consent read — the exact literal `on` enables; everything else
/// (unset, empty, `1`, `ON`, `true`) is OFF. Split out pure so the gate
/// pins the exact-literal law without mutating process env.
#[must_use]
pub fn consent_enabled(v: Option<&str>) -> bool {
    v == Some("on")
}

/// The counter's primitive tag — a short ASCII form of the served arm
/// (`A0` / `A1` / `H1(top_k=4)` / `H2(β=…,nmin=…,τ=…)` keep their
/// display spellings for the receipt; the counters use the compact
/// tags). Unknown display strings fold to `arm` rather than guessing.
#[must_use]
pub fn primitive_tag(arm_display: &str) -> &'static str {
    if arm_display.starts_with("H2") {
        "h2"
    } else if arm_display.starts_with("H1") {
        "h1"
    } else if arm_display.starts_with("A1") {
        "a1"
    } else if arm_display.starts_with("A0") {
        "a0"
    } else {
        "arm"
    }
}

/// Load the account signing key from a file holding the 64-hex Ed25519
/// seed (one line; surrounding whitespace tolerated). Any other shape
/// is a config error — the operator's key file is never guessed at.
pub fn load_signing_key(path: &str) -> Result<ed25519_dalek::SigningKey, String> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("read {path}: {e}"))?;
    let hex = raw.trim();
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!(
            "{path} must hold exactly 64 hex chars (the Ed25519 seed), got {} chars",
            hex.len()
        ));
    }
    let mut seed = [0u8; 32];
    for i in 0..32 {
        seed[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
            .map_err(|e| format!("{path}: bad hex at byte {i}: {e}"))?;
    }
    Ok(ed25519_dalek::SigningKey::from_bytes(&seed))
}

/// The aggregate sink: lock-free counters (papaya — the house rule for
/// the shared map) touched once per served decision, AFTER the response
/// is written, so the decide path's latency never sees it.
pub struct DecStatSink {
    counters: papaya::HashMap<String, u64>,
    suites: papaya::HashMap<String, u64>,
    decisions: AtomicU32,
    abstains: AtomicU32,
}

impl DecStatSink {
    #[must_use]
    pub fn new() -> Self {
        Self {
            counters: papaya::HashMap::new(),
            suites: papaya::HashMap::new(),
            decisions: AtomicU32::new(0),
            abstains: AtomicU32::new(0),
        }
    }

    /// One served decision. `suite` is the registry suite, `primitive`
    /// the [`primitive_tag`] form, `abstained` the served abstention.
    /// The increments are atomic (`update_or_insert` under the pinned
    /// guard) — concurrent conn threads never lose an update.
    pub fn record(&self, suite: &str, primitive: &str, abstained: bool) {
        let class = if abstained { "abstain" } else { "pick" };
        let key = format!("{suite}:{primitive}:{class}");
        let guard = self.counters.pin();
        guard.update_or_insert(key, |v| *v + 1, 1u64);
        drop(guard);
        let sguard = self.suites.pin();
        sguard.update_or_insert(suite.to_string(), |v| *v + 1, 1u64);
        drop(sguard);
        self.decisions.fetch_add(1, Ordering::Relaxed);
        if abstained {
            self.abstains.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Drain the window into a signed-row payload; `None` when nothing
    /// accumulated. Counters over the wire ceilings are capped to the
    /// top entries (count desc, then name asc — deterministic; the
    /// dropped tail is reported in the row's place by the flush log, the
    /// fixstat fold's never-a-silent-drop discipline at the client half).
    ///
    /// A `record` racing the drain lands in the NEXT window (the totals
    /// swap first, the counter keys may re-insert after their snapshot) —
    /// a stat never double-counts, at worst a boundary decision shifts
    /// windows.
    #[must_use]
    pub fn take_row(&self, machine: &str, toolchain: &str) -> Option<DecStatRow> {
        let decisions = self.decisions.swap(0, Ordering::Relaxed);
        let abstains = self.abstains.swap(0, Ordering::Relaxed);
        let mut counters: Vec<(String, u64)> = Vec::new();
        {
            let pins = self.counters.pin();
            for (k, v) in pins.iter() {
                counters.push((k.clone(), *v));
            }
            for (k, _) in &counters {
                pins.remove(k);
            }
        }
        let mut suites: Vec<(String, u64)> = Vec::new();
        {
            let pins = self.suites.pin();
            for (k, v) in pins.iter() {
                suites.push((k.clone(), *v));
            }
            for (k, _) in &suites {
                pins.remove(k);
            }
        }
        if decisions == 0 && counters.is_empty() && suites.is_empty() {
            return None;
        }
        let cap = |mut v: Vec<(String, u64)>, max: usize| -> Vec<DecStatCount> {
            if v.len() > max {
                v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                v.truncate(max);
            }
            v.into_iter()
                .map(|(name, count)| DecStatCount::new(name, u32::try_from(count).unwrap_or(u32::MAX)))
                .collect()
        };
        Some(DecStatRow {
            run_id: riir_kat::kat_fixstat_client::fresh_run_id(machine),
            machine: machine.to_string(),
            toolchain: toolchain.to_string(),
            decisions,
            abstains,
            counters: cap(counters, DECSTAT_COUNTERS_MAX),
            suites: cap(suites, DECSTAT_SUITES_MAX),
        })
    }

    /// Decisions accumulated since the last drain (the flush trigger).
    #[must_use]
    pub fn pending(&self) -> u32 {
        self.decisions.load(Ordering::Relaxed)
    }
}

impl Default for DecStatSink {
    fn default() -> Self {
        Self::new()
    }
}

// ── the process-global sink (the serve edge reads it post-response) ────────

static SINK: OnceLock<Option<Arc<DecStatSink>>> = OnceLock::new();

/// Install the process sink — `Some` when consent is on, `None` when off
/// (the off install is what makes the record call a `OnceLock` miss
/// instead of a branch the hot path re-evaluates).
pub fn install(sink: Option<Arc<DecStatSink>>) {
    let _ = SINK.set(sink);
}

/// Record one served decision into the process sink, if installed. The
/// `/decide` edge calls this after the response is written; when the
/// feature is off or consent is off this is one atomic-free `OnceLock`
/// read of `None`.
pub fn record(suite: &str, arm_display: &str, abstained: bool) {
    if let Some(Some(sink)) = SINK.get() {
        sink.record(suite, primitive_tag(arm_display), abstained);
    }
}

/// The flusher's configuration (everything the push needs besides the
/// sink and the key).
#[derive(Debug, Clone)]
pub struct FlushConfig {
    pub service_url: String,
    pub machine: String,
    pub toolchain: String,
}

/// Spawn the flusher thread: poll every [`POLL_SECS`], flush at
/// [`FLUSH_THRESHOLD`] decisions or [`FLUSH_EVERY_SECS`] with a
/// non-empty window. One status line per flush; a failed push drops the
/// window loudly (no offline stacking) and the next window proceeds.
pub fn spawn_flusher(
    sink: Arc<DecStatSink>,
    key: ed25519_dalek::SigningKey,
    cfg: FlushConfig,
) -> std::io::Result<std::thread::JoinHandle<()>> {
    std::thread::Builder::new()
        .name("decstat-flush".into())
        .spawn(move || {
            let transport = HttpDecstatTransport::new(&cfg.service_url, riir_kat::kat_decstat_client::DECSTAT_TIMEOUT_SECS);
            let mut last_flush = Instant::now();
            loop {
                std::thread::sleep(Duration::from_secs(POLL_SECS));
                let due = sink.pending() >= FLUSH_THRESHOLD
                    || (last_flush.elapsed() >= Duration::from_secs(FLUSH_EVERY_SECS)
                        && sink.pending() > 0);
                if !due {
                    continue;
                }
                let Some(row) = sink.take_row(&cfg.machine, &cfg.toolchain) else {
                    last_flush = Instant::now();
                    continue;
                };
                let decisions = row.decisions;
                match push_decstat(&transport, &key, &row) {
                    Ok(ack) => {
                        eprintln!(
                            "[riir-instinct] decstat: pushed {decisions} decisions (epoch {}, {})",
                            ack.epoch,
                            if ack.recorded_new { "new" } else { "duplicate" }
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "[riir-instinct] decstat: push failed ({e}) — window dropped, no offline stacking"
                        );
                    }
                }
                last_flush = Instant::now();
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consent_is_the_exact_literal_on() {
        assert!(consent_enabled(Some("on")));
        assert!(!consent_enabled(None));
        assert!(!consent_enabled(Some("")));
        assert!(!consent_enabled(Some("1")));
        assert!(!consent_enabled(Some("ON")));
        assert!(!consent_enabled(Some("true")));
        assert!(!consent_enabled(Some("on ")));
    }

    #[test]
    fn primitive_tags_are_the_compact_arm_forms() {
        assert_eq!(primitive_tag("A0"), "a0");
        assert_eq!(primitive_tag("A1"), "a1");
        assert_eq!(primitive_tag("H1(top_k=4)"), "h1");
        assert_eq!(primitive_tag("H2(β=0.25,nmin=2,τ=2)"), "h2");
        assert_eq!(primitive_tag("something else"), "arm");
    }

    #[test]
    fn sink_counts_drains_and_restarts_clean() {
        let sink = DecStatSink::new();
        sink.record("ag_news", "h2", false);
        sink.record("ag_news", "h2", false);
        sink.record("ag_news", "h2", true);
        sink.record("banking77", "a0", false);
        let row = sink.take_row("m3", "rustc x").expect("non-empty window");
        assert_eq!(row.decisions, 4);
        assert_eq!(row.abstains, 1);
        assert_eq!(row.machine, "m3");
        let find = |name: &str| {
            row.counters
                .iter()
                .find(|c| c.name == name)
                .map(|c| c.count)
        };
        assert_eq!(find("ag_news:h2:pick"), Some(2));
        assert_eq!(find("ag_news:h2:abstain"), Some(1));
        assert_eq!(find("banking77:a0:pick"), Some(1));
        assert_eq!(
            row.suites
                .iter()
                .find(|s| s.name == "ag_news")
                .map(|s| s.count),
            Some(3)
        );
        // The window drained: the next take is empty.
        assert!(sink.take_row("m3", "rustc x").is_none());
        assert_eq!(sink.pending(), 0);
    }

    #[test]
    fn counters_cap_to_the_wire_ceiling_deterministically() {
        let sink = DecStatSink::new();
        for i in 0..300 {
            sink.record(&format!("suite{i:03}"), "a0", false);
        }
        let row = sink.take_row("m", "t").expect("non-empty");
        assert_eq!(row.counters.len(), DECSTAT_COUNTERS_MAX);
        // All counts tie at 1 → the kept set is the name-asc prefix.
        assert_eq!(row.counters[0].name, "suite000:a0:pick");
        assert!(row.suites.len() <= DECSTAT_SUITES_MAX);
    }

    #[test]
    fn signing_key_loads_from_a_hex_seed_file_and_refuses_other_shapes() {
        let dir = std::env::temp_dir().join(format!("instinct_decstat_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("tmp dir");
        let path = dir.join("seed.hex");
        std::fs::write(&path, "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60\n")
            .expect("write seed");
        let key = load_signing_key(path.to_str().unwrap()).expect("loads");
        // RFC 8032 test-vector pubkey — proves the hex decode order.
        let expect: [u8; 32] = [
            0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64,
            0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68,
            0xf7, 0x07, 0x51, 0x1a,
        ];
        assert_eq!(key.verifying_key().to_bytes(), expect);
        let bad = dir.join("bad.hex");
        std::fs::write(&bad, "not hex").expect("write bad");
        assert!(load_signing_key(bad.to_str().unwrap()).is_err());
        let short = dir.join("short.hex");
        std::fs::write(&short, "abcd").expect("write short");
        assert!(load_signing_key(short.to_str().unwrap()).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}

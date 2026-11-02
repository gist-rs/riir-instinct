//! The hosted-seat fair-use meter (Plan 326 T8–T12; the design contract
//! is `../riir-refine/.proposals/027_points_drain_fair_use.md` — "the
//! enforcement point is the seat"). Opt-in behind the `points_drain`
//! feature: the DEFAULT build (the public teaching posture) carries none
//! of this — the /decide path is byte-identical without the feature.
//!
//! Five surfaces, one module:
//!
//! - **T8 — the day counters** (REPORT-ONLY until ratified config
//!   exists): joined calls count under `(account_hex, utc_day)`, all
//!   unjoined traffic under `(source_hash, utc_day)`. The per-source key
//!   is a salted BLAKE3 hash of the client's /24 (IPv4) or /64 (IPv6)
//!   prefix — the salt rotates at the UTC day boundary (it is derived
//!   from the date + a random boot secret), lives IN MEMORY ONLY, and
//!   the hash is kept for THE DAY ONLY — never persisted past it (the
//!   plain-words disclosure beside the T16.1 client-UUID law lives in
//!   this repo's README).
//! - **T9 — the attestation verify**: the three `X-Points-*` headers
//!   bind the call to a contribution-points account without any
//!   registry — `blake3(pubkey) == account`, then a detached Ed25519
//!   signature over the wire crate's ONE signing-bytes home
//!   ([`riir_ledger::attest::attestation_signing_bytes`]). ANY failure
//!   (bad hex, hash mismatch, bad sig, replayed seq) → the
//!   ANONYMOUS posture, never an error response.
//!   ⚠ Deviation note (ratify in 027): the signed message embeds
//!   `call_seq`, which Ed25519 cannot message-recover — the client MUST
//!   also send it as a plain fourth header (`X-Points-Seq`), untrusted:
//!   the seat uses it only to RECONSTRUCT the signed payload, the
//!   signature authenticates it (a forged value fails the sig), and the
//!   seat reads the authoritative seq out of the VERIFIED payload bytes
//!   `[42..50]` (offset 32+10, u64 BE) for the replay check.
//! - **T10 — the replay window**: a FIXED `N = 128` bitmap per
//!   `(account, day)` (the IPsec anti-replay shape). Unseen-and-in-
//!   window accepts — out-of-order arrivals from concurrent runs
//!   ACCEPT (that is the point); seen or below-floor rejects (→
//!   anonymous posture); above-max accepts and slides.
//! - **T11 — the enforcement hooks**: config-gated OFF. The ratified
//!   numbers (joined allowance `A`, billable tier `B` at balance ≥ `R`,
//!   per-source budget `P`) arm from env that DOES NOT SHIP — absent or
//!   malformed config = report-only counting + verify, never a refusal,
//!   never a default-path change (G3). The joined/balance read rides
//!   the plane's `GET /points/usage-state` behind a TTL cache; a
//!   failed/absent answer demotes the caller to the unjoined posture.
//!   The tiered fail: counter-store outage ⇒ per-source traffic fails
//!   OPEN (served, uncounted), the billable tier fails CLOSED (demoted
//!   to the anonymous posture) — spend is never unbounded because the
//!   meter degraded.
//! - **T12 — the aggregate export**: a background thread (never on the
//!   /decide path) drains completed days at the UTC rollover and POSTs
//!   [`riir_ledger::dto::DrainUsageExport`] to `<plane>/points/usage`
//!   with the service bearer. Without a configured plane (or on any
//!   POST failure) the rows land in a LOCAL daily JSONL dump beside the
//!   seat's state dir — that dump is the `P`-sizing data. Counts are
//!   never lost either way.
//!
//! House-rule deviations, declared: `papaya` was removed at the
//! Proposal-052 carve (moat-only), so the counters run on std sync
//! primitives — a `try_lock` miss IS the store-down branch (the
//! tiered-fail law by construction) and at seat call volumes a held
//! mutex on an O(1) path is far under the decision latency.

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use riir_ledger::dto::{DrainUsageExport, DrainUsageRow, UsageState};

/// The drain attestation header names (lowercase comparison form; the
/// wire spells them `X-Points-Account` / `X-Points-Pubkey` /
/// `X-Points-Sig` — Appendix A — plus the `X-Points-Seq` transport
/// header, the module-note deviation).
pub const HDR_ACCOUNT: &str = "x-points-account";
pub const HDR_PUBKEY: &str = "x-points-pubkey";
pub const HDR_SEQ: &str = "x-points-seq";
pub const HDR_SIG: &str = "x-points-sig";

/// The replay-window size (Plan 326 T10 — the IPsec shape).
const REPLAY_WINDOW: u64 = 128;
/// Byte offset of `call_seq` inside the 82-byte signing payload
/// (32-byte account id + 10-byte ASCII day, u64 BIG-endian).
const SEQ_OFFSET: usize = 42;
/// The plane dial budget (the ~5 s budget law — the decide path never
/// blocks on the meter beyond this when enforcement is armed).
const PLANE_TIMEOUT: Duration = Duration::from_secs(5);
/// The response-read cap for the std HTTP client (a plane answer is a
/// few hundred bytes; anything larger is a protocol surprise).
const RESPONSE_CAP: usize = 1024 * 1024;
/// Memory caps for the day maps — a flood of unique keys degrades to
/// fail-open counting (serve, uncounted), never to unbounded memory.
const MAX_JOINED_ENTRIES: usize = 100_000;
const MAX_SOURCE_ENTRIES: usize = 1_000_000;
const MAX_WINDOWS: usize = 1_000_000;

// ─── UTC day helpers ─────────────────────────────────────────────────

/// Seconds since the Unix epoch (0 on clock failure — a degraded clock
/// counts under the epoch day, never panics).
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Howard Hinnant's `civil_from_days` — days since 1970-01-01 to
/// (year, month, day). Integer math only; no chrono dep.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// The `YYYY-MM-DD` UTC day for a Unix-epoch timestamp — the wire's
/// `utc_day` spelling (Appendix A).
#[must_use]
pub fn utc_day_from_secs(secs: u64) -> String {
    let (y, m, d) = civil_from_days((secs / 86_400) as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// The seat's current UTC day — the server clock is authoritative for
/// enforcement (a skewed client clock only skews its own pre-check).
#[must_use]
pub fn utc_day_now() -> String {
    utc_day_from_secs(now_secs())
}

// ─── hex helpers ─────────────────────────────────────────────────────

#[must_use]
pub fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn parse_hex_fixed(s: &str, out: &mut [u8]) -> bool {
    if s.len() != out.len() * 2 {
        return false;
    }
    let bytes = s.as_bytes();
    for (i, slot) in out.iter_mut().enumerate() {
        let hi = (bytes[2 * i] as char).to_digit(16);
        let lo = (bytes[2 * i + 1] as char).to_digit(16);
        match (hi, lo) {
            (Some(h), Some(l)) => *slot = ((h << 4) | l) as u8,
            _ => return false,
        }
    }
    true
}

// ─── the per-source key (T8 — the salted daily-rotating prefix hash) ─

/// The boot salt: 32 bytes from the OS randomness. IN MEMORY ONLY —
/// never persisted, never logged, never exported; it dies with the
/// process, which (with the day bound into the hash) is what makes the
/// per-source key un-linkable across days.
fn boot_salt() -> [u8; 32] {
    use std::io::Read as _;
    let mut salt = [0u8; 32];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        if f.read_exact(&mut salt).is_ok() {
            return salt;
        }
    }
    // Degraded fallback (non-Unix): hash the boot instant + pid. Still
    // process-local and day-rotating; never a persisted identity.
    let mut h = blake3::Hasher::new();
    h.update(&now_secs().to_be_bytes());
    h.update(&std::process::id().to_be_bytes());
    *h.finalize().as_bytes()
}

/// The meaningful prefix bytes: /24 for IPv4 (3 bytes), /64 for IPv6
/// (8 bytes) — the network, never the individual address.
fn source_prefix(ip: IpAddr) -> ([u8; 8], usize) {
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            ([o[0], o[1], o[2], 0, 0, 0, 0, 0], 3)
        }
        IpAddr::V6(v6) => {
            let o = v6.octets();
            ([o[0], o[1], o[2], o[3], o[4], o[5], o[6], o[7]], 8)
        }
    }
}

/// The per-source key for one day: `BLAKE3(salt ‖ utc_day ‖ prefix)`
/// truncated to 16 bytes, hex. The day in the message IS the rotation —
/// the same network hashes differently tomorrow, and yesterday's hash
/// is un-derivable once the salt is gone.
#[doc(hidden)]
#[must_use]
pub fn source_key(salt: &[u8; 32], day: &str, ip: IpAddr) -> String {
    let (prefix, len) = source_prefix(ip);
    let mut h = blake3::Hasher::new();
    h.update(salt);
    h.update(day.as_bytes());
    h.update(&prefix[..len]);
    hex_lower(&h.finalize().as_bytes()[..16])
}

/// The client IP the meter keys on. Trust posture: THE SEAT TRUSTS ITS
/// EDGE — a loopback peer's `X-Forwarded-For` carries the client IP the
/// edge saw (the container front), and only that spelling is honored.
/// A non-loopback peer keys on its own socket address; an unparseable
/// forwarded value falls back to the peer.
#[must_use]
pub fn resolve_client_ip(peer_ip: Option<IpAddr>, xff: Option<&str>) -> Option<IpAddr> {
    if let Some(xff) = xff {
        let from_loopback = peer_ip.is_some_and(|ip| ip.is_loopback());
        if from_loopback {
            if let Some(first) = xff.split(',').next() {
                if let Ok(ip) = first.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }
    peer_ip
}

// ─── T10 — the replay window (fixed N=128 bitmap) ────────────────────

/// One `(account, utc_day)` window: a fixed `[u64; 2]` bitmap over the
/// last 128 sequence numbers. Bit `i` (0..128) represents
/// `max_seen - i`; limb 0 holds the newest 64.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ReplayWindow {
    max_seen: u64,
    seen: [u64; 2],
}

impl ReplayWindow {
    #[must_use]
    pub fn new(first: u64) -> Self {
        let mut w = Self {
            max_seen: first,
            seen: [0, 0],
        };
        w.set_bit(first);
        w
    }

    /// `Some(age)` when `seq` is inside `[max_seen-127, max_seen]`,
    /// `None` when it is below the window floor (or unrepresentable).
    fn age(seq: u64, max_seen: u64) -> Option<u32> {
        let age = max_seen.checked_sub(seq)?;
        if age < REPLAY_WINDOW {
            Some(age as u32)
        } else {
            None
        }
    }

    fn set_bit(&mut self, seq: u64) {
        if let Some(a) = Self::age(seq, self.max_seen) {
            self.seen[(a / 64) as usize] |= 1u64 << (a % 64);
        }
    }

    /// The IPsec shape: unseen-and-in-window ACCEPTS (out-of-order
    /// arrivals from concurrent runs on one account — this workspace's
    /// ordinary posture — must land here); a seen seq or one below the
    /// window floor REJECTS (a replay → the anonymous posture); a seq
    /// above the seen max ACCEPTS and slides the window (wrap = 128
    /// distinct seqs).
    pub fn check_and_set(&mut self, seq: u64) -> bool {
        if seq > self.max_seen {
            let diff = seq - self.max_seen;
            self.seen = if diff >= REPLAY_WINDOW {
                [0, 0]
            } else {
                shift_bits_left(self.seen, diff as u32)
            };
            self.max_seen = seq;
            self.set_bit(seq);
            true
        } else if let Some(a) = Self::age(seq, self.max_seen) {
            let slot = &mut self.seen[(a / 64) as usize];
            let bit = 1u64 << (a % 64);
            if *slot & bit == 0 {
                *slot |= bit;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// 128-bit LEFT shift (limb 0 = least significant = the newest bits).
/// Bit `i` represents `max_seen - i`, so when the max advances by
/// `shift`, every existing bit's index GROWS by `shift` — a seen seq
/// must stay seen across the slide (a right shift would silently wipe
/// history and re-admit replays; the day-counters gate caught exactly
/// that).
fn shift_bits_left(bits: [u64; 2], shift: u32) -> [u64; 2] {
    debug_assert!(shift < 128);
    if shift == 0 {
        return bits;
    }
    if shift >= 64 {
        [0, bits[0] << (shift - 64)]
    } else {
        [
            bits[0] << shift,
            (bits[1] << shift) | (bits[0] >> (64 - shift)),
        ]
    }
}

// ─── T9 — the attestation verify ─────────────────────────────────────

/// The drain attestation headers, extracted verbatim by the serve edge.
/// All four are OPTIONAL — absence (or any failure below) is the
/// anonymous posture, never an error.
#[doc(hidden)]
#[derive(Debug, Clone, Default)]
pub struct AttestHeaders {
    /// `X-Points-Account` — `BLAKE3(pubkey)` hex (64 chars).
    pub account: Option<String>,
    /// `X-Points-Pubkey` — the raw Ed25519 public key, hex (64 chars).
    pub pubkey: Option<String>,
    /// `X-Points-Seq` — the call sequence, DECIMAL. The untrusted
    /// transport for payload reconstruction; see the module deviation
    /// note (Ed25519 cannot message-recover, so the seat needs the seq
    /// to rebuild the signed bytes — the signature authenticates it).
    pub seq: Option<String>,
    /// `X-Points-Sig` — the detached Ed25519 signature, hex (128 chars).
    pub sig: Option<String>,
}

/// The verify outcome. `Joined` carries the account hex (lowercase) and
/// the seq recovered FROM THE VERIFIED PAYLOAD (never the header).
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attested {
    Joined { account_hex: String, seq: u64 },
    Anonymous,
}

/// Verify the per-call attestation (T9). ANY failure class — missing
/// header, bad hex, `blake3(pubkey) != account`, malformed seq, bad
/// signature — returns [`Attested::Anonymous`]. This fn NEVER errors
/// and NEVER panics on attacker-controlled input.
#[doc(hidden)]
#[must_use]
pub fn verify_attestation(headers: Option<&AttestHeaders>, body: &[u8], utc_day: &str) -> Attested {
    let Some(h) = headers else {
        return Attested::Anonymous;
    };
    let (Some(account_hex), Some(pubkey_hex), Some(sig_hex), Some(seq_text)) =
        (&h.account, &h.pubkey, &h.sig, &h.seq)
    else {
        return Attested::Anonymous;
    };
    let mut account = [0u8; 32];
    let mut pubkey = [0u8; 32];
    let mut sig = [0u8; 64];
    if !parse_hex_fixed(account_hex, &mut account)
        || !parse_hex_fixed(pubkey_hex, &mut pubkey)
        || !parse_hex_fixed(sig_hex, &mut sig)
    {
        return Attested::Anonymous;
    }
    let Ok(seq_from_header) = seq_text.parse::<u64>() else {
        return Attested::Anonymous;
    };
    // The id is bound to the key that presents it — a fresh unregistered
    // key verifies fine; nobody can attribute calls to someone else's
    // account (anti-spoof), and the body hash ties the signature to the
    // exact request it authenticates.
    if blake3::hash(&pubkey).as_bytes() != &account {
        return Attested::Anonymous;
    }
    // The ONE signing-bytes home (the wire crate — zero local
    // re-derivation). The seq rides in from the untrusted header ONLY
    // to complete the payload; the signature below authenticates it.
    let body_hash = blake3::hash(body);
    let payload = riir_ledger::attest::attestation_signing_bytes(
        &account,
        utc_day,
        seq_from_header,
        body_hash.as_bytes(),
    );
    let Ok(vk) = VerifyingKey::from_bytes(&pubkey) else {
        return Attested::Anonymous;
    };
    let Ok(signature) = Signature::from_slice(&sig) else {
        return Attested::Anonymous;
    };
    if vk.verify(&payload, &signature).is_err() {
        return Attested::Anonymous;
    }
    // Signature verified — the payload bytes are now AUTHENTICATED.
    // Read the authoritative seq out of the verified payload (offset
    // 32+10, u64 BE); the header's copy was only the transport.
    let Some(seq_bytes) = payload
        .get(SEQ_OFFSET..SEQ_OFFSET + 8)
        .and_then(|s| <[u8; 8]>::try_from(s).ok())
    else {
        return Attested::Anonymous;
    };
    Attested::Joined {
        account_hex: account_hex.to_ascii_lowercase(),
        seq: u64::from_be_bytes(seq_bytes),
    }
}

// ─── T8 — the day counters ───────────────────────────────────────────

/// The day-key shape shared by every counter map: `(identity, utc_day)`
/// where identity is the lowercase account hex or the per-source hash.
type DayKey = (String, String);
type JoinedMap = HashMap<DayKey, JoinCounts>;
type SourceMap = HashMap<DayKey, u64>;
type WindowMap = HashMap<DayKey, ReplayWindow>;

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy)]
pub struct JoinCounts {
    pub free: u64,
    pub billable: u64,
}

/// One per-source day row (the LOCAL dump's P-sizing line; never
/// exported to the plane — the rotated salt makes cross-day
/// aggregation meaningless by construction).
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceCount {
    pub utc_day: String,
    pub source_hash: String,
    pub calls: u64,
}

#[derive(Debug, Default)]
pub struct DayCounters {
    joined: Mutex<JoinedMap>,
    source: Mutex<SourceMap>,
}

/// The admission decision inside the counters (pub(crate) — the caller
/// sees only the [`Verdict`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Admit {
    AdmittedFree,
    AdmittedBillable,
    Exhausted,
    StoreDown,
    CapReached,
}

impl DayCounters {
    /// Atomic check-and-increment for a joined caller (T11): free
    /// allowance first, then the billable tier while `balance ≥ R` and
    /// `B` is not exhausted, else exhausted. One lock hold — no
    /// check/increment race.
    fn try_admit_joined(
        &self,
        account: &str,
        day: &str,
        b: &Budgets,
        balance: Option<u64>,
    ) -> Admit {
        let Ok(mut map) = self.joined.try_lock() else {
            return Admit::StoreDown;
        };
        let key = (account.to_string(), day.to_string());
        if map.len() >= MAX_JOINED_ENTRIES && !map.contains_key(&key) {
            return Admit::CapReached;
        }
        let entry = map.entry(key).or_default();
        if entry.free < b.allowance_a {
            entry.free += 1;
            return Admit::AdmittedFree;
        }
        if let Some(balance) = balance {
            if balance >= b.rate_r && entry.billable < b.billable_b {
                entry.billable += 1;
                return Admit::AdmittedBillable;
            }
        }
        Admit::Exhausted
    }

    /// Atomic check-and-increment for the per-source budget `P`
    /// (shared by ALL unjoined callers — signed or not, the
    /// consistency law: a throwaway keygen must never out-draw an
    /// un-attested caller).
    fn try_admit_source(&self, key: &str, day: &str, p: u64) -> Admit {
        let Ok(mut map) = self.source.try_lock() else {
            return Admit::StoreDown;
        };
        let key = (key.to_string(), day.to_string());
        if map.len() >= MAX_SOURCE_ENTRIES && !map.contains_key(&key) {
            return Admit::CapReached;
        }
        let used = map.entry(key).or_insert(0);
        if *used < p {
            *used += 1;
            Admit::AdmittedFree
        } else {
            Admit::Exhausted
        }
    }

    /// REPORT-ONLY increment (T8, config absent): count the joined
    /// call under its free bucket, never refuse, never dial the plane.
    fn count_joined_free(&self, account: &str, day: &str) {
        let Ok(mut map) = self.joined.try_lock() else {
            return;
        };
        let key = (account.to_string(), day.to_string());
        if map.len() >= MAX_JOINED_ENTRIES && !map.contains_key(&key) {
            return;
        }
        map.entry(key).or_default().free += 1;
    }

    /// REPORT-ONLY increment for unjoined traffic.
    fn count_source(&self, key: &str, day: &str) {
        let Ok(mut map) = self.source.try_lock() else {
            return;
        };
        let key = (key.to_string(), day.to_string());
        if map.len() >= MAX_SOURCE_ENTRIES && !map.contains_key(&key) {
            return;
        }
        *map.entry(key).or_insert(0) += 1;
    }

    /// Remove every entry whose day completed (`entry_day < today`,
    /// lexicographic on `YYYY-MM-DD` = chronological). The counts leave
    /// the map exactly once — the exporter (plane POST or local dump)
    /// is the only reader.
    fn drain_completed(&self, today: &str) -> (Vec<DrainUsageRow>, Vec<SourceCount>) {
        let mut rows = Vec::new();
        let mut sources = Vec::new();
        if let Ok(mut map) = self.joined.lock() {
            let expired: Vec<DayKey> = map
                .keys()
                .filter(|(_, d)| d.as_str() < today)
                .cloned()
                .collect();
            for key in expired {
                if let Some(c) = map.remove(&key) {
                    rows.push(DrainUsageRow {
                        account_id_hex: key.0,
                        utc_day: key.1,
                        free_calls: c.free,
                        billable_calls: c.billable,
                    });
                }
            }
        }
        if let Ok(mut map) = self.source.lock() {
            let expired: Vec<DayKey> = map
                .keys()
                .filter(|(_, d)| d.as_str() < today)
                .cloned()
                .collect();
            for key in expired {
                if let Some(calls) = map.remove(&key) {
                    sources.push(SourceCount {
                        utc_day: key.1,
                        source_hash: key.0,
                        calls,
                    });
                }
            }
        }
        (rows, sources)
    }
}

// ─── T11 — the config seam (ABSENT by default) ───────────────────────

/// The ratified enforcement numbers — served by the plane, mirrored in
/// seat env at ratification. ABSENT = report-only (the shipped posture).
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budgets {
    /// `A` — the joined personal free allowance per UTC day.
    pub allowance_a: u64,
    /// `B` — the billable hosted calls per UTC day (balance ≥ `R`).
    pub billable_b: u64,
    /// `R` — the billable-day fee threshold on the (TTL-cached) balance.
    pub rate_r: u64,
    /// `P` — the per-source daily budget shared by all unjoined callers.
    pub per_source_p: u64,
}

impl Budgets {
    /// Env seam: `INSTINCT_DRAIN_ENFORCE` arms, and all four numbers
    /// must parse. Absent OR malformed ⇒ `None` (report-only) — the
    /// plan's own rule, loud once so an operator notices a typo.
    fn from_env() -> Option<Self> {
        let armed = std::env::var("INSTINCT_DRAIN_ENFORCE")
            .ok()
            .is_some_and(|v| matches!(v.trim(), "1" | "true" | "on"));
        if !armed {
            return None;
        }
        match (
            env_u64("INSTINCT_DRAIN_ALLOWANCE_A"),
            env_u64("INSTINCT_DRAIN_BILLABLE_B"),
            env_u64("INSTINCT_DRAIN_RATE_R"),
            env_u64("INSTINCT_DRAIN_PER_SOURCE_P"),
        ) {
            (Some(allowance_a), Some(billable_b), Some(rate_r), Some(per_source_p)) => Some(Self {
                allowance_a,
                billable_b,
                rate_r,
                per_source_p,
            }),
            _ => {
                eprintln!(
                    "[riir-instinct] points drain: INSTINCT_DRAIN_ENFORCE is set but \
                     A/B/R/P are missing or malformed — running REPORT-ONLY"
                );
                None
            }
        }
    }
}

/// The points-plane service credentials (base URL + bearer). The plane
/// carries the daily aggregate POST and the usage-state reads.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct PlaneCreds {
    pub base_url: String,
    pub bearer: String,
}

impl PlaneCreds {
    fn from_env() -> Option<Self> {
        let base_url = std::env::var("INSTINCT_DRAIN_PLANE_URL").ok()?;
        let bearer = std::env::var("INSTINCT_DRAIN_PLANE_TOKEN").ok()?;
        if base_url.trim().is_empty() || bearer.trim().is_empty() {
            return None;
        }
        Some(Self {
            base_url: base_url.trim().to_string(),
            bearer,
        })
    }
}

/// The meter's whole configuration — [`MeterConfig::from_env`] is the
/// served boot; everything defaults to the inert/report-only posture.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct MeterConfig {
    /// `Some` only when the ratified numbers exist in env (absent ships).
    pub enforcement: Option<Budgets>,
    /// `Some` only when plane URL + bearer exist in env (absent ships).
    pub plane: Option<PlaneCreds>,
    /// The enforced-side TTL on the plane's joined+balance answer —
    /// staleness here is the disclosed bounded leak (027 caveat 2).
    pub usage_ttl: Duration,
    /// Where the local daily JSONL dump lands (also the POST-failure
    /// fallback) — the P-sizing data.
    pub dump_dir: PathBuf,
}

impl MeterConfig {
    #[must_use]
    pub fn from_env() -> Self {
        let usage_ttl =
            Duration::from_secs(env_u64("INSTINCT_DRAIN_USAGE_TTL_SECS").unwrap_or(300));
        let dump_dir = PathBuf::from(
            std::env::var("INSTINCT_DRAIN_DUMP_DIR")
                .ok()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "drain_dump".to_string()),
        );
        Self {
            enforcement: Budgets::from_env(),
            plane: PlaneCreds::from_env(),
            usage_ttl,
            dump_dir,
        }
    }
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
}

// ─── T12 — the std-only HTTP client (http:// plane dials) ────────────

/// Split `http://host[:port]/path` — the ONLY scheme supported. The
/// plane is dialed over the container/loopback network or fronted with
/// TLS at the edge; there is no TLS client in this repo (std-only law).
#[doc(hidden)]
pub fn split_http_url(url: &str) -> Result<(String, String), String> {
    let Some(rest) = url.strip_prefix("http://") else {
        return Err("only http:// plane URLs are supported — dial the plane over the container/loopback network or front it with TLS at the edge".to_string());
    };
    let (hostport, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    if hostport.is_empty() {
        return Err("empty host in plane URL".to_string());
    }
    Ok((hostport.to_string(), path.to_string()))
}

fn parse_status(head: &str) -> Option<u16> {
    head.split_whitespace().nth(1)?.parse().ok()
}

fn http_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    bearer: Option<&str>,
    timeout: Duration,
) -> Result<(u16, String), String> {
    let (hostport, path) = split_http_url(url)?;
    let stream =
        TcpStream::connect(hostport.as_str()).map_err(|e| format!("connect {hostport}: {e}"))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    let mut req = format!("{method} {path} HTTP/1.1\r\nHost: {hostport}\r\nConnection: close\r\n");
    if let Some(bearer) = bearer {
        req.push_str(&format!("Authorization: Bearer {bearer}\r\n"));
    }
    if let Some(body) = body {
        req.push_str("Content-Type: application/json\r\n");
        req.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    req.push_str("\r\n");
    (&stream)
        .write_all(req.as_bytes())
        .map_err(|e| format!("write request: {e}"))?;
    if let Some(body) = body {
        (&stream)
            .write_all(body.as_bytes())
            .map_err(|e| format!("write body: {e}"))?;
    }
    (&stream).flush().map_err(|e| format!("flush: {e}"))?;
    read_response(&stream)
}

fn read_response(mut stream: &TcpStream) -> Result<(u16, String), String> {
    let mut buf = Vec::with_capacity(4096);
    let mut chunk = [0u8; 4096];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                if buf.len() > RESPONSE_CAP {
                    return Err("plane response exceeded the read cap".to_string());
                }
            }
            Err(e) => return Err(format!("read response: {e}")),
        }
    }
    let text = String::from_utf8_lossy(&buf);
    let (head, body) = match text.split_once("\r\n\r\n") {
        Some((h, b)) => (h, b),
        None => (text.as_ref(), ""),
    };
    let status = parse_status(head).ok_or_else(|| "unparseable plane status line".to_string())?;
    Ok((status, body.to_string()))
}

/// POST JSON with the optional bearer; returns the HTTP status.
#[doc(hidden)]
pub fn http_post_json(
    url: &str,
    bearer: Option<&str>,
    body: &str,
    timeout: Duration,
) -> Result<u16, String> {
    http_request("POST", url, Some(body), bearer, timeout).map(|(status, _)| status)
}

/// GET JSON with the optional bearer; returns `(status, body)`.
#[doc(hidden)]
pub fn http_get_json(
    url: &str,
    bearer: Option<&str>,
    timeout: Duration,
) -> Result<(u16, String), String> {
    http_request("GET", url, None, bearer, timeout)
}

// ─── the meter ───────────────────────────────────────────────────────

/// The verdict the /decide hook acts on. `Refuse` fires ONLY when the
/// ratified enforcement config exists (it does not ship pre-ratification).
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Refuse(String),
}

fn exhausted_message() -> String {
    "daily hosted fair-use budget reached — local healing is unaffected; \
     contribute contribution points for hosted runway or retry tomorrow"
        .to_string()
}

/// The seat-side fair-use meter (T8 counters + T9 verify + T10 replay
/// windows + T11 enforcement + T12 export).
#[doc(hidden)]
pub struct Meter {
    salt: [u8; 32],
    config: MeterConfig,
    counters: DayCounters,
    windows: Mutex<WindowMap>,
    usage: Mutex<HashMap<String, (UsageState, Instant)>>,
    /// The UTC day the exporter last ran for (days since epoch).
    last_export_day: AtomicU64,
}

static METER: OnceLock<Meter> = OnceLock::new();

impl Meter {
    /// The served boot: salt from the OS, config from env. ABSENT env
    /// is the shipped posture — report-only counting + verify.
    #[must_use]
    pub fn boot() -> Self {
        Self::with_parts(boot_salt(), MeterConfig::from_env())
    }

    #[doc(hidden)]
    #[must_use]
    pub fn with_parts(salt: [u8; 32], config: MeterConfig) -> Self {
        Self {
            salt,
            config,
            counters: DayCounters::default(),
            windows: Mutex::new(HashMap::new()),
            usage: Mutex::new(HashMap::new()),
            last_export_day: AtomicU64::new(now_secs() / 86_400),
        }
    }

    /// The /decide hook's whole body (T8+T9+T10+T11 in one call):
    /// verify → replay window → classify → count → (armed) enforce.
    /// Never errors; a `Refuse` only exists when enforcement is armed.
    pub fn evaluate(
        &self,
        client_ip: Option<IpAddr>,
        headers: Option<&AttestHeaders>,
        body: &[u8],
        day: &str,
    ) -> Verdict {
        let joined_account = match verify_attestation(headers, body, day) {
            Attested::Anonymous => None,
            Attested::Joined { account_hex, seq } => {
                // T10: a replayed sequence is treated as unjoined —
                // still served, counted per-source.
                if self.window_accept(&account_hex, day, seq) {
                    Some(account_hex)
                } else {
                    None
                }
            }
        };
        match joined_account {
            Some(account_hex) => self.admit_joined(&account_hex, day, client_ip),
            None => self.admit_source(client_ip, day),
        }
    }

    fn window_accept(&self, account_hex: &str, day: &str, seq: u64) -> bool {
        let Ok(mut map) = self.windows.try_lock() else {
            return true; // store down: fail open — the caller is never broken
        };
        if map.len() >= MAX_WINDOWS {
            return true;
        }
        match map.entry((account_hex.to_string(), day.to_string())) {
            Entry::Occupied(mut e) => e.get_mut().check_and_set(seq),
            Entry::Vacant(e) => {
                e.insert(ReplayWindow::new(seq));
                true
            }
        }
    }

    fn admit_joined(&self, account_hex: &str, day: &str, client_ip: Option<IpAddr>) -> Verdict {
        let Some(budgets) = self.config.enforcement else {
            // T8 report-only (the shipped posture): count, never refuse,
            // never dial the plane for a balance.
            self.counters.count_joined_free(account_hex, day);
            return Verdict::Pass;
        };
        // The billable tier needs the plane's joined+balance answer. A
        // failed/absent lookup demotes the caller to the unjoined
        // posture — never an error, never unbounded spend.
        let Some(state) = self.usage_lookup(account_hex) else {
            return self.admit_source(client_ip, day);
        };
        if !state.joined {
            return self.admit_source(client_ip, day);
        }
        match self
            .counters
            .try_admit_joined(account_hex, day, &budgets, Some(state.balance))
        {
            Admit::AdmittedFree | Admit::AdmittedBillable => Verdict::Pass,
            Admit::Exhausted => Verdict::Refuse(exhausted_message()),
            // Tiered fail: the counter store is unavailable — the
            // billable tier FAILS CLOSED (the caller is demoted to the
            // anonymous posture) and the caller is never broken.
            Admit::StoreDown | Admit::CapReached => Verdict::Pass,
        }
    }

    fn admit_source(&self, client_ip: Option<IpAddr>, day: &str) -> Verdict {
        let key = self.source_key_for(client_ip, day);
        let Some(budgets) = self.config.enforcement else {
            self.counters.count_source(&key, day);
            return Verdict::Pass;
        };
        match self
            .counters
            .try_admit_source(&key, day, budgets.per_source_p)
        {
            Admit::AdmittedFree | Admit::AdmittedBillable => Verdict::Pass,
            Admit::Exhausted => Verdict::Refuse(exhausted_message()),
            // Tiered fail: per-source traffic FAILS OPEN — the coarse
            // allowance family serves, the caller is never broken by the
            // meter (a `CapReached` flood degrades the same way).
            Admit::StoreDown | Admit::CapReached => Verdict::Pass,
        }
    }

    fn source_key_for(&self, ip: Option<IpAddr>, day: &str) -> String {
        match ip {
            Some(ip) => source_key(&self.salt, day, ip),
            // No socket address available — a fixed non-identifying
            // bucket so the call is still counted (never an identity).
            None => "ip-unavailable".to_string(),
        }
    }

    /// The joined+balance read behind the TTL cache. A hit answers
    /// without the network; a miss dials `GET /points/usage-state`
    /// once. A failed/absent/version-mismatched answer is `None` — the
    /// caller demotes to the unjoined posture (a failed read is never
    /// cached, so recovery is the next call).
    fn usage_lookup(&self, account_hex: &str) -> Option<UsageState> {
        let plane = self.config.plane.as_ref()?;
        if let Ok(map) = self.usage.lock() {
            if let Some((state, fetched_at)) = map.get(account_hex) {
                if fetched_at.elapsed() < self.config.usage_ttl {
                    return Some(state.clone());
                }
            }
        }
        let url = format!(
            "{}/points/usage-state?account={account_hex}",
            plane.base_url.trim_end_matches('/')
        );
        let (status, body) = http_get_json(&url, Some(&plane.bearer), PLANE_TIMEOUT).ok()?;
        if status != 200 {
            return None;
        }
        let state: UsageState = serde_json::from_str(&body).ok()?;
        // Fail-safe: a version or account mismatch is a failed read.
        if state.format_version != riir_ledger::FORMAT_VERSION
            || state.account_id_hex != account_hex
        {
            return None;
        }
        if let Ok(mut map) = self.usage.lock() {
            map.insert(account_hex.to_string(), (state.clone(), Instant::now()));
        }
        Some(state)
    }

    // ── T12 — the export ──

    /// One exporter tick: on a UTC-day change, drain every completed
    /// day and deliver it (plane POST, else local dump). Best-effort,
    /// background, never on the /decide path.
    pub(crate) fn export_cycle(&self) {
        let today_days = now_secs() / 86_400;
        if self.last_export_day.load(Ordering::Acquire) == today_days {
            return;
        }
        let today = utc_day_from_secs(today_days * 86_400);
        let (rows, sources) = self.counters.drain_completed(&today);
        self.prune_windows(&today);
        if !rows.is_empty() || !sources.is_empty() {
            self.export_rows(&rows, &sources);
        }
        self.last_export_day.store(today_days, Ordering::Release);
    }

    /// Deliver rows: POST [`DrainUsageExport`] to `<plane>/points/usage`
    /// with the service bearer when configured; on ANY failure (or no
    /// plane) append to the LOCAL daily JSONL dump — counts are never
    /// lost. Returns true when the plane accepted.
    #[doc(hidden)]
    pub fn export_rows(&self, rows: &[DrainUsageRow], sources: &[SourceCount]) -> bool {
        let mut delivered = false;
        if let Some(plane) = &self.config.plane {
            if rows.is_empty() {
                delivered = true;
            } else {
                let export = DrainUsageExport {
                    format_version: riir_ledger::FORMAT_VERSION,
                    rows: rows.to_vec(),
                };
                match serde_json::to_string(&export) {
                    Ok(body) => {
                        let url = format!("{}/points/usage", plane.base_url.trim_end_matches('/'));
                        match http_post_json(&url, Some(&plane.bearer), &body, PLANE_TIMEOUT) {
                            Ok(status) if (200..300).contains(&status) => delivered = true,
                            Ok(status) => eprintln!(
                                "[riir-instinct] points drain: the plane answered HTTP \
                                 {status} — keeping the day's rows in the local dump"
                            ),
                            Err(e) => eprintln!(
                                "[riir-instinct] points drain: the plane is unreachable \
                                 ({e}) — keeping the day's rows in the local dump"
                            ),
                        }
                    }
                    Err(e) => eprintln!(
                        "[riir-instinct] points drain: export serialize failed ({e}) — \
                         keeping the day's rows in the local dump"
                    ),
                }
            }
        }
        if !delivered {
            if let Err(e) = self.dump_rows(rows, sources) {
                eprintln!(
                    "[riir-instinct] points drain: LOCAL DUMP FAILED ({e}) — counts \
                     lost for this cycle"
                );
            }
        }
        delivered
    }

    /// The local daily JSONL dump (the P-sizing data): one
    /// [`DrainUsageRow`] per line, then the per-source counts as
    /// [`SourceCount`] lines — one file per UTC day present in the
    /// batch, append-mode (a retried cycle never duplicates a plane
    /// delivery, and a failed one never loses the first write).
    fn dump_rows(&self, rows: &[DrainUsageRow], sources: &[SourceCount]) -> std::io::Result<()> {
        use std::io::Write as _;
        std::fs::create_dir_all(&self.config.dump_dir)?;
        let mut days: BTreeSet<&str> = BTreeSet::new();
        days.extend(rows.iter().map(|r| r.utc_day.as_str()));
        days.extend(sources.iter().map(|s| s.utc_day.as_str()));
        for day in days {
            let path = self
                .config
                .dump_dir
                .join(format!("drain_usage_{day}.jsonl"));
            let mut out = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            for row in rows.iter().filter(|r| r.utc_day == day) {
                let line = serde_json::to_string(row).map_err(std::io::Error::other)?;
                writeln!(out, "{line}")?;
            }
            for src in sources.iter().filter(|s| s.utc_day == day) {
                let line = serde_json::to_string(src).map_err(std::io::Error::other)?;
                writeln!(out, "{line}")?;
            }
        }
        Ok(())
    }

    fn prune_windows(&self, today: &str) {
        if let Ok(mut map) = self.windows.lock() {
            map.retain(|(_, d), _| d.as_str() >= today);
        }
    }

    // ── gate peekers (doc-hidden; the integration gates drive these) ──

    #[doc(hidden)]
    #[must_use]
    pub fn joined_count(&self, account: &str, day: &str) -> Option<(u64, u64)> {
        let map = self.counters.joined.lock().ok()?;
        map.get(&(account.to_string(), day.to_string()))
            .map(|c| (c.free, c.billable))
    }

    #[doc(hidden)]
    #[must_use]
    pub fn source_count(&self, key: &str, day: &str) -> Option<u64> {
        let map = self.counters.source.lock().ok()?;
        map.get(&(key.to_string(), day.to_string())).copied()
    }

    #[doc(hidden)]
    #[must_use]
    pub fn drain_completed_for_test(&self, today: &str) -> (Vec<DrainUsageRow>, Vec<SourceCount>) {
        self.counters.drain_completed(today)
    }

    /// The raw store handles — the tiered-fail gate poisons one to
    /// simulate a counter-store outage.
    #[doc(hidden)]
    pub fn debug_counter_stores(&self) -> (&Mutex<JoinedMap>, &Mutex<SourceMap>) {
        (&self.counters.joined, &self.counters.source)
    }
}

/// Spawn the daily aggregate exporter (T12) — idempotent, best-effort,
/// a 60 s tick that acts at the UTC rollover. Never the /decide path.
pub fn ensure_exporter() {
    static STARTED: OnceLock<()> = OnceLock::new();
    if STARTED.set(()).is_ok() {
        let _ = std::thread::Builder::new()
            .name("points-drain-export".into())
            .spawn(|| loop {
                std::thread::sleep(Duration::from_secs(60));
                if let Some(meter) = METER.get() {
                    meter.export_cycle();
                }
            });
    }
}

/// The /decide hook (T8–T12): the serve edge calls this once per
/// accepted POST /decide body. Boots the global meter lazily (env
/// config, ABSENT = report-only) and starts the exporter thread on
/// first use.
#[must_use]
pub fn observe_decide(
    client_ip: Option<IpAddr>,
    headers: Option<&AttestHeaders>,
    body: &[u8],
) -> Verdict {
    let meter = METER.get_or_init(Meter::boot);
    ensure_exporter();
    let day = utc_day_now();
    meter.evaluate(client_ip, headers, body, &day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_day_helpers() {
        assert_eq!(utc_day_from_secs(0), "1970-01-01");
        assert_eq!(civil_from_days(11_017), (2000, 3, 1));
        // 2026-11-02 (the appendix-A example day) = day 20759.
        assert_eq!(civil_from_days(20_759), (2026, 11, 2));
        assert_eq!(utc_day_from_secs(20_759 * 86_400), "2026-11-02");
    }

    #[test]
    fn hex_roundtrip() {
        assert_eq!(hex_lower(&[0xde, 0xad, 0x01]), "dead01");
        let mut out = [0u8; 3];
        assert!(parse_hex_fixed("dead01", &mut out));
        assert_eq!(out, [0xde, 0xad, 0x01]);
        assert!(!parse_hex_fixed("dead0", &mut out));
        assert!(!parse_hex_fixed("zzzz01", &mut out));
    }

    #[test]
    fn window_first_call_accepts_and_repeats_reject() {
        let mut w = ReplayWindow::new(7);
        assert!(!w.check_and_set(7), "the init seq is already seen");
        assert!(w.check_and_set(6));
        assert!(!w.check_and_set(6));
    }

    #[test]
    fn split_url_shapes() {
        assert_eq!(
            split_http_url("http://127.0.0.1:9/points/usage").unwrap(),
            ("127.0.0.1:9".to_string(), "/points/usage".to_string())
        );
        assert_eq!(
            split_http_url("http://plane:8080").unwrap(),
            ("plane:8080".to_string(), "/".to_string())
        );
        assert!(split_http_url("https://plane/points/usage").is_err());
    }
}

//! The arsenal manifest (Proposal 001 T1/T2) — the ONE selection surface
//! for the hosted serving lane (law A5): suite → artifact digest → class
//! → serving posture → pins → budget, one `arsenal.toml` per
//! HOST/deployment. The hard-coded posture match table and the
//! `REGISTERED_SUITES` const it replaces are deleted; from here on the
//! posture is DATA.
//!
//! Boot discipline (T1):
//!
//! 1. **Parse** — a malformed manifest refuses loud (the toml error names
//!    the field; `deny_unknown_fields` turns a typo into a refusal, never
//!    a silently-ignored row).
//! 2. **Validate** — before any lane resolves: digest format, class
//!    vocabulary, posture arm + params (an unknown arm refuses, NEVER
//!    defaulted — law A6), budget policy, duplicate suites, pin-key
//!    resolution (vessel mode), and manifest ↔ files on disk (artifact
//!    BLAKE3 must match the pinned digest; class-vs-reader-capability is
//!    checked against the artifact's SIGNED header up front, so a row the
//!    public reader would refuse at `open()` fails as a manifest-time
//!    loud refusal instead of a mid-boot surprise). An ABSENT artifact
//!    file is left to the lane loader's existing error path — the
//!    dataless dev posture (a bare clone skips loud); drift is checked
//!    wherever the bytes exist.
//! 3. **Pin** — the embedded default manifest's bytes are pinned by
//!    `tests/serve_gates.rs` (law A6): a TOML edit reds exactly like a
//!    code edit does.

use std::collections::HashSet;
use std::path::Path;

use serde::Deserialize;

use crate::server::Arm;

/// The default manifest — the embedded copy of the repo-root
/// `arsenal.toml` (the frozen Bench-002 verdicts, raw-winner mode).
pub const EMBEDDED_MANIFEST: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/arsenal.toml"));

/// The hosted payload cap, MiB — must equal reflexer-vessel's
/// `MAX_HOSTED_PAYLOAD >> 20` (asserted by a unit test under the `vessel`
/// feature, the only posture where the format crate is in the graph).
pub const MAX_HOSTED_PAYLOAD_MB: u64 = 16;

/// The digest algorithm tag every row pins with.
const DIGEST_TAG: &str = "blake3:";

/// The whole manifest: one `[[vessel]]` row per served suite.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArsenalManifest {
    #[serde(default)]
    pub vessel: Vec<VesselRow>,
}

/// One selection row (Proposal 001's `[[vessel]]` sketch, adapted to the
/// actual code shapes — reflexer-vessel pins are u32 key-ids, not string
/// labels).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VesselRow {
    /// The served suite (the reflex harness seat name).
    pub suite: String,
    /// `"blake3:<64 hex>"` of the artifact file this row loads.
    pub digest: String,
    /// `"public_release" | "hosted_only"` — the hosted lane refuses
    /// `public_release` rows loud (the moat law).
    pub class: String,
    /// The serving arm WITH its params.
    pub posture: PostureSpec,
    /// reflexer-vessel PinTable key ids this row accepts.
    #[serde(default)]
    pub pin_keys: Vec<u32>,
    /// The load budget (enforcement lands with T5; validated now).
    pub budget: BudgetSpec,
    /// OPTIONAL artifact filename override (bare filename). Default: the
    /// established convention — `<suite>_winner_v1.bin` raw /
    /// `<suite>_v1.vessel` vessel.
    pub file: Option<String>,
}

/// The serving arm + params (`{ arm = "H2", beta = 1.0, … }` — arms carry
/// params, never a bare string).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostureSpec {
    pub arm: String,
    pub top_k: Option<usize>,
    pub beta: Option<f64>,
    pub n_min: Option<f64>,
    pub tau_n: Option<f64>,
}

/// The load budget for the row's artifact.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BudgetSpec {
    /// `"eager"` (boot loads the lane — the default manifest's posture) |
    /// `"lazy"` (validated now; the lazy/budgeted loader is T5).
    pub load: String,
    /// Artifact size ceiling, MiB (the hosted cap is
    /// [`MAX_HOSTED_PAYLOAD_MB`]).
    pub max_payload_mb: u64,
}

/// What the boot validates against — the deployment's artifact source and
/// (vessel mode) the operator pin table. Construct with [`ValidateCtx::raw`]
/// or [`ValidateCtx::vessel`].
pub struct ValidateCtx<'a> {
    /// The raw sealed-winner dir (raw mode, and carried in vessel mode
    /// for error messages).
    pub winners_dir: &'a Path,
    /// `Some` = vessel mode (the dir the minted vessels load from).
    pub vessels_dir: Option<&'a Path>,
    /// The operator pin table (vessel mode only).
    #[cfg(feature = "vessel")]
    pub pins: Option<&'a reflexer_vessel::PinTable>,
}

impl<'a> ValidateCtx<'a> {
    /// Raw sealed-winner mode — today's default posture.
    pub fn raw(winners_dir: &'a Path) -> Self {
        Self {
            winners_dir,
            vessels_dir: None,
            #[cfg(feature = "vessel")]
            pins: None,
        }
    }

    /// Vessel mode — the minted-artifact posture (requires the `vessel`
    /// feature: the HOSTED-ONLY reader lives there).
    #[cfg(feature = "vessel")]
    pub fn vessel(
        winners_dir: &'a Path,
        vessels_dir: &'a Path,
        pins: &'a reflexer_vessel::PinTable,
    ) -> Self {
        Self {
            winners_dir,
            vessels_dir: Some(vessels_dir),
            pins: Some(pins),
        }
    }
}

impl ArsenalManifest {
    /// Parse a manifest from TOML text. Parse failures name the field
    /// (serde + toml spans); they never name a row that was not at fault.
    pub fn parse(toml_text: &str) -> Result<Self, String> {
        let manifest: Self = toml::from_str(toml_text)
            .map_err(|e| format!("manifest parse refused: {e}"))?;
        Ok(manifest)
    }

    /// The embedded default manifest (the repo-root `arsenal.toml`,
    /// compiled in — the default serve path never depends on the file
    /// being present at runtime).
    pub fn embedded_default() -> Result<Self, String> {
        Self::parse(EMBEDDED_MANIFEST)
    }

    /// BLAKE3 (hex) over manifest bytes — the law-A6 pin quantity.
    #[must_use]
    pub fn digest_of(toml_text: &str) -> String {
        blake3::hash(toml_text.as_bytes()).to_hex()[..].to_string()
    }

    /// BLAKE3 (hex) over the embedded default manifest's bytes.
    #[must_use]
    pub fn embedded_manifest_digest() -> String {
        Self::digest_of(EMBEDDED_MANIFEST)
    }

    /// The row for one suite, in manifest order.
    #[must_use]
    pub fn row(&self, suite: &str) -> Option<&VesselRow> {
        self.vessel.iter().find(|r| r.suite == suite)
    }

    /// All rows, in manifest order (the serve order).
    #[must_use]
    pub fn rows(&self) -> &[VesselRow] {
        &self.vessel
    }

    /// The served suite names, in manifest order.
    pub fn suites(&self) -> impl Iterator<Item = &str> {
        self.vessel.iter().map(|r| r.suite.as_str())
    }

    /// Full boot validation — schema semantics, then mode capability,
    /// then manifest ↔ files on disk. Every refusal names the offending
    /// row (suite) + field.
    pub fn validate(&self, ctx: &ValidateCtx) -> Result<(), String> {
        if self.vessel.is_empty() {
            return Err("the manifest has no [[vessel]] rows — nothing would serve".into());
        }
        let mut seen: HashSet<&str> = HashSet::with_capacity(self.vessel.len());
        for row in &self.vessel {
            row.validate()
                .map_err(|e| format!("suite {:?}: {e}", row.suite))?;
            if !seen.insert(row.suite.as_str()) {
                return Err(format!("suite {:?}: duplicate row", row.suite));
            }
        }
        let vessel_mode = ctx.vessels_dir.is_some();
        if vessel_mode && !cfg!(feature = "vessel") {
            return Err(
                "vessel mode requires the `vessel` feature (rebuild with --features vessel)"
                    .into(),
            );
        }
        #[cfg(feature = "vessel")]
        if vessel_mode {
            let Some(pins) = ctx.pins else {
                return Err("vessel mode validation requires a pin table".into());
            };
            for row in &self.vessel {
                if row.pin_keys.is_empty() {
                    return Err(format!(
                        "suite {:?}: pin_keys: vessel mode requires ≥1 pinned key id per row",
                        row.suite
                    ));
                }
                for id in &row.pin_keys {
                    pins.resolve_key(*id).map_err(|e| {
                        format!(
                            "suite {:?}: pin_keys: key id {id} does not resolve against the \
                             operator pins: {e}",
                            row.suite
                        )
                    })?;
                }
            }
        }
        for row in &self.vessel {
            self.validate_row_files(row, ctx, vessel_mode)?;
        }
        Ok(())
    }

    /// The manifest ↔ files half: where the artifact file exists, its
    /// BLAKE3 must match the pinned digest (drift refuses loud), its size
    /// must fit the budget, and — vessel mode — its signed header must
    /// agree with the row (format version, class, mint key-id). An absent
    /// file is left to the lane loader's error path (the dataless
    /// dev-box posture).
    fn validate_row_files(
        &self,
        row: &VesselRow,
        ctx: &ValidateCtx,
        vessel_mode: bool,
    ) -> Result<(), String> {
        let dir = if vessel_mode {
            ctx.vessels_dir.expect("vessel mode checked by caller")
        } else {
            ctx.winners_dir
        };
        let name = row.artifact_file(if vessel_mode {
            format!("{}_v1.vessel", row.suite)
        } else {
            format!("{}_winner_v1.bin", row.suite)
        });
        let path = dir.join(&name);
        if !path.exists() {
            return Ok(());
        }
        let cap_bytes = row.budget.max_payload_mb << 20;
        let len = std::fs::metadata(&path)
            .map_err(|e| format!("suite {:?}: file {name}: {e}", row.suite))?
            .len();
        if len > cap_bytes {
            return Err(format!(
                "suite {:?}: budget.max_payload_mb: artifact {name} is {len} bytes, over the \
                 {} MiB cap",
                row.suite, row.budget.max_payload_mb
            ));
        }
        let bytes =
            std::fs::read(&path).map_err(|e| format!("suite {:?}: file {name}: {e}", row.suite))?;
        let actual = blake3::hash(&bytes).to_hex();
        let pinned = row
            .digest
            .strip_prefix(DIGEST_TAG)
            .expect("digest format validated");
        if actual.as_str() != pinned {
            return Err(format!(
                "suite {:?}: digest: artifact {name} is {DIGEST_TAG}{actual} but the manifest \
                 pins {DIGEST_TAG}{pinned} — drift fails loud (law A5/A9)",
                row.suite
            ));
        }
        if vessel_mode {
            #[cfg(feature = "vessel")]
            validate_vessel_header(row, ctx, &bytes, &name, cap_bytes)?;
        }
        Ok(())
    }
}

impl VesselRow {
    /// Semantic row validation — everything decidable without the
    /// deployment's files. Mode-dependent pin checks live in
    /// [`ArsenalManifest::validate`].
    fn validate(&self) -> Result<(), String> {
        if self.suite.trim().is_empty() {
            return Err("suite: empty".into());
        }
        validate_digest_format(&self.digest)?;
        match self.class.as_str() {
            "hosted_only" => {}
            "public_release" => {
                return Err(
                    "class: public_release refused — the hosted lane loads HOSTED-ONLY \
                     artifacts only (the moat law; an extractable artifact here is a leak)"
                        .into(),
                )
            }
            other => {
                return Err(format!(
                    "class: unknown class {other:?} (expected \"public_release\" or \
                     \"hosted_only\")"
                ))
            }
        }
        // The arm + params — an unknown arm refuses here, NEVER defaulted
        // (law A6), and params must match the arm exactly.
        self.posture.to_arm()?;
        self.budget.validate()?;
        if let Some(f) = &self.file {
            if f.is_empty() || f.contains('/') || f.contains('\\') || f.contains("..") {
                return Err(format!(
                    "file: {f:?} must be a bare filename (no path separators)"
                ));
            }
        }
        Ok(())
    }

    /// The artifact filename this row loads: the explicit `file` override
    /// or the mode's default convention (the caller supplies the mode's
    /// default — raw and vessel conventions differ).
    #[must_use]
    pub fn artifact_file(&self, mode_default: String) -> String {
        self.file.clone().unwrap_or(mode_default)
    }

    /// The row's parsed serving arm — the strict posture mapping (an
    /// unknown arm refuses here, never defaulted).
    pub fn to_arm(&self) -> Result<Arm, String> {
        self.posture.to_arm()
    }
}

impl PostureSpec {
    /// The strict arm mapping — also the arm validator. The result feeds
    /// the server's [`Arm`] directly, so the served arm IS the manifest
    /// row (no re-derivation anywhere).
    pub fn to_arm(&self) -> Result<Arm, String> {
        match self.arm.as_str() {
            "A0" => {
                self.forbid_params(&["top_k", "beta", "n_min", "tau_n"])?;
                Ok(Arm::A0)
            }
            "A1" => {
                self.forbid_params(&["top_k", "beta", "n_min", "tau_n"])?;
                Ok(Arm::A1)
            }
            "H1" => {
                self.forbid_params(&["beta", "n_min", "tau_n"])?;
                let top_k = self
                    .top_k
                    .ok_or_else(|| "posture: arm H1 requires top_k".to_string())?;
                if !(1..=crate::MAX_TOP_K).contains(&top_k) {
                    return Err(format!(
                        "posture: top_k {top_k} outside 1..={}",
                        crate::MAX_TOP_K
                    ));
                }
                Ok(Arm::H1 { top_k })
            }
            "H2" => {
                self.forbid_params(&["top_k"])?;
                let (beta, n_min, tau_n) = match (self.beta, self.n_min, self.tau_n) {
                    (Some(b), Some(n), Some(t)) => (b, n, t),
                    _ => {
                        return Err(
                            "posture: arm H2 requires beta, n_min and tau_n".to_string()
                        )
                    }
                };
                if !beta.is_finite() || !n_min.is_finite() || !tau_n.is_finite() {
                    return Err("posture: H2 params must be finite".into());
                }
                if n_min < 0.0 || tau_n < 0.0 {
                    return Err("posture: n_min and tau_n must be ≥ 0".into());
                }
                Ok(Arm::H2 {
                    beta: beta as f32,
                    n_min: n_min as f32,
                    tau_n: tau_n as f32,
                })
            }
            other => Err(format!(
                "posture: unknown arm {other:?} — never defaulted (law A6; arms are A0, A1, \
                 H1, H2)"
            )),
        }
    }

    /// Params the arm does not take must be ABSENT — a typo'd or
    /// misassigned param is drift, never ignored.
    fn forbid_params(&self, taken: &[&str]) -> Result<(), String> {
        for (name, present) in [
            ("top_k", self.top_k.is_some()),
            ("beta", self.beta.is_some()),
            ("n_min", self.n_min.is_some()),
            ("tau_n", self.tau_n.is_some()),
        ] {
            if present && taken.contains(&name) {
                return Err(format!("posture: arm {} takes no {name}", self.arm));
            }
        }
        Ok(())
    }
}

impl BudgetSpec {
    fn validate(&self) -> Result<(), String> {
        match self.load.as_str() {
            "eager" | "lazy" => {}
            other => {
                return Err(format!(
                    "budget.load: unknown load policy {other:?} (expected \"eager\" or \"lazy\")"
                ))
            }
        }
        if !(1..=MAX_HOSTED_PAYLOAD_MB).contains(&self.max_payload_mb) {
            return Err(format!(
                "budget.max_payload_mb: {} outside 1..={MAX_HOSTED_PAYLOAD_MB}",
                self.max_payload_mb
            ));
        }
        Ok(())
    }
}

/// `"blake3:"` + exactly 64 lowercase hex chars.
fn validate_digest_format(digest: &str) -> Result<(), String> {
    let hex = digest
        .strip_prefix(DIGEST_TAG)
        .ok_or_else(|| format!("digest: {digest:?} must start with {DIGEST_TAG:?}"))?;
    if hex.len() != 64 || !hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(format!(
            "digest: {digest:?} must be {DIGEST_TAG:?} + 64 lowercase hex chars"
        ));
    }
    Ok(())
}

/// The vessel-mode header checks — the class-vs-reader-capability half is
/// the point: a row whose signed header disagrees with the manifest fails
/// HERE (manifest-time loud refusal), not as a mid-boot surprise from the
/// reader (`open()`'s `HostedOnlyPath` refusal included).
#[cfg(feature = "vessel")]
fn validate_vessel_header(
    row: &VesselRow,
    ctx: &ValidateCtx,
    bytes: &[u8],
    name: &str,
    cap_bytes: u64,
) -> Result<(), String> {
    let (header, _) = reflexer_vessel::peek(bytes)
        .map_err(|e| format!("suite {:?}: file {name}: peek refused: {e}", row.suite))?;
    if header.format_version != reflexer_vessel::FORMAT_VERSION {
        return Err(format!(
            "suite {:?}: file {name}: format version {} ≠ the reader's {} (law A9 — \
             same-engine-class only)",
            row.suite, header.format_version, reflexer_vessel::FORMAT_VERSION
        ));
    }
    // The row class must match the artifact's SIGNED class. The
    // hosted_only case is exactly the `HostedOnlyPath` trap: the public
    // reader refuses it at open() — caught up front, named, instead of
    // mid-boot.
    let declared = match row.class.as_str() {
        "hosted_only" => reflexer_vessel::Class::HostedOnly,
        _ => reflexer_vessel::Class::PublicRelease,
    };
    if header.class != declared {
        return Err(format!(
            "suite {:?}: class: the manifest declares {} but the artifact's signed header is \
             {} — the reader posture disagrees; fix the manifest",
            row.suite,
            row.class,
            header.class.as_str()
        ));
    }
    if !row.pin_keys.contains(&header.key_id) {
        return Err(format!(
            "suite {:?}: pin_keys: the vessel was minted under key id {} which the row does \
             not pin",
            row.suite, header.key_id
        ));
    }
    let Some(pins) = ctx.pins else {
        return Err("vessel mode validation requires a pin table".into());
    };
    pins.resolve_key(header.key_id).map_err(|e| {
        format!(
            "suite {:?}: pin_keys: the vessel's key id {} does not resolve: {e}",
            row.suite, header.key_id
        )
    })?;
    if header.payload_len > cap_bytes {
        return Err(format!(
            "suite {:?}: budget.max_payload_mb: the vessel payload is {} bytes, over the {} \
             MiB cap",
            row.suite, header.payload_len, row.budget.max_payload_mb
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest_with(toml_text: &str) -> Result<ArsenalManifest, String> {
        ArsenalManifest::parse(toml_text)
    }

    fn one_row(posture: &str) -> String {
        format!(
            "[[vessel]]\n\
             suite = \"ag_news\"\n\
             digest = \"blake3:{:064}\"\n\
             class = \"hosted_only\"\n\
             posture = {posture}\n\
             pin_keys = []\n\
             budget = {{ load = \"eager\", max_payload_mb = 16 }}\n",
            "0".repeat(64)
        )
    }

    #[test]
    fn embedded_default_parses_with_the_six_suites() {
        let m = ArsenalManifest::embedded_default().expect("embedded manifest parses");
        let suites: Vec<&str> = m.suites().collect();
        assert_eq!(
            suites,
            ["ag_news", "emotion", "sst5", "massive_intent_en", "banking77", "xnli_en"]
        );
        assert_eq!(m.rows().len(), 6);
    }

    #[test]
    fn unknown_posture_arm_refuses_never_defaulted() {
        let m = manifest_with(&one_row("{ arm = \"H3\" }")).expect("parse");
        let err = m.validate(&ValidateCtx::raw(Path::new("/nonexistent"))).unwrap_err();
        assert!(err.contains("unknown arm"), "{err}");
        assert!(err.contains("never defaulted"), "{err}");
        assert!(err.contains("ag_news"), "{err}");
    }

    #[test]
    fn posture_params_must_match_the_arm() {
        // H2 missing tau_n.
        let m = manifest_with(&one_row("{ arm = \"H2\", beta = 1.0, n_min = 2.0 }"))
            .expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("H2 requires beta, n_min and tau_n")
        );
        // H2 carrying top_k.
        let m = manifest_with(&one_row(
            "{ arm = \"H2\", beta = 1.0, n_min = 2.0, tau_n = 8.0, top_k = 4 }",
        ))
        .expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("takes no top_k")
        );
        // A0 carrying beta.
        let m = manifest_with(&one_row("{ arm = \"A0\", beta = 0.5 }")).expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("takes no beta")
        );
        // H1 missing top_k.
        let m = manifest_with(&one_row("{ arm = \"H1\" }")).expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("H1 requires top_k")
        );
        // H1 out of range.
        let m = manifest_with(&one_row("{ arm = \"H1\", top_k = 64 }")).expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("outside 1..=")
        );
    }

    #[test]
    fn digest_format_is_strict() {
        for bad in [
            "ec32aac3",                      // no tag
            "blake3:EC327AC30250205B",       // wrong everything
            "blake3:ec327ac3",               // too short
            "sha3:ec327ac30250205b2b26c6b00976f9fb50bdf44a9bcbb1d87e35c6ca06f5cf33",
            "blake3:EC327AC30250205B2B26C6B00976F9FB50BDF44A9BCBB1D87E35C6CA06F5CF33",
        ] {
            let text = one_row("{ arm = \"A0\" }").replace(
                "blake3:0000000000000000000000000000000000000000000000000000000000000000",
                bad,
            );
            let m = manifest_with(&text).expect("parse");
            let err = m
                .validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err();
            assert!(err.contains("digest:"), "{bad}: {err}");
        }
    }

    #[test]
    fn class_vocabulary_and_moat_law() {
        let m = manifest_with(&one_row("{ arm = \"A0\" }").replace(
            "class = \"hosted_only\"",
            "class = \"public_release\"",
        ))
        .expect("parse");
        let err = m
            .validate(&ValidateCtx::raw(Path::new("/nonexistent")))
            .unwrap_err();
        assert!(err.contains("public_release refused"), "{err}");
        let m = manifest_with(&one_row("{ arm = \"A0\" }").replace(
            "class = \"hosted_only\"",
            "class = \"sealed\"",
        ))
        .expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("unknown class")
        );
    }

    #[test]
    fn budget_is_validated() {
        let m = manifest_with(&one_row("{ arm = \"A0\" }").replace(
            "load = \"eager\"",
            "load = \"whenever\"",
        ))
        .expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("budget.load")
        );
        let m = manifest_with(&one_row("{ arm = \"A0\" }").replace(
            "max_payload_mb = 16",
            "max_payload_mb = 64",
        ))
        .expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("budget.max_payload_mb")
        );
    }

    #[test]
    fn duplicates_and_empty_manifests_refuse() {
        let text = format!("{}{}", one_row("{ arm = \"A0\" }"), one_row("{ arm = \"A1\" }"));
        let m = manifest_with(&text).expect("parse");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("duplicate row")
        );
        let m = manifest_with("").expect("empty parses");
        assert!(
            m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                .unwrap_err()
                .contains("no [[vessel]] rows")
        );
    }

    #[test]
    fn file_override_must_be_a_bare_filename() {
        for (bad, toml_value) in [
            ("../escape.bin", "\"../escape.bin\""),
            ("sub/dir.bin", "\"sub/dir.bin\""),
            // A TOML literal string (single quotes) carries the raw
            // backslash the basic-string form would escape.
            ("back\\slash.bin", "'back\\slash.bin'"),
        ] {
            let text = one_row("{ arm = \"A0\" }")
                .replace("pin_keys = []", &format!("pin_keys = []\nfile = {toml_value}"));
            let m = manifest_with(&text).expect("parse");
            assert!(
                m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
                    .unwrap_err()
                    .contains("bare filename"),
                "{bad}"
            );
        }
    }

    #[test]
    fn unknown_fields_refuse_loud() {
        let text = one_row("{ arm = \"A0\" }").replace(
            "pin_keys = []",
            "pin_keys = []\ndgest = \"typo\"",
        );
        assert!(manifest_with(&text).is_err(), "a typo'd field must refuse");
    }

    #[test]
    fn digest_drift_fails_loud_naming_row_and_field() {
        // A real file whose bytes disagree with the pinned digest.
        let dir = std::env::temp_dir().join(format!("arsenal_drift_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let artifact = dir.join("ag_news_winner_v1.bin");
        std::fs::write(&artifact, b"not the real winner bytes").expect("write");
        let text = one_row("{ arm = \"A0\" }");
        let m = manifest_with(&text).expect("parse");
        let err = m.validate(&ValidateCtx::raw(&dir)).unwrap_err();
        assert!(err.contains("ag_news"), "{err}");
        assert!(err.contains("digest:"), "{err}");
        assert!(err.contains("drift fails loud"), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn size_over_budget_refuses_before_the_read() {
        let dir = std::env::temp_dir().join(format!("arsenal_cap_{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let artifact = dir.join("ag_news_winner_v1.bin");
        // 17 MiB — one over the cap. The size check must fire BEFORE the
        // read (the refusal names the budget, never the digest).
        std::fs::write(&artifact, vec![0u8; 17 << 20]).expect("write");
        let text = one_row("{ arm = \"A0\" }");
        let m = manifest_with(&text).expect("parse");
        let err = m.validate(&ValidateCtx::raw(&dir)).unwrap_err();
        assert!(err.contains("budget.max_payload_mb"), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn absent_artifact_left_to_the_lane_loader() {
        let m = manifest_with(&one_row("{ arm = \"A0\" }")).expect("parse");
        m.validate(&ValidateCtx::raw(Path::new("/nonexistent")))
            .expect("absent file is not a validation refusal");
    }

    #[test]
    fn vessel_mode_pin_rules() {
        // vessel mode without the pins field can never be constructed
        // (ValidateCtx::vessel is feature-gated and takes &PinTable);
        // the pin-related arms below are exercised in vessel_gates.
        let m = manifest_with(&one_row("{ arm = \"A0\" }")).expect("parse");
        assert_eq!(m.rows()[0].pin_keys, Vec::<u32>::new());
    }

    #[cfg(feature = "vessel")]
    #[test]
    fn hosted_cap_matches_the_format_crate() {
        assert_eq!(
            MAX_HOSTED_PAYLOAD_MB,
            (reflexer_vessel::MAX_HOSTED_PAYLOAD >> 20) as u64,
            "the manifest MiB cap must track reflexer-vessel's hosted cap"
        );
    }
}

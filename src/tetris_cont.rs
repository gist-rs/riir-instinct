//! Multi-continuation teacher collection — the `TETCRIT2` dataset
//! (riir-train Plan 429 T1.1, with Plan 460 T1.1's per-step format
//! requirement).
//!
//! For a teacher-visited (state, option) sample, run `M` continuation
//! rollouts from the option's afterstate, each under its own fresh-bag
//! piece stream, and keep the **per-placement reward sequence** of every
//! rollout — not just its total. Plan 429 consumes the totals (the
//! per-option continuation-value distribution); Plan 460 fits cheap-basis
//! prefixes of the sequences (its horizon selector) and needs each
//! rollout's length (its truncation mask).
//!
//! # The continuation policy
//!
//! The champion genome's **1-ply argmax** over the champion evaluator —
//! the budget-0 PUCT prior's argmax (the b0 shape), decided per step at
//! the step's own mode, no preview, no hold, a fresh bag (the serving
//! information rule). Cheap by design: a teacher-budget continuation would
//! cost ~10³× more per step. The score is f64 and the pick is the FIRST
//! strict max (the site's argmax convention).
//!
//! # The reward channel
//!
//! Step `t` stores the lines cleared by placement `t` as one `u8` (0..=4):
//! lossless, and the points channel is `LINES_SCORE[lines]` (the table is
//! written into the manifest). Step 0 is the sampled option ITSELF, so a
//! sequence is the whole return from `(s, a)`, not from `s'`.
//!
//! # Determinism
//!
//! A rollout is a pure function of `(game seed, decision, option index,
//! rollout j)` through [`cont_seed`] — independent of thread count and of
//! which other samples were selected.
//!
//! # `TETCRIT2` layout (little-endian)
//!
//! Header: magic `TETCRIT2`, `n_samples:u32`, `m:u8`, `cap:u16`, `pad:u8`.
//! Per sample: the [`encode_sample_v1`] record (the `TETCRIT1` 299-byte
//! layout, unchanged), then for each of the `m` rollouts: `len:u16`,
//! `flags:u8` (bit 0 = topped out before `cap`), `lines:[u8; len]`.
//! Readers of v1 refuse a v2 file by magic, and [`decode_tetcrit2`]
//! refuses a v1 file by magic, loudly.

use crate::tetris_critic::{feat, RawOpt, Sample};
use katgpt_tetris::lookahead::{apply, Bag};
use katgpt_tetris::rulebook::{Genome, Leaf, Mode};
use katgpt_tetris::sim::{landing_options_with, Board, DropRule};

/// `TETCRIT2` magic.
pub const MAGIC_V2: &[u8; 8] = b"TETCRIT2";
/// `TETCRIT1` magic (refused by the v2 reader).
pub const MAGIC_V1: &[u8; 8] = b"TETCRIT1";
/// The fixed-width v1 sample record (shared verbatim by v2).
pub const SAMPLE_V1_BYTES: usize = 8 + 4 + 1 + 1 + 1 + 1 + 8 + feat::NUM * 8;
/// Header bytes of a v2 file.
pub const HEADER_V2_BYTES: usize = 8 + 4 + 1 + 2 + 1;
/// Salt separating continuation piece streams from the teacher's own.
pub const CONT_SALT: u64 = 0xC047_7E22_0429_0460;

/// What to collect continuations for, and how far to roll them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContConfig {
    /// Rollouts per sample (`M`; Plan 429 pre-registers 8). `1..=255`.
    pub m: u8,
    /// Placements per rollout INCLUDING the sampled option (step 0).
    /// A rollout that survives `cap` placements is truncated, not topped
    /// out — its flag says which.
    pub cap: u16,
    /// Collect at decisions `d` with `d % decision_stride == 0` (`>= 1`).
    pub decision_stride: u32,
    /// Per selected decision, the `top_k` teacher-visited options by
    /// visit count (ties → lower option index); `0` = every visited one.
    pub top_k: usize,
}

impl ContConfig {
    /// Panics on a configuration no collection can honour.
    pub fn validate(&self) {
        assert!(self.m >= 1, "ContConfig: m must be >= 1");
        assert!(self.cap >= 1, "ContConfig: cap must be >= 1");
        assert!(
            self.decision_stride >= 1,
            "ContConfig: decision_stride must be >= 1"
        );
    }
}

/// One continuation: lines cleared per placement (step 0 = the sampled
/// option) and whether it ended by topping out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rollout {
    pub lines: Vec<u8>,
    pub topped_out: bool,
}

/// A v1 sample plus its `M` continuations.
#[derive(Clone, Debug)]
pub struct ContSample {
    pub sample: Sample,
    pub rollouts: Vec<Rollout>,
}

/// SplitMix64 finaliser — the piece-stream seed of rollout `j` of option
/// `option` at `decision` in game `seed`.
pub fn cont_seed(seed: u64, decision: u32, option: usize, j: u8) -> u64 {
    let mut z =
        seed ^ CONT_SALT ^ (u64::from(decision) << 24) ^ ((option as u64) << 8) ^ u64::from(j);
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The champion evaluator at one leaf — the teacher's eval closure body.
pub fn champion_eval(genome: &Genome, mode: Mode, b: &Board, lines: u32, tetrises: u32) -> f64 {
    let scan = b.scan();
    let leaf = Leaf {
        board: b,
        heights: scan.heights,
        lines,
        tetrises,
        held: None,
    };
    genome.eval_with(&leaf, mode, &scan)
}

/// One continuation from `after` (the board once the sampled option is
/// placed, which cleared `first_lines`), `cap` placements including that
/// first one, champion 1-ply argmax, fresh bag seeded by `piece_seed`.
pub fn rollout(
    genome: &Genome,
    after: &Board,
    first_lines: u32,
    piece_seed: u64,
    cap: u16,
) -> Rollout {
    let cap = usize::from(cap);
    let mut lines = Vec::with_capacity(cap);
    lines.push(first_lines.min(4) as u8);
    let mut board = after.clone();
    let mut bag = Bag::new(piece_seed);
    let mut topped_out = false;
    while lines.len() < cap {
        let piece = bag.draw();
        let options = landing_options_with(&board, piece, DropRule::FromTop);
        if options.is_empty() {
            topped_out = true;
            break;
        }
        let mode = genome.mode_of(&board);
        let mut best: Option<(Board, u32)> = None;
        let mut best_v = f64::NEG_INFINITY;
        for p in &options {
            let (b1, l1) = apply(&board, &p.cells);
            let v = champion_eval(genome, mode, &b1, l1, u32::from(l1 == 4));
            if best.is_none() || v > best_v {
                best_v = v;
                best = Some((b1, l1));
            }
        }
        let (nb, l1) = best.expect("options non-empty");
        lines.push(l1.min(4) as u8);
        board = nb;
    }
    Rollout { lines, topped_out }
}

/// The `M` continuations of option `option` (afterstate `after`, cleared
/// `first_lines`) at `decision` of game `seed`.
pub fn rollouts_for(
    genome: &Genome,
    cfg: &ContConfig,
    seed: u64,
    decision: u32,
    option: usize,
    after: &Board,
    first_lines: u32,
) -> Vec<Rollout> {
    (0..cfg.m)
        .map(|j| {
            rollout(
                genome,
                after,
                first_lines,
                cont_seed(seed, decision, option, j),
                cfg.cap,
            )
        })
        .collect()
}

/// Which visited options (by root index) get continuations at a selected
/// decision: the `top_k` by visits, ties → lower index, returned in index
/// order. `visited` is `(index, visits)` with `visits > 0`.
pub fn select_top_k(visited: &[(usize, u32)], top_k: usize) -> Vec<usize> {
    let mut v: Vec<(usize, u32)> = visited.to_vec();
    if top_k > 0 && v.len() > top_k {
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.truncate(top_k);
    }
    let mut idx: Vec<usize> = v.into_iter().map(|(i, _)| i).collect();
    idx.sort_unstable();
    idx
}

// ── Encoding ─────────────────────────────────────────────────────────────

/// Append one sample in the fixed-width `TETCRIT1` layout:
/// `seed:u64 decision:u32 piece:u8 mode:u8 is_pick:u8 pad:u8 q:f64
/// num:[f64; feat::NUM]`.
pub fn encode_sample_v1(s: &Sample, buf: &mut Vec<u8>) {
    buf.extend_from_slice(&s.seed.to_le_bytes());
    buf.extend_from_slice(&s.decision.to_le_bytes());
    buf.push(s.raw.piece);
    buf.push(s.raw.mode);
    buf.push(u8::from(s.is_pick));
    buf.push(0);
    buf.extend_from_slice(&s.q.to_le_bytes());
    for v in &s.raw.num {
        buf.extend_from_slice(&v.to_le_bytes());
    }
}

/// Encode a whole `TETCRIT2` file. Panics if a sample's rollout count is
/// not `cfg.m` or a rollout exceeds `cfg.cap` (a collector bug — never
/// written to disk).
pub fn encode_tetcrit2(samples: &[ContSample], cfg: &ContConfig) -> Vec<u8> {
    let steps: usize = samples
        .iter()
        .flat_map(|s| s.rollouts.iter().map(|r| r.lines.len()))
        .sum();
    let mut buf = Vec::with_capacity(
        HEADER_V2_BYTES + samples.len() * (SAMPLE_V1_BYTES + usize::from(cfg.m) * 3) + steps,
    );
    buf.extend_from_slice(MAGIC_V2);
    buf.extend_from_slice(
        &u32::try_from(samples.len())
            .expect("n_samples fits u32")
            .to_le_bytes(),
    );
    buf.push(cfg.m);
    buf.extend_from_slice(&cfg.cap.to_le_bytes());
    buf.push(0);
    for s in samples {
        assert_eq!(s.rollouts.len(), usize::from(cfg.m), "rollout count != m");
        encode_sample_v1(&s.sample, &mut buf);
        for r in &s.rollouts {
            assert!(
                !r.lines.is_empty() && r.lines.len() <= usize::from(cfg.cap),
                "rollout length outside 1..=cap"
            );
            buf.extend_from_slice(&(r.lines.len() as u16).to_le_bytes());
            buf.push(u8::from(r.topped_out));
            buf.extend_from_slice(&r.lines);
        }
    }
    buf
}

/// A decoded `TETCRIT2` file.
#[derive(Clone, Debug)]
pub struct Tetcrit2 {
    pub m: u8,
    pub cap: u16,
    pub samples: Vec<ContSample>,
}

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        let end = self
            .at
            .checked_add(n)
            .filter(|&e| e <= self.b.len())
            .ok_or_else(|| format!("TETCRIT2: truncated at byte {}", self.at))?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().expect("2")))
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().expect("4")))
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().expect("8")))
    }
    fn f64(&mut self) -> Result<f64, String> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().expect("8")))
    }
}

/// Decode a `TETCRIT2` file; refuses `TETCRIT1` (and anything else) by
/// magic, truncation, trailing bytes, out-of-range lines and lengths.
pub fn decode_tetcrit2(bytes: &[u8]) -> Result<Tetcrit2, String> {
    let mut c = Cursor { b: bytes, at: 0 };
    let magic = c.take(8)?;
    if magic == MAGIC_V1 {
        return Err("TETCRIT2 reader refuses a TETCRIT1 file (no continuations — re-collect with export_tetris_critic_cont)".into());
    }
    if magic != MAGIC_V2 {
        return Err("TETCRIT2: bad magic".into());
    }
    let n = c.u32()? as usize;
    let m = c.u8()?;
    let cap = c.u16()?;
    let _pad = c.u8()?;
    if m == 0 || cap == 0 {
        return Err(format!("TETCRIT2: invalid header m={m} cap={cap}"));
    }
    let mut samples = Vec::with_capacity(n.min(bytes.len() / SAMPLE_V1_BYTES + 1));
    for _ in 0..n {
        let seed = c.u64()?;
        let decision = c.u32()?;
        let piece = c.u8()?;
        let mode = c.u8()?;
        let is_pick = c.u8()? != 0;
        let _pad = c.u8()?;
        let q = c.f64()?;
        let mut num = [0.0; feat::NUM];
        for v in &mut num {
            *v = c.f64()?;
        }
        let mut rollouts = Vec::with_capacity(usize::from(m));
        for _ in 0..m {
            let len = c.u16()?;
            if len == 0 || len > cap {
                return Err(format!("TETCRIT2: rollout length {len} outside 1..={cap}"));
            }
            let flags = c.u8()?;
            if flags > 1 {
                return Err(format!("TETCRIT2: unknown rollout flags {flags:#x}"));
            }
            let lines = c.take(usize::from(len))?.to_vec();
            if let Some(&bad) = lines.iter().find(|&&l| l > 4) {
                return Err(format!("TETCRIT2: lines value {bad} > 4"));
            }
            rollouts.push(Rollout {
                lines,
                topped_out: flags & 1 == 1,
            });
        }
        samples.push(ContSample {
            sample: Sample {
                seed,
                decision,
                raw: RawOpt { num, piece, mode },
                q,
                is_pick,
            },
            rollouts,
        });
    }
    if c.at != bytes.len() {
        return Err(format!(
            "TETCRIT2: {} trailing bytes after {n} samples",
            bytes.len() - c.at
        ));
    }
    Ok(Tetcrit2 { m, cap, samples })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetris_lane::{start_board, Regime};

    fn genome() -> Genome {
        Genome::champion_hybrid()
    }

    fn sample(seed: u64, decision: u32) -> Sample {
        let mut num = [0.0; feat::NUM];
        for (i, v) in num.iter_mut().enumerate() {
            *v = i as f64 * 0.5 - seed as f64;
        }
        Sample {
            seed,
            decision,
            raw: RawOpt {
                num,
                piece: 3,
                mode: 1,
            },
            q: 0.625,
            is_pick: decision.is_multiple_of(2),
        }
    }

    fn cfg() -> ContConfig {
        ContConfig {
            m: 3,
            cap: 64,
            decision_stride: 1,
            top_k: 0,
        }
    }

    #[test]
    fn rollout_is_deterministic_and_starts_with_the_option() {
        let g = genome();
        let b = start_board(211, Regime::garbage(16, 75));
        let a = rollout(&g, &b, 2, 0xABCD, 128);
        let b2 = rollout(&g, &b, 2, 0xABCD, 128);
        assert_eq!(a, b2);
        assert_eq!(a.lines[0], 2, "step 0 is the sampled option's own lines");
        assert!(a.lines.len() <= 128 && !a.lines.is_empty());
        assert!(a.topped_out || a.lines.len() == 128);
        assert!(a.lines.iter().all(|&l| l <= 4));
    }

    #[test]
    fn piece_seeds_differ_across_every_axis() {
        let base = cont_seed(201, 7, 3, 0);
        for other in [
            cont_seed(202, 7, 3, 0),
            cont_seed(201, 8, 3, 0),
            cont_seed(201, 7, 4, 0),
            cont_seed(201, 7, 3, 1),
        ] {
            assert_ne!(base, other);
        }
        // Distinct seeds actually yield distinct continuations somewhere.
        let g = genome();
        let b = start_board(211, Regime::garbage(16, 75));
        let rs = rollouts_for(&g, &cfg(), 211, 0, 0, &b, 0);
        assert_eq!(rs.len(), 3);
        assert!(
            rs.windows(2).any(|w| w[0] != w[1]),
            "M rollouts all identical"
        );
    }

    #[test]
    fn cap_one_is_the_option_alone() {
        let g = genome();
        let b = start_board(5, Regime::EMPTY);
        let r = rollout(&g, &b, 1, 9, 1);
        assert_eq!(r.lines, vec![1]);
        assert!(!r.topped_out);
    }

    #[test]
    fn top_k_by_visits_ties_to_lower_index_returned_in_index_order() {
        let v = [(0, 5), (1, 9), (2, 9), (3, 1), (4, 5)];
        assert_eq!(select_top_k(&v, 2), vec![1, 2]);
        assert_eq!(select_top_k(&v, 3), vec![0, 1, 2]);
        assert_eq!(select_top_k(&v, 0), vec![0, 1, 2, 3, 4]);
        assert_eq!(select_top_k(&v, 9), vec![0, 1, 2, 3, 4]);
        assert!(select_top_k(&[], 4).is_empty());
    }

    #[test]
    fn tetcrit2_round_trips_and_refuses_v1_and_corruption() {
        let c = cfg();
        let samples: Vec<ContSample> = (0..4)
            .map(|d| ContSample {
                sample: sample(300, d),
                rollouts: (0..c.m)
                    .map(|j| Rollout {
                        lines: (0..(1 + d as usize * 7 + j as usize))
                            .map(|t| (t % 5) as u8)
                            .collect(),
                        topped_out: j == 1,
                    })
                    .collect(),
            })
            .collect();
        let buf = encode_tetcrit2(&samples, &c);
        let back = decode_tetcrit2(&buf).expect("decode");
        assert_eq!((back.m, back.cap), (c.m, c.cap));
        assert_eq!(back.samples.len(), samples.len());
        for (x, y) in back.samples.iter().zip(&samples) {
            assert_eq!(x.rollouts, y.rollouts);
            assert_eq!(x.sample.seed, y.sample.seed);
            assert_eq!(x.sample.decision, y.sample.decision);
            assert_eq!(x.sample.is_pick, y.sample.is_pick);
            assert_eq!(x.sample.q.to_bits(), y.sample.q.to_bits());
            assert_eq!(
                (x.sample.raw.piece, x.sample.raw.mode),
                (y.sample.raw.piece, y.sample.raw.mode)
            );
            assert!(x
                .sample
                .raw
                .num
                .iter()
                .zip(&y.sample.raw.num)
                .all(|(a, b)| a.to_bits() == b.to_bits()));
        }
        // The v1 sample record is embedded verbatim.
        let mut v1 = Vec::new();
        encode_sample_v1(&samples[0].sample, &mut v1);
        assert_eq!(v1.len(), SAMPLE_V1_BYTES);
        assert_eq!(
            &buf[HEADER_V2_BYTES..HEADER_V2_BYTES + SAMPLE_V1_BYTES],
            &v1[..]
        );

        let mut v1file = buf.clone();
        v1file[..8].copy_from_slice(MAGIC_V1);
        assert!(decode_tetcrit2(&v1file)
            .unwrap_err()
            .contains("refuses a TETCRIT1"));
        assert!(decode_tetcrit2(&buf[..buf.len() - 1])
            .unwrap_err()
            .contains("truncated"));
        let mut trailing = buf.clone();
        trailing.push(0);
        assert!(decode_tetcrit2(&trailing).unwrap_err().contains("trailing"));
        let mut bad_lines = buf.clone();
        let last = bad_lines.len() - 1;
        bad_lines[last] = 5;
        assert!(decode_tetcrit2(&bad_lines)
            .unwrap_err()
            .contains("lines value"));
    }

    #[test]
    #[should_panic(expected = "rollout count != m")]
    fn encoder_refuses_wrong_rollout_count() {
        let s = ContSample {
            sample: sample(1, 0),
            rollouts: vec![],
        };
        encode_tetcrit2(&[s], &cfg());
    }
}

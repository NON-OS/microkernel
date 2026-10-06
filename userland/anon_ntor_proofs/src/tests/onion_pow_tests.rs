// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Onion-service proof of work. The puzzle itself is held to the fork's own
//! solve loop (equix_proofs/vectors/equix.expect, made by equix_gen.c from
//! the fork's src/ext/equix and hs_pow.c), the extension field to
//! test_hs_pow.c's encoded vectors, the INTRODUCE1 cell to
//! hs_ntor_vector.py, and the descriptor line to onion_descriptor.py, which
//! writes it as hs_descriptor.c does.

use alloc::vec::Vec;

use crate::hex;
use crate::onion::cache::{CacheKey, DescCache};
use crate::onion::cells::introduce1;
use crate::onion::desc::{decode, IntroPoint};
use crate::onion::hs_ntor::intro_keys;
use crate::onion::pow::{effort, extension, params, PowParams, PowSolution, Puzzle, CLIENT_MAX_EFFORT, EXTENSION_BYTES};

const GEN: &str = include_str!("../../../equix_proofs/vectors/equix.expect");
const HS: &str = include_str!("../../vectors/hs_ntor_vector.expect");
const DESC_POW: &[u8] = include_bytes!("../../vectors/onion_descriptor_pow.txt");
const DESC: &[u8] = include_bytes!("../../vectors/onion_descriptor.txt");
const NOW: u64 = 1_790_000_000;
/// 2026-10-03T12:00:00, the fixture's seed expiry.
const POW_EXPIRES: u64 = 1_791_028_800;

fn arr<const N: usize>(text: &str) -> [u8; N] {
    hex(text).try_into().unwrap()
}

#[test]
fn a_descriptor_under_load_carries_its_puzzle() {
    let blinded = arr("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8");
    let desc = decode(DESC_POW, &blinded, &[0x55; 32], NOW, |_| None).expect("decodes");
    assert_eq!(desc.pow, Some(PowParams { seed: [0xAA; 32], suggested_effort: 250, expires: POW_EXPIRES }));
    assert_eq!(desc.intro_points.len(), 1);
    let plain = decode(DESC, &blinded, &[0x55; 32], NOW, |_| None).expect("decodes");
    assert_eq!(plain.pow, None);
}

const SEED_B64: &str = "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqo=";

fn inner(lines: &str) -> Vec<u8> {
    let mut out = b"create2-formats 2\n".to_vec();
    out.extend_from_slice(lines.as_bytes());
    out.extend_from_slice(b"introduction-point AAAA\n");
    out
}

#[test]
fn the_line_reads_as_hs_descriptor_c_writes_it() {
    let line = alloc::format!("pow-params v1 {SEED_B64} 4294967295 2026-10-03T12:00:00\n");
    let p = params(&inner(&line)).unwrap().unwrap();
    assert_eq!(p.seed, [0xAA; 32]);
    assert_eq!(p.suggested_effort, u32::MAX);
    assert_eq!(p.expires, POW_EXPIRES);
    // Unpadded base64 is the same seed.
    let line = alloc::format!("pow-params v1 {} 0 2026-10-03T12:00:00\n", SEED_B64.trim_end_matches('='));
    assert_eq!(params(&inner(&line)).unwrap().unwrap().seed, [0xAA; 32]);
}

#[test]
fn no_line_and_unknown_kinds_ask_for_nothing() {
    assert_eq!(params(&inner("")), Ok(None));
    let line = alloc::format!("pow-params v2 {SEED_B64} 10 2026-10-03T12:00:00\n");
    assert_eq!(params(&inner(&line)), Ok(None));
    // A name that only begins with v1 is not v1.
    let line = alloc::format!("pow-params v1x {SEED_B64} 10 2026-10-03T12:00:00\n");
    assert_eq!(params(&inner(&line)), Ok(None));
}

#[test]
fn a_malformed_v1_line_refuses_the_descriptor() {
    let short = "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq==";
    for line in [
        "pow-params v1\n".into(),
        alloc::format!("pow-params v1 {SEED_B64}\n"),
        alloc::format!("pow-params v1 {SEED_B64} 10\n"),
        alloc::format!("pow-params v1 {short} 10 2026-10-03T12:00:00\n"),
        "pow-params v1 !!!! 10 2026-10-03T12:00:00\n".into(),
        alloc::format!("pow-params v1 {SEED_B64} 4294967296 2026-10-03T12:00:00\n"),
        alloc::format!("pow-params v1 {SEED_B64} -1 2026-10-03T12:00:00\n"),
        alloc::format!("pow-params v1 {SEED_B64} 10 2026-10-03 12:00:00\n"),
        alloc::format!("pow-params v1 {SEED_B64} 10 2026-13-03T12:00:00\n"),
        alloc::format!("pow-params v1 {SEED_B64} 10 2026-10-03T12:00:00Z\n"),
    ] {
        assert!(params(&inner(&line)).is_err(), "{line}");
    }
}

#[test]
fn a_second_line_refuses_the_descriptor() {
    let line = alloc::format!("pow-params v1 {SEED_B64} 10 2026-10-03T12:00:00\n");
    assert!(params(&inner(&alloc::format!("{line}{line}"))).is_err());
    let other = alloc::format!("pow-params v2 {SEED_B64} 10 2026-10-03T12:00:00\n");
    assert!(params(&inner(&alloc::format!("{other}{line}"))).is_err());
}

#[test]
fn a_line_inside_an_introduction_point_is_not_the_services() {
    let mut text = inner("");
    text.extend_from_slice(alloc::format!("pow-params v1 {SEED_B64} 10 2026-10-03T12:00:00\n").as_bytes());
    assert_eq!(params(&text), Ok(None));
}

#[test]
fn effort_follows_hs_client_c() {
    // (suggested, introduction points that failed) -> effort
    for (suggested, unreachable, expected) in [
        (0, 0, 0),
        (0, 1, 8),
        (3, 1, 8),
        (100, 0, 100),
        (100, 1, 200),
        (100, 3, 800),
        (600, 1, 1200),
        (1000, 1, 1500),
        (1001, 1, 1501),
        (9000, 1, CLIENT_MAX_EFFORT),
        (5000, 3, CLIENT_MAX_EFFORT),
        (20000, 0, CLIENT_MAX_EFFORT),
        (u32::MAX, u32::MAX, CLIENT_MAX_EFFORT),
    ] {
        assert_eq!(effort(suggested, unreachable), expected, "{suggested} after {unreachable}");
    }
}

#[test]
fn the_extension_matches_test_hs_pow_c() {
    // (nonce, effort, seed head, solution, encoded_hex), the last laid out
    // piece by piece as test_hs_pow.c writes it.
    for (nonce, effort, head, sol, encoded) in [
        (
            "55555555555555555555555555555555",
            0,
            "aaaaaaaa",
            "4312f87ceab844c78e1c793a913812d7",
            concat!("01", "55555555555555555555555555555555", "00000000", "aaaaaaaa", "4312f87ceab844c78e1c793a913812d7"),
        ),
        (
            "59217255555555555555555555555555",
            1000000,
            "aaaaaaaa",
            "0f3db97b9cac20c1771680a1a34848d3",
            concat!("01", "59217255555555555555555555555555", "000f4240", "aaaaaaaa", "0f3db97b9cac20c1771680a1a34848d3"),
        ),
        (
            "2eff9fdbc34326d9d2f18ed277469c63",
            100000,
            "86fb0acf",
            "400cb091139f86b352119f6e131802d6",
            concat!("01", "2eff9fdbc34326d9d2f18ed277469c63", "000186a0", "86fb0acf", "400cb091139f86b352119f6e131802d6"),
        ),
    ] {
        let s = PowSolution { nonce: arr(nonce), effort, seed_head: arr(head), solution: arr(sol) };
        let field = extension(&s);
        assert_eq!(field[0], 0x02, "TRUNNEL_EXT_TYPE_POW");
        assert_eq!(usize::from(field[1]), EXTENSION_BYTES - 2);
        assert_eq!(field[2..].to_vec(), hex(encoded));
    }
}

fn hs(name: &str) -> Vec<u8> {
    let line = HS.lines().find(|l| l.split(' ').next() == Some(name)).expect(name);
    hex(line.split(' ').nth(1).unwrap())
}

#[test]
fn introduce1_with_a_solution_matches_the_reference_byte_for_byte() {
    let auth = [0x41; 32];
    let x: [u8; 32] = hs("X").try_into().unwrap();
    let keys = intro_keys(&hs("dh_bx").try_into().unwrap(), &auth, &x, &hs("B").try_into().unwrap(), &[0x42; 32]);
    let mut specs = alloc::vec![2u8, 0, 6, 10, 0, 0, 1, 0x23, 0x29, 2, 20];
    specs.extend_from_slice(&[0x53; 20]);
    let s = PowSolution {
        nonce: arr("59217255555555555555555555555555"),
        effort: 1_000_000,
        seed_head: [0xAA; 4],
        solution: arr("0f3db97b9cac20c1771680a1a34848d3"),
    };
    let cell = introduce1(&auth, &x, &keys, &[0x51; 20], (&[0x52; 32], &specs), Some(&extension(&s))).expect("fits");
    assert_eq!(cell, hs("introduce1_pow"));
    // The solution rides inside the padding's room: the cell is no longer.
    assert_eq!(cell.len(), hs("introduce1").len());
}

struct Gen {
    seed: [u8; 32],
    blinded: [u8; 32],
    first: [u8; 16],
    effort: u32,
    nonce: [u8; 16],
    solution: [u8; 16],
    solves: u32,
}

fn generated() -> Vec<Gen> {
    GEN.lines()
        .filter(|l| l.starts_with("pow "))
        .map(|l| {
            let f: Vec<&str> = l.split(' ').collect();
            Gen {
                seed: arr(f[1]),
                blinded: arr(f[2]),
                first: arr(f[3]),
                effort: f[4].parse().unwrap(),
                nonce: arr(f[5]),
                solution: arr(f[6]),
                solves: f[7].parse().unwrap(),
            }
        })
        .collect()
}

/// Runs a puzzle in the slices the capsule uses until it is solved.
fn run(puzzle: &mut Puzzle, budget: u32) -> PowSolution {
    for _ in 0..100_000 {
        if let Some(s) = puzzle.step(budget) {
            return s;
        }
    }
    panic!("never solved");
}

#[test]
fn the_puzzle_finds_what_the_forks_solve_loop_finds() {
    let all = generated();
    assert_eq!(all.len(), 5);
    for g in all {
        let params = PowParams { seed: g.seed, suggested_effort: g.effort, expires: 0 };
        let mut puzzle = Puzzle::new(&g.blinded, &params, g.effort, g.first).expect("host heap");
        let s = run(&mut puzzle, 4096);
        assert_eq!(s.nonce, g.nonce, "effort {}", g.effort);
        assert_eq!(s.solution, g.solution, "effort {}", g.effort);
        assert_eq!(s.effort, g.effort);
        assert_eq!(s.seed_head, g.seed[..4]);
        assert_eq!(puzzle.solves + 1, g.solves, "Equi-X instances");
    }
}

#[test]
fn slices_of_any_size_reach_the_same_solution() {
    let g = generated().into_iter().find(|g| g.solves == 3).expect("the effort 8 line");
    let params = PowParams { seed: g.seed, suggested_effort: g.effort, expires: 0 };
    for budget in [1000, 65536, u32::MAX] {
        let mut puzzle = Puzzle::new(&g.blinded, &params, g.effort, g.first).unwrap();
        assert_eq!(run(&mut puzzle, budget).solution, g.solution, "budget {budget}");
    }
}

#[test]
fn a_challenge_without_a_program_is_skipped_like_an_unlucky_nonce() {
    // Search for a nonce whose challenge selects a program that fails the
    // uniformity rules, about one in ten thousand; then check the puzzle
    // counts it as one instance and moves on to the next nonce.
    let blinded = [0x11; 32];
    let params = PowParams { seed: [0xAA; 32], suggested_effort: 1, expires: 0 };
    let mut challenge = b"Tor hs intro v1\0".to_vec();
    challenge.extend_from_slice(&blinded);
    challenge.extend_from_slice(&params.seed);
    challenge.extend_from_slice(&[0; 16]);
    challenge.extend_from_slice(&1u32.to_be_bytes());
    let bad = (0u32..200_000)
        .find(|n| {
            challenge[80..84].copy_from_slice(&n.to_le_bytes());
            nonos_equix::HashX::new(&challenge).is_none()
        })
        .expect("a refused challenge exists in range");
    let mut nonce = [0u8; 16];
    nonce[..4].copy_from_slice(&bad.to_le_bytes());
    let mut puzzle = Puzzle::new(&blinded, &params, 1, nonce).unwrap();
    assert_eq!(puzzle.step(u32::MAX), None);
    assert_eq!(puzzle.solves, 1);
    let s = run(&mut puzzle, u32::MAX);
    assert_ne!(s.nonce, nonce, "the refused nonce is never submitted");
}

fn point() -> IntroPoint {
    let blinded = arr("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8");
    decode(DESC, &blinded, &[0x55; 32], NOW, |_| None).unwrap().intro_points.remove(0)
}

#[test]
fn the_cache_keeps_the_puzzle_and_refetches_when_its_seed_expires() {
    let mut c = DescCache::default();
    let key = CacheKey { identity: [1; 32], blinded: [2; 32], period: 7 };
    let pow = PowParams { seed: [3; 32], suggested_effort: 50, expires: 5000 };
    assert!(c.insert(key, 1, 10_000, 0, alloc::vec![point()], Some(pow)));
    assert_eq!(c.get(&[2; 32], 7, 5000).unwrap().1, Some(pow));
    assert!(c.get(&[2; 32], 7, 5001).is_none(), "a stale seed makes the entry a miss");
    let key2 = CacheKey { identity: [4; 32], blinded: [5; 32], period: 7 };
    assert!(c.insert(key2, 1, 10_000, 0, alloc::vec![point()], None));
    assert_eq!(c.get(&[5; 32], 7, 9999).unwrap().1, None);
}

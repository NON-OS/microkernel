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

/*
 * Random hostile input against the two gates every trailer meets: the v4
 * parse and the path check. Arbitrary bytes, every truncation, and random
 * multi-byte damage to a good trailer. None may panic, and nothing but the
 * enrolled trailer, unchanged, may verify for its own slot. The generator is
 * a fixed xorshift, so a failure names a seed that reproduces it.
 */

use super::fixture::{capsule_ctx, policy, DEPTH, EPOCH};
use crate::leaf::Kind;
use crate::v4::{encode_v4, parse_v4, MAX_PROOF_V4};
use crate::verify::verify;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next() as u8).collect()
    }
}

fn enrolled() -> ([u8; 32], Vec<u8>, Vec<u8>) {
    let (root, tree, cap, _) = policy();
    let path = tree.trailer(0).expect("trailer");
    let ctx = capsule_ctx(&cap, 0x7, EPOCH);
    assert!(verify(&root, DEPTH, Kind::Capsule, &ctx, &path), "fixture must verify");
    (root, path, ctx)
}

#[test]
fn arbitrary_bytes_never_parse_into_a_verifying_path() {
    let (root, _, ctx) = enrolled();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for round in 0..20_000 {
        let len = rng.below(4096);
        let t = rng.bytes(len);
        assert!(!verify(&root, DEPTH, Kind::Capsule, &ctx, &t), "round {round}");
        if let Some(v) = parse_v4(&t, Kind::Capsule, MAX_PROOF_V4) {
            assert!(!verify(&root, DEPTH, Kind::Capsule, &ctx, v.path), "round {round}");
        }
    }
}

#[test]
fn a_random_header_on_a_real_path_never_verifies_for_another_slot() {
    let (root, path, ctx) = enrolled();
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for round in 0..5_000 {
        let mut t = rng.bytes(13);
        t[..8].copy_from_slice(&path[..8.min(path.len())]);
        t.extend_from_slice(&path);
        // The capsule's own path verifies only with its own context.
        let other = capsule_ctx(&rng.bytes(16), rng.next(), EPOCH);
        if let Some(v) = parse_v4(&t, Kind::Capsule, MAX_PROOF_V4) {
            assert!(!verify(&root, DEPTH, Kind::Capsule, &other, v.path), "round {round}");
        }
        assert!(!verify(&root, DEPTH, Kind::Capsule, &other, &path), "round {round}");
    }
    assert!(verify(&root, DEPTH, Kind::Capsule, &ctx, &path));
}

#[test]
fn every_truncation_of_an_enrolled_trailer_is_refused() {
    let (root, path, ctx) = enrolled();
    for cut in 0..path.len() {
        assert!(!verify(&root, DEPTH, Kind::Capsule, &ctx, &path[..cut]), "cut at {cut}");
    }
    let t = encode_v4(Kind::Capsule, &path, &[7u8; 96]).expect("encode");
    for cut in 0..t.len() {
        assert!(parse_v4(&t[..cut], Kind::Capsule, MAX_PROOF_V4).is_none(), "v4 cut at {cut}");
    }
}

#[test]
fn random_damage_to_an_enrolled_path_never_verifies() {
    let (root, path, ctx) = enrolled();
    let mut rng = Rng(0xd1b5_4a32_d192_ed03);
    for round in 0..20_000 {
        let mut bad = path.clone();
        let hits = 1 + rng.below(8);
        for _ in 0..hits {
            let at = rng.below(bad.len());
            let flip = (rng.next() as u8) | 1;
            bad[at] ^= flip;
        }
        assert!(!verify(&root, DEPTH, Kind::Capsule, &ctx, &bad), "round {round}");
    }
}

#[test]
fn damage_to_a_v4_header_or_path_is_refused_and_proof_bytes_stay_proof_bytes() {
    let (root, path, ctx) = enrolled();
    let proof = [0x5au8; 128];
    let t = encode_v4(Kind::Capsule, &path, &proof).expect("encode");
    let proof_at = t.len() - proof.len();
    let mut rng = Rng(0x94d0_49bb_1331_11eb);
    for round in 0..20_000 {
        let mut bad = t.clone();
        let at = rng.below(bad.len());
        bad[at] ^= (rng.next() as u8) | 1;
        match parse_v4(&bad, Kind::Capsule, MAX_PROOF_V4) {
            None => assert!(at < proof_at, "round {round}: damage in the proof refused the parse"),
            Some(v) if at < proof_at => {
                assert!(
                    !verify(&root, DEPTH, Kind::Capsule, &ctx, v.path),
                    "round {round}: a damaged header or path at {at} still verified"
                );
            }
            Some(v) => {
                // The proof is the STARK verifier's to judge; the path is untouched.
                assert_eq!(v.path, &path[..], "round {round}");
                assert!(verify(&root, DEPTH, Kind::Capsule, &ctx, v.path));
                assert_ne!(v.proof, &proof[..], "round {round}");
            }
        }
    }
}

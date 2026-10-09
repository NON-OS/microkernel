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

//! The STARK half under attack: a proof is about one slot only, and the gate's
//! two checks must be about the same slot.

use nonos_attest_path::{encode_v4, parse_v4, Kind, MAX_PROOF_V4};

use crate::context::capsule_context;
use crate::selftest::check;
use crate::stark::check_v4;

fn parts(t: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let v = parse_v4(t, Kind::Capsule, MAX_PROOF_V4).unwrap_or_else(|| crate::io::die("v4 parse"));
    (v.path.to_vec(), v.proof.to_vec())
}

pub fn run(root: &[u8; 32], a: &[u8; 32], b: &[u8; 32], trailers: &[Vec<u8>]) {
    let ok = |img, caps, t: &[u8]| check_v4(root, Kind::Capsule, &capsule_context(img, caps), t).is_ok();
    let (path_a, proof_a) = parts(&trailers[0]);
    let (path_b, proof_b) = parts(&trailers[1]);

    let spliced = |path: &[u8], proof: &[u8]| encode_v4(Kind::Capsule, path, proof).unwrap_or_default();
    check(!ok(a, 0x7, &spliced(&path_a, &proof_b)), "capsule A's path with capsule B's proof");
    check(!ok(b, 0x11, &spliced(&path_b, &proof_a)), "capsule B's path with capsule A's proof");

    for at in [proof_a.len() / 3, proof_a.len() / 2, proof_a.len() - 1] {
        let mut bad = proof_a.clone();
        bad[at] ^= 0x01;
        check(!ok(a, 0x7, &spliced(&path_a, &bad)), &format!("a proof with byte {at} flipped"));
    }
    check(!ok(a, 0x7, &spliced(&path_a, &proof_a[..proof_a.len() - 1])), "a truncated proof");
    check(!ok(a, 0x7, &path_a), "a bare v3 path where v4 is required");
}

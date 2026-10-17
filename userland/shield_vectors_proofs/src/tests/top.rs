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

//! The committed 48-byte top, fixtures/periodic-synthetic.top: its sha256
//! pinned, its bytes the derivation the README names at this nox_prover,
//! taken, and refused with any one bit of its root changed.

use sha2::{Digest, Sha256};

pub(super) static SYNTHETIC: &[u8] = include_bytes!("../../fixtures/periodic-synthetic.top");

const SYNTHETIC_SHA256: &str = "820b8d2098bb8315d1bc56dd1da19c57408650ac83fe94d5d16f629025647f6b";

/// Where the root starts: after NXTT, the cut, the level count, the length.
const ROOT_AT: usize = 16;

pub(super) fn sha256(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

/// A tree top of one level of one node, `root`, in nox_prover's file format:
/// NXTT, cut 0, one level, that level's length 1, then the node.
fn top(root: &[u8]) -> Vec<u8> {
    let mut b = b"NXTT".to_vec();
    for word in [0u32, 1, 1] {
        b.extend_from_slice(&word.to_le_bytes());
    }
    b.extend_from_slice(root);
    b
}

/// The fixture with bit `bit` of its root changed.
pub(super) fn flipped(bit: usize) -> Vec<u8> {
    let mut t = SYNTHETIC.to_vec();
    t[ROOT_AT + bit / 8] ^= 1 << (bit % 8);
    t
}

#[test]
fn the_fixture_is_the_pinned_bytes() {
    assert_eq!(SYNTHETIC.len(), 48);
    assert_eq!(sha256(SYNTHETIC), SYNTHETIC_SHA256, "fixtures/periodic-synthetic.top changed");
}

#[test]
fn the_fixture_is_its_derivation_at_this_prover() {
    assert_eq!(SYNTHETIC, top(&nox_prover::PERIODIC_ROOT));
}

#[test]
fn the_fixture_is_taken_and_any_root_bit_flipped_is_refused() {
    assert!(nox_prover::load_cache(SYNTHETIC).is_ok());
    for bit in 0..8 * nox_prover::PERIODIC_ROOT.len() {
        assert!(nox_prover::load_cache(&flipped(bit)).is_err(), "root bit {bit} flipped was taken");
    }
}

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

//! The prover's PERIODIC_ROOT check, on the files this capsule may be handed.

use super::{accepted, SHIPPED};

/// A tree top of one level holding `root`, in nox_prover's file format:
/// the magic, the cut, the level count, then each level's length and nodes.
fn top(root: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(b"NXTT");
    b.extend_from_slice(&0u32.to_le_bytes());
    b.extend_from_slice(&1u32.to_le_bytes());
    b.extend_from_slice(&1u32.to_le_bytes());
    b.extend_from_slice(root);
    b
}

#[test]
fn a_tree_under_the_circuits_root_is_taken() {
    assert!(accepted(&top(&nox_prover::PERIODIC_ROOT)));
}

#[test]
fn a_tree_under_any_other_root_is_refused() {
    for byte in 0..nox_prover::PERIODIC_ROOT.len() {
        let mut root = nox_prover::PERIODIC_ROOT;
        root[byte] ^= 1;
        assert!(!accepted(&top(&root)), "a root differing in byte {byte} was taken");
    }
    assert!(!accepted(&top(&[0u8; 32])));
}

#[test]
fn a_malformed_file_is_refused() {
    let good = top(&nox_prover::PERIODIC_ROOT);
    assert!(!accepted(&[]));
    assert!(!accepted(&good[..good.len() - 1]));
    let mut longer = good.clone();
    longer.push(0);
    assert!(!accepted(&longer));
    let mut magic = good;
    magic[0] = b'X';
    assert!(!accepted(&magic));
}

/// The shipped file, when this build embeds one (the Nix build always does):
/// taken whole, refused once its root is touched, and refused cut short.
#[test]
fn the_shipped_cache_is_taken_and_a_touched_copy_is_not() {
    if option_env!("NONOS_PERIODIC_CACHE").is_none() {
        assert!(SHIPPED.is_empty(), "a cache was embedded that no build named");
        return;
    }
    assert!(accepted(SHIPPED));
    let mut touched = SHIPPED.to_vec();
    let last = touched.len() - 1;
    touched[last] ^= 0x80;
    assert!(!accepted(&touched));
    assert!(!accepted(&SHIPPED[..SHIPPED.len() / 2]));
}

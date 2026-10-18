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

//! A leaf's proof in its tree: the sibling at each level, none where the
//! node is promoted unpaired. `verify` hashes up through the proof as
//! OpenZeppelin's MerkleProof does, so dispute() accepts what this builds.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use crate::tree::{pair, up};

/// The proof for leaf `i`; None past the last leaf.
pub fn proof(k: &impl Keccak, leaves: &[[u8; 32]], mut i: usize) -> Option<Vec<[u8; 32]>> {
    if i >= leaves.len() {
        return None;
    }
    let mut level = leaves.to_vec();
    let mut out = Vec::new();
    while level.len() > 1 {
        if let Some(sib) = level.get(i ^ 1) {
            out.push(*sib);
        }
        level = up(k, &level);
        i /= 2;
    }
    Some(out)
}

pub fn verify(k: &impl Keccak, leaf: &[u8; 32], proof: &[[u8; 32]], root: &[u8; 32]) -> bool {
    proof.iter().fold(*leaf, |node, sib| pair(k, &node, sib)) == *root
}

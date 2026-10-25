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

//! The sorted-pair tree OpenZeppelin's MerkleProof verifies: a pair is hashed
//! smaller first, so a proof carries no left-or-right bits. Leaves are sorted
//! before the tree is built, and an odd node at the end of a level is carried
//! up unhashed. The root of one leaf is that leaf; of none, zero.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

pub fn hash_pair(k: &impl Keccak, a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut pre = [0u8; 64];
    pre[..32].copy_from_slice(lo);
    pre[32..].copy_from_slice(hi);
    k.keccak256(&pre)
}

pub fn root(k: &impl Keccak, leaves: &[[u8; 32]]) -> [u8; 32] {
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    level.sort_unstable();
    if level.is_empty() {
        return [0u8; 32];
    }
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for pair in level.chunks(2) {
            next.push(if let [a, b] = pair { hash_pair(k, a, b) } else { pair[0] });
        }
        level = next;
    }
    level[0]
}

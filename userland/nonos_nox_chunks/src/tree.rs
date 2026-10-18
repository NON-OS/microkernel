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

//! The root over leaves in index order: pairs hash sorted (OpenZeppelin's
//! MerkleProof), and a level with an odd count promotes its last node
//! unpaired. One leaf is its own root; no leaves have none.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

pub(crate) fn pair(k: &impl Keccak, a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut pre = [0u8; 64];
    pre[..32].copy_from_slice(lo);
    pre[32..].copy_from_slice(hi);
    k.keccak256(&pre)
}

pub(crate) fn up(k: &impl Keccak, level: &[[u8; 32]]) -> Vec<[u8; 32]> {
    level
        .chunks(2)
        .map(|p| match p {
            [a, b] => pair(k, a, b),
            [a] => *a,
            _ => unreachable!(),
        })
        .collect()
}

pub fn root(k: &impl Keccak, leaves: &[[u8; 32]]) -> Option<[u8; 32]> {
    let mut level = leaves.to_vec();
    while level.len() > 1 {
        level = up(k, &level);
    }
    level.first().copied()
}

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

//! A tree's leaf: keccak256(abi.encode(uint8 tree, uint64 i, bytes32 h)),
//! tree 0 for the plain chunks and 1 for the cipher chunks, so a leaf of one
//! tree is never a leaf of the other.

use nonos_marketplace_abi::Keccak;

pub const PLAIN_TREE: u8 = 0;
pub const CIPHER_TREE: u8 = 1;

pub fn leaf(k: &impl Keccak, tree: u8, i: u64, h: &[u8; 32]) -> [u8; 32] {
    let mut pre = [0u8; 96];
    pre[31] = tree;
    pre[56..64].copy_from_slice(&i.to_be_bytes());
    pre[64..].copy_from_slice(h);
    k.keccak256(&pre)
}

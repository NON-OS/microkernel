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

//! The roots an escrow release and order commit to. chunk_root is over
//! leaf(0, i, keccak256(chunk_i)), cipherRoot over leaf(1, i,
//! keccak256(c_i)). The buyer gets the plain hash list from the seller, so
//! it checks chunk_root from hashes alone, before it holds any plaintext.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use crate::chunk::{chunk, chunk_count};
use crate::cipher::xor_chunk;
use crate::leaf::{leaf, CIPHER_TREE, PLAIN_TREE};
use crate::tree::root;

/// keccak256 of each padded chunk of `content`.
pub fn plain_hashes(k: &impl Keccak, content: &[u8]) -> Vec<[u8; 32]> {
    (0..chunk_count(content.len() as u64))
        .filter_map(|i| chunk(content, i))
        .map(|c| k.keccak256(&c))
        .collect()
}

/// The root of `tree` over chunk hashes in index order.
pub fn root_of_hashes(k: &impl Keccak, tree: u8, hashes: &[[u8; 32]]) -> Option<[u8; 32]> {
    let leaves: Vec<[u8; 32]> =
        hashes.iter().enumerate().map(|(i, h)| leaf(k, tree, i as u64, h)).collect();
    root(k, &leaves)
}

pub fn chunk_root(k: &impl Keccak, content: &[u8]) -> Option<[u8; 32]> {
    root_of_hashes(k, PLAIN_TREE, &plain_hashes(k, content))
}

/// cipherRoot of `content` under `key`, as the seller computes it.
pub fn cipher_root(k: &impl Keccak, key: &[u8; 32], content: &[u8]) -> Option<[u8; 32]> {
    let hashes: Vec<[u8; 32]> = (0..chunk_count(content.len() as u64))
        .filter_map(|i| Some(k.keccak256(&xor_chunk(k, key, i, &chunk(content, i)?))))
        .collect();
    root_of_hashes(k, CIPHER_TREE, &hashes)
}

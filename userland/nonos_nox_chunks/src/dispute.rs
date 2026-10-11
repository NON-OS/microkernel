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

//! A dispute's arguments for chunk i: the cipher chunk,
//! its listed plain hash, the proof of leaf(1, i, keccak256(c_i)) in
//! cipherRoot and the proof of leaf(0, i, plainHash) in chunk_root. The
//! contract decrypts c_i with the delivered K and slashes when its hash is
//! not plainHash.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use crate::chunk::CHUNK;
use crate::leaf::{leaf, CIPHER_TREE, PLAIN_TREE};
use crate::offer::cipher_hashes;
use crate::proof::proof;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dispute {
    pub index: u64,
    pub cipher_chunk: Vec<u8>,
    pub plain_hash: [u8; 32],
    pub cipher_proof: Vec<[u8; 32]>,
    pub leaf_proof: Vec<[u8; 32]>,
}

pub fn dispute(k: &impl Keccak, i: u64, hashes: &[[u8; 32]], ciphertext: &[u8]) -> Option<Dispute> {
    let at = usize::try_from(i).ok()?;
    // An index past the content never wraps onto another chunk.
    let start = at.checked_mul(CHUNK)?;
    let cipher_chunk = ciphertext.get(start..start.checked_add(CHUNK)?)?.to_vec();
    let c = cipher_hashes(k, ciphertext)?;
    let leaves = |tree: u8, hs: &[[u8; 32]]| -> Vec<[u8; 32]> {
        hs.iter().enumerate().map(|(j, h)| leaf(k, tree, j as u64, h)).collect()
    };
    Some(Dispute {
        index: i,
        cipher_chunk,
        plain_hash: *hashes.get(at)?,
        cipher_proof: proof(k, &leaves(CIPHER_TREE, &c), at)?,
        leaf_proof: proof(k, &leaves(PLAIN_TREE, hashes), at)?,
    })
}

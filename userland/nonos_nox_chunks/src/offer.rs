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

//! The buyer's check of a seller's offer, before funding:
//! the plain hash list must make the release's chunk_root for its
//! content_len, and the ciphertext, whole chunks only, must make the
//! offered cipherRoot. Neither needs the key.

use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use crate::chunk::{chunk_count, CHUNK};
use crate::leaf::{CIPHER_TREE, PLAIN_TREE};
use crate::roots::root_of_hashes;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OfferWrong {
    /// Not one hash per chunk of content_len, or no content at all.
    HashCount,
    /// The hash list does not make the release's chunk_root.
    ChunkRoot,
    /// The ciphertext is not one whole chunk per hash.
    CipherLength,
    /// The ciphertext does not make the offered cipherRoot.
    CipherRoot,
}

/// keccak256 of each whole cipher chunk; None unless whole chunks.
pub fn cipher_hashes(k: &impl Keccak, ciphertext: &[u8]) -> Option<Vec<[u8; 32]>> {
    ciphertext
        .len()
        .is_multiple_of(CHUNK)
        .then(|| ciphertext.chunks(CHUNK).map(|c| k.keccak256(c)).collect())
}

pub fn check_offer(
    k: &impl Keccak,
    chunk_root: &[u8; 32],
    content_len: u64,
    hashes: &[[u8; 32]],
    ciphertext: &[u8],
    cipher_root: &[u8; 32],
) -> Result<(), OfferWrong> {
    if hashes.is_empty() || hashes.len() as u64 != chunk_count(content_len) {
        return Err(OfferWrong::HashCount);
    }
    if root_of_hashes(k, PLAIN_TREE, hashes).as_ref() != Some(chunk_root) {
        return Err(OfferWrong::ChunkRoot);
    }
    let c = cipher_hashes(k, ciphertext)
        .filter(|c| c.len() == hashes.len())
        .ok_or(OfferWrong::CipherLength)?;
    if root_of_hashes(k, CIPHER_TREE, &c).as_ref() != Some(cipher_root) {
        return Err(OfferWrong::CipherRoot);
    }
    Ok(())
}

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

//! The seller's offer for one request: the release's bytes
//! under a fresh K, chunk by chunk, with the plain hash list, cipherRoot and
//! keyCommit = keccak256(abi.encode(K)).

use nonos_nox_chunks::{chunk, chunk_count, cipher_hashes, key_commit, plain_hashes};
use nonos_nox_chunks::{root_of_hashes, xor_chunk, Offer, CIPHER_TREE};

use crate::hosts::Host;

pub fn offer(content: &[u8], key: &[u8; 32]) -> Option<Offer> {
    let mut ciphertext = Vec::with_capacity(content.len());
    for i in 0..chunk_count(content.len() as u64) {
        ciphertext.extend_from_slice(&xor_chunk(&Host, key, i, &chunk(content, i)?));
    }
    let cipher_root = root_of_hashes(&Host, CIPHER_TREE, &cipher_hashes(&Host, &ciphertext)?)?;
    let hashes = plain_hashes(&Host, content);
    Some(Offer { cipher_root, key_commit: key_commit(&Host, key), hashes, ciphertext })
}

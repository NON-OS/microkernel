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

//! A seller's offer for the tests: the hash list, the ciphertext under KEY
//! and its root, honest or lying in one chunk.

use nonos_nox_chunks::CIPHER_TREE;
use nonos_nox_chunks::{
    chunk, chunk_count, cipher_hashes, plain_hashes, root_of_hashes, xor_chunk,
};

use crate::k::K;

pub const KEY: [u8; 32] = [0x42; 32];

pub fn content() -> Vec<u8> {
    (0..3000u32).map(|i| (i % 251) as u8).collect()
}

/// What a seller sends: the hash list, the ciphertext and its root. A
/// lying seller encrypts other bytes for chunk `lie`.
pub fn offer(content: &[u8], lie: Option<u64>) -> (Vec<[u8; 32]>, Vec<u8>, [u8; 32]) {
    let mut ct = Vec::new();
    for i in 0..chunk_count(content.len() as u64) {
        let mut c = chunk(content, i).unwrap();
        if lie == Some(i) {
            c[7] ^= 0xff;
        }
        ct.extend_from_slice(&xor_chunk(&K, &KEY, i, &c));
    }
    let root = root_of_hashes(&K, CIPHER_TREE, &cipher_hashes(&K, &ct).unwrap()).unwrap();
    (plain_hashes(&K, content), ct, root)
}

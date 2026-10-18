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

//! A seller for the tests: an offer of CONTENT under K, sealed by an agent
//! key to a buyer for this release, as escrow-serve answers.

use nonos_nox_chunks::{chunk, chunk_count, chunk_root, cipher_hashes, key_commit, plain_hashes};
use nonos_nox_chunks::{root_of_hashes, seal_answer, xor_chunk, Offer, CIPHER_TREE};

use super::host::Host;
use super::x::D;

pub const K: [u8; 32] = [0x42; 32];
pub const AGENT: [u8; 32] = [0x0a; 32];

pub fn content() -> Vec<u8> {
    (0..70_000u32).map(|i| (i % 251) as u8).collect()
}

pub fn root() -> [u8; 32] {
    chunk_root(&Host, &content()).unwrap()
}

pub fn offer() -> Offer {
    let c = content();
    let mut ciphertext = Vec::new();
    for i in 0..chunk_count(c.len() as u64) {
        ciphertext.extend_from_slice(&xor_chunk(&Host, &K, i, &chunk(&c, i).unwrap()));
    }
    let cipher_root =
        root_of_hashes(&Host, CIPHER_TREE, &cipher_hashes(&Host, &ciphertext).unwrap()).unwrap();
    Offer {
        cipher_root,
        key_commit: key_commit(&Host, &K),
        hashes: plain_hashes(&Host, &c),
        ciphertext,
    }
}

/// The sealed answer to buyer_pk for (listing, release).
pub fn sealed(buyer_pk: &[u8; 32], listing: &[u8; 32], release: &[u8; 32]) -> Vec<u8> {
    seal_answer((&Host, &D), &AGENT, buyer_pk, (listing, release), &offer()).unwrap()
}

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

//! The offer inside the escrow agent's sealed answer: cipherRoot[32]
//! | keyCommit[32] | n plain chunk hashes[32] | n cipher chunks[CHUNK], with
//! n = ceil(content_len / CHUNK) from the request, never from the seller. Any
//! other length is refused. The buyer runs check_offer on it before ORDER_NEW.

use alloc::vec::Vec;

use crate::chunk::{chunk_count, CHUNK};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Offer {
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
    pub hashes: Vec<[u8; 32]>,
    pub ciphertext: Vec<u8>,
}

/// The inner length for `content_len`, or None for no content or past usize.
pub fn offer_len(content_len: u64) -> Option<usize> {
    let n = usize::try_from(chunk_count(content_len)).ok().filter(|n| *n > 0)?;
    n.checked_mul(32 + CHUNK)?.checked_add(64)
}

impl Offer {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = [self.cipher_root, self.key_commit].concat();
        self.hashes.iter().for_each(|h| out.extend_from_slice(h));
        out.extend_from_slice(&self.ciphertext);
        out
    }

    /// The offer for `content_len`, or None unless exactly offer_len bytes.
    pub fn parse(b: &[u8], content_len: u64) -> Option<Self> {
        (Some(b.len()) == offer_len(content_len)).then_some(())?;
        let n = usize::try_from(chunk_count(content_len)).ok()?;
        let end = n.checked_mul(32)?.checked_add(64)?;
        let hashes =
            b.get(64..end)?.chunks(32).map(|h| h.try_into().ok()).collect::<Option<_>>()?;
        Some(Offer {
            cipher_root: b.get(..32)?.try_into().ok()?,
            key_commit: b.get(32..64)?.try_into().ok()?,
            hashes,
            ciphertext: b.get(end..)?.to_vec(),
        })
    }
}

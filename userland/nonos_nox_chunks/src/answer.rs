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

//! The offer inside the escrow agent's sealed answer:
//! cipherRoot[32] | keyCommit[32] | n u32 LE | n plain chunk hashes | the
//! ciphertext, the rest. The buyer runs check_offer on it before funding.

use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Offer {
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
    pub hashes: Vec<[u8; 32]>,
    pub ciphertext: Vec<u8>,
}

impl Offer {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = [self.cipher_root, self.key_commit].concat();
        out.extend_from_slice(&(self.hashes.len() as u32).to_le_bytes());
        self.hashes.iter().for_each(|h| out.extend_from_slice(h));
        out.extend_from_slice(&self.ciphertext);
        out
    }

    /// None for a body too short for its own count.
    pub fn parse(b: &[u8]) -> Option<Self> {
        let n = u32::from_le_bytes(b.get(64..68)?.try_into().ok()?) as usize;
        let end = n.checked_mul(32)?.checked_add(68)?;
        let hashes =
            b.get(68..end)?.chunks(32).map(|h| h.try_into().ok()).collect::<Option<_>>()?;
        Some(Offer {
            cipher_root: b[..32].try_into().ok()?,
            key_commit: b[32..64].try_into().ok()?,
            hashes,
            ciphertext: b[end..].to_vec(),
        })
    }
}

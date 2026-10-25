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

//! ORDER_OFFER's result (plan/29): offerId = keccak256(cipherRoot |
//! keyCommit) | cipherRoot | keyCommit, 96 bytes.

use alloc::vec::Vec;

use crate::cursor::Cursor;

pub const OFFER_MADE_LEN: usize = 96;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OfferMade {
    pub offer_id: [u8; 32],
    pub cipher_root: [u8; 32],
    pub key_commit: [u8; 32],
}

impl OfferMade {
    pub fn encode(&self) -> Vec<u8> {
        [self.offer_id, self.cipher_root, self.key_commit].concat()
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let (offer_id, cipher_root, key_commit) = (r.fixed()?, r.fixed()?, r.fixed()?);
        r.end(OfferMade { offer_id, cipher_root, key_commit })
    }
}

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

//! An offer as the meter keeps it (plan/29), one sealed file each: chain_id
//! u64 | listing_key | release_key | chunk_root | content_len u64 | buyer_sk |
//! made_at u64 | the Offer. ORDER_NEW takes one whose keys and roots match.

use alloc::vec::Vec;

use nonos_nox_chunks::Offer;

pub const KEPT_HEAD: usize = 8 + 32 * 3 + 8 + 32 + 8;

#[derive(Clone, PartialEq, Eq)]
pub struct KeptOffer {
    pub chain_id: u64,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub buyer_sk: [u8; 32],
    pub made_at: u64,
    pub offer: Offer,
}

impl KeptOffer {
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::from(self.chain_id.to_le_bytes());
        [self.listing_key, self.release_key, self.chunk_root].iter().for_each(|k| out.extend(k));
        out.extend_from_slice(&self.content_len.to_le_bytes());
        out.extend_from_slice(&self.buyer_sk);
        out.extend_from_slice(&self.made_at.to_le_bytes());
        out.extend_from_slice(&self.offer.encode());
        out
    }

    /// One record, or None unless its offer is exactly its content_len's.
    pub fn decode(b: &[u8]) -> Option<Self> {
        let w = |at: usize| -> Option<[u8; 32]> { b.get(at..at + 32)?.try_into().ok() };
        let n = |at: usize| -> Option<u64> {
            Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
        };
        let content_len = n(104)?;
        Some(KeptOffer {
            chain_id: n(0)?,
            listing_key: w(8)?,
            release_key: w(40)?,
            chunk_root: w(72)?,
            content_len,
            buyer_sk: w(112)?,
            made_at: n(144)?,
            offer: Offer::parse(b.get(KEPT_HEAD..)?, content_len)?,
        })
    }
}

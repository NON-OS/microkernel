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

//! ORDER_OFFER (op 17, the plan), from the wallet alone; a job (op 15).
//! Request, LE: chain_id u64 | listing_key[32] | release_key[32] |
//! chunk_root[32] | content_len u64 | lp(u16) package_url, at most MAX_URL.
//! Its result is OfferMade (offer_made.rs).

use alloc::vec::Vec;

use crate::cursor::Cursor;
use crate::cursor_put::put_lp;

/// The longest package_url, as settle's FETCH takes it.
pub const MAX_URL: usize = 256;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OfferAsk {
    pub chain_id: u64,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    pub url: Vec<u8>,
}

impl OfferAsk {
    pub fn encode(&self) -> Option<Vec<u8>> {
        (self.url.len() <= MAX_URL).then_some(())?;
        let mut out = Vec::from(self.chain_id.to_le_bytes());
        [self.listing_key, self.release_key, self.chunk_root].iter().for_each(|k| out.extend(k));
        out.extend_from_slice(&self.content_len.to_le_bytes());
        put_lp(&mut out, &self.url)?;
        Some(out)
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let mut r = Cursor::new(b);
        let chain_id = r.u64()?;
        let (listing_key, release_key, chunk_root) = (r.fixed()?, r.fixed()?, r.fixed()?);
        let content_len = r.u64()?;
        let url = r.lp()?.to_vec();
        (url.len() <= MAX_URL).then_some(())?;
        r.end(OfferAsk { chain_id, listing_key, release_key, chunk_root, content_len, url })
    }
}

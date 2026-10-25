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

//! FETCH 1 ESCROW, from payment only: chain_id
//! u64 | lp(u16) package_url | listing_key | release_key | buyer_pk[32] |
//! lp(u16) the Equi-X solution. Settle POSTs `agent_body` once to the url
//! and pages the sealed answer back unopened, keyed by buyer_pk.

use alloc::vec::Vec;

use super::form::{FETCH_ESCROW, MAX_URL};

use crate::bytes::len_u16;

pub const MAX_SOLUTION: usize = 256;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Escrow {
    pub chain_id: u64,
    pub url: Vec<u8>,
    pub listing_key: [u8; 32],
    pub release_key: [u8; 32],
    pub buyer_pk: [u8; 32],
    pub solution: Vec<u8>,
}

impl Escrow {
    /// The FETCH body, form byte first; None past a length limit.
    pub fn encode(&self) -> Option<Vec<u8>> {
        if self.url.len() > MAX_URL || self.solution.len() > MAX_SOLUTION {
            return None;
        }
        let mut out = Vec::from([FETCH_ESCROW]);
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.extend_from_slice(&len_u16(self.url.len())?.to_le_bytes());
        out.extend_from_slice(&self.url);
        Some([out, self.agent_body()?.get(8..)?.to_vec()].concat())
    }

    /// What the agent is sent: the request without its url, chain_id first;
    /// None for a solution whose length does not fit its u16.
    pub fn agent_body(&self) -> Option<Vec<u8>> {
        let mut out = Vec::from(self.chain_id.to_le_bytes());
        [self.listing_key, self.release_key, self.buyer_pk].iter().for_each(|k| out.extend(k));
        out.extend_from_slice(&len_u16(self.solution.len())?.to_le_bytes());
        out.extend_from_slice(&self.solution);
        Some(out)
    }
}

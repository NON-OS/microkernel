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

//! nonos.prove's ops and their bodies, all little-endian (plan/14). One job at
//! a time: a second PROVE is EBUSY. STATUS and GET answer only the endpoint
//! whose PROVE made the job; anyone else gets ENOENT.
//!
//!   1 PROVE   listing_id[32] | policy[256]      -> job u32
//!   2 STATUS  job u32                           -> stage u8 | total u32
//!   3 GET     job u32 | offset u32              -> container[offset..], at most GET_MAX
//!   4 HEALTH  (empty)                           -> (empty)
//!
//! `total` is the container's length once the stage is Done, else 0.

pub const PROVE: u16 = 1;
pub const STATUS: u16 = 2;
pub const GET: u16 = 3;
pub const HEALTH: u16 = 4;

pub const GET_MAX: usize = 64 * 1024;
pub const PROVE_BODY: usize = 32 + 256;

pub struct ProveRequest<'a> {
    pub listing_id: &'a [u8; 32],
    pub policy: &'a [u8; 256],
}

impl<'a> ProveRequest<'a> {
    pub fn encode(&self) -> [u8; PROVE_BODY] {
        let mut b = [0u8; PROVE_BODY];
        b[..32].copy_from_slice(self.listing_id);
        b[32..].copy_from_slice(self.policy);
        b
    }

    /// Exactly PROVE_BODY bytes, or nothing.
    pub fn decode(b: &'a [u8]) -> Option<ProveRequest<'a>> {
        if b.len() != PROVE_BODY {
            return None;
        }
        Some(ProveRequest {
            listing_id: b[..32].try_into().ok()?,
            policy: b[32..].try_into().ok()?,
        })
    }
}

/// STATUS's and GET's job word, and GET's offset after it.
pub fn job_of(b: &[u8]) -> Option<(u32, Option<u32>)> {
    let word = |i: usize| Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?));
    match b.len() {
        4 => Some((word(0)?, None)),
        8 => Some((word(0)?, Some(word(4)?))),
        _ => None,
    }
}

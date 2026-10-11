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

//! nonos.prove.fetch's ops and their bodies, all little-endian (plan/14), in
//! the same header under "NPFT". One transfer at a time: another is EBUSY.
//! PUT answers app.store only; GET and STATUS app.store and market.index.
//!
//!   1 PUT     listing_id[32] | total u32 | offset u32 | chunk   -> anchor[32] on the last
//!   2 GET     listing_id[32] | anchor[32] | offset u32          -> container[offset..]
//!   3 STATUS  anchor[32]                                        -> stage u8 | total u32
//!
//! Chunks come in order, each at most CHUNK_MAX, the whole at most 1 MiB. A
//! GET for a container not yet held is PENDING and starts its download.

use alloc::vec::Vec;

use crate::container::MAX_BYTES;
use crate::wire::GET_MAX;

/// "NPFT", nonos.prove.fetch's magic.
pub const FETCH_MAGIC: u32 = 0x5446_504E;
pub const PUT: u16 = 1;
pub const GET: u16 = 2;
pub const STATUS: u16 = 3;

pub const PUT_HEAD: usize = 40;
/// A PUT frame is no larger than a GET's reply.
pub const CHUNK_MAX: usize = GET_MAX - PUT_HEAD;
pub const GET_BODY: usize = 68;

pub struct Put<'a> {
    pub listing_id: &'a [u8; 32],
    pub total: u32,
    pub offset: u32,
    pub chunk: &'a [u8],
}

impl<'a> Put<'a> {
    pub fn encode(&self) -> Vec<u8> {
        let head = [&self.total.to_le_bytes()[..], &self.offset.to_le_bytes()];
        [&self.listing_id[..], head[0], head[1], self.chunk].concat()
    }

    /// `None` for an empty or oversized chunk, a total over 1 MiB, or a chunk
    /// that runs past the total.
    pub fn decode(b: &'a [u8]) -> Option<Put<'a>> {
        let word = |i: usize| Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?));
        let (total, offset) = (word(32)?, word(36)?);
        let chunk = &b[PUT_HEAD..];
        let end = (offset as usize).checked_add(chunk.len())?;
        if chunk.is_empty() || chunk.len() > CHUNK_MAX || total as usize > MAX_BYTES {
            return None;
        }
        let listing_id = b[..32].try_into().ok()?;
        (end <= total as usize).then_some(Put { listing_id, total, offset, chunk })
    }

    pub fn last(&self) -> bool {
        (self.offset as usize).checked_add(self.chunk.len()) == Some(self.total as usize)
    }
}

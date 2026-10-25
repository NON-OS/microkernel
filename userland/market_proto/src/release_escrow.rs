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

//! What an escrow purchase needs of `OP_GET_RELEASE` (B8, v1.4.36): the
//! package url the seller's agent answers at, and the chunk tree's root and
//! length, which the market appends after the validation time. A reply
//! without them, or with bytes past them, is refused.

use alloc::vec::Vec;

use super::lp::{lp, skip, skip_list};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReleaseEscrow {
    pub package_url: Vec<u8>,
    /// Zero, as content_len, for a release that is not escrowed.
    pub chunk_root: [u8; 32],
    pub content_len: u64,
}

pub fn parse_release_escrow(body: &[u8]) -> Option<ReleaseEscrow> {
    let (_, at) = lp(body, 0)?;
    let at = skip(body, at, 64)?;
    let (url, at) = lp(body, at)?;
    let (_, at) = lp(body, at)?;
    let at = skip(body, skip_list(body, at)?, 4)?;
    let at = skip(body, skip_list(body, at)?, 1)?;
    let (_, at) = lp(body, at)?;
    let (_, at) = lp(body, at)?;
    let at = skip(body, at, 8)?;
    let tail = body.get(at..)?;
    (tail.len() == 32 + 8).then_some(())?;
    Some(ReleaseEscrow {
        package_url: url.to_vec(),
        chunk_root: tail.get(..32)?.try_into().ok()?,
        content_len: u64::from_le_bytes(tail.get(32..)?.try_into().ok()?),
    })
}

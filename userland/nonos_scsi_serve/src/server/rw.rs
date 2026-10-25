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

//! A READ or WRITE request checked against the unit before anything reaches
//! the adapter. Pure, for the host proofs.

use crate::wire::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub lba: u64,
    pub count: u32,
}

/// The range `body` names on a unit of `blocks` blocks of `block_bytes`,
/// at most `max` blocks a request. A read carries the twelve bytes alone; a
/// write carries them and then exactly the blocks.
pub fn parse(
    body: &[u8],
    write: bool,
    blocks: u64,
    block_bytes: u32,
    max: u32,
) -> Result<Range, i32> {
    if body.len() < RW_HEADER_LEN {
        return Err(E_MSGSIZE);
    }
    let lba = u64::from_le_bytes(body[0..8].try_into().map_err(|_| E_MSGSIZE)?);
    let count = u32::from_le_bytes(body[8..12].try_into().map_err(|_| E_MSGSIZE)?);
    if count == 0 || count > max {
        return Err(E_INVAL);
    }
    let bytes = count as usize * block_bytes as usize;
    let want = if write { RW_HEADER_LEN + bytes } else { RW_HEADER_LEN };
    if body.len() != want {
        return Err(E_MSGSIZE);
    }
    match lba.checked_add(count as u64) {
        Some(end) if end <= blocks => Ok(Range { lba, count }),
        _ => Err(E_NXIO),
    }
}

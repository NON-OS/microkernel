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

//! The READ and WRITE body: lba (u64 LE), count (u32 LE), and for a WRITE
//! the blocks themselves. Every bound is checked before the card is asked
//! for anything; the arithmetic cannot wrap.

use super::errno::{E_INVAL, E_MSGSIZE, E_NXIO};
use super::ops::{BLOCK_BYTES, MAX_RW_BLOCKS, RW_HEADER_LEN};

/// A range of the card that lies inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub lba: u32,
    pub count: u16,
}

/// The range `body` asks for on a card of `blocks`, `write` when the
/// blocks follow. E_MSGSIZE for a body of the wrong length, E_INVAL for a
/// count of zero or past MAX_RW_BLOCKS, E_NXIO for a range past the end
/// or a block a 32-bit argument cannot name.
pub fn parse_rw(body: &[u8], blocks: u64, write: bool) -> Result<Span, i32> {
    let head: &[u8; RW_HEADER_LEN] =
        body.get(..RW_HEADER_LEN).and_then(|h| h.try_into().ok()).ok_or(E_MSGSIZE)?;
    let [l0, l1, l2, l3, l4, l5, l6, l7, c0, c1, c2, c3] = *head;
    let lba = u64::from_le_bytes([l0, l1, l2, l3, l4, l5, l6, l7]);
    let count = u32::from_le_bytes([c0, c1, c2, c3]);
    if count == 0 || count > MAX_RW_BLOCKS {
        return Err(E_INVAL);
    }
    let data = if write { count as usize * BLOCK_BYTES } else { 0 };
    if body.len() != RW_HEADER_LEN + data {
        return Err(E_MSGSIZE);
    }
    let end = lba.checked_add(count as u64).ok_or(E_NXIO)?;
    if end > blocks {
        return Err(E_NXIO);
    }
    let lba = u32::try_from(lba).map_err(|_| E_NXIO)?;
    Ok(Span { lba, count: count as u16 })
}

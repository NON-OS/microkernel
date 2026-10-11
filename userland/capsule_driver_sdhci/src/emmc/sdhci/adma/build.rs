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

//! Writing the transfer descriptors for one buffer into a table.

use super::{AdmaError, Width, ACT_TRAN, ATTR_END, ATTR_VALID, MAX_CHUNK};

/// Write the descriptors that move `len` bytes at `data_bus` into `table`,
/// in pieces of at most MAX_CHUNK, End on the last. Returns the number of
/// descriptors; nothing is written when it fails.
pub fn build(
    table: &mut [u8],
    width: Width,
    data_bus: u64,
    len: usize,
) -> Result<usize, AdmaError> {
    if len == 0 || !len.is_multiple_of(4) {
        return Err(AdmaError::Length);
    }
    if !data_bus.is_multiple_of(width.align()) {
        return Err(AdmaError::Align);
    }
    let end = data_bus.checked_add(len as u64).ok_or(AdmaError::Reach)?;
    if width == Width::A32 && end > 1 << 32 {
        return Err(AdmaError::Reach);
    }
    let count = len.div_ceil(MAX_CHUNK);
    let dl = width.desc_len();
    if count * dl > table.len() {
        return Err(AdmaError::Room);
    }
    for i in 0..count {
        let off = i * MAX_CHUNK;
        let piece = core::cmp::min(MAX_CHUNK, len - off);
        let mut attr = ACT_TRAN | ATTR_VALID;
        if i + 1 == count {
            attr |= ATTR_END;
        }
        let addr = data_bus + off as u64;
        let d = &mut table[i * dl..(i + 1) * dl];
        d[0..2].copy_from_slice(&attr.to_le_bytes());
        d[2..4].copy_from_slice(&(piece as u16).to_le_bytes());
        match width {
            Width::A32 => d[4..8].copy_from_slice(&(addr as u32).to_le_bytes()),
            Width::A64 => d[4..12].copy_from_slice(&addr.to_le_bytes()),
        }
    }
    Ok(count)
}

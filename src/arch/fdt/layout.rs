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

//! Where the three blocks sit inside the blob, checked before any is sliced.

use core::ops::Range;

use super::error::FdtError;
use super::header::{FdtHeader, HEADER_SIZE};
use super::limits::MAX_BLOB;

pub struct Layout {
    pub total: usize,
    pub rsvmap: Range<usize>,
    pub structure: Range<usize>,
    pub strings: Range<usize>,
}

/// The block ranges `h` describes, against `available` bytes actually there.
/// Every block has to sit past the header and inside `totalsize`; the
/// reservation map and structure block also keep the alignment their entries
/// are read at, so a misplaced block fails here rather than mid-walk.
pub fn check(h: &FdtHeader, available: usize) -> Result<Layout, FdtError> {
    let total = h.totalsize as usize;
    if total > MAX_BLOB || total > available {
        return Err(FdtError::TooLarge);
    }
    let structure = block(h.off_dt_struct, h.size_dt_struct, total)?;
    let strings = block(h.off_dt_strings, h.size_dt_strings, total)?;
    let rsv_start = h.off_mem_rsvmap as usize;
    if rsv_start < HEADER_SIZE || rsv_start >= total {
        return Err(FdtError::OutOfBounds);
    }
    let aligned = structure.start.is_multiple_of(4) && structure.len().is_multiple_of(4);
    if !aligned || !rsv_start.is_multiple_of(8) {
        return Err(FdtError::BadLayout);
    }
    // The map has no size of its own: it runs to its terminator, which has to
    // come before whichever block follows it, or failing that the blob's end.
    let rsv_end = [structure.start, strings.start, total]
        .into_iter()
        .filter(|&end| end > rsv_start)
        .min()
        .unwrap_or(total);
    Ok(Layout { total, rsvmap: rsv_start..rsv_end, structure, strings })
}

fn block(offset: u32, size: u32, total: usize) -> Result<Range<usize>, FdtError> {
    let start = offset as usize;
    let end = start.checked_add(size as usize).ok_or(FdtError::OutOfBounds)?;
    if start < HEADER_SIZE || end > total {
        return Err(FdtError::OutOfBounds);
    }
    Ok(start..end)
}

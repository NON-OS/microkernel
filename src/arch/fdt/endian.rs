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

use super::error::FdtError;

#[inline]
pub fn be_u32(bytes: &[u8], offset: usize) -> Result<u32, FdtError> {
    let end = offset.checked_add(4).ok_or(FdtError::OutOfBounds)?;
    let slice = bytes.get(offset..end).ok_or(FdtError::OutOfBounds)?;
    Ok(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

#[inline]
pub fn be_u64(bytes: &[u8], offset: usize) -> Result<u64, FdtError> {
    let hi = be_u32(bytes, offset)? as u64;
    let lo = be_u32(bytes, offset.checked_add(4).ok_or(FdtError::OutOfBounds)?)? as u64;
    Ok((hi << 32) | lo)
}

// A value spread over `cells` big-endian u32 cells. Zero cells reads as zero,
// which is what a `#size-cells = <0>` bus means; past two cells the value
// does not fit a u64 and is refused rather than truncated.
#[inline]
pub fn be_cells(bytes: &[u8], offset: usize, cells: u32) -> Result<u64, FdtError> {
    match cells {
        0 => Ok(0),
        1 => be_u32(bytes, offset).map(|v| v as u64),
        2 => be_u64(bytes, offset),
        _ => Err(FdtError::BadCells),
    }
}

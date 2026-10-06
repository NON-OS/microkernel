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

use super::error::PeError;

pub(super) fn u16_at(f: &[u8], at: usize) -> Result<u16, PeError> {
    let b = f.get(at..at.checked_add(2).ok_or(PeError::Truncated)?).ok_or(PeError::Truncated)?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

pub(super) fn u32_at(f: &[u8], at: usize) -> Result<u32, PeError> {
    let b = f.get(at..at.checked_add(4).ok_or(PeError::Truncated)?).ok_or(PeError::Truncated)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// The certificate table's size, once its offset and size are inside the file.
/// No directory entry, or an empty one, is a table of size zero.
pub(super) fn cert_table(f: &[u8], entry: Option<usize>) -> Result<usize, PeError> {
    let Some(e) = entry else { return Ok(0) };
    let (at, size) = (u32_at(f, e)? as usize, u32_at(f, e + 4)? as usize);
    if size > 0 && at.checked_add(size).is_none_or(|end| end > f.len()) {
        return Err(PeError::OutOfFile);
    }
    Ok(size)
}

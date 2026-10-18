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

//! Little-endian fields of a body, `None` past its end.

pub fn le_u32(buf: &[u8], off: usize) -> Option<u32> {
    let bytes = buf.get(off..off.checked_add(4)?)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub fn le_u64(buf: &[u8], off: usize) -> Option<u64> {
    let lo = le_u32(buf, off)?;
    let hi = le_u32(buf, off.checked_add(4)?)?;
    Some((hi as u64) << 32 | lo as u64)
}

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

//! What a terminal request's argument says.

use crate::linux::abi::errno;

/// TCFLSH's selector, an int: TCIFLUSH and TCIOFLUSH drop typed input,
/// TCOFLUSH has nothing to drop, anything else is EINVAL.
pub fn flush_input(arg: u64) -> Result<bool, i64> {
    match arg as u32 {
        0 | 2 => Ok(true),
        1 => Ok(false),
        _ => Err(errno::EINVAL),
    }
}

/// TCXONC's action, an int: TCOOFF, TCOON, TCIOFF and TCION, each with
/// nothing to do; anything else is EINVAL.
pub fn flow_ok(arg: u64) -> Result<(), i64> {
    match arg as u32 {
        0..=3 => Ok(()),
        _ => Err(errno::EINVAL),
    }
}

/// `struct winsize`'s rows and columns, from the eight bytes at the
/// argument; the pixel sizes after them mean nothing here.
pub fn winsize(raw: &[u8]) -> Option<(u16, u16)> {
    let rows = u16::from_le_bytes(raw.get(0..2)?.try_into().ok()?);
    let cols = u16::from_le_bytes(raw.get(2..4)?.try_into().ok()?);
    Some((rows, cols))
}

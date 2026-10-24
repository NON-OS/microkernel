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

//! The framebuffer's byte order, set little endian where the adapter has
//! the register: every pixel is written as a little-endian word, whatever
//! the adapter's default.

use crate::constants::{QEXT_ENDIAN, QEXT_LITTLE_ENDIAN, QEXT_SIZE};
use crate::regs::Regs;

/// Whether the register was there to set, in the `reg_len` bytes mapped.
/// A mapping that ends before it, or a size under 8 or past the mapping, is
/// an adapter without it, which is little endian already.
pub fn little_endian(regs: Regs, reg_len: u64) -> bool {
    if reg_len < QEXT_ENDIAN as u64 + 4 {
        return false;
    }
    // SAFETY: the size register lies below QEXT_ENDIAN + 4, which the
    // mapping covers.
    let size = unsafe { regs.r32(QEXT_SIZE) } as u64;
    if !(8..=reg_len).contains(&size) {
        return false;
    }
    // SAFETY: as above, for the byte-order register.
    unsafe { regs.w32(QEXT_ENDIAN, QEXT_LITTLE_ENDIAN) };
    true
}

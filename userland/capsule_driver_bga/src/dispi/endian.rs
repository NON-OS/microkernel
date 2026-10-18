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
//! QEMU's extension register for it: every pixel is written as a
//! little-endian word, whatever byte order the host machine has.

use crate::constants::{QEXT_ENDIAN, QEXT_LITTLE_ENDIAN, QEXT_SIZE, REG_BAR_MIN};
use crate::regs::Regs;

/// Whether the register was there to set, in a register mapping `mapped`
/// bytes long. A mapping that ends before the extension block, or a block
/// that names a size under 8 bytes or past the mapping, is an adapter
/// without the register, which is little endian already.
pub fn little_endian(regs: Regs, mapped: u64) -> bool {
    if mapped < REG_BAR_MIN {
        return false;
    }
    // SAFETY: the mapping reaches REG_BAR_MIN, past the size register.
    let size = unsafe { regs.r32(QEXT_SIZE) } as u64;
    if !(8..=mapped).contains(&size) {
        return false;
    }
    // SAFETY: as above; the byte-order register ends at REG_BAR_MIN.
    unsafe { regs.w32(QEXT_ENDIAN, QEXT_LITTLE_ENDIAN) };
    true
}

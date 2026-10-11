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

//! QEMU's framebuffer byte order, set little endian where the adapter has
//! the register, as bochs_hw_init does: the pixels are written as
//! little-endian words whatever the host.

use crate::constants::{QEXT_ENDIAN, QEXT_LITTLE_ENDIAN, QEXT_SIZE};
use crate::hw::Hw;

/// Whether the register was there to set. A size under 8 (or past the BAR)
/// is an adapter without it, which is little endian already.
pub fn little_endian<H: Hw>(hw: &H, reg_bar: u64) -> bool {
    let size = hw.qext(QEXT_SIZE) as u64;
    if !(8..=reg_bar).contains(&size) {
        return false;
    }
    hw.set_qext(QEXT_ENDIAN, QEXT_LITTLE_ENDIAN);
    true
}

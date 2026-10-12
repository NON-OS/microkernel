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

//! A DISPI mode set: the adapter disabled, the mode registers written,
//! the virtual screen as wide as the mode and scrolled to its origin, then
//! enabled with the linear framebuffer decoding and video memory kept, so
//! the picture changes in one step. GETCAPS (ENABLE bit 1) is never
//! written: alone it clears ENABLED and LFB, and bochs-display lacks it.

use super::dispi_off::dispi_off;
use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH,
    DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET,
    DISPI_LFB_ENABLED, DISPI_NOCLEARMEM,
};
use crate::regs::Regs;

pub fn set_mode(regs: Regs, width: u16, height: u16) {
    let writes = [
        (DISPI_INDEX_ENABLE, 0),
        (DISPI_INDEX_XRES, width),
        (DISPI_INDEX_YRES, height),
        (DISPI_INDEX_BPP, DISPI_BPP_32),
        (DISPI_INDEX_VIRT_WIDTH, width),
        (DISPI_INDEX_X_OFFSET, 0),
        (DISPI_INDEX_Y_OFFSET, 0),
        (DISPI_INDEX_ENABLE, DISPI_ENABLED | DISPI_LFB_ENABLED | DISPI_NOCLEARMEM),
    ];
    for (index, value) in writes {
        // SAFETY: every index written is in the DISPI block the window maps.
        unsafe { regs.w16(dispi_off(index), value) };
    }
}

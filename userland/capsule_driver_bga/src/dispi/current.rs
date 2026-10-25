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

//! The mode DISPI is scanning out, read back from its registers: firmware's
//! (OVMF and SeaBIOS program DISPI for their own framebuffer) until this
//! driver's first MODE_SET, which section 12 says a driver keeps. Only a
//! mode scanned out from the start of video memory is one: an offset one
//! would show a picture away from where every copy writes.

use super::dispi_off::dispi_off;
use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH,
    DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET,
    DISPI_LFB_ENABLED,
};
use crate::regs::Regs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispiMode {
    pub width: u32,
    pub height: u32,
    /// Bytes per line in video memory: the virtual width, 4 bytes a pixel.
    pub stride: u32,
}

/// The linear 32-bit mode DISPI shows, or None when it shows none.
pub fn current(regs: Regs) -> Option<DispiMode> {
    // SAFETY: every index read is in the DISPI block the window maps.
    let r = |i| unsafe { regs.r16(dispi_off(i)) };
    let on = DISPI_ENABLED | DISPI_LFB_ENABLED;
    let at_start = r(DISPI_INDEX_X_OFFSET) == 0 && r(DISPI_INDEX_Y_OFFSET) == 0;
    if r(DISPI_INDEX_ENABLE) & on != on || r(DISPI_INDEX_BPP) != DISPI_BPP_32 || !at_start {
        return None;
    }
    let (width, height) = (r(DISPI_INDEX_XRES) as u32, r(DISPI_INDEX_YRES) as u32);
    let virt = r(DISPI_INDEX_VIRT_WIDTH) as u32;
    // Lines narrower than the mode would overlap one another in video
    // memory, which no copy draws at any stride: not a mode to keep.
    let drawn = width > 0 && height > 0 && virt >= width;
    drawn.then_some(DispiMode { width, height, stride: virt * 4 })
}

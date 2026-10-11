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

//! What the adapter says of itself before anything is written to it.

use super::choose::Seen;
use crate::constants::{
    DISPI_ID0, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_ID, DISPI_INDEX_VIDEO_MEMORY_64K,
    DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES,
    DISPI_INDEX_Y_OFFSET,
};
use crate::hw::Hw;

/// The DISPI id, when it is one: bochs_hw_init refuses any other.
pub fn id<H: Hw>(hw: &H) -> Option<u16> {
    let id = hw.dispi(DISPI_INDEX_ID);
    (id & 0xFFF0 == DISPI_ID0).then_some(id)
}

pub fn seen<H: Hw>(hw: &H) -> Seen {
    Seen {
        enable: hw.dispi(DISPI_INDEX_ENABLE),
        xres: hw.dispi(DISPI_INDEX_XRES),
        yres: hw.dispi(DISPI_INDEX_YRES),
        bpp: hw.dispi(DISPI_INDEX_BPP),
        virt_width: hw.dispi(DISPI_INDEX_VIRT_WIDTH),
        x_offset: hw.dispi(DISPI_INDEX_X_OFFSET),
        y_offset: hw.dispi(DISPI_INDEX_Y_OFFSET),
    }
}

/// The VRAM the mode may use: the framebuffer BAR, and no more than the
/// adapter reports in 64 KiB units when it reports any.
pub fn vram<H: Hw>(hw: &H, fb_bar: u64) -> u64 {
    match hw.dispi(DISPI_INDEX_VIDEO_MEMORY_64K) as u64 * 64 * 1024 {
        0 => fb_bar,
        reported => reported.min(fb_bar),
    }
}

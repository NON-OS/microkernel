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

//! The mode set, in bochs_hw_setmode's order: the screen unblanked, the
//! adapter disabled, every mode register written, then enabled with its
//! linear framebuffer.

use super::mode::Mode;
use crate::constants::{
    BYTES_PER_PIXEL, DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BANK, DISPI_INDEX_BPP,
    DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_HEIGHT, DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES,
    DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET, DISPI_LFB_ENABLED, VGA_ATT_PAS,
    VGA_ATT_W, VGA_IS1_RC,
};
use crate::hw::Hw;

/// `false` when the adapter did not take the mode: it reads back other
/// than written, as QEMU does for a mode past its VRAM.
pub fn set_mode<H: Hw>(hw: &H, m: Mode) -> bool {
    // Reading input status 1 resets the attribute flip-flop to its index.
    let _ = hw.vga(VGA_IS1_RC);
    hw.set_vga(VGA_ATT_W, VGA_ATT_PAS);
    let (w, h) = (m.width as u16, m.height as u16);
    hw.set_dispi(DISPI_INDEX_ENABLE, 0);
    hw.set_dispi(DISPI_INDEX_BPP, DISPI_BPP_32);
    hw.set_dispi(DISPI_INDEX_XRES, w);
    hw.set_dispi(DISPI_INDEX_YRES, h);
    hw.set_dispi(DISPI_INDEX_BANK, 0);
    hw.set_dispi(DISPI_INDEX_VIRT_WIDTH, (m.stride / BYTES_PER_PIXEL) as u16);
    hw.set_dispi(DISPI_INDEX_VIRT_HEIGHT, h);
    hw.set_dispi(DISPI_INDEX_X_OFFSET, 0);
    hw.set_dispi(DISPI_INDEX_Y_OFFSET, 0);
    hw.set_dispi(DISPI_INDEX_ENABLE, DISPI_ENABLED | DISPI_LFB_ENABLED);
    hw.dispi(DISPI_INDEX_XRES) == w
        && hw.dispi(DISPI_INDEX_YRES) == h
        && hw.dispi(DISPI_INDEX_BPP) == DISPI_BPP_32
}

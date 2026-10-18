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

use super::dispi_off::{dispi_off, vga_off};
use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BANK, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE,
    DISPI_INDEX_VIDEO_MEMORY_64K, DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET,
    DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET, DISPI_LFB_ENABLED, VGA_ATT_PAS, VGA_ATT_W, VGA_IS1_RC,
};
use crate::regs::Regs;

/// How far into the register mapping the driver reaches: through
/// VIDEO_MEMORY_64K, the highest DISPI index it reads.
pub fn regs_len() -> u64 {
    dispi_off(DISPI_INDEX_VIDEO_MEMORY_64K) as u64 + 2
}

/// Program the mode with the screen unblanked first and the adapter
/// disabled, the bank back at 0, ENABLE written last but for the line
/// layout: one mode-wide line after another from the start of video memory,
/// which an adapter enabled from disabled would reset to anyway.
pub fn set_mode(regs: Regs, width: u16, height: u16) {
    // SAFETY: every register written lies below `regs_len`, which the caller
    // has checked the register mapping covers: the VGA ports at 0x400 and up,
    // then the DISPI block at 0x500.
    unsafe {
        // Reading input status 1 points the attribute flip-flop at its index,
        // so the write after it sets the palette address source: the screen
        // is shown, not left blank by whatever programmed the VGA last.
        let _ = regs.r8(vga_off(VGA_IS1_RC));
        regs.w8(vga_off(VGA_ATT_W), VGA_ATT_PAS);
        regs.w16(dispi_off(DISPI_INDEX_ENABLE), 0);
        regs.w16(dispi_off(DISPI_INDEX_BANK), 0);
        regs.w16(dispi_off(DISPI_INDEX_XRES), width);
        regs.w16(dispi_off(DISPI_INDEX_YRES), height);
        regs.w16(dispi_off(DISPI_INDEX_BPP), DISPI_BPP_32);
        regs.w16(dispi_off(DISPI_INDEX_ENABLE), DISPI_ENABLED | DISPI_LFB_ENABLED);
        regs.w16(dispi_off(DISPI_INDEX_VIRT_WIDTH), width);
        regs.w16(dispi_off(DISPI_INDEX_X_OFFSET), 0);
        regs.w16(dispi_off(DISPI_INDEX_Y_OFFSET), 0);
    }
}

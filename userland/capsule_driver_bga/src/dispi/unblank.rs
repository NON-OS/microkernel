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

//! The screen shown: a mode set on an adapter whose attribute controller
//! still has its palette address source clear scans out nothing, so the
//! driver sets it before programming its own mode. A kept firmware mode is
//! already showing and is left alone.

use crate::constants::{VGA_ATT_PAS, VGA_ATT_W, VGA_IS1_RC, VGA_MMIO_BASE, VGA_PORT_BASE};
use crate::regs::Regs;

fn vga_off(port: u16) -> u32 {
    VGA_MMIO_BASE + (port - VGA_PORT_BASE) as u32
}

pub fn unblank(regs: Regs) {
    // SAFETY: both VGA registers lie below the DISPI block, so below
    // `regs_len`, which the caller has checked the register mapping covers.
    unsafe {
        // Reading input status 1 sets the attribute flip-flop to its index.
        let _ = regs.r8(vga_off(VGA_IS1_RC));
        regs.w8(vga_off(VGA_ATT_W), VGA_ATT_PAS);
    }
}

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

//! The part's internal memory through the window registers, as tg3_read_mem
//! and tg3_write_mem do when SRAM_USE_CONFIG is clear (it is, on the 57765
//! class). The window base is left at zero after each access, as Linux
//! leaves it.

use crate::constants::pcicfg::{TG3PCI_MEM_WIN_BASE_ADDR, TG3PCI_MEM_WIN_DATA};
use crate::regs::Regs;

/*
 * SAFETY for both functions: `regs` carries the broker MmioMap base for BAR0,
 * and both window registers are 32-bit registers inside its first page.
 */
pub fn read(regs: &Regs, offset: u32) -> u32 {
    unsafe {
        regs.w32f(TG3PCI_MEM_WIN_BASE_ADDR, offset);
        let v = regs.r32(TG3PCI_MEM_WIN_DATA);
        regs.w32f(TG3PCI_MEM_WIN_BASE_ADDR, 0);
        v
    }
}

pub fn write(regs: &Regs, offset: u32, value: u32) {
    unsafe {
        regs.w32f(TG3PCI_MEM_WIN_BASE_ADDR, offset);
        regs.w32f(TG3PCI_MEM_WIN_DATA, value);
        regs.w32f(TG3PCI_MEM_WIN_BASE_ADDR, 0);
    }
}

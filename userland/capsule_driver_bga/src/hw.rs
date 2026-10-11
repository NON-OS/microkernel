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

//! What the mode set needs of the adapter, so the proofs can stand in for it
//! and see the order of every write.

use crate::constants::{VGA_MMIO_BASE, VGA_PORT_BASE};
use crate::dispi::dispi_off;
use crate::regs::Regs;

pub trait Hw {
    fn dispi(&self, index: u32) -> u16;
    fn set_dispi(&self, index: u32, value: u16);
    fn vga(&self, port: u16) -> u8;
    fn set_vga(&self, port: u16, value: u8);
    fn qext(&self, off: u32) -> u32;
    fn set_qext(&self, off: u32, value: u32);
}

fn vga_off(port: u16) -> u32 {
    VGA_MMIO_BASE + (port - VGA_PORT_BASE) as u32
}

impl Hw for Regs {
    fn dispi(&self, index: u32) -> u16 {
        self.r16(dispi_off(index))
    }

    fn set_dispi(&self, index: u32, value: u16) {
        self.w16(dispi_off(index), value)
    }

    fn vga(&self, port: u16) -> u8 {
        self.r8(vga_off(port))
    }

    fn set_vga(&self, port: u16, value: u8) {
        self.w8(vga_off(port), value)
    }

    fn qext(&self, off: u32) -> u32 {
        self.r32(off)
    }

    fn set_qext(&self, off: u32, value: u32) {
        self.w32(off, value)
    }
}

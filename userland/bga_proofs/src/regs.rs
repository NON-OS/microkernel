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

//! The driver's register access as it ships (`shipping`), with each 16-bit
//! write handed to the shim's adapter model once it is made
//! (`nonos_libc::wrote16`). The shipping stores are raw volatile writes no
//! model sees, so an adapter that answers a write, as one clamping a mode it
//! cannot show does, is called from here before the driver's next
//! instruction. The reads and the writes themselves are the shipping code.

#[path = "../../capsule_driver_bga/src/regs.rs"]
mod shipping;

#[derive(Clone, Copy)]
pub struct Regs {
    shipped: shipping::Regs,
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { shipped: shipping::Regs::new(base), base }
    }

    /// # Safety
    /// As for the shipping `r8`.
    pub unsafe fn r8(self, off: u32) -> u8 {
        self.shipped.r8(off)
    }

    /// # Safety
    /// As for the shipping `w8`: the VGA ports, which no model answers.
    pub unsafe fn w8(self, off: u32, value: u8) {
        self.shipped.w8(off, value);
    }

    /// # Safety
    /// As for the shipping `r16`.
    pub unsafe fn r16(self, off: u32) -> u16 {
        self.shipped.r16(off)
    }

    /// # Safety
    /// As for the shipping `w16`.
    pub unsafe fn w16(self, off: u32, value: u16) {
        self.shipped.w16(off, value);
        nonos_libc::wrote16(self.base + off as u64, value);
    }

    /// # Safety
    /// As for the shipping `r32`.
    pub unsafe fn r32(self, off: u32) -> u32 {
        self.shipped.r32(off)
    }

    /// # Safety
    /// As for the shipping `w32`: the extension block, which no model answers.
    pub unsafe fn w32(self, off: u32, value: u32) {
        self.shipped.w32(off, value);
    }
}

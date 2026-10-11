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

//! The register BAR as mapped: volatile access at an offset into it.

use core::ptr::{read_volatile, write_volatile};

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    /// # Safety
    ///
    /// `base` is a live mapping of the adapter's register BAR, at least
    /// `REG_BAR_MIN` bytes long, for as long as this value is used.
    pub const unsafe fn new(base: u64) -> Self {
        Self { base }
    }

    fn at<T>(self, off: u32) -> *mut T {
        (self.base + off as u64) as *mut T
    }

    pub fn r8(self, off: u32) -> u8 {
        // SAFETY: `new`'s contract; every offset used is below REG_BAR_MIN.
        unsafe { read_volatile(self.at(off)) }
    }

    pub fn w8(self, off: u32, value: u8) {
        // SAFETY: as `r8`.
        unsafe { write_volatile(self.at(off), value) }
    }

    pub fn r16(self, off: u32) -> u16 {
        // SAFETY: as `r8`; DISPI registers are 16-bit aligned.
        unsafe { read_volatile(self.at(off)) }
    }

    pub fn w16(self, off: u32, value: u16) {
        // SAFETY: as `r16`.
        unsafe { write_volatile(self.at(off), value) }
    }

    pub fn r32(self, off: u32) -> u32 {
        // SAFETY: as `r8`; the extension registers are 32-bit aligned.
        unsafe { read_volatile(self.at(off)) }
    }

    pub fn w32(self, off: u32, value: u32) {
        // SAFETY: as `r32`.
        unsafe { write_volatile(self.at(off), value) }
    }
}

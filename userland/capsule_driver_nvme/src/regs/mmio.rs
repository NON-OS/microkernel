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

use super::lo_hi::{join_lo_hi, split_lo_hi};

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// # Safety
    /// `off` and the 4 bytes from it lie inside the register window `base`
    /// maps; so for every accessor here.
    pub unsafe fn r32(self, off: u32) -> u32 {
        nonos_libc::mmio_read32((self.base + off as u64) as usize)
    }

    /// A 64-bit register read as its low dword and then its high one.
    ///
    /// # Safety
    /// As `r32`, for the 8 bytes from `off`.
    pub unsafe fn r64(self, off: u32) -> u64 {
        let lo = self.r32(off);
        let hi = self.r32(off + 4);
        join_lo_hi(lo, hi)
    }

    /// # Safety
    /// As `r32`.
    pub unsafe fn w32(self, off: u32, value: u32) {
        nonos_libc::mmio_write32((self.base + off as u64) as usize, value);
    }

    /// A 64-bit register written as its low dword and then its high one.
    ///
    /// # Safety
    /// As `r32`, for the 8 bytes from `off`.
    pub unsafe fn w64(self, off: u32, value: u64) {
        let (lo, hi) = split_lo_hi(value);
        self.w32(off, lo);
        self.w32(off + 4, hi);
    }
}

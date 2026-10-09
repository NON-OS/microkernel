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

//! 32-bit MMIO accessor over the broker-mapped BAR0 window. Every I225/I226
//! register is 32 bits wide; `volatile` keeps the compiler from merging or
//! reordering accesses the part cares about the order of. The base is the
//! user_va the broker handed back from `mk_mmio_map`.

use core::ptr;

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// # Safety
    /// `offset` must be a 4-byte-aligned register inside the mapped BAR0
    /// window. The broker's `MmioMap` grant guarantees the page is present,
    /// user-mapped, uncached and read+write; the offset bound is the caller's.
    pub unsafe fn r32(&self, offset: usize) -> u32 {
        ptr::read_volatile((self.base as usize + offset) as *const u32)
    }

    /// # Safety
    /// Same conditions as `r32`. The part may observe the store any time
    /// after it retires; a caller that needs ordering issues its own fence.
    pub unsafe fn w32(&self, offset: usize, value: u32) {
        ptr::write_volatile((self.base as usize + offset) as *mut u32, value);
    }
}

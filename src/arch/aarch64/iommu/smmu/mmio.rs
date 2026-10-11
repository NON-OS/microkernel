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

use core::ptr::{read_volatile, write_volatile};

/// The mapped register window. Copyable because it is only an address: the
/// state lock, not this, decides who programs the unit.
#[derive(Debug, Clone, Copy)]
pub(super) struct Regs {
    va: u64,
}

impl Regs {
    /// # Safety
    ///
    /// `va` maps both register pages as device memory for as long as any
    /// copy of the returned value is used.
    pub(super) unsafe fn new(va: u64) -> Self {
        Self { va }
    }

    pub(super) fn read32(&self, offset: u64) -> u32 {
        // SAFETY: `new`'s contract; every offset in `regs` is inside the window.
        unsafe { read_volatile((self.va + offset) as *const u32) }
    }

    pub(super) fn write32(&self, offset: u64, value: u32) {
        // SAFETY: as for `read32`.
        unsafe { write_volatile((self.va + offset) as *mut u32, value) }
    }

    pub(super) fn write64(&self, offset: u64, value: u64) {
        // SAFETY: as for `read32`; the 64-bit registers are 8-byte aligned.
        unsafe { write_volatile((self.va + offset) as *mut u64, value) }
    }
}

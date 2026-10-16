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
//! The SPI function's registers through the broker's mapping of BAR 0.

use crate::core::Regs32;

#[derive(Clone, Copy)]
pub struct Mmio {
    base: u64,
    len: u64,
}

impl Mmio {
    pub fn new(base: u64, len: u64) -> Self {
        Self { base, len }
    }
}

impl Regs32 for Mmio {
    fn read32(&self, off: u64) -> u32 {
        if off + 4 > self.len {
            return u32::MAX;
        }
        // SAFETY: the broker mapped `len` bytes at `base`; `off` is inside.
        unsafe { core::ptr::read_volatile((self.base + off) as *const u32) }
    }

    fn write32(&self, off: u64, v: u32) {
        if off + 4 > self.len {
            return;
        }
        // SAFETY: as for read32.
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u32, v) }
    }
}

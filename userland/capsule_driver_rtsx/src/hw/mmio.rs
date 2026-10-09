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

//! The host register block, mapped uncached by the broker.

#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    /// `base` is the user address of the mapped register BAR, at least
    /// 0x1C bytes long (the BIER register's end), checked at setup.
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    pub fn r32(&self, offset: u32) -> u32 {
        // SAFETY: offsets are the host register constants (0x00 to 0x18),
        // inside the BAR mapping setup checked; the mapping is uncached
        // device memory that lives as long as the driver.
        unsafe { core::ptr::read_volatile((self.base + offset as u64) as *const u32) }
    }

    pub fn w32(&self, offset: u32, value: u32) {
        // SAFETY: as r32.
        unsafe { core::ptr::write_volatile((self.base + offset as u64) as *mut u32, value) }
    }
}

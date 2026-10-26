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

//! The engine's `Mmio` over the capsule's uncached mapping of BAR0. Every
//! access is volatile and inside the mapping; one outside it reads 0 and
//! writes nothing, so a wrong offset cannot reach memory past the BAR.

use core::ptr::{read_volatile, write_volatile};

use nonos_intel_display::engine::access::Mmio;

pub struct Bar0 {
    pub base: u64,
    pub len: u64,
}

impl Bar0 {
    fn at(&self, offset: u32, width: u64) -> Option<u64> {
        let end = offset as u64 + width;
        (end <= self.len && (offset as u64).is_multiple_of(width))
            .then_some(self.base + offset as u64)
    }
}

impl Mmio for Bar0 {
    fn read(&mut self, offset: u32) -> u32 {
        // SAFETY: `at` keeps the access aligned and inside the mapping.
        self.at(offset, 4).map_or(0, |a| unsafe { read_volatile(a as *const u32) })
    }

    fn write(&mut self, offset: u32, value: u32) {
        if let Some(a) = self.at(offset, 4) {
            // SAFETY: as for `read`.
            unsafe { write_volatile(a as *mut u32, value) };
        }
    }

    fn write64(&mut self, offset: u32, value: u64) {
        if let Some(a) = self.at(offset, 8) {
            // SAFETY: as for `read`; a GGTT entry is one 64-bit store.
            unsafe { write_volatile(a as *mut u64, value) };
        }
    }

    fn read64(&mut self, offset: u32) -> u64 {
        // SAFETY: as for `read`.
        self.at(offset, 8).map_or(0, |a| unsafe { read_volatile(a as *const u64) })
    }
}

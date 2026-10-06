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

//! Direct CFGBAR access for the assignment walk, before the domain is
//! registered and reachable by segment.

use super::domain::{cfg_offset, ConfigPort};

pub(super) struct Cfgbar {
    pub va: u64,
    pub bus_start: u8,
    pub bus_count: u16,
}

impl ConfigPort for Cfgbar {
    fn read32(&mut self, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        match cfg_offset(self.bus_start, self.bus_count, bus, device, function, offset) {
            // SAFETY: eK@nonos.systems - the offset is bounded to the decoded
            // buses of the CFGBAR mapped below, uncached and never unmapped.
            Some(at) => unsafe { core::ptr::read_volatile((self.va + at) as *const u32) },
            None => 0xFFFF_FFFF,
        }
    }

    fn write32(&mut self, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        if let Some(at) = cfg_offset(self.bus_start, self.bus_count, bus, device, function, offset)
        {
            // SAFETY: eK@nonos.systems - as the read.
            unsafe { core::ptr::write_volatile((self.va + at) as *mut u32, value) };
        }
    }
}

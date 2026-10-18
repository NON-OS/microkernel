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

//! A mapped controller window as the engine's `Window`: dword accesses at
//! an offset, refused past the mapping's length (a read past it answers
//! all ones, a write past it is dropped), so a wrong offset can never
//! reach memory the map did not cover.

use nonos_gpio::window::Window;

pub struct Mmio {
    pub va: u64,
    pub len: u64,
}

impl Mmio {
    fn inside(&self, offset: u32) -> bool {
        u64::from(offset) + 4 <= self.len && offset.is_multiple_of(4)
    }
}

impl Window for Mmio {
    fn read(&mut self, offset: u32) -> u32 {
        if !self.inside(offset) {
            return u32::MAX;
        }
        // SAFETY: `va` is the start of an MMIO mapping of `len` bytes the
        // broker granted this capsule, and the access lies inside it,
        // dword aligned.
        unsafe { core::ptr::read_volatile((self.va + u64::from(offset)) as *const u32) }
    }

    fn write(&mut self, offset: u32, value: u32) {
        if self.inside(offset) {
            // SAFETY: as for `read`.
            unsafe { core::ptr::write_volatile((self.va + u64::from(offset)) as *mut u32, value) }
        }
    }
}

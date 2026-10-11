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

//! Sixteen-bit volatile access to the broker-mapped BAR1 window. A
//! doubleword store to RDP would switch the part to DWIO and move every
//! register, so this type offers words only.

use core::ptr;

#[derive(Clone, Copy)]
pub struct Mmio {
    base: u64,
}

impl Mmio {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// # Safety
    /// `offset` is a word register inside the window the broker mapped at
    /// `base`, which `setup::mmio` asks for whole.
    pub unsafe fn r16(&self, offset: usize) -> u16 {
        ptr::read_volatile((self.base as usize + offset) as *const u16)
    }

    /// # Safety
    /// As for `r16`. The part may act on the store as soon as it retires.
    pub unsafe fn w16(&self, offset: usize, value: u16) {
        ptr::write_volatile((self.base as usize + offset) as *mut u16, value);
    }
}

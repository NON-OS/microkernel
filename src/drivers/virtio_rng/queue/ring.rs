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

//! Volatile access to the ring at byte offsets: the device writes the used
//! ring under the CPU, so nothing here is read through a plain reference.

use core::ptr::{read_volatile, write_volatile};

use super::core::RngQueue;

impl RngQueue {
    pub(super) fn put16(&self, offset: usize, value: u16) {
        // SAFETY: `offset` lies inside the ring region `layout` sized.
        unsafe { write_volatile((self.ring.virt + offset as u64) as *mut u16, value) }
    }

    pub(super) fn put32(&self, offset: usize, value: u32) {
        // SAFETY: as for `put16`, four bytes wide and aligned by the layout.
        unsafe { write_volatile((self.ring.virt + offset as u64) as *mut u32, value) }
    }

    pub(super) fn put64(&self, offset: usize, value: u64) {
        // SAFETY: as for `put16`, eight bytes wide; descriptors are 16-aligned.
        unsafe { write_volatile((self.ring.virt + offset as u64) as *mut u64, value) }
    }

    pub(super) fn get16(&self, offset: usize) -> u16 {
        // SAFETY: as for `put16`.
        unsafe { read_volatile((self.ring.virt + offset as u64) as *const u16) }
    }

    pub(super) fn get32(&self, offset: usize) -> u32 {
        // SAFETY: as for `put32`.
        unsafe { read_volatile((self.ring.virt + offset as u64) as *const u32) }
    }
}

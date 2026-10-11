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

//! VMMDevMemory: its size and version, written once by the host, and the
//! byte it raises while events wait for the guest.

use core::ptr::read_volatile;

use crate::constants::req::MEMORY_HAVE_EVENTS;

#[derive(Clone, Copy)]
pub struct Memory {
    base: u64,
}

impl Memory {
    /// `base` maps at least the first page of BAR1, held for the driver's
    /// life.
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    fn word(&self, at: usize) -> u32 {
        // SAFETY: `at` is one of the page's first three words, inside the
        // mapping `new` was given.
        unsafe { read_volatile((self.base as usize + at) as *const u32) }
    }

    pub fn size(&self) -> u32 {
        self.word(0)
    }

    pub fn version(&self) -> u32 {
        self.word(4)
    }

    pub fn have_events(&self) -> bool {
        // SAFETY: as for `word`.
        unsafe { read_volatile((self.base as usize + MEMORY_HAVE_EVENTS) as *const u8) != 0 }
    }
}

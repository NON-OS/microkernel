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

//! The operational registers through the MMIO grant of BAR0: every access
//! a volatile 32-bit one. An offset outside the grant reads as all ones and
//! is not written, as an absent register would behave.

use core::ptr::{read_volatile, write_volatile};

use nonos_libc::MmioMapOut;

use crate::env::Regs;

pub struct Registers {
    va: *mut u8,
    len: u64,
}

impl Registers {
    pub fn new(map: &MmioMapOut) -> Self {
        Registers { va: map.user_va as *mut u8, len: map.length }
    }

    /// How many bytes of registers the grant covers.
    pub fn mapped(&self) -> u64 {
        self.len
    }

    fn at(&self, off: u32) -> Option<*mut u32> {
        let fits = off.is_multiple_of(4) && off as u64 + 4 <= self.len && !self.va.is_null();
        // SAFETY: `off + 4` is within the mapped grant.
        fits.then(|| unsafe { self.va.add(off as usize) } as *mut u32)
    }
}

impl Regs for Registers {
    fn r32(&mut self, off: u32) -> u32 {
        // SAFETY: `at` checked the word lies in the grant, aligned.
        self.at(off).map_or(u32::MAX, |p| unsafe { read_volatile(p) })
    }

    fn w32(&mut self, off: u32, v: u32) {
        if let Some(p) = self.at(off) {
            // SAFETY: as for `r32`.
            unsafe { write_volatile(p, v) }
        }
    }
}

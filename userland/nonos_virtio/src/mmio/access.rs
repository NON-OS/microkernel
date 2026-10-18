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

//! Reads and writes of one width each, bounds checked by `slot`: outside the
//! window a read answers all ones and a write is dropped.

use core::ptr::{read_volatile, write_volatile};

use super::order::{after_load, before_store};
use super::window::Mmio;

impl Mmio {
    fn read<T: Copy>(self, off: usize, outside: T) -> T {
        // SAFETY: `slot` returns only in-range, aligned addresses.
        let Some(value) = self.slot(off).map(|p| unsafe { read_volatile(p) }) else {
            return outside;
        };
        if self.registers {
            after_load();
        }
        value
    }

    fn write<T>(self, off: usize, value: T) {
        if let Some(p) = self.slot(off) {
            if self.registers {
                before_store();
            }
            // SAFETY: as in `read`.
            unsafe { write_volatile(p, value) }
        }
    }

    pub fn r8(self, off: usize) -> u8 {
        self.read(off, u8::MAX)
    }

    pub fn r16(self, off: usize) -> u16 {
        self.read(off, u16::MAX)
    }

    pub fn r32(self, off: usize) -> u32 {
        self.read(off, u32::MAX)
    }

    pub fn w8(self, off: usize, value: u8) {
        self.write(off, value)
    }

    pub fn w16(self, off: usize, value: u16) {
        self.write(off, value)
    }

    pub fn w32(self, off: usize, value: u32) {
        self.write(off, value)
    }
}

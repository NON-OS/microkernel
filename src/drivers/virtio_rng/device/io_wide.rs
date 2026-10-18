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

//! The 16- and 32-bit registers, by the same transport rules as `io`.

use super::core::VirtioRngDevice;
use super::io::at;
use super::types::AccessMode;
use crate::sys::io;
use core::ptr;

impl VirtioRngDevice {
    #[inline]
    pub(super) fn read16(&self, offset: u16) -> u16 {
        match &self.access {
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Io(iobase) => unsafe { io::inw(*iobase + offset) },
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::read_volatile(at(*base, offset) as *const u16)
            },
        }
    }

    #[inline]
    pub(super) fn write16(&self, offset: u16, val: u16) {
        match &self.access {
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Io(iobase) => unsafe { io::outw(*iobase + offset, val) },
            // SAFETY: as for `read8`, two bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::write_volatile(at(*base, offset) as *mut u16, val);
            },
        }
    }

    #[inline]
    pub(super) fn read32(&self, offset: u16) -> u32 {
        match &self.access {
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Io(iobase) => unsafe { io::inl(*iobase + offset) },
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::read_volatile(at(*base, offset) as *const u32)
            },
        }
    }

    #[inline]
    pub(super) fn write32(&self, offset: u16, val: u32) {
        match &self.access {
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Io(iobase) => unsafe { io::outl(*iobase + offset, val) },
            // SAFETY: as for `read8`, four bytes wide.
            AccessMode::Mmio(base) => unsafe {
                ptr::write_volatile(at(*base, offset) as *mut u32, val);
            },
        }
    }
}

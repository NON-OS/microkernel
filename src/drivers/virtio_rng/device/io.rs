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

//! Port access to the legacy registers. Only a device with an I/O BAR 0 is
//! driven here; a memory BAR was once used as a pointer, unmapped.

use super::core::VirtioRngDevice;
use crate::sys::io;

impl VirtioRngDevice {
    pub(super) fn read8(&self, offset: u16) -> u8 {
        // SAFETY: `offset` is a register inside the device's own I/O BAR.
        unsafe { io::inb(self.iobase + offset) }
    }

    pub(super) fn write8(&self, offset: u16, val: u8) {
        // SAFETY: as for `read8`.
        unsafe { io::outb(self.iobase + offset, val) }
    }

    pub(super) fn write16(&self, offset: u16, val: u16) {
        // SAFETY: as for `read8`, two bytes wide.
        unsafe { io::outw(self.iobase + offset, val) }
    }

    pub(super) fn write32(&self, offset: u16, val: u32) {
        // SAFETY: as for `read8`, four bytes wide.
        unsafe { io::outl(self.iobase + offset, val) }
    }
}

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

//! One port I/O grant. The kernel performs every in and out; a refused read
//! comes back as zero, which every wait below reads as "not yet", and a
//! refused write is dropped.

use nonos_libc::{mk_pio_read, mk_pio_write};

#[derive(Clone, Copy)]
pub struct Port {
    grant: u64,
}

impl Port {
    pub const fn new(grant: u64) -> Self {
        Self { grant }
    }

    fn read(&self, off: u16, width: u8) -> u32 {
        let mut v = 0u32;
        if mk_pio_read(self.grant, off, width, &mut v) < 0 {
            return 0;
        }
        v
    }

    pub fn read8(&self, off: u16) -> u8 {
        self.read(off, 1) as u8
    }

    pub fn read16(&self, off: u16) -> u16 {
        self.read(off, 2) as u16
    }

    pub fn read32(&self, off: u16) -> u32 {
        self.read(off, 4)
    }

    pub fn write8(&self, off: u16, v: u8) {
        let _ = mk_pio_write(self.grant, off, 1, v as u32);
    }

    pub fn write16(&self, off: u16, v: u16) {
        let _ = mk_pio_write(self.grant, off, 2, v as u32);
    }

    pub fn write32(&self, off: u16, v: u32) {
        let _ = mk_pio_write(self.grant, off, 4, v);
    }
}

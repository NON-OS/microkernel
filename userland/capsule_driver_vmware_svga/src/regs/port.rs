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

//! The index and value ports, through the broker's port I/O grant. The
//! kernel performs every in and out; a refused access reads as zero and a
//! refused write is dropped, which the register checks above catch.

use nonos_libc::{mk_pio_read, mk_pio_write};

#[derive(Clone, Copy)]
pub struct Port {
    grant: u64,
}

impl Port {
    pub const fn new(grant: u64) -> Self {
        Self { grant }
    }

    pub fn read32(&self, port: u16) -> u32 {
        let mut v = 0u32;
        if mk_pio_read(self.grant, port, 4, &mut v) < 0 {
            return 0;
        }
        v
    }

    pub fn write32(&self, port: u16, value: u32) {
        let _ = mk_pio_write(self.grant, port, 4, value);
    }
}

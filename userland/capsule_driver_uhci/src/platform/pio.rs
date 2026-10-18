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

//! The controller's registers through the PIO grant; the kernel runs every
//! `in` and `out`, bounded to BAR4. A refused access reads as all ones,
//! as an absent device does, so every wait on it ends at its bound.

use nonos_libc::{mk_pio_read, mk_pio_write};

use crate::env::Io;

pub struct Ports {
    grant: u64,
}

impl Ports {
    pub fn new(grant: u64) -> Self {
        Ports { grant }
    }

    fn read(&self, off: u16, width: u8) -> u32 {
        let mut v = 0u32;
        if mk_pio_read(self.grant, off, width, &mut v) < 0 {
            return u32::MAX;
        }
        v
    }
}

impl Io for Ports {
    fn r16(&mut self, off: u16) -> u16 {
        self.read(off, 2) as u16
    }

    fn w16(&mut self, off: u16, v: u16) {
        let _ = mk_pio_write(self.grant, off, 2, v as u32);
    }

    fn w32(&mut self, off: u16, v: u32) {
        let _ = mk_pio_write(self.grant, off, 4, v);
    }
}

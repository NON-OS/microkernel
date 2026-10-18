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

//! The host's ports through the PIO grant: the kernel runs every `in` and
//! `out`, bounded to the SMBus BAR.

use nonos_libc::{mk_idle_ms, mk_pio_read, mk_pio_write, Deadline};

use crate::errno::E_IO;
use crate::ports::Ports;

pub struct Host {
    grant: u64,
    deadline: Deadline,
}

impl Host {
    pub fn new(grant: u64) -> Self {
        Self { grant, deadline: Deadline::after_ms(0) }
    }
}

impl Ports for Host {
    fn r8(&mut self, off: u16) -> Result<u8, i32> {
        let mut v = 0u32;
        if mk_pio_read(self.grant, off, 1, &mut v) < 0 {
            return Err(E_IO);
        }
        Ok(v as u8)
    }

    fn w8(&mut self, off: u16, value: u8) -> Result<(), i32> {
        if mk_pio_write(self.grant, off, 1, value as u32) < 0 {
            return Err(E_IO);
        }
        Ok(())
    }

    fn arm(&mut self, ms: u64) {
        self.deadline = Deadline::after_ms(ms);
    }

    fn expired(&mut self) -> bool {
        self.deadline.expired()
    }

    fn pause(&mut self, ms: u64) {
        let _ = mk_idle_ms(ms);
    }
}

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

//! The sink as the driver's registers: each access sent to the side that
//! answers it (`sink_write.rs`, `sink_command.rs`, `sink_status.rs`).

use super::sink::Sink;
use crate::engine::access::Mmio;
use crate::engine::gmbus_regs::*;

impl Mmio for Sink<'_> {
    fn read(&mut self, off: u32) -> u32 {
        match off {
            GMBUS2 => self.status(),
            GMBUS3 => self.word(),
            _ => 0,
        }
    }

    fn write(&mut self, off: u32, v: u32) {
        match off {
            GMBUS0 => self.select(v),
            GMBUS1 => self.command(v),
            GMBUS3 => self.data(v),
            _ => {}
        }
    }

    fn write64(&mut self, _: u32, _: u64) {}

    fn read64(&mut self, _: u32) -> u64 {
        0
    }
}

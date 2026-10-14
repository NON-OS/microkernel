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
//! Another device the touchpad's driver has no business with: a thermal
//! sensor of the LM75 kind, which boards put at 0x48-0x4F. A write's first
//! byte points at a register and the next ones set it, so an HID register
//! pointer, 01 00, writes zero to its configuration register; a read
//! returns the temperature. Both are recorded, so a proof can say no
//! driver touched it.

use crate::bus::{Dir, Target};

pub struct Sensor {
    addr: u8,
    reads: u32,
    writes: Vec<u8>,
}

impl Sensor {
    pub fn new(addr: u8) -> Self {
        Self { addr, reads: 0, writes: Vec::new() }
    }

    /// Bytes read from it.
    pub fn reads(&self) -> u32 {
        self.reads
    }

    /// Every byte written to it, register pointers included.
    pub fn writes(&self) -> &[u8] {
        &self.writes
    }
}

impl Target for Sensor {
    fn address(&self) -> u8 {
        self.addr
    }

    fn start(&mut self, _dir: Dir, _repeated: bool) {}

    fn write(&mut self, byte: u8) -> bool {
        self.writes.push(byte);
        true
    }

    /// The temperature register's high byte: 25 degrees Celsius.
    fn read(&mut self, _last: bool) -> u8 {
        self.reads += 1;
        25
    }

    fn stop(&mut self) {}
}

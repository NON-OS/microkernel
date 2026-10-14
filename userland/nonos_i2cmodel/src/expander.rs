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

//! A device the touchpad's driver has no business with, on the same bus: a
//! GPIO expander of the PCA9555 kind, which laptop boards put beside the
//! touchpad for lid, backlight and power lines. A read with no register
//! named returns its input port, which clears its interrupt latch; a write
//! names a register and then sets it, which can turn an input into an
//! output. Both are recorded, so a proof can say no driver touched it.

use crate::bus::{Dir, Target};

pub struct Expander {
    addr: u8,
    reads: u32,
    writes: Vec<u8>,
}

impl Expander {
    pub fn new(addr: u8) -> Self {
        Self { addr, reads: 0, writes: Vec::new() }
    }

    /// Bytes read from it, each of which cleared its interrupt latch.
    pub fn reads(&self) -> u32 {
        self.reads
    }

    /// Every byte written to it, register pointers included.
    pub fn writes(&self) -> &[u8] {
        &self.writes
    }
}

impl Target for Expander {
    fn address(&self) -> u8 {
        self.addr
    }

    fn start(&mut self, _dir: Dir, _repeated: bool) {}

    fn write(&mut self, byte: u8) -> bool {
        self.writes.push(byte);
        true
    }

    /// An input port with every line high.
    fn read(&mut self, _last: bool) -> u8 {
        self.reads += 1;
        0xFF
    }

    fn stop(&mut self) {}
}

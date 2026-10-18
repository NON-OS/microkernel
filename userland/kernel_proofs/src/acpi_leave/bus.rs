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

//! A register bus over bytes: every port and memory byte firmware's blocks
//! hold, and every byte written, so a test sees what changed and where.

use super::gas::RegisterBus;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Bus {
    /// (memory, address) to its byte; a byte never set reads as 0xFF.
    pub bytes: BTreeMap<(bool, u64), u8>,
    pub written: Vec<(bool, u64)>,
}

impl Bus {
    pub fn byte(&self, mem: bool, at: u64) -> u8 {
        self.bytes.get(&(mem, at)).copied().unwrap_or(0xFF)
    }

    fn read(&self, mem: bool, at: u64, bits: u8) -> u64 {
        (0..u64::from(bits / 8)).fold(0, |v, i| v | u64::from(self.byte(mem, at + i)) << (8 * i))
    }

    fn write(&mut self, mem: bool, at: u64, bits: u8, v: u64) {
        for i in 0..u64::from(bits / 8) {
            self.bytes.insert((mem, at + i), (v >> (8 * i)) as u8);
            self.written.push((mem, at + i));
        }
    }
}

impl RegisterBus for Bus {
    fn read_io(&mut self, port: u16, bits: u8) -> u32 {
        self.read(false, u64::from(port), bits) as u32
    }

    fn write_io(&mut self, port: u16, bits: u8, value: u32) {
        self.write(false, u64::from(port), bits, u64::from(value));
    }

    fn read_mem(&mut self, phys: u64, bits: u8) -> Option<u64> {
        Some(self.read(true, phys, bits))
    }

    fn write_mem(&mut self, phys: u64, bits: u8, value: u64) -> bool {
        self.write(true, phys, bits, value);
        true
    }
}

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

//! A register by number: the index port, then the value port. The capsule
//! is one thread, which is what keeps the two together.

use super::port::Port;
use crate::constants::regs::{INDEX_PORT, VALUE_PORT};

#[derive(Clone, Copy)]
pub struct Regs {
    port: Port,
}

impl Regs {
    pub const fn new(port: Port) -> Self {
        Self { port }
    }

    pub fn read(&self, reg: u32) -> u32 {
        self.port.write32(INDEX_PORT, reg);
        self.port.read32(VALUE_PORT)
    }

    pub fn write(&self, reg: u32, value: u32) {
        self.port.write32(INDEX_PORT, reg);
        self.port.write32(VALUE_PORT, value);
    }
}

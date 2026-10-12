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

//! An AMD bank as the registers behave: per-pin dwords whose two status
//! bits clear when written as one and are otherwise kept, a pin's
//! interrupt raised only while it is enabled, and every end of interrupt
//! counted.

use nonos_gpio::amd::AmdBank;
use nonos_gpio::amd_bits::*;
use nonos_gpio::window::Window;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Bank {
    pub regs: BTreeMap<u32, u32>,
    pub eois: u32,
}

impl Window for Bank {
    fn read(&mut self, off: u32) -> u32 {
        *self.regs.get(&off).unwrap_or(&0)
    }
    fn write(&mut self, off: u32, v: u32) {
        if off == MASTER {
            self.eois += u32::from(v & MASTER_EOI != 0);
            return;
        }
        let w1c = INTERRUPT_STS | WAKE_STS;
        let old = self.read(off);
        self.regs.insert(off, (v & !w1c) | (old & w1c & !v));
    }
}

pub fn bank() -> AmdBank<Bank> {
    AmdBank { window: Bank::default(), len: 0x400 }
}

impl Bank {
    /// The pin's line moves as its trigger wants.
    pub fn raise(&mut self, pin: u16) {
        let at = u32::from(pin) * 4;
        let v = self.read(at);
        if v & INTERRUPT_ENABLE != 0 {
            self.regs.insert(at, v | INTERRUPT_STS);
        }
    }
    pub fn pin(&mut self, pin: u16) -> u32 {
        self.read(u32::from(pin) * 4)
    }
}

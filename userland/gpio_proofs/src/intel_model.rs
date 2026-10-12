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

//! An Intel community as the registers behave: PADBAR pointing at the pad
//! array, a 16-byte pad stride, group status dwords that clear the bits
//! written as one, and a group bit raised only while its enable is set.

use nonos_gpio::intel::{Community, IntelGpio, IntelPad};
use nonos_gpio::window::Window;
use std::collections::BTreeMap;

pub const PADS: u32 = 0x600;
pub const GPI_IS: u32 = 0x100;
pub const GPI_IE: u32 = 0x120;

#[derive(Default)]
pub struct Regs(pub BTreeMap<u32, u32>);

impl Window for Regs {
    fn read(&mut self, off: u32) -> u32 {
        if off == nonos_gpio::intel_bits::PADBAR {
            return PADS;
        }
        *self.0.get(&off).unwrap_or(&0)
    }
    fn write(&mut self, off: u32, v: u32) {
        let w1c = (GPI_IS..GPI_IS + 0x20).contains(&off);
        let old = self.read(off);
        self.0.insert(off, if w1c { old & !v } else { v });
    }
}

/// Firmware pins 0..=63 in community 0 (two groups of 32), 100..=131 in
/// community 1's group 0; anything else is not placed.
pub fn place(pin: u16) -> Option<IntelPad> {
    match pin {
        0..=63 => Some(IntelPad {
            community: 0,
            pad: pin,
            group: (pin / 32) as u8,
            bit: (pin % 32) as u8,
        }),
        100..=131 => {
            Some(IntelPad { community: 1, pad: pin - 100, group: 0, bit: (pin - 100) as u8 })
        }
        _ => None,
    }
}

pub fn gpio() -> IntelGpio<Regs> {
    let c = || Community { window: Regs::default(), stride: 16, gpi_is: GPI_IS, gpi_ie: GPI_IE };
    IntelGpio { communities: vec![c(), c()], place }
}

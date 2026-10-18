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

//! SERIAL's flag bits, and the bus and interrupt fields `wire` writes.

use super::state::Peripheral;
use nonos_aml::serial::Bus;

pub const HID_I2C: u8 = 1 << 0;
pub const TEN_BIT: u8 = 1 << 1;
pub const DESC_FROM_DSM: u8 = 1 << 2;
pub const GPIO_INT: u8 = 1 << 3;
pub const APIC_INT: u8 = 1 << 4;
pub const EDGE: u8 = 1 << 5;
pub const ACTIVE_LOW: u8 = 1 << 6;
pub const WAKE: u8 = 1 << 7;

pub(super) fn bus(p: &Peripheral) -> (u8, u8, u16, u32) {
    match p.dev.connections[0].bus {
        Bus::I2c { address, speed_hz, ten_bit } => {
            (1, if ten_bit { TEN_BIT } else { 0 }, address, speed_hz)
        }
        Bus::Spi { speed_hz, chip_select, .. } => (2, 0, chip_select, speed_hz),
        Bus::Uart { baud, .. } => (3, 0, 0, baud),
    }
}

/// The interrupt's flags and number: a GpioInt first, else an APIC one.
pub(super) fn interrupt(p: &Peripheral) -> (u8, u32) {
    let bits = |edge: bool, low: bool, wake: bool| {
        [(edge, EDGE), (low, ACTIVE_LOW), (wake, WAKE)]
            .iter()
            .fold(0, |f, (on, b)| if *on { f | b } else { f })
    };
    if let Some(g) = p.dev.gpios.iter().find(|g| g.interrupt) {
        let pin = g.pins.first().copied().unwrap_or(0);
        return (GPIO_INT | bits(g.edge, g.polarity == 1, g.wake), u32::from(pin));
    }
    match p.dev.interrupts.first() {
        Some(i) => (APIC_INT | bits(i.edge, i.active_low, i.wake), i.gsiv),
        None => (0, 0),
    }
}

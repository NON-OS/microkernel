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

//! One line per serial-bus device: what it is, where it sits on its bus,
//! and how it interrupts.

use super::state::Peripheral;
use crate::boot::path;
use crate::sys::log::Line;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::serial::Bus;

fn on(l: Line, aml: &Aml, id: Option<Id>, text: &[u8]) -> Line {
    let l = l.str(b" on ");
    match id {
        Some(id) => l.str(path(aml, id).as_bytes()),
        None => l.str(text).str(b" (not found)"),
    }
}

pub fn say(aml: &Aml, p: &Peripheral) {
    let c = &p.dev.connections[0];
    let l = Line::new(b"serial ").str(path(aml, p.dev.id).as_bytes()).str(b" ").str(&p.hid);
    let l = if p.hid_over_i2c { l.str(b" HID-I2C") } else { l };
    let khz = |l: Line, hz: u32| l.str(b" ").dec(u64::from(hz / 1000)).str(b" kHz");
    let l = match &c.bus {
        Bus::I2c { address, speed_hz, .. } => {
            khz(l.str(b": i2c ").hex(u64::from(*address)), *speed_hz)
        }
        Bus::Spi { speed_hz, chip_select, .. } => {
            khz(l.str(b": spi cs ").dec(u64::from(*chip_select)), *speed_hz)
        }
        Bus::Uart { baud, .. } => l.str(b": uart ").dec(u64::from(*baud)).str(b" baud"),
    };
    let l = on(l, aml, p.controller, &c.source);
    let l = match p.dev.hid_register {
        Some(r) => l.str(b", descriptor ").hex(u64::from(r)),
        None if p.hid_over_i2c => l.str(b", no descriptor register"),
        None => l,
    };
    let l = match (p.dev.gpios.iter().find(|g| g.interrupt), p.dev.interrupts.first()) {
        (Some(g), _) => {
            let pin = g.pins.first().copied().unwrap_or(0);
            on(l.str(b", GpioInt ").hex(u64::from(pin)), aml, p.gpio_controller, &g.source)
        }
        (None, Some(i)) => l.str(b", interrupt ").hex(u64::from(i.gsiv)),
        (None, None) => l.str(b", no interrupt"),
    };
    l.end();
}

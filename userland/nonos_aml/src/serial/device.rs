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

//! One device on a serial bus, read whole: _STA first, then _CRS (firmware
//! often builds _CRS from what _STA decided), its connections, GPIO pins
//! and interrupts, its _DSD properties, and for HID over I2C the
//! descriptor register _DSM gives.

use super::bus::Connection;
use super::bus_parse::serial_buses;
use crate::props::{properties, Property};
use super::gpio::{gpios, Gpio};
use super::hid::descriptor_register;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, interrupt_entries, sta, Eval, Interrupt, PRESENT};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SerialDevice {
    pub id: Id,
    pub connections: Vec<Connection>,
    pub gpios: Vec<Gpio>,
    pub interrupts: Vec<Interrupt>,
    /// HID over I2C only: _DSM function 1's answer.
    pub hid_register: Option<u16>,
    pub properties: Vec<Property>,
}

/// None for a device that is absent, whose _CRS fails or is no buffer, or
/// that names no serial bus connection.
pub fn serial_device(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    hid_over_i2c: bool,
) -> Option<SerialDevice> {
    if !sta(aml, eval, dev).is_ok_and(|s| s & PRESENT != 0) {
        return None;
    }
    let Some(Ok(Object::Buffer(crs))) = child(aml, eval, dev, *b"_CRS", Vec::new()) else {
        return None;
    };
    let t = crs.borrow().clone();
    let connections = serial_buses(&t);
    if connections.is_empty() {
        return None;
    }
    Some(SerialDevice {
        id: dev,
        connections,
        gpios: gpios(&t),
        interrupts: interrupt_entries(&t),
        hid_register: if hid_over_i2c { descriptor_register(aml, eval, dev) } else { None },
        properties: properties(aml, eval, dev),
    })
}

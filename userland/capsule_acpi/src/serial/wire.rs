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

//! SERIAL (9, index u32): one serial-bus device, for the I2C, SPI and
//! UART controller capsules, from the evaluated tables. Little-endian:
//!
//! - count u32 (devices in all), kind u8 (1 I2C, 2 SPI, 3 UART), flags u8
//!   (`HID_I2C`, `TEN_BIT`, `DESC_FROM_DSM`, `GPIO_INT`, `APIC_INT`,
//!   `EDGE`, `ACTIVE_LOW`, `WAKE`);
//! - address u16 (I2C address, SPI chip select, 0 for UART), speed u32
//!   (Hz, or baud), descriptor register u16 (0 when none);
//! - interrupt u32: the GpioInt's first pin, else the first GSI;
//! - a slot once the bus controller's _ADR, now always all ones (a capsule
//!   is never given a PCI function), the bus controller's _UID and the
//!   GPIO controller's _UID u64, all ones when unknown;
//! - then four strings, a length byte and bytes each: the device's path,
//!   its _HID, the bus controller's path, the GPIO controller's path;
//! - then the bus controller's key u64 (`bus_key`: 1 << 63 | its broker
//!   device id), all ones when no device in the table is that controller;
//! - then 21 bytes of SPI and HID-over-SPI facts (`wire_spi`).

use super::bus_key::{key, Controller};
use super::known::known;
use super::wire_fields::{bus, interrupt, DESC_FROM_DSM, HID_I2C};
use super::wire_spi::tail;
use crate::boot::{path, Acpi};
use alloc::vec::Vec;

pub fn serial(acpi: &Acpi, index: usize, spi_hid_reader: bool) -> Option<Vec<u8>> {
    let p = acpi.serial.list.get(index)?;
    let (kind, mut flags, address, speed) = bus(p);
    let (irq_flags, irq) = interrupt(p);
    flags |= irq_flags | if p.hid_over_i2c { HID_I2C } else { 0 };
    flags |= if p.dev.hid_register.is_some() { DESC_FROM_DSM } else { 0 };
    let info = |id| acpi.devices.iter().find(|d| Some(d.id) == id);
    let (bus_ctl, gpio_ctl) = (info(p.controller), info(p.gpio_controller));
    let mut v = (acpi.serial.list.len() as u32).to_le_bytes().to_vec();
    v.extend_from_slice(&[kind, flags]);
    v.extend_from_slice(&address.to_le_bytes());
    v.extend_from_slice(&speed.to_le_bytes());
    v.extend_from_slice(&p.dev.hid_register.unwrap_or(0).to_le_bytes());
    v.extend_from_slice(&irq.to_le_bytes());
    for n in [None, bus_ctl.and_then(|d| d.uid), gpio_ctl.and_then(|d| d.uid)] {
        v.extend_from_slice(&n.unwrap_or(u64::MAX).to_le_bytes());
    }
    let paths =
        [Some(p.dev.id), p.controller, p.gpio_controller].map(|i| i.map(|i| path(&acpi.aml, i)));
    let [dev, ctl, gpio] = paths.map(|s| s.unwrap_or_default().into_bytes());
    for s in [&dev, &p.hid, &ctl, &gpio] {
        let s = &s[..s.len().min(100)];
        v.push(s.len() as u8);
        v.extend_from_slice(s);
    }
    let by = p.controller_bdf.map_or(Controller::Path(&ctl), Controller::Pci);
    v.extend_from_slice(&key(by, &known()).unwrap_or(u64::MAX).to_le_bytes());
    v.extend_from_slice(&tail(p, spi_hid_reader));
    Some(v)
}

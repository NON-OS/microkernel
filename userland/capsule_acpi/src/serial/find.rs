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

//! Every present device whose _CRS names a serial bus, read once the
//! platform is up (after _INI), with the controllers found by path.

use super::say::say;
use super::state::{Peripheral, Serial};
use crate::boot::evaluator;
use crate::dpower::bridge_bus;
use crate::host::CapsuleHost;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::platform::{device_pci_path, function_of, resolve_text};
use nonos_aml::platform::{DeviceInfo, HardwareId, PRESENT};
use nonos_aml::serial::{is_hid_over_i2c, serial_device};

pub fn find(devices: &[DeviceInfo], aml: &mut Aml, host: &mut CapsuleHost) -> Serial {
    let mut serial = Serial::default();
    for d in devices.iter().filter(|d| d.sta & PRESENT != 0) {
        let ids: Vec<HardwareId> = d.hid.iter().chain(&d.cids).cloned().collect();
        let hid_over_i2c = is_hid_over_i2c(&ids);
        let Some(dev) = serial_device(aml, &mut evaluator(host), d.id, hid_over_i2c) else {
            continue;
        };
        let controller = resolve_text(&aml.ns, d.id, &dev.connections[0].source);
        let controller_bdf = controller.and_then(|c| {
            let path = device_pci_path(aml, &mut evaluator(host), c)?;
            function_of(&path, &mut bridge_bus)
        });
        let pin = dev.gpios.iter().find(|g| g.interrupt).or(dev.gpios.first());
        let gpio_controller = pin.and_then(|g| resolve_text(&aml.ns, d.id, &g.source));
        let hid = d.hid.as_ref().map_or(Vec::new(), |h| h.bytes().to_vec());
        let p = Peripheral { dev, hid, hid_over_i2c, controller, controller_bdf, gpio_controller };
        serial.list.push(p);
    }
    serial.list.iter().for_each(|p| say(aml, p));
    serial
}

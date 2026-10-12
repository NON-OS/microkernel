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

//! At late boot, after _PS0 woke what it would: each device with an _ADR
//! and ports below it is offered to the kernel as a USB host (the kernel
//! answers ENOTSUP for a function that is not one), and each whose _DSD
//! says ExternalFacingPort is declared, for the kernel to flag what lies
//! behind it.

use super::external::declare;
use super::facts::fact;
use super::wire::{encode, MAX_PORTS};
use crate::boot::{evaluator, path, Acpi};
use crate::host::pci_function;
use crate::sys::log::Line;
use crate::sys::ports::{usb_ports, Bdf};
use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::platform::device_pci_path;
use nonos_aml::ports::usb_ports as ports_of;

/// What the kernel answers for a function that is no USB host.
const ENOTSUP: i64 = -95;

pub(super) fn function(acpi: &mut Acpi, dev: Id) -> Option<Bdf> {
    let path = device_pci_path(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev)?;
    pci_function(&path)
}

fn host(acpi: &mut Acpi, dev: Id) {
    let ports = ports_of(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev);
    if ports.is_empty() {
        return;
    }
    let Some(f) = function(acpi, dev) else { return };
    let line = Line::new(b"usb ports ").str(path(&acpi.aml, dev).as_bytes());
    let facts: Vec<_> = ports.iter().take(MAX_PORTS).map(fact).collect();
    let Some(wire) = encode(&facts) else { return line.str(b": facts unencodable").end() };
    let line = line.str(b": ").dec(facts.len() as u64).str(b" ports");
    match usb_ports(f, &wire) {
        Ok(id) => line.str(b", device ").dec(id).end(),
        // a port's own _ADR under a non-host device: nothing to say
        Err(ENOTSUP) => {}
        Err(e) => line.str(b", refused ").dec(e.unsigned_abs()).end(),
    }
}

pub fn publish(acpi: &mut Acpi) {
    let devs: Vec<Id> = acpi.devices.iter().filter(|d| d.adr.is_some()).map(|d| d.id).collect();
    for dev in devs {
        host(acpi, dev);
        declare(acpi, dev);
    }
}

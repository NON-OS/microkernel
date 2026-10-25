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

//! ACPI platform records among the devices a check or eject covers: a
//! controller firmware describes by `_HID` (an I2C or SD host, a GPIO
//! bank) that sits behind a dock and leaves with it. The kernel's record
//! is found by the device's namespace path, the only name both share.

use crate::boot::{path, Acpi};
use crate::platform::publish_one;
use crate::serial::bus_key::Known;
use crate::serial::known::known;
use crate::sys::log::Line;
use crate::sys::pci_depart::{depart_record, Why};
use alloc::vec::Vec;
use nonos_aml::namespace::Id;

/// The platform record (no PCI function) the kernel keeps for `id`.
pub fn record_of(acpi: &Acpi, table: &[Known], id: Id) -> Option<u64> {
    let p = path(&acpi.aml, id).into_bytes();
    table.iter().find(|k| k.bdf.is_none() && !p.is_empty() && k.path == p).map(|k| k.device_id)
}

/// Each of `ids` the kernel keeps a platform record for, with its id.
pub fn records(acpi: &Acpi, ids: &[Id]) -> Vec<(Id, u64)> {
    let table = known();
    ids.iter().filter_map(|i| Some((*i, record_of(acpi, &table, *i)?))).collect()
}

/// A device found gone: its record, if it has one, leaves the table; the
/// record's device id when it did.
pub fn depart_gone(acpi: &Acpi, id: Id) -> Option<u64> {
    let r = record_of(acpi, &known(), id)?;
    depart_record(r, Why::Gone).ok()
}

/// A device found there with no record yet: published, if it is a listed
/// controller.
pub fn arrive_record(acpi: &mut Acpi, id: Id) {
    if record_of(acpi, &known(), id).is_some() {
        return;
    }
    if let Some(r) = publish_one(acpi, id) {
        let l = Line::new(b"hotplug ").str(path(&acpi.aml, id).as_bytes());
        l.str(b": platform record ").hex(r).str(b" arrived").end();
    }
}

/// Put back the records an eject took out before firmware refused it.
pub fn put_back(acpi: &mut Acpi, ids: &[Id]) {
    for id in ids {
        let _ = publish_one(acpi, *id);
    }
}

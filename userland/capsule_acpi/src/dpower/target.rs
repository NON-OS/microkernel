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

//! The ACPI device a broker device id stands for: a PCI function through
//! MkAcpiFunctionOf, else an ACPI-bus device through MkAcpiDeviceName, by
//! its path or, for one the kernel's static scan found, by `_HID` and
//! `_UID`. Either way the kernel makes the same claim test for a holder.

use super::find::device_at;
use crate::boot::Acpi;
use crate::server::protocol::{ENOENT, EPERM};
use crate::sys::device_name::device_name;
use crate::sys::function_of::{function_of, Asker};
use nonos_aml::namespace::{Id, ROOT};
use nonos_aml::platform::resolve_text;

fn refused(e: i64) -> i32 {
    if e == EPERM as i64 {
        EPERM
    } else {
        ENOENT
    }
}

pub fn target(acpi: &mut Acpi, asker: Asker, device_id: u64) -> Result<Id, i32> {
    match function_of(asker, device_id) {
        Ok(bdf) => return device_at(acpi, bdf).ok_or(ENOENT),
        Err(e) if e == EPERM as i64 => return Err(EPERM),
        Err(_) => {}
    }
    let name = device_name(asker, device_id).map_err(refused)?;
    if !name.path.is_empty() {
        return resolve_text(&acpi.aml.ns, ROOT, &name.path).ok_or(ENOENT);
    }
    let hid = name.hid.split(|c| *c == 0).next().unwrap_or(&[]);
    let same = |d: &&nonos_aml::platform::DeviceInfo| {
        d.hid.as_ref().is_some_and(|h| h.bytes() == hid) && d.uid.unwrap_or(0) == name.uid
    };
    acpi.devices.iter().find(same).map(|d| d.id).ok_or(ENOENT)
}

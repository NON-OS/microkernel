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

//! One controller published after start: a dock that arrives brings its
//! controllers with it, and an eject that firmware refused puts back what
//! it had taken out. The same walk as at start (`platform_devices`), over
//! that device alone, with `_STA` read now rather than at start. `_DEP`
//! is not looked at again: what a hot-added controller depends on came
//! with it or was there already.

use super::wire::record;
use crate::boot::{evaluator, path, Acpi};
use crate::sys::platform::platform_device;
use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{sta, DeviceInfo};
use nonos_aml::platform_device::{known, platform_devices};

/// The new record's device id; None when the device is not a listed
/// controller, is not there, or the kernel refused it.
pub fn publish_one(acpi: &mut Acpi, id: Id) -> Option<u64> {
    let mut d: DeviceInfo = acpi.devices.iter().find(|d| d.id == id)?.clone();
    d.sta = sta(&mut acpi.aml, &mut evaluator(&mut acpi.host), id).ok()?;
    let (list, _) = known(super::publish::IDS);
    let f = platform_devices(&mut acpi.aml, &mut evaluator(&mut acpi.host), &[d], &list);
    let found = f.devices.first()?;
    let p = path(&acpi.aml, id);
    let r: Vec<u8> = record(found, p.as_bytes(), false)?;
    platform_device(&r).ok()
}

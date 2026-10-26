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

//! Each platform device's namespace path and identity, kept by the kernel
//! beside its record: the wire record a capsule sees has no room for a
//! path, and the census and the logs name the device by it.

use alloc::vec::Vec;
use spin::Mutex;

pub struct Named {
    pub device_id: u64,
    pub hid: [u8; 8],
    pub uid: u64,
    pub path: Vec<u8>,
}

static NAMES: Mutex<Vec<Named>> = Mutex::new(Vec::new());

pub fn keep(named: Named) {
    NAMES.lock().push(named);
}

/// A controller the kernel's static scan found keeps no path, only the
/// `_HID` its driver reads beside the record.
pub fn keep_unpathed(device_id: u64, hid: [u8; 8], uid: u64) {
    keep(Named { device_id, hid, uid, path: Vec::new() });
}

/// A platform device's `_HID`, `_UID` and path, for the drivers (the
/// `_HID` picks a GPIO pad layout) and for the ACPI capsule (the path is
/// how it finds the device's object). The path is empty for a controller
/// the kernel's own static scan found, which keeps no path.
pub fn named(device_id: u64) -> Option<([u8; 8], u64, Vec<u8>)> {
    let names = NAMES.lock();
    let n = names.iter().find(|n| n.device_id == device_id)?;
    Some((n.hid, n.uid, n.path.clone()))
}

/// A platform device that left (`depart_record`): its name goes with it.
pub fn forget(device_id: u64) {
    NAMES.lock().retain(|n| n.device_id != device_id);
}

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

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::super::error::{PciError, Result};

static DEVICE_BLOCKLIST: Mutex<Vec<(u16, u16)>> = Mutex::new(Vec::new());
static DEVICE_ALLOWLIST: Mutex<Option<Vec<(u16, u16)>>> = Mutex::new(None);

pub fn check_device_allowed(vendor_id: u16, device_id: u16) -> Result<()> {
    let blocklist = DEVICE_BLOCKLIST.lock();
    if blocklist.iter().any(|(v, d)| *v == vendor_id && *d == device_id) {
        return Err(PciError::DeviceBlocked { vendor: vendor_id, device: device_id });
    }
    drop(blocklist);

    let allowlist = DEVICE_ALLOWLIST.lock();
    if let Some(ref list) = *allowlist {
        if !list.is_empty() && !list.iter().any(|(v, d)| *v == vendor_id && *d == device_id) {
            return Err(PciError::DeviceNotAllowed { vendor: vendor_id, device: device_id });
        }
    }

    Ok(())
}

pub fn add_to_blocklist(vendor_id: u16, device_id: u16) {
    let mut blocklist = DEVICE_BLOCKLIST.lock();
    if !blocklist.iter().any(|(v, d)| *v == vendor_id && *d == device_id) {
        blocklist.push((vendor_id, device_id));
    }
}

pub fn remove_from_blocklist(vendor_id: u16, device_id: u16) {
    let mut blocklist = DEVICE_BLOCKLIST.lock();
    blocklist.retain(|(v, d)| !(*v == vendor_id && *d == device_id));
}

pub fn clear_blocklist() {
    DEVICE_BLOCKLIST.lock().clear();
}

pub fn set_allowlist(list: Option<Vec<(u16, u16)>>) {
    *DEVICE_ALLOWLIST.lock() = list;
}

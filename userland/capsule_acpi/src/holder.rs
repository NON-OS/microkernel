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

//! HOLDER (21, pid u32, key u64): whether that pid holds the device the
//! key names now, through the kernel's claim test, for the GPIO capsule to
//! give a pin's events only to its device's holder. The key is a PCI
//! function (segment << 16 | bus << 8 | devfn), or, with `PLATFORM` set,
//! the broker device id of an ACPI platform device such as an AMD I2C
//! controller, which has no PCI function. The body is 1 or 0. ENOENT for
//! no such device; EINVAL for a short request.

use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::acpi_holds::claimed_by;
use crate::sys::device_name::device_name;
use crate::sys::function_of::Asker;
use alloc::vec::Vec;

/// Bit 63 of a key: the rest is a broker device id, not a PCI function.
pub const PLATFORM: u64 = 1 << 63;
const EPERM: i64 = -1;

fn platform_held(pid: u32, device_id: u64) -> Result<bool, i64> {
    match device_name(Asker::Holder(pid), device_id) {
        Ok(_) => Ok(true),
        Err(EPERM) => Ok(false),
        Err(e) => Err(e),
    }
}

pub fn holder(req: &[u8]) -> (i32, Vec<u8>) {
    let (Some(pid), Some(key)) = (req.get(1..5), req.get(5..13)) else {
        return (EINVAL, Vec::new());
    };
    let pid = u32::from_le_bytes([pid[0], pid[1], pid[2], pid[3]]);
    let key = u64::from_le_bytes(key.try_into().unwrap_or([0; 8]));
    let held = if key & PLATFORM != 0 {
        platform_held(pid, key & !PLATFORM)
    } else {
        let devfn = key as u8;
        claimed_by(pid, ((key >> 16) as u16, (key >> 8) as u8, devfn >> 3, devfn & 7))
    };
    match held {
        Ok(held) => (0, alloc::vec![u8::from(held)]),
        Err(_) => (ENOENT, Vec::new()),
    }
}

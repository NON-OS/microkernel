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

//! HOLDER (21, pid u32, key u64): whether that pid holds the bus controller
//! the key names now, through the kernel's claim test, for the GPIO capsule
//! to give a pin's events only to its device's holder. The key is the one
//! form a capsule is given (`serial::bus_key`): 1 << 63 | broker device id,
//! for a PCI function and an ACPI platform device alike. A key without the
//! bit is a PCI function's own form, which no capsule is given, and is
//! refused. The body is 1 or 0. ENOENT for no such device; EINVAL for a
//! short request or a key not of that form.

use crate::serial::bus_key::device_of;
use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::device_name::device_name;
use crate::sys::function_of::{function_of, Asker};
use alloc::vec::Vec;

const EPERM: i64 = -1;

/// An ACPI-bus device's claim test first, then a PCI function's: the kernel
/// answers each for its own kind and ENOENT for the other.
fn held(pid: u32, device_id: u64) -> Result<bool, i64> {
    match device_name(Asker::Holder(pid), device_id) {
        Ok(_) => return Ok(true),
        Err(EPERM) => return Ok(false),
        Err(_) => {}
    }
    match function_of(Asker::Holder(pid), device_id) {
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
    let Some(device_id) = device_of(key) else { return (EINVAL, Vec::new()) };
    match held(pid, device_id) {
        Ok(held) => (0, alloc::vec![u8::from(held)]),
        Err(_) => (ENOENT, Vec::new()),
    }
}

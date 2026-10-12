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

//! HOLDER (21, pid u32, function u64 as segment << 16 | bus << 8 |
//! devfn): whether that pid holds that PCI function now, through the
//! kernel's claim test, for the GPIO capsule to give a pin's events only
//! to its device's holder. The body is 1 or 0. ENOENT for no such
//! function; EINVAL for a short request.

use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::acpi_holds::claimed_by;
use alloc::vec::Vec;

pub fn holder(req: &[u8]) -> (i32, Vec<u8>) {
    let (Some(pid), Some(key)) = (req.get(1..5), req.get(5..13)) else {
        return (EINVAL, Vec::new());
    };
    let pid = u32::from_le_bytes([pid[0], pid[1], pid[2], pid[3]]);
    let key = u64::from_le_bytes(key.try_into().unwrap_or([0; 8]));
    let devfn = key as u8;
    let function = ((key >> 16) as u16, (key >> 8) as u8, devfn >> 3, devfn & 7);
    match claimed_by(pid, function) {
        Ok(held) => (0, alloc::vec![u8::from(held)]),
        Err(_) => (ENOENT, Vec::new()),
    }
}

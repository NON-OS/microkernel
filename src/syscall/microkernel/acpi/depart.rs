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

//! `MkPciDepart(bus, devfn, mode) -> device id`: a function is out of the
//! device table when this answers. Mode 0: it has left (its slot's `_STA`
//! absent after a bus or device check), and everything its holder was given
//! is revoked first (`broker::acpi::depart`). Mode 1: the ACPI capsule is
//! about to run `_EJ0`, which waits for the holder: EBUSY while anyone
//! holds it, and nothing changes. Only the `acpi` service's owner may ask.
//! EPERM for anyone else; ENOENT for a function not in the table; EINVAL
//! for a field or mode out of range.

use crate::hardware::broker::acpi::depart::{depart, DepartError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

pub fn sys_pci_depart(bus: u64, devfn: u64, mode: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let (Ok(bus), Ok(devfn), true) = (u8::try_from(bus), u8::try_from(devfn), mode <= 1) else {
        return ERRNO_INVAL;
    };
    match depart(bus, devfn >> 3, devfn & 7, mode == 1) {
        Ok(d) => d.device_id as i64,
        Err(DepartError::Unknown) => ERRNO_NOENT,
        Err(DepartError::Held(_)) => ERRNO_BUSY,
    }
}

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

//! `MkPciDepart(a, b, mode) -> device id`: a device is out of the device
//! table when this answers. Modes 0 and 1 name a PCI function (a bus, b
//! devfn); modes 2 and 3 an ACPI platform record (a its device id, b 0).
//! Modes 0 and 2: it has left, and everything its holder was given is
//! revoked first (`broker::acpi::depart`, `depart_record`). Modes 1 and 3:
//! the ACPI capsule is about to run `_EJ0`, which waits for the holder:
//! EBUSY while anyone holds it, and nothing changes. Only the `acpi`
//! service's owner may ask. EPERM for anyone else; ENOENT for a device not
//! in the table; EINVAL for a field or mode out of range.

use crate::hardware::broker::acpi::depart::{depart, DepartError, Departed};
use crate::hardware::broker::acpi::depart_record::depart_record;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

fn answer(r: Result<Departed, DepartError>) -> i64 {
    match r {
        Ok(d) => d.device_id as i64,
        Err(DepartError::Unknown) => ERRNO_NOENT,
        Err(DepartError::Held(_)) => ERRNO_BUSY,
    }
}

pub fn sys_pci_depart(a: u64, b: u64, mode: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    match (mode, u8::try_from(a), u8::try_from(b)) {
        (0 | 1, Ok(bus), Ok(devfn)) => answer(depart(bus, devfn >> 3, devfn & 7, mode == 1)),
        (2 | 3, _, _) if b == 0 => answer(depart_record(a, mode == 3)),
        _ => ERRNO_INVAL,
    }
}

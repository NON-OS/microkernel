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

//! `MkPciProbeFunction(bus, devfn) -> device id`: the ACPI capsule has run
//! a function's _PS0 and asks the kernel to read that one function and add
//! it to the device table. Only the `acpi` service's owner may ask, on top
//! of the Admin gate every ACPI call has. EPERM for anyone else; EEXIST
//! for a function already in the table; ENOENT when it still does not
//! answer; EINVAL for a field out of range.

use crate::hardware::broker::acpi::probe_function::{probe_late, ProbeError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_EXIST, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

pub fn sys_pci_probe_function(bus: u64, devfn: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let (Ok(bus), Ok(devfn)) = (u8::try_from(bus), u8::try_from(devfn)) else {
        return ERRNO_INVAL;
    };
    match probe_late(bus, devfn >> 3, devfn & 7) {
        Ok(id) => id as i64,
        Err(ProbeError::Present) => ERRNO_EXIST,
        Err(ProbeError::Absent) => ERRNO_NOENT,
    }
}

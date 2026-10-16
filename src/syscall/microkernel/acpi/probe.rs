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

use crate::hardware::broker::acpi::claim_holder::tgid;
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::hardware::broker::acpi::marf_rule::Marf;
use crate::hardware::broker::acpi::probe_function::{probe_late, ProbeError};
use crate::hardware::broker::acpi::restore_function::{restore_function, RestoreError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_AGAIN, ERRNO_EXIST, ERRNO_INVAL, ERRNO_IO, ERRNO_NODEV, ERRNO_NOENT, ERRNO_PERM,
};

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

/// `MkAcpiRestoreFunction(device_id) -> 0`, the acpi owner's thread group
/// only (L64): after a claimed function's move back to D0, its BARs put
/// back. ENOENT for 0 or an unknown id, ENODEV when unclaimed, EAGAIN
/// unless in D0, EIO when a BAR would not stick and the claim was released.
pub fn sys_acpi_restore_function(device_id: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let Some(caller) = current_pid() else { return ERRNO_PERM };
    if !admits(owner, caller, tgid) {
        return ERRNO_PERM;
    }
    match restore_function(device_id) {
        Ok(()) => 0,
        Err(RestoreError::Refused(Marf::Unknown)) => ERRNO_NOENT,
        Err(RestoreError::Refused(Marf::NotClaimed)) => ERRNO_NODEV,
        Err(RestoreError::Refused(Marf::NotInD0)) => ERRNO_AGAIN,
        Err(RestoreError::Lost) => ERRNO_IO,
    }
}

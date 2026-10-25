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

//! `MkAcpiSlot(port, which, write, value)`: capsule_acpi drives a native
//! hot-plug slot through four registers of a port the kernel chose, and
//! asks the kernel what is below it (wire in broker::acpi::slot_wire).
//! Only the acpi owner's thread group may call it.

use crate::hardware::broker::acpi::claim_holder::tgid;
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::hardware::broker::acpi::slot_access::{port, read, write, SlotError};
use crate::hardware::broker::acpi::slot_below::{below_at, secondary};
use crate::hardware::broker::acpi::slot_wire::{unpack, BELOW_AT, PORT_AT, SECONDARY_BUS};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_INVAL, ERRNO_IO, ERRNO_NODEV, ERRNO_NOENT, ERRNO_PERM,
};

fn answer(r: Result<u64, SlotError>) -> i64 {
    match r {
        Ok(v) => v as i64,
        Err(SlotError::NotPort) => ERRNO_NODEV,
        Err(SlotError::Absent) => ERRNO_NOENT,
        Err(SlotError::Refused(_)) => ERRNO_ACCES,
        Err(SlotError::Platform) => ERRNO_IO,
    }
}

pub(super) fn sys_acpi_slot(at: u64, which: u64, write_it: u64, value: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let Some(caller) = current_pid() else { return ERRNO_PERM };
    if !admits(owner, caller, tgid) {
        return ERRNO_PERM;
    }
    let (Ok(n), Ok(v)) = (usize::try_from(value), u32::try_from(value)) else {
        return ERRNO_INVAL;
    };
    if which == PORT_AT {
        let asked = at == 0 && write_it == 0;
        return if asked { answer(port(n).ok_or(SlotError::Absent)) } else { ERRNO_INVAL };
    }
    let Some(f) = unpack(at) else { return ERRNO_INVAL };
    match (which, write_it) {
        (BELOW_AT, 0) => answer(below_at(f, n)),
        (SECONDARY_BUS, 0) if value == 0 => answer(secondary(f).map(u64::from)),
        (_, 0) if value == 0 => answer(read(f, which).map(u64::from)),
        (_, 1) => answer(write(f, which, v).map(|()| 0)),
        _ => ERRNO_INVAL,
    }
}

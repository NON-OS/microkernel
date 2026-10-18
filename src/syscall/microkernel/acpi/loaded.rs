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

//! `MkAcpiLoaded(ptr, len) -> 0`: a table AML is about to load at run time,
//! handed over by the acpi owner's thread group before it loads (F-93). The
//! kernel copies it, checks it whole and measures it into the firmware list
//! (broker::acpi::table_loaded). EINVAL for a length past the bound or
//! bytes that are not one whole table, ENOSPC when the list is full (the
//! registry then marked incomplete); refused, the capsule does not load it.

use crate::hardware::broker::acpi::claim_holder::tgid;
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::hardware::broker::acpi::loaded_rule::MAX_LOADED;
use crate::hardware::broker::acpi::table_loaded::{record_loaded, LoadedError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOSPC, ERRNO_PERM};
use alloc::vec;

pub fn sys_acpi_loaded(ptr: u64, len: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let Some(caller) = current_pid() else { return ERRNO_PERM };
    if !admits(owner, caller, tgid) {
        return ERRNO_PERM;
    }
    let Some(len) = usize::try_from(len).ok().filter(|l| (1..=MAX_LOADED).contains(l)) else {
        return ERRNO_INVAL;
    };
    let mut bytes = vec![0u8; len];
    if crate::usercopy::copy_from_user(ptr, &mut bytes).is_err() {
        return ERRNO_FAULT;
    }
    match record_loaded(&bytes) {
        Ok(()) => 0,
        Err(LoadedError::Malformed) => ERRNO_INVAL,
        Err(LoadedError::Full) => ERRNO_NOSPC,
    }
}

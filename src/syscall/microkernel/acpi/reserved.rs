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

//! `MkAcpiReserved(ranges, len) -> kept`: the ACPI capsule publishes the
//! I/O and memory its motherboard resource devices (PNP0C01, PNP0C02)
//! keep, 24 bytes a range, and the broker refuses from then on any grant
//! that runs into one. Only the `acpi` service's owner may call it; ranges
//! are only ever added. EINVAL for a malformed list; ENOSPC when the
//! broker keeps as many as it will.

use crate::hardware::broker::acpi::reserved_publish::{publish, ReservedError};
use crate::hardware::broker::reserved::{MAX_RANGES, RECORD_LEN};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOSPC, ERRNO_PERM};
use crate::usercopy::read_user_bytes;

pub fn sys_acpi_reserved(ptr: u64, len: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let most = MAX_RANGES * RECORD_LEN;
    let Some(len) = usize::try_from(len).ok().filter(|l| *l <= most) else { return ERRNO_INVAL };
    let Ok(bytes) = read_user_bytes(ptr, len) else { return ERRNO_FAULT };
    match publish(&bytes) {
        Ok(kept) => kept as i64,
        Err(ReservedError::Malformed) => ERRNO_INVAL,
        Err(ReservedError::Full) => ERRNO_NOSPC,
    }
}

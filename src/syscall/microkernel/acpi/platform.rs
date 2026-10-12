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

//! `MkAcpiPlatformDevice(record, len) -> device id`: the ACPI capsule
//! describes a platform controller from its evaluated `_CRS`, and the
//! kernel adds it to the device table for a driver to claim. Only the
//! `acpi` service's owner may call it. EEXIST, with the id, is not
//! returned: a device already in the table answers its id as success.
//! EINVAL for a malformed record; EBUSY when a window overlaps a device
//! already in the table; EACCES when the region rule refuses a window.

use crate::hardware::broker::acpi::platform_add::add;
use crate::hardware::broker::acpi::platform_decode::decode;
use crate::hardware::broker::acpi::platform_rule::Verdict;
use crate::hardware::broker::acpi::platform_table::judge;
use crate::hardware::broker::acpi::platform_wire::MAX_RECORD;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM,
};
use crate::usercopy::read_user_bytes;

pub fn sys_acpi_platform_device(ptr: u64, len: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let Some(pid) = owner.filter(|p| Some(*p) == current_pid()) else { return ERRNO_PERM };
    let Some(len) = usize::try_from(len).ok().filter(|l| *l <= MAX_RECORD) else {
        return ERRNO_INVAL;
    };
    let Ok(bytes) = read_user_bytes(ptr, len) else { return ERRNO_FAULT };
    let Some(p) = decode(&bytes) else { return ERRNO_INVAL };
    match judge(&p, pid) {
        Verdict::New => add(p) as i64,
        Verdict::Present(id) => id as i64,
        Verdict::Overlaps(_) => ERRNO_BUSY,
        Verdict::Refused(_) => ERRNO_ACCES,
    }
}

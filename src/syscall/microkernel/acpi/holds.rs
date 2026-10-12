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

//! `MkAcpiHolds(pid, segment, bus, devfn) -> 1 or 0`: whether `pid` (any
//! pid, for 0) holds the claim on that PCI function (devfn is device << 3 | function). The
//! ACPI capsule asks it before moving a device's power state for the pid
//! asking. ENOENT when there is no such function; EINVAL for a field
//! wider than its range.

use crate::hardware::broker::acpi::claim_holder::holds;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT};

pub fn sys_acpi_holds(pid: u64, segment: u64, bus: u64, devfn: u64) -> i64 {
    let (Ok(pid), Ok(seg), Ok(bus), Ok(devfn)) =
        (u32::try_from(pid), u16::try_from(segment), u8::try_from(bus), u8::try_from(devfn))
    else {
        return ERRNO_INVAL;
    };
    match holds(pid, (seg, bus, devfn >> 3, devfn & 7)) {
        Some(true) => 1,
        Some(false) => 0,
        None => ERRNO_NOENT,
    }
}

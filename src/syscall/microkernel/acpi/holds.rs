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

//! `MkAcpiHolds(mode, pid, segment, bus << 8 | devfn) -> 1 or 0`: whether
//! that PCI function is claimed, by anyone (mode 0, pid 0) or by that pid
//! (mode 1); devfn is device << 3 | function. ENOENT when there is no such
//! function; EINVAL for a mode and pid that do not go together, or a field
//! out of range.

use super::asker::asker;
use crate::hardware::broker::acpi::claim_holder::holds;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT};

pub fn sys_acpi_holds(mode: u64, pid: u64, segment: u64, bus_devfn: u64) -> i64 {
    let (Some(who), Ok(seg), Ok(bd)) =
        (asker(mode, pid), u16::try_from(segment), u16::try_from(bus_devfn))
    else {
        return ERRNO_INVAL;
    };
    let (bus, devfn) = ((bd >> 8) as u8, bd as u8);
    match holds(who, (seg, bus, devfn >> 3, devfn & 7)) {
        Some(true) => 1,
        Some(false) => 0,
        None => ERRNO_NOENT,
    }
}

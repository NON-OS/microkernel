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

//! `MkAcpiHwError(severity) -> 0`: the ACPI capsule decoded one hardware
//! error record (a GHES block or the boot error region) and logged it; the
//! kernel counts it on the posture line by its Generic Error Severity.
//! EINVAL for a severity ACPI 6.5 does not define.

use crate::hardware::broker::acpi::hw_errors::count;
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

pub fn sys_acpi_hw_error(severity: u64) -> i64 {
    if count(severity) {
        0
    } else {
        ERRNO_INVAL
    }
}

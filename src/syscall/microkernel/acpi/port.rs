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

//! `MkAcpiPort(port, width, write, value) -> value read, or 0`: one `in` or
//! `out` the kernel runs for the ACPI capsule on a SystemIO window
//! `MkAcpiRegion` granted it. Width is 1, 2 or 4 bytes.

use super::errno::region_errno;
use crate::hardware::broker::acpi::{port_read, port_write};
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::syscall::microkernel::narrow::{u32_arg, u8_arg};

pub fn sys_acpi_port(port: u64, width: u64, write: u64, value: u64) -> i64 {
    let Some(pid) = current_pid() else { return ERRNO_PERM };
    let Some(width) = u8_arg(width) else { return ERRNO_INVAL };
    if write == 0 {
        return port_read(pid, port, width).map_or_else(region_errno, i64::from);
    }
    let Some(value) = u32_arg(value) else { return ERRNO_INVAL };
    port_write(pid, port, width, value).map_or_else(region_errno, |()| 0)
}

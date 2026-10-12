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

//! `MkAcpiFunctionOf(mode, pid, device_id) -> segment << 16 | bus << 8 |
//! devfn`: the PCI function behind a broker device id, for the ACPI capsule
//! to find the device's ACPI object. Mode 0 (pid 0) is the capsule's own
//! look, for reading a device's state; mode 1 asks for that pid as the
//! device's holder, which is how DEVICE_POWER knows a driver asking to move
//! a device is the device's own. ENOENT for a device that is no PCI
//! function; EPERM when the pid does not hold it; EINVAL for a mode and pid
//! that do not go together.

use super::asker::asker;
use crate::hardware::broker::acpi::claim_holder::{function_of_device, FunctionError};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

pub fn sys_acpi_function_of(mode: u64, pid: u64, device_id: u64) -> i64 {
    let Some(who) = asker(mode, pid) else { return ERRNO_INVAL };
    match function_of_device(who, device_id) {
        Ok((seg, bus, dev, func)) => {
            i64::from(seg) << 16 | i64::from(bus) << 8 | i64::from(dev << 3 | func)
        }
        Err(FunctionError::NotPci) => ERRNO_NOENT,
        Err(FunctionError::NotHolder) => ERRNO_PERM,
    }
}

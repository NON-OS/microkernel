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

//! `MkAcpiDeviceName(mode, pid, device_id, out) -> bytes`: an ACPI-bus
//! device's `_HID`, `_UID` and namespace path, which its record has no room
//! for. `out` takes `NAME_MAX` bytes: the `_HID` (8, NUL-padded), the `_UID`
//! (u64), the path's length (u8, 0 when the kernel's static scan found the
//! device) and the path. Mode 0 (pid 0) is a look; mode 1 asks for that
//! pid as the device's holder, the same claim test MkAcpiFunctionOf makes
//! for a PCI function, so DEVICE_POWER reaches ACPI-bus devices too.
//! ENOENT for a device with no name kept; EPERM when the pid does not hold
//! it; EINVAL for a mode and pid that do not go together.

use super::asker::asker;
use crate::hardware::broker::acpi::device_holder::answers;
use crate::hardware::broker::acpi::platform_paths::named;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};
use crate::usercopy::write_user_bytes;
use alloc::vec::Vec;

const NAME_MAX: usize = 8 + 8 + 1 + 127;

pub fn sys_acpi_device_name(mode: u64, pid: u64, device_id: u64, out: u64) -> i64 {
    let Some(who) = asker(mode, pid) else { return ERRNO_INVAL };
    let Some((hid, uid, path)) = named(device_id) else { return ERRNO_NOENT };
    if !answers(who, device_id) {
        return ERRNO_PERM;
    }
    let path = &path[..path.len().min(127)];
    let mut b = Vec::with_capacity(NAME_MAX);
    b.extend_from_slice(&hid);
    b.extend_from_slice(&uid.to_le_bytes());
    b.push(path.len() as u8);
    b.extend_from_slice(path);
    match write_user_bytes(out, &b) {
        Ok(()) => b.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}

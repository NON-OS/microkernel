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

//! `MkSmbiosTable(which, out, len) -> bytes`: the SMBIOS entry point (0) or
//! its structure table (1), checked and copied, for the ACPI capsule to
//! serve read-only. A null `out` asks the length. ENOENT without SMBIOS;
//! EIO when the entry point fails its checksum.

use super::errno::table_errno;
use crate::hardware::broker::acpi::smbios::smbios_for_capsule;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_RANGE};
use crate::usercopy::copy_to_user;

pub fn sys_smbios_table(which: u64, out_ptr: u64, out_len: u64) -> i64 {
    let bytes = match smbios_for_capsule(which) {
        Ok(b) => b,
        Err(e) => return table_errno(e),
    };
    let len = bytes.len() as u64;
    if out_ptr == 0 {
        return len as i64;
    }
    if out_len < len {
        return ERRNO_RANGE;
    }
    match copy_to_user(out_ptr, &bytes) {
        Ok(()) => len as i64,
        Err(_) => ERRNO_FAULT,
    }
}

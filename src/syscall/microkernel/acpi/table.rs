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

//! `MkAcpiTable(signature, instance, out_ptr, out_len) -> length`: a copy of
//! one firmware table, checked against its own length and checksum, for the
//! ACPI capsule's interpreter. A null `out_ptr` asks only for the length;
//! a buffer shorter than the table gets nothing and ERANGE.

use super::errno::table_errno;
use crate::hardware::broker::acpi::table_for_capsule;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_RANGE};
use crate::usercopy::copy_to_user;

pub fn sys_acpi_table(signature: u64, instance: u64, out_ptr: u64, out_len: u64) -> i64 {
    let (Ok(sig), Ok(instance)) = (u32::try_from(signature), u32::try_from(instance)) else {
        return ERRNO_INVAL;
    };
    let bytes = match table_for_capsule(sig.to_le_bytes(), instance) {
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

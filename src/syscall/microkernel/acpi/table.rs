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

#[path = "loaded.rs"]
mod loaded;
pub use loaded::sys_acpi_loaded;

use super::errno::table_errno;
use crate::hardware::broker::acpi::table_for_capsule;
use crate::hardware::broker::acpi::table_measured::{record, records};
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_RANGE};
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
        Ok(()) => {
            // Measured as handed over, so what is attested is what was loaded.
            record(sig.to_le_bytes(), instance, &bytes);
            len as i64
        }
        Err(_) => ERRNO_FAULT,
    }
}

/// `MkAcpiMeasured(out_ptr, out_len)`: the measured firmware tables, 44
/// bytes each in order (broker::acpi::table_measure), for the About screen
/// and a verifier. Their digest is the attest registry's pid 0 entry, so
/// like `MkAttestEntries` they need not be believed, and are gated the same
/// way; a buffer too small is refused without writing.
pub fn sys_acpi_measured(out_ptr: u64, out_len: u64) -> i64 {
    let all = records();
    if all.len() as u64 > out_len {
        return ERRNO_NOMEM;
    }
    if crate::usercopy::validate_user_write(out_ptr, all.len()).is_err() {
        return ERRNO_FAULT;
    }
    match crate::usercopy::write_user_bytes(out_ptr, &all) {
        Ok(()) => all.len() as i64,
        Err(_) => ERRNO_FAULT,
    }
}

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

//! MkAcpiMeasured: the firmware tables the kernel measured (F-90), one
//! record a table in the order they were measured: signature (4 bytes),
//! instance and length (u32 big-endian each), BLAKE3 digest (32 bytes).
//! Their digest is the attest registry's pid 0 entry, so like
//! `mk_attest_entries` they need not be believed. Needs AttestRead.

use crate::syscall::call_raw;

const N_MK_ACPI_MEASURED: i64 = u32::from_le_bytes(*b"MAMT") as i64;

/// Bytes per record.
pub const ACPI_MEASURED_LEN: usize = 44;

/// The records, as many as fit; the bytes written, or a negative errno. A
/// buffer too small for all of them is refused without writing.
pub fn mk_acpi_measured(out: &mut [u8]) -> i64 {
    call_raw(N_MK_ACPI_MEASURED, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}

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

//! A table handed to the ACPI capsule, or refused, says so once on serial.

extern crate alloc;

use alloc::vec::Vec;

use super::table_check::TableError;
use super::tables::copy_table;
use crate::sys::serial::Line;

/// The table, checked, with its line. A table that does not exist is the
/// capsule's normal way of learning so and prints nothing.
pub fn table_for_capsule(sig: [u8; 4], instance: u32) -> Result<Vec<u8>, TableError> {
    let result = copy_table(sig, instance);
    let mut line = Line::new();
    line.str(b"[ACPI] table ").str(&sig).str(b" instance=").dec(u64::from(instance));
    match &result {
        Ok(bytes) => line.str(b" len=").dec(bytes.len() as u64).str(b" handed").end(),
        Err(TableError::Absent) => {}
        Err(e) => line.str(b" refused: ").str(reason(*e)).end(),
    }
    result
}

fn reason(e: TableError) -> &'static [u8] {
    match e {
        TableError::Absent => b"absent",
        TableError::Unmapped => b"outside the direct map",
        TableError::Memory => b"not in firmware memory",
        TableError::Length => b"length",
        TableError::Signature => b"signature",
        TableError::Checksum => b"checksum",
    }
}

/// One line per table measured (F-90): its first four digest bytes.
pub(super) fn measured(m: &super::table_measure::Measured) {
    let lead = u32::from_be_bytes([m.digest[0], m.digest[1], m.digest[2], m.digest[3]]);
    Line::new()
        .str(b"[ACPI] measured ")
        .str(&m.sig)
        .str(b" instance=")
        .dec(u64::from(m.instance))
        .str(b" len=")
        .dec(u64::from(m.len))
        .str(b" blake3=")
        .hex(u64::from(lead))
        .end();
}

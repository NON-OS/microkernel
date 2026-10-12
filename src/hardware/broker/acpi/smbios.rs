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

//! The SMBIOS entry point and structure table, copied for the ACPI capsule.
//! The bootloader found the entry point in the UEFI configuration table
//! (SMBIOS 3 preferred) and handed its address over; the kernel checks it
//! and reads only the bytes it names.

extern crate alloc;

use alloc::vec::Vec;

use super::smbios_entry::{entry_len, parse_entry};
use super::table_check::TableError;
use super::tables::read;

pub const ENTRY_POINT: u64 = 0;
pub const STRUCTURES: u64 = 1;
/// Bytes `entry_len` needs: "_SM3_" puts the length at offset 6.
const LEN_AT: usize = 7;

fn entry_point() -> Result<(u64, Vec<u8>), TableError> {
    let phys = crate::boot::handoff::get_handoff().map(|h| h.smbios.entry);
    let phys = phys.filter(|p| *p != 0).ok_or(TableError::Absent)?;
    // The anchor and the length byte first, then exactly the entry point:
    // a fixed ENTRY_MAX could run past it into memory the rule refuses.
    let len = entry_len(&read(phys, LEN_AT)?).ok_or(TableError::Signature)?;
    Ok((phys, read(phys, len)?))
}

pub fn smbios_for_capsule(which: u64) -> Result<Vec<u8>, TableError> {
    let (_, entry) = entry_point()?;
    let e = parse_entry(&entry).ok_or(TableError::Checksum)?;
    match which {
        ENTRY_POINT => Ok(entry),
        STRUCTURES => read(e.table, e.len as usize),
        _ => Err(TableError::Absent),
    }
}

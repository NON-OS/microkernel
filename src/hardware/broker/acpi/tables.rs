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

//! Copying a firmware table out for the ACPI capsule. The capsule gets bytes
//! the kernel copied and checked, never a mapping of firmware's memory, so
//! nothing it does reaches the tables the kernel itself parsed.

extern crate alloc;

use alloc::vec::Vec;
use core::ptr;

use super::kernel::platform::fw::parser::{phys::directmap, state::TABLES};
use super::kernel::Machine;
use super::rule::table_read;
use super::table_allow::readable;
use super::table_check::{declared_length, verify, TableError, FACS_HEADER, HEADER};

/// Where the `instance`th table with `sig` is. The DSDT and FACS are named
/// by the FADT (ACPI 6.5, 5.2.9), SSDTs share one signature so each has an
/// index, and every other table is one per signature.
pub(super) fn locate(sig: [u8; 4], instance: u32) -> Option<u64> {
    let tables = TABLES.read();
    let reg = tables.as_ref()?;
    let one = |a: u64| (instance == 0 && a != 0).then_some(a);
    match &sig {
        b"DSDT" => one(reg.data.fadt?.dsdt),
        b"FACS" => one(reg.data.fadt?.firmware_ctrl),
        b"SSDT" => reg.ssdt_addresses.get(instance as usize).copied(),
        _ => one(*reg.tables.get(&u32::from_le_bytes(sig))?),
    }
}

/// Bytes of firmware's memory (`rule::table`) through the direct map.
pub(super) fn read(phys: u64, len: usize) -> Result<Vec<u8>, TableError> {
    table_read(phys, len as u64, &Machine { pid: 0 }).map_err(|_| TableError::Memory)?;
    let last = phys.checked_add(len as u64 - 1).ok_or(TableError::Unmapped)?;
    let virt = directmap(phys).ok_or(TableError::Unmapped)?;
    directmap(last).ok_or(TableError::Unmapped)?;
    let mut buf = Vec::new();
    buf.try_reserve_exact(len).map_err(|_| TableError::Length)?;
    // SAFETY: both ends of the range are inside the direct map, which maps
    // physical memory contiguously; `buf` has room for `len` bytes and its
    // length is set only once they are written.
    unsafe {
        ptr::copy_nonoverlapping(virt as *const u8, buf.as_mut_ptr(), len);
        buf.set_len(len);
    }
    Ok(buf)
}

/// The table, verified; any table outside `readable` reads as absent.
pub(super) fn copy_table(sig: [u8; 4], instance: u32) -> Result<Vec<u8>, TableError> {
    if !readable(sig) {
        return Err(TableError::Absent);
    }
    let phys = locate(sig, instance).ok_or(TableError::Absent)?;
    let head = read(phys, if sig == *b"FACS" { FACS_HEADER } else { HEADER })?;
    let bytes = read(phys, declared_length(&head, sig)?)?;
    verify(&bytes, sig)?;
    Ok(bytes)
}

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

//! The ACPI TPM2 table: which interface the firmware says the part has,
//! decoded once by the one parser (`tpm2_table`) and served read-only to
//! whoever drives the TPM. A table that fails its length or checksum is
//! ignored rather than believed, and the register file decides alone.

use super::tpm2_table::{decode, Tpm2Fault, Tpm2Table, MAX_LEN, MIN_LEN};
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;
use alloc::vec::Vec;

pub(super) use super::tpm2_start::{START_ACPI, START_CRB, START_CRB_ACPI, START_FIFO};

/// The table as firmware published it, or None without one that checks out.
pub fn tpm2_table() -> Option<Tpm2Table> {
    let phys = crate::arch::firmware_table::acpi_table_address(b"TPM2")?;
    let base = phys_to_virt(PhysAddr::new(phys))?.as_u64() as *const u8;
    let len = u32::from_le_bytes(bytes::<4>(base, 4)) as usize;
    // Nothing past the header is read until its length is in bounds.
    let len = if (MIN_LEN..=MAX_LEN).contains(&len) { len } else { 8 };
    let t: Vec<u8> = (0..len).map(|i| bytes::<1>(base, i)[0]).collect();
    match decode(&t) {
        Ok(table) => Some(table),
        Err(Tpm2Fault::Length(n)) => {
            crate::log::warn!("[TPM] ACPI TPM2 table length {} out of range; ignored", n);
            None
        }
        Err(Tpm2Fault::Checksum(sum)) => {
            crate::log::warn!("[TPM] ACPI TPM2 table checksum off by {}; ignored", sum);
            None
        }
    }
}

/// The start method the firmware published.
pub(super) fn start_method() -> Option<u32> {
    tpm2_table().map(|t| t.start_method)
}

/// The CRB control area's physical address as the TPM2 table gives it.
/// Intel's firmware TPM puts it in the register window at 0xFED40040;
/// AMD's puts it in memory of its own, with no register window at all.
pub(in crate::security::tpm) fn control_area() -> Option<u64> {
    tpm2_table().map(|t| t.control_area).filter(|at| *at != 0)
}

fn bytes<const N: usize>(base: *const u8, at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    for (i, slot) in out.iter_mut().enumerate() {
        // SAFETY: eK@nonos.systems - `base` is the directmap view of a table
        // the ACPI parser found in the root table; every offset read is
        // below its length bounded to MAX_LEN, and the first read is inside
        // the fixed 36-byte header every table has.
        *slot = unsafe { core::ptr::read_volatile(base.add(at + i)) };
    }
    out
}

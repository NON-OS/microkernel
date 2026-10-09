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

//! The ACPI TPM2 table: which interface the firmware says the part has.

use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;

/// Start methods of the TCG ACPI specification this kernel drives.
pub(super) const START_FIFO: u32 = 6;
pub(super) const START_CRB: u32 = 7;
pub(super) const START_CRB_ACPI: u32 = 8;
/// The ACPI start method of AMD's firmware TPM: the doorbell is a _DSM call,
/// which needs an AML interpreter this kernel does not have.
pub(super) const START_ACPI: u32 = 2;

/// Header (36), platform class and reserved (4), control area (8), start
/// method (4). Anything after is start-method parameters and the log area.
const CONTROL_AREA_AT: usize = 40;
const START_METHOD_AT: usize = 48;
const MIN_LEN: usize = 52;
/// No revision of the table comes near this; a larger length is corrupt.
const MAX_LEN: usize = 4096;

/// The start method the firmware published, or `None` when there is no
/// TPM2 table or it does not check out. A table that fails its checksum is
/// ignored rather than believed, and the register file decides alone.
pub(super) fn start_method() -> Option<u32> {
    let base = table()?;
    Some(u32::from_le_bytes(bytes::<4>(base, START_METHOD_AT)))
}

/// The CRB control area's physical address as the TPM2 table gives it, or
/// `None` without a table that checks out. Intel's firmware TPM puts it in
/// the register window at 0xFED40040; AMD's puts it in memory of its own,
/// with no register window at 0xFED40000 at all.
pub(in crate::security::tpm) fn control_area() -> Option<u64> {
    let base = table()?;
    let at = u64::from_le_bytes(bytes::<8>(base, CONTROL_AREA_AT));
    (at != 0).then_some(at)
}

/// The table's directmap view, once its length and checksum check out.
fn table() -> Option<*const u8> {
    let phys = crate::arch::firmware_table::acpi_table_address(b"TPM2")?;
    let base = phys_to_virt(PhysAddr::new(phys))?.as_u64() as *const u8;
    let len = u32::from_le_bytes(bytes::<4>(base, 4)) as usize;
    if !(MIN_LEN..=MAX_LEN).contains(&len) {
        crate::log::warn!("[TPM] ACPI TPM2 table length {} out of range; ignored", len);
        return None;
    }
    let mut sum = 0u8;
    for i in 0..len {
        sum = sum.wrapping_add(bytes::<1>(base, i)[0]);
    }
    if sum != 0 {
        crate::log::warn!("[TPM] ACPI TPM2 table checksum off by {}; ignored", sum);
        return None;
    }
    Some(base)
}

fn bytes<const N: usize>(base: *const u8, at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    for (i, slot) in out.iter_mut().enumerate() {
        // SAFETY: eK@nonos.systems - `base` is the directmap view of a table
        // the ACPI parser found in the root table; every offset read is
        // below its length, which was bounded to MAX_LEN before the loop
        // that reads the whole table, and the first read is inside the
        // fixed 36-byte header every table has.
        *slot = unsafe { core::ptr::read_volatile(base.add(at + i)) };
    }
    out
}

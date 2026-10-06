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

//! Finding one MSI-X table entry, reading its pending bit, and clearing an
//! entry the kernel is done with.

use super::super::constants::*;
use super::super::error::{PciError, Result};
use super::super::types::{MsixInfo, PciBar};
use super::msix_window::{map_msix_window, MappedMsixWindow};

pub(super) fn map_msix_table_entry(
    msix: &MsixInfo,
    bars: &[PciBar; 6],
    vector: u16,
) -> Result<MappedMsixWindow> {
    if vector > msix.table_size {
        return Err(PciError::MsixVectorOutOfRange { vector, max: msix.table_size });
    }
    let entry_offset = (msix.table_offset as u64)
        .checked_add((vector as u64) * (MSIX_ENTRY_SIZE as u64))
        .ok_or(PciError::MsixTableAccessFailed)?;
    map_msix_window(bars, msix.table_bar, entry_offset, MSIX_ENTRY_SIZE as u64)
}

pub fn is_msix_vector_pending(msix: &MsixInfo, bars: &[PciBar; 6], vector: u16) -> Result<bool> {
    if vector > msix.table_size {
        return Err(PciError::MsixVectorOutOfRange { vector, max: msix.table_size });
    }

    let qword_index = vector / 64;
    let bit_index = vector % 64;
    let pba_offset = (msix.pba_offset as u64)
        .checked_add(qword_index as u64 * 8)
        .ok_or(PciError::MsixTableAccessFailed)?;
    let pba = map_msix_window(bars, msix.pba_bar, pba_offset, 8)?;

    let low = crate::memory::mmio::mmio_r32(pba.addr) as u64;
    let high = crate::memory::mmio::mmio_r32(pba.addr + 4u64) as u64;
    let pending = (high << 32) | low;

    Ok((pending & (1u64 << bit_index)) != 0)
}

pub fn zero_msix_vector(msix: &MsixInfo, bars: &[PciBar; 6], vector: u16) -> Result<()> {
    let entry = map_msix_table_entry(msix, bars, vector)?;
    crate::memory::mmio::mmio_w32(entry.addr, 0);
    crate::memory::mmio::mmio_w32(entry.addr + 4u64, 0);
    crate::memory::mmio::mmio_w32(entry.addr + 8u64, 0);
    crate::memory::mmio::mmio_w32(entry.addr + 12u64, MSIX_ENTRY_MASKED);
    Ok(())
}

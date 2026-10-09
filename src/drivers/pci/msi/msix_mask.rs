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

//! The function-wide MSI-X mask and the per-entry mask bits.

use super::super::config::ConfigSpace;
use super::super::constants::*;
use super::super::error::Result;
use super::super::types::{MsixInfo, PciBar};
use super::msix_entry::map_msix_table_entry;

pub fn mask_all_msix(config: &ConfigSpace, msix: &MsixInfo) -> Result<()> {
    let offset = msix.offset as u16;
    let mut ctrl = config.read16(offset + 2)?;
    ctrl |= MSIX_CTRL_FUNCTION_MASK;
    config.write16(offset + 2, ctrl)?;
    Ok(())
}

pub fn unmask_all_msix(config: &ConfigSpace, msix: &MsixInfo) -> Result<()> {
    let offset = msix.offset as u16;
    let mut ctrl = config.read16(offset + 2)?;
    ctrl &= !MSIX_CTRL_FUNCTION_MASK;
    config.write16(offset + 2, ctrl)?;
    Ok(())
}

pub fn mask_msix_vector(msix: &MsixInfo, bars: &[PciBar; 6], vector: u16) -> Result<()> {
    let entry = map_msix_table_entry(msix, bars, vector)?;
    let ctrl_addr = entry.addr + MSIX_ENTRY_VECTOR_CTRL as u64;

    let current = crate::memory::mmio::mmio_r32(ctrl_addr);
    crate::memory::mmio::mmio_w32(ctrl_addr, current | MSIX_ENTRY_MASKED);

    Ok(())
}

pub fn unmask_msix_vector(msix: &MsixInfo, bars: &[PciBar; 6], vector: u16) -> Result<()> {
    let entry = map_msix_table_entry(msix, bars, vector)?;
    let ctrl_addr = entry.addr + MSIX_ENTRY_VECTOR_CTRL as u64;

    let current = crate::memory::mmio::mmio_r32(ctrl_addr);
    crate::memory::mmio::mmio_w32(ctrl_addr, current & !MSIX_ENTRY_MASKED);

    Ok(())
}

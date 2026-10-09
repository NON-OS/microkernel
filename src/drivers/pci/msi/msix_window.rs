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

//! Mapping the BAR page holding an MSI-X entry or PBA qword for one access.

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout::{PAGE_SIZE, PAGE_SIZE_U64};
use crate::memory::mmio::{map_device_memory, unmap_mmio};

use super::super::error::{PciError, Result};
use super::super::types::PciBar;

const PAGE_OFFSET_MASK: u64 = PAGE_SIZE_U64 - 1;

pub(super) struct MappedMsixWindow {
    base: VirtAddr,
    pub(super) addr: VirtAddr,
}

impl Drop for MappedMsixWindow {
    fn drop(&mut self) {
        let _ = unmap_mmio(self.base);
    }
}

pub(super) fn map_msix_window(
    bars: &[PciBar; 6],
    bar_index: u8,
    offset: u64,
    access_len: u64,
) -> Result<MappedMsixWindow> {
    if access_len == 0 {
        return Err(PciError::MsixTableAccessFailed);
    }
    let bar = bars.get(bar_index as usize).ok_or(PciError::MsixTableAccessFailed)?;
    let bar_base = bar.address().ok_or(PciError::MsixTableAccessFailed)?;
    let bar_size = bar.size();
    let end = offset.checked_add(access_len).ok_or(PciError::MsixTableAccessFailed)?;
    if end > bar_size {
        return Err(PciError::MsixTableAccessFailed);
    }
    let phys = bar_base.as_u64().checked_add(offset).ok_or(PciError::MsixTableAccessFailed)?;
    let page_phys = phys & !PAGE_OFFSET_MASK;
    let page_off = phys & PAGE_OFFSET_MASK;
    let map_len = if page_off + access_len > PAGE_SIZE_U64 { PAGE_SIZE * 2 } else { PAGE_SIZE };
    let base = map_device_memory(PhysAddr::new(page_phys), map_len)
        .map_err(|_| PciError::MsixTableAccessFailed)?;
    Ok(MappedMsixWindow { base, addr: base + page_off })
}

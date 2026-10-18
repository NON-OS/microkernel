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

use crate::hardware::broker::dma::cache::flush_range;
use crate::hardware::broker::dma::flags::{DMA_MAP_COHERENT, DMA_MAP_WC};
use crate::hardware::broker::dma::types::DmaMapError;
use crate::hardware::broker::dma::va;
use crate::memory::addr::PhysAddr;

// Reserve user VA + install page table entries, from the holder's window
//; a failure gives the range back. The caller frees physical frames;
// this layer owns only the VA/page state. The zeroing went through the
// caches. A coherent or write-combining grant is flushed first, so its non
// write-back mapping does not read memory the zeroing left behind in a dirty
// line; so is every grant where devices need not snoop, or the device would
// read the frames' last owner's data, and a dirty zero line evicted later
// would land on what the device wrote.
pub(super) fn install(
    pid: u32,
    pages: u64,
    length: u64,
    phys_start: u64,
    flags: u32,
) -> Result<u64, DmaMapError> {
    let user_va = va::reserve(pid, pages).ok_or(DmaMapError::NoVaSpace)?;
    let phys = PhysAddr::new(phys_start);
    let uncached = flags & (DMA_MAP_COHERENT | DMA_MAP_WC) != 0;
    if uncached || !crate::arch::dma_snoops_caches() {
        flush_range(phys_start, length);
    }
    let mapped = if uncached {
        let wc = flags & DMA_MAP_WC != 0;
        crate::memory::paging::map_user_dma_coherent(user_va, phys, length as usize, wc)
    } else {
        crate::memory::paging::map_user_dma(user_va, phys, length as usize)
    };
    if mapped.is_err() {
        uninstall(pid, user_va.as_u64(), length);
        return Err(DmaMapError::MapFailed);
    }
    if flags & (DMA_MAP_COHERENT | DMA_MAP_WC) != 0 {
        super::super::cache::say_mapped(phys_start, pages, flags & DMA_MAP_WC != 0);
    }
    Ok(user_va.as_u64())
}

// Undo `install` for a grant that failed later, its range back to the window.
pub(super) fn uninstall(pid: u32, user_va: u64, length: u64) {
    let _ = crate::memory::paging::unmap_user_dma(
        crate::memory::addr::VirtAddr::new(user_va),
        length as usize,
    );
    va::release(pid, user_va, length);
}

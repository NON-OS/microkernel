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

// Reserve user VA + install page table entries. On a map failure the VA
// slot is not released — the bump allocator leaks it for the life of the
// boot; same shape as the original. The caller frees physical frames; this
// layer owns only the VA/page state. A coherent or write-combining grant is
// flushed from the caches first, so its non write-back mapping does not read
// memory the zeroing left behind in a dirty line.
pub(super) fn install(
    pages: u64,
    length: u64,
    phys_start: u64,
    flags: u32,
) -> Result<u64, DmaMapError> {
    let user_va = va::reserve(pages).ok_or(DmaMapError::NoVaSpace)?;
    let phys = PhysAddr::new(phys_start);
    let mapped = if flags & (DMA_MAP_COHERENT | DMA_MAP_WC) != 0 {
        flush_range(phys_start, length);
        let wc = flags & DMA_MAP_WC != 0;
        crate::memory::paging::map_user_dma_coherent(user_va, phys, length as usize, wc)
    } else {
        crate::memory::paging::map_user_dma(user_va, phys, length as usize)
    };
    if mapped.is_err() {
        return Err(DmaMapError::MapFailed);
    }
    if flags & (DMA_MAP_COHERENT | DMA_MAP_WC) != 0 {
        super::super::cache::say_mapped(phys_start, pages, flags & DMA_MAP_WC != 0);
    }
    Ok(user_va.as_u64())
}

// Undo `install` for a grant that failed later; the VA slot stays leaked, as above.
pub(super) fn uninstall(user_va: u64, length: u64) {
    let _ = crate::memory::paging::unmap_user_dma(
        crate::memory::addr::VirtAddr::new(user_va),
        length as usize,
    );
}

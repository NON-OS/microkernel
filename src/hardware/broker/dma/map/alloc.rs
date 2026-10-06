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

use super::super::pool;
use crate::hardware::broker::dma::flags::{DMA_MAP_DMA32, DMA_MAP_HIGH};
use crate::hardware::broker::dma::types::DmaMapError;
use crate::memory::phys::{alloc_contiguous, free_contiguous, AllocFlags};

// Allocate `pages` physically-contiguous frames and re-scrub through
// the direct map so the buffer is provably zero before it leaves the
// kernel. The phys allocator's ZERO flag is best-effort across zones;
// the volatile loop here is the load-bearing scrub.
//
// Where the frames come from, by request flag:
//   HIGH   the display pool, else anywhere, highest first
//   DMA32  the low pool, else below 4 GiB, else no memory: the device
//          names buffers with 32-bit addresses (an AHCI HBA without
//          CAP.S64A) and a frame above 4 GiB would be truncated
//   none   the low pool, else below 4 GiB, else anywhere, so a device
//          with no IOMMU in front of it still gets low memory when
//          there is some
pub(super) fn alloc_and_zero(pages: u64, length: u64, dma_flags: u32) -> Result<u64, DmaMapError> {
    let phys_start = take(pages as usize, dma_flags).ok_or(DmaMapError::NoMemory)?;
    super::zero::zero_run(phys_start, length);
    Ok(phys_start)
}

fn take(pages: usize, dma_flags: u32) -> Option<u64> {
    let base = AllocFlags::DMA | AllocFlags::ZERO;
    if dma_flags & DMA_MAP_HIGH != 0 {
        return pool::alloc(pages).or_else(|| alloc_contiguous(pages, base | AllocFlags::HIGH));
    }
    // A buffer small enough for the reserved low pool goes there first. A map
    // that is not DMA32 stops short of the pool's floor, kept for those that are.
    if let Some(phys_start) = pool::low32_alloc(pages, dma_flags & DMA_MAP_DMA32 != 0) {
        return Some(phys_start);
    }
    if let Some(phys_start) = alloc_contiguous(pages, base | AllocFlags::DMA32) {
        return Some(phys_start);
    }
    if dma_flags & DMA_MAP_DMA32 != 0 {
        return None;
    }
    alloc_contiguous(pages, base)
}

/// Give back frames `alloc_and_zero` handed out, to the pool they came from:
/// the low pool, the display pool, or the general allocator. Freeing a pool
/// frame into the general allocator would hand it out twice.
pub(super) fn free(phys_start: u64, pages: u64) {
    let pages = pages as usize;
    if !pool::give_back(phys_start, pages) {
        let _ = free_contiguous(phys_start, pages);
    }
}

pub(super) use crate::hardware::broker::dma::placement::fits_below_4g;

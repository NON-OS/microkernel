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
use super::take::take;
use crate::hardware::broker::dma::types::DmaMapError;
use crate::memory::phys::free_contiguous;

// Allocate `pages` physically-contiguous frames (take.rs says from where)
// and re-scrub them through the direct map: the allocator's ZERO is
// best-effort, the volatile loop here is what holds.
pub(super) fn alloc_and_zero(
    pages: u64,
    length: u64,
    dma_flags: u32,
    uncached_frames: bool,
) -> Result<u64, DmaMapError> {
    let phys_start = if uncached_frames {
        pool::uncached_alloc(pages as usize)
    } else {
        take(pages as usize, dma_flags)
    };
    let phys_start = phys_start.ok_or(DmaMapError::NoMemory)?;
    super::zero::zero_run(phys_start, length);
    Ok(phys_start)
}

/// Give back frames `alloc_and_zero` handed out, to the pool they came from:
/// the uncached, low or display pool, or the general allocator. Freeing a pool
/// frame into the general allocator would hand it out twice.
pub(super) fn free(phys_start: u64, pages: u64) {
    let pages = pages as usize;
    if !pool::give_back(phys_start, pages) {
        let _ = free_contiguous(phys_start, pages);
    }
}

pub(super) use crate::hardware::broker::dma::placement::fits_below_4g;

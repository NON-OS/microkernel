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

use super::super::error::{PageAllocError, PageAllocResult};
use super::super::types::{PageAllocatorStats, PageInfo};
use super::globals::{with_allocator, ALLOCATOR_STATS};
use crate::memory::addr::VirtAddr;
use crate::memory::{buddy_alloc, layout};
use core::sync::atomic::Ordering;

pub fn init() -> PageAllocResult<()> {
    buddy_alloc::init().map_err(|_| PageAllocError::MappingFailed)?;
    with_allocator(|a| a.init())
}
pub fn allocate_pages(count: usize) -> PageAllocResult<VirtAddr> {
    // checked_mul so an oversized count returns InvalidSize instead of
    // overflowing (and aborting under release overflow-checks) before the
    // allocator's own size ceiling is even consulted.
    let size = count.checked_mul(layout::PAGE_SIZE).ok_or(PageAllocError::InvalidSize)?;
    super::sized::allocate_sized(size)
}

pub fn get_page_info(va: VirtAddr) -> Option<PageInfo> {
    with_allocator(|a| a.get_page_info(va).copied()).map(|p| PageInfo {
        page_id: p.page_id,
        virtual_addr: p.virtual_addr,
        physical_addr: p.physical_addr,
        allocation_time: p.allocation_time,
        size: p.size,
    })
}

pub fn get_stats() -> PageAllocatorStats {
    with_allocator(|a| a.get_allocator_stats())
}
pub fn is_allocated(va: VirtAddr) -> bool {
    with_allocator(|a| a.get_page_info(va).is_some())
}
pub fn get_allocation_count() -> usize {
    ALLOCATOR_STATS.active_pages.load(Ordering::Relaxed)
}
pub fn get_total_bytes_allocated() -> u64 {
    ALLOCATOR_STATS.bytes_allocated.load(Ordering::Relaxed)
}
pub fn get_peak_pages() -> usize {
    ALLOCATOR_STATS.peak_pages.load(Ordering::Relaxed)
}
pub fn is_initialized() -> bool {
    with_allocator(|a| a.initialized)
}

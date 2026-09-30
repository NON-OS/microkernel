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

//! Allocations of any page-multiple size: map, zero and record on the way
//! in; forget, scrub and unmap on the way out.

use super::super::constants::ZERO_PATTERN;
use super::super::error::PageAllocResult;
use super::globals::{with_allocator, ALLOCATOR_STATS};
use super::mapping::{allocate_virtual_pages, free_virtual_pages, get_physical_address};
use crate::memory::addr::VirtAddr;
use crate::memory::layout;

/// Map fresh pages, then track them. The allocator lock is held only to
/// admit and to record, never across the mapping (see `alloc`).
pub fn allocate_sized(size: usize) -> PageAllocResult<VirtAddr> {
    let page_count = with_allocator(|a| a.admit(size))?;
    let total_size = page_count * layout::PAGE_SIZE;
    let va = allocate_virtual_pages(page_count)?;
    let recorded = get_physical_address(va).and_then(|pa| {
        /*
         * SAFETY: eK@nonos.systems - `va` is `total_size` bytes this call
         * mapped writable a moment ago and nobody else has been given yet.
         */
        unsafe { core::ptr::write_bytes(va.as_mut_ptr::<u8>(), ZERO_PATTERN, total_size) };
        with_allocator(|a| a.record(va, pa, total_size))
    });
    if let Err(e) = recorded {
        let _ = free_virtual_pages(va, page_count);
        return Err(e);
    }
    Ok(va)
}
/// Stop tracking the allocation at `va`, then scrub and unmap it outside the
/// allocator lock. Called from the timer tick for kernel stacks.
pub fn deallocate_page(va: VirtAddr) -> PageAllocResult<()> {
    let page = with_allocator(|a| a.take(va))?;
    /*
     * SAFETY: eK@nonos.systems - the record said `page.size` bytes at `va`
     * are ours and mapped; taking it out means nobody else frees them.
     */
    unsafe { core::ptr::write_bytes(va.as_mut_ptr::<u8>(), ZERO_PATTERN, page.size) };
    free_virtual_pages(va, page.size / layout::PAGE_SIZE)?;
    ALLOCATOR_STATS.record_deallocation(page.size);
    Ok(())
}

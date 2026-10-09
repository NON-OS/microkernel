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

//! The allocator's two short critical sections around an allocation.
//!
//! Mapping the pages is done between them, outside the lock: it takes the
//! paging lock and may take part in a TLB shootdown, and a lock held across a
//! shootdown wait stalls every cpu that wants it with interrupts masked. That
//! was a machine halt: a spawn held this lock while its mapping waited for
//! acknowledgements, and a cpu freeing a kernel stack from its timer tick
//! spun on this lock and could not acknowledge.

use super::super::constants::{MAX_ALLOCATION_SIZE, MAX_TRACKED_PAGES};
use super::super::error::{PageAllocError, PageAllocResult};
use super::super::types::AllocatedPage;
use super::allocator::PageAllocator;
use super::globals::{get_timestamp, ALLOCATOR_STATS};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout;

impl PageAllocator {
    /// Whether an allocation of `size` bytes may go ahead, as a page count.
    pub(super) fn admit(&self, size: usize) -> PageAllocResult<usize> {
        if !self.initialized {
            return Err(PageAllocError::NotInitialized);
        }
        if size == 0 || size > MAX_ALLOCATION_SIZE {
            return Err(PageAllocError::InvalidSize);
        }
        if self.allocated_pages.len() >= MAX_TRACKED_PAGES {
            return Err(PageAllocError::TooManyPages);
        }
        Ok(size.div_ceil(layout::PAGE_SIZE))
    }

    /// Track pages already mapped at `va`. Refused when the table filled up
    /// since `admit`; the caller then gives the pages back.
    pub(super) fn record(
        &mut self,
        va: VirtAddr,
        pa: PhysAddr,
        total_size: usize,
    ) -> PageAllocResult<()> {
        if self.allocated_pages.len() >= MAX_TRACKED_PAGES {
            return Err(PageAllocError::TooManyPages);
        }
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        self.allocated_pages.push(AllocatedPage {
            page_id,
            virtual_addr: va,
            physical_addr: pa,
            allocation_time: get_timestamp(),
            size: total_size,
        });
        ALLOCATOR_STATS.record_allocation(total_size);
        Ok(())
    }
}

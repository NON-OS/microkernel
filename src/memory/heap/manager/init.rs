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

//! The heap's second region. The kernel starts on the bootstrap heap, a fixed
//! array in .bss, and keeps every block it allocates there; this adds the
//! second region beside it rather than moving the heap, since the allocator's
//! blocks cannot be moved and a free must reach the region that handed the
//! block out (types/dealloc_impl.rs). The region is physically contiguous and
//! reached through the directmap, so it costs no page-table mapping and no
//! paging record per page.

use super::super::constants::{BOOTSTRAP_HEAP_SIZE, SECOND_REGION_MAX, SECOND_REGION_MIN};
use super::super::error::{HeapError, HeapResult};
use super::globals::{HEAP_STATS, KERNEL_HEAP, USING_BOOTSTRAP};
use crate::memory::addr::PhysAddr;
use crate::memory::phys::{self, AllocFlags};
use core::sync::atomic::Ordering;

const PAGE: usize = 4096;

/// Add the second region, once paging and the frame allocator are up. When
/// no run of that size is free, half of it is tried, down to the minimum; with
/// none, the kernel stays on the bootstrap heap and says so.
pub fn init() -> HeapResult<usize> {
    if !KERNEL_HEAP.is_initialized() {
        return Err(HeapError::NotInitialized);
    }
    if KERNEL_HEAP.second_extent().is_some() {
        return Err(HeapError::AlreadyInitialized);
    }
    let mut size = super::sizing::second_region_size(
        phys::total_memory(),
        SECOND_REGION_MIN,
        SECOND_REGION_MAX,
        PAGE,
    );
    while size >= SECOND_REGION_MIN {
        if let Some(pa) = phys::alloc_contiguous(size / PAGE, AllocFlags::HIGH) {
            let Some(va) = crate::memory::unified::phys_to_virt(PhysAddr::new(pa)) else {
                let _ = phys::free_contiguous(pa, size / PAGE);
                return Err(HeapError::MappingFailed);
            };
            // SAFETY: the frames were just taken from the allocator, so nothing
            // else holds them, and the directmap maps them writable.
            if !unsafe { KERNEL_HEAP.add_region(va.as_mut_ptr(), size) } {
                let _ = phys::free_contiguous(pa, size / PAGE);
                return Err(HeapError::AlreadyInitialized);
            }
            HEAP_STATS.set_total_size(BOOTSTRAP_HEAP_SIZE + size);
            USING_BOOTSTRAP.store(false, Ordering::Release);
            return Ok(size);
        }
        size /= 2;
    }
    Err(HeapError::FrameAllocationFailed)
}

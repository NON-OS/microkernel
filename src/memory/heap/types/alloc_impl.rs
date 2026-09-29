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

use super::super::constants::MIN_ALIGNMENT;
use super::allocator::SecureHeapAllocator;
use super::header::{data_offset, AllocationHeader};
use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr::{self, null_mut};
use core::sync::atomic::Ordering;

// The data pointer is aligned to `layout.align().max(MIN_ALIGNMENT)` ≥ 8, and
// the header before it to 8.
#[allow(clippy::cast_ptr_alignment)]
pub(super) unsafe fn alloc_impl(allocator: &SecureHeapAllocator, layout: Layout) -> *mut u8 {
    unsafe {
        if !allocator.is_initialized() {
            return null_mut();
        }

        // The data, not the block, must meet the alignment: a header in front
        // of an aligned block left a 64-aligned request only 32-aligned.
        let header_size = mem::size_of::<AllocationHeader>();
        let align = layout.align().max(MIN_ALIGNMENT);
        let offset = data_offset(align);
        let Some(total_size) =
            offset.checked_add(layout.size()).and_then(|s| s.checked_add(mem::size_of::<u64>()))
        else {
            return null_mut();
        };
        let Ok(adjusted_layout) = Layout::from_size_align(total_size, align) else {
            return null_mut();
        };

        let raw_ptr =
            crate::arch::run_without_interrupts(|| allocator.inner.alloc(adjusted_layout));
        if raw_ptr.is_null() {
            return null_mut();
        }

        let data_ptr = raw_ptr.add(offset);
        let header_ptr = data_ptr.sub(header_size) as *mut AllocationHeader;
        let canary_ptr = data_ptr.add(layout.size()) as *mut u64;

        let header = AllocationHeader::new(layout.size(), super::super::manager::get_timestamp());
        ptr::write_volatile(header_ptr, header);
        ptr::write_volatile(canary_ptr, allocator.canary_value);

        super::super::manager::HEAP_STATS.record_allocation(layout.size());

        #[cfg(feature = "heap-track")]
        crate::arch::run_without_interrupts(|| {
            if allocator.allocated_ptrs.lock().insert(data_ptr as usize).is_err() {
                allocator.tracking_overflowed.store(true, Ordering::Relaxed);
            }
        });

        if super::super::manager::HEAP_ZERO_ON_ALLOC.load(Ordering::Relaxed) {
            ptr::write_bytes(data_ptr, 0, layout.size());
        }

        data_ptr
    }
}

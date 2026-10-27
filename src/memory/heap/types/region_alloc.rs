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

//! Where a block comes from and goes back to, between the heap's two regions
//! (regions.rs), and how much of both is free. A small block is served by
//! its size class (size_class.rs) and never walks a region's list.

use super::allocator::SecureHeapAllocator;
use super::size_class::class_of;
use core::alloc::{GlobalAlloc, Layout};

impl SecureHeapAllocator {
    /// Bytes not handed out, over both regions.
    pub fn free_bytes(&self) -> usize {
        crate::arch::run_without_interrupts(|| {
            let first = if self.is_initialized() { self.inner.lock().free() } else { 0 };
            let second =
                if self.second_extent().is_some() { self.second.heap.lock().free() } else { 0 };
            first + second + self.small.lock().free_bytes()
        })
    }

    /// A small block from its size class; any other from the regions.
    pub(super) unsafe fn raw_alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            match class_of(layout.size(), layout.align()) {
                Some(class) => self.small_alloc(class),
                None => self.list_alloc(layout),
            }
        }
    }

    /// A block from the bootstrap heap, or from the second region when the
    /// first has no room.
    pub(super) unsafe fn list_alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let first = self.inner.alloc(layout);
            if first.is_null() && self.second_extent().is_some() {
                self.second.heap.alloc(layout)
            } else {
                first
            }
        }
    }

    /// Back to the size class its layout chose, or to the region that handed
    /// the block out. Slabs are never given back to a region: the classes
    /// keep the most the kernel's small blocks ever held.
    pub(super) unsafe fn raw_dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if let Some(class) = class_of(layout.size(), layout.align()) {
                return self.small_dealloc(class, ptr);
            }
            if self.in_second(ptr) {
                self.second.heap.dealloc(ptr, layout)
            } else {
                self.inner.dealloc(ptr, layout)
            }
        }
    }
}

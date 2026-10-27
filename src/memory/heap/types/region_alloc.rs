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
//! (regions.rs), and how much of both is free.

use super::allocator::SecureHeapAllocator;
use super::bins::{class_of, Taken};
use core::alloc::Layout;

impl SecureHeapAllocator {
    /// Bytes not handed out, over both regions.
    pub fn free_bytes(&self) -> usize {
        crate::arch::run_without_interrupts(|| {
            let first = if self.is_initialized() { self.inner.lock().free() } else { 0 };
            let second =
                if self.second_extent().is_some() { self.second.heap.lock().free() } else { 0 };
            first + second
        })
    }

    /// A kept block of the request's class, else a block from the regions.
    pub(super) unsafe fn raw_alloc(&self, layout: Layout) -> *mut u8 {
        let Some((class, made)) = class_of(layout) else {
            return unsafe { self.from_regions(layout) };
        };
        match unsafe { self.second.front.pop(class, |at| self.in_heap(at)) } {
            Taken::Block(block) => block,
            Taken::Empty => unsafe { self.from_regions(made) },
            Taken::Broken => {
                crate::sys::serial::println(b"[HEAP-GUARD] a kept block's link was overwritten");
                unsafe { self.from_regions(made) }
            }
        }
    }

    /// A small block back to its class, else to the region that handed it out.
    pub(super) unsafe fn raw_dealloc(&self, ptr: *mut u8, layout: Layout) {
        match class_of(layout) {
            Some((class, _)) if unsafe { self.second.front.push(class, ptr) } => {}
            Some((_, made)) => unsafe { self.to_regions(ptr, made) },
            None => unsafe { self.to_regions(ptr, layout) },
        }
    }
}

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

//! The heap's two regions: the bootstrap array it starts on, and the second
//! region boot adds (manager/init.rs). A block is handed out by the first
//! that has room and goes back to the one its address lies in.

use super::allocator::SecureHeapAllocator;
use core::sync::atomic::{AtomicUsize, Ordering};
use linked_list_allocator::LockedHeap;

pub(crate) struct SecondRegion {
    pub(super) heap: LockedHeap,
    start: AtomicUsize,
    size: AtomicUsize,
}

impl SecondRegion {
    pub(super) const fn new() -> Self {
        Self { heap: LockedHeap::empty(), start: AtomicUsize::new(0), size: AtomicUsize::new(0) }
    }
}

impl SecureHeapAllocator {
    /// Give the heap its second region. Once only: a region added again would
    /// orphan every block the first one holds.
    ///
    /// # Safety
    /// `[start, start + size)` is mapped writable, and nothing else uses it.
    pub unsafe fn add_region(&self, start: *mut u8, size: usize) -> bool {
        if self.second.size.load(Ordering::Acquire) != 0 || size == 0 {
            return false;
        }
        unsafe {
            crate::arch::run_without_interrupts(|| self.second.heap.lock().init(start, size));
        }
        self.second.start.store(start as usize, Ordering::Release);
        self.second.size.store(size, Ordering::Release);
        true
    }

    /// Base and length of the second region, once boot has added it; the
    /// shutdown wipe erases it beside the first.
    pub fn second_extent(&self) -> Option<(*mut u8, usize)> {
        let size = self.second.size.load(Ordering::Acquire);
        if size == 0 {
            return None;
        }
        Some((self.second.start.load(Ordering::Acquire) as *mut u8, size))
    }
}

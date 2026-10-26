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

use super::super::constants::CANARY_VALUE;
use super::regions::SecondRegion;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
#[cfg(feature = "heap-track")]
use heapless::index_set::FnvIndexSet;
use linked_list_allocator::LockedHeap;
#[cfg(feature = "heap-track")]
use spin::Mutex;

#[cfg(feature = "heap-track")]
pub(crate) const TRACK_CAPACITY: usize = 8192;

pub struct SecureHeapAllocator {
    pub inner: LockedHeap,
    #[cfg(feature = "heap-track")]
    pub allocated_ptrs: Mutex<FnvIndexSet<usize, TRACK_CAPACITY>>,
    #[cfg(feature = "heap-track")]
    pub tracking_overflowed: AtomicBool,
    pub canary_value: u64,
    pub initialized: AtomicBool,
    pub heap_size: AtomicUsize,
    // Where the heap actually lives. The shutdown wipe used to erase the
    // KHEAP_BASE window from the layout constants, which the bootstrap heap
    // never occupies, so it wiped an unmapped range and left the real heap
    // intact. Recording the base at init is what lets the wipe find it.
    pub(super) heap_start: AtomicUsize,
    // The second region (types/regions.rs), empty until boot adds it.
    pub(super) second: SecondRegion,
}

impl SecureHeapAllocator {
    pub const fn new() -> Self {
        Self {
            inner: LockedHeap::empty(),
            #[cfg(feature = "heap-track")]
            allocated_ptrs: Mutex::new(FnvIndexSet::new()),
            #[cfg(feature = "heap-track")]
            tracking_overflowed: AtomicBool::new(false),
            canary_value: CANARY_VALUE,
            initialized: AtomicBool::new(false),
            heap_size: AtomicUsize::new(0),
            heap_start: AtomicUsize::new(0),
            second: SecondRegion::new(),
        }
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    pub unsafe fn init(&self, heap_start: *mut u8, heap_size: usize) {
        unsafe {
            self.inner.lock().init(heap_start, heap_size);
            self.heap_start.store(heap_start as usize, Ordering::Release);
            self.heap_size.store(heap_size, Ordering::Release);
            self.initialized.store(true, Ordering::Release);
        }
    }

    #[inline]
    pub fn get_heap_size(&self) -> usize {
        self.heap_size.load(Ordering::Acquire)
    }
}

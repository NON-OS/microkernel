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

//! The regions' own hole lists, behind the bins (region_alloc.rs). The
//! bootstrap region is tried first; once it fails a size, it is passed over
//! for that size and larger until a block goes back to it, so a full first
//! region's list is not walked to no end on every allocation.

use super::allocator::SecureHeapAllocator;
use super::bins::ALIGN;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::Ordering;

impl SecureHeapAllocator {
    pub(super) unsafe fn from_regions(&self, layout: Layout) -> *mut u8 {
        let short = &self.second.front.first_short;
        let second = self.second_extent().is_some();
        let pass =
            second && layout.align() <= ALIGN && layout.size() >= short.load(Ordering::Relaxed);
        let first = if pass { core::ptr::null_mut() } else { unsafe { self.inner.alloc(layout) } };
        if !first.is_null() || !second {
            return first;
        }
        if !pass && layout.align() <= ALIGN {
            short.fetch_min(layout.size(), Ordering::Relaxed);
        }
        unsafe { self.second.heap.alloc(layout) }
    }

    pub(super) unsafe fn to_regions(&self, ptr: *mut u8, layout: Layout) {
        if self.in_second(ptr) {
            unsafe { self.second.heap.dealloc(ptr, layout) }
        } else {
            unsafe { self.inner.dealloc(ptr, layout) };
            self.second.front.first_short.store(usize::MAX, Ordering::Relaxed);
        }
    }
}

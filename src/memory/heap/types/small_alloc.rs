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
//! A small block from its size class (size_class.rs), the class's list
//! filled a slab at a time from the first-fit lists when it runs dry.

use super::allocator::SecureHeapAllocator;
use super::size_class::{SLAB_ALIGN, SLAB_BYTES};
use core::alloc::Layout;
use core::ptr::null_mut;

impl SecureHeapAllocator {
    /// A block of `class`. The class lock is never held across the
    /// first-fit lists: a slab is taken first, and then cut.
    pub(super) unsafe fn small_alloc(&self, class: usize) -> *mut u8 {
        unsafe {
            if let Some(block) = self.small.lock().pop(class) {
                return block;
            }
            let Ok(slab_layout) = Layout::from_size_align(SLAB_BYTES, SLAB_ALIGN) else {
                return null_mut();
            };
            let slab = self.list_alloc(slab_layout);
            if slab.is_null() {
                return null_mut();
            }
            self.small.lock().carve(class, slab)
        }
    }

    /// `block` back on the list of `class`, which its layout chose.
    pub(super) unsafe fn small_dealloc(&self, class: usize, block: *mut u8) {
        unsafe { self.small.lock().push(class, block) }
    }
}

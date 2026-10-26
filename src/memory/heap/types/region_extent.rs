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

//! Where each of the heap's regions lies: for a free to find its region,
//! and for the wipes that erase the heap.

use super::allocator::SecureHeapAllocator;
use core::sync::atomic::Ordering;

impl SecureHeapAllocator {
    /// Base and length of the memory this heap allocates from, once it has
    /// been initialized. The shutdown wipe uses this to erase the heap the
    /// kernel actually ran on rather than a range from the layout constants.
    pub fn extent(&self) -> Option<(*mut u8, usize)> {
        if !self.is_initialized() {
            return None;
        }
        let start = self.heap_start.load(Ordering::Acquire);
        let size = self.heap_size.load(Ordering::Acquire);
        if start == 0 || size == 0 {
            return None;
        }
        Some((start as *mut u8, size))
    }

    /// Both regions' extents, for the wipes that erase the heap.
    pub fn extents(&self) -> [Option<(*mut u8, usize)>; 2] {
        [self.extent(), self.second_extent()]
    }

    /// Whether a block starting at `ptr` was handed out by the second region.
    pub(super) fn in_second(&self, ptr: *mut u8) -> bool {
        self.second_extent().is_some_and(|(s, n)| (ptr as usize).wrapping_sub(s as usize) < n)
    }
}

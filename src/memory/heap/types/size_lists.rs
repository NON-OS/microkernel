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
//! Each size class's free blocks (size_class.rs says which class a block
//! is): a list threaded through the blocks themselves, and a count.

use super::size_class::CLASSES;

/// Every class's free blocks, each holding the address of the next in its
/// first word; 0 ends a list. `free` counts each list, for the free bytes
/// the heap reports.
pub(crate) struct SizeClasses {
    heads: [usize; CLASSES],
    pub(super) free: [usize; CLASSES],
}

impl SizeClasses {
    pub(crate) const fn new() -> Self {
        Self { heads: [0; CLASSES], free: [0; CLASSES] }
    }

    /// A free block of `class`, or None when its list is empty.
    ///
    /// # Safety
    /// Every block on the list was pushed by `push` or `carve` and is not
    /// in use.
    pub(crate) unsafe fn pop(&mut self, class: usize) -> Option<*mut u8> {
        let head = self.heads[class];
        if head == 0 {
            return None;
        }
        // SAFETY: a free block's first word is the next block's address.
        self.heads[class] = unsafe { core::ptr::read(head as *const usize) };
        self.free[class] -= 1;
        Some(head as *mut u8)
    }

    /// `block`, a block of `class` its caller is done with, back on its list.
    ///
    /// # Safety
    /// `block` came from `pop` or `carve` for this `class` and is not in use.
    pub(crate) unsafe fn push(&mut self, class: usize, block: *mut u8) {
        // SAFETY: the block is the caller's to give back and at least a word.
        unsafe { core::ptr::write(block as *mut usize, self.heads[class]) };
        self.heads[class] = block as usize;
        self.free[class] += 1;
    }
}

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

//! An allocator that clears a block before returning it to the free list.
//!
//! Same guarantee `nonos_userland_libc` gives, for capsules linking this crate
//! instead: exposure should not depend on which one a capsule picked. Copies
//! left by a clone or a Vec growth reach dealloc as ordinary bytes, and a
//! plain free list only overwrites its own links at the head of the block.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

use linked_list_allocator::LockedHeap;

pub struct ZeroOnFree {
    inner: LockedHeap,
}

impl ZeroOnFree {
    pub const fn empty() -> Self {
        Self { inner: LockedHeap::empty() }
    }

    /// Hand the backing region to the inner allocator.
    ///
    /// # Safety
    /// `base` must point at `bytes` of memory that is writable, unaliased, and
    /// lives as long as the allocator, which is the whole run.
    pub unsafe fn init(&self, base: *mut u8, bytes: usize) {
        // SAFETY: the caller guarantees the region; this only forwards it.
        unsafe { self.inner.lock().init(base, bytes) }
    }
}

// SAFETY: every method forwards to `LockedHeap`, which is itself a sound
// `GlobalAlloc`. `dealloc` writes only within the block the caller is giving
// up, using the layout it was allocated with, and does so before the block
// re-enters the free list, so no live allocation is touched.
unsafe impl GlobalAlloc for ZeroOnFree {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged.
        unsafe { self.inner.alloc(layout) }
    }

    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        if !block.is_null() {
            // SAFETY: the caller states the block is `layout.size()` bytes it
            // owns and is releasing, so every byte is in bounds and unaliased.
            unsafe { wipe(block, layout.size()) };
        }
        // SAFETY: forwarded unchanged, after the block was cleared in place.
        unsafe { self.inner.dealloc(block, layout) }
    }
}

/// Zero over the `len` bytes at `block`, each store volatile because the
/// block is dead as far as the compiler is concerned: a plain `write_bytes`
/// is a store to memory nothing reads again, which it is free to drop. Bytes
/// up to the first 8-byte boundary and after the last whole word, words in
/// between, so every byte is written and a large frame buffer takes an
/// eighth of the stores a byte loop does.
///
/// # Safety
/// `block` must be valid for writes of `len` bytes, none of them part of a
/// live allocation.
pub(crate) unsafe fn wipe(block: *mut u8, len: usize) {
    let head = block.align_offset(8).min(len);
    let words = (len - head) / 8;
    let tail = head + words * 8;
    for i in (0..head).chain(tail..len) {
        // SAFETY: `i < len`, inside the block the caller vouches for.
        unsafe { ptr::write_volatile(block.add(i), 0) };
    }
    // SAFETY: `block + head` is 8-aligned (align_offset), and its `words`
    // words end at `block + tail <= block + len`, inside the block.
    let at = unsafe { block.add(head) }.cast::<u64>();
    for i in 0..words {
        // SAFETY: `i < words`, so the word is inside the block and aligned.
        unsafe { ptr::write_volatile(at.add(i), 0) };
    }
}

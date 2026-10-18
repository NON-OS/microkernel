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

//! A pass-through allocator that, for the one block a test names, keeps
//! what the block held at the moment it was freed.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Mutex;

const KEEP: usize = 64;

struct Watching;

static WATCHED: AtomicUsize = AtomicUsize::new(0);
static SEEN: spin::Mutex<([u8; KEEP], usize)> = spin::Mutex::new(([0; KEEP], 0));
static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

unsafe impl GlobalAlloc for Watching {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        if WATCHED.compare_exchange(p as usize, 0, SeqCst, SeqCst).is_ok() {
            let n = layout.size().min(KEEP);
            let mut seen = SEEN.lock();
            // SAFETY: `p` is a live block of `layout.size()` bytes until freed below.
            unsafe { core::ptr::copy_nonoverlapping(p, seen.0.as_mut_ptr(), n) };
            seen.1 = n;
        }
        unsafe { System.dealloc(p, layout) }
    }
}

#[global_allocator]
static WATCHING: Watching = Watching;

/// What the block at `p` held when it was freed during `f`; empty when it
/// was not freed.
pub(super) fn freed_bytes(p: *const u8, f: impl FnOnce()) -> Vec<u8> {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    SEEN.lock().1 = 0;
    WATCHED.store(p as usize, SeqCst);
    f();
    WATCHED.store(0, SeqCst);
    let seen = SEEN.lock();
    seen.0[..seen.1].to_vec()
}

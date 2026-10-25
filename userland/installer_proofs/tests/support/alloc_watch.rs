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

//! The largest single allocation a call makes on this thread, and the bytes it leaves held.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

struct Watch;

#[global_allocator]
static WATCH: Watch = Watch;

thread_local! {
    static LARGEST: Cell<usize> = const { Cell::new(0) };
    static HELD: Cell<isize> = const { Cell::new(0) };
}

// SAFETY: every call is passed straight to System; only sizes are noted.
unsafe impl GlobalAlloc for Watch {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = LARGEST.try_with(|m| m.set(m.get().max(layout.size())));
        let _ = HELD.try_with(|h| h.set(h.get() + layout.size() as isize));
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _ = HELD.try_with(|h| h.set(h.get() - layout.size() as isize));
        System.dealloc(ptr, layout)
    }
}

/// Runs f and returns its value with the largest allocation it asked for.
pub fn largest_during<T>(f: impl FnOnce() -> T) -> (T, usize) {
    LARGEST.with(|m| m.set(0));
    let out = f();
    (out, LARGEST.with(Cell::get))
}

/// Runs f and returns its value with the bytes it left allocated on this
/// thread: what it returns, and anything it failed to free.
pub fn held_after<T>(f: impl FnOnce() -> T) -> (T, isize) {
    let before = HELD.with(Cell::get);
    let out = f();
    (out, HELD.with(Cell::get) - before)
}

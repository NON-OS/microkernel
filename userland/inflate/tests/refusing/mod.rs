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

//! An allocator that refuses any block above LIMIT, so a test can make the
//! refusal on purpose. It is the whole binary's, as a global allocator is.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub static LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);

struct Refusing;

unsafe impl GlobalAlloc for Refusing {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        if l.size() > LIMIT.load(Ordering::Relaxed) {
            return core::ptr::null_mut();
        }
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}

#[global_allocator]
static A: Refusing = Refusing;

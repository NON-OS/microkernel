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

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

/* Per thread, so tests running side by side do not count each other. */
thread_local! {
    static LIVE: Cell<usize> = const { Cell::new(0) };
    static PEAK: Cell<usize> = const { Cell::new(0) };
    static COUNT: Cell<usize> = const { Cell::new(0) };
}

pub struct Counting;

fn grew(by: usize) {
    let live = LIVE.with(|l| l.get()) + by;
    LIVE.with(|l| l.set(live));
    PEAK.with(|p| p.set(p.get().max(live)));
    COUNT.with(|c| c.set(c.get() + 1));
}

fn shrank(by: usize) {
    LIVE.with(|l| l.set(l.get().saturating_sub(by)));
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        grew(l.size());
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        shrank(l.size());
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new: usize) -> *mut u8 {
        shrank(l.size());
        grew(new);
        unsafe { System.realloc(p, l, new) }
    }
}

/* What a measured call did to this thread's heap: the most bytes it held
live above its start, the bytes still held when it returned, and how many
allocations it made. */
pub struct Heap {
    pub peak: usize,
    pub held: usize,
    pub allocs: usize,
}

pub fn measure<R>(f: impl FnOnce() -> R) -> (R, Heap) {
    let (live, count) = (LIVE.with(|l| l.get()), COUNT.with(|c| c.get()));
    PEAK.with(|p| p.set(live));
    let r = f();
    let peak = PEAK.with(|p| p.get()) - live;
    let held = LIVE.with(|l| l.get()).saturating_sub(live);
    (r, Heap { peak, held, allocs: COUNT.with(|c| c.get()) - count })
}

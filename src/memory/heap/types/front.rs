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

//! The bins' lists (bins.rs), and the first region's hint (region_alloc.rs).

use super::bins::{Taken, ALIGN, CLASSES, KEEP};
use core::sync::atomic::AtomicUsize;
use spin::Mutex;

#[derive(Clone, Copy)]
struct Bin {
    head: usize,
    count: usize,
}

pub(crate) struct Front {
    bins: Mutex<[Bin; CLASSES.len()]>,
    /// The least size the first region failed since a block last went back.
    pub(crate) first_short: AtomicUsize,
}

impl Front {
    pub(crate) const fn new() -> Self {
        let (bins, short) = ([Bin { head: 0, count: 0 }; CLASSES.len()], usize::MAX);
        Self { bins: Mutex::new(bins), first_short: AtomicUsize::new(short) }
    }

    /// A kept block of `class`; `heap` says whether an address is the heap's.
    /// Safety: every block pushed to `class` was 16 bytes or more, the heap's.
    pub(crate) unsafe fn pop(&self, class: usize, heap: impl Fn(usize) -> bool) -> Taken {
        let mut bins = self.bins.lock();
        let bin = &mut bins[class];
        if bin.head == 0 {
            return Taken::Empty;
        }
        let block = bin.head as *mut u8;
        let next = unsafe { (block.add(8) as *const usize).read() };
        if next != 0 && (!heap(next) || next % ALIGN != 0) {
            *bin = Bin { head: 0, count: 0 };
            return Taken::Broken;
        }
        *bin = Bin { head: next, count: bin.count - 1 };
        Taken::Block(block)
    }

    /// Keep `block` of `class` unless the class holds KEEP bytes; true when
    /// kept. Its first word, the header's magic, is cleared, so a second free
    /// of it is refused (dealloc_impl.rs). Safety: a freed block of `class`.
    pub(crate) unsafe fn push(&self, class: usize, block: *mut u8) -> bool {
        let mut bins = self.bins.lock();
        let bin = &mut bins[class];
        if (bin.count + 1) * CLASSES[class] > KEEP {
            return false;
        }
        unsafe {
            (block as *mut usize).write(0);
            (block.add(8) as *mut usize).write(bin.head);
        }
        *bin = Bin { head: block as usize, count: bin.count + 1 };
        true
    }
}

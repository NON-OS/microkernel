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

//! What the heap holds: bytes in live blocks now, the most it has held, and
//! the region it was given. Two atomics the allocator bumps on each alloc and
//! free, so a capsule can say how close an operation came to its ceiling.

use core::sync::atomic::{AtomicUsize, Ordering};

static USED: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static SIZE: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stats {
    pub used: usize,
    pub peak: usize,
    pub size: usize,
}

pub fn stats() -> Stats {
    let o = Ordering::Relaxed;
    Stats { used: USED.load(o), peak: PEAK.load(o), size: SIZE.load(o) }
}

/// Start a new peak from what is held now, so one operation's peak can be
/// read on its own.
pub fn reset_peak() {
    PEAK.store(USED.load(Ordering::Relaxed), Ordering::Relaxed);
}

pub(super) fn sized(bytes: usize) {
    SIZE.store(bytes, Ordering::Relaxed);
}

/// `block` as the allocator gave it, counted when it is one.
pub(super) fn took(block: *mut u8, bytes: usize) -> *mut u8 {
    if !block.is_null() {
        let now = USED.fetch_add(bytes, Ordering::Relaxed) + bytes;
        PEAK.fetch_max(now, Ordering::Relaxed);
    }
    block
}

pub(super) fn gave(bytes: usize) {
    USED.fetch_sub(bytes, Ordering::Relaxed);
}

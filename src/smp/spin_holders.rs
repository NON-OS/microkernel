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

//! Who took each lock last.
//!
//! Only the taking is written down, never the release: a lock somebody waits
//! on is held, and its last taker is its holder. That keeps the cost on every
//! take to three plain stores, with no read of the interrupt controller.

use core::panic::Location;
use core::sync::atomic::{AtomicUsize, Ordering::*};

use super::spin_watch::{slot_of, SLOTS};

struct Taker {
    lock: AtomicUsize,
    site: AtomicUsize,
    sp: AtomicUsize,
}

static TAKERS: [Taker; SLOTS] = [const {
    Taker { lock: AtomicUsize::new(0), site: AtomicUsize::new(0), sp: AtomicUsize::new(0) }
}; SLOTS];

/// An address in the caller's frame, to tell one stack from another.
#[inline(always)]
pub(super) fn stack_mark() -> usize {
    let mark = 0u8;
    core::ptr::addr_of!(mark) as usize
}

/// `lock` was just taken at `site` by a frame at `sp`.
#[inline]
pub(super) fn note(lock: usize, site: &'static Location<'static>, sp: usize) {
    let t = &TAKERS[slot_of(lock)];
    t.lock.store(0, Relaxed);
    t.site.store(site as *const _ as usize, Relaxed);
    t.sp.store(sp, Relaxed);
    t.lock.store(lock, Release);
}

/// The site and frame that took `lock` last, if no other lock has used its
/// slot since.
pub(super) fn taker(lock: usize) -> Option<(&'static Location<'static>, usize)> {
    let t = &TAKERS[slot_of(lock)];
    let (site, sp) = (t.site.load(Acquire), t.sp.load(Acquire));
    if t.lock.load(Acquire) != lock || site == 0 {
        return None;
    }
    // SAFETY: only `note` writes a site, and only from a `&'static Location`.
    Some((unsafe { &*(site as *const Location<'static>) }, sp))
}

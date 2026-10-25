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
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::smp::lock_responsive;

use super::register_race::ranges_meet;

/*
 * Ranges an mmap or munmap is changing right now, by address space (the
 * tables' CR3, which every thread of a process shares): an mmap whose
 * rollback frees the pages it mapped, a munmap from before it reads the
 * surfaces until it has settled them. A register of any of those pages
 * meanwhile is refused (`register_race`).
 */
static BUSY: Mutex<Vec<(u64, u64, u64, u64)>> = Mutex::new(Vec::new());
static NEXT: AtomicU64 = AtomicU64::new(1);

/// Marks the range busy until dropped.
pub struct Busy {
    ticket: u64,
}

pub fn begin(addr: u64, len: u64) -> Busy {
    let ticket = NEXT.fetch_add(1, Ordering::Relaxed);
    let entry = (current_space(), addr, addr.saturating_add(len), ticket);
    lock_responsive(&BUSY).push(entry);
    Busy { ticket }
}

impl Drop for Busy {
    fn drop(&mut self) {
        lock_responsive(&BUSY).retain(|e| e.3 != self.ticket);
    }
}

/// True when the caller's address space has a busy range meeting this one.
pub(in crate::kernel_core::surface_registry) fn busy_over(addr: u64, len: u64) -> bool {
    let (space, end) = (current_space(), addr.saturating_add(len));
    lock_responsive(&BUSY).iter().any(|e| e.0 == space && ranges_meet(addr, end, e.1, e.2))
}

fn current_space() -> u64 {
    crate::process::current_process().map_or(0, |p| p.cr3.load(Ordering::Acquire))
}

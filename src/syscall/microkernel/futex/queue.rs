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

//! The futex wait queue, keyed on (thread group, address), and its limits.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

pub(super) const SAFETY_MS: u64 = 20;
/// The longest one timed wait sleeps before its caller looks again.
pub(super) const MAX_TIMED_MS: u64 = 60_000;

pub(super) static FUTEX_QUEUE: spin::Mutex<BTreeMap<(u32, u64), Vec<u32>>> =
    spin::Mutex::new(BTreeMap::new());

pub(super) fn tgid_of(pid: u32) -> u32 {
    crate::process::nonos_core::PROCESS_TABLE
        .find_by_pid(pid)
        .map(|pcb| pcb.tgid.load(Ordering::Acquire))
        .unwrap_or(pid)
}

/// A torn-down thread taken off every futex it waits on: it never runs its
/// own leave, and a wake spent on it was lost to a live waiter behind it
/// (L77e). Called once it is a zombie.
pub fn forget(pid: u32) {
    FUTEX_QUEUE.lock().retain(|_, waiting| {
        waiting.retain(|&p| p != pid);
        !waiting.is_empty()
    });
}

/// Killed or gone: asked of a waiter directly, since a sleeper killed while
/// it slept is never woken to leave by itself.
pub(super) fn ended(pid: u32) -> bool {
    use crate::process::core::ProcessState;
    crate::process::nonos_core::PROCESS_TABLE.find_by_pid(pid).is_none_or(|pcb| {
        matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    })
}

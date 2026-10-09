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

//! Wakes an interrupt had to leave, kept until a tick can make them.

use super::retry::try_wake_now;
use core::sync::atomic::Ordering;

/*
 * Wakes an interrupt could not do on the spot. A slot holds a pid or 0; a
 * full list falls back to the caller's own retry (false), as before.
 */
const DEFERRED_SLOTS: usize = 64;
static DEFERRED: [core::sync::atomic::AtomicU32; DEFERRED_SLOTS] =
    [const { core::sync::atomic::AtomicU32::new(0) }; DEFERRED_SLOTS];

pub(super) fn defer(pid: u32) -> bool {
    for slot in DEFERRED.iter() {
        let held = slot.load(Ordering::Acquire);
        if held == pid {
            return true;
        }
        if held == 0 && slot.compare_exchange(0, pid, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            return true;
        }
    }
    false
}

/// Run the wakes interrupts had to leave, from the timer tick of every CPU. A
/// state still held elsewhere stays for the next tick.
pub fn drain_deferred() {
    for slot in DEFERRED.iter() {
        let pid = slot.swap(0, Ordering::AcqRel);
        if pid != 0 && !try_wake_now(pid) {
            let _ = slot.compare_exchange(0, pid, Ordering::AcqRel, Ordering::Acquire);
        }
    }
}

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

//! The FACS Global Lock word (ACPI 6.5, 5.2.10.1): firmware and the
//! capsule share it in memory, so it is compare-and-swapped where it lies,
//! through a mapping of the FACS, never in the kernel's copy.

use super::state::CapsuleHost;
use crate::sys::acpi_event::lock_told;
use core::sync::atomic::{AtomicU32, Ordering};

/// The Global Lock's offset in the FACS (table 5.14).
const LOCK_OFFSET: u64 = 16;
/// The word's owned bit.
const OWNED: u32 = 2;

/// `va` is the FACS's address in this capsule. Returns the word found,
/// which equals `old` when `new` was stored.
pub fn swap(va: u64, old: u32, new: u32) -> u32 {
    // SAFETY: `va` maps the FACS for this capsule's life; the lock word is
    // four-byte aligned (the FACS is 64-byte aligned, 5.2.10), and firmware
    // changes it only with locked operations too.
    let word = unsafe { &*((va + LOCK_OFFSET) as *const AtomicU32) };
    match word.compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(found) | Err(found) => found,
    }
}

/// The interpreter's swap, noting when the capsule takes the lock (Owned
/// set) and gives it (Owned cleared); marking Pending changes neither.
/// The kernel hears of a take after the swap and of a release before it,
/// so an exit between the two leaves the lock held, and the kernel never
/// releases a lock firmware holds (F-107).
pub(super) fn word(h: &mut CapsuleHost, old: u32, new: u32) -> Option<u32> {
    let va = h.facs?;
    let (had, has) = (old & OWNED != 0, new & OWNED != 0);
    if had && !has {
        lock_told(false);
    }
    let found = swap(va, old, new);
    if found == old && had != has {
        h.lock_held = has;
        if has {
            lock_told(true);
        }
    }
    Some(found)
}

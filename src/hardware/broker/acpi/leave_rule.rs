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

//! The two facts the ACPI capsule's exit needs that only it can give: who
//! holds the Global Lock, and a power-button press the kernel's poll read
//! while no capsule served the button. Atomics only, pure: the poll sets
//! the press from the tick, `kernel_proofs` mounts this file as it is.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static LOCK_HELD_BY: AtomicU32 = AtomicU32::new(0);
static GAP_PRESS: AtomicBool = AtomicBool::new(false);

/// The capsule took (`held`) or is about to give back the Global Lock: told
/// after a take and before a release, so a gap leaves it marked free while
/// held, never held while firmware has it.
pub fn lock_held(pid: u32, held: bool) {
    if held {
        LOCK_HELD_BY.store(pid, Ordering::Release);
    } else {
        let _ = LOCK_HELD_BY.compare_exchange(pid, 0, Ordering::AcqRel, Ordering::Acquire);
    }
}

/// `pid` is leaving: true when it held the lock, which is then forgotten.
pub fn lock_at_exit(pid: u32) -> bool {
    pid != 0 && LOCK_HELD_BY.compare_exchange(pid, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()
}

/// From the tick: a press read while no capsule serves the button.
pub fn press_in_gap() {
    GAP_PRESS.store(true, Ordering::Release);
}

/// The press, once: true when one waits to be acted on.
pub fn take_gap_press() -> bool {
    GAP_PRESS.swap(false, Ordering::AcqRel)
}

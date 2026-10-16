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

//! Who reads the fixed power button. The kernel polls PWRBTN_STS until the
//! ACPI capsule says its SCI serves the button; two readers of one
//! write-one-to-clear bit would each lose the presses the other cleared.
//! The poll stands down only while the pid that said so is the capsule's
//! live pid, so a capsule that dies hands the button back to the kernel.
//! Atomics only: the poll runs from the timer tick.

use core::sync::atomic::{AtomicU32, Ordering};

static TAKEN_BY: AtomicU32 = AtomicU32::new(0);

pub fn take(pid: u32) {
    TAKEN_BY.store(pid, Ordering::Release);
}

/// Whether the kernel's poll leaves the button alone, given the acpi
/// capsule's live pid (0 when none is alive).
pub fn stands_down(alive: u32) -> bool {
    rule(TAKEN_BY.load(Ordering::Acquire), alive)
}

/// The rule, pure: a capsule that took the button and is still the live one.
pub fn rule(taken_by: u32, alive: u32) -> bool {
    taken_by != 0 && taken_by == alive
}

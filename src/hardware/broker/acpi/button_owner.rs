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
//! live pid, and that pid's exit gives the button straight back.
//! Atomics only: the poll runs from the timer tick.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static TAKEN_BY: AtomicU32 = AtomicU32::new(0);
/// A capsule has served the button since boot: its policy, a press is the
/// wiped power-off (F-88), holds while it restarts.
static SERVED: AtomicBool = AtomicBool::new(false);

/// What a press the kernel's own poll reads is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    /// The Power key, to the desktop: no capsule has served the button.
    Desktop,
    /// The wiped power-off, as the capsule would have run it.
    PowerOff,
}

/// True when a capsule served the button before: a restart.
pub fn take(pid: u32) -> bool {
    TAKEN_BY.store(pid, Ordering::Release);
    SERVED.swap(true, Ordering::AcqRel)
}

/// `pid` is leaving; true when it held the button, which is the kernel's
/// again from this moment.
pub fn give_back(pid: u32) -> bool {
    pid != 0 && TAKEN_BY.compare_exchange(pid, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()
}

/// Whether the kernel's poll leaves the button alone, given the acpi
/// capsule's live pid (0 when none is alive).
pub fn stands_down(alive: u32) -> bool {
    rule(TAKEN_BY.load(Ordering::Acquire), alive)
}

pub fn kernel_press() -> Press {
    press_rule(SERVED.load(Ordering::Acquire))
}

/// The rule, pure: a capsule that took the button and is still the live one.
pub fn rule(taken_by: u32, alive: u32) -> bool {
    taken_by != 0 && taken_by == alive
}

/// Pure: once a capsule served the button, a press while none does is
/// still the power-off it would have run.
pub fn press_rule(served: bool) -> Press {
    if served {
        Press::PowerOff
    } else {
        Press::Desktop
    }
}

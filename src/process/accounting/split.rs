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

//! Where a tick landed: in the process's own code or in the kernel on its
//! behalf. The timer trampoline knows from the interrupted frame's CS and
//! records it here before the tick is charged, so each process's ticks
//! split into user and kernel time without any per-instruction cost.

use core::sync::atomic::{AtomicBool, Ordering};

use super::counters::bump;
use super::kind::Kind;
use super::totals::{bump_total, Total};

static TICK_FROM_USER: AtomicBool = AtomicBool::new(false);

/// Called by the timer trampoline with the interrupted frame's privilege.
#[inline]
pub fn set_tick_origin(from_user: bool) {
    TICK_FROM_USER.store(from_user, Ordering::Relaxed);
}

/// Called by the scheduler tick beside the run-tick charge, never for an
/// idle tick: the halt is neither user nor kernel work.
#[inline]
pub fn tick_charge(pid: u32) {
    if TICK_FROM_USER.load(Ordering::Relaxed) {
        bump(pid, Kind::UserTick);
        bump_total(Total::UserTicks);
    } else {
        bump_total(Total::KernelTicks);
    }
}

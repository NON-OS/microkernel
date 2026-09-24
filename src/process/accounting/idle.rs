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

//! Idle time as its own account.
//!
//! When nothing is runnable the scheduler halts the processor, and the
//! timer tick that wakes it found `CURRENT_PID` still naming whichever
//! process ran last. Every one of those ticks was charged to that process,
//! so an idle desktop showed init at a third of the processor and the load
//! card read ninety percent. The scheduler now marks the halt, and the
//! tick charges this counter instead while the mark is up.

use core::sync::atomic::{AtomicU64, Ordering};

static IDLE_TICKS: AtomicU64 = AtomicU64::new(0);

#[inline]
pub fn idle_enter() {
    crate::smp::percpu::current().accounting_idle.store(true, Ordering::Relaxed);
}

#[inline]
pub fn idle_leave() {
    crate::smp::percpu::current().accounting_idle.store(false, Ordering::Relaxed);
}

#[inline]
pub fn is_idle() -> bool {
    crate::smp::percpu::current().accounting_idle.load(Ordering::Relaxed)
}

/// Called by the timer tick instead of charging a process.
#[inline]
pub fn tick_idle() {
    IDLE_TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn idle_ticks() -> u64 {
    IDLE_TICKS.load(Ordering::Relaxed)
}

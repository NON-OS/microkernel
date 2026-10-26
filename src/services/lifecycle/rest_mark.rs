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

//! A check that no listed spin lock is held where a caller rests (L81b).
//! Kernel code is never preempted, so a thread leaves its CPU only where it
//! rests or yields: a count of listed spin locks held on this CPU that is
//! not 0 at a rest is a lock held across a wait, the defect this unit
//! closes. Said once a boot; a debug build stops on it.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::smp::MAX_CPUS;

static HELD: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(0) }; MAX_CPUS];
static SAID: AtomicBool = AtomicBool::new(false);

/// Kept beside a listed spin lock's guard, and dropped with it.
pub(crate) struct Quiet(usize);

/// Count a listed spin lock held on this CPU until the mark drops.
pub(crate) fn quiet() -> Quiet {
    let cpu = crate::smp::cpu_id().min(MAX_CPUS - 1);
    HELD[cpu].fetch_add(1, Ordering::Relaxed);
    Quiet(cpu)
}

impl Drop for Quiet {
    fn drop(&mut self) {
        HELD[self.0].fetch_sub(1, Ordering::Relaxed);
    }
}

/// Asked by every rest before it yields or sleeps.
pub(super) fn check() {
    let cpu = crate::smp::cpu_id().min(MAX_CPUS - 1);
    let held = HELD[cpu].load(Ordering::Relaxed);
    if held == 0 {
        return;
    }
    debug_assert!(held == 0, "a wait rested holding {held} listed spin locks");
    if !SAID.swap(true, Ordering::Relaxed) {
        crate::log::error!("[LOCK] a wait rested holding {} listed spin locks", held);
    }
}

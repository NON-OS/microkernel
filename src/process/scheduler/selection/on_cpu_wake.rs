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

//! Which CPU to tell when a pid is queued.
//!
//! A pid that a CPU names in `OWNED` or `LEAVING` may be claimed by that CPU
//! only (see `on_cpu`). A process that sleeps in a yield keeps its CPU parked
//! on its stack, halted, with the pid still owned. Waking an arbitrary idle
//! CPU for it is a wake spent: that CPU finds the pid held and halts again,
//! and the owner hears of it at its next timer tick at best. Every IPC hop
//! then costs up to a tick, and a call with a short reply deadline times out.
//!
//! So the wake goes to the owner. A pid only being left is not claimable by
//! anyone until its CPU is off that stack, so nobody is woken for it here;
//! `release_leaving` wakes an idle CPU once the pid becomes claimable. A pid
//! nobody names goes to any idle CPU, as before. The single-CPU image keeps
//! the old path, which has nobody to wake.

use core::sync::atomic::{fence, Ordering};

use super::on_cpu::{this_cpu, LEAVING, OWNED, TRACKED};
use crate::smp::MAX_CPUS;

/// Tell the CPU that can run `pid`, which the caller has just queued.
pub(crate) fn wake_for(pid: u32) {
    if !TRACKED || pid == 0 {
        crate::smp::wake_idle_cpu();
        return;
    }
    // Pairs with the fence in `release_leaving`: either this reads the slot
    // still naming the pid, or that CPU reads the queue already holding it.
    fence(Ordering::SeqCst);
    let me = this_cpu();
    let cpus = crate::smp::cpu_count().min(MAX_CPUS);
    // OWNED before LEAVING, as in `on_cpu::named_by`.
    if let Some(cpu) = (0..cpus).find(|&cpu| OWNED[cpu].load(Ordering::SeqCst) == pid) {
        // This CPU, owning it, is the one queueing it: it is yielding or
        // being preempted and reaches the pick itself.
        if cpu != me {
            crate::smp::send_reschedule_ipi(cpu);
        }
        return;
    }
    if (0..cpus).any(|cpu| LEAVING[cpu].load(Ordering::SeqCst) == pid) {
        return;
    }
    crate::smp::wake_idle_cpu();
}

/// This CPU has stopped naming `left`. If it is queued, it is claimable only
/// from now on, so an idle CPU is told.
pub(super) fn left_claimable(left: u32) {
    fence(Ordering::SeqCst);
    if super::super::dispatch::is_in_run_queue(left) {
        crate::smp::wake_idle_cpu();
    }
}

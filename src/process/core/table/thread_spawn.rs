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

//! Threads, built in two steps.

use core::sync::atomic::Ordering;

use super::super::types::{Pid, Priority, ProcessState};
use super::build_pcb::build_pcb;
use super::types::{allocate_tid, CURRENT_PID, PROCESS_TABLE};

/// Create a schedulable thread inside the current process, running.
pub fn spawn_thread(entry: u64, stack: u64) -> Result<Pid, &'static str> {
    spawn_thread_in(CURRENT_PID.load(Ordering::Relaxed), entry, stack)
}

/// A thread in `parent_pid` rather than in the caller, running.
pub fn spawn_thread_in(parent_pid: Pid, entry: u64, stack: u64) -> Result<Pid, &'static str> {
    let tid = spawn_thread_parked(parent_pid, entry, stack)?;
    admit_thread(tid);
    Ok(tid)
}

/// A thread in `parent_pid`, built but on no run queue.
pub fn spawn_thread_parked(parent_pid: Pid, entry: u64, stack: u64) -> Result<Pid, &'static str> {
    let parent = PROCESS_TABLE.find_by_pid(parent_pid).ok_or("no such process")?;
    let tid = allocate_tid().ok_or("pid space exhausted")?;
    let caps = super::inherit::compute_inherited_caps(tid, parent_pid);
    let pcb = build_pcb(tid, parent_pid, "thread", ProcessState::New, Priority::Normal, 0, caps)?;
    crate::process::address_space::lifecycle::inherit(&pcb, &parent);
    pcb.tgid.store(parent.tgid.load(Ordering::Relaxed), Ordering::Relaxed);
    crate::process::caps::rebind_address_space(&pcb).ok_or("boot session nonce missing")?;
    PROCESS_TABLE.add(pcb);
    crate::kernel_core::process_spawn::allocate_kernel_stack(tid)
        .map_err(|_| "thread kernel stack")?;
    crate::kernel_core::process_spawn::setup_initial_user_context(tid, entry, stack)
        .map_err(|_| "thread user context")?;
    Ok(tid)
}

/// Let a parked thread run. The claim is what makes it runnable, so a
/// second call finds the thread already started and queues nothing.
pub fn admit_thread(tid: Pid) {
    if super::claim::claim_new(tid) {
        crate::sched::add_to_run_queue(tid);
    }
}

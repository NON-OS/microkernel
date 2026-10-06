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

extern crate alloc;

use core::sync::atomic::Ordering;

use super::super::types::{Pid, Priority, ProcessState};
use super::build_pcb::build_pcb;
use super::current_pid::CURRENT_PID;
use super::pid::allocate_tid;
use super::thread_start::start_in_user_half;
use super::types::PROCESS_TABLE;
use crate::syscall::microkernel::errnos::ERRNO_NOMEM;

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
    if !start_in_user_half(entry, stack) {
        return Err("thread start outside the user half");
    }
    let parent = PROCESS_TABLE.find_by_pid(parent_pid).ok_or("no such process")?;
    let tid = allocate_tid().ok_or("pid space exhausted")?;
    /*
     * A thread is its process, not a child of it: it runs in the same address
     * space, so whatever the process may do, any of its threads can already
     * have done for it. It carries the process's own capabilities. Bounded to
     * the ambient set, a std worker lost Network and every IPC call it made to
     * a network service was refused, though its sibling could make the same
     * call with the same memory.
     */
    let caps = parent.caps_bits.load(Ordering::Acquire);
    let pcb = build_pcb(tid, parent_pid, "thread", ProcessState::New, Priority::Normal, 0, caps)?;
    crate::process::address_space::lifecycle::inherit(&pcb, &parent);
    pcb.tgid.store(parent.tgid.load(Ordering::Relaxed), Ordering::Relaxed);
    crate::process::caps::rebind_address_space(&pcb).ok_or("boot session nonce missing")?;
    PROCESS_TABLE.add(pcb);
    /*
     * Where the thread's own IPC calls are answered. A process has
     * `proc.<pid>` from its spawn; a thread had none, so every call it made
     * was sent and its answer lost. It goes with the thread at teardown.
     */
    if crate::ipc::nonos_inbox::register_inbox(&alloc::format!("proc.{}", tid), tid).is_err() {
        return Err(unstarted(tid, "thread reply inbox"));
    }
    /*
     * Published from here on. A step below that fails ends the thread instead
     * of returning past it: nothing claims or reaps a thread that never
     * started, so it stayed New in the table for good, holding its pid and
     * its kernel stack, once per refused MkThreadSpawn.
     */
    if crate::kernel_core::process_spawn::allocate_kernel_stack(tid).is_err() {
        return Err(unstarted(tid, "thread kernel stack"));
    }
    if crate::kernel_core::process_spawn::setup_initial_user_context(tid, entry, stack).is_err() {
        return Err(unstarted(tid, "thread user context"));
    }
    Ok(tid)
}

/// End a thread that was published but cannot run, and pass on why.
fn unstarted(tid: Pid, why: &'static str) -> &'static str {
    crate::process::exit::teardown(tid, ERRNO_NOMEM as i32, false);
    why
}

/// Let a parked thread run. The claim is what makes it runnable, so a
/// second call finds the thread already started and queues nothing.
pub fn admit_thread(tid: Pid) {
    if super::claim::claim_new(tid) {
        crate::sched::add_to_run_queue(tid);
    }
}

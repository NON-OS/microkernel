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

//! `MkForeignInterrupt`: stopping a guest thread that is running its own code.
//!
//! A signal is delivered by answering a parked call with a handler to enter,
//! so a thread that makes no call never receives one. A supervisor marks such
//! a thread here. At the next timer tick that interrupts it in user mode the
//! kernel parks it with its whole register file, as if it had made a call
//! numbered `NR_INTERRUPTED`, and hands that to the supervisor. The answer is
//! a handler to enter, or anything else to run on exactly where it was. The
//! kernel stops and holds the thread; what it is stopped for is the
//! supervisor's.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

use super::frame::{ForeignFrame, NR_INTERRUPTED};
use super::interrupt_frame::{to_user, to_words, WORDS};
use super::trap_table::{is_parked, park, Answer};

static MARKED: Mutex<Vec<u32>> = Mutex::new(Vec::new());
/// Set while any thread is marked, so a tick with nothing marked costs one load.
static ANY: AtomicBool = AtomicBool::new(false);

/// Mark `pid`, one of the caller's guests. 1 says it is parked in a call
/// already, whose answer can carry the handler; 0 says it is marked.
pub fn sys_foreign_interrupt(pid: u64) -> i64 {
    let pid = match super::signal_call::supervised(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if is_parked(pid) {
        return 1;
    }
    let mut marked = MARKED.lock();
    if !marked.contains(&pid) {
        marked.push(pid);
    }
    ANY.store(true, Ordering::Release);
    0
}

/// A thread that is gone keeps no mark for a later one with its pid.
pub(super) fn forget(pid: u32) {
    let mut marked = MARKED.lock();
    marked.retain(|&p| p != pid);
    ANY.store(!marked.is_empty(), Ordering::Release);
}

fn take(pid: u32) -> bool {
    let mut marked = MARKED.lock();
    let Some(at) = marked.iter().position(|&p| p == pid) else {
        return false;
    };
    marked.swap_remove(at);
    ANY.store(!marked.is_empty(), Ordering::Release);
    true
}

/// Called by the timer trampoline, after the tick, for a tick that
/// interrupted user mode. `frame` is the interrupted register file the
/// trampoline restores (`interrupt_frame`).
pub fn on_user_tick(frame: &mut [u64; WORDS]) {
    if !ANY.load(Ordering::Acquire) || !crate::smp::preempt_enabled() {
        return;
    }
    let Some(pid) = crate::process::current_pid() else {
        return;
    };
    if !take(pid) {
        return;
    }
    let Some(supervisor) = super::registry::supervisor_of(pid) else {
        return;
    };
    let fs_base = crate::process::with_process(pid, |p| p.get_tls_base()).unwrap_or(0);
    let held = to_user(frame, fs_base);
    super::trap_frame::keep(pid, held);
    if !park(ForeignFrame::new(pid, NR_INTERRUPTED, [0; 6], held.rip)) {
        super::trap_frame::drop_frame(pid);
        return;
    }
    crate::sched::wake_process(supervisor);
    if let Answer::Deliver(to) = super::trap_wait::wait_raw(pid) {
        super::signal_enter::enter_fpu(pid);
        crate::arch::context::set_user_tls(to.fs_base);
        *frame = to_words(&to);
    }
}

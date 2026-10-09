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

//! A second thread inside a guest.

use super::peer_guard::{in_user_half, pid_arg};
use crate::arch::context::SavedUser;
use crate::process::core::{admit_thread, spawn_thread_parked};
use crate::syscall::microkernel::errnos::{
    ERRNO_AGAIN, ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_PERM,
};

/// `MkForeignThread`: a thread in `pid`, sharing its address space and
/// supervised by the same caller.
///
/// `from` is zero, or the thread of that guest parked in the call that asked
/// for this one. Given, the new thread starts on a copy of its registers, as a
/// Linux clone child does: zero in the return register, the stack and entry
/// given here, and the parent's thread pointer unless `tls` names another. Go
/// hands the child its function and its thread state in registers and calls
/// through them, so a child started on fresh registers called address zero.
pub fn sys_foreign_thread(pid: u64, entry: u64, rsp: u64, tls: u64, from: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = match pid_arg(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if super::registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    /*
     * Ring three addresses only, and the thread pointer among them: the switch
     * writes that one to an MSR that faults in ring zero on a non-canonical
     * value.
     */
    if !crate::process::core::start_in_user_half(entry, rsp) {
        return ERRNO_INVAL;
    }
    if tls != 0 && !in_user_half(tls, 1) {
        return ERRNO_INVAL;
    }
    let parent = match from {
        0 => None,
        raw => match parked_parent(caller, pid, raw) {
            Ok(p) => Some(p),
            Err(e) => return e,
        },
    };
    if !super::room::has_room(super::registry::guest_count(caller)) {
        return ERRNO_AGAIN;
    }
    let Ok(tid) = spawn_thread_parked(pid, entry, rsp) else {
        return ERRNO_NOMEM;
    };
    let tls = match (tls, &parent) {
        (0, Some((_, parent_tls))) => *parent_tls,
        _ => tls,
    };
    if tls != 0 {
        crate::process::with_process(tid, |pcb| pcb.set_tls_base(tls));
    }
    if let Some((mut regs, _)) = parent {
        regs.rax = 0;
        regs.rip = entry;
        regs.rsp = rsp;
        /*
         * A saved context is what the switch resumes when no first entry is
         * pending, which is how a forked child starts; the fresh entry the
         * spawn prepared would otherwise win and drop every register.
         */
        crate::process::with_process(tid, |pcb| {
            pcb.pending_user_entry.lock().take();
            *pcb.saved_user_context.lock() = Some(regs);
        });
    }
    if !super::enrol::enrol(tid, caller) {
        crate::process::exit::teardown(tid, ERRNO_NOMEM as i32, false);
        return ERRNO_NOMEM;
    }
    admit_thread(tid);
    tid as i64
}

/*
 * The registers and thread pointer of the thread that asked. It must be one
 * this caller supervises, in the same thread group as `pid`: a copy across
 * groups would hand one guest another's register contents. It must also be
 * parked in a call, since only then are its registers held aside.
 */
fn parked_parent(caller: u32, pid: u32, raw: u64) -> Result<(SavedUser, u64), i64> {
    let from = pid_arg(raw)?;
    if super::registry::supervisor_of(from) != Some(caller) {
        return Err(ERRNO_PERM);
    }
    let group = |p: u32| crate::process::with_process(p, |pcb| pcb.thread_group_id());
    let (Some(want), Some(have)) = (group(pid), group(from)) else {
        return Err(ERRNO_INVAL);
    };
    if want != have {
        return Err(ERRNO_PERM);
    }
    let regs = super::trap_frame::parked_frame(from).ok_or(ERRNO_NOENT)?;
    let parent_tls = crate::process::with_process(from, |pcb| pcb.get_tls_base()).unwrap_or(0);
    Ok((regs, parent_tls))
}

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

//! `MkForeignContext` and `MkForeignSignal`: what a supervisor needs to deliver
//! a signal. It reads a parked guest's registers, and answers it with a whole
//! context instead of a value: a handler to enter, or a frame to return to.

use super::peer_guard::pid_arg;
use super::registry;
use super::signal_regs::{from_words, to_words, WORDS};
use super::trap_reply::answer_raw;
use super::trap_table::Answer;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};
use crate::usercopy::{read_user_value, write_user_value};

const DELIVER: u64 = 0;
const SIGRETURN: u64 = 1;

pub fn sys_foreign_context(pid: u64, out: u64) -> i64 {
    let pid = match supervised(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let Some(frame) = super::trap_frame::parked_frame(pid) else {
        return ERRNO_NOENT;
    };
    match write_user_value(out, &to_words(&frame)) {
        Ok(()) => 0,
        Err(_) => ERRNO_FAULT,
    }
}

pub fn sys_foreign_signal(pid: u64, regs: u64, kind: u64) -> i64 {
    let pid = match supervised(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let Ok(words) = read_user_value::<[u64; WORDS]>(regs) else {
        return ERRNO_FAULT;
    };
    let fs_base = crate::process::with_process(pid, |p| p.get_tls_base()).unwrap_or(0);
    let Some(ctx) = from_words(&words, fs_base) else {
        return ERRNO_INVAL;
    };
    match kind {
        DELIVER => answer_raw(pid, Answer::Deliver(ctx)),
        SIGRETURN => answer_raw(pid, Answer::Sigreturn(ctx)),
        _ => ERRNO_INVAL,
    }
}

// Only the guest's recorded supervisor, as for a reply.
pub(super) fn supervised(pid: u64) -> Result<u32, i64> {
    let caller = crate::process::current_pid().ok_or(ERRNO_INVAL)?;
    let pid = pid_arg(pid)?;
    match registry::supervisor_of(pid) == Some(caller) {
        true => Ok(pid),
        false => Err(ERRNO_PERM),
    }
}

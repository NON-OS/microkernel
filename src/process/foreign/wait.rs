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

//! The supervisor's side of the loop: block until one of my guests trapped,
//! then take its frame.

use core::mem::size_of;

use super::frame::ForeignFrame;
use super::trap_table;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_TIMEDOUT};
use crate::usercopy::{validate_user_write, write_user_value};

/// `MkForeignWait`: wait for a guest of the calling process to issue a
/// syscall this kernel refuses, and copy its frame out. Returns the size
/// written, or `ERRNO_TIMEDOUT` when the deadline passes with nothing
/// waiting. A caller that supervises nothing waits like any other.
pub fn sys_foreign_wait(out_ptr: u64, out_len: u64, timeout_ms: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let size = size_of::<ForeignFrame>();
    if (out_len as usize) < size {
        return ERRNO_INVAL;
    }
    if validate_user_write(out_ptr, size).is_err() {
        return ERRNO_FAULT;
    }
    let start = crate::time::timestamp_millis();
    loop {
        if let Some(frame) = trap_table::claim_next(caller) {
            if write_user_value(out_ptr, &frame).is_err() {
                return ERRNO_FAULT;
            }
            return size as i64;
        }
        let waited = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && waited >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        let token = crate::sched::wake_token(caller);
        if let Some(frame) = trap_table::claim_next(caller) {
            if write_user_value(out_ptr, &frame).is_err() {
                return ERRNO_FAULT;
            }
            return size as i64;
        }
        crate::sched::sleep_until_unless_woken(caller, deadline, token);
        crate::sched::yield_now();
    }
}

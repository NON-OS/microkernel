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
//! then take its frame; or until a message came to my own inbox, and say so.

mod deliver;
mod inbox;
mod wake_rule;

use core::mem::size_of;

use super::frame::ForeignFrame;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_TIMEDOUT};
use crate::usercopy::validate_user_write;
use deliver::{deliver_or_repost, next_delivery};
pub(super) use inbox::forget;
use wake_rule::Look;

/// What the wait returns, with nothing written, when a message came to the
/// caller's own inbox: neither a frame's size nor an errno.
const INBOX: i64 = 0;

/// `MkForeignWait`: wait for a guest of the calling process to issue a syscall
/// this kernel refuses, and copy its frame out; its size. A message put in the
/// caller's own inbox ends the wait too, once, with 0, so a personality that
/// serves IPC is woken at once instead of at its timeout (wake_rule.rs).
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
        if let Some(rc) = look(caller, out_ptr, size) {
            return rc;
        }
        let waited = crate::time::timestamp_millis().saturating_sub(start);
        if timeout_ms > 0 && waited >= timeout_ms {
            return ERRNO_TIMEDOUT;
        }
        let deadline = if timeout_ms == 0 { u64::MAX } else { start.saturating_add(timeout_ms) };
        let token = crate::sched::wake_token(caller);
        if let Some(rc) = look(caller, out_ptr, size) {
            return rc;
        }
        crate::sched::sleep_until_unless_woken(caller, deadline, token);
        crate::sched::yield_now();
    }
}

/// A frame handed over, or INBOX, or None to sleep on.
fn look(caller: u32, out_ptr: u64, size: usize) -> Option<i64> {
    let frame = next_delivery(caller);
    match inbox::look(caller, frame.is_some()) {
        Look::Frame => frame.map(|frame| deliver_or_repost(caller, out_ptr, frame, size)),
        Look::Inbox => Some(INBOX),
        Look::Sleep => None,
    }
}

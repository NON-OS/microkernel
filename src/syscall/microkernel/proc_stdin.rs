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

//! Parent-to-child stdin channel, the write-side counterpart to the
//! mirrored stdout drain in `proc_output.rs`. A launcher (the
//! terminal) feeds one message to a running child capsule's
//! `stdin.<pid>` inbox; the child drains it with `MkStdinRead`. Same
//! inbox-naming convention and parent gate as `sys_proc_output`.
//! The kernel's copies of stdin bytes are zeroed once handed on, since
//! they can be what a person typed.

mod read;

use super::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};
use crate::ipc::nonos_inbox::StrictEnqueueError;
use crate::process::{current_pid, get_parent_pid};

pub use read::sys_stdin_read;

pub fn sys_proc_input(pid: u64, buf_ptr: u64, buf_len: usize) -> i64 {
    if buf_ptr == 0 || buf_len == 0 || pid == 0 || pid > u32::MAX as u64 {
        return ERRNO_INVAL;
    }
    if buf_len > crate::ipc::nonos_channel::MAX_MESSAGE_SIZE {
        return ERRNO_INVAL;
    }
    // Only the parent that loaded the capsule may feed its stdin. Without
    // this any IPC-capable capsule could inject input into another
    // capsule's stdin by passing its pid, since the inbox name is derived
    // from the argument.
    let target = pid as u32;
    let caller = current_pid().unwrap_or(0);
    if caller == 0 || get_parent_pid(target) != Some(caller) {
        return ERRNO_PERM;
    }
    if crate::usercopy::validate_user_read(buf_ptr, buf_len).is_err() {
        return ERRNO_FAULT;
    }
    let mut data = alloc::vec![0u8; buf_len];
    if crate::usercopy::copy_from_user(buf_ptr, &mut data).is_err() {
        return ERRNO_FAULT;
    }
    let name = alloc::format!("stdin.{}", target);
    let from = alloc::format!("proc.{}", caller);
    let msg = crate::ipc::nonos_channel::IpcMessage::new(&from, &name, &data);
    let len = data.len() as i64;
    crate::crypto::secure_zero(&mut data);
    let Ok(msg) = msg else {
        return ERRNO_INVAL;
    };
    match crate::ipc::nonos_inbox::try_enqueue_strict(&name, msg) {
        Ok(()) => len,
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => ERRNO_NOENT,
        Err(StrictEnqueueError::QueueFull(mut refused)) => {
            crate::crypto::secure_zero(&mut refused.data);
            ERRNO_BUSY
        }
    }
}

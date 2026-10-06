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
//! `MkPrivateWrite`: output that only the caller's launcher reads.
//!
//! The bytes go to the caller's own `proc.<pid>` inbox and nowhere else:
//! never the boot serial, never the framebuffer console, whatever the
//! caller holds. A program whose output is someone's private text writes
//! it here. Nothing is dropped quietly: with no inbox the call is ENODEV,
//! and with a full one it is EBUSY and takes nothing.

use super::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV};

const MAX_LEN: usize = 256;

pub fn sys_private_write(user_ptr: u64, len: u64) -> i64 {
    if user_ptr == 0 || len == 0 || len as usize > MAX_LEN {
        return ERRNO_INVAL;
    }
    let len = len as usize;
    let mut buf = [0u8; MAX_LEN];
    if crate::usercopy::copy_from_user(user_ptr, &mut buf[..len]).is_err() {
        return ERRNO_FAULT;
    }
    let Some(pid) = crate::process::current_pid() else {
        return ERRNO_NODEV;
    };
    let name = alloc::format!("proc.{}", pid);
    if !crate::ipc::nonos_inbox::exists(&name) {
        return ERRNO_NODEV;
    }
    if crate::ipc::nonos_inbox::is_full(&name) {
        return ERRNO_BUSY;
    }
    let sent = crate::ipc::nonos_channel::IpcMessage::new(&name, &name, &buf[..len])
        .ok()
        .and_then(|m| crate::ipc::nonos_inbox::try_enqueue_strict(&name, m).ok());
    buf.fill(0);
    match sent {
        Some(_) => len as i64,
        None => ERRNO_BUSY,
    }
}

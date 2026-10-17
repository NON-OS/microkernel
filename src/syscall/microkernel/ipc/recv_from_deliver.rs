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

use crate::syscall::microkernel::errnos::ERRNO_FAULT;

use super::sender_pid::from_envelope;

pub(super) fn deliver(
    pid: u32,
    msg: &crate::ipc::nonos_channel::IpcMessage,
    buf: u64,
    len: usize,
    (sender_pid_out, enqueued_ms_out): (u64, u64),
) -> i64 {
    let sender_pid = from_envelope(&msg.from);
    let copy_len = msg.data.len().min(len);
    if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
        return ERRNO_FAULT;
    }
    if sender_pid_out != 0 {
        let bytes = sender_pid.to_le_bytes();
        if crate::usercopy::copy_to_user(sender_pid_out, &bytes).is_err() {
            return ERRNO_FAULT;
        }
    }
    if enqueued_ms_out != 0 {
        let now = (crate::sys::clock::since_boot_ms(), crate::time::timestamp_millis());
        let at = super::enqueued_at::enqueued_uptime(now.0, now.1, msg.timestamp_ms);
        if crate::usercopy::copy_to_user(enqueued_ms_out, &at.to_le_bytes()).is_err() {
            return ERRNO_FAULT;
        }
    }
    crate::process::accounting::bump(pid, crate::process::accounting::Kind::IpcRx);
    copy_len as i64
}

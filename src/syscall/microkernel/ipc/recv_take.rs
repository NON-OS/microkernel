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

use crate::ipc::nonos_channel::IpcMessage;
use crate::syscall::microkernel::errnos::ERRNO_FAULT;

/// A dequeued message handed to the receiver: the server's token recorded,
/// the bytes copied out.
pub(super) fn taken(pid: u32, msg: &IpcMessage, buf: u64, len: usize) -> i64 {
    // The reply redirect pairs by this token, not by queue position:
    // the entry popped for the server's next reply is the one whose
    // token names the request it just picked up.
    super::pending_reply::record_served(pid, msg.correlation);
    let copy_len = msg.data.len().min(len);
    if crate::usercopy::copy_to_user(buf, &msg.data[..copy_len]).is_err() {
        return ERRNO_FAULT;
    }
    crate::process::accounting::bump(pid, crate::process::accounting::Kind::IpcRx);
    copy_len as i64
}

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

//! The pending entry an MkIpcReplyTo answers, taken out (served_rule.rs).

use alloc::string::String;

use super::served_rule::named_at;
use super::state::PENDING;

/// Remove the dequeued call `server_pid` knows as `request` and return its
/// caller's pid, reply inbox and token. None: no such call is owed a reply,
/// because it was never queued here, is still in the inbox, or is answered.
pub(in crate::syscall::microkernel::ipc) fn take_named(
    server_pid: u32,
    request: u64,
) -> Option<(u32, String, u64)> {
    let mut map = PENDING.lock();
    let queue = map.get_mut(&server_pid)?;
    let at = named_at(queue.iter(), request)?;
    let entry = queue.remove(at).map(|e| (e.pid, e.inbox, e.token));
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    entry
}

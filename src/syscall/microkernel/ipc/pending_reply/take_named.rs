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

//! The pending entry an MkIpcReplyTo answers (served_rule.rs), held while
//! its answer is delivered and taken out after (give_up_rule.rs).

use alloc::string::String;

use super::give_up_rule::take;
use super::served_rule::named_at;
use super::state::PENDING;

/// What an MkIpcReplyTo found.
pub(in crate::syscall::microkernel::ipc) enum Named {
    /// Owed: the caller's pid, reply inbox and token. Taken now, so no
    /// other reply finds it; `delivered` takes it out.
    Owed(u32, String, u64),
    /// Its call gave up: spent, with nothing sent.
    GivenUp,
}

/// The dequeued call `server_pid` knows as `request`. None: no such call
/// is owed, because it was never queued here, is still in the inbox, or
/// is answered or being answered.
pub(in crate::syscall::microkernel::ipc) fn take_named(
    server_pid: u32,
    request: u64,
) -> Option<Named> {
    let mut map = PENDING.lock();
    let queue = map.get_mut(&server_pid)?;
    let at = named_at(queue.iter(), request)?;
    let e = queue.get_mut(at)?;
    if take(e) {
        return Some(Named::Owed(e.pid, e.inbox.clone(), e.token));
    }
    queue.remove(at);
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    Some(Named::GivenUp)
}

/// The answer to `request` was delivered, or could not be: its entry goes.
pub(in crate::syscall::microkernel::ipc) fn delivered(server_pid: u32, request: u64) {
    let mut map = PENDING.lock();
    let Some(queue) = map.get_mut(&server_pid) else { return };
    super::give_up_rule::done(queue, request);
    if queue.is_empty() {
        map.remove(&server_pid);
    }
}

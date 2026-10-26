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

//! The pending entry a direct reply answers, taken out (reply_rule.rs).

use super::reply_rule::pick;
use super::state::{last_served, PENDING};

/// Remove the entry a direct reply from `server_pid` to `caller_inbox`
/// answers and return its token. None: no call is owed this reply, a
/// forged or duplicate one, which the caller drops.
pub(in crate::syscall::microkernel::ipc) fn take_for_reply(
    server_pid: u32,
    caller_inbox: &str,
) -> Option<u64> {
    let served = last_served(server_pid);
    let mut map = PENDING.lock();
    let queue = map.get_mut(&server_pid)?;
    let entries = queue.iter().map(|e| (e.inbox.as_str(), e.token, e.served));
    let at = pick(entries, caller_inbox, served)?;
    let token = queue.remove(at)?.token;
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    Some(token)
}

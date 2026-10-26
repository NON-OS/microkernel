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

//! A request the server has dequeued: its token recorded as the last
//! served, and its pending entry marked (served_rule.rs), so a reply can
//! answer it and nothing still in the inbox. Called at every place a
//! server's receive takes a message (recv_take.rs, recv_from_drain.rs),
//! after the message has left the inbox and with no lock held;
//! scripts/check_pending_order.py holds that.

use super::served_rule::mark;
use super::state::{record_served, PENDING};

/// `server_pid` dequeued the request carrying `token`. A message no call
/// is owed (a kernel sender's, token 0) is recorded and marks nothing,
/// without taking PENDING.
pub(crate) fn mark_served(server_pid: u32, token: u64) {
    record_served(server_pid, token);
    if token == 0 {
        return;
    }
    if let Some(q) = PENDING.lock().get_mut(&server_pid) {
        mark(q.iter_mut(), token);
    }
}

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

//! A server's pending replies as the kernel keeps them, taken the way
//! pending_reply/take_reply.rs takes them.

use super::reply_rule::pick;

#[derive(Default)]
pub(super) struct Server {
    /// Each owed call: the caller's reply inbox and its token, oldest first.
    pending: Vec<(String, u64)>,
    /// The token of the request the server dequeued last (record_served).
    served: u64,
}

impl Server {
    /// A caller's mk_ipc_call: its entry, pushed.
    pub(super) fn call(&mut self, inbox: &str, token: u64) {
        self.pending.push((inbox.to_string(), token));
    }

    /// The server's receive took the request carrying `token`.
    pub(super) fn serve(&mut self, token: u64) {
        self.served = token;
    }

    /// The server's mk_ipc_reply to `inbox`: the token the reply is stamped
    /// with, or None when it is dropped.
    pub(super) fn reply(&mut self, inbox: &str) -> Option<u64> {
        let entries = self.pending.iter().map(|(i, t)| (i.as_str(), *t));
        let at = pick(entries, inbox, self.served)?;
        Some(self.pending.remove(at).1)
    }
}

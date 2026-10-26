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

//! A server's pending replies as the kernel keeps them: entries marked by
//! pending_reply/served_rule.rs, taken by reply_rule.rs for a direct reply
//! and by served_rule.rs for a redirect, as take_reply.rs and pop.rs do;
//! MkIpcReplyTo and a caller giving up in model_answer.rs.

use std::collections::VecDeque;

use super::entry::Entry;
use super::reply_rule::pick;
use super::served_rule::{eligible, mark, redirect_at};

#[derive(Default)]
pub(super) struct Server {
    /// Each owed call, oldest first.
    pub(super) pending: VecDeque<Entry>,
    /// The token of the request the server dequeued last (record_served).
    pub(super) served: u64,
    /// The last request id given out (request_ids.rs).
    last_id: u64,
}

impl Server {
    /// A caller's mk_ipc_call: its entry, pushed.
    pub(super) fn call(&mut self, inbox: &str, token: u64) {
        self.last_id += 1;
        self.pending.push_back(Entry::new(0, inbox.to_string(), token, self.last_id));
    }

    /// The server's receive took the request carrying `token`
    /// (pending_reply/mark.rs): the id MkIpcRecvReq tells it, 0 for none.
    pub(super) fn serve(&mut self, token: u64) -> u64 {
        self.served = token;
        mark(self.pending.iter_mut(), token).unwrap_or(0)
    }

    /// The server's mk_ipc_reply to `inbox`: the token the reply is stamped
    /// with, or None when it is dropped.
    pub(super) fn reply(&mut self, inbox: &str) -> Option<u64> {
        let entries = self.pending.iter().map(|e| (e.inbox.as_str(), e.token, eligible(e)));
        let at = pick(entries, inbox, self.served)?;
        self.pending.remove(at).map(|e| e.token)
    }

    /// The server's reply through its own reply endpoint (pop.rs): the
    /// caller's inbox and token, or None when it reaches no one. Either
    /// way the request is spent.
    pub(super) fn redirect(&mut self) -> Option<(String, u64)> {
        let at = redirect_at(self.pending.iter(), self.served);
        self.served = 0;
        self.pending.remove(at?).map(|e| (e.inbox, e.token))
    }
}

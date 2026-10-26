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

//! The model's MkIpcReplyTo, as reply_to.rs and take_named.rs make it, in
//! its two steps (the entry taken, then done once delivered), and a call
//! that stops waiting (call/given_up.rs), each on the kernel's own rules
//! (served_rule.rs, give_up_rule.rs).

use super::give_up_rule::{done, mark_given_up, take, GiveUp};
use super::model::Server;
use super::served_rule::named_at;

/// What an MkIpcReplyTo found.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Begun {
    /// Owed: the caller's inbox and token; taken, delivery next.
    Owed(String, u64),
    /// Its call gave up: spent, nothing sent, ETIMEDOUT.
    GivenUp,
}

impl Server {
    /// MkIpcReplyTo's first step; None is ENOENT.
    pub(super) fn begin(&mut self, request: u64) -> Option<Begun> {
        let at = named_at(self.pending.iter(), request)?;
        let e = self.pending.get_mut(at)?;
        if take(e) {
            return Some(Begun::Owed(e.inbox.clone(), e.token));
        }
        self.pending.remove(at);
        Some(Begun::GivenUp)
    }

    /// Its last: delivered or not, the entry goes.
    pub(super) fn finish(&mut self, request: u64) {
        done(&mut self.pending, request);
    }

    /// A whole MkIpcReplyTo that delivers: the caller's inbox and token,
    /// or None for ENOENT and for a call that gave up.
    pub(super) fn reply_to(&mut self, request: u64) -> Option<(String, u64)> {
        let Begun::Owed(inbox, token) = self.begin(request)? else { return None };
        self.finish(request);
        Some((inbox, token))
    }

    /// The caller of `token` stops waiting.
    pub(super) fn give_up(&mut self, token: u64) -> GiveUp {
        mark_given_up(self.pending.iter_mut(), token)
    }
}

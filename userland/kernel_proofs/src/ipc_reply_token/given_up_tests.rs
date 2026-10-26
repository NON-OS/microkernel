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

//! A call that stops waiting and the MkIpcReplyTo that answers it, in every
//! order: the server's 0 and the caller holding the answer always go
//! together, so a server never counts as told an answer no one read, and
//! no entry is left behind (call/given_up.rs, reply_to.rs).

use super::given_up_race::{race, ETIMEDOUT};
use super::model::Server;
use super::model_answer::Begun;

#[test]
fn the_servers_0_and_the_callers_answer_go_together_in_every_order() {
    for first in 0..=3 {
        for lands in [true, false] {
            let (rc, got) = race(first, lands);
            assert_eq!(
                rc == 0,
                got == Some(11),
                "first {first}, lands {lands}: rc {rc}, got {got:?}"
            );
        }
    }
    assert_eq!(race(0, true), (ETIMEDOUT, None), "given up first: ETIMEDOUT, nothing sent");
    assert_eq!(race(1, true), (0, Some(11)), "taken first: the caller waits and takes it");
}

/// An answer being delivered is found by no other reply.
#[test]
fn an_answer_being_delivered_is_answered_by_nothing_else() {
    let mut s = Server::default();
    s.call("c", 11);
    let id = s.serve(11);
    assert_eq!(s.begin(id), Some(Begun::Owed("c".to_string(), 11)));
    assert_eq!(s.begin(id), None, "a second MkIpcReplyTo");
    assert_eq!(s.reply("c"), None, "an MkIpcReply");
    assert_eq!(s.redirect(), None, "a redirect");
    s.finish(id);
    assert!(s.pending.is_empty());
}

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

//! A call giving up while its server's MkIpcReplyTo runs: the server's
//! steps as reply_to.rs makes them, the caller's as call/given_up.rs does.

use super::give_up_rule::GiveUp;
use super::model::Server;
use super::model_answer::Begun;

pub(super) const ETIMEDOUT: i64 = -110;
const EBUSY: i64 = -16;

struct Race {
    s: Server,
    id: u64,
    lands: bool,
    owed: Option<u64>,
    inbox: Option<u64>,
    rc: Option<i64>,
    step: usize,
}

impl Race {
    fn advance(&mut self) {
        match self.step {
            0 => match self.s.begin(self.id) {
                Some(Begun::Owed(_, token)) => self.owed = Some(token),
                _ => self.rc = Some(ETIMEDOUT),
            },
            1 if self.lands => self.inbox = self.owed,
            2 if self.owed.is_some() => {
                self.s.finish(self.id);
                self.rc = Some(if self.lands { 0 } else { EBUSY });
            }
            _ => {}
        }
        self.step += 1;
    }
}

/// The caller gives up after `first` of the server's steps, the enqueue
/// landing when `lands`: the server's return, and the caller's token.
pub(super) fn race(first: usize, lands: bool) -> (i64, Option<u64>) {
    let mut s = Server::default();
    s.call("c", 11);
    let id = s.serve(11);
    let mut r = Race { s, id, lands, owed: None, inbox: None, rc: None, step: 0 };
    (0..first).for_each(|_| r.advance());
    let got = loop {
        match r.s.give_up(11) {
            GiveUp::Marked => break None,
            GiveUp::Delivering => r.advance(),
            GiveUp::Gone => break r.inbox.take(),
        }
    };
    while r.step < 3 {
        r.advance();
    }
    assert!(r.s.pending.is_empty(), "first {first}: an entry left behind");
    (r.rc.expect("the server returned"), got)
}

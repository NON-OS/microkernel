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

//! The last answer each caller was given, for its numbered exchange.
//!
//! An answer carries stream bytes taken off the stream to build it, and the
//! kernel drops a reply whose caller already stopped waiting. A caller that
//! asks again with the same number gets the same answer instead of losing
//! those bytes. It outlives the conversation, since the answer that ends a
//! conversation can be lost like any other.
//!
//! A caller moves to its next number only once an exchange is answered, so
//! what it sends after a lost answer comes under the old number, and that
//! is often new bytes: the browser's write after a poll it stopped waiting
//! for is its TLS hello, or the request. Giving the kept answer back for
//! those dropped them while the caller counted them sent, which is the rule
//! net.socks5's kept.rs already corrects. So the kept answer alone goes
//! only to a request with nothing to carry or with the very bytes already
//! carried; new bytes are carried, and the answer the caller missed goes in
//! front of what they bring back.

extern crate alloc;

use alloc::vec::Vec;

use super::front::CALLERS_MAX;
use super::who::{pid_of, Who, STREAMS_PER_CALLER};

struct Entry {
    pid: Who,
    seq: u32,
    carried: Vec<u8>,
    reply: Vec<u8>,
}

/// What a numbered request is, against the answer kept for its caller.
pub enum Again {
    /// A new exchange.
    New,
    /// The exchange asked again with nothing new: its answer, as given.
    Same(Vec<u8>),
    /// New bytes under the number of an answer the caller never received:
    /// that answer, to go in front of what the new bytes bring back.
    Missed(Vec<u8>),
}

#[derive(Default)]
pub struct Kept {
    list: Vec<Entry>,
}

impl Kept {
    /// What `pid`'s request for exchange `seq`, carrying `body`, is.
    pub fn again(&self, pid: Who, seq: u32, body: &[u8]) -> Again {
        match self.list.iter().find(|k| k.pid == pid && k.seq == seq) {
            None => Again::New,
            Some(k) if body.is_empty() || k.carried == body => Again::Same(k.reply.clone()),
            Some(k) => Again::Missed(k.reply.clone()),
        }
    }

    /// Keep the answer to `pid`'s exchange `seq`, which carried `carried`,
    /// replacing its last one. Bounded like the conversations, the oldest
    /// going first: in all, and for one caller, so naming ever more streams
    /// cannot push out the answers kept for others.
    pub fn keep(&mut self, pid: Who, seq: u32, carried: &[u8], reply: &[u8]) {
        self.forget(pid);
        let caller = pid_of(pid);
        if self.list.iter().filter(|k| pid_of(k.pid) == caller).count() >= STREAMS_PER_CALLER {
            if let Some(oldest) = self.list.iter().position(|k| pid_of(k.pid) == caller) {
                self.list.remove(oldest);
            }
        }
        if self.list.len() >= CALLERS_MAX {
            self.list.remove(0);
        }
        self.list.push(Entry { pid, seq, carried: carried.to_vec(), reply: reply.to_vec() });
    }

    pub fn forget(&mut self, pid: Who) {
        self.list.retain(|k| k.pid != pid);
    }

    /// Forget the answers of every caller `alive` says has ended.
    pub fn forget_ended(&mut self, alive: impl Fn(u32) -> bool) {
        self.list.retain(|k| alive(pid_of(k.pid)));
    }
}

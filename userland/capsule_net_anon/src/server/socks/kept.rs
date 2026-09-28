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

extern crate alloc;

use alloc::vec::Vec;

use super::front::CALLERS_MAX;

struct Entry {
    pid: u32,
    seq: u32,
    reply: Vec<u8>,
}

#[derive(Default)]
pub struct Kept {
    list: Vec<Entry>,
}

impl Kept {
    /// The answer already given to `pid` for exchange `seq`, if that is the
    /// exchange being asked again.
    pub fn again(&self, pid: u32, seq: u32) -> Option<Vec<u8>> {
        self.list.iter().find(|k| k.pid == pid && k.seq == seq).map(|k| k.reply.clone())
    }

    /// Keep the answer to `pid`'s exchange `seq`, replacing its last one.
    /// Bounded like the conversations, the oldest going first.
    pub fn keep(&mut self, pid: u32, seq: u32, reply: &[u8]) {
        self.forget(pid);
        if self.list.len() >= CALLERS_MAX {
            self.list.remove(0);
        }
        self.list.push(Entry { pid, seq, reply: reply.to_vec() });
    }

    pub fn forget(&mut self, pid: u32) {
        self.list.retain(|k| k.pid != pid);
    }
}

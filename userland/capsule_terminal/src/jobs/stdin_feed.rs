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

//! Queueing input as messages, and handing them to the program a tick at a time.

use nonos_libc::mk_proc_input;

use super::{StdinQueue, QUEUE_MAX};
use crate::jobs::stdin_cut::cut_at;

/// Messages handed over in one tick.
const TICK_MESSAGES: usize = 64;

impl StdinQueue {
    /// Queue `bytes` as messages of at most `MESSAGE_MAX`, cut between
    /// characters. False, with nothing queued, when full.
    pub fn push(&mut self, bytes: &[u8]) -> bool {
        if self.sealed || self.bytes + bytes.len() > QUEUE_MAX {
            return false;
        }
        let mut rest = bytes;
        while !rest.is_empty() {
            let cut = cut_at(rest);
            self.pending.push_back(rest[..cut].to_vec());
            rest = &rest[cut..];
        }
        self.bytes += bytes.len();
        true
    }

    /// Hand the kernel what it takes this tick, oldest first. A message it
    /// refuses (a full inbox, or one not made yet) waits for the next tick.
    pub fn feed(&mut self, pid: u32) {
        for _ in 0..TICK_MESSAGES {
            let Some(msg) = self.pending.front() else { return };
            if mk_proc_input(pid as u64, msg.as_ptr(), msg.len() as u64) <= 0 {
                return;
            }
            self.bytes -= msg.len();
            self.pending.pop_front();
        }
    }
}

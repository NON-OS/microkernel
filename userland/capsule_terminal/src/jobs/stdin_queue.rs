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

//! Input sent to a running program and not yet taken, as whole messages the
//! kernel hands it one read at a time. Each typed line is its own message, and
//! so is the lone 0x04 ending input, which merged into a line reads as a character.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use nonos_libc::mk_proc_input;

use super::stdin_cut::cut_at;

/// End of transmission: sent alone, the program's input is over.
pub const EOT: u8 = 0x04;
/// Held and not yet taken. Past this, input is refused, not queued.
const QUEUE_MAX: usize = 1024 * 1024;
/// Messages handed over in one tick.
const TICK_MESSAGES: usize = 64;

pub struct StdinQueue {
    pending: VecDeque<Vec<u8>>,
    bytes: usize,
    /// Whether the program takes a lone 0x04 as the end of its input.
    pub hears_eot: bool,
    /// Its input came whole from a file (`< f`) and is over: the keyboard
    /// is not its input, so nothing more is taken.
    sealed: bool,
}

impl StdinQueue {
    pub fn new(hears_eot: bool) -> Self {
        Self { pending: VecDeque::new(), bytes: 0, hears_eot, sealed: false }
    }

    /// The most a file given as input (`< f`) may hold: what the queue
    /// takes, less the end of input after it.
    pub const FILE_MAX: usize = QUEUE_MAX - 1;

    /// The program's whole input: `bytes`, then the end of input, and
    /// nothing after, from the keyboard or anywhere. False, with nothing
    /// queued, for a program that hears no end of input or for more than
    /// `FILE_MAX`.
    pub fn whole(&mut self, bytes: &[u8]) -> bool {
        if !self.hears_eot || bytes.len() > Self::FILE_MAX || !self.push(bytes) {
            return false;
        }
        let ended = self.push(&[EOT]);
        self.sealed = true;
        ended
    }

    /// Whether typed keys still go to the program.
    pub fn takes_keys(&self) -> bool {
        !self.sealed
    }

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

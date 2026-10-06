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

//! Raising a signal, in Linux's order: a standard signal already pending for
//! the same target is not queued again, a realtime one queues each time until
//! the queue is full; taking one is in sigtake.

use super::sigdefault::{default_of, Default};
use super::siginfo::SigInfo;
use super::sigqueue::{Signals, QUEUE_MAX};
use super::sigstate::bit;

/// The first realtime signal: from here up each raise is queued.
const SIGRTMIN: u8 = 32;

impl Signals {
    /// Queue `info` for thread `tid`, or for the process when `tid` is 0.
    /// False when nothing was queued: an ignored signal no thread blocks is
    /// discarded as it is sent, as Linux does, and so is a coalesced one.
    pub fn raise(&mut self, tid: u32, info: SigInfo) -> bool {
        let s = info.signo;
        if self.discards(s) && !self.threads.iter().any(|t| t.blocked & bit(s) != 0) {
            return false;
        }
        if s < SIGRTMIN && self.pending.iter().any(|(t, i)| *t == tid && i.signo == s) {
            return false;
        }
        if s >= SIGRTMIN && self.pending.len() >= QUEUE_MAX {
            return false;
        }
        self.pending.push((tid, info));
        true
    }

    /// True when taking `signum` would do nothing: ignored by its handler,
    /// or by a default of ignore or continue with nothing ever stopped.
    pub fn discards(&self, signum: u8) -> bool {
        let act = self.actions[signum as usize - 1];
        act.ignores()
            || (!act.catches() && matches!(default_of(signum), Default::Ignore | Default::Continue))
    }
}

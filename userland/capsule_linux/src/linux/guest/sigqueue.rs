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

//! Signals raised against a process's threads and not yet delivered, with the
//! disposition of each. The queue names the thread a signal is for.

use alloc::vec::Vec;

use super::sigstate::{SigAction, NSIG};

#[derive(Clone)]
pub struct Signals {
    actions: [SigAction; NSIG],
    pending: Vec<(u32, u8)>,
}

impl Default for Signals {
    fn default() -> Self {
        Self { actions: [SigAction::default(); NSIG], pending: Vec::new() }
    }
}

impl Signals {
    /// Record a disposition; `signum` is 1..=NSIG.
    pub fn set(&mut self, signum: usize, act: SigAction) {
        if (1..=NSIG).contains(&signum) {
            self.actions[signum - 1] = act;
        }
    }

    pub fn action(&self, signum: usize) -> Option<SigAction> {
        (1..=NSIG).contains(&signum).then(|| self.actions[signum - 1])
    }

    /// Queue a signal against a thread. A standard signal already pending is
    /// not queued twice, as Linux coalesces non-realtime signals.
    pub fn raise(&mut self, tid: u32, signum: u8) {
        if !self.pending.iter().any(|p| *p == (tid, signum)) {
            self.pending.push((tid, signum));
        }
    }

    /// The next signal for `tid` its disposition catches, removed. Signals
    /// with no handler are left for the caller to default.
    pub fn take_caught(&mut self, tid: u32) -> Option<(u8, SigAction)> {
        let at = self
            .pending
            .iter()
            .position(|(t, s)| *t == tid && self.actions[*s as usize - 1].catches())?;
        let signum = self.pending.remove(at).1;
        Some((signum, self.actions[signum as usize - 1]))
    }

    /// Drop every signal pending for a thread that has gone.
    pub fn forget(&mut self, tid: u32) {
        self.pending.retain(|(t, _)| *t != tid);
    }
}

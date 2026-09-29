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

//! Reading and recording a process's signal state: the disposition of each
//! signal, the nearest deadline its timers and waits have, and what is
//! pending for a thread.

use super::sigqueue::Signals;
use super::sigstate::{bit, SigAction, NSIG};

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

    /// The nearest deadline of a timer or a sigtimedwait, for the serve loop's
    /// wait to end by.
    pub fn next_due(&self) -> Option<u64> {
        let timers = self.timers.iter().filter_map(|t| t.due);
        let waits = self.sigwaits.iter().filter_map(|w| w.due);
        self.real.map(|t| t.due).into_iter().chain(timers).chain(waits).min()
    }

    /// Every signal pending for `tid` or for its process, as a mask.
    pub fn pending_for(&self, tid: u32) -> u64 {
        let mine = self.pending.iter().filter(|(t, _)| *t == tid || *t == 0);
        mine.fold(0, |m, (_, i)| m | bit(i.signo))
    }
}

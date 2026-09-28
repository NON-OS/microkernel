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

//! The signal state a new program starts with: a forked child's, copied from
//! the forking thread, and what exec leaves of a process's own.

use super::sigqueue::Signals;
use super::sigstate::SigAction;
use super::sigthread::ThreadSig;

impl Signals {
    /// A forked child: the dispositions, and the forking thread's mask and
    /// alternate stack for its one thread. Nothing pending, no timers.
    pub fn forked(&self, caller: u32, child: u32) -> Signals {
        let mut s = Signals { actions: self.actions, ..Signals::default() };
        let mine = self.threads.iter().find(|t| t.tid == caller).copied();
        let mut t = mine.unwrap_or(ThreadSig::new(child, 0));
        t.tid = child;
        t.saved = None;
        s.threads.push(t);
        s
    }

    /// Exec keeps the mask, the pending signals and ITIMER_REAL; caught
    /// signals go back to their default, ignored ones stay ignored; the
    /// alternate stack and the POSIX timers are gone.
    pub fn exec_reset(&mut self, tid: u32) {
        for act in self.actions.iter_mut().filter(|a| a.catches()) {
            *act = SigAction::default();
        }
        self.timers.clear();
        self.pending.retain(|(_, i)| i.timer.is_none());
        let blocked = self.blocked(tid);
        self.threads = alloc::vec![ThreadSig::new(tid, blocked)];
    }
}

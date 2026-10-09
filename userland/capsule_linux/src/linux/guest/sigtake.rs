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

//! Taking a signal, in Linux's order: the lowest-numbered first, the thread's
//! own before the process's; and dropping what a deleted timer or a newly
//! ignored signal leaves pending.

use super::siginfo::SigInfo;
use super::sigqueue::Signals;
use super::sigstate::bit;

impl Signals {
    /// Take the next signal `tid` may take: one of `allow`, its own before the
    /// process's, lowest first. A timer's signal carries the overruns counted
    /// while it waited, and its timer is free to queue again.
    pub fn take(&mut self, tid: u32, allow: u64) -> Option<SigInfo> {
        let lowest = |want: u32| {
            let each = self.pending.iter().enumerate();
            let fit = each.filter(|(_, (t, i))| *t == want && allow & bit(i.signo) != 0);
            fit.min_by_key(|(_, (_, i))| i.signo).map(|(at, _)| at)
        };
        let at = lowest(tid).or_else(|| lowest(0))?;
        let mut info = self.pending.remove(at).1;
        if let Some((id, _)) = info.timer {
            if let Some(t) = self.timers.iter_mut().find(|t| t.id == id) {
                info.timer = Some((id, t.overrun));
                t.last_overrun = t.overrun;
                t.overrun = 0;
                t.queued = false;
            }
        }
        Some(info)
    }

    /// Drop the signal a deleted timer left queued.
    pub fn drop_timer_signal(&mut self, id: i32) {
        self.pending.retain(|(_, i)| i.timer.is_none_or(|(t, _)| t != id));
    }

    /// Drop every pending `signum`, as setting SIG_IGN on it does.
    pub fn discard(&mut self, signum: u8) {
        self.pending.retain(|(_, i)| i.signo != signum);
        self.timers_requeue();
    }

    /// A timer whose queued signal was dropped may queue again.
    pub fn timers_requeue(&mut self) {
        for t in self.timers.iter_mut().filter(|t| t.queued) {
            t.queued = self.pending.iter().any(|(_, i)| i.timer.is_some_and(|(id, _)| id == t.id));
        }
    }

    /// Each pending signal with the thread it is for, 0 for the process.
    pub fn queued(&self) -> alloc::vec::Vec<(u32, u8)> {
        self.pending.iter().map(|(t, i)| (*t, i.signo)).collect()
    }
}

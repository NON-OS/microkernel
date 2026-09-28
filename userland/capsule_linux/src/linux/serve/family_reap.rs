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

//! Ending what has exited, and telling each parent: the ended child waits as
//! a zombie until it is waited for, and its exit signal, SIGCHLD unless clone
//! named another, is raised at the parent with CLD_EXITED or CLD_KILLED. A
//! parent that ignores SIGCHLD, or asked for SA_NOCLDWAIT, has its children
//! reaped as they end, as Linux does.

use super::family::Family;
use crate::linux::guest::siginfo::{SigInfo, CLD_EXITED, CLD_KILLED};
use crate::linux::guest::sigstate::{SA_NOCLDWAIT, SIGCHLD};
use crate::linux::guest::Guest;

impl Family {
    /// End every process that asked to, answer the waits and signals that
    /// follows, and go again while that ends anything more.
    pub fn reap(&mut self) {
        self.settle_pipes();
        loop {
            let ended = self.end_exited();
            self.settle_child_waits();
            self.settle_signals();
            if !ended && !self.guests.iter().any(|g| g.exited.is_some()) {
                return;
            }
        }
    }
}

/// Tell `p` that `gone` ended with `status`: kept as a zombie or reaped at
/// once, and its exit signal raised with CLD_EXITED or CLD_KILLED.
pub fn tell_parent(p: &mut Guest, gone: &Guest, status: i32) {
    let sig = gone.signals.exit_signal;
    let chld = p.signals.action(SIGCHLD as usize).unwrap_or_default();
    if sig == SIGCHLD && (chld.ignores() || chld.flags & SA_NOCLDWAIT != 0) {
        p.children.retain(|c| *c != gone.pid);
    } else {
        p.ended.push((gone.pid, status));
        p.signals.kid_groups.push((gone.pid, gone.pgid));
    }
    if sig != 0 {
        let (code, value) = match status & 0x7f {
            0 => (CLD_EXITED, (status >> 8) & 0xff),
            s => (CLD_KILLED, s),
        };
        let info = SigInfo { value: value as u64, ..SigInfo::from(sig, code, gone.pid) };
        let _ = p.signals.raise(0, info);
    }
}

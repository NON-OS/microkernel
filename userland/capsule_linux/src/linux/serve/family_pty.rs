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

//! The family's pseudo-terminals settled after each answer: the note of
//! which sides are open is taken again, so a master that went with its
//! process hangs its slave up, and the signals the pairs raised (^C typed
//! at a master, a new window size, a hang-up) reach their processes.

use super::family::Family;
use super::pid_space::outward;
use crate::linux::guest::siginfo::{SigInfo, SI_KERNEL};
use crate::linux::pty;
use crate::linux::pty::table::To;

impl Family {
    pub(super) fn settle_ptys(&mut self) {
        pty::note(self.guests.iter().flat_map(|g| g.fds.iter()));
        for (to, signo) in pty::signals() {
            for g in self.guests.iter_mut().filter(|g| g.exited.is_none()) {
                let hit = match to {
                    To::Group(group) => outward(g.pgid) == group,
                    To::Leader(pid) => g.pid == pid,
                };
                if hit {
                    let _ = g.raise_and_wake(0, SigInfo::from(signo, SI_KERNEL, 0));
                }
            }
        }
    }
}

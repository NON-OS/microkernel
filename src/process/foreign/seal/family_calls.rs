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

//! A family's open calls: stamped at trap, then closed once, by an answer,
//! by a failed park, or as died when the thread leaves without one.

use super::super::trap_reply::ABANDONED as ENOENT;
use super::super::trap_table::Answer;
use super::family::Family;
use super::trace::{Row, ABANDONED, DELIVER, DIED, EXECED, SIGRETURN, VALUE};

impl Family {
    /// A thread's new call; one it still had open is closed as died first.
    pub(crate) fn trapped(&mut self, pid: u32, nr: u64) {
        self.leave(pid);
        let seq = self.stamp();
        self.open.push((pid, seq, nr));
    }

    pub(crate) fn holds(&self, pid: u32) -> bool {
        self.open.iter().any(|c| c.0 == pid)
    }

    fn close(&mut self, pid: u32, nr: u64, what: u32, ret: u64) {
        let Some(at) = self.open.iter().position(|c| c.0 == pid && c.2 == nr) else {
            return;
        };
        let (_, seq, nr) = self.open.swap_remove(at);
        self.absorb(&Row { seq, pid, what, nr, ret });
    }

    /// The supervisor's answer to the call `pid` parked with `nr`.
    pub(super) fn answered(&mut self, pid: u32, nr: u64, answer: Answer) {
        let (what, ret) = match answer {
            Answer::Value(v) => (VALUE, v),
            Answer::Execed => (EXECED, 0),
            Answer::Deliver(_) => (DELIVER, 0),
            Answer::Sigreturn(_) => (SIGRETURN, 0),
        };
        self.close(pid, nr, what, ret);
    }

    /// The call could not be parked: the guest got ENOENT.
    pub(crate) fn park_failed(&mut self, pid: u32, nr: u64) {
        self.close(pid, nr, ABANDONED, ENOENT);
    }

    /// Every open call of `pid` closed as died while parked.
    pub(crate) fn leave(&mut self, pid: u32) {
        while let Some(&(_, _, nr)) = self.open.iter().find(|c| c.0 == pid) {
            self.close(pid, nr, DIED, 0);
        }
    }

    /// Whatever is still open when the family seals, in trap order.
    pub(crate) fn leave_all(&mut self) {
        self.open.sort_unstable_by_key(|c| c.1);
        while let Some(&(pid, ..)) = self.open.first() {
            self.leave(pid);
        }
    }
}

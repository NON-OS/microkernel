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

//! One parked wait4 or waitid tried against its caller's children: the first
//! ended child that fits is reported, or the call answers at once with none,
//! or it stays parked until a child ends.

use super::family::Family;
use super::family_wait_report::report;
use crate::linux::abi::errno;
use crate::linux::call::{WALL, WCLONE, WEXITED, WNOHANG, WNOWAIT};
use crate::linux::guest::sigwaits::{ChildWait, Which};

impl Family {
    pub(super) fn try_wait(&mut self, i: usize, w: &ChildWait) -> Option<u64> {
        let group_of = |pid: u32| {
            let live = self.guests.iter().find(|g| g.pid == pid).map(|g| g.pgid);
            live.or_else(|| {
                self.guests[i].signals.kid_groups.iter().find(|k| k.0 == pid).map(|k| k.1)
            })
        };
        let p = &self.guests[i];
        let fits = |pid: u32| {
            let clone = p.signals.clone_kids.contains(&pid);
            let kind = w.options & WALL != 0 || (w.options & WCLONE != 0) == clone;
            kind && match w.which {
                Which::Any => true,
                Which::Pid(want) => pid == want,
                Which::Group(g) => group_of(pid) == Some(g),
            }
        };
        let exits = w.options & WEXITED != 0;
        let done = p.ended.iter().position(|(pid, _)| exits && fits(*pid));
        let any = p.children.iter().any(|c| fits(*c));
        let p = &mut self.guests[i];
        let Some(at) = done else {
            return match (any, w.options & WNOHANG != 0) {
                (false, _) => Some(report(p, w, None, errno::fail(errno::ECHILD))),
                (true, true) => Some(report(p, w, None, 0)),
                (true, false) => None,
            };
        };
        let (pid, status) = p.ended[at];
        if w.options & WNOWAIT == 0 {
            p.ended.remove(at);
            p.children.retain(|c| *c != pid);
            p.signals.clone_kids.retain(|c| *c != pid);
            p.signals.kid_groups.retain(|k| k.0 != pid);
        }
        let shown = u64::from(crate::linux::serve::guest_pid(pid));
        Some(report(p, w, Some((pid, status)), if w.waitid { 0 } else { shown }))
    }
}

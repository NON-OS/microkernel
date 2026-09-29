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

//! A signal from the outbox reaches every process its target names, and its
//! sender is answered 0, or ESRCH when it named no one, as kill answers; an
//! ended child not yet waited for counts, as a zombie does on Linux.

use super::family::Family;
use super::family_wait::answer;
use crate::linux::abi::errno;
use crate::linux::guest::sigwaits::{Outbound, Target};
use crate::linux::guest::Guest;

impl Family {
    pub(super) fn route_outbox(&mut self) {
        let mut out = alloc::vec::Vec::new();
        for g in self.guests.iter_mut() {
            out.extend(core::mem::take(&mut g.signals.outbox).into_iter().map(|o| (g.pid, o)));
        }
        for (sender, o) in out {
            let zombie = |pid: u32| self.guests.iter().any(|g| g.ended.iter().any(|e| e.0 == pid));
            let mut reached =
                matches!(o.to, Target::Process(p) | Target::Thread(_, p) if zombie(p));
            for g in self.guests.iter_mut() {
                if let Some(t) = taker(g, sender, &o) {
                    reached = true;
                    if o.info.signo != 0 && g.exited.is_none() {
                        let _ = g.signals.raise(t, o.info);
                    }
                }
            }
            let value = if reached { 0 } else { errno::fail(errno::ESRCH) };
            if let Some(g) = self.guests.iter_mut().find(|g| g.owns(o.from)) {
                answer(g, o.from, value);
            }
        }
    }
}

/// The thread of `g` a signal is for, 0 for the whole process, or None.
fn taker(g: &Guest, sender: u32, o: &Outbound) -> Option<u32> {
    match o.to {
        Target::Process(p) => g.owns(p).then_some(0),
        Target::Thread(tgid, t) => (g.owns(t) && (tgid == 0 || tgid == g.pid)).then_some(t),
        Target::Group(pg) => (g.pgid == pg).then_some(0),
        Target::All => (g.pid != sender).then_some(0),
    }
}

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

//! Settling the calls parked in `waits`.

use core::mem;

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use super::waits::attempt;
use crate::linux::abi::nr;
use crate::linux::call::now_ms;
use crate::linux::guest::{Blocked, Guest, Kind};

const CLOCK_MONOTONIC: u64 = 1;
/// How often a wait on a socket or a timer is looked at again. Their
/// readiness changes with no call for the family to answer.
const TICK_MS: u64 = 10;

impl Family {
    /// Try every parked call again: answer the ones that can complete now,
    /// answer the ones whose deadline has passed with nothing ready, and
    /// leave the rest parked.
    pub fn settle_waits(&mut self) {
        let Some(now) = now_ms(CLOCK_MONOTONIC) else {
            return;
        };
        for i in 0..self.guests.len() {
            if self.guests[i].blocked.is_empty() {
                continue;
            }
            self.lend(i);
            let g = &mut self.guests[i];
            for wait in mem::take(&mut g.blocked) {
                let value = match attempt(g, &wait) {
                    Some(v) => v,
                    None if wait.deadline.is_some_and(|d| d <= now) => 0,
                    None => {
                        g.blocked.push(wait);
                        continue;
                    }
                };
                // A caught signal for this thread is delivered in place of the reply.
                if !super::deliver::maybe_deliver(g, wait.tid, value) {
                    let _ = mk_foreign_reply(wait.tid, value);
                }
            }
            self.take_back(i);
        }
    }

    /// Milliseconds until a parked call is due to be looked at again.
    pub(super) fn next_wait_ms(&self, now: u64) -> Option<u64> {
        let due = self.guests.iter().flat_map(|g| g.blocked.iter());
        let deadline = due.filter_map(|w| w.deadline).map(|d| d.saturating_sub(now)).min();
        let outside = self.guests.iter().any(|g| g.blocked.iter().any(|w| watches_outside(g, w)));
        [deadline, outside.then_some(TICK_MS)].into_iter().flatten().min()
    }
}

/// An epoll wait watching a socket or a timer.
fn watches_outside(guest: &Guest, wait: &Blocked) -> bool {
    if wait.nr == nr::READ || wait.nr == nr::WRITE {
        return false;
    }
    let Some(list) = guest.fds.get(wait.args[0] as usize) else {
        return false;
    };
    list.watch.iter().any(|w| {
        matches!(guest.fds.get(w.fd as usize).map(|f| f.kind), Some(Kind::Socket | Kind::Timer))
    })
}

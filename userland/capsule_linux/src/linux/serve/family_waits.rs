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
use super::waits::{attempt, expire};
use super::waits_fds::watched;
use crate::linux::call::now_ms;
use crate::linux::guest::Kind;

const CLOCK_MONOTONIC: u64 = 1;
/// How often a wait on a socket is looked at again: its readiness changes
/// with no call for the family to answer. A timer is looked at when it fires.
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
                    None if wait.deadline.is_some_and(|d| d <= now) => expire(g, &wait),
                    None => {
                        g.blocked.push(wait);
                        continue;
                    }
                };
                // A caught signal for this thread is delivered in place of the reply.
                super::deliver_pipe::broken_pipe(g, wait.tid, wait.nr, wait.args, value);
                if !super::deliver::maybe_deliver(g, wait.tid, value) {
                    let _ = mk_foreign_reply(wait.tid, value);
                }
            }
            self.take_back(i);
        }
    }

    /// Milliseconds until a parked call is due to be looked at again: its
    /// deadline, a timer it watches firing, or the next look at a socket.
    pub(super) fn next_wait_ms(&self, now: u64) -> Option<u64> {
        let mut soonest: Option<u64> = None;
        let mut keep = |at: u64| soonest = Some(soonest.map_or(at, |s| s.min(at)));
        for g in self.guests.iter() {
            for wait in g.blocked.iter() {
                if let Some(d) = wait.deadline {
                    keep(d.saturating_sub(now));
                }
                for fd in watched(g, wait) {
                    match g.fds.get(fd as usize) {
                        Some(f) if f.kind == Kind::Socket => keep(TICK_MS),
                        Some(f) if f.kind == Kind::Timer => {
                            // One that has already fired was seen by the last look.
                            let due = self.timers.get(f.handle as usize).map_or(0, |t| t.due);
                            if due > now {
                                keep(due - now);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        soonest
    }
}

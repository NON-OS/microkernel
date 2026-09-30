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

//! When the family next looks at its parked calls: the soonest deadline,
//! timer or tick among them.

use super::family::Family;
use super::waits_fds::watched;
use crate::linux::guest::Kind;
use crate::linux::net::outside;

/// How often a wait on a stream net.sockets holds, or on the console, is
/// looked at again: what the terminal typed and room in the launcher's
/// inbox arrive with no answer to mark them. A family socket changes only
/// in an answer, and a timer is looked at when it fires.
const TICK_MS: u64 = 10;

impl Family {
    /// Milliseconds until a parked call is due to be looked at again: its
    /// deadline, a timer it watches firing, or the next look at a socket or
    /// the console. None when nothing parked needs a look before an answer.
    pub(super) fn next_wait_ms(&self, now: u64) -> Option<u64> {
        let mut soonest: Option<u64> = None;
        let mut keep = |at: u64| soonest = Some(soonest.map_or(at, |s| s.min(at)));
        for g in self.guests.iter() {
            for wait in g.blocked.iter() {
                if let Some(d) = wait.deadline {
                    keep(d.saturating_sub(now));
                }
                super::waits_sock::ticks(g, wait).then(|| keep(TICK_MS));
                for fd in watched(g, wait) {
                    match g.fds.get(fd as usize) {
                        Some(f) if f.kind == Kind::Socket && outside(f.handle) => keep(TICK_MS),
                        Some(f) if matches!(f.kind, Kind::Stdin | Kind::Stdout | Kind::Stderr) => {
                            keep(TICK_MS)
                        }
                        Some(f) if f.kind == Kind::Timer => {
                            /*
                             * One that has already fired was seen by the last look.
                             */
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

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

//! Answering sleepers whose deadline has passed.

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use crate::linux::call::now_ms;

const CLOCK_MONOTONIC: u64 = 1;

impl Family {
    /// Wake every thread whose sleep is over.
    pub fn settle_sleeps(&mut self) {
        let Some(now) = now_ms(CLOCK_MONOTONIC) else {
            return;
        };
        for g in self.guests.iter_mut() {
            g.sleepers.retain(|&(deadline, tid)| {
                if deadline > now {
                    return true;
                }
                let _ = mk_foreign_reply(tid, 0);
                false
            });
        }
    }

    /// Milliseconds until the nearest sleeper, futex timeout or parked wait
    /// is due, if any.
    pub fn next_wake_ms(&self) -> Option<u64> {
        let now = now_ms(CLOCK_MONOTONIC)?;
        let sleeper = self
            .guests
            .iter()
            .flat_map(|g| g.sleepers.iter().chain(g.futex_until.iter()))
            .map(|&(d, _)| d.saturating_sub(now))
            .min();
        [sleeper, self.next_wait_ms(now)].into_iter().flatten().min()
    }
}

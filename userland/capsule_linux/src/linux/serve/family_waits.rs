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

//! Settling the calls parked in `waits`; when to look again is
//! `family_waits_next`.

use core::mem;

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use super::waits_try::{attempt, expire};
use crate::linux::call::now_ms;

const CLOCK_MONOTONIC: u64 = 1;

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
            for mut wait in mem::take(&mut g.blocked) {
                let value = match attempt(g, &mut wait) {
                    Some(v) => v,
                    None if wait.deadline.is_some_and(|d| d <= now) => expire(g, &wait),
                    None => {
                        g.blocked.push(wait);
                        continue;
                    }
                };
                /* A wait that answers puts back the mask it waited under. */
                super::waits_mask::leave(g, wait.tid, wait.nr);
                // A caught signal for this thread is delivered in place of the reply.
                super::deliver_pipe::broken_pipe(g, wait.tid, wait.nr, wait.args, value);
                if !super::deliver::maybe_deliver(g, wait.tid, value) {
                    let _ = mk_foreign_reply(wait.tid, value);
                }
            }
            self.take_back(i);
        }
    }
}

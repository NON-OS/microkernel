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

//! Ending futex waits whose timeout has passed.

use alloc::vec::Vec;

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use crate::linux::abi::errno;
use crate::linux::call::now_ms;

const CLOCK_MONOTONIC: u64 = 1;

impl Family {
    /// Answer ETIMEDOUT to every futex waiter whose deadline has passed and
    /// that nothing woke first.
    pub fn settle_futex(&mut self) {
        let Some(now) = now_ms(CLOCK_MONOTONIC) else {
            return;
        };
        for g in self.guests.iter_mut() {
            let due: Vec<u32> =
                g.futex_until.iter().filter(|&&(d, _)| d <= now).map(|&(_, t)| t).collect();
            g.futex_until.retain(|&(d, _)| d > now);
            for tid in due {
                let Some(at) = g.waits.iter().position(|&(w, _)| w == tid) else {
                    continue;
                };
                g.waits.remove(at);
                let value = errno::fail(errno::ETIMEDOUT);
                // A caught signal for this thread is delivered in place of the reply.
                if !super::deliver::maybe_deliver(g, tid, value) {
                    let _ = mk_foreign_reply(tid, value);
                }
            }
        }
    }
}

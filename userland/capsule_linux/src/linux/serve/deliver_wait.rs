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

//! Signals that reach a process whose threads are parked, not returning.
//! Run after every answer: a sigtimedwait takes a signal in its set; an
//! uncaught signal's default acts at once, as Linux's does when it is sent;
//! and a caught signal ends the wait of a thread that does not block it, with
//! EINTR, or restarts the call under SA_RESTART where Linux restarts it.

use super::deliver_interrupt::interrupt;
use super::deliver_say::stop_unserved;
use super::deliver_sigwait::taken_by_sigtimedwait;
use crate::linux::call::killed;
use crate::linux::guest::sigdefault::{default_of, Default};
use crate::linux::guest::sigstate::bit;
use crate::linux::guest::Guest;

pub fn settle(guest: &mut Guest) {
    taken_by_sigtimedwait(guest);
    for (tid, signo) in guest.signals.queued() {
        if guest.exited.is_some() {
            return;
        }
        let act = guest.signals.action(signo as usize).unwrap_or_default();
        let takers: alloc::vec::Vec<u32> = match tid {
            0 => guest.live_threads(),
            t => alloc::vec![t],
        };
        let free = |g: &Guest, t: &u32| g.signals.blocked(*t) & bit(signo) == 0;
        if !act.catches() {
            if takers.iter().any(|t| free(guest, t)) {
                let _ = guest.signals.take(takers[0], bit(signo));
                match (act.ignores(), default_of(signo)) {
                    (false, Default::Terminate) => killed(guest, signo),
                    (false, Default::Stop) => stop_unserved(signo),
                    _ => {}
                }
            }
            continue;
        }
        let parked = takers.iter().find(|t| free(guest, t) && guest.parked(**t).is_some());
        if let Some(&t) = parked {
            if let Some(info) = guest.signals.take(t, bit(signo)) {
                interrupt(guest, t, info);
            }
        }
    }
}

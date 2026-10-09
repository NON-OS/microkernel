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

//! A caught signal ending a parked thread's wait: the handler is entered over
//! the parked call, whose answer is EINTR or a restart as deliver_restart
//! decides, and a relative sleep writes what it had left first.

use nonos_libc::{mk_foreign_context, mk_foreign_reply, ForeignRegs};

use super::deliver_enter::enter;
use super::deliver_rem::write_rem;
use super::deliver_restart::rewind;
use crate::linux::guest::siginfo::SigInfo;
use crate::linux::guest::Guest;
use crate::linux::net::sock;

/// End `tid`'s wait with a handler entered over its parked call.
pub fn interrupt(guest: &mut Guest, tid: u32, info: SigInfo) {
    let mut regs: ForeignRegs = [0; 18];
    if mk_foreign_context(tid, &mut regs) != 0 {
        let _ = guest.signals.raise(tid, info);
        return;
    }
    /*
     * Read before the wait is dropped: what a pipe write or a socket call
     * had already moved, and whether the wait had a limit of its own.
     */
    let wait = guest.blocked.iter().find(|w| w.tid == tid);
    let timed = wait.is_some_and(|w| w.deadline.is_some());
    let moved = wait.map_or(0, |w| w.done).max(sock::progress(tid) as u64);
    let Some(parked) = guest.leave_waits(tid) else {
        let _ = guest.signals.raise(tid, info);
        return;
    };
    sock::set_progress(tid, 0);
    let act = guest.signals.action(info.signo as usize).unwrap_or_default();
    write_rem(guest, &regs, parked);
    rewind(&mut regs, act.flags, moved, timed);
    if !enter(guest, tid, &regs, info) {
        /*
         * Out of its wait with no handler entered: it must still be answered,
         * under its own mask again, since no frame will put that back.
         */
        if let Some(old) = guest.signals.thread(tid).saved.take() {
            guest.signals.set_blocked(tid, old);
        }
        let _ =
            mk_foreign_reply(tid, crate::linux::abi::errno::fail(crate::linux::abi::errno::EINTR));
    }
}

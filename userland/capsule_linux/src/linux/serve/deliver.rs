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

//! A thread taking the signals it may take. A caught one enters its handler
//! through Linux's rt_sigframe; an uncaught one does what its default says,
//! and a default of ending the process ends all of it. A thread takes them
//! when it returns from a call, with the call's value in rax (maybe_deliver);
//! when a signal ends the wait it is parked in (deliver_wait); and when the
//! kernel stops it running, on the registers it stopped with (deliver_on).

use nonos_libc::{mk_foreign_context, ForeignRegs};

use super::deliver_enter::enter;
use super::deliver_say::stop_unserved;
use crate::linux::call::killed;
use crate::linux::guest::sigdefault::{default_of, Default};
use crate::linux::guest::Guest;

/// rax in the register word order.
const RAX: usize = 13;

/// True when the thread was answered, into a handler or by its process
/// ending, so the caller must not also reply.
pub fn maybe_deliver(guest: &mut Guest, tid: u32, reply: u64) -> bool {
    if guest.signals.pending_for(tid) & !guest.signals.blocked(tid) == 0 {
        return false;
    }
    let mut regs: ForeignRegs = [0; 18];
    if mk_foreign_context(tid, &mut regs) != 0 {
        return false;
    }
    regs[RAX] = reply;
    deliver_on(guest, tid, regs)
}

/// Take what `tid` may take and act on it, over `regs` as they stand. For a
/// frame the kernel stopped while running as much as for a returning call.
pub fn deliver_on(guest: &mut Guest, tid: u32, regs: ForeignRegs) -> bool {
    loop {
        let allow = !guest.signals.blocked(tid);
        let Some(info) = guest.signals.take(tid, allow) else {
            return false;
        };
        let act = guest.signals.action(info.signo as usize).unwrap_or_default();
        if act.catches() {
            return enter(guest, tid, &regs, info);
        }
        if act.ignores() {
            continue;
        }
        match default_of(info.signo) {
            Default::Terminate => {
                killed(guest, info.signo);
                return true;
            }
            Default::Stop => stop_unserved(info.signo),
            Default::Ignore | Default::Continue => {}
        }
    }
}

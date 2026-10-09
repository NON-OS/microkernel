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

//! Entering a handler: the frame on the thread's stack, or at the top of its
//! alternate stack when the handler asked for SA_ONSTACK and the thread is not
//! already on it; the mask while the handler runs is the thread's own, the
//! handler's sa_mask, and the signal itself unless SA_NODEFER.

use nonos_libc::{mk_foreign_signal, ForeignRegs, SIGNAL_DELIVER};

use super::deliver_say::say;
use super::deliver_stack::placement;
use crate::linux::call::killed;
use crate::linux::call::sigframe::Entry;
use crate::linux::call::sigframe_build::build;
use crate::linux::guest::sigalt::entered;
use crate::linux::guest::siginfo::SigInfo;
use crate::linux::guest::sigstate::{bit, SigAction, SA_NODEFER, SA_RESETHAND, SIGSEGV};
use crate::linux::guest::Guest;

const RSP: usize = 15;

/// True once the thread is answered: in its handler, or its process ended
/// because no frame could be written, which Linux answers with SIGSEGV.
pub fn enter(guest: &mut Guest, tid: u32, regs: &ForeignRegs, info: SigInfo) -> bool {
    let act = guest.signals.action(info.signo as usize).unwrap_or_default();
    let t = *guest.signals.thread(tid);
    let (alt_top, stack) = placement(t.alt, act.flags, regs[RSP]);
    let saved = t.saved.unwrap_or(t.blocked);
    let bytes = info.bytes();
    let entry = Entry {
        handler: act.handler,
        restorer: act.restorer,
        signum: u32::from(info.signo),
        blocked: saved,
        alt_top,
        stack,
        info: &bytes,
    };
    let Some((at, buf, into)) = build(regs, &entry).filter(|_| act.restorer != 0) else {
        killed(guest, SIGSEGV);
        return true;
    };
    if guest.write(at, &buf) < buf.len() as i64 {
        killed(guest, SIGSEGV);
        return true;
    }
    if mk_foreign_signal(tid, &into, SIGNAL_DELIVER) != 0 {
        /* Not parked after all: the signal waits for its next return. */
        let _ = guest.signals.raise(tid, info);
        return false;
    }
    let during =
        t.blocked | act.mask | if act.flags & SA_NODEFER != 0 { 0 } else { bit(info.signo) };
    guest.signals.set_blocked(tid, during);
    let thread = guest.signals.thread(tid);
    thread.saved = None;
    /* SS_AUTODISARM: the stack is off until this handler's frame returns it. */
    thread.alt = entered(thread.alt);
    if act.flags & SA_RESETHAND != 0 {
        guest.signals.set(info.signo as usize, SigAction::default());
    }
    say(tid, info.signo, act.handler);
    true
}

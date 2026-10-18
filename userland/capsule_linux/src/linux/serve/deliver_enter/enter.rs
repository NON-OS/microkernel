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

//! A signal's frame written to the thread's stack and the thread sent into
//! its handler; one whose frame cannot be written ends its process.

use nonos_libc::{mk_foreign_signal, ForeignRegs, SIGNAL_DELIVER};

use super::super::deliver_stack::placement;
use super::handler::in_handler;
use crate::linux::call::killed;
use crate::linux::call::sigframe::Entry;
use crate::linux::call::sigframe_build::build;
use crate::linux::guest::siginfo::SigInfo;
use crate::linux::guest::sigstate::SIGSEGV;
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
    in_handler(guest, tid, t.blocked, &act, info.signo);
    true
}

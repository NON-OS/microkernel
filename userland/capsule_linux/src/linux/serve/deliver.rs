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

//! Delivering a caught signal to a thread being answered: one returning from
//! a syscall is entered with that call's return value in rax, so the program
//! sees it when the handler returns through rt_sigreturn; one stopped at a
//! tick (family_interrupt) keeps its own rax. A thread answered without a
//! handler is marked for its next tick instead (`Guest::rearm`).

use nonos_libc::{mk_foreign_context, mk_foreign_signal, ForeignRegs, SIGNAL_DELIVER};

use crate::linux::call::sigframe::build;
use crate::linux::guest::Guest;

/// rax in the register word order.
const RAX: usize = 13;
/// `sa_flags`: enter the handler on the thread's alternate stack.
const SA_ONSTACK: u64 = 0x0800_0000;

/// True when a handler was entered, so the caller must not also reply.
pub fn maybe_deliver(guest: &mut Guest, tid: u32, reply: u64) -> bool {
    let Some((signum, act)) = guest.signals.take_caught(tid) else {
        return false;
    };
    let mut regs: ForeignRegs = [0; 18];
    let alt = guest.signals.stack(tid).map(|s| (s.sp, s.size));
    let onstack = act.flags & SA_ONSTACK != 0;
    let built = (mk_foreign_context(tid, &mut regs) == 0).then(|| {
        regs[RAX] = reply;
        build(&regs, act.handler, act.restorer, u32::from(signum), 0, alt, onstack)
    });
    let Some(Some((_, buf, enter))) = built else {
        // Could not read the thread or shape a frame: keep the signal pending.
        guest.signals.raise(tid, signum);
        return false;
    };
    if guest.write(enter[15], &buf) < buf.len() as i64 {
        guest.signals.raise(tid, signum);
        return false;
    }
    if mk_foreign_signal(tid, &enter, SIGNAL_DELIVER) != 0 {
        guest.signals.raise(tid, signum);
        return false;
    }
    say(tid, signum, act.handler);
    true
}

/// A line for the log, not for the terminal's window: how a run is going.
fn say(tid: u32, signum: u8, handler: u64) {
    let line = alloc::format!("[LINUX] signal {signum} to tid {tid}, handler {handler:#x}\n");
    crate::linux::say::note(line.as_bytes());
}

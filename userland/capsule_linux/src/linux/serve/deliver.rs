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

//! Delivering a caught signal to the thread returning from a syscall. The
//! handler is entered with the interrupted syscall's return value already in
//! rax, so when it returns through rt_sigreturn the program sees that value.
//! Only the trapping thread is delivered to here; a signal raised against a
//! thread parked elsewhere waits in the queue until that thread next traps.

use nonos_libc::{mk_foreign_context, mk_foreign_signal, ForeignRegs, SIGNAL_DELIVER};

use crate::linux::call::sigframe::{Entry, INFO_LEN};
use crate::linux::call::sigframe_build::build;
use crate::linux::guest::Guest;

/// rax in the register word order.
const RAX: usize = 13;

/// True when a handler was entered, so the caller must not also reply.
pub fn maybe_deliver(guest: &mut Guest, tid: u32, reply: u64) -> bool {
    let Some((signum, act)) = guest.signals.take_caught(tid) else {
        return false;
    };
    let mut regs: ForeignRegs = [0; 18];
    let built = (mk_foreign_context(tid, &mut regs) == 0).then(|| {
        regs[RAX] = reply;
        /* The siginfo carries the number; uc_stack is SS_DISABLE, no alternate stack. */
        let mut info = [0u8; INFO_LEN];
        info[..4].copy_from_slice(&i32::from(signum).to_le_bytes());
        let entry = Entry {
            handler: act.handler,
            restorer: act.restorer,
            signum: u32::from(signum),
            blocked: 0,
            alt_top: None,
            stack: [0, 2, 0],
            info: &info,
        };
        build(&regs, &entry)
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

fn say(tid: u32, signum: u8, handler: u64) {
    let line = alloc::format!("[LINUX] signal {signum} to tid {tid}, handler {handler:#x}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

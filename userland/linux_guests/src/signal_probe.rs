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

//! Signal delivery, from the guest's side: install a handler with its own
//! restorer, raise a signal at this thread, and see the handler run and
//! return. This is the one probe that wants success, not a refusal: it is
//! how a program's timeouts, its Ctrl+C, and a Go runtime's preemption work.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::sys::{call, out, GETPID};

const RT_SIGACTION: u64 = 13;
const KILL: u64 = 62;
const SIGUSR1: u64 = 10;
const SA_RESTORER: u64 = 0x0400_0000;
const SA_RESTART: u64 = 0x1000_0000;

static RAN: AtomicU32 = AtomicU32::new(0);

// The restorer a handler returns through: rt_sigreturn, nothing else.
core::arch::global_asm!(".globl nonos_sigrestore", "nonos_sigrestore:", "mov rax, 15", "syscall");
extern "C" {
    fn nonos_sigrestore();
}

extern "C" fn handler(_sig: i32) {
    RAN.store(1, Ordering::SeqCst);
    // A handler making a syscall traps and is answered like any other: it must
    // return to the handler, not somewhere else.
    out(b"[GUEST] signal handler ran\n");
}

/// True when the handler ran and control returned past the raise.
pub fn scan() -> bool {
    let act: [u64; 4] = [
        handler as *const () as u64,
        SA_RESTORER | SA_RESTART,
        nonos_sigrestore as *const () as u64,
        0,
    ];
    if call(RT_SIGACTION, [SIGUSR1, act.as_ptr() as u64, 0, 8, 0, 0]) < 0 {
        out(b"[GUEST] signal FAIL: rt_sigaction refused\n");
        return false;
    }
    let pid = call(GETPID, [0; 6]) as u64;
    if call(KILL, [pid, SIGUSR1, 0, 0, 0, 0]) < 0 {
        out(b"[GUEST] signal FAIL: raise refused\n");
        return false;
    }
    let ran = RAN.load(Ordering::SeqCst) == 1;
    out(match ran {
        true => b"[GUEST] signal PASS: handler ran and returned\n".as_slice(),
        false => b"[GUEST] signal FAIL: handler did not run\n".as_slice(),
    });
    ran
}

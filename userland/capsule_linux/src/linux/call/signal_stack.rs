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

//! `sigaltstack`: a thread names, reads or disables its alternate signal
//! stack, with Linux's answers (`do_sigaltstack` in `kernel/signal.c`): the
//! old setting is what was there before this call, a thread running on its
//! alternate stack may not change it (EPERM), a stack under MINSIGSTKSZ is
//! ENOMEM, and a mode other than 0, SS_ONSTACK or SS_DISABLE is EINVAL.

use nonos_libc::{mk_foreign_context, ForeignRegs};

use crate::linux::abi::errno;
use crate::linux::guest::sigstack::{AltStack, SS_DISABLE, SS_ONSTACK};
use crate::linux::guest::sigstack_t::{decode, encode, SS_AUTODISARM, STACK_T};
use crate::linux::guest::Guest;

/// x86-64's MINSIGSTKSZ, the smallest stack the kernel accepts.
const MINSIGSTKSZ: u64 = 2048;
const RSP: usize = 15;

pub fn sigaltstack(guest: &mut Guest, tid: u32, new: u64, old: u64) -> u64 {
    let wanted = match new {
        0 => None,
        at => match guest.read(at, STACK_T) {
            Some(raw) => Some(decode(&raw)),
            None => return errno::fail(errno::EFAULT),
        },
    };
    let rsp = rsp_of(tid);
    let now = guest.signals.stack(tid);
    let before = encode(now, rsp);
    if let Some((sp, flags, size)) = wanted {
        if now.is_some_and(|s| s.holds(rsp)) {
            return errno::fail(errno::EPERM);
        }
        if flags & SS_AUTODISARM != 0 {
            crate::linux::start::say(b"[LINUX] unserved sigaltstack SS_AUTODISARM\n");
            return errno::fail(errno::EINVAL);
        }
        match flags {
            SS_DISABLE => guest.signals.set_stack(tid, None),
            0 | SS_ONSTACK if size < MINSIGSTKSZ => return errno::fail(errno::ENOMEM),
            0 | SS_ONSTACK => guest.signals.set_stack(tid, Some(AltStack { sp, size })),
            _ => return errno::fail(errno::EINVAL),
        }
    }
    if old != 0 && guest.write(old, &before) < STACK_T as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

/// The thread's stack pointer where it made the call.
fn rsp_of(tid: u32) -> u64 {
    let mut regs: ForeignRegs = [0; 18];
    if mk_foreign_context(tid, &mut regs) == 0 {
        regs[RSP]
    } else {
        0
    }
}

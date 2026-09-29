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

//! `rt_sigreturn`: a thread leaving a signal handler. Its rsp points at the
//! ucontext the frame carried, so the saved registers, the mask and the
//! alternate stack are read back from there and the kernel resumes the thread
//! into them. Nothing is replied: the thread is no longer in the syscall, it
//! is back where the signal interrupted. A frame that cannot be read ends the
//! process with SIGSEGV, as Linux's does.

use nonos_libc::{mk_foreign_context, mk_foreign_signal, ForeignRegs, SIGNAL_RETURN};

use super::sigframe::{SIGMASK_OFF, WORDS};
use super::sigframe_read::{returned, returned_mask, returned_stack};
use crate::linux::guest::sigstate::SIGSEGV;
use crate::linux::guest::sigthread::{SS_DISABLE, SS_ONSTACK};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// rsp in the register word order.
const RSP: usize = 15;

pub fn rt_sigreturn(guest: &mut Guest, tid: u32) -> Answer {
    let mut regs: ForeignRegs = [0; WORDS];
    if mk_foreign_context(tid, &mut regs) != 0 {
        return Answer::Park;
    }
    /* The trampoline's `ret` left rsp at the ucontext; the mask ends it. */
    let read = guest.read(regs[RSP], SIGMASK_OFF + 8);
    let back = read
        .as_deref()
        .and_then(|uc| Some((returned(uc)?, returned_mask(uc)?, returned_stack(uc)?)));
    let Some((restored, mask, stack)) = back else {
        super::killed(guest, SIGSEGV);
        return Answer::Park;
    };
    guest.signals.set_blocked(tid, mask);
    let t = guest.signals.thread(tid);
    let on_now = t.alt[1] & SS_DISABLE == 0 && restored[RSP].wrapping_sub(t.alt[0]) < t.alt[2];
    if !on_now {
        t.alt = match stack[1] & SS_DISABLE {
            0 => [stack[0], stack[1] & !SS_ONSTACK, stack[2]],
            _ => [0, SS_DISABLE, 0],
        };
    }
    if mk_foreign_signal(tid, &restored, SIGNAL_RETURN) != 0 {
        super::killed(guest, SIGSEGV);
    }
    Answer::Park
}

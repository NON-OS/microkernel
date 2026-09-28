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
//! ucontext the frame carried, so the saved registers are read back from
//! there and the kernel resumes the thread into them. Nothing is replied: the
//! thread is no longer in the syscall, it is back where the signal interrupted.

use nonos_libc::{mk_foreign_context, mk_foreign_signal, ForeignRegs, SIGNAL_RETURN};

use super::sigframe::{SIGCONTEXT_OFF, WORDS};
use super::sigframe_read::returned;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// rsp in the register word order.
const RSP: usize = 15;

pub fn rt_sigreturn(guest: &Guest, tid: u32) -> Answer {
    let mut regs: ForeignRegs = [0; WORDS];
    if mk_foreign_context(tid, &mut regs) != 0 {
        return Answer::Park;
    }
    // The trampoline's `ret` left rsp at the ucontext; the sigcontext follows.
    let want = SIGCONTEXT_OFF + WORDS * 8;
    let Some(bytes) = guest.read(regs[RSP], want) else {
        return Answer::Park;
    };
    let Some(restored) = returned(&bytes) else {
        return Answer::Park;
    };
    let _ = mk_foreign_signal(tid, &restored, SIGNAL_RETURN);
    Answer::Park
}

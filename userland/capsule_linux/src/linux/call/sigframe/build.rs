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

//! The bytes of a signal frame, and the registers that enter the handler.

use alloc::vec::Vec;

use super::layout::{put, FRAME_SIZE, INFO_OFF, SIGCONTEXT_OFF, SIGMASK_OFF, STACK_OFF, UC_OFF};
use super::layout::{SS_DISABLE, SS_ONSTACK, WORDS};
use super::place::{on_alt, place};

/// Where the frame lands, the bytes to write there, and the registers that
/// enter the handler. `alt` is the thread's alternate stack (base, size), and
/// `onstack` is the handler's SA_ONSTACK. `None` if the frame does not fit.
pub fn build(
    regs: &[u64; WORDS],
    handler: u64,
    restorer: u64,
    signum: u32,
    blocked: u64,
    alt: Option<(u64, u64)>,
    onstack: bool,
) -> Option<(u64, Vec<u8>, [u64; WORDS])> {
    let rsp = regs[15];
    let frame = place(rsp, alt, onstack)?;
    let mut buf = alloc::vec![0u8; FRAME_SIZE];
    put(&mut buf, 0, restorer);
    /* uc_stack: the alternate stack, flagged as sas_ss_flags gives it for rsp. */
    let (ss_sp, ss_size, ss_flags) = match alt {
        None => (0, 0, SS_DISABLE),
        Some((sp, size)) => (sp, size, if on_alt(alt, rsp) { SS_ONSTACK } else { 0 }),
    };
    put(&mut buf, UC_OFF + STACK_OFF, ss_sp);
    put(&mut buf, UC_OFF + STACK_OFF + 8, ss_flags);
    put(&mut buf, UC_OFF + STACK_OFF + 16, ss_size);
    let mc = UC_OFF + SIGCONTEXT_OFF;
    for (i, w) in regs.iter().enumerate() {
        put(&mut buf, mc + i * 8, *w);
    }
    put(&mut buf, UC_OFF + SIGMASK_OFF, blocked);
    /* siginfo: si_signo alone. */
    put(&mut buf, INFO_OFF, u64::from(signum));
    Some((frame, buf, entry(regs, frame, handler, signum)))
}

/// The handler's registers: rdi the signal, rsi the siginfo, rdx the
/// ucontext, rsp the frame and rip the handler. rflags is the interrupted
/// one, which the kernel masks; rax stays 0, and no vector register is set.
fn entry(regs: &[u64; WORDS], frame: u64, handler: u64, signum: u32) -> [u64; WORDS] {
    let mut out = [0u64; WORDS];
    out[8] = u64::from(signum);
    out[9] = frame + INFO_OFF as u64;
    out[12] = frame + UC_OFF as u64;
    out[15] = frame;
    out[16] = handler;
    out[17] = regs[17];
    out
}

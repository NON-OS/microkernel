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

//! The `rt_sigframe` x86-64 puts on a thread's stack to enter a signal handler,
//! and where to read it back on return. Pure, so the layout is checked without
//! a guest. It matches Linux `struct rt_sigframe`: pretcode u64, ucontext at
//! +8, siginfo at +312. Within the ucontext, `uc_stack` is at +16, the
//! sigcontext (`uc_mcontext`) at +40 and `uc_sigmask` at +296, as musl, glibc
//! and Go all read them; the sigcontext starts with the 18 words
//! `mk_foreign_context` uses, in that order. A handler that reads or edits its
//! context (Go's does, to preempt) finds each register where Linux puts it.

use alloc::vec::Vec;

pub const WORDS: usize = 18; // r8..r15,rdi,rsi,rbp,rbx,rdx,rax,rcx,rsp,rip,rflags
const FRAME_SIZE: usize = 440;
const UC_OFF: usize = 8;
pub const SIGCONTEXT_OFF: usize = 40; // uc_mcontext within the ucontext
pub const SIGMASK_OFF: usize = 296; // uc_sigmask within the ucontext
const INFO_OFF: usize = 312;
const REDZONE: u64 = 128; // the System V red zone below rsp

fn put(buf: &mut [u8], at: usize, v: u64) {
    buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
}
/// Where the frame lands, the bytes to write there, and the registers that
/// enter the handler. `None` if the stack is too low to hold a frame.
pub fn build(
    regs: &[u64; WORDS],
    handler: u64,
    restorer: u64,
    signum: u32,
    blocked: u64,
) -> Option<(u64, Vec<u8>, [u64; WORDS])> {
    // Below the red zone, 16-aligned, then down 8 so the handler sees rsp+8
    // aligned as a call would leave it.
    let frame =
        (regs[15].checked_sub(REDZONE)?.checked_sub(FRAME_SIZE as u64)? & !15u64).checked_sub(8)?;
    let mut buf = alloc::vec![0u8; FRAME_SIZE];
    put(&mut buf, 0, restorer);
    let mc = UC_OFF + SIGCONTEXT_OFF;
    for (i, w) in regs.iter().enumerate() {
        put(&mut buf, mc + i * 8, *w);
    }
    put(&mut buf, UC_OFF + SIGMASK_OFF, blocked);
    put(&mut buf, INFO_OFF, u64::from(signum)); // siginfo: si_signo
    let mut out = [0u64; WORDS];
    out[8] = u64::from(signum); // rdi
    out[9] = frame + INFO_OFF as u64; // rsi, &siginfo
    out[12] = frame + UC_OFF as u64; // rdx, &ucontext
    out[15] = frame; // rsp at the frame; rax stays 0, no vector registers
    out[16] = handler; // rip
    out[17] = regs[17]; // rflags, the kernel masks it
    Some((frame, buf, out))
}

/// The 18 words a returning frame carries, from the ucontext the guest's rsp
/// points at: the trampoline's `ret` left rsp there.
pub fn returned(uc: &[u8]) -> Option<[u64; WORDS]> {
    let mut out = [0u64; WORDS];
    for (i, slot) in out.iter_mut().enumerate() {
        let at = SIGCONTEXT_OFF + i * 8;
        *slot = u64::from_le_bytes(uc.get(at..at + 8)?.try_into().ok()?);
    }
    Some(out)
}

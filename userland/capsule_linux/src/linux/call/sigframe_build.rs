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

//! Building the `rt_sigframe` a handler is entered through: the bytes, where
//! on the stack they go, and the registers the handler starts with. Pure, so
//! the frame is checked without a guest.

use alloc::vec::Vec;

use super::sigframe::{Entry, FRAME_SIZE, INFO_OFF, OLDMASK_WORD, SIGCONTEXT_OFF};
use super::sigframe::{SIGMASK_OFF, STACK_OFF, UC_OFF, WORDS};

const REDZONE: u64 = 128; /* the System V red zone below rsp */
/* The flags a handler is entered with cleared: direction, resume, trap. */
const ENTRY_CLEARS: u64 = 0x400 | 0x1_0000 | 0x100;

fn put(buf: &mut [u8], at: usize, v: u64) {
    buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
}

/// Where the frame lands, the bytes to write there, and the registers that
/// enter the handler. `None` if the stack is too low to hold a frame.
pub fn build(regs: &[u64; WORDS], e: &Entry) -> Option<(u64, Vec<u8>, [u64; WORDS])> {
    /*
     * Below the red zone, or at the top of the alternate stack; 16-aligned,
     * then down 8 so the handler sees rsp+8 aligned as a call would leave it.
     */
    let top = match e.alt_top {
        Some(top) => top,
        None => regs[15].checked_sub(REDZONE)?,
    };
    let frame = (top.checked_sub(FRAME_SIZE as u64)? & !15u64).checked_sub(8)?;
    let mut buf = alloc::vec![0u8; FRAME_SIZE];
    put(&mut buf, 0, e.restorer);
    let (st, mc) = (UC_OFF + STACK_OFF, UC_OFF + SIGCONTEXT_OFF);
    put(&mut buf, st, e.stack[0]);
    buf[st + 8..st + 12].copy_from_slice(&(e.stack[1] as u32).to_le_bytes());
    put(&mut buf, st + 16, e.stack[2]);
    for (i, w) in regs.iter().enumerate() {
        put(&mut buf, mc + i * 8, *w);
    }
    put(&mut buf, mc + OLDMASK_WORD * 8, e.blocked);
    put(&mut buf, UC_OFF + SIGMASK_OFF, e.blocked);
    buf[INFO_OFF..].copy_from_slice(e.info);
    /* The rest of the registers are the interrupted ones, as Linux leaves them. */
    let mut out = *regs;
    out[8] = u64::from(e.signum); /* rdi */
    out[9] = frame + INFO_OFF as u64; /* rsi, &siginfo */
    out[12] = frame + UC_OFF as u64; /* rdx, &ucontext */
    out[13] = 0; /* rax: no vector registers passed */
    out[15] = frame;
    out[16] = e.handler;
    out[17] = regs[17] & !ENTRY_CLEARS;
    Some((frame, buf, out))
}

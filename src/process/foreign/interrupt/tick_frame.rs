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

//! The words the timer trampoline saves for a tick that interrupted user
//! mode, and the register file they hold.
//!
//! The trampoline pushes the fifteen general registers under the CPU's iretq
//! frame, which is the leading 160 bytes of `SavedUser` and nothing more. For
//! a tick from user mode that frame sits at the top of the kernel stack, so
//! the TLS words that follow it in `SavedUser` are not the thread's: they are
//! never read or written through the trampoline's pointer.

use core::mem::offset_of;

use crate::arch::context::SavedUser;

/// r15 down to rax, then rip, cs, rflags, rsp, ss.
pub const WORDS: usize = 20;

const _: () = assert!(offset_of!(SavedUser, r15) == 0);
const _: () = assert!(offset_of!(SavedUser, rax) == 14 * 8);
const _: () = assert!(offset_of!(SavedUser, rip) == 15 * 8);
const _: () = assert!(offset_of!(SavedUser, ss) == 19 * 8);

pub(super) fn to_user(w: &[u64; WORDS], fs_base: u64) -> SavedUser {
    SavedUser {
        r15: w[0],
        r14: w[1],
        r13: w[2],
        r12: w[3],
        r11: w[4],
        r10: w[5],
        r9: w[6],
        r8: w[7],
        rdi: w[8],
        rsi: w[9],
        rbp: w[10],
        rbx: w[11],
        rdx: w[12],
        rcx: w[13],
        rax: w[14],
        rip: w[15],
        cs: w[16],
        rflags: w[17],
        rsp: w[18],
        ss: w[19],
        fs_base,
        gs_base: 0,
    }
}

pub(super) fn to_words(c: &SavedUser) -> [u64; WORDS] {
    [
        c.r15, c.r14, c.r13, c.r12, c.r11, c.r10, c.r9, c.r8, c.rdi, c.rsi, c.rbp, c.rbx, c.rdx,
        c.rcx, c.rax, c.rip, c.cs, c.rflags, c.rsp, c.ss,
    ]
}

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

//! The `rt_sigframe` x86-64 puts on a thread's stack to enter a signal handler:
//! its layout, and what a handler is entered with (sigframe_build writes it,
//! sigframe_read reads it back). Pure, so the layout is checked against a
//! round trip without a guest. It matches Linux `struct rt_sigframe`:
//! pretcode u64, then the ucontext at +8 (uc_flags, uc_link, the 24-byte
//! uc_stack, the 256-byte sigcontext at +40, uc_sigmask at +296), then the
//! 128-byte siginfo at +312. The sigcontext starts with the 18 words
//! `mk_foreign_context` uses, in that order.

pub const WORDS: usize = 18; /* r8..r15,rdi,rsi,rbp,rbx,rdx,rax,rcx,rsp,rip,rflags */
pub const FRAME_SIZE: usize = 440;
pub const UC_OFF: usize = 8;
pub const STACK_OFF: usize = 16; /* uc_stack within the ucontext */
pub const SIGCONTEXT_OFF: usize = 40; /* uc_mcontext within the ucontext */
pub const OLDMASK_WORD: usize = 21; /* sigcontext.oldmask, after cs/gs/fs/ss, err, trapno */
pub const SIGMASK_OFF: usize = 296; /* uc_sigmask within the ucontext */
pub const INFO_OFF: usize = 312;
pub const INFO_LEN: usize = 128;

/// What a handler is entered with, beyond the registers it interrupts.
pub struct Entry<'a> {
    pub handler: u64,
    pub restorer: u64,
    pub signum: u32,
    /// The mask the thread had, restored by rt_sigreturn.
    pub blocked: u64,
    /// The top of the alternate stack when the frame goes there.
    pub alt_top: Option<u64>,
    /// uc_stack as Linux saves it: ss_sp, ss_flags, ss_size.
    pub stack: [u64; 3],
    pub info: &'a [u8; INFO_LEN],
}

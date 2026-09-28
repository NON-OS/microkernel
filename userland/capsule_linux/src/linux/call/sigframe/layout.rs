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

//! Where each part of Linux's `rt_sigframe` sits, and reading a returning
//! frame's registers back.

/// The words `mk_foreign_context` uses: r8..r15, rdi, rsi, rbp, rbx, rdx,
/// rax, rcx, rsp, rip, rflags.
pub const WORDS: usize = 18;
pub(super) const FRAME_SIZE: usize = 440;
pub(super) const UC_OFF: usize = 8;
/// `uc_mcontext` within the ucontext.
pub const SIGCONTEXT_OFF: usize = 40;
/// `uc_sigmask` within the ucontext.
pub(super) const SIGMASK_OFF: usize = 296;
/// `uc_stack` within the ucontext.
pub(super) const STACK_OFF: usize = 16;
pub(super) const INFO_OFF: usize = 312;
pub(super) const SS_ONSTACK: u64 = 1;
pub(super) const SS_DISABLE: u64 = 2;

pub(super) fn put(buf: &mut [u8], at: usize, v: u64) {
    buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
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

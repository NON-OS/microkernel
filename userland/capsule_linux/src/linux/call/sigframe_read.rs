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

//! Reading back the frame `sigframe` built, as rt_sigreturn does: the
//! registers, the mask and the alternate stack the handler returns to. Pure,
//! and any bytes at all may sit where the guest's rsp points, so every read is
//! checked and none can panic.

use super::sigframe::{SIGCONTEXT_OFF, SIGMASK_OFF, STACK_OFF, WORDS};

fn word(uc: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(uc.get(at..at + 8)?.try_into().ok()?))
}

/// The 18 words a returning frame carries, from the ucontext the guest's rsp
/// points at: the trampoline's `ret` left rsp there.
pub fn returned(uc: &[u8]) -> Option<[u64; WORDS]> {
    let mut out = [0u64; WORDS];
    for (i, slot) in out.iter_mut().enumerate() {
        *slot = word(uc, SIGCONTEXT_OFF + i * 8)?;
    }
    Some(out)
}

/// The mask a returning frame restores, from uc_sigmask.
pub fn returned_mask(uc: &[u8]) -> Option<u64> {
    word(uc, SIGMASK_OFF)
}

/// uc_stack as the handler left it: ss_sp, ss_flags, ss_size.
pub fn returned_stack(uc: &[u8]) -> Option<[u64; 3]> {
    let flags = u32::from_le_bytes(uc.get(STACK_OFF + 8..STACK_OFF + 12)?.try_into().ok()?);
    Some([word(uc, STACK_OFF)?, u64::from(flags), word(uc, STACK_OFF + 16)?])
}

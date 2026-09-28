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

//! Where a Linux program reads each part of the frame its handler gets.

use super::sigframe_tests::{regs, RSP};
use crate::sigframe::{build, SIGCONTEXT_OFF};

#[test]
fn the_sigcontext_sits_where_the_ucontext_says() {
    /* uc_mcontext is at SIGCONTEXT_OFF within the ucontext, at frame + 8. */
    let saved = regs();
    let (_, buf, _) = build(&saved, 1, 2, 3, 0, None, false).expect("frame");
    let at = 8 + SIGCONTEXT_OFF;
    let r8 = u64::from_le_bytes(buf[at..at + 8].try_into().unwrap());
    assert_eq!(r8, saved[0]);
}

#[test]
fn the_registers_and_mask_sit_where_linux_programs_read_them() {
    /*
     * Offsets within the ucontext, from musl's and glibc's own
     * offsetof(ucontext_t, ...) on x86-64: uc_mcontext.gregs[REG_R8] at 40,
     * REG_RSP at 160, REG_RIP at 168, uc_sigmask at 296. The ucontext is at
     * frame + 8, after the return address.
     */
    let saved = regs();
    let (_, buf, _) = build(&saved, 1, 2, 3, 0x0000_8000_0000_0001, None, false).expect("frame");
    let word = |at: usize| u64::from_le_bytes(buf[8 + at..8 + at + 8].try_into().unwrap());
    /* r8, rsp, rip, then the mask. */
    assert_eq!(word(40), saved[0]);
    assert_eq!(word(160), saved[RSP]);
    assert_eq!(word(168), saved[16]);
    assert_eq!(word(296), 0x0000_8000_0000_0001);
}

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

//! The register file a supervisor reads and writes for a parked guest, in the
//! order of Linux's `struct sigcontext`, and the checks on the way back in.

use crate::arch::context::SavedUser;
use crate::process::userspace::{USER_CS, USER_DS};

/// r8..r15, rdi, rsi, rbp, rbx, rdx, rax, rcx, rsp, rip, rflags.
pub(super) const WORDS: usize = 18;

const USER_VA_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;
// CF PF AF ZF SF DF OF are the program's. IF and the reserved bit 1 are forced
// on; TF, IOPL, NT, RF, AC and the rest stay the kernel's.
const USER_FLAGS: u64 = 0x0CD5;
const FORCED_FLAGS: u64 = 0x202;

pub(super) fn to_words(c: &SavedUser) -> [u64; WORDS] {
    [
        c.r8, c.r9, c.r10, c.r11, c.r12, c.r13, c.r14, c.r15, c.rdi, c.rsi, c.rbp, c.rbx, c.rdx,
        c.rax, c.rcx, c.rsp, c.rip, c.rflags,
    ]
}

/// A context the guest may resume into, or None. Selectors and the TLS base
/// come from the kernel, never from the supervisor.
pub(super) fn from_words(w: &[u64; WORDS], fs_base: u64) -> Option<SavedUser> {
    let (rsp, rip) = (w[15], w[16]);
    if rip > USER_VA_MAX || rsp > USER_VA_MAX || rsp == 0 {
        return None;
    }
    Some(SavedUser {
        r8: w[0],
        r9: w[1],
        r10: w[2],
        r11: w[3],
        r12: w[4],
        r13: w[5],
        r14: w[6],
        r15: w[7],
        rdi: w[8],
        rsi: w[9],
        rbp: w[10],
        rbx: w[11],
        rdx: w[12],
        rax: w[13],
        rcx: w[14],
        rsp,
        rip,
        rflags: (w[17] & USER_FLAGS) | FORCED_FLAGS,
        cs: USER_CS as u64,
        ss: USER_DS as u64,
        fs_base,
        gs_base: 0,
    })
}

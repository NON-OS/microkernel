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

//! The SSE-only round, for a part or an XCR0 without AVX state.

use core::arch::asm;

use super::state_regs::SCHED_YIELD;

/// The SSE-only version: xmm0..xmm15, 16 bytes each.
pub fn round_xmm(pattern: &[u8; 32], out: &mut [u8; 512]) {
    // SAFETY: `pattern` and `out` are as in `round_ymm`; r12 and r13 survive
    // the syscall. Only the first 256 bytes of `out` are written.
    unsafe {
        asm!(
            ".irp r,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15",
            "movdqu xmm\\r, [r13]",
            ".endr",
            "mov eax, {nr}",
            "syscall",
            ".irp r,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15",
            "movdqu [r12], xmm\\r",
            "add r12, 16",
            ".endr",
            nr = const SCHED_YIELD,
            inout("r12") out.as_mut_ptr() => _,
            in("r13") pattern.as_ptr(),
            clobber_abi("C"),
            options(nostack),
        );
    }
}

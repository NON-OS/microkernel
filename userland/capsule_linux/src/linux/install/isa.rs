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

//! Which build of the chat program this CPU can run.
//!
//! The shipped program is built for x86-64-v3 (AVX2, FMA, F16C, BMI1/2,
//! LZCNT, MOVBE), which a CPU without them stops on an invalid opcode. A
//! second build for x86-64-v2 ships beside it, and a tier starts it when
//! this CPU, or the kernel's saved register state, lacks any of those.

use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

/// The v2 build's path, beside the v3 one.
pub const CHAT_V2: &[u8] = b"/bin/qwenchat-x86_64_v2";

/// `program`, or the v2 build when `program` is the v3 chat program and
/// this CPU cannot run it.
pub fn for_this_cpu(program: &'static [u8]) -> &'static [u8] {
    if program == b"/bin/qwenchat" && !runs_v3() {
        CHAT_V2
    } else {
        program
    }
}

/// Whether every x86-64-v3 feature is present and the OS saves the YMM
/// registers, without which AVX faults even on a CPU that has it.
pub fn runs_v3() -> bool {
    /*
     * SAFETY: CPUID exists on every x86-64 CPU; XGETBV runs only once
     * CPUID says the OS enabled it (OSXSAVE).
     */
    unsafe {
        if __cpuid(0).eax < 7 {
            return false;
        }
        let one = __cpuid(1);
        let seven = __cpuid_count(7, 0);
        let ext = __cpuid(0x8000_0001);
        let fma = one.ecx & (1 << 12) != 0;
        let movbe = one.ecx & (1 << 22) != 0;
        let osxsave = one.ecx & (1 << 27) != 0;
        let avx = one.ecx & (1 << 28) != 0;
        let f16c = one.ecx & (1 << 29) != 0;
        let bmi1 = seven.ebx & (1 << 3) != 0;
        let avx2 = seven.ebx & (1 << 5) != 0;
        let bmi2 = seven.ebx & (1 << 8) != 0;
        let lzcnt = ext.ecx & (1 << 5) != 0;
        let cpu = fma && movbe && avx && f16c && bmi1 && avx2 && bmi2 && lzcnt;
        cpu && osxsave && _xgetbv(0) & 0b110 == 0b110
    }
}

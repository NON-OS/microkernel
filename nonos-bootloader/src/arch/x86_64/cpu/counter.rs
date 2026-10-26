// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The time-stamp counter.

/// The time-stamp counter, as read before.
#[inline(always)]
pub fn cycles() -> u64 {
    // SAFETY: RDTSC exists on every x86_64 CPU and reads no memory.
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// The time-stamp counter between two LFENCEs, so no earlier or later
/// instruction is measured with it.
#[inline(always)]
pub fn cycles_serialized() -> u64 {
    // SAFETY: LFENCE and RDTSC read no memory and change no state but the
    // registers named.
    unsafe {
        core::arch::asm!("lfence", options(nostack, nomem));

        let mut hi: u32;
        let mut lo: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );

        core::arch::asm!("lfence", options(nostack, nomem));

        ((hi as u64) << 32) | (lo as u64)
    }
}

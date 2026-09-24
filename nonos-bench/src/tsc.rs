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

//! Reading the cycle counter without letting the processor reorder around it.
//!
//! `rdtsc` is not a barrier. Instructions from either side may be moved across
//! it, so the obvious pair of reads around a body can close before the body has
//! retired and report a number smaller than the work took. At the magnitudes
//! here, tens of cycles, that is not a rounding error; it is the measurement.
//!
//! `lfence` on both sides is the fix: it waits for everything already issued to
//! complete before the counter is sampled. `rdtscp` would serialise the leading
//! edge on its own, but it is not architecturally guaranteed present, and a
//! benchmark that silently measures something else on the machine that lacks it
//! is worse than one that is uniformly slightly conservative.
//!
//! Reading the counter is unprivileged here: no shipped profile sets `CR4.TSD`,
//! so a capsule at CPL=3 samples the same counter the kernel does.

/// The cycle counter, fenced on both sides.
#[inline(always)]
pub fn read_serialised() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let lo: u32;
        let hi: u32;
        // SAFETY: rdtsc and lfence have no memory operands and no side effects
        // beyond the registers named. `nomem` is correct because nothing is
        // read or written; `preserves_flags` because neither touches them.
        unsafe {
            core::arch::asm!(
                "lfence",
                "rdtsc",
                "lfence",
                out("eax") lo,
                out("edx") hi,
                options(nomem, nostack, preserves_flags),
            );
        }
        ((hi as u64) << 32) | lo as u64
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // The host build compiles this for the summary proofs. It never times
        // anything there, and returning a constant makes that obvious rather
        // than producing plausible nonsense.
        0
    }
}

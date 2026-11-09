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

//! Which vector path this processor runs. AVX2 needs the processor to have
//! it and the kernel to have turned on the YMM state it saves for each task
//! (XCR0 bits 1 and 2, read with XGETBV); without both, SSE2, which every
//! x86-64 has. Read once and kept.

use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Scalar = 1,
    Sse2 = 2,
    Avx2 = 3,
}

static LEVEL: AtomicU8 = AtomicU8::new(0);

pub fn level() -> Level {
    match LEVEL.load(Ordering::Relaxed) {
        1 => Level::Scalar,
        2 => Level::Sse2,
        3 => Level::Avx2,
        _ => {
            let l = detect();
            LEVEL.store(l as u8, Ordering::Relaxed);
            l
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn detect() -> Level {
    use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};
    const OSXSAVE: u32 = 1 << 27;
    const AVX: u32 = 1 << 28;
    const AVX2: u32 = 1 << 5;
    const YMM_SAVED: u64 = 0b110;
    // CPUID is a safe call in the compiler the tree is built with and an unsafe
    // function in the one Kani carries, so it sits in an unsafe block, which the
    // first calls unused.
    #[allow(unused_unsafe)]
    // SAFETY: CPUID is in every x86-64.
    let highest = unsafe { __cpuid(0) }.eax;
    if highest < 7 {
        return Level::Sse2;
    }
    #[allow(unused_unsafe)]
    // SAFETY: as above, and leaf 7 is one the CPU says it has.
    let (one, seven) = unsafe { (__cpuid(1), __cpuid_count(7, 0)) };
    let cpu = one.ecx & OSXSAVE != 0 && one.ecx & AVX != 0 && seven.ebx & AVX2 != 0;
    // SAFETY: XGETBV runs only once CPUID says the kernel set CR4.OSXSAVE.
    if cpu && unsafe { _xgetbv(0) } & YMM_SAVED == YMM_SAVED {
        Level::Avx2
    } else {
        Level::Sse2
    }
}

#[cfg(not(target_arch = "x86_64"))]
fn detect() -> Level {
    Level::Scalar
}

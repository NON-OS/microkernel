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

//! Extended processor state: the components the boot CPU enabled and the area
//! size for them. FXSAVE covers x87 and SSE only: with AVX on it dropped the upper ymm halves
//! (zmm and opmask on AVX-512), so threads sharing a CPU corrupted each other.

use crate::arch::x86_64::boot::constants::{CR4_OSXSAVE, XCR0_AVX, XCR0_SSE, XCR0_X87};
use crate::arch::x86_64::boot::cpu_ops::{cpuid_count, read_cr4, read_xcr0, write_cr4, write_xcr0};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Bytes in every thread's area, 64-byte aligned by its owner.
pub const AREA: usize = 4096;
static XSAVE: AtomicBool = AtomicBool::new(false);
static XCR0: AtomicU64 = AtomicU64::new(0);

// CPUID.(0DH,0).EBX: the save area size for the components XCR0 enables now.
pub fn needed() -> usize {
    cpuid_count(0x0D, 0).1 as usize
}

/// Boot CPU, after SSE/AVX bring-up: what does not fit AREA is turned off.
///
/// # Safety
/// Runs once, on the boot CPU, before any thread is created.
pub unsafe fn record_boot() {
    if read_cr4() & CR4_OSXSAVE == 0 {
        return;
    }
    for fallback in [XCR0_X87 | XCR0_SSE | XCR0_AVX, XCR0_X87 | XCR0_SSE] {
        if needed() <= AREA {
            break;
        }
        // SAFETY: eK@nonos.systems - OSXSAVE is set, checked above; both keep x87 and SSE.
        unsafe { write_xcr0(read_xcr0() & fallback) };
    }
    XCR0.store(read_xcr0(), Ordering::Release);
    XSAVE.store(needed() <= AREA, Ordering::Release);
}

/// On each AP before it runs a thread: the boot CPU's components.
///
/// # Safety
/// Runs once per AP during its bring-up, with interrupts off.
pub unsafe fn mirror_on_ap() {
    if !XSAVE.load(Ordering::Acquire) {
        return;
    }
    // SAFETY: eK@nonos.systems - the boot CPU has XSAVE and every CPU the same features.
    unsafe {
        write_cr4(read_cr4() | CR4_OSXSAVE);
        write_xcr0(XCR0.load(Ordering::Acquire));
    }
}

pub fn uses_xsave() -> bool {
    XSAVE.load(Ordering::Acquire)
}

pub fn enabled() -> u64 {
    XCR0.load(Ordering::Acquire)
}

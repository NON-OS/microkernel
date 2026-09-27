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

//! The processor state user threads may use, decided here and not left to
//! firmware. Nothing set CR4.OSXSAVE or XCR0 before this step, so a program saw
//! whatever the firmware left, and the switch saved it with FXSAVE whether or
//! not AVX was on. Before the process runtime and the APs, which mirror it.

#[cfg(target_arch = "x86_64")]
pub(super) fn init_extended_state() {
    use crate::arch::x86_64::cpu::xstate;
    // Nothing on this path has run CPUID into the cache yet, and without it
    // every feature reads absent and SSE bring-up refuses.
    crate::arch::x86_64::cpu::detect_features();
    // SAFETY: eK@nonos.systems - the boot CPU, once, before any thread exists.
    // Each enable step checks CPUID first and leaves a missing feature off.
    if let Err(e) = unsafe { crate::arch::x86_64::boot::validation::enable_sse_avx() } {
        super::fatal::fatal("cpu: SSE bring-up failed", e.as_str());
    }
    let serial = crate::sys::serial::print;
    serial(b"[CPU-FPU] xsave=");
    serial(if xstate::uses_xsave() { b"1" } else { b"0" });
    serial(b" xcr0=");
    crate::sys::serial::print_hex(xstate::enabled());
    serial(b" area=");
    crate::sys::serial::print_dec(xstate::needed() as u64);
    crate::sys::serial::println(b"");
}

#[cfg(not(target_arch = "x86_64"))]
pub(super) fn init_extended_state() {}

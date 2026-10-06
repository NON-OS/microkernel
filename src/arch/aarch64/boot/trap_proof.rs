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

//! Deliberate exceptions for the aarch64 boot lane.
//!
//! Each feature here compiles one exception into the boot path at a fixed
//! point, and the lane asserts the exact line the kernel prints for it. What is
//! under test is that an exception this kernel does not resume from is named,
//! with its syndrome, before the CPU parks. No profile enables either feature,
//! so no shipped image carries them.
//!
//! Neither function returns, but both are typed as returning, so the boot code
//! after each call site stays reachable to the compiler and a proof build
//! carries the same warnings as the image it stands in for.

/// `brk #0x5350` taken while EL1 runs on SP_EL0, which lands in the SP_EL0
/// vector group this kernel never uses. Runs with the MMU off, before the
/// device tree is read, so it also proves the vectors are live that early.
/// The syndrome is fixed by the architecture: EC 0x3C, IL set, the immediate
/// in the ISS, so ESR_EL1 reads 0xF2005350. SP_EL0 is loaded with a value
/// nothing else would hold, so the lane can check the vector saved the stack
/// pointer the interrupted code was on rather than its own.
#[cfg(feature = "nonos-trap-proof-sp0")]
pub(super) fn sp_el0_vector() {
    const SP_EL0_MARK: u64 = 0x0000_5350_5350_5350;
    // SAFETY: SP_EL0 is written while SPSel still selects SP_EL1, where the
    // write is permitted. The SP switch and the breakpoint are one asm block,
    // so nothing runs on SP_EL0 in between. The exception is taken on SP_EL1,
    // which still holds the boot stack, and its handler never returns.
    unsafe {
        core::arch::asm!(
            "msr sp_el0, {mark}",
            "msr spsel, #0",
            "brk #0x5350",
            mark = in(reg) SP_EL0_MARK,
            options(noreturn, nostack)
        );
    }
}

/// A kernel read of a virtual address no boot table describes. Level 0 entry
/// 128 of the boot tables is never filled, so the walk stops there with a
/// level 0 translation fault. Runs after the MMU is on, so the report goes
/// through the trap contract and the serial lock is live.
#[cfg(feature = "nonos-trap-proof-kernel-abort")]
pub(super) fn kernel_data_abort() {
    const UNMAPPED: u64 = 0x0000_4000_0000_0000;
    // SAFETY: nothing maps UNMAPPED, so the read faults and never returns
    // data; the fault is terminal and the CPU parks in the trap path.
    let value = unsafe { core::ptr::read_volatile(UNMAPPED as *const u64) };
    crate::sys::serial::Line::new()
        .str(b"[TRAP-PROOF] read of an unmapped address returned ")
        .hex(value)
        .end();
    crate::arch::aarch64::cpu::halt()
}

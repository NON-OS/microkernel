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

//! Speculative store bypass turned off in hardware (FEAT_SSBS).
//!
//! PSTATE.SSBS = 1 *permits* the bypass and 0 forbids it (Arm ARM, PSTATE
//! fields). On every exception the PE sets PSTATE.SSBS from
//! SCTLR_EL1.DSSBS, bit 44, so with DSSBS clear the kernel runs without the
//! bypass from every entry. EL0's starts at 0 too, from its initial SPSR.

use core::arch::asm;

const SCTLR_DSSBS: u64 = 1 << 44;

/// `msr ssbs, #0`. The immediate form, because FEAT_SSBS (ID_AA64PFR1_EL1.SSBS
/// = 1) has only that one; the register form needs SSBS = 2. Encoded by hand:
/// MSR (immediate), op1 = 3, op2 = 1, CRm = 0 (Arm ARM C6, MSR immediate),
/// since the ARMv8.0 assembler does not know the name.
pub(super) fn clear() {
    // SAFETY: only reached where ID_AA64PFR1_EL1.SSBS is at least 1, so both
    // PSTATE.SSBS and SCTLR_EL1.DSSBS exist. Clearing them only narrows what
    // the core may speculate.
    unsafe {
        asm!(
            "mrs {t}, sctlr_el1",
            "bic {t}, {t}, {dssbs}",
            "msr sctlr_el1, {t}",
            "isb",
            ".inst 0xd503403f",
            t = out(reg) _,
            dssbs = in(reg) SCTLR_DSSBS,
            options(nostack),
        );
    }
}

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

//! What the CPU offers the loader's security checks, from its ID registers.

use super::id_fields::IdRegs;

/// The ID registers, read afresh at each call; they never change.
pub fn id_regs() -> IdRegs {
    let (isar0, mmfr0, mmfr1, mmfr2): (u64, u64, u64, u64);
    // SAFETY: ID registers, readable at the EL UEFI runs at; no side effects.
    unsafe {
        core::arch::asm!(
            "mrs {0}, id_aa64isar0_el1",
            "mrs {1}, id_aa64mmfr0_el1",
            "mrs {2}, id_aa64mmfr1_el1",
            "mrs {3}, id_aa64mmfr2_el1",
            out(reg) isar0, out(reg) mmfr0, out(reg) mmfr1, out(reg) mmfr2,
            options(nomem, nostack, preserves_flags),
        )
    };
    IdRegs { isar0, mmfr0, mmfr1, mmfr2 }
}

/// (execute-never for the kernel, PAN, a third that x86_64 fills): every
/// AArch64 translation regime has PXN, so the first is always true.
pub fn security_features() -> (bool, bool, bool) {
    (true, id_regs().pan(), false)
}

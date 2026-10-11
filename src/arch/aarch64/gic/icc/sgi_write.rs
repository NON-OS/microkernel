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

use core::arch::asm;

/// The one way both send paths raise an SGI. The target must see every
/// store made before it: a TLB shootdown publishes its pending flush, then
/// signals. A system-register write is not ordered after earlier stores by
/// itself (Arm ARM, DDI 0487, the synchronization requirements for system
/// registers), so `dsb ishst` completes them first, and the asm is a memory
/// clobber so the compiler cannot sink a store past it either. GICv2's
/// SGIR write does the same.
pub(super) fn write_sgi1r(value: u64) {
    // SAFETY: ICC_SGI1R_EL1 is write-only and accessible at EL1 once the CPU
    // interface is enabled, which `icc::init` did during GIC bring-up.
    unsafe {
        asm!(
            "dsb ishst",
            "msr icc_sgi1r_el1, {0}",
            "isb",
            in(reg) value,
            options(nostack, preserves_flags),
        );
    }
}

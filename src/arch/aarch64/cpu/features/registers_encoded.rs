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

//! ID registers newer than ARMv8.0, read by their encodings. The assembler
//! only knows their mnemonics for a target of the version that added them,
//! and this kernel builds for ARMv8.0. On an older core each is a reserved
//! ID register and reads as zero, which is the answer "nothing implemented"
//! (Arm ARM D19.1, the ID space).

use core::arch::asm;

/// `ID_AA64MMFR2_EL1`, S3_0_C0_C7_2, which reports E0PD.
pub(super) fn read_aa64mmfr2() -> u64 {
    let value: u64;
    // SAFETY: an identification register, always readable at EL1, no side effects.
    unsafe {
        asm!("mrs {}, s3_0_c0_c7_2", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

/// `ID_AA64ISAR2_EL1`, S3_0_C0_C6_2, which reports QARMA3 address
/// authentication (APA3, bits 15:12).
pub(super) fn read_aa64isar2() -> u64 {
    let value: u64;
    // SAFETY: as for `read_aa64mmfr2`.
    unsafe {
        asm!("mrs {}, s3_0_c0_c6_2", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

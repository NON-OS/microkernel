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

/// `ID_AA64MMFR2_EL1`, which reports E0PD among the memory-model features.
///
/// Named by its encoding, S3_0_C0_C7_2, because the assembler only knows the
/// mnemonic for an ARMv8.2 target and this kernel builds for ARMv8.0. On a
/// v8.0 core the encoding is a reserved ID register and reads as zero, which
/// is the answer "nothing implemented" (Arm ARM D19.1, the ID space).
pub(super) fn read_aa64mmfr2() -> u64 {
    let value: u64;
    // SAFETY: an identification register, always readable at EL1, no side effects.
    unsafe {
        asm!("mrs {}, s3_0_c0_c7_2", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

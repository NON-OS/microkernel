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

//! FEAT_RNG's RNDR and RNDRRS, used only where ID_AA64ISAR0_EL1 says the
//! CPU has them. Each sets PSTATE.Z when it could not produce a value.

/// ID_AA64ISAR0_EL1.RNDR, bits 63:60: 0b0001 or above means both registers.
fn has_rndr() -> bool {
    super::security::id_regs().rndr()
}

macro_rules! rndr_read {
    ($reg:literal) => {{
        let (v, ok): (u64, u64);
        // SAFETY: present (checked by the caller); writes only registers and flags.
        unsafe {
            core::arch::asm!(concat!("mrs {v}, ", $reg), "cset {ok}, ne",
                v = out(reg) v, ok = out(reg) ok, options(nomem, nostack))
        };
        (ok != 0).then_some(v)
    }};
}

/// A word from RNDR, the generator's output, or None where it is absent or
/// failed.
pub fn hw_random() -> Option<u64> {
    if !has_rndr() {
        return None;
    }
    rndr_read!("s3_3_c2_c4_0")
}

/// A word from RNDRRS, reseeded before it is read: seed grade, or None.
pub fn hw_seed() -> Option<u64> {
    if !has_rndr() {
        return None;
    }
    rndr_read!("s3_3_c2_c4_1")
}

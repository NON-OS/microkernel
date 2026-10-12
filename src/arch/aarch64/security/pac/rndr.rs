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

//! RNDR (FEAT_RNG): a 64-bit random number, or a failure the instruction
//! reports in PSTATE.Z with the value zero (Arm ARM, RNDR). A key built
//! without checking the flag is a key of zeros on the bad read.

use core::arch::asm;

/// Reads to try before calling the source unavailable. A failure is
/// transient, the generator not yet ready; persistent failure is not.
const TRIES: usize = 16;

pub(super) fn rndr() -> Option<u64> {
    for _ in 0..TRIES {
        let (value, ok): (u64, u64);
        // SAFETY: the caller checked FEAT_RNG, so S3_3_C2_C4_0 (RNDR) exists
        // and is readable at EL1. It writes its register and NZCV only.
        unsafe {
            asm!(
                "mrs {v}, s3_3_c2_c4_0",
                "cset {ok}, ne",
                v = out(reg) value,
                ok = out(reg) ok,
                options(nomem, nostack),
            );
        }
        if ok != 0 {
            return Some(value);
        }
    }
    None
}

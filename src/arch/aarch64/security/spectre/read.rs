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

//! This CPU's identification registers and firmware answers, the decision's
//! two inputs. ID_AA64MMFR2_EL1 and ID_AA64ISAR2_EL1 are read by encoding:
//! the kernel builds for ARMv8.0, whose assembler does not name them, and on
//! an older core they read as zero, "nothing implemented" (Arm ARM D19.1).

use core::arch::asm;

use super::answer::{Answer, Firmware};
use super::ids::Ids;
use crate::arch::aarch64::smccc::{
    arch_features, ARCH_WORKAROUND_1, ARCH_WORKAROUND_2, ARCH_WORKAROUND_3,
};

pub(super) fn ids() -> Ids {
    let (pfr0, pfr1, mmfr1, mmfr2, isar2, midr): (u64, u64, u64, u64, u64, u64);
    // SAFETY: identification registers, readable at EL1, with no effects.
    unsafe {
        asm!(
            "mrs {0}, id_aa64pfr0_el1",
            "mrs {1}, id_aa64pfr1_el1",
            "mrs {2}, id_aa64mmfr1_el1",
            "mrs {3}, s3_0_c0_c7_2",
            "mrs {4}, s3_0_c0_c6_2",
            "mrs {5}, midr_el1",
            out(reg) pfr0, out(reg) pfr1, out(reg) mmfr1,
            out(reg) mmfr2, out(reg) isar2, out(reg) midr,
            options(nomem, nostack, preserves_flags),
        );
    }
    Ids { pfr0, pfr1, mmfr1, mmfr2, isar2, midr }
}

/// Each answer is for the CPU that asks, so every CPU asks for itself.
pub(super) fn firmware() -> Firmware {
    Firmware {
        wa1: Answer::from_ret(arch_features(ARCH_WORKAROUND_1)),
        wa2: Answer::from_ret(arch_features(ARCH_WORKAROUND_2)),
        wa3: Answer::from_ret(arch_features(ARCH_WORKAROUND_3)),
    }
}

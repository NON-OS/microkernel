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

use crate::arch::aarch64::cpu::features::{has_feature, CpuFeature};

use super::built::built_with_bti;

/// SCTLR_EL1.BT1: an indirect branch at EL1 into a guarded page must land on
/// a landing pad (Arm ARM, FEAT_BTI). BT0, the same for EL0, stays clear: no
/// user page is mapped guarded.
const SCTLR_BT1: u64 = 1 << 36;

/// Whether kernel text is mapped guarded and EL1 branches into it checked:
/// the core implements FEAT_BTI and this kernel carries the landing pads.
pub fn kernel_guarded() -> bool {
    has_feature(CpuFeature::Bti) && built_with_bti()
}

/// Turn BT1 on for this CPU. False, changing nothing, where the kernel is not
/// guarded.
pub fn init_bti() -> bool {
    if !kernel_guarded() {
        return false;
    }
    // SAFETY: SCTLR_EL1 is writable at EL1 and BT1 is defined with FEAT_BTI.
    // It only adds a check to indirect branches into guarded pages, every one
    // of which, `kernel_guarded` has just confirmed, lands on a pad.
    unsafe {
        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nomem, nostack, preserves_flags));
        sctlr |= SCTLR_BT1;
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr, options(nomem, nostack, preserves_flags));
    }
    true
}

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

//! PAN (FEAT_PAN): with PSTATE.PAN set, an EL1 data access to a page EL0 may
//! reach faults (Arm ARM, D8 the VMSA, "Privileged access never").
//!
//! The kernel reaches user memory only through `usercopy`, which walks the
//! process's tables and copies through the direct map, a mapping EL0 cannot
//! reach. So PAN is set and stays set: any EL1 access through a user address
//! is a bug, and with PAN it faults instead of succeeding. `with_user_access`
//! is the one window that clears it, and nothing opens it today.
//!
//! SCTLR_EL1.SPAN clear makes every exception taken to EL1 set PSTATE.PAN,
//! so a handler starts with it set whatever EL0 or an interrupted window
//! left; `eret` restores the interrupted value from SPSR_EL1. Without
//! FEAT_PAN, SPAN is RES1 and is left alone.

use core::arch::asm;

use crate::arch::aarch64::cpu::{has_feature, CpuFeature};

const SCTLR_SPAN: u64 = 1 << 23;

/// Turn PAN on for this CPU. False on a core without it.
pub fn init_pan() -> bool {
    if !has_feature(CpuFeature::Pan) {
        return false;
    }
    // SAFETY: SCTLR_EL1 is writable at EL1; SPAN only decides what an
    // exception entry does to PSTATE.PAN. `MSR PAN, #1` is encoded by hand
    // (0xd500419f) so the build need not enable +pan for every function; it
    // exists because FEAT_PAN does, and only narrows what EL1 may touch.
    unsafe {
        let mut sctlr: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nomem, nostack, preserves_flags));
        sctlr &= !SCTLR_SPAN;
        asm!("msr sctlr_el1, {}", "isb", in(reg) sctlr, options(nomem, nostack, preserves_flags));
        asm!(".inst 0xd500419f", options(nomem, nostack, preserves_flags));
    }
    true
}

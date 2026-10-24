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

//! The word the EL0 entry vectors read (`SPECTRE_FROM_EL0` in vectors.S),
//! built from each CPU's plan. One word serves every CPU, so it carries the
//! union of what any CPU needs: a CPU that needs less pays for the others,
//! which on a mixed big.LITTLE part is the cost of never running a core
//! without its mitigation. CLRBHB is a hint, a NOP where not implemented.

use super::plan::{Bhb, Plan, V2};

/// Bits 1:0, the firmware call: none, ARCH_WORKAROUND_1, or _3.
pub(crate) const CALL_MASK: u32 = 0b11;
pub(crate) const CALL_WA1: u32 = 1;
pub(crate) const CALL_WA3: u32 = 2;
/// Bit 2: the conduit is HVC, not SMC.
pub(crate) const HVC: u32 = 1 << 2;
/// Bit 3: CLRBHB then ISB.
pub(crate) const CLRBHB: u32 = 1 << 3;
/// Bits 15:8: the history loop's branch count, 0 for none.
pub(crate) const LOOP_SHIFT: u32 = 8;
pub(crate) const LOOP_MASK: u32 = 0xFF << LOOP_SHIFT;

/// One CPU's word. ARCH_WORKAROUND_3 does what _1 does as well (DEN0028), so
/// a CPU needing both makes one call.
pub(crate) const fn word(plan: &Plan, hvc: bool) -> u32 {
    let mut word = match (plan.bhb, plan.v2) {
        (Bhb::Firmware, _) => CALL_WA3,
        (_, V2::Firmware) => CALL_WA1,
        _ => 0,
    };
    if hvc && word != 0 {
        word |= HVC;
    }
    match plan.bhb {
        Bhb::Clrbhb => word | CLRBHB,
        Bhb::Loop(k) => word | ((k as u32) << LOOP_SHIFT),
        _ => word,
    }
}

const fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Two CPUs' words as one: the stronger call, the longer loop, CLRBHB if
/// either needs it.
pub(crate) const fn merge(a: u32, b: u32) -> u32 {
    let call = max(a & CALL_MASK, b & CALL_MASK);
    let k = max(a & LOOP_MASK, b & LOOP_MASK);
    call | k | ((a | b) & (HVC | CLRBHB))
}

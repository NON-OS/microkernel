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

//! The Immediate Command interface (HDA 1.0a section 3.4.3): one verb out
//! through ICOI, its answer back through ICII, with ICIS carrying the busy
//! (ICB) and result-valid (IRV) bits. The specification has software use this
//! or the CORB, not both at once, so it is taken only as the fallback once a
//! CORB send has failed to reach a codec, and never while the ring runs.

use crate::constants::{ICII, ICIS, ICIS_ICB, ICIS_IRV, ICOI, VERB_GET_PARAMETER};
use crate::controller::compose_verb;
use crate::controller::wait::until;
use crate::regs::Regs;

/// Far more than the microseconds a codec needs to answer, under the
/// hundred-millisecond ceiling the rest of the driver waits a codec out.
const IMMEDIATE_MS: u64 = 100;

/// Read a codec's Get Parameter result from `address`'s root node over the
/// Immediate Command interface. `None` when the controller stays busy or never
/// marks the result valid inside the bounded wait.
pub fn get_parameter(regs: Regs, address: u8, param: u16) -> Option<u32> {
    // A prior command is read out before a new one is posted: ICB clears when
    // the controller is free.
    if !until(IMMEDIATE_MS, || unsafe { regs.r16(ICIS) } & ICIS_ICB == 0) {
        return None;
    }
    let verb = compose_verb(address, 0, VERB_GET_PARAMETER, param);
    unsafe {
        regs.w32(ICOI, verb);
        // Posting sets ICB; the controller clears it and sets IRV once the
        // answer is in ICII.
        regs.w16(ICIS, ICIS_ICB);
    }
    if !until(IMMEDIATE_MS, || unsafe { regs.r16(ICIS) } & ICIS_IRV != 0) {
        return None;
    }
    let result = unsafe { regs.r32(ICII) };
    // IRV is write-one-to-clear; leave it clear for the next command.
    unsafe { regs.w16(ICIS, ICIS_IRV) };
    Some(result)
}

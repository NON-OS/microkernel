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

//! The SWSM semaphore that guards SW_FW_SYNC. SMBI arbitrates between
//! software agents (reading it as 0 takes it); SWESMBI then arbitrates with
//! firmware and is held only if it reads back set.

use crate::bring::wait::until;
use crate::facts::bounds::SEMAPHORE_MS;
use crate::facts::phy::{SWSM_SMBI, SWSM_SWESMBI};
use crate::facts::regs::REG_SWSM;
use crate::regs::Regs;

pub const SMBI_HELD: &str = "phy semaphore not granted (SWSM.SMBI held)";
pub const SWESMBI_HELD: &str = "phy semaphore not granted (SWSM.SWESMBI not latched)";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and REG_SWSM is a 32-bit register inside it (facts::regs).
 */

pub fn get(regs: &Regs) -> Result<(), &'static str> {
    if !smbi_free(regs) {
        // SMBI is between software agents only, and this driver is the only
        // one, so a held SMBI is one a previous owner left behind: it is
        // cleared once and tried again before giving up.
        put(regs);
        if !smbi_free(regs) {
            return Err(SMBI_HELD);
        }
    }
    let latched = until(SEMAPHORE_MS, || unsafe {
        let swsm = regs.r32(REG_SWSM);
        regs.w32(REG_SWSM, swsm | SWSM_SWESMBI);
        regs.r32(REG_SWSM) & SWSM_SWESMBI != 0
    });
    if !latched {
        put(regs);
        return Err(SWESMBI_HELD);
    }
    Ok(())
}

fn smbi_free(regs: &Regs) -> bool {
    until(SEMAPHORE_MS, || unsafe { regs.r32(REG_SWSM) } & SWSM_SMBI == 0)
}

pub fn put(regs: &Regs) {
    unsafe {
        let swsm = regs.r32(REG_SWSM);
        regs.w32(REG_SWSM, swsm & !(SWSM_SMBI | SWSM_SWESMBI));
    }
}

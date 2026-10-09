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

//! igc_get_hw_semaphore_i225 and igc_put_hw_semaphore. SWSM.SMBI arbitrates
//! between software agents (reading it as 0 takes it); SWSM.SWESMBI then
//! arbitrates with firmware and is held only if it reads back set.

use crate::constants::phy::{SWSM_SMBI, SWSM_SWESMBI};
use crate::constants::regs::REG_SWSM;
use crate::constants::timeouts::SEMAPHORE_MS;
use crate::init::wait::until;
use crate::regs::Regs;

pub const SMBI_HELD: &str = "phy semaphore not granted (SWSM.SMBI held)";
pub const SWESMBI_HELD: &str = "phy semaphore not granted (SWSM.SWESMBI not latched)";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and REG_SWSM is a 32-bit register inside it (igc_regs.h).
 */

pub fn get(regs: &Regs) -> Result<(), &'static str> {
    if !smbi_free(regs) {
        // Linux clears a stuck SMBI once ("may already be held
        // unintentionally") and tries again before giving up.
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

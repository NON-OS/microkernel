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

//! igc_acquire_swfw_sync_i225 / igc_release_swfw_sync_i225 for the PHY bit
//! (IGC_SWFW_PHY0_SM, what igc_acquire_phy_base asks for). SW_FW_SYNC is
//! only read or written under the SWSM semaphore, and the semaphore is
//! dropped between tries so firmware can finish with the PHY.

use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::phy::{SWFW_PHY0_FW, SWFW_PHY0_SW};
use crate::constants::regs::REG_SW_FW_SYNC;
use crate::constants::timeouts::{SWFW_RETRY_MS, SWFW_SYNC_MS};
use crate::regs::Regs;

use super::semaphore;

pub const PHY_HELD: &str = "phy semaphore not granted (SW_FW_SYNC PHY bit held)";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and REG_SW_FW_SYNC is a 32-bit register inside it.
 */

pub fn acquire(regs: &Regs) -> Result<(), &'static str> {
    let deadline = Deadline::after_ms(SWFW_SYNC_MS);
    loop {
        semaphore::get(regs)?;
        let sync = unsafe { regs.r32(REG_SW_FW_SYNC) };
        if sync & (SWFW_PHY0_SW | SWFW_PHY0_FW) == 0 {
            unsafe { regs.w32(REG_SW_FW_SYNC, sync | SWFW_PHY0_SW) };
            semaphore::put(regs);
            return Ok(());
        }
        semaphore::put(regs);
        if deadline.expired() {
            return Err(PHY_HELD);
        }
        let _ = mk_idle_ms(SWFW_RETRY_MS);
    }
}

/// Linux logs and returns when it cannot take the semaphore to release;
/// here that is an error, so the attempt stops and the reset of the next
/// one starts clean, rather than serving with firmware locked out.
pub fn release(regs: &Regs) -> Result<(), &'static str> {
    semaphore::get(regs)?;
    unsafe {
        let sync = regs.r32(REG_SW_FW_SYNC);
        regs.w32(REG_SW_FW_SYNC, sync & !SWFW_PHY0_SW);
    }
    semaphore::put(regs);
    Ok(())
}

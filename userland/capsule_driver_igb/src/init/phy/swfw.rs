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
//! igb_acquire_swfw_sync_82575 / igb_release_swfw_sync_82575 for the PHY bit
//! igb_acquire_phy_82575 asks for: the one of this port, by the function
//! number in STATUS, since every port of a multi-port card shares SW_FW_SYNC.
//! SW_FW_SYNC is only read or written under the SWSM semaphore, and the
//! semaphore is dropped between tries so firmware can finish with the PHY.

use nonos_libc::{mk_idle_ms, Deadline};

use crate::constants::ctrl::{STATUS_FUNC_MASK, STATUS_FUNC_SHIFT};
use crate::constants::phy::{SWFW_FW_SHIFT, SWFW_PHY_SM};
use crate::constants::regs::{REG_STATUS, REG_SW_FW_SYNC};
use crate::constants::timeouts::{SWFW_RETRY_MS, SWFW_SYNC_MS};
use crate::regs::Regs;

use super::semaphore;

pub const PHY_HELD: &str = "phy semaphore not granted (SW_FW_SYNC PHY bit held)";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and REG_STATUS and REG_SW_FW_SYNC are 32-bit registers in it.
 */

/// This port's PHY bit in SW_FW_SYNC, for a STATUS value.
pub fn phy_bit(status: u32) -> u32 {
    SWFW_PHY_SM[((status & STATUS_FUNC_MASK) >> STATUS_FUNC_SHIFT) as usize]
}

pub fn acquire(regs: &Regs) -> Result<(), &'static str> {
    let sw = phy_bit(unsafe { regs.r32(REG_STATUS) });
    let deadline = Deadline::after_ms(SWFW_SYNC_MS);
    loop {
        semaphore::get(regs)?;
        let sync = unsafe { regs.r32(REG_SW_FW_SYNC) };
        if sync & (sw | sw << SWFW_FW_SHIFT) == 0 {
            unsafe { regs.w32(REG_SW_FW_SYNC, sync | sw) };
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
    let sw = phy_bit(unsafe { regs.r32(REG_STATUS) });
    semaphore::get(regs)?;
    unsafe {
        let sync = regs.r32(REG_SW_FW_SYNC);
        regs.w32(REG_SW_FW_SYNC, sync & !sw);
    }
    semaphore::put(regs);
    Ok(())
}

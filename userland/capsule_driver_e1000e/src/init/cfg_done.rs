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

//! Waiting for the configuration a reset starts, per family.

use crate::constants::regs::{REG_EECD, REG_STATUS};
use crate::constants::status::{EECD_AUTO_RD, STATUS_LAN_INIT_DONE, STATUS_PHYRA};
use crate::constants::timeouts::{AUTO_RD_MS, CFG_DONE_MS, LAN_INIT_DONE_MS, PHY_CFG_82574_MS};
use crate::log::say;
use crate::regs::Regs;
use crate::wait::{idle_until, sleep_ms};

/*
 * e1000_get_cfg_done_ich8lan: 10 ms, then STATUS.LAN_INIT_DONE
 * (e1000_lan_init_done_ich8lan), cleared for the next event, then the PHY
 * Reset Asserted bit cleared. Linux only logs a LAN_INIT_DONE timeout, since
 * a part with no NVM never sets it and still links; so does this driver.
 * The NVM bank check that follows in Linux reads the flash, which this
 * driver does not map, and is not done.
 */
pub fn pch(regs: &Regs) {
    sleep_ms(CFG_DONE_MS);
    // SAFETY: `regs` is the broker-mapped BAR0 window; STATUS is a 4-byte
    // register inside it.
    let done = || unsafe { regs.r32(REG_STATUS) } & STATUS_LAN_INIT_DONE != 0;
    if !idle_until(LAN_INIT_DONE_MS, done) {
        say("STATUS.LAN_INIT_DONE not set in 300 ms, continuing");
    }
    // SAFETY: as above.
    unsafe {
        regs.modify(REG_STATUS, STATUS_LAN_INIT_DONE, 0);
        let status = regs.r32(REG_STATUS);
        if status & STATUS_PHYRA != 0 {
            regs.w32(REG_STATUS, status & !STATUS_PHYRA);
        }
    }
}

/// e1000e_get_auto_rd_done, then the 25 ms reset_hw_82571 gives the PHY to
/// load its configuration. Linux stops the reset here when the NVM read
/// never finishes, and so does this driver.
pub fn i82574(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: `regs` is the broker-mapped BAR0 window; EECD is a 4-byte
    // register inside it.
    if !idle_until(AUTO_RD_MS, || unsafe { regs.r32(REG_EECD) } & EECD_AUTO_RD != 0) {
        return Err("EECD.AUTO_RD not set in 20 ms");
    }
    sleep_ms(PHY_CFG_82574_MS);
    Ok(())
}

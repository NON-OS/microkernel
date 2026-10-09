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

//! A PHY-only reset on the PCH parts, Linux e1000e_phy_hw_reset_generic: if
//! ME allows it, pulse CTRL.PHY_RST with the software flag held, then wait
//! for the configuration the reset starts (e1000_get_cfg_done_ich8lan).

use crate::constants::ctrl::CTRL_PHY_RST;
use crate::constants::regs::{REG_CTRL, REG_STATUS};
use crate::constants::timeouts::SHORT_MS;
use crate::constants::Family;
use crate::log::say;
use crate::regs::Regs;
use crate::swflag;
use crate::wait::sleep_ms;

use super::{cfg_done, reset_block};

pub fn run(regs: &Regs, family: Family) -> Result<(), &'static str> {
    if reset_block::blocked(regs) {
        // Linux returns success here: the PHY is the firmware's to reset.
        say("PHY reset blocked by ME, skipped");
        return Ok(());
    }
    swflag::acquire(regs, family)?;
    // SAFETY: `regs` is the broker-mapped BAR0 window; CTRL and STATUS are
    // 4-byte registers inside it.
    let ctrl = unsafe { regs.r32(REG_CTRL) };
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_CTRL, ctrl | CTRL_PHY_RST);
        let _ = regs.r32(REG_STATUS);
    }
    // Linux holds it reset_delay_us (100 us), then 150 to 300 us after.
    sleep_ms(SHORT_MS);
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_CTRL, ctrl & !CTRL_PHY_RST);
        let _ = regs.r32(REG_STATUS);
    }
    sleep_ms(SHORT_MS);
    swflag::release(regs);
    cfg_done::pch(regs);
    Ok(())
}

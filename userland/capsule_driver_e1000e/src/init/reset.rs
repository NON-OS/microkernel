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

//! The global reset, Linux reset_hw_ich8lan and reset_hw_82571: the flag
//! taken, CTRL.RST set (with CTRL.PHY_RST on the PCH parts unless ME blocks
//! PHY resets, so MAC and PHY come out of reset together), the reset waited
//! out on the clock, the flag let go, then the configuration the reset
//! started waited for.

use crate::constants::ctrl::{CTRL_PHY_RST, CTRL_RST};
use crate::constants::regs::REG_CTRL;
use crate::constants::timeouts::{PCH_RESET_SETTLE_MS, RESET_CLEAR_MS, RESET_SETTLE_MS};
use crate::constants::Family;
use crate::log::Line;
use crate::regs::Regs;
use crate::swflag;
use crate::wait::{idle_until, sleep_ms};

use super::{cfg_done, post_phy_reset, reset_block};

pub fn run(regs: &Regs, family: Family) -> Result<(), &'static str> {
    let phy_too = family.is_pch() && !reset_block::blocked(regs);
    // Linux resets whether or not the flag came; it logs and goes on.
    let flag = swflag::acquire(regs, family);
    if let Err(e) = flag {
        Line::new().text(e).text(", resetting anyway").send();
    }
    // SAFETY: `regs` is the broker-mapped BAR0 window; CTRL is a 4-byte
    // register inside it.
    unsafe { regs.modify(REG_CTRL, 0, CTRL_RST | if phy_too { CTRL_PHY_RST } else { 0 }) };
    // Nothing may touch the part while it resets: Linux does not even flush.
    sleep_ms(if family.is_pch() { PCH_RESET_SETTLE_MS } else { RESET_SETTLE_MS });
    // SAFETY: as above.
    let cleared = idle_until(RESET_CLEAR_MS, || unsafe { regs.r32(REG_CTRL) } & CTRL_RST == 0);
    if flag.is_ok() {
        swflag::release(regs);
    }
    if !cleared {
        return Err("reset did not clear in 50 ms");
    }
    if !family.is_pch() {
        return cfg_done::i82574(regs);
    }
    if phy_too {
        cfg_done::pch(regs);
        post_phy_reset::run(regs, family);
    }
    Ok(())
}

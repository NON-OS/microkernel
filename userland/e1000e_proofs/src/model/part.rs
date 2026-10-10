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

//! The MAC side of the modelled part, as it answers the driver: a global
//! reset that completes (or never does), a PHY reset that reports itself in
//! STATUS, MDIC transactions run against the PHY, an ME that finishes
//! leaving ULP when asked, and a PHY that wakes on a LANPHYPC toggle.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use nonos_devmodel::FakeBar;

use super::phy::Phy;
use crate::constants::ctrl::{CTRL_LANPHYPC_OVERRIDE, CTRL_PHY_RST, CTRL_RST};
use crate::constants::regs::{REG_CTRL, REG_EECD, REG_EXTCNF_CTRL, REG_STATUS};
use crate::constants::pch_bits::{FEXTNVM3_PHY_CFG_COUNTER_50MSEC, FEXTNVM3_PHY_CFG_COUNTER_MASK};
use crate::constants::regs_pch::{REG_FEXTNVM3, REG_FWSM, REG_H2ME};
use crate::constants::status::*;

#[derive(Default)]
pub struct Behaviour {
    /// CTRL as each global reset found it, to see whether PHY_RST came too.
    pub resets: Mutex<Vec<u32>>,
    pub reset_stuck: AtomicBool,
    pub no_auto_rd: AtomicBool,
    pub me_leaves_ulp: AtomicBool,
    /// A PHY that only answers once LANPHYPC has power-cycled it.
    pub wakes_on_lanphypc: AtomicBool,
}

pub fn step(bar: &FakeBar, phy: &Phy, how: &Behaviour) {
    let ctrl = bar.wrote32(REG_CTRL);
    if ctrl & CTRL_RST != 0 && !how.reset_stuck.load(Ordering::SeqCst) {
        how.resets.lock().unwrap().push(ctrl);
        bar.present32(REG_EXTCNF_CTRL, 0);
        if !how.no_auto_rd.load(Ordering::SeqCst) {
            bar.present32(REG_EECD, bar.wrote32(REG_EECD) | EECD_AUTO_RD);
        } else {
            bar.present32(REG_EECD, bar.wrote32(REG_EECD) & !EECD_AUTO_RD);
        }
        let phyra = if ctrl & CTRL_PHY_RST != 0 { STATUS_PHYRA } else { 0 };
        bar.present32(REG_STATUS, bar.wrote32(REG_STATUS) | STATUS_LAN_INIT_DONE | phyra);
        bar.present32(REG_CTRL, ctrl & !(CTRL_RST | CTRL_PHY_RST));
    } else if ctrl & CTRL_PHY_RST != 0 && ctrl & CTRL_RST == 0 {
        bar.present32(REG_STATUS, bar.wrote32(REG_STATUS) | STATUS_LAN_INIT_DONE | STATUS_PHYRA);
    }
    if how.me_leaves_ulp.load(Ordering::SeqCst)
        && bar.wrote32(REG_H2ME) & H2ME_ENFORCE_SETTINGS != 0
    {
        bar.present32(REG_FWSM, bar.wrote32(REG_FWSM) & !FWSM_ULP_CFG_DONE);
    }
    // The pin is held for one millisecond, which a model thread on a loaded
    // runner can sleep through. The toggle's first write, the PHY counter to
    // 50 ms, stays in the window and only the toggle makes it, so it marks
    // the power cycle as well; the PHY is not looked at again until after.
    let toggled = ctrl & CTRL_LANPHYPC_OVERRIDE != 0
        || bar.wrote32(REG_FEXTNVM3) & FEXTNVM3_PHY_CFG_COUNTER_MASK == FEXTNVM3_PHY_CFG_COUNTER_50MSEC;
    if how.wakes_on_lanphypc.load(Ordering::SeqCst) && toggled {
        phy.silent.store(false, Ordering::SeqCst);
    }
    super::mdic::serve(bar, phy);
}

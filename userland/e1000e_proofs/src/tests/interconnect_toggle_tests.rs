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

//! The LANPHYPC toggle in the MAC-PHY link check: a PHY that never answers
//! is named after the power cycle, one that wakes on it is reached.

use crate::constants::ctrl::{CTRL_EXT_FORCE_SMBUS, CTRL_LANPHYPC_OVERRIDE};
use crate::constants::pch_bits::FEXTNVM3_PHY_CFG_COUNTER_50MSEC;
use crate::constants::regs::{REG_CTRL, REG_CTRL_EXT};
use crate::constants::regs_pch::REG_FEXTNVM3;
use crate::constants::Family;
use crate::init::interconnect::run;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{pch, silent_phy};
use crate::regs::Regs;
use std::sync::atomic::Ordering;
use std::time::Instant;

#[test]
fn a_silent_phy_is_power_cycled_then_named() {
    let bar = pch();
    let _part = live(&bar, silent_phy(), Behaviour::default());
    let start = Instant::now();
    let r = run(&Regs::new(bar.base()), Family::PchAdp);
    assert_eq!(r, Err("PHY not reachable after LANPHYPC toggle"));
    assert!(start.elapsed().as_millis() >= 50 + 30, "SMBus settle and LANPHYPC settle");
    assert_eq!(bar.wrote32(REG_FEXTNVM3), FEXTNVM3_PHY_CFG_COUNTER_50MSEC);
    assert_eq!(bar.wrote32(REG_CTRL) & CTRL_LANPHYPC_OVERRIDE, 0, "pin handed back");
    assert_eq!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_FORCE_SMBUS, 0, "MAC out of SMBus");
}

#[test]
fn a_phy_that_wakes_on_the_toggle_is_reached() {
    let bar = pch();
    let how = Behaviour::default();
    how.wakes_on_lanphypc.store(true, Ordering::SeqCst);
    let part = live(&bar, silent_phy(), how);
    assert_eq!(run(&Regs::new(bar.base()), Family::PchLpt), Ok(()));
    assert!(!part.phy.silent.load(Ordering::SeqCst));
    assert_eq!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_FORCE_SMBUS, 0);
}

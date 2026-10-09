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

//! The global reset against a modelled part: the PHY reset rides along on
//! PCH parts unless ME blocks it, the flag is held across and let go, the
//! configuration waits are done, and every way it can fail is named.

use std::time::Instant;

use nonos_libc::{clear_log, logged};

use crate::constants::ctrl::{CTRL_PHY_RST, CTRL_RST};
use crate::constants::phy::BM_PORT_GEN_CFG;
use crate::constants::regs::{REG_CTRL, REG_EXTCNF_CTRL, REG_STATUS};
use crate::constants::regs_pch::REG_FWSM;
use crate::constants::status::{EXTCNF_CTRL_SWFLAG, STATUS_LAN_INIT_DONE, STATUS_PHYRA};
use crate::constants::Family;
use crate::init::reset::run;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};
use crate::regs::Regs;

#[test]
fn a_pch_reset_takes_the_phy_along_and_cleans_up_after_it() {
    let bar = pch();
    let part = live(&bar, phy_at(2), Behaviour::default());
    part.phy.set(1, BM_PORT_GEN_CFG.0, BM_PORT_GEN_CFG.1, 0x0010);
    assert_eq!(run(&Regs::new(bar.base()), Family::PchCnp), Ok(()));
    let resets = part.how.resets.lock().unwrap().clone();
    assert_eq!(resets.len(), 1);
    assert_ne!(resets[0] & CTRL_PHY_RST, 0, "MAC and PHY reset together");
    assert_eq!(bar.wrote32(REG_CTRL) & CTRL_RST, 0);
    assert_eq!(bar.wrote32(REG_STATUS) & (STATUS_LAN_INIT_DONE | STATUS_PHYRA), 0);
    assert_eq!(bar.wrote32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG, 0, "flag let go");
    assert_eq!(part.phy.get(1, 769, 17), 0, "host wake-up bit cleared");
}

#[test]
fn with_me_blocking_phy_resets_only_the_mac_is_reset() {
    let bar = pch();
    bar.present32(REG_FWSM, 0);
    let part = live(&bar, phy_at(2), Behaviour::default());
    let start = Instant::now();
    assert_eq!(run(&Regs::new(bar.base()), Family::PchTgp), Ok(()));
    assert!(start.elapsed().as_millis() >= 300, "RSPCIPHY waited for 300 ms");
    assert_eq!(part.how.resets.lock().unwrap()[0] & CTRL_PHY_RST, 0);
}

#[test]
fn a_flag_firmware_holds_is_logged_and_the_reset_goes_ahead() {
    let bar = pch();
    bar.present32(REG_EXTCNF_CTRL, EXTCNF_CTRL_SWFLAG);
    let part = live(&bar, phy_at(2), Behaviour::default());
    clear_log();
    assert_eq!(run(&Regs::new(bar.base()), Family::PchAdp), Ok(()));
    assert_eq!(part.how.resets.lock().unwrap().len(), 1);
    let line = "e1000e: swflag held by firmware for 100 ms, resetting anyway";
    assert!(logged().iter().any(|l| l == line), "{:?}", logged());
}

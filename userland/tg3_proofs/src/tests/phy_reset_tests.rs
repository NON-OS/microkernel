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

//! tg3_phy_reset for the 57765 class against the modelled PHY: a BMCR
//! reset that clears, auto power-down as the board asks, then the extended
//! packet length, FIFO elasticity, auto-MDIX and wire speed, in that order.

use std::sync::{Arc, Mutex};

use crate::constants::phy::*;
use crate::phy::{reset, Mdio};

use super::model::{live, regs, window};
use super::phy_model::{phy, Phy};

fn run(apd: bool) -> Phy {
    let bar = window();
    let state = Arc::new(Mutex::new(Phy::default()));
    let part = live(&bar, phy(state.clone()));
    assert_eq!(reset::run(&Mdio { regs: regs(&bar) }, apd), Ok(()));
    drop(part);
    Arc::try_unwrap(state).ok().unwrap().into_inner().unwrap()
}

#[test]
fn reset_then_the_shadow_settings() {
    let p = run(false);
    assert_eq!(p.wrote(MII_BMCR), [BMCR_RESET]);
    // SCR5 first, then the APD word with the 84 ms wake timer only.
    assert_eq!(p.wrote(MII_TG3_MISC_SHDW), [0x941F, 0xA801]);
    assert_eq!(p.shadows[0] & MII_TG3_AUXCTL_ACTL_EXTPKTLEN, MII_TG3_AUXCTL_ACTL_EXTPKTLEN);
    assert_eq!(p.regs[MII_TG3_EXT_CTRL as usize], MII_TG3_EXT_CTRL_FIFO_ELASTIC);
    let misc = MII_TG3_AUXCTL_MISC_FORCE_AMDIX | MII_TG3_AUXCTL_MISC_WIRESPD_EN;
    assert_eq!(p.shadows[7], misc | MII_TG3_AUXCTL_MISC_WREN | 7);
    // The BMCR reset comes before every shadow write.
    let first_shadow = p.writes.iter().position(|(r, _)| *r != MII_BMCR).unwrap();
    assert!(p.writes[..first_shadow].iter().all(|(r, _)| *r == MII_BMCR));
}

#[test]
fn auto_power_down_when_the_board_asks() {
    let p = run(true);
    assert_eq!(p.wrote(MII_TG3_MISC_SHDW), [0x941F, 0xA821]);
}

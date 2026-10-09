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

//! The MAC-PHY link check of e1000_init_phy_workarounds_pchlan against a
//! modelled PHY that answers, never answers, or answers only once LANPHYPC
//! has power-cycled it.

use crate::constants::ctrl::CTRL_EXT_FORCE_SMBUS;
use crate::constants::phy::CV_SMB_CTRL;
use crate::constants::regs::REG_CTRL_EXT;
use crate::constants::regs_pch::{REG_FEXTNVM3, REG_FWSM};
use crate::constants::Family;
use crate::init::interconnect::run;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at, silent_phy};
use crate::regs::Regs;

#[test]
fn a_phy_that_answers_is_taken_out_of_forced_smbus_at_both_ends() {
    let bar = pch();
    bar.present32(REG_CTRL_EXT, bar.wrote32(REG_CTRL_EXT) | CTRL_EXT_FORCE_SMBUS);
    let part = live(&bar, phy_at(2), Behaviour::default());
    part.phy.set(1, CV_SMB_CTRL.0, CV_SMB_CTRL.1, 0x0001);
    assert_eq!(run(&Regs::new(bar.base()), Family::PchCnp), Ok(()));
    assert_eq!(part.phy.get(1, CV_SMB_CTRL.0, CV_SMB_CTRL.1), 0);
    assert_eq!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_FORCE_SMBUS, 0);
}

#[test]
fn a_silent_phy_with_me_blocking_resets_is_named_without_a_toggle() {
    let bar = pch();
    bar.present32(REG_FWSM, 0);
    let _part = live(&bar, silent_phy(), Behaviour::default());
    let r = run(&Regs::new(bar.base()), Family::PchSpt);
    assert_eq!(r, Err("PHY silent and LANPHYPC toggle blocked by ME"));
    assert_eq!(bar.wrote32(REG_FEXTNVM3), 0, "LANPHYPC was never touched");
}

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

//! pch_mtp and later: K1 reconfigured, and dynamic power gating turned off
//! on pch_ptp only.

use crate::constants::ctrl::CTRL_EXT_DPG_EN;
use crate::constants::pch_bits::*;
use crate::constants::phy::I217_PHY_TIMEOUTS;
use crate::constants::regs::*;
use crate::constants::regs_pch::*;
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};
use nonos_libc::{entropy, set_ring_status};

#[test]
fn a_meteor_lake_part_gets_k1_reconfigured_and_dynamic_power_gating_kept() {
    let _turn = entropy(true);
    let bar = pch();
    bar.present32(REG_CTRL_EXT, bar.wrote32(REG_CTRL_EXT) | CTRL_EXT_DPG_EN);
    let part = live(&bar, phy_at(2), Behaviour::default());
    set_ring_status(0);
    let mut mem = Memory::new();
    finish(mem.driver(&bar, Family::PchMtp, 0x550A)).expect("brought up");
    let p1 = bar.wrote32(REG_FEXTNVM12) & FEXTNVM12_PHYPD_CTRL_MASK;
    assert_eq!(p1, FEXTNVM12_PHYPD_CTRL_P1);
    assert_eq!(part.phy.get(1, I217_PHY_TIMEOUTS.0, I217_PHY_TIMEOUTS.1), 0x0F00);
    assert_ne!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_DPG_EN, 0, "DPG_EN is cleared on PTP");
}

#[test]
fn a_panther_lake_part_has_dynamic_power_gating_turned_off() {
    let _turn = entropy(true);
    let bar = pch();
    bar.present32(REG_CTRL_EXT, bar.wrote32(REG_CTRL_EXT) | CTRL_EXT_DPG_EN);
    let _part = live(&bar, phy_at(2), Behaviour::default());
    set_ring_status(0);
    let mut mem = Memory::new();
    finish(mem.driver(&bar, Family::PchPtp, 0x57B3)).expect("brought up");
    assert_eq!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_DPG_EN, 0);
}

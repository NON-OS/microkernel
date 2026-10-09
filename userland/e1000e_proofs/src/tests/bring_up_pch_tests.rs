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

//! The whole bring-up of PCH parts against the modelled part: ULP left, PHY
//! reached, reset, family bits and errata, the `up` line naming the family.

use nonos_libc::{clear_log, entropy, logged, set_ring_status};

use super::mac_text;
use crate::constants::ctrl::CTRL_EXT_DRV_LOAD;
use crate::constants::pch_bits::*;
use crate::constants::phy::MII_BMCR;
use crate::constants::regs::*;
use crate::constants::regs_pch::*;
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{pch, phy_at};

#[test]
fn an_i219_on_cannon_lake_comes_up_with_its_errata() {
    let _turn = entropy(true);
    let bar = pch();
    bar.present32(REG_STATUS, STATUS_LINK_1000_FULL);
    let part = live(&bar, phy_at(2), Behaviour::default());
    set_ring_status(0);
    let mut mem = Memory::new();
    clear_log();
    let d = finish(mem.driver(&bar, Family::PchCnp, 0x15BB)).expect("brought up");
    let up = format!("e1000e: up 15bb mac {} family=pch_cnp link=up", mac_text(&d.mac));
    assert_eq!(logged().last(), Some(&up), "{:?}", logged());
    assert_ne!(bar.wrote32(REG_RCTL) & RCTL_EN, 0);
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    assert_ne!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_DRV_LOAD, 0);
    assert_eq!(bar.wrote32(REG_KABGTXD) & KABGTXD_BGSQLBIAS, KABGTXD_BGSQLBIAS);
    assert_ne!(bar.wrote32(REG_FEXTNVM7) & FEXTNVM7_SIDE_CLK_UNGATE, 0);
    let gates = FEXTNVM9_IOSFSB_CLKGATE_DIS | FEXTNVM9_IOSFSB_CLKREQ_DIS;
    assert_eq!(bar.wrote32(REG_FEXTNVM9) & gates, gates);
    assert_eq!(bar.wrote32(REG_IOSFPC), 0, "the SPT-only errata stays off");
    assert_eq!(bar.wrote32(REG_FEXTNVM12), 0, "K1 is pch_mtp and later");
    assert_eq!(part.phy.get(2, MII_BMCR.0, MII_BMCR.1), 0x1340);
}

/// STATUS with LU, FD and speed 1000 (10b in bits 7:6).
const STATUS_LINK_1000_FULL: u32 = 0b1000_0011;

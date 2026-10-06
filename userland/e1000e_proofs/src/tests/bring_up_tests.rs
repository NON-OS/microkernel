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

//! The whole bring-up of an 82574 (QEMU's `-device e1000e`) against the
//! modelled part, through `finish`: on the air under a drawn address with
//! the one `up` line, or every grant given back with the step named.

use nonos_libc::{clear_log, entropy, given_back, logged};

use super::mac_text;
use crate::constants::ctrl::{CTRL_EXT_DRV_LOAD, CTRL_FRCDPX, CTRL_FRCSPD, CTRL_SLU};
use crate::constants::phy::{MII_ADVERTISE, MII_BMCR, MII_CTRL1000};
use crate::constants::regs::*;
use crate::constants::rxtx::{RCTL_EN, RFCTL_EXTEN, TCTL_EN};
use crate::constants::Family;
use crate::init::finish::finish;
use crate::model::live::live;
use crate::model::memory::Memory;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, phy_at};

#[test]
fn an_82574_comes_up_under_a_drawn_address_with_autoneg_restarted() {
    let _turn = entropy(true);
    let bar = i82574();
    let part = live(&bar, phy_at(1), Behaviour::default());
    let mut mem = Memory::new();
    clear_log();
    let before = given_back().len();
    let d = finish(mem.driver(&bar, Family::I82574, 0x10D3)).expect("brought up");
    assert_eq!(given_back().len(), before, "a working attempt keeps its grants");
    let up = format!("e1000e: up 10d3 mac {} family=82574 link=down", mac_text(&d.mac));
    assert_eq!(logged().last(), Some(&up), "{:?}", logged());
    assert_eq!(d.mac[0] & 0x03, 0x02, "locally administered unicast");
    assert_ne!(bar.wrote32(REG_RCTL) & RCTL_EN, 0);
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    assert_eq!(bar.wrote32(REG_RFCTL) & RFCTL_EXTEN, 0, "legacy receive descriptors");
    let ctrl = bar.wrote32(REG_CTRL);
    assert_eq!(ctrl & (CTRL_SLU | CTRL_FRCSPD | CTRL_FRCDPX), CTRL_SLU);
    assert_ne!(bar.wrote32(REG_CTRL_EXT) & CTRL_EXT_DRV_LOAD, 0);
    let at = |r: (u16, u32)| part.phy.get(1, r.0, r.1);
    assert_eq!((at(MII_ADVERTISE), at(MII_CTRL1000)), (0x01E1, 0x0200));
    assert_eq!(at(MII_BMCR), 0x1340, "autoneg enabled and restarted");
}

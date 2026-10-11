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

//! PHYs in the states the copper tests start from, and one copper setup
//! run against each over a driver in host memory.

use std::sync::{Arc, Mutex};

use crate::constants::mac::{MAC_MODE, MAC_RX_MODE};
use crate::constants::phy::*;
use crate::link::LinkState;
use crate::phy::copper;

use super::build::{LOM, REV_57766};
use super::memory::Memory;
use super::model::{live, window};
use super::phy_model::{phy, Phy};

impl Phy {
    /// The writes to one register, in order.
    pub fn wrote(&self, reg: u32) -> Vec<u32> {
        self.writes.iter().filter(|(r, _)| *r == reg).map(|(_, v)| *v).collect()
    }
}

/// A PHY with the link up at `aux` (AUX_STAT speed bits), advertising what
/// this driver asks for and a partner offering symmetric pause.
pub fn linked(aux: u32) -> Phy {
    let mut p = Phy::default();
    p.regs[MII_BMCR as usize] = BMCR_ANENABLE;
    p.regs[MII_BMSR as usize] = BMSR_LSTATUS;
    p.regs[MII_ADVERTISE as usize] = ADVERTISE_CSMA | ADVERTISE_ALL | ADVERTISE_PAUSE_CAP;
    p.regs[MII_CTRL1000 as usize] = ADVERTISE_1000HALF | ADVERTISE_1000FULL;
    p.regs[MII_LPA as usize] = ADVERTISE_ALL | ADVERTISE_PAUSE_CAP;
    p.regs[MII_TG3_AUX_STAT as usize] = aux;
    p
}

/// What setup concluded, the PHY after it, and MAC_MODE as written.
pub fn setup(p: Phy, fe: bool) -> (LinkState, Phy, u32, u32) {
    let bar = window();
    let state = Arc::new(Mutex::new(p));
    let mut m = Memory::new();
    let mut d = m.driver(&bar, REV_57766, LOM);
    d.fast_ethernet_only = fe;
    let part = live(&bar, phy(state.clone()));
    let link = copper::setup(&mut d);
    drop(part);
    assert_eq!(d.modes.link_up, link.up);
    let p = Arc::try_unwrap(state).ok().unwrap().into_inner().unwrap();
    (link, p, bar.wrote32(MAC_MODE), bar.wrote32(MAC_RX_MODE))
}

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

//! MDIO frames through MAC_MI_COM against the modelled PHY: the frame's
//! address, register and data where __tg3_readphy puts them, the Broadcom
//! shadows reached as tg3_phy_auxctl_* reach them, and a PHY that never
//! answers failing after the whole wait.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::constants::mi::MAC_MI_COM;
use crate::constants::phy::*;
use crate::constants::timeouts::MDIO_MS;
use crate::phy::mdio::NO_ANSWER;
use crate::phy::Mdio;

use super::model::{live, regs, timed, window};
use super::phy_model::{phy, Phy};

#[test]
fn reads_and_writes_reach_the_register_named() {
    let bar = window();
    let state = Arc::new(Mutex::new(Phy::default()));
    state.lock().unwrap().regs[MII_LPA as usize] = 0x45E1;
    let _part = live(&bar, phy(state.clone()));
    let m = Mdio { regs: regs(&bar) };
    assert_eq!(m.read(MII_LPA), Ok(0x45E1));
    assert_eq!(m.write(MII_ADVERTISE, 0x1_05E1), Ok(()));
    // The data field is 16 bits; anything above it never reaches the PHY.
    assert_eq!(state.lock().unwrap().wrote(MII_ADVERTISE), [0x05E1]);
    // PHY address 1 (bit 21), register 4 (bit 16), write; BUSY dropped.
    assert_eq!(bar.wrote32(MAC_MI_COM) & 0x3FFF_0000, 0x0424_0000);
}

#[test]
fn shadows_are_selected_then_read_and_misc_writes_enable() {
    let bar = window();
    let state = Arc::new(Mutex::new(Phy::default()));
    state.lock().unwrap().shadows[2] = 0x0C02;
    let _part = live(&bar, phy(state.clone()));
    let m = Mdio { regs: regs(&bar) };
    assert_eq!(m.aux_read(MII_TG3_AUXCTL_SHDWSEL_PWRCTL), Ok(0x0C02));
    m.aux_set(MII_TG3_AUXCTL_SHDWSEL_MISC, MII_TG3_AUXCTL_MISC_WIRESPD_EN);
    let s = state.lock().unwrap();
    assert_eq!(s.wrote(MII_TG3_AUX_CTRL)[0], 0x2007);
    assert_eq!(s.shadows[7], 0x8017);
}

#[test]
fn a_silent_phy_fails_after_the_whole_wait() {
    let bar = window();
    let m = Mdio { regs: regs(&bar) };
    let (r, took) = timed(|| m.read(MII_BMSR));
    assert_eq!(r, Err(NO_ANSWER));
    assert!(took >= Duration::from_millis(MDIO_MS), "{took:?}");
}

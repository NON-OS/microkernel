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

//! tg3_setup_copper_phy against the modelled PHY: a link already up on what
//! this driver advertises is kept, and the MAC set to match it, pause
//! included.

use crate::constants::mac::*;
use crate::constants::phy::*;
use crate::link::LinkState;

use super::phy_states::{linked, setup};

#[test]
fn a_gigabit_link_is_kept_with_symmetric_pause() {
    let (link, p, mac_mode, rx_mode) = setup(linked(0x0700), false);
    assert_eq!(link, LinkState { up: true, mbps: 1000, full: true });
    assert!(p.wrote(MII_BMCR).is_empty(), "no restart: {:?}", p.writes);
    assert_eq!(mac_mode & (MAC_MODE_PORT_MODE_MASK | MAC_MODE_HALF_DUPLEX), 0x8);
    assert_ne!(rx_mode & RX_MODE_FLOW_CTRL_ENABLE, 0);
    assert_eq!(p.wrote(MII_TG3_IMASK), [0xFFFF]);
}

#[test]
fn a_100_half_link_is_mii_and_half() {
    let (link, _, mac_mode, rx_mode) = setup(linked(0x0300), false);
    assert_eq!(link, LinkState { up: true, mbps: 100, full: false });
    assert_eq!(mac_mode & (MAC_MODE_PORT_MODE_MASK | MAC_MODE_HALF_DUPLEX), 0x6);
    assert_eq!(rx_mode & RX_MODE_FLOW_CTRL_ENABLE, 0);
}

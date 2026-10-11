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

//! tg3_setup_copper_phy when the link is not one to keep: no link, or one
//! on another advertisement, is advertised again and restarted, without
//! gigabit on a 10/100 part.

use crate::constants::mac::{MAC_MODE_HALF_DUPLEX, MAC_MODE_PORT_MODE_MASK};
use crate::constants::phy::*;

use super::phy_model::Phy;
use super::phy_states::{linked, setup};

#[test]
fn no_link_advertises_and_restarts() {
    let (link, p, mac_mode, _) = setup(Phy::default(), false);
    assert!(!link.up);
    // Duplex is unknown with no link, and tg3 sets half only for half.
    assert_eq!(mac_mode & (MAC_MODE_PORT_MODE_MASK | MAC_MODE_HALF_DUPLEX), 0x8);
    assert_eq!(p.wrote(MII_ADVERTISE), [0x05E1]);
    assert_eq!(p.wrote(MII_CTRL1000), [0x0300]);
    assert_eq!(p.wrote(MII_BMCR), [BMCR_ANENABLE | BMCR_ANRESTART]);
    let (_, p, _, _) = setup(Phy::default(), true);
    assert!(p.wrote(MII_CTRL1000).is_empty(), "10/100 part");
}

#[test]
fn a_link_on_another_advertisement_is_renegotiated() {
    let mut p = linked(0x0700);
    p.regs[MII_ADVERTISE as usize] = ADVERTISE_CSMA | ADVERTISE_ALL;
    let (_, p, _, _) = setup(p, false);
    assert_eq!(p.wrote(MII_BMCR), [BMCR_ANENABLE | BMCR_ANRESTART]);
}

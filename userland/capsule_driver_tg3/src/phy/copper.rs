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

//! tg3_setup_copper_phy for the 57765 class, without a forced reset: read
//! what the PHY has, keep the link when it is up on what was advertised,
//! else advertise and restart; then set the MAC's port mode, duplex and
//! pause to match.

use crate::constants::phy::*;
use crate::link::LinkState;
use crate::setup::Driver;

use super::established::{link_bit, read_established};
use super::mac_side;
use super::mdio::Mdio;
use super::negotiate::begin;

/// The Broadcom PHY's MISCTEST shadow bit 10, set once for a board with
/// capacitive coupling before the first autonegotiation.
const CAPACITIVE_BIT: u32 = 1 << 10;

pub fn setup(d: &mut Driver) -> LinkState {
    let phy = Mdio { regs: d.regs };
    mac_side::clear_status(&d.regs);
    let _ = phy.aux_write(MII_TG3_AUXCTL_SHDWSEL_PWRCTL, 0);
    let _ = phy.read(MII_TG3_ISTAT);
    let _ = phy.read(MII_TG3_ISTAT);
    let _ = phy.write(MII_TG3_IMASK, 0xFFFF);
    let mut relink = false;
    if d.chip.hw.capacitive_coupling {
        if let Ok(v) = phy.aux_read(MII_TG3_AUXCTL_SHDWSEL_MISCTEST) {
            if v & CAPACITIVE_BIT == 0 {
                let _ = phy.aux_write(MII_TG3_AUXCTL_SHDWSEL_MISCTEST, v | CAPACITIVE_BIT);
                relink = true;
            }
        }
    }
    let mut state = if relink { LinkState::DOWN } else { read_established(&phy, d) };
    if !state.up {
        begin(&phy, d.fast_ethernet_only);
        state.up = link_bit(&phy, 0);
    }
    mac_side::set_port(d, state);
    d.modes.link_up = state.up;
    state
}

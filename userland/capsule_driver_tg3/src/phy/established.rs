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

//! The link the PHY already holds, if it is the one this driver asked for:
//! speed and duplex from AUX_STAT, autonegotiation on, the advertisement
//! unchanged, and the pause the two ends agreed on applied to the MAC.

use crate::constants::phy::*;
use crate::constants::timeouts::{AUX_STAT_MS, BMCR_SETTLE_MS, LINK_UP_MS};
use crate::init::wait;
use crate::link::LinkState;
use crate::setup::Driver;

use super::mac_side;
use super::mdio::Mdio;
use super::negotiate::agreed;
use super::resolve::speed_duplex;

pub fn read_established(phy: &Mdio, d: &mut Driver) -> LinkState {
    if !link_bit(phy, LINK_UP_MS) {
        return LinkState::DOWN;
    }
    let mut aux = 0;
    let _ = wait::until(AUX_STAT_MS, || {
        aux = phy.read(MII_TG3_AUX_STAT).unwrap_or(0);
        aux != 0
    });
    let mut bmcr = 0;
    let _ = wait::until(BMCR_SETTLE_MS, || {
        bmcr = phy.read(MII_BMCR).unwrap_or(0);
        bmcr != 0 && bmcr != 0x7FFF
    });
    let Some((mbps, full)) = speed_duplex(aux) else { return LinkState::DOWN };
    let mut state = LinkState { up: false, mbps, full };
    if bmcr & BMCR_ANENABLE != 0 {
        if let Some((lcl, rmt)) = agreed(phy, full, d.fast_ethernet_only) {
            state.up = true;
            if full {
                mac_side::pause(d, lcl, rmt);
            }
        }
    }
    state
}

/// BMSR's link bit, read twice since it latches low, within `ms`.
pub fn link_bit(phy: &Mdio, ms: u64) -> bool {
    wait::until(ms, || {
        let _ = phy.read(MII_BMSR);
        matches!(phy.read(MII_BMSR), Ok(v) if v & BMSR_LSTATUS != 0)
    })
}

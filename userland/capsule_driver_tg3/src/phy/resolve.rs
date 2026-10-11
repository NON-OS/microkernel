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

//! The pure halves of tg3's copper link handling, so tg3_proofs can hold
//! each to Linux over every register value: what the PHY reports, what this
//! driver advertises and whether the PHY still advertises it.

use crate::constants::phy::*;

/// tg3_aux_stat_to_speed_duplex for a non-FET PHY: (Mb/s, full duplex).
pub fn speed_duplex(aux_stat: u32) -> Option<(u16, bool)> {
    match aux_stat & MII_TG3_AUX_STAT_SPDMASK {
        0x0100 => Some((10, false)),
        0x0200 => Some((10, true)),
        0x0300 => Some((100, false)),
        0x0500 => Some((100, true)),
        0x0600 => Some((1000, false)),
        0x0700 => Some((1000, true)),
        _ => None,
    }
}

/// What tg3_phy_autoneg_cfg writes: MII_ADVERTISE for every 10/100 mode and
/// symmetric pause (mii_advertise_flowctrl of TX | RX), and MII_CTRL1000
/// for both gigabit modes unless the part is 10/100 only.
pub fn advertise(fast_ethernet_only: bool) -> (u32, Option<u32>) {
    let mii = ADVERTISE_CSMA | ADVERTISE_ALL | ADVERTISE_PAUSE_CAP;
    let gig = (!fast_ethernet_only).then_some(ADVERTISE_1000HALF | ADVERTISE_1000FULL);
    (mii, gig)
}

/// tg3_phy_copper_an_config_ok: the PHY still advertises what `advertise`
/// asked for, pause included at full duplex.
pub fn advertising_ok(lcladv: u32, ctrl1000: Option<u32>, full: bool, fe: bool) -> bool {
    let (mut target, mut mask) = (ADVERTISE_ALL, ADVERTISE_ALL);
    if full {
        target |= ADVERTISE_PAUSE_CAP;
        mask |= ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM;
    }
    if lcladv & mask != target {
        return false;
    }
    fe || ctrl1000.map(|c| c & (ADVERTISE_1000HALF | ADVERTISE_1000FULL)) == advertise(fe).1
}

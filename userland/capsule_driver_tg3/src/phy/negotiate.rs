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

//! Reading back what the PHY advertises and the partner offered, and
//! starting autonegotiation over (tg3_phy_copper_an_config_ok,
//! tg3_phy_copper_fetch_rmtadv, tg3_phy_copper_begin with autoneg on).
//! EEE is not advertised: this driver leaves TG3_PHYFLG_EEE_CAP clear, the
//! state Linux falls back to when a 57766 runs without its patch firmware.

use crate::constants::phy::*;

use super::mdio::Mdio;
use super::resolve::{advertise, advertising_ok};

/// The local and partner advertisements when the PHY still advertises what
/// this driver asked for; `None` sends the link back to `begin`.
pub fn agreed(phy: &Mdio, full: bool, fe: bool) -> Option<(u32, u32)> {
    let lcladv = phy.read(MII_ADVERTISE).ok()?;
    let ctrl1000 = if fe { None } else { Some(phy.read(MII_CTRL1000).ok()?) };
    if !advertising_ok(lcladv, ctrl1000, full, fe) {
        return None;
    }
    if !fe {
        phy.read(MII_STAT1000).ok()?;
    }
    let rmtadv = phy.read(MII_LPA).ok()?;
    Some((lcladv, rmtadv))
}

/// Advertise and restart. A failed write leaves the link to the next look,
/// as tg3_phy_copper_begin ignores tg3_phy_autoneg_cfg's result.
pub fn begin(phy: &Mdio, fe: bool) {
    let (mii, gig) = advertise(fe);
    if phy.write(MII_ADVERTISE, mii).is_err() {
        return;
    }
    if let Some(g) = gig {
        if phy.write(MII_CTRL1000, g).is_err() {
            return;
        }
    }
    let _ = phy.write(MII_BMCR, BMCR_ANENABLE | BMCR_ANRESTART);
}

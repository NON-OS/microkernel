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

//! tg3_phy_reset as it runs on the 57765 class: a BMCR reset, auto
//! power-down as the board asks, the extended packet length and FIFO
//! elasticity a jumbo-capable part sets, then auto-MDIX and wire speed on.
//! The OTP, 5784, 5703-5705 and FET branches do not apply to the class.

use crate::constants::phy::*;
use crate::constants::timeouts::BMCR_RESET_MS;
use crate::init::wait;

use super::mdio::{Mdio, NO_ANSWER};

pub fn run(phy: &Mdio, auto_power_down: bool) -> Result<(), &'static str> {
    // BMSR latches; tg3 reads it twice and gives up only if a read fails.
    phy.read(MII_BMSR)?;
    phy.read(MII_BMSR)?;
    bmcr_reset(phy)?;
    toggle_apd(phy, auto_power_down);
    phy.aux_set(MII_TG3_AUXCTL_SHDWSEL_AUXCTL, MII_TG3_AUXCTL_ACTL_EXTPKTLEN);
    if let Ok(v) = phy.read(MII_TG3_EXT_CTRL) {
        let _ = phy.write(MII_TG3_EXT_CTRL, v | MII_TG3_EXT_CTRL_FIFO_ELASTIC);
    }
    phy.aux_set(MII_TG3_AUXCTL_SHDWSEL_MISC, MII_TG3_AUXCTL_MISC_FORCE_AMDIX);
    phy.aux_set(MII_TG3_AUXCTL_SHDWSEL_MISC, MII_TG3_AUXCTL_MISC_WIRESPD_EN);
    Ok(())
}

/// tg3_bmcr_reset: the reset bit self-clears; a settle follows it.
fn bmcr_reset(phy: &Mdio) -> Result<(), &'static str> {
    phy.write(MII_BMCR, BMCR_RESET)?;
    let mut failed = false;
    let cleared = wait::until(BMCR_RESET_MS, || match phy.read(MII_BMCR) {
        Ok(v) => v & BMCR_RESET == 0,
        Err(_) => {
            failed = true;
            true
        }
    });
    if failed {
        return Err(NO_ANSWER);
    }
    if !cleared {
        return Err("PHY reset did not complete");
    }
    wait::hold(1);
    Ok(())
}

/// tg3_phy_toggle_apd for a part that is not a 5784.
fn toggle_apd(phy: &Mdio, enable: bool) {
    let _ = phy.shadow_write(MII_TG3_MISC_SHDW_SCR5_SEL, MII_TG3_MISC_SHDW_SCR5_BITS);
    let mut apd = MII_TG3_MISC_SHDW_APD_WKTM_84MS;
    if enable {
        apd |= MII_TG3_MISC_SHDW_APD_ENABLE;
    }
    let _ = phy.shadow_write(MII_TG3_MISC_SHDW_APD_SEL, apd);
}

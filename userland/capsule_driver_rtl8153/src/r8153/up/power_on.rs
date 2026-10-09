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

//! The front of Linux r8153_init and r8153b_init: wait for the chip to
//! load its settings (AUTOLOAD_DONE), wait for the PHY to finish its own
//! start, set the ADC clock on the first RTL8153s, take the PHY out of
//! power down, and wait until it is on the LAN.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{
    mdio_read, mdio_write, phy_read, phy_write, read_word, wait_until, Dev, PLA,
};
use crate::r8153::regs::bits::AUTOLOAD_DONE;
use crate::r8153::regs::mii::{BMCR_PDOWN, MII_BMCR};
use crate::r8153::regs::phy::{ADC_EN, CKADSEL_L, EN_EMI_L, OCP_ADC_CFG, OCP_PHY_STATUS};
use crate::r8153::regs::phy::{PHY_STAT_EXT_INIT, PHY_STAT_LAN_ON, PHY_STAT_MASK, PHY_STAT_PWRDN};
use crate::r8153::regs::pla::BOOT_CTRL;
use crate::r8153::Version;

/// Linux gives both waits 500 passes of 20 ms.
const WAIT_MS: u64 = 10_000;
const PAUSE_MS: u64 = 20;

pub fn power_on<B: Bus>(dev: &mut Dev<B>, v: Version) -> Result<(), Fail> {
    let loaded = |d: &mut Dev<B>| Ok(read_word(d, PLA, BOOT_CTRL)? & AUTOLOAD_DONE != 0);
    at("autoload not done", wait_until(dev, WAIT_MS, PAUSE_MS, loaded))?;
    at("PHY did not start", phy_state(dev, None))?;
    if matches!(v, Version::V03 | Version::V04 | Version::V05) {
        let adc = CKADSEL_L | ADC_EN | EN_EMI_L;
        at("PHY ADC config refused", phy_write(dev, OCP_ADC_CFG, adc))?;
    }
    let bmcr = at("PHY BMCR unread", mdio_read(dev, MII_BMCR))?;
    if bmcr & BMCR_PDOWN != 0 {
        at("PHY power down not left", mdio_write(dev, MII_BMCR, bmcr & !BMCR_PDOWN))?;
    }
    at("PHY not on the LAN", phy_state(dev, Some(PHY_STAT_LAN_ON)))
}

/// Linux r8153_phy_status: with no state wanted, LAN on, power down or
/// external init each end the wait.
fn phy_state<B: Bus>(dev: &mut Dev<B>, want: Option<u16>) -> Result<(), i32> {
    wait_until(dev, WAIT_MS, PAUSE_MS, |d| {
        let s = phy_read(d, OCP_PHY_STATUS)? & PHY_STAT_MASK;
        Ok(match want {
            Some(w) => s == w,
            None => matches!(s, PHY_STAT_LAN_ON | PHY_STAT_PWRDN | PHY_STAT_EXT_INIT),
        })
    })
}

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

//! The steps from a configured device to a chip that passes traffic once
//! the link is up. Linux runs r8153_init and set_ethernet_addr at probe,
//! the PHY work (r8153_hw_phy_cfg, rtl8152_set_speed) after it, and
//! rtl8153_up at open; here they run back to back in that order.

use nonos_usbnet::Bus;

use super::aldps::aldps_off;
use super::first_init::first_init;
use super::mac::station_address;
use super::phy_cfg::phy_cfg;
use super::power_on::power_on;
use super::rx_agg::rx_one_frame_per_transfer;
use super::speed::autoneg;
use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{update_word, write_dword, Dev, PLA, USB};
use crate::r8153::regs::bits::PLA_MCU_SPDWN_EN;
use crate::r8153::regs::pla::MAC_PWR_CTRL3;
use crate::r8153::regs::usb::{RX_BUF_TH, RX_THR_B};
use crate::r8153::Version;

pub fn bring_up<B: Bus>(dev: &mut Dev<B>, v: Version) -> Result<[u8; 6], Fail> {
    // r8153_init and r8153b_init.
    power_on(dev, v)?;
    rx_one_frame_per_transfer(dev)?;
    // set_ethernet_addr.
    let mac = station_address(dev)?;
    // The PHY work: r8153_hw_phy_cfg, then rtl8152_set_speed.
    aldps_off(dev)?;
    phy_cfg(dev, v)?;
    autoneg(dev)?;
    // rtl8153_up and rtl8153b_up.
    first_init(dev, v)?;
    if v.is_8153b() {
        rtl8153b_up_rest(dev)?;
    }
    Ok(mac)
}

/// What rtl8153b_up adds after r8153_first_init: the RX buffer threshold,
/// and the MCU kept at full clock.
fn rtl8153b_up_rest<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    at("RX buffer threshold refused", write_dword(dev, USB, RX_BUF_TH, RX_THR_B))?;
    let spdwn = update_word(dev, PLA, MAC_PWR_CTRL3, PLA_MCU_SPDWN_EN, 0);
    at("MCU speed down not cleared", spdwn)
}

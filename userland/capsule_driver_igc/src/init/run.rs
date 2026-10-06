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

//! The bring-up, in Linux's order (igc_reset, igc_power_up_link,
//! igc_configure): reset, receive filters under the drawn address, PHY
//! power, link, MAC enables, transmit ring, receive ring.

use crate::setup::Driver;

use super::{control, link, mac_filter, phy, reset, rx_queue, station_address, tx_queue};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    let regs = driver.regs;
    reset::run(&regs)?;
    /*
     * Drawn, not read out of the NVM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk. It is drawn before either enable bit is written, so a
     * card with no address never receives or sends.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    mac_filter::program(&regs, &mac);
    phy::power_up::run(&regs)?;
    link::run(&regs);
    control::enable(&regs);
    tx_queue::program(&regs, &driver.tx, driver.tx_ring_device_addr)?;
    rx_queue::program(&regs, &driver.rx, driver.rx_ring_device_addr)?;
    Ok(())
}

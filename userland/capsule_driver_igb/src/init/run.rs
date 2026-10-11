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
//! The bring-up, in Linux's order (igb_reset, igb_init_hw_82575,
//! igb_power_up_link, igb_configure): reset, the port's media, receive
//! filters under the drawn address, link, PHY power, MAC enables, transmit
//! ring, receive ring.

use crate::setup::Driver;

use super::{control, link, mac_filter, media, phy, reset, rx_queue, station_address, tx_queue};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    let regs = driver.regs;
    let part = driver.part;
    reset::run(&regs, part)?;
    // The reset reloads CTRL_EXT from the NVM, which says what the port is.
    media::copper(&regs)?;
    /*
     * Drawn, not read out of the NVM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk. It is drawn before either enable bit is written, so a
     * card with no address never receives or sends.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    mac_filter::program(&regs, part, &mac);
    link::run(&regs, part);
    phy::power_up::run(&regs, part)?;
    control::enable(&regs);
    tx_queue::program(&regs, &driver.tx, driver.tx_ring_device_addr)?;
    rx_queue::program(&regs, &driver.rx, driver.rx_ring_device_addr)?;
    Ok(())
}

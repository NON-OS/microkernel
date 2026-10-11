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

//! One bring-up in Linux's order (ixgbe_reset_hw, ixgbe_start_hw,
//! ixgbe_configure): reset, the 82599's link mode put back, the port taken
//! from firmware, receive filters under the drawn address, MAC settings,
//! transmit ring, receive ring, then the receiver.

use crate::setup::Driver;

use super::{autoc, control, hw_control, mac_filter, reset, rx_dma, rx_queue};
use super::{station_address, tx_queue};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    let regs = driver.regs;
    let part = driver.part;
    let autoc_before = reset::run(&regs)?;
    autoc::restore(&regs, part, autoc_before);
    hw_control::take(&regs);
    /*
     * Drawn, not read out of the EEPROM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk. It is drawn before either queue is enabled, so a card
     * with no address never receives or sends.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    mac_filter::program(&regs, &mac);
    control::program(&regs);
    tx_queue::program(&regs, &driver.tx, driver.tx_ring_device_addr)?;
    rx_queue::program(&regs, &driver.rx, driver.rx_ring_device_addr)?;
    rx_dma::enable(&regs, part);
    Ok(())
}

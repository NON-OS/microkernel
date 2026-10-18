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

//! A reset part put on the air, in igc_configure's order: receive filters
//! under the station address, PHY power, link, the MAC enables, both rings
//! named with their queues still off, Bus Master on, and only then the two
//! queues. Bus Master goes on after the rings so a part never masters while
//! its queue registers name a previous owner's frames; the queues go on
//! after it, since a queue is what fetches descriptors.

use crate::setup::{command, Driver};

use super::{control, link, mac_filter, phy, rx_queue, tx_queue};

pub fn run(driver: &Driver) -> Result<(), &'static str> {
    let regs = &driver.regs;
    mac_filter::program(regs, &driver.mac);
    phy::power_up::run(regs)?;
    link::run(regs);
    control::enable(regs);
    tx_queue::program(regs, &driver.tx, driver.tx_ring_device_addr);
    rx_queue::program(regs, &driver.rx, driver.rx_ring_device_addr);
    command(driver.device_id, driver.claim_epoch, true)?;
    tx_queue::enable(regs)?;
    rx_queue::enable(regs)
}

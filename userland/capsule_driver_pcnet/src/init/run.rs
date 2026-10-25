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

//! The bring-up, in order: reset and software style, chip id, station
//! address, rings, init block, start. Nothing is handed to the part before
//! the reset has stopped it.

use crate::constants::csr::CSR0;
use crate::constants::layout::INIT_BLOCK_OFF;
use crate::report;
use crate::setup::Driver;

use super::{chip, init_block, reset, start, station_address};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.regs)?;
    driver.part = chip::identify(&driver.regs)?;
    /*
     * Drawn, not read out of the APROM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk. The init block's PADR is the filter and the source the
     * part uses, so the APROM is never read at all.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    driver.rx.prime();
    driver.tx.clear();
    let rx = driver.rx.ring_device_addr as u32;
    let tx = driver.tx.ring_device_addr as u32;
    init_block::store(driver.control_va, &init_block::encode(&mac, rx, tx));
    start::run(&driver.regs, (driver.control_device_addr + INIT_BLOCK_OFF as u64) as u32)?;
    report::up(driver.part, driver.regs.csr(CSR0));
    Ok(())
}

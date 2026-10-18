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

//! The bring-up, in order: reset, station address, then `start`: receive
//! address filter and multicast table, both rings, Bus Master, the engines.
//! The driver's ring state is programmed in place; the server loop reads it
//! afterwards.

use crate::constants::regs::{REG_CTRL, REG_STATUS};
use crate::report;
use crate::setup::Driver;

use super::{reset, start, station_address};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.regs)?;
    /*
     * Drawn, not read out of the EEPROM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk.
     */
    driver.mac = station_address::draw()?;
    start::run(driver)?;
    // SAFETY: `driver.regs` carries the broker MmioMap base for BAR0; CTRL
    // and STATUS are 32-bit aligned offsets in it.
    let (ctrl, status) = unsafe { (driver.regs.r32(REG_CTRL), driver.regs.r32(REG_STATUS)) };
    report::up(ctrl, status);
    Ok(())
}

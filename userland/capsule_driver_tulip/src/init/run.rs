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

//! The bring-up, in order: reset, station address, rings, start. Nothing
//! is handed to the part before the reset has stopped it.

use crate::constants::csr::CSR5;
use crate::report;
use crate::setup::Driver;

use super::{reset, start, station_address};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    reset::run(&driver.regs)?;
    /*
     * Drawn, not read out of the serial ROM. The factory address identifies
     * this card to every network it ever joins, which outlives a system that
     * keeps nothing on disk. The part puts no source address on a frame and
     * filters on what the setup frame loads, so the ROM is never read.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    driver.rx.prime();
    driver.tx.clear();
    start::run(driver, &mac)?;
    report::up(driver.device, driver.regs.csr(CSR5));
    Ok(())
}

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

//! The bring-up, in order: revision, a reset that drops whatever a previous
//! instance left active, the station address, the rings, the shared area,
//! activation, the receive producer, the filter.

use crate::constants::cmd::{GET_LINK, RESET_DEV};
use crate::constants::layout::SHARED_OFF;
use crate::report;
use crate::setup::Driver;

use super::{activate, shared_area, station_address, version};

pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    version::select(&driver.regs)?;
    driver.regs.command(RESET_DEV);
    /*
     * Drawn, not read with GET_PERM_MAC. The hypervisor's address names this
     * virtual card to every network it is bridged onto, across boots of a
     * system that keeps nothing.
     */
    let mac = station_address::draw()?;
    driver.mac = mac;
    driver.tx.reset();
    let prod = driver.rx.reset();
    shared_area::write(driver.control_va, driver.control_dev);
    activate::run(&driver.regs, driver.control_dev + SHARED_OFF as u64, &mac)?;
    driver.regs.rx_prod(prod, 0);
    activate::apply_filter(&driver.regs);
    report::up(driver.regs.command(GET_LINK));
    Ok(())
}

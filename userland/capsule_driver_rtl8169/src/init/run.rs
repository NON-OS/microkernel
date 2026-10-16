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

use crate::hw::initialize;
use crate::setup::Driver;

use super::{mac, reset, start};

/// The per-generation setup, the reset with Bus Master off, the station
/// address drawn, and the part put on the air (`start`). The address is
/// programmed before TE and RE go on, so the part is never enabled under the
/// factory one.
pub fn bring_up(driver: &mut Driver) -> Result<(), &'static str> {
    initialize(&driver.regs, driver.chip.ver);
    reset::run(&driver.regs, driver.chip.ver)?;
    driver.mac = mac::draw()?;
    start::run(driver)
}

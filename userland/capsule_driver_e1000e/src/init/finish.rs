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

//! The end of one bring-up attempt. Programming the part either works and
//! the one `up` line is logged, or every grant goes back before the attempt
//! reports failure, so the next attempt claims the device afresh instead of
//! meeting its own leftover claim.

use crate::constants::regs::REG_STATUS;
use crate::constants::status::STATUS_LU;
use crate::log::Line;
use crate::setup::Driver;

use super::run::bring_up;

pub fn finish(mut driver: Driver) -> Result<Driver, &'static str> {
    if let Err(e) = bring_up(&mut driver) {
        driver.release();
        return Err(e);
    }
    // SAFETY: `driver.regs` is the broker-mapped BAR0 window; STATUS is a
    // 4-byte register inside it.
    let up = unsafe { driver.regs.r32(REG_STATUS) } & STATUS_LU != 0;
    driver.link_logged = Some(up);
    Line::new()
        .text("up ")
        .hex16(driver.pci_device)
        .text(" mac ")
        .mac(&driver.mac)
        .text(" family=")
        .text(driver.family.name())
        .text(if up { " link=up" } else { " link=down" })
        .send();
    Ok(driver)
}

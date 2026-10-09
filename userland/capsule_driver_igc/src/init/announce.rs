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

//! The line a working bring-up prints, "igc: up <device id> <mac>
//! link=<up|down>", and the link state it records so the first change after
//! it is what link_status logs next.

use crate::constants::regs::REG_STATUS;
use crate::link::{decode, LinkState};
use crate::log::Line;
use crate::setup::Driver;

pub fn up(driver: &mut Driver) {
    // SAFETY: `driver.regs` carries a valid broker MmioMap base for BAR0 and
    // REG_STATUS is a 32-bit register inside it (igc_regs.h).
    let state: LinkState = decode(unsafe { driver.regs.r32(REG_STATUS) });
    driver.link = Some(state);
    let mut line = Line::new();
    line.text("up ").hex(driver.pci_device as u32, 4).text(" ");
    for (i, b) in driver.mac.iter().enumerate() {
        if i > 0 {
            line.text(":");
        }
        line.hex(*b as u32, 2);
    }
    line.text(if state.up { " link=up" } else { " link=down" });
    line.send();
}

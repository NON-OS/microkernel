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

use super::{decode, LinkState};
use crate::constants::regs::REG_PHY_STATUS;
use crate::log::Line;
use crate::setup::Driver;

/// Read the link, and log it when it differs from the last one logged:
/// "rtl8169: link up 1000 full", "rtl8169: link down". The first read
/// after bring-up is always logged.
pub fn poll(driver: &mut Driver) -> LinkState {
    // SAFETY: PHYstatus (0x6C..0x6D) lies inside every mapped window.
    let raw = unsafe {
        if driver.chip.ver.is_8125() {
            driver.regs.r16(REG_PHY_STATUS)
        } else {
            driver.regs.r8(REG_PHY_STATUS) as u16
        }
    };
    let now = decode(raw, driver.chip.ver);
    if driver.link != Some(now) {
        driver.link = Some(now);
        say(now);
    }
    now
}

fn say(link: LinkState) {
    if !link.up {
        return Line::new("rtl8169: link down").send();
    }
    let line = Line::new("rtl8169: link up ");
    let line = if link.speed == 0 { line.text("speed unknown") } else { line.dec(link.speed, 1) };
    line.text(if link.full { " full" } else { " half" }).send();
}

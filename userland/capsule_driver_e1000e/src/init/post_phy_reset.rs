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

//! Linux e1000_post_phy_reset_ich8lan for pch_lpt and later: let the PHY
//! settle, then clear the host wake-up bit the reset can leave set. The LCD
//! configuration from the NVM extended region and the OEM bits that follow
//! in Linux need the NVM, which this driver does not read, and are not done.

use crate::constants::phy::{BM_PORT_GEN_CFG, BM_WUC_HOST_WU_BIT};
use crate::constants::timeouts::POST_PHY_RESET_MS;
use crate::constants::Family;
use crate::log::Line;
use crate::phy::access::{read, write};
use crate::regs::Regs;
use crate::swflag;
use crate::wait::sleep_ms;

use super::reset_block;

pub fn run(regs: &Regs, family: Family) {
    if reset_block::blocked(regs) {
        return;
    }
    sleep_ms(POST_PHY_RESET_MS);
    let cleared = swflag::acquire(regs, family).and_then(|()| {
        let v = read(regs, family, BM_PORT_GEN_CFG);
        let r = v.and_then(|v| write(regs, family, BM_PORT_GEN_CFG, v & !BM_WUC_HOST_WU_BIT));
        swflag::release(regs);
        r
    });
    if let Err(e) = cleared {
        // Linux ignores the result of this access too.
        Line::new().text("host wake-up bit not cleared (").text(e).text(")").send();
    }
}

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
//! The MAC told to take whatever the PHY resolves: Go Link Disconnect
//! cleared where the MAC has it, forced speed and duplex cleared, and Set
//! Link Up. Then CTRL_EXT.DRV_LOAD tells manageability firmware a driver
//! owns the port.

use crate::facts::mac::{CTRL_EXT_DRV_LOAD, CTRL_FRCDPX, CTRL_FRCSPD, CTRL_SLU, PHPM_GO_LINKD};
use crate::facts::parts::Part;
use crate::facts::regs::{REG_CTRL, REG_CTRL_EXT, REG_PHPM};
use crate::regs::Regs;

pub fn run(regs: &Regs, part: Part) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (facts::regs); REG_PHPM is only
    // touched on the MACs that have it.
    unsafe {
        let mut ctrl = regs.r32(REG_CTRL);
        ctrl &= !(CTRL_FRCSPD | CTRL_FRCDPX);
        regs.w32(REG_CTRL, ctrl | CTRL_SLU);
        if part.has_go_link_disconnect() {
            let phpm = regs.r32(REG_PHPM);
            regs.w32(REG_PHPM, phpm & !PHPM_GO_LINKD);
        }
        let ext = regs.r32(REG_CTRL_EXT);
        regs.w32(REG_CTRL_EXT, ext | CTRL_EXT_DRV_LOAD);
    }
}

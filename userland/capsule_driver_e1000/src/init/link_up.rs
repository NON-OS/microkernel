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

//! Step 4 of the reset: with the part out of reset and the EEPROM loaded,
//! mask every cause again, clear the causes latched meanwhile, and set the
//! link up with speed detection on.
//!
//! Speed and duplex are left to the PHY, as Linux
//! e1000_copper_link_preconfig does for every part after the 82543: the
//! EEPROM's Init Control Word 1 can load CTRL.FRCSPD, and a MAC forced to
//! its default speed while the PHY negotiates another passes no frame.

use crate::constants::regs::{REG_CTRL, REG_ICR, REG_IMC};
use crate::constants::status::{CTRL_ASDE, CTRL_FRCDPLX, CTRL_FRCSPD, CTRL_LRST, CTRL_SLU};
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a base from a valid broker MmioMap grant;
    // CTRL, ICR and IMC are 32-bit aligned offsets in BAR0 per the 8254x
    // manual.
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let _ = regs.r32(REG_ICR);
        let mut ctrl = regs.r32(REG_CTRL);
        ctrl &= !(CTRL_LRST | CTRL_FRCSPD | CTRL_FRCDPLX);
        ctrl |= CTRL_SLU | CTRL_ASDE;
        regs.w32(REG_CTRL, ctrl);
    }
}

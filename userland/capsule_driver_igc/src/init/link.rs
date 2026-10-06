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

//! igc_setup_copper_link_base with autonegotiation on: forced speed and
//! duplex are cleared and CTRL.SLU set, so the MAC takes whatever the PHY
//! resolves. CTRL_EXT.DRV_LOAD then tells manageability firmware a driver
//! owns the port (igc_get_hw_control, the first step of igc_configure).

use crate::constants::ctrl::{
    CTRL_EXT_DRV_LOAD, CTRL_FD, CTRL_FRCDPX, CTRL_FRCSPD, CTRL_SLU, CTRL_SPEED_MASK,
};
use crate::constants::regs::{REG_CTRL, REG_CTRL_EXT};
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and both
    // offsets are 32-bit registers inside it (igc_regs.h).
    unsafe {
        let mut ctrl = regs.r32(REG_CTRL);
        ctrl &= !(CTRL_FRCSPD | CTRL_FRCDPX | CTRL_SPEED_MASK | CTRL_FD);
        regs.w32(REG_CTRL, ctrl | CTRL_SLU);
        let ext = regs.r32(REG_CTRL_EXT);
        regs.w32(REG_CTRL_EXT, ext | CTRL_EXT_DRV_LOAD);
    }
}

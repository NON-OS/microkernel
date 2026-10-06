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

//! What rts522a_extra_init_hw adds after the RTS5227's steps. The OCP block
//! is powered down because Linux's card_exist is still zero at this point
//! on a first probe; turning the card's power on enables it again.

use crate::error::Result;
use crate::hw::write_register;
use crate::regs::clk::{FPDCTL, OC_POWER_DOWN};
use crate::regs::pm::*;
use crate::setup::Driver;

pub fn extra_init_hw_522a(drv: &Driver) -> Result<()> {
    let regs = drv.regs;
    write_register(regs, FPDCTL, OC_POWER_DOWN, OC_POWER_DOWN)?;
    write_register(regs, FUNC_FORCE_CTL, FUNC_FORCE_UPME_XMT_DBG, FUNC_FORCE_UPME_XMT_DBG)?;
    write_register(regs, PCLK_CTL, 0x04, 0x04)?;
    write_register(regs, PM_EVENT_DEBUG, PME_DEBUG_0, PME_DEBUG_0)?;
    write_register(regs, PM_CLK_FORCE_CTL, 0xFF, 0x11)
}

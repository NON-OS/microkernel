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

//! Over-current protection on the RTS522A (rtsx_pci_init_ocp and
//! rtsx_pci_enable_ocp): 800 us at 800 mA, a 10 MHz glitch filter, and the
//! detector on. The RTS5227 has it off (ocp_en unset).

use crate::chip::Family;
use crate::error::Result;
use crate::hw::write_register;
use crate::regs::clk::{FPDCTL, OC_POWER_DOWN};
use crate::regs::ocp::*;
use crate::setup::Driver;

pub fn init_ocp(drv: &Driver) -> Result<()> {
    if drv.family != Family::Rts522a {
        return Ok(());
    }
    let regs = drv.regs;
    write_register(regs, FPDCTL, OC_POWER_DOWN, 0)?;
    write_register(regs, REG_OCPPARA1, SD_OCP_TIME_MASK, SD_OCP_TIME_800)?;
    write_register(regs, REG_OCPPARA2, SD_OCP_THD_MASK, RTS522A_OCP_THD_800)?;
    write_register(regs, REG_OCPGLITCH, SD_OCP_GLITCH_MASK, SD_OCP_GLITCH_10M)?;
    enable_ocp(drv)
}

pub fn enable_ocp(drv: &Driver) -> Result<()> {
    if drv.family != Family::Rts522a {
        return Ok(());
    }
    write_register(drv.regs, FPDCTL, OC_POWER_DOWN, 0)?;
    write_register(drv.regs, REG_OCPCTL, 0xFF, SD_OCP_INT_EN | SD_DETECT_EN)
}

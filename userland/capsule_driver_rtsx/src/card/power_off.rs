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

//! Powering the slot down after the card leaves (sd_power_off with
//! rts5227_card_power_off). Over-current protection goes off with it on
//! the RTS522A, as rtsx_pci_disable_ocp does.

use super::pull::pull_disable;
use crate::chip::Family;
use crate::error::Result;
use crate::hw::write_register;
use crate::regs::card::*;
use crate::regs::clk::{FPDCTL, OC_POWER_DOWN};
use crate::regs::ocp::{REG_OCPCTL, SD_DETECT_EN, SD_OCP_INT_EN};
use crate::regs::pm::{LDO3318_PWR_MASK, PWR_GATE_CTRL};
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn power_off(drv: &mut Driver) -> Result<()> {
    drv.cur_clock = 0;
    let mut buf = CmdBuf::new();
    buf.write(CARD_CLK_EN, SD_CLK_EN, 0);
    buf.write(CARD_OE, SD_OUTPUT_EN, 0);
    crate::engine::send(drv, &buf, 100)?;
    let regs = drv.regs;
    if drv.family == Family::Rts522a {
        write_register(regs, REG_OCPCTL, SD_OCP_INT_EN | SD_DETECT_EN, 0)?;
        write_register(regs, FPDCTL, OC_POWER_DOWN, OC_POWER_DOWN)?;
    }
    let off = SD_POWER_OFF | PMOS_STRG_400MA;
    write_register(regs, CARD_PWR_CTL, SD_POWER_MASK | PMOS_STRG_MASK, off)?;
    write_register(regs, PWR_GATE_CTRL, LDO3318_PWR_MASK, 0)?;
    pull_disable(drv)
}

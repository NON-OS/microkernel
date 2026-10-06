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

//! Powering the slot (sd_power_on with rts5227_card_power_on): the slot
//! selected and its clock enabled, the pulls set, power through the partial
//! stage so the in-rush current stays low, then the outputs on and the clock
//! left toggling so the card sees its 74 initial clocks.

use super::pull::pull_enable;
use super::select::select_sd;
use crate::clock::sleep_ms;
use crate::error::Result;
use crate::hw::write_register;
use crate::init::enable_ocp;
use crate::regs::card::*;
use crate::regs::pm::{LDO3318_PWR_MASK, PWR_GATE_CTRL};
use crate::regs::sd::{SD_BUS_STAT, SD_CLK_TOGGLE_EN};
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn power_on(drv: &Driver) -> Result<()> {
    sleep_ms(100);
    let mut buf = CmdBuf::new();
    select_sd(&mut buf);
    buf.write(CARD_CLK_EN, SD_CLK_EN, SD_CLK_EN);
    crate::engine::send(drv, &buf, 100)?;
    pull_enable(drv)?;
    enable_ocp(drv)?;
    let mut buf = CmdBuf::new();
    buf.write(CARD_PWR_CTL, SD_POWER_MASK, SD_PARTIAL_POWER_ON);
    buf.write(PWR_GATE_CTRL, LDO3318_PWR_MASK, 0x02);
    crate::engine::send(drv, &buf, 100)?;
    sleep_ms(20);
    let mut buf = CmdBuf::new();
    buf.write(CARD_PWR_CTL, SD_POWER_MASK, SD_POWER_ON);
    buf.write(PWR_GATE_CTRL, LDO3318_PWR_MASK, 0x06);
    buf.write(CARD_OE, SD_OUTPUT_EN, SD_OUTPUT_EN);
    buf.write(CARD_OE, MS_OUTPUT_EN, MS_OUTPUT_EN);
    crate::engine::send(drv, &buf, 100)?;
    sleep_ms(1);
    write_register(drv.regs, CARD_OE, SD_OUTPUT_EN, SD_OUTPUT_EN)?;
    write_register(drv.regs, SD_BUS_STAT, SD_CLK_TOGGLE_EN, SD_CLK_TOGGLE_EN)
}

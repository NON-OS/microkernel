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

//! rts5227_extra_init_hw, with the vendor settings Linux reads from PCI
//! config 0x724 and 0x814 at their defaults: the broker gives a capsule only
//! the first 256 bytes of config space. So the card-detect and write-protect
//! pins are taken as not reversed, RTD3 as off, and CLKREQ# is forced low
//! as Linux does when no L1 substate is enabled. LTR is left unprogrammed.

use super::driving::fill_driving_3v3;
use crate::chip::Family;
use crate::error::Result;
use crate::regs::clk::{GPIO_CTL, OLT_LED_CTL};
use crate::regs::pm::*;
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn extra_init_hw(drv: &Driver) -> Result<()> {
    let a = drv.family == Family::Rts522a;
    let mut buf = CmdBuf::new();
    buf.write(GPIO_CTL, 0x02, 0x02);
    buf.write(ASPM_FORCE_CTL, 0x3F, 0);
    // LDO3318 from DV33 to card_3v3.
    buf.write(LDO_PWR_SEL, 0x03, 0x00);
    buf.write(LDO_PWR_SEL, 0x03, 0x01);
    buf.write(OLT_LED_CTL, 0x0F, 0x02);
    buf.write(OBFF_CFG, 0x03, 0x03);
    fill_driving_3v3(&mut buf);
    buf.write(PETXCFG, 0x20, 0);
    buf.write(PETXCFG, 0x10, 0);
    if a {
        buf.write(RTS522A_AUTOLOAD_CFG1, CD_RESUME_EN_MASK, CD_RESUME_EN_MASK);
        buf.write(RTS522A_PM_CTRL3, 0x01, 0x00);
        buf.write(RTS522A_PME_FORCE_CTL, 0x30, 0x20);
    } else {
        buf.write(PME_FORCE_CTL, 0xFF, 0x30);
        buf.write(PM_CTRL3, 0x01, 0x00);
    }
    buf.write(PETXCFG, FORCE_CLKREQ_DELINK_MASK, FORCE_CLKREQ_LOW);
    buf.write(if a { RTS522A_PM_CTRL3 } else { PM_CTRL3 }, 0x10, 0x00);
    crate::engine::send(drv, &buf, 100)
}

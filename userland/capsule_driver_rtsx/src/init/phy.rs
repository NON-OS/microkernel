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

//! PHY tuning (rts5227_optimize_phy, rts522a_optimize_phy): D3 delink off,
//! then the receive sensitivity value on the RTS5227, or the IC version A
//! values on the RTS522A.

use super::ic::IC_VER_A;
use crate::chip::Family;
use crate::error::Result;
use crate::hw::{write_phy, write_register};
use crate::regs::phy::*;
use crate::regs::pm::{D3_DELINK_MODE_EN, PM_CTRL3, RTS522A_PM_CTRL3};
use crate::setup::Driver;

pub fn optimize_phy(drv: &Driver) -> Result<()> {
    let regs = drv.regs;
    match drv.family {
        Family::Rts5227 => {
            write_register(regs, PM_CTRL3, D3_DELINK_MODE_EN, 0x00)?;
            write_phy(regs, 0x00, 0xBA42)
        }
        Family::Rts522a => {
            write_register(regs, RTS522A_PM_CTRL3, D3_DELINK_MODE_EN, 0x00)?;
            if drv.ic_version != IC_VER_A {
                return Ok(());
            }
            write_phy(regs, PHY_RCR2, PHY_RCR2_INIT_27S)?;
            write_phy(regs, PHY_RCR1, PHY_RCR1_INIT_27S)?;
            write_phy(regs, PHY_FLD0, PHY_FLD0_INIT_27S)?;
            write_phy(regs, PHY_FLD3, PHY_FLD3_INIT_27S)?;
            write_phy(regs, PHY_FLD4, PHY_FLD4_INIT_27S)
        }
    }
}

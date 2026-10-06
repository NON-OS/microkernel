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

//! After the reset, before the hardware bits: mask and clear every cause
//! again (reset_hw_ich8lan and reset_hw_82571 both end so), the PCH band-gap
//! bias and power-gating bits, then tell firmware a driver owns the port
//! (e1000e_get_hw_control, which e1000e_reset runs for every part with AMT)
//! and turn wake-up off (`ew32(WUC, 0)`).

use crate::constants::ctrl::{CTRL_EXT_DPG_EN, CTRL_EXT_DRV_LOAD};
use crate::constants::pch_bits::KABGTXD_BGSQLBIAS;
use crate::constants::regs::{REG_CTRL_EXT, REG_ICR, REG_IMC, REG_WUC};
use crate::constants::regs_pch::REG_KABGTXD;
use crate::constants::Family;
use crate::regs::Regs;

pub fn run(regs: &Regs, family: Family) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; every offset here is
    // a 4-byte register inside it, KABGTXD on the PCH parts only.
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let _ = regs.r32(REG_ICR);
        if family.is_pch() {
            regs.modify(REG_KABGTXD, 0, KABGTXD_BGSQLBIAS);
        }
        // DPG_EN resets to 1 on pch_ptp; left on, the part power-gates itself.
        if family >= Family::PchPtp {
            regs.modify(REG_CTRL_EXT, CTRL_EXT_DPG_EN, 0);
        }
        regs.modify(REG_CTRL_EXT, 0, CTRL_EXT_DRV_LOAD);
        regs.w32(REG_WUC, 0);
    }
}

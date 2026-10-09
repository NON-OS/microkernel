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

//! Whether the PCH PHY answers on the MAC-PHY link, and if it does with the
//! management engine absent, take both ends out of forced SMBus mode. Linux
//! e1000_phy_is_accessible_pchlan, for pch_lpt and later. The caller holds
//! the software flag.

use crate::constants::ctrl::CTRL_EXT_FORCE_SMBUS;
use crate::constants::phy::{CV_SMB_CTRL, CV_SMB_CTRL_FORCE_SMBUS, MII_PHYSID1, MII_PHYSID2};
use crate::constants::regs::REG_CTRL_EXT;
use crate::constants::regs_pch::REG_FWSM;
use crate::constants::status::FWSM_FW_VALID;
use crate::constants::Family;
use crate::regs::Regs;

use super::access::{read_once, write_once};

pub fn accessible(regs: &Regs, family: Family) -> bool {
    if !answers(regs, family) {
        return false;
    }
    // SAFETY: `regs` is the broker-mapped BAR0 window; FWSM and CTRL_EXT are
    // 4-byte registers inside it on the PCH parts.
    let me_active = unsafe { regs.r32(REG_FWSM) } & FWSM_FW_VALID != 0;
    if !me_active {
        // Switching the interface always ends this access in an MDI error,
        // so the write's result says nothing; Linux ignores it too.
        if let Ok(v) = read_once(regs, family, CV_SMB_CTRL) {
            let _ = write_once(regs, family, CV_SMB_CTRL, v & !CV_SMB_CTRL_FORCE_SMBUS);
        }
        // SAFETY: as above.
        unsafe {
            let ext = regs.r32(REG_CTRL_EXT);
            regs.w32(REG_CTRL_EXT, ext & !CTRL_EXT_FORCE_SMBUS);
        }
    }
    true
}

/// Two tries at both PHY ID words; an all-ones word is a PHY not driving MDIO.
fn answers(regs: &Regs, family: Family) -> bool {
    for _ in 0..2 {
        let hi = read_once(regs, family, MII_PHYSID1);
        let lo = read_once(regs, family, MII_PHYSID2);
        if matches!((hi, lo), (Ok(h), Ok(l)) if h != 0xFFFF && l != 0xFFFF) {
            return true;
        }
    }
    false
}

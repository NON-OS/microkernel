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

//! The ULP exit the driver does itself when no ME firmware is running, from
//! Linux e1000_disable_ulp_lpt_lp: power-cycle the PHY, take it out of forced
//! SMBus mode (forcing the MAC into SMBus first if the PHY only answers
//! there), re-enable K1, clear and restart the PHY's ULP configuration, and
//! let SMBus go on PERST#. The caller holds the software flag.

use crate::constants::ctrl::CTRL_EXT_FORCE_SMBUS;
use crate::constants::pch_bits::FEXTNVM7_DISABLE_SMB_PERST;
use crate::constants::phy::{
    CV_SMB_CTRL, CV_SMB_CTRL_FORCE_SMBUS, HV_PM_CTRL, HV_PM_CTRL_K1_ENABLE, I218_ULP_CONFIG1,
    I218_ULP_CONFIG1_CLEAR, I218_ULP_CONFIG1_START,
};
use crate::constants::regs::REG_CTRL_EXT;
use crate::constants::regs_pch::REG_FEXTNVM7;
use crate::constants::timeouts::SMBUS_SETTLE_MS;
use crate::constants::Family;
use crate::phy::access::{read, read_once, write, write_once};
use crate::regs::Regs;
use crate::wait::sleep_ms;

use super::lanphypc;

pub fn run(regs: &Regs, family: Family) -> Result<(), &'static str> {
    lanphypc::toggle(regs);
    let smb = match read_once(regs, family, CV_SMB_CTRL) {
        Ok(v) => v,
        Err(_) => {
            // SAFETY: `regs` is the broker-mapped BAR0 window; CTRL_EXT and
            // FEXTNVM7 are 4-byte registers inside it on the PCH parts.
            unsafe { regs.modify(REG_CTRL_EXT, 0, CTRL_EXT_FORCE_SMBUS) };
            sleep_ms(SMBUS_SETTLE_MS);
            read_once(regs, family, CV_SMB_CTRL)
                .map_err(|_| "PHY did not answer over SMBus during ULP exit")?
        }
    };
    // Switching the link ends this write in an MDI error; Linux ignores it.
    let _ = write_once(regs, family, CV_SMB_CTRL, smb & !CV_SMB_CTRL_FORCE_SMBUS);
    // SAFETY: as above.
    unsafe { regs.modify(REG_CTRL_EXT, CTRL_EXT_FORCE_SMBUS, 0) };
    // ULP entry turned K1 off in the PHY; it is turned back on on the way out.
    let pm = read(regs, family, HV_PM_CTRL).map_err(|_| "HV_PM_CTRL read failed in ULP exit")?;
    let _ = write(regs, family, HV_PM_CTRL, pm | HV_PM_CTRL_K1_ENABLE);
    let ulp = read(regs, family, I218_ULP_CONFIG1)
        .map_err(|_| "ULP_CONFIG1 read failed in ULP exit")?
        & !I218_ULP_CONFIG1_CLEAR;
    let _ = write(regs, family, I218_ULP_CONFIG1, ulp);
    let _ = write(regs, family, I218_ULP_CONFIG1, ulp | I218_ULP_CONFIG1_START);
    // SAFETY: as above.
    unsafe { regs.modify(REG_FEXTNVM7, FEXTNVM7_DISABLE_SMB_PERST, 0) };
    Ok(())
}

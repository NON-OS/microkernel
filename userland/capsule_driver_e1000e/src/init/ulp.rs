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

//! Linux e1000_disable_ulp_lpt_lp with `force`: take the PHY out of Ultra
//! Low Power mode, which firmware or Intel ME can leave set across a reboot,
//! and in which the PHY never answers on PCIe. With ME running it is asked
//! to do it (H2ME) and the driver waits; without, the driver does it.

use crate::constants::ids::NO_ULP;
use crate::constants::regs_pch::{REG_FWSM, REG_H2ME};
use crate::constants::status::{FWSM_FW_VALID, FWSM_ULP_CFG_DONE, H2ME_ENFORCE_SETTINGS, H2ME_ULP};
use crate::constants::timeouts::{SMBUS_SETTLE_MS, ULP_ME_MS};
use crate::constants::Family;
use crate::regs::Regs;
use crate::setup::Driver;
use crate::swflag;
use crate::wait::{idle_until, sleep_ms};

use super::{phy_reset, ulp_host};

pub fn disable(d: &Driver) -> Result<(), &'static str> {
    let regs = &d.regs;
    if d.family < Family::PchLpt || NO_ULP.contains(&d.pci_device) {
        return Ok(());
    }
    // SAFETY: `regs` is the broker-mapped BAR0 window; FWSM and H2ME are
    // 4-byte registers inside it on the PCH parts.
    if unsafe { regs.r32(REG_FWSM) } & FWSM_FW_VALID != 0 {
        return by_me(regs);
    }
    swflag::acquire(regs, d.family)?;
    let done = ulp_host::run(regs, d.family);
    swflag::release(regs);
    // Linux resets the PHY and waits 50 ms whether or not the exit worked.
    let reset = phy_reset::run(regs, d.family);
    sleep_ms(SMBUS_SETTLE_MS);
    done.and(reset)
}

fn by_me(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: as in `disable`.
    unsafe { regs.modify(REG_H2ME, H2ME_ULP, H2ME_ENFORCE_SETTINGS) };
    // SAFETY: as in `disable`.
    let cleared = || unsafe { regs.r32(REG_FWSM) } & FWSM_ULP_CFG_DONE == 0;
    if !idle_until(ULP_ME_MS, cleared) {
        return Err("ME did not clear ULP_CFG_DONE in 2500 ms");
    }
    // SAFETY: as in `disable`.
    unsafe { regs.modify(REG_H2ME, H2ME_ENFORCE_SETTINGS, 0) };
    Ok(())
}

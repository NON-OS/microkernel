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

//! The MAC-PHY link check of Linux e1000_init_phy_workarounds_pchlan, for
//! pch_lpt and later: if the PHY does not answer, force SMBus and look
//! again; if it still does not, power-cycle it with LANPHYPC (unless ME
//! forbids PHY resets) and look with SMBus forced and then released. The
//! caller holds the software flag.

use crate::constants::ctrl::CTRL_EXT_FORCE_SMBUS;
use crate::constants::regs::REG_CTRL_EXT;
use crate::constants::timeouts::SMBUS_SETTLE_MS;
use crate::constants::Family;
use crate::phy::probe::accessible;
use crate::regs::Regs;
use crate::wait::sleep_ms;

use super::{k1, lanphypc, reset_block};

pub fn run(regs: &Regs, family: Family) -> Result<(), &'static str> {
    if family >= Family::PchMtp {
        // Linux does not let a failure here stop it: the PHY may not answer yet.
        let _ = k1::reconfigure(regs, family);
    }
    if accessible(regs, family) {
        return Ok(());
    }
    // SAFETY: `regs` is the broker-mapped BAR0 window; CTRL_EXT is a 4-byte
    // register inside it.
    unsafe { regs.modify(REG_CTRL_EXT, 0, CTRL_EXT_FORCE_SMBUS) };
    // Time for the MAC to finish retries of earlier PHY reads.
    sleep_ms(SMBUS_SETTLE_MS);
    if accessible(regs, family) {
        return Ok(());
    }
    if reset_block::blocked(regs) {
        return Err("PHY silent and LANPHYPC toggle blocked by ME");
    }
    lanphypc::toggle(regs);
    if accessible(regs, family) {
        return Ok(());
    }
    // The toggle took the PHY out of SMBus; the MAC follows.
    // SAFETY: as above.
    unsafe { regs.modify(REG_CTRL_EXT, CTRL_EXT_FORCE_SMBUS, 0) };
    if accessible(regs, family) {
        return Ok(());
    }
    Err("PHY not reachable after LANPHYPC toggle")
}

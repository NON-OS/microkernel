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

//! Taking the flag, the PCH way and the 82574 way.

use crate::constants::regs::REG_EXTCNF_CTRL;
use crate::constants::status::EXTCNF_CTRL_SWFLAG;
use crate::constants::timeouts::{MDIO_OWNERSHIP_MS, SWFLAG_FREE_MS, SWFLAG_GRANT_MS};
use crate::constants::Family;
use crate::regs::Regs;
use crate::wait::idle_until;

pub fn acquire(regs: &Regs, family: Family) -> Result<(), &'static str> {
    if family.is_pch() {
        pch(regs)
    } else {
        mdio_ownership(regs)
    }
}

/*
 * e1000_acquire_swflag_ich8lan: wait for the flag to be free, set it, and
 * wait for it to read back set. Firmware that holds it, or will not let go,
 * fails the step, and the flag is put back clear as Linux leaves it.
 */
fn pch(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: `regs` is the broker-mapped BAR0 window; EXTCNF_CTRL is a
    // 4-byte-aligned register inside it on every family this driver claims.
    let read = || unsafe { regs.r32(REG_EXTCNF_CTRL) };
    if !idle_until(SWFLAG_FREE_MS, || read() & EXTCNF_CTRL_SWFLAG == 0) {
        return Err("swflag held by firmware for 100 ms");
    }
    // SAFETY: as above.
    unsafe { regs.w32(REG_EXTCNF_CTRL, read() | EXTCNF_CTRL_SWFLAG) };
    if !idle_until(SWFLAG_GRANT_MS, || read() & EXTCNF_CTRL_SWFLAG != 0) {
        super::release(regs);
        return Err("swflag not granted in 1000 ms");
    }
    Ok(())
}

/// e1000_get_hw_semaphore_82573: set ownership and read it back, again and
/// again until it sticks or the budget runs out.
fn mdio_ownership(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: as in `pch`.
    let granted = idle_until(MDIO_OWNERSHIP_MS, || unsafe {
        regs.w32(REG_EXTCNF_CTRL, regs.r32(REG_EXTCNF_CTRL) | EXTCNF_CTRL_SWFLAG);
        regs.r32(REG_EXTCNF_CTRL) & EXTCNF_CTRL_SWFLAG != 0
    });
    if !granted {
        super::release(regs);
        return Err("MDIO ownership not granted in 40 ms");
    }
    Ok(())
}

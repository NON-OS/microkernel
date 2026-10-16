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

//! The end of igc_reset_hw_base: CTRL.RST, then igc_get_auto_rd_done waits
//! for EECD.AUTO_RD, then every cause is masked again and ICR read to clear
//! what is latched. Linux does not wait for CTRL.RST to clear; this driver
//! does, on the clock: AUTO_RD read before the reset lands is the last
//! reset's, and the queues are only known off once it has.
//!
//! A missing auto-read is not fatal, as in Linux: a blank-NVM part (0x125F,
//! 0x15FD) never finishes one, and the station address is drawn, not read.
//! It is logged so a photo shows it.

use crate::constants::ctrl::{CTRL_RST, EECD_AUTO_RD};
use crate::constants::regs::{REG_CTRL, REG_EECD, REG_ICR, REG_IMC};
use crate::constants::timeouts::{AUTO_READ_MS, RESET_MS};
use crate::init::wait::until;
use crate::log::Line;
use crate::regs::Regs;

/// Quote AUTO_READ_MS and RESET_MS; igc_proofs holds them equal.
pub const AUTO_READ_LATE: &str = "NVM auto read not done in 20 ms, going on as Linux does";
pub const RESET_STUCK: &str = "CTRL.RST did not self-clear in 100 ms";

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (igc_regs.h).
    unsafe {
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_RST);
    }
    // SAFETY: as above.
    if !until(RESET_MS, || unsafe { regs.r32(REG_CTRL) } & CTRL_RST == 0) {
        return Err(RESET_STUCK);
    }
    // SAFETY: as above.
    let loaded = || unsafe { regs.r32(REG_EECD) } & EECD_AUTO_RD != 0;
    if !until(AUTO_READ_MS, loaded) {
        Line::new().text(AUTO_READ_LATE).send();
    }
    // SAFETY: as above.
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let _ = regs.r32(REG_ICR);
    }
    Ok(())
}

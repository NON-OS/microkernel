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

//! The reset: the part quieted (quiet.rs), CTRL.RST set, and the NVM read
//! that follows waited for; then every cause masked again and what is
//! latched cleared.
//!
//! An NVM read that does not finish is not fatal: a flashless I210 has no
//! NVM to read, and the station address is drawn, not read. It is logged
//! so a photo of the console shows it. An 82575 with no NVM is refused: it
//! needs an init script of SerDes, clock and lane settings (as observed in
//! Linux igb, unverified) that this driver does not carry.

use crate::bring::quiet;
use crate::bring::wait::until;
use crate::facts::bounds::AUTO_READ_MS;
use crate::facts::mac::{CTRL_RST, EECD_AUTO_RD, EECD_PRES};
use crate::facts::parts::Part;
use crate::facts::regs::{REG_CTRL, REG_EECD, REG_ICR, REG_IMC};
use crate::log::Line;
use crate::regs::Regs;

/// Quotes AUTO_READ_MS; igb_proofs holds the two equal.
pub const AUTO_READ_LATE: &str = "NVM auto read not done in 20 ms, going on: the address is drawn";
pub const NO_NVM_82575: &str = "82575 with no NVM needs an init script not carried here";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and every offset is a 32-bit register inside it (facts::regs).
 */

pub fn run(regs: &Regs, part: Part) -> Result<(), &'static str> {
    quiet::run(regs)?;
    unsafe {
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_RST);
    }
    let loaded = || unsafe { regs.r32(REG_EECD) } & EECD_AUTO_RD != 0;
    if !until(AUTO_READ_MS, loaded) {
        Line::new().text(AUTO_READ_LATE).send();
    }
    if part == Part::I82575 && unsafe { regs.r32(REG_EECD) } & EECD_PRES == 0 {
        return Err(NO_NVM_82575);
    }
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let _ = regs.r32(REG_ICR);
    }
    Ok(())
}

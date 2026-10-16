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

//! The reset: AUTOC read so the Mode stage can put the 82599's link mode
//! back, the part quieted (quiet.rs), then CTRL.RST twice, each seen to
//! clear and given time to settle. The second reset is there for error
//! states one does not clear (as observed, unverified). Then
//! the EEPROM read is waited for, not fatally, and every cause masked again.

use super::quiet::{self, mask_all};
use crate::bring::wait::{hold, until};
use crate::facts::bounds::{AUTO_READ_MS, RESET_CLEAR_MS, RESET_SETTLE_MS};
use crate::facts::mac::{CTRL_RST, CTRL_RST_MASK, EEC_ARD};
use crate::facts::regs::{REG_AUTOC, REG_CTRL, REG_EEC, REG_EICR};
use crate::log::Line;
use crate::regs::Regs;

/// Quotes RESET_CLEAR_MS; ixgbe_proofs holds the two equal.
pub const RESET_STUCK: &str = "CTRL.RST did not clear in 3 ms";
/// Quotes AUTO_READ_MS.
pub const AUTO_READ_LATE: &str = "EEPROM auto read not done in 10 ms, going on";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and every offset is a 32-bit register inside it (facts::regs).
 */

/// AUTOC as it was before the reset.
pub fn run(regs: &Regs) -> Result<u32, &'static str> {
    let autoc = unsafe { regs.r32(REG_AUTOC) };
    quiet::run(regs)?;
    for _ in 0..2 {
        unsafe {
            let ctrl = regs.r32(REG_CTRL);
            regs.w32(REG_CTRL, ctrl | CTRL_RST);
        }
        hold(1);
        let done = || unsafe { regs.r32(REG_CTRL) } & CTRL_RST_MASK == 0;
        if !until(RESET_CLEAR_MS, done) {
            return Err(RESET_STUCK);
        }
        hold(RESET_SETTLE_MS);
    }
    let loaded = || unsafe { regs.r32(REG_EEC) } & EEC_ARD != 0;
    if !until(AUTO_READ_MS, loaded) {
        Line::new().text(AUTO_READ_LATE).send();
    }
    unsafe {
        mask_all(regs);
        let _ = regs.r32(REG_EICR);
    }
    Ok(autoc)
}

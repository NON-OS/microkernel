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

//! The MAC reset as ixgbe_reset_hw_82599 and _X540 run it: CTRL.RST, wait
//! for it to self-clear, settle, and again, the double reset "required for
//! recovery from certain error conditions". Then the EEPROM auto-read is
//! waited for, and every cause masked again.

use super::quiesce::mask_all;
use crate::constants::ctrl::{CTRL_RST, CTRL_RST_MASK, EEC_ARD};
use crate::constants::regs::{REG_CTRL, REG_EEC, REG_EICR};
use crate::constants::timeouts::{AUTO_READ_MS, RESET_CLEAR_MS, RESET_SETTLE_MS};
use crate::init::wait::{hold, until};
use crate::log::Line;
use crate::regs::Regs;

/// Quotes RESET_CLEAR_MS; ixgbe_proofs holds the two equal.
pub const RESET_STUCK: &str = "CTRL.RST did not clear in 3 ms";
/// Quotes AUTO_READ_MS.
pub const AUTO_READ_LATE: &str = "EEPROM auto read not done in 10 ms, going on";

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    for _ in 0..2 {
        // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and
        // every offset is a 32-bit register inside it (ixgbe_type.h).
        unsafe {
            let ctrl = regs.r32(REG_CTRL);
            regs.w32(REG_CTRL, ctrl | CTRL_RST);
        }
        hold(1);
        // SAFETY: as above.
        let done = || unsafe { regs.r32(REG_CTRL) } & CTRL_RST_MASK == 0;
        if !until(RESET_CLEAR_MS, done) {
            return Err(RESET_STUCK);
        }
        hold(RESET_SETTLE_MS);
    }
    // SAFETY: as above.
    let loaded = || unsafe { regs.r32(REG_EEC) } & EEC_ARD != 0;
    if !until(AUTO_READ_MS, loaded) {
        Line::new().text(AUTO_READ_LATE).send();
    }
    // SAFETY: as above.
    unsafe {
        mask_all(regs);
        let _ = regs.r32(REG_EICR);
    }
    Ok(())
}

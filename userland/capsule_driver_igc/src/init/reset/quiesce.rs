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

//! The middle of igc_reset_hw_base: mask every interrupt cause, stop the
//! receiver, leave the transmitter with only PSP set, flush the posted
//! writes with a read, and let the part settle before the reset.

use crate::constants::regs::{REG_IMC, REG_RCTL, REG_STATUS, REG_TCTL};
use crate::constants::timeouts::QUIESCE_SETTLE_MS;
use crate::constants::tx_bits::TCTL_PSP;
use crate::init::wait::hold;
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (igc_regs.h).
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, TCTL_PSP);
        let _ = regs.r32(REG_STATUS);
    }
    hold(QUIESCE_SETTLE_MS);
}

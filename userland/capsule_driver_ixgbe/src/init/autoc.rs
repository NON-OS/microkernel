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

//! The 82599's link mode across the reset. ixgbe_reset_hw_82599 keeps the
//! AUTOC it found ("orig_autoc") and writes it back, restarting
//! autonegotiation, when the reset left another: the reset reloads AUTOC
//! from the EEPROM, which on many boards is not the mode the port's SFP+,
//! backplane or KX4 link was brought up in. The X540 and X550 run their
//! copper PHY on their own and have no AUTOC to keep.

use crate::constants::ctrl::AUTOC_AN_RESTART;
use crate::constants::part::Part;
use crate::constants::regs::REG_AUTOC;
use crate::regs::Regs;

pub fn restore(regs: &Regs, part: Part, before: u32) {
    if !part.is_82599() {
        return;
    }
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and AUTOC
    // is a 32-bit register inside it (ixgbe_type.h).
    unsafe {
        if regs.r32(REG_AUTOC) != before {
            regs.w32(REG_AUTOC, before | AUTOC_AN_RESTART);
        }
    }
}

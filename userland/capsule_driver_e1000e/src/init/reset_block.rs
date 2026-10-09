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

//! Whether the management engine forbids resetting the PHY: Linux
//! e1000_check_reset_block_ich8lan waits up to 300 ms for FWSM.RSPCIPHY and
//! reports the reset blocked if it never shows.

use crate::constants::regs_pch::REG_FWSM;
use crate::constants::status::FWSM_RSPCIPHY;
use crate::constants::timeouts::RESET_BLOCK_MS;
use crate::regs::Regs;
use crate::wait::idle_until;

pub fn blocked(regs: &Regs) -> bool {
    // SAFETY: `regs` is the broker-mapped BAR0 window; FWSM is a 4-byte
    // register inside it on the PCH parts.
    !idle_until(RESET_BLOCK_MS, || unsafe { regs.r32(REG_FWSM) } & FWSM_RSPCIPHY != 0)
}

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

//! One MDIC cycle to the internal PHY, bounded on the clock. The caller holds
//! the PHY semaphore (swfw.rs), as igc_read_phy_reg_gpy does around it.

use crate::constants::regs::REG_MDIC;
use crate::constants::timeouts::MDIC_MS;
use crate::init::wait::until;
use crate::regs::Regs;

use super::mdic_word::{outcome, read_cmd, write_cmd, Mdic};

pub const READ_SLOW: &str = "mdic read did not complete in 100 ms";
pub const READ_FAILED: &str = "mdic read reported a PHY error";
pub const WRITE_SLOW: &str = "mdic write did not complete in 100 ms";
pub const WRITE_FAILED: &str = "mdic write reported a PHY error";

pub fn read(regs: &Regs, reg: u32) -> Result<u16, &'static str> {
    issue(regs, read_cmd(reg)?, READ_SLOW, READ_FAILED)
}

pub fn write(regs: &Regs, reg: u32, data: u16) -> Result<(), &'static str> {
    issue(regs, write_cmd(reg, data)?, WRITE_SLOW, WRITE_FAILED).map(|_| ())
}

fn issue(
    regs: &Regs,
    cmd: u32,
    slow: &'static str,
    failed: &'static str,
) -> Result<u16, &'static str> {
    let mut last = 0u32;
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and
    // REG_MDIC is a 32-bit register inside it (igc_regs.h). The command
    // carries READY clear, so a READY left from the last cycle is not seen.
    unsafe { regs.w32(REG_MDIC, cmd) };
    let _ = until(MDIC_MS, || {
        // SAFETY: as above.
        last = unsafe { regs.r32(REG_MDIC) };
        outcome(last) != Mdic::Busy
    });
    match outcome(last) {
        Mdic::Done(data) => Ok(data),
        Mdic::Failed => Err(failed),
        Mdic::Busy => Err(slow),
    }
}

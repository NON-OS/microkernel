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

//! One UIC command (ufshcd_send_uic_cmd, polled): wait for UCRDY, clear a
//! stale completion, write the three arguments then the command, wait for
//! IS.UCCS, and read the result out of UCMDARG2.

use super::regs::{HCS, HCS_UCRDY, IS, IS_UCCS, UCMDARG1, UCMDARG2, UCMDARG3, UICCMD};
use super::uic_codes::RESULT_MASK;
use super::wait::until;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::time::UIC_MS;

/// Run `cmd` with its arguments; Ok carries UCMDARG3, which a DME_GET
/// answers with. Each wait is UIC_CMD_TIMEOUT.
pub fn run<H: Hw>(hw: &H, cmd: u32, args: [u32; 3]) -> Result<u32, UfsError> {
    if !until(hw, UIC_MS, || hw.r32(HCS) & HCS_UCRDY != 0) {
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UCCS);
    hw.w32(UCMDARG1, args[0]);
    hw.w32(UCMDARG2, args[1]);
    hw.w32(UCMDARG3, args[2]);
    hw.w32(UICCMD, cmd);
    if !until(hw, UIC_MS, || hw.r32(IS) & IS_UCCS != 0) {
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UCCS);
    match hw.r32(UCMDARG2) & RESULT_MASK {
        0 => Ok(hw.r32(UCMDARG3)),
        r => Err(UfsError::Uic(r as u8)),
    }
}

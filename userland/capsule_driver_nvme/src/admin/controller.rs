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

use super::cc_value::cc_value;
use super::disable_step::{await_ready_before_disable, disable_start, DisableStart};
use super::ready_wait::wait_ready;
use crate::clock::uptime_ms;
use crate::constants::{REG_CC, REG_CSTS};
use crate::controller::ControllerInfo;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

/// Reset the controller: wait out an enable firmware left half done, clear
/// CC.EN, and wait for RDY (and CFS) to clear.
pub fn reset_to_disabled(regs: Regs, info: ControllerInfo) -> NvmeResult<()> {
    let csts = || unsafe { regs.r32(REG_CSTS) };
    match disable_start(unsafe { regs.r32(REG_CC) }, csts()) {
        DisableStart::Gone => return Err(NvmeError::UnsupportedController),
        DisableStart::AwaitReady => await_ready_before_disable(info.cap, csts, uptime_ms)?,
        DisableStart::Clear => {}
    }
    unsafe { regs.w32(REG_CC, 0) };
    wait_ready(info.cap, false, csts, uptime_ms)
}

pub fn enable(regs: Regs, info: ControllerInfo) -> NvmeResult<()> {
    let cc = cc_value(info.cap)?;
    unsafe { regs.w32(REG_CC, cc) };
    wait_ready(info.cap, true, || unsafe { regs.r32(REG_CSTS) }, uptime_ms)
}

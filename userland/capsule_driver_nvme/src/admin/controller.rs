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
use super::shutdown_step::shutdown_budget_ms;
use super::shutdown_wait::await_shutdown;
use crate::clock::uptime_ms;
use crate::constants::{CC_SHN_MASK, CC_SHN_NORMAL, REG_CC, REG_CSTS};
use crate::controller::ControllerInfo;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

/// Reset the controller: wait out an enable firmware left half done, clear
/// CC.EN, and wait for RDY (and CFS) to clear.
pub fn reset_to_disabled(regs: Regs, info: ControllerInfo) -> NvmeResult<()> {
    // SAFETY: CSTS lies in the register block's first 0x40 bytes, inside the
    // window bring-up mapped (ControllerInfo::read).
    let csts = || unsafe { regs.r32(REG_CSTS) };
    // SAFETY: CC, likewise.
    match disable_start(unsafe { regs.r32(REG_CC) }, csts()) {
        DisableStart::Gone => return Err(NvmeError::UnsupportedController),
        DisableStart::AwaitReady => await_ready_before_disable(info.cap, csts, uptime_ms)?,
        DisableStart::Clear => {}
    }
    // SAFETY: CC, likewise.
    unsafe { regs.w32(REG_CC, 0) };
    wait_ready(info.cap, false, csts, uptime_ms)
}

pub fn enable(regs: Regs, info: ControllerInfo) -> NvmeResult<()> {
    let cc = cc_value(info.cap)?;
    // SAFETY: CC and CSTS lie in the register block's first 0x40 bytes,
    // inside the window bring-up mapped (ControllerInfo::read).
    unsafe { regs.w32(REG_CC, cc) };
    // SAFETY: as above.
    wait_ready(info.cap, true, || unsafe { regs.r32(REG_CSTS) }, uptime_ms)
}

/// Tell the controller power is going (CC.SHN 01b) and wait, bounded by
/// RTD3E as `shutdown_budget_ms` caps it, for CSTS.SHST to read complete.
/// EN stays set: a shut-down controller takes no further command (NVMe 2.0,
/// 3.6.2), and power goes before any would be sent.
pub fn shut_down(regs: Regs, rtd3e_us: u32) -> NvmeResult<()> {
    // SAFETY: CC and CSTS lie in the register block's first 0x40 bytes,
    // inside the window bring-up mapped (ControllerInfo::read).
    let cc = unsafe { regs.r32(REG_CC) };
    if cc == u32::MAX {
        return Err(NvmeError::UnsupportedController);
    }
    // SAFETY: as above.
    unsafe { regs.w32(REG_CC, (cc & !CC_SHN_MASK) | CC_SHN_NORMAL) };
    let budget = shutdown_budget_ms(rtd3e_us);
    // SAFETY: as above.
    await_shutdown(budget, || unsafe { regs.r32(REG_CSTS) }, uptime_ms)
}

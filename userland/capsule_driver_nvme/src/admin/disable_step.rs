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

//! What the reset does before it clears CC.EN. Firmware that used the drive
//! to boot, or an earlier attempt, can leave CC.EN set with CSTS.RDY still
//! clear: the controller is part way through its own enable. NVMe (3.5.4,
//! controller shutdown and reset) asks the host to wait for RDY=1 then, since
//! clearing EN under a transition that has not finished is undefined, and
//! real controllers have been seen to wedge on it. A fatal status ends that
//! wait too, as clearing EN is the reset that clears CFS. Pure, so the host
//! proofs hold the decision and the loop.

use super::ready_step::ready_timeout_ms;
use crate::constants::{CC_EN, CSTS_CFS, CSTS_RDY};
use crate::error::{NvmeError, NvmeResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableStart {
    /// CC.EN is set and CSTS shows neither RDY nor CFS: wait for one of them
    /// before clearing EN.
    AwaitReady,
    /// Clear CC.EN now.
    Clear,
    /// CSTS reads all ones: nothing answers at BAR0.
    Gone,
}

/// The first step of the reset, from CC and CSTS as the driver found them.
pub const fn disable_start(cc: u32, csts: u32) -> DisableStart {
    if csts == u32::MAX {
        return DisableStart::Gone;
    }
    if (cc & CC_EN) != 0 && (csts & (CSTS_RDY | CSTS_CFS)) == 0 {
        return DisableStart::AwaitReady;
    }
    DisableStart::Clear
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreDisableStep {
    /// Read CSTS again.
    Wait,
    /// RDY or CFS came up, or CAP.TO ran out: clear EN now. A controller
    /// whose enable never finished is reset anyway, as that is the only way
    /// left to bring it back.
    Clear,
}

/// One CSTS read while waiting to clear EN. All ones is a device gone from
/// the bus, which no reset brings back.
pub fn pre_disable_step(cap: u64, csts: u32, elapsed_ms: u64) -> NvmeResult<PreDisableStep> {
    if csts == u32::MAX {
        return Err(NvmeError::UnsupportedController);
    }
    if (csts & (CSTS_RDY | CSTS_CFS)) != 0 || elapsed_ms >= ready_timeout_ms(cap) {
        return Ok(PreDisableStep::Clear);
    }
    Ok(PreDisableStep::Wait)
}

/// Wait, bounded by CAP.TO, until EN may be cleared. The register read and
/// the clock are passed in as for `wait_ready`; a failed clock read ends the
/// wait as `ClockFailed`.
pub fn await_ready_before_disable<R, C>(cap: u64, mut read_csts: R, mut now_ms: C) -> NvmeResult<()>
where
    R: FnMut() -> u32,
    C: FnMut() -> Option<u64>,
{
    let start = now_ms().ok_or(NvmeError::ClockFailed)?;
    loop {
        let csts = read_csts();
        let now = now_ms().ok_or(NvmeError::ClockFailed)?;
        match pre_disable_step(cap, csts, now.saturating_sub(start))? {
            PreDisableStep::Wait => core::hint::spin_loop(),
            PreDisableStep::Clear => return Ok(()),
        }
    }
}

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

//! The normal shutdown before power goes (NVMe 2.0, 3.6.2): the host sets
//! CC.SHN to 01b and waits for CSTS.SHST to read 10b, the controller's word
//! that its volatile state is on the medium. How long that may take the
//! controller says in RTD3E; the wait is that long, capped, so a machine
//! never stays up for a drive that does not finish. Pure, so the host proofs
//! hold the budget and the loop.

use crate::constants::{CSTS_CFS, CSTS_SHST_COMPLETE, CSTS_SHST_MASK};
use crate::error::{NvmeError, NvmeResult};

/// When RTD3E is 0 the controller names no latency: wait this long.
pub const SHUTDOWN_DEFAULT_MS: u64 = 5_000;
/// The most a power-off waits on one controller, whatever RTD3E says.
pub const SHUTDOWN_CAP_MS: u64 = 10_000;

/// The shutdown wait for a controller reporting `rtd3e_us`: RTD3E rounded up
/// to whole milliseconds and capped, or the default when it reports none.
pub const fn shutdown_budget_ms(rtd3e_us: u32) -> u64 {
    if rtd3e_us == 0 {
        return SHUTDOWN_DEFAULT_MS;
    }
    let ms = (rtd3e_us as u64).div_ceil(1_000);
    if ms > SHUTDOWN_CAP_MS {
        SHUTDOWN_CAP_MS
    } else {
        ms
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShutdownStep {
    /// Read CSTS again.
    Wait,
    /// SHST reads complete.
    Complete,
}

/// One CSTS read during the wait. All ones is a device gone from the bus; a
/// fatal status ends the wait, as no shutdown will complete under it.
pub fn shutdown_step(csts: u32, elapsed_ms: u64, budget_ms: u64) -> NvmeResult<ShutdownStep> {
    if csts == u32::MAX {
        return Err(NvmeError::UnsupportedController);
    }
    if csts & CSTS_SHST_MASK == CSTS_SHST_COMPLETE {
        return Ok(ShutdownStep::Complete);
    }
    if csts & CSTS_CFS != 0 {
        return Err(NvmeError::ControllerFatal);
    }
    if elapsed_ms >= budget_ms {
        return Err(NvmeError::ShutdownTimeout);
    }
    Ok(ShutdownStep::Wait)
}

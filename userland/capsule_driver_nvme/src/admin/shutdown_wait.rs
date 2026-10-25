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

//! The wait on a normal shutdown (shutdown_step.rs), the register read and
//! the clock passed in so the host proofs drive it.

use super::shutdown_step::{shutdown_step, ShutdownStep};
use crate::error::{NvmeError, NvmeResult};

/// Wait, bounded by `budget_ms`, for SHST to read complete. The register read
/// and the clock are passed in as for `await_ready_before_disable`.
pub fn await_shutdown<R, C>(budget_ms: u64, mut read_csts: R, mut now_ms: C) -> NvmeResult<()>
where
    R: FnMut() -> u32,
    C: FnMut() -> Option<u64>,
{
    let start = now_ms().ok_or(NvmeError::ClockFailed)?;
    loop {
        let csts = read_csts();
        let now = now_ms().ok_or(NvmeError::ClockFailed)?;
        match shutdown_step(csts, now.saturating_sub(start), budget_ms)? {
            ShutdownStep::Wait => core::hint::spin_loop(),
            ShutdownStep::Complete => return Ok(()),
        }
    }
}

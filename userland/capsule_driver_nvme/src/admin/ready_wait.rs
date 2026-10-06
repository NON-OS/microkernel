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

use super::ready_step::{ready_step, ReadyStep};
use crate::error::{NvmeError, NvmeResult};

/// Poll CSTS until RDY matches `want_ready`, the controller reports a fatal
/// status, or the time CAP.TO allows runs out. The register read and the
/// monotonic clock are passed in, so the host proofs drive this same loop
/// with a scripted controller and a scripted clock. CSTS is read before the
/// clock, so a controller that turned ready just as the time ran out still
/// counts. A clock read that fails (None) ends the wait as `ClockFailed`:
/// with no clock the wait has no bound, and the attempt goes back to the
/// retry schedule instead of spinning for good.
pub fn wait_ready<R, C>(
    cap: u64,
    want_ready: bool,
    mut read_csts: R,
    mut now_ms: C,
) -> NvmeResult<()>
where
    R: FnMut() -> u32,
    C: FnMut() -> Option<u64>,
{
    let start = now_ms().ok_or(NvmeError::ClockFailed)?;
    loop {
        let csts = read_csts();
        let now = now_ms().ok_or(NvmeError::ClockFailed)?;
        match ready_step(cap, csts, want_ready, now.saturating_sub(start)) {
            ReadyStep::Wait => core::hint::spin_loop(),
            ReadyStep::Reached => return Ok(()),
            ReadyStep::Failed(e) => return Err(e),
        }
    }
}

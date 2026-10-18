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

//! The order recovery takes, over a controller passed in, so the host
//! proofs drive it with a scripted one. An Abort is tried only on a
//! controller that reads ready and not fatal; a fatal or disabled one would
//! not answer it. Whatever the Abort does not settle is reset away.

use super::state::Recovered;
use crate::constants::{CSTS_CFS, CSTS_RDY};
use crate::error::NvmeResult;

/// How long the stuck command gets, once the controller says it aborted it,
/// to come back with its own completion. An aborted command completes with
/// Command Abort Requested around the Abort's answer; two seconds covers a
/// slow controller, and one still out after that is reset away.
pub const ABORT_GRACE_MS: u64 = 2_000;

/// What recovery does to the controller. Every step is bounded in time.
pub trait Stuck {
    /// CSTS as it reads now.
    fn csts(&mut self) -> u32;
    /// Send an Abort for the stuck command and wait for its answer: whether
    /// the controller says it aborted the command.
    fn abort(&mut self) -> NvmeResult<bool>;
    /// Wait at most `ms` for the stuck command's own completion; at 0, look
    /// at the ring once.
    fn grace(&mut self, ms: u64) -> NvmeResult<()>;
    /// Disable the controller, rebuild its queues, enable it again.
    fn reset(&mut self) -> NvmeResult<()>;
}

/// Recover from a command whose wait ran out.
pub fn recover(c: &mut impl Stuck) -> Recovered {
    let csts = c.csts();
    if csts == u32::MAX {
        return Recovered::Gone;
    }
    // An Abort the controller says it carried out sends the command's own
    // completion back, and that gets the grace. Refused, answered with DW0
    // bit 0 set (not aborted) or not answered at all, the command may still
    // have completed before the Abort reached it, so the ring is looked at
    // once. A command still out after that is reset away.
    let ready = csts & (CSTS_RDY | CSTS_CFS) == CSTS_RDY;
    if ready {
        let grace = if matches!(c.abort(), Ok(true)) { ABORT_GRACE_MS } else { 0 };
        if c.grace(grace).is_ok() {
            return Recovered::Aborted;
        }
    }
    match c.reset() {
        Ok(()) => Recovered::Reset,
        Err(_) => Recovered::Failed,
    }
}

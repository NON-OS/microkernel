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

//! When a read, write or flush must be answered by: within REPLY_BY_MS of
//! being taken, or by the bound a recovery left (bound), whichever is
//! sooner. Its waits get what is left, and with nothing left it is refused
//! before any command goes out. The bound is the kernel's request's
//! (served): every request is held to it, only the kernel's spends it.

use super::constants::{COMPLETION_TIMEOUT_MS, REPLY_BY_MS};
use super::queue::IoQueue;
use super::retry::RETRY_WINDOW_MS;
use crate::clock::{self, Budget};
use crate::error::NvmeError::{ClockFailed, ControllerTimeout};
use crate::error::NvmeResult;

impl IoQueue {
    /// Run one request's `op` (retry), within RETRY_WINDOW_MS of its start
    /// and before its answer is due: REPLY_BY_MS from now, or the bound a
    /// recovery left. One past its answer is refused before anything goes
    /// out, and with no clock no wait can be bounded, so nothing goes out
    /// either. `sender`: the pid that sent it, 0 the kernel. A wait that
    /// runs out is never retried here.
    pub(crate) fn retried<F>(&mut self, sender: u32, op: F) -> NvmeResult<()>
    where
        F: FnMut(&mut IoQueue) -> NvmeResult<()>,
    {
        let (bound, now) = (self.held(sender), clock::uptime_ms());
        let Some(start) = now else { return self.refused(sender, bound, ClockFailed) };
        let by = bound.unwrap_or(u64::MAX).min(start.saturating_add(REPLY_BY_MS));
        if by <= start {
            return self.refused(sender, bound, ControllerTimeout);
        }
        (self.answer_by, self.taken) = (Some(by), start);
        let done = self.tries(op, by.min(start.saturating_add(RETRY_WINDOW_MS)));
        self.served_under(sender, bound);
        done
    }
}

/// A wait's budget from now: COMPLETION_TIMEOUT_MS, cut to what is left
/// before the request in progress must be answered (`answer_by`).
pub(super) fn wait_budget(answer_by: Option<u64>) -> Budget {
    let now = clock::uptime_ms();
    let left = match (answer_by, now) {
        (Some(by), Some(now)) => COMPLETION_TIMEOUT_MS.min(by.saturating_sub(now)),
        _ => COMPLETION_TIMEOUT_MS,
    };
    Budget::begin(now, left)
}

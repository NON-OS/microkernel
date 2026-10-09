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

//! A wait's time budget on a clock that can fail. A failed uptime read used
//! to count as zero: a wait begun on a failed read measured every later
//! failed read as no time at all, and one begun on a good read never reached
//! its end once the reads failed, so the controller's silence was waited on
//! for good. Here a failed read spends the budget at once, and the attempt
//! fails and goes back to the retry schedule. Pure, so the host proofs hold it.

#[derive(Clone, Copy, Debug)]
pub struct Budget {
    start: Option<u64>,
    limit_ms: u64,
}

impl Budget {
    /// A budget of `limit_ms` starting at the clock read `now`.
    pub const fn begin(now: Option<u64>, limit_ms: u64) -> Self {
        Self { start: now, limit_ms }
    }

    /// Whether the budget is gone at the clock read `now`: `limit_ms` have
    /// passed since the start, or either read failed. A clock that went
    /// backwards counts as no time passed.
    pub const fn spent(&self, now: Option<u64>) -> bool {
        match (self.start, now) {
            (Some(start), Some(now)) => now.saturating_sub(start) >= self.limit_ms,
            _ => true,
        }
    }
}

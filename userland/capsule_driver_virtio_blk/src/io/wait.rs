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

//! Waiting for the request in flight, bounded by the clock (plan/50 C7).
//!
//! The wait used to count passes: only slices slept out whole counted
//! toward five seconds, and every wake toward 5000 passes. A shared line
//! firing just under the slice held a request for 5000 slices, over eight
//! minutes; one firing at once gave up on a healthy request in a few
//! milliseconds. Now the budget is time: each sleep is the slice or what is
//! left, whichever is less, and the request ends when none is left,
//! however often the line fires. Pure over its three questions, so the
//! host proofs drive it with a scripted line and clock.

use crate::constants::WAIT_SLICE_MS;
use crate::queue::Fault;

/// Until `done` answers: `left` says how many milliseconds the request has
/// left, and `sleep` waits on the interrupt for at most the given number.
/// The used ring is looked at before the clock, so a completion that
/// arrived in the last sleep is taken, not timed out.
pub(super) fn wait(
    mut done: impl FnMut() -> Option<Result<(), Fault>>,
    mut sleep: impl FnMut(u64) -> Result<(), Fault>,
    mut left: impl FnMut() -> u64,
) -> Result<(), Fault> {
    loop {
        if let Some(answer) = done() {
            return answer;
        }
        let ms = left();
        if ms == 0 {
            return Err(Fault::TimedOut);
        }
        sleep(ms.min(WAIT_SLICE_MS))?;
    }
}

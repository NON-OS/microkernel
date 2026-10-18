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

//! How long `bring_up` keeps trying, by the clock. Seven attempts
//! were a bound on tries, not on time: an attempt that waits on its device
//! for seconds made them minutes, and one that fails at once spent only the
//! sleeps between them. A driver now tries until BRINGUP_BUDGET_MS have
//! passed since its first attempt began, sleeping the schedule's doubling
//! delay between tries, never past the budget. Pure, so it is proven on the
//! host.

/// How long a driver keeps trying a present device that will not come up.
pub const BRINGUP_BUDGET_MS: u64 = 10_000;

/// After a failed attempt `elapsed_ms` past the first one's start, with the
/// schedule's next sleep `delay_ms`: the sleep before trying again, or None
/// to give up.
pub const fn retry_after(elapsed_ms: u64, delay_ms: u64) -> Option<u64> {
    if elapsed_ms >= BRINGUP_BUDGET_MS {
        return None;
    }
    let left = BRINGUP_BUDGET_MS - elapsed_ms;
    Some(if delay_ms < left { delay_ms } else { left })
}

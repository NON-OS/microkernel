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

//! Waiting on a unit by the clock. A count of loop passes is a different
//! length of time on every CPU model and core count; milliseconds of uptime
//! are the same everywhere.

use crate::sys::timer::tsc::tsc_frequency;
use crate::sys::timer::uptime::uptime_ms;

/// Poll `done` until it holds or `budget_ms` of uptime pass, and say which.
/// Before the TSC is calibrated uptime never advances, so a wait then is one
/// look, not a hang.
pub fn wait_ms(budget_ms: u64, mut done: impl FnMut() -> bool) -> bool {
    if done() {
        return true;
    }
    if tsc_frequency() == 0 {
        return false;
    }
    let start = uptime_ms();
    loop {
        if done() {
            return true;
        }
        if uptime_ms().saturating_sub(start) >= budget_ms {
            // One last look: the unit may have answered while the clock was read.
            return done();
        }
        core::hint::spin_loop();
    }
}

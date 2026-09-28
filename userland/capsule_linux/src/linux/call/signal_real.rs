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

//! ITIMER_REAL on the family's monotonic clock: arming it, and what it has
//! left. alarm and setitimer both come here.

use crate::linux::call::now_ms;
use crate::linux::guest::sigtimer::Itimer;
use crate::linux::guest::Guest;

const CLOCK_MONOTONIC: u64 = 1;

/// Arm ITIMER_REAL for `value` ms, repeating every `interval`; 0 disarms.
/// Answers what it had left and its old interval.
pub fn arm(guest: &mut Guest, value: u64, interval: u64) -> (u64, u64) {
    let was = remaining(guest);
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    guest.signals.real = (value != 0).then(|| Itimer { due: now.saturating_add(value), interval });
    was
}

pub fn remaining(guest: &Guest) -> (u64, u64) {
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    guest.signals.real.map_or((0, 0), |t| (t.due.saturating_sub(now).max(1), t.interval))
}

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

//! The monotonic clock, a sleep the scheduler can park, and the pause
//! between two polls of a wait.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::budget::Budget;

/// How long a wait polls without a break. A register access or a short
/// command buffer ends in microseconds, and a parked sleep would cost it a
/// scheduler tick; a transfer that is still going after this is slept on.
/// Two is at least a whole millisecond on a millisecond clock, where one
/// ends microseconds in on a wait begun late in one.
const SPIN_MS: u64 = 2;

/// `None` when the clock cannot be read; a Budget then counts as spent.
pub fn now() -> Option<u64> {
    let t = mk_uptime_ms();
    if t < 0 {
        None
    } else {
        Some(t as u64)
    }
}

/// Linux's msleep and the udelay/mdelay of a millisecond or less all
/// become one parked sleep of at least `ms`.
pub fn sleep_ms(ms: u64) {
    let _ = mk_idle_ms(ms.max(1));
}

/// Between two polls of a wait: nothing in its first millisecond or so,
/// then a parked millisecond each time, so a stuck transfer holds no core
/// (C7).
pub fn pause(budget: &Budget, now: Option<u64>) {
    if !budget.early(now, SPIN_MS) {
        sleep_ms(1);
    }
}

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

//! The clock as the kernel answers it, and the waits built on it.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::Budget;

/// Milliseconds since boot, or `None` when the clock cannot be read.
pub fn uptime_ms() -> Option<u64> {
    u64::try_from(mk_uptime_ms()).ok()
}

/// A budget of `ms` from now.
pub fn budget(ms: u64) -> Budget {
    Budget::begin(uptime_ms(), ms)
}

/// A polled wait's give-up test: false, after a millisecond's sleep, until
/// `ms` have passed since it was made or the clock stops answering.
pub fn expired_after(ms: u64) -> impl FnMut() -> bool {
    let budget = budget(ms);
    move || {
        if budget.left(uptime_ms()) == 0 {
            return true;
        }
        let _ = mk_idle_ms(1);
        false
    }
}

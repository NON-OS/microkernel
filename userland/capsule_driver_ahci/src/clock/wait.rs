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

use super::Deadline;

/// Wait until `cond` holds or `ms` milliseconds pass. True when it held; the
/// condition is read once more after the deadline, so a register that
/// settled while the clock was read is not taken for a timeout.
pub fn wait_until(ms: u64, mut cond: impl FnMut() -> bool) -> bool {
    let deadline = Deadline::after_ms(ms);
    loop {
        if cond() {
            return true;
        }
        if deadline.expired() {
            return cond();
        }
        core::hint::spin_loop();
    }
}

/// Spin for `ms` milliseconds.
pub fn pause_ms(ms: u64) {
    let deadline = Deadline::after_ms(ms);
    while !deadline.expired() {
        core::hint::spin_loop();
    }
}

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

//! Busy waits on the uptime clock: the reset's fixed settle times, and its
//! wait for the part to answer.

use nonos_libc::Deadline;

// At least `ms` milliseconds: uptime counts whole milliseconds, so a deadline
// `ms` ahead can fall due up to one early.
pub fn hold_ms(ms: u64) {
    let until = Deadline::after_ms(ms + 1);
    while !until.expired() {
        core::hint::spin_loop();
    }
}

/// True once `done` holds, false only after at least `ms` milliseconds (one
/// more is added, as in `hold_ms`). `done` is asked once more after the
/// deadline, so a part that answered while this thread was preempted is not
/// reported as silent.
pub fn until(ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let deadline = Deadline::after_ms(ms + 1);
    while !done() {
        if deadline.expired() {
            return done();
        }
        core::hint::spin_loop();
    }
    true
}

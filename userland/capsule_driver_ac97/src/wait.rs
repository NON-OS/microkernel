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

//! Waits on the uptime clock, asleep a millisecond between looks so a
//! codec that takes its time does not cost a spinning core.

use nonos_libc::{mk_idle_ms, Deadline};

/// Whether `done` came to hold within `ms` milliseconds. It is asked once
/// more at the deadline, so a condition met during the last sleep counts.
pub fn until(ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let deadline = Deadline::after_ms(ms);
    loop {
        if done() {
            return true;
        }
        if deadline.expired() {
            return done();
        }
        let _ = mk_idle_ms(1);
    }
}

/// At least `ms` milliseconds: uptime counts whole milliseconds, so a
/// deadline `ms` ahead can fall due up to one early.
pub fn hold(ms: u64) {
    let deadline = Deadline::after_ms(ms + 1);
    while !deadline.expired() {
        let _ = mk_idle_ms(1);
    }
}

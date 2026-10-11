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

//! Waiting on the part, in milliseconds of uptime. Pass rates change with the
//! core count and the scheduler, so no wait here counts loop passes, and each
//! pass sleeps with `mk_idle_ms` rather than holding a core.

use nonos_libc::{mk_idle_ms, Deadline};

/// True once `done` holds, false if `ms` milliseconds pass first. `done` is
/// asked once more after the deadline, so a part that answered during the
/// last sleep is not reported as silent.
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

/// At least `ms` milliseconds. Uptime counts whole milliseconds, so a
/// deadline `ms` ahead can fall due up to one early; one more is added.
pub fn hold(ms: u64) {
    let deadline = Deadline::after_ms(ms + 1);
    while !deadline.expired() {
        let _ = mk_idle_ms(1);
    }
}

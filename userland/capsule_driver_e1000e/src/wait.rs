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

//! Waiting on the uptime clock. Linux counts loop passes of a fixed sleep;
//! here every bound is milliseconds of `mk_uptime_ms`, because a pass takes
//! a different time with every core count and scheduler load.

use nonos_libc::{mk_idle_ms, Deadline};

/// Sleep at least `ms`. Uptime counts whole milliseconds, so a deadline `ms`
/// ahead can fall due up to one early; one more is added.
pub fn sleep_ms(ms: u64) {
    let until = Deadline::after_ms(ms + 1);
    while !until.expired() {
        let _ = mk_idle_ms(1);
    }
}

/// Poll `done` until it holds or `ms` pass, sleeping between looks. For the
/// waits Linux sleeps in (milliseconds per pass), where a core spun for the
/// whole budget would be taken from everything else.
pub fn idle_until(ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let until = Deadline::after_ms(ms + 1);
    loop {
        if done() {
            return true;
        }
        if until.expired() {
            return done();
        }
        let _ = mk_idle_ms(1);
    }
}

/// Poll `done` without sleeping, for the MDIC handshake, which Linux polls
/// every 50 us and which completes in microseconds on a working part.
pub fn spin_until(ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let until = Deadline::after_ms(ms + 1);
    loop {
        if done() {
            return true;
        }
        if until.expired() {
            return done();
        }
        core::hint::spin_loop();
    }
}

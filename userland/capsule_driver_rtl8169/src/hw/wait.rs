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

use nonos_libc::Deadline;

/*
 * Linux rtl_loop_wait_high/_low poll a condition n times with a d us sleep
 * between, n * d in all. Here the same total is a deadline on the clock,
 * rounded up to whole milliseconds plus one, since uptime counts whole
 * milliseconds and a deadline `ms` ahead can fall due up to one early. A
 * count of polls would change with core count and clock; the clock does not.
 */

/// Poll `cond` until it reads `want` or `ms` milliseconds pass. True when it
/// read `want`; the last look is taken after the deadline, so a condition
/// that came true while the caller was preempted is still seen.
pub fn wait_for(ms: u64, want: bool, mut cond: impl FnMut() -> bool) -> bool {
    let deadline = Deadline::after_ms(ms + 1);
    loop {
        if cond() == want {
            return true;
        }
        if deadline.expired() {
            return cond() == want;
        }
        core::hint::spin_loop();
    }
}

/// At least `ms` milliseconds (Linux fsleep / msleep in the steps copied).
pub fn hold_ms(ms: u64) {
    let until = Deadline::after_ms(ms + 1);
    while !until.expired() {
        core::hint::spin_loop();
    }
}

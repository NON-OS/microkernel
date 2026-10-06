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

//! A busy wait on the uptime clock, for the reset's fixed settle times.

use nonos_libc::Deadline;

// At least `ms` milliseconds: uptime counts whole milliseconds, so a deadline
// `ms` ahead can fall due up to one early.
pub fn hold_ms(ms: u64) {
    let until = Deadline::after_ms(ms + 1);
    while !until.expired() {
        core::hint::spin_loop();
    }
}

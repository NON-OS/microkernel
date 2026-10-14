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

//! The pause between two polls of the controller. The controller answers a
//! command within microseconds and a device within a millisecond or two, so
//! the first few polls follow each other at once; past those the wait is
//! for a slow device (a keyboard's self test takes up to half a second) and
//! sleeps a millisecond between polls instead of holding a core through it.

use nonos_libc::mk_idle_ms;

/// Polls made back to back before the wait starts to sleep.
const FAST_POLLS: u32 = 8;

pub struct Pause(u32);

impl Pause {
    pub const fn new() -> Self {
        Pause(0)
    }

    pub fn wait(&mut self) {
        if self.0 < FAST_POLLS {
            self.0 += 1;
            core::hint::spin_loop();
        } else {
            let _ = mk_idle_ms(1);
        }
    }
}

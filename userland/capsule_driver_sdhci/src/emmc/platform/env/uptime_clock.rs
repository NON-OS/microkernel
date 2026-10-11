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

//! The kernel uptime as the engine's millisecond clock.

use nonos_libc::mk_uptime_ms;

use super::super::super::env::Clock;

/// Milliseconds since boot, from the kernel. A refused read is the end of
/// time (`Clock`), never zero: a deadline set or checked against zero would
/// not pass while the clock stays broken.
#[derive(Clone, Copy, Default)]
pub struct UptimeClock;

impl Clock for UptimeClock {
    fn now_ms(&self) -> u64 {
        let t = mk_uptime_ms();
        if t < 0 {
            u64::MAX
        } else {
            t as u64
        }
    }
}

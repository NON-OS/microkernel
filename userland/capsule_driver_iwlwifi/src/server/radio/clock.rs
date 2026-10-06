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

//! The gen3 `Clock` on the uptime clock. A wait first re-checks its condition
//! in a short spin (register handshakes finish in microseconds), then sleeps a
//! millisecond between checks until the deadline. The number of sleeps is
//! capped as well, so a wait still ends if the uptime clock never advances.

use nonos_libc::{mk_idle_ms, mk_uptime_ms, Deadline};

use crate::firmware::gen3::region::Clock;

/// Checks made before the first sleep.
const SPINS: u32 = 2000;

pub struct Uptime;

/// Milliseconds since boot, or zero if the clock cannot be read.
pub fn now_ms() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}

impl Clock for Uptime {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        let deadline = Deadline::after_ms(u64::from(ms));
        for _ in 0..SPINS {
            if ready() {
                return true;
            }
            core::hint::spin_loop();
        }
        let naps_max = ms.saturating_mul(2).saturating_add(10);
        let mut naps = 0u32;
        loop {
            if ready() {
                return true;
            }
            if deadline.expired() || naps >= naps_max {
                return ready();
            }
            let _ = mk_idle_ms(1);
            naps += 1;
        }
    }

    // At least the asked time: whole milliseconds, plus one for the clock's
    // granularity, slept a millisecond at a time against the deadline.
    fn delay_us(&mut self, us: u32) {
        let ms = u64::from(us).div_ceil(1000).saturating_add(1);
        let deadline = Deadline::after_ms(ms);
        let mut naps = 0u64;
        while !deadline.expired() && naps < ms.saturating_mul(4) {
            let _ = mk_idle_ms(1);
            naps += 1;
        }
    }

    fn now_ms(&mut self) -> u64 {
        now_ms()
    }
}

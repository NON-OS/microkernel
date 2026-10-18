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

//! Where a wait stands on the uptime clock, which counts whole milliseconds:
//! begun late in one, and looked at again as a later one begins.

use std::time::{Duration, Instant};

use nonos_libc::uptime_ms;

/// 0.9 ms into a millisecond of uptime, where a deadline is worst placed.
pub fn late_in_a_millisecond() {
    let tick = uptime_ms();
    while uptime_ms() == tick {
        std::hint::spin_loop();
    }
    let edge = Instant::now();
    while edge.elapsed() < Duration::from_micros(900) {
        std::hint::spin_loop();
    }
}

/// Sleeps to the millisecond before uptime `count`, then spins (too short
/// a spin to run out its time slice): true if the count was seen to reach
/// it within 20 us of a read that found it short, so what follows comes as
/// that millisecond begins. A thread preempted anywhere in between is late.
pub fn land_on(count: u64) -> bool {
    while uptime_ms() + 1 < count {
        std::thread::sleep(Duration::from_micros(200));
    }
    let mut short = (Instant::now(), uptime_ms() < count);
    loop {
        let began = Instant::now();
        if uptime_ms() >= count {
            return short.1 && short.0.elapsed() < Duration::from_micros(20);
        }
        short = (began, true);
    }
}

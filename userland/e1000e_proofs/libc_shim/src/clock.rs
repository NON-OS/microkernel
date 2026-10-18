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

//! The uptime clock and the idle sleep, on the host clock. The driver's
//! waits are in milliseconds of this clock, so a test that times one out
//! takes that long and can measure that it did.

use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(timeout_ms) }
    }

    /// Past the deadline, and, while a device model is running
    /// (nonos_devmodel::run), only after that model has taken two more
    /// turns, as rtl8169_proofs' clock does. The driver waits on this while
    /// the model answers from another thread; on a loaded builder the driver
    /// can hold the core past its whole deadline before the model is
    /// scheduled, and a reset the model would have completed reads as a
    /// timeout. With this the driver's last look always follows the model's
    /// turn; a part that never answers still times out, on the same clock.
    /// Five seconds bound the wait for a model that has stopped.
    pub fn expired(&self) -> bool {
        if Instant::now() < self.end {
            return false;
        }
        if let Some(seen) = nonos_devmodel::turns() {
            let give_up = Instant::now() + Duration::from_secs(5);
            while nonos_devmodel::turns().is_some_and(|t| t < seen + 2) && Instant::now() < give_up
            {
                std::thread::yield_now();
            }
        }
        true
    }
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    std::thread::sleep(Duration::from_millis(ms));
    0
}

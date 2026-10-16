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

//! The uptime deadline the reset waits on and the transmit watchdog keeps,
//! on the host clock counted as the real one is: whole milliseconds since
//! the first look. A deadline `ms` ahead can then fall due up to one
//! millisecond short of `ms`, as it can on the machine, and the tests wait
//! the reset's bounds for real. A test's part can act on each look
//! (`on_clock`).

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::look::look;

static START: OnceLock<Instant> = OnceLock::new();

/// Whole milliseconds since the first look.
pub fn uptime_ms() -> u64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}

pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        look();
        Self { end_ms: uptime_ms().saturating_add(timeout_ms) }
    }

    /// Past the deadline, and, while a device model is running
    /// (nonos_devmodel::run), only after that model has taken two more
    /// turns, as rtl8169_proofs' clock does. The driver busy-waits on this
    /// while the model answers from another thread; on a loaded builder the
    /// driver can hold the core past its whole deadline before the model is
    /// scheduled, and a reset the model would have completed reads as a
    /// timeout. With this the driver's last look always follows the model's
    /// turn; a part that never answers still times out, on the same clock.
    /// Five seconds bound the wait for a model that has stopped.
    pub fn expired(&self) -> bool {
        look();
        if uptime_ms() < self.end_ms {
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

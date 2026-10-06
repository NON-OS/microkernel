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

//! The clock the driver waits on: milliseconds since the first look, from
//! the host's monotonic clock, and the same `Deadline` the real libc has.

use std::sync::OnceLock;
use std::time::Instant;

static START: OnceLock<Instant> = OnceLock::new();

pub fn mk_uptime_ms() -> i64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as i64
}

#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self::at((mk_uptime_ms() as u64).saturating_add(timeout_ms))
    }

    pub const fn at(end_ms: u64) -> Self {
        Self { end_ms }
    }

    pub const fn is_past(&self, now_ms: u64) -> bool {
        now_ms >= self.end_ms
    }

    /// Past the wall-clock deadline, and, while a device model is running
    /// (nonos_devmodel::run), only after that model has taken two more
    /// turns. The driver busy-waits on this while the model answers from
    /// another thread; on a loaded builder the driver can hold the core past
    /// its whole deadline before the model is scheduled, and a reset the
    /// model would have completed reads as a timeout. With this the driver's
    /// last look always follows the model's turn; a part that never answers
    /// still times out, on the same clock. Five seconds bound the wait for a
    /// model that has stopped.
    pub fn expired(&self) -> bool {
        if !self.is_past(mk_uptime_ms() as u64) {
            return false;
        }
        if let Some(seen) = nonos_devmodel::turns() {
            let give_up = Instant::now() + std::time::Duration::from_secs(5);
            while nonos_devmodel::turns().is_some_and(|t| t < seen + 2) && Instant::now() < give_up {
                std::thread::yield_now();
            }
        }
        true
    }
}

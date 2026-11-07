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

//! Nanoseconds since the harness started: ticks at one per nanosecond, so
//! the capsule's line printer reads them as it reads calibrated cycles.

use std::time::Instant;

pub const PER_MS: u64 = 1_000_000;

pub struct Clock(Instant);

impl Clock {
    pub fn new() -> Clock {
        Clock(Instant::now())
    }

    /// Never 0, which the record reads as "not stamped".
    pub fn now(&self) -> u64 {
        u64::try_from(self.0.elapsed().as_nanos()).unwrap_or(u64::MAX).max(1)
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new()
    }
}

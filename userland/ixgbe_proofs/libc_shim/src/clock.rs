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

//! A deadline and a sleep on the host clock, with the capsule's API. The
//! driver's waits are milliseconds of uptime, and the tests wait them for
//! real, so a timeout path takes as long here as on the machine.

use std::time::{Duration, Instant};

pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        Instant::now() >= self.end
    }
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    std::thread::sleep(Duration::from_millis(ms));
    0
}

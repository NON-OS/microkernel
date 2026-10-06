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

    pub fn expired(&self) -> bool {
        self.is_past(mk_uptime_ms() as u64)
    }
}

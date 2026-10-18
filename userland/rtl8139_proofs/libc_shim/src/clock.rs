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
//! millisecond short of `ms`, as it can on the machine.

use std::sync::OnceLock;
use std::time::Instant;

static START: OnceLock<Instant> = OnceLock::new();

/// Whole milliseconds since the first look.
pub fn uptime_ms() -> u64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as u64
}

#[derive(Clone, Copy)]
pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end_ms: uptime_ms().saturating_add(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        uptime_ms() >= self.end_ms
    }
}

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

use super::mk_uptime_ms;

/// A monotonic timeout deadline measured against the boot uptime clock, so it is
/// immune to wall-clock adjustments and usable before the wall clock is set. It
/// gives drivers a CPU-speed-independent timeout in place of a raw spin count.
///
/// A failed uptime read counts as expired, when set and when checked. Read as
/// zero, a deadline set on a good read was never reached once the clock
/// failed, and the wait hung instead of failing.
#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    end_ms: Option<u64>,
}

impl Deadline {
    /// A deadline `timeout_ms` milliseconds from the current uptime.
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self::from_read(read_uptime_ms(), timeout_ms)
    }

    /// A deadline `timeout_ms` after the clock read `now`; expired if it failed.
    pub const fn from_read(now: Option<u64>, timeout_ms: u64) -> Self {
        match now {
            Some(now) => Self::at(now.saturating_add(timeout_ms)),
            None => Self { end_ms: None },
        }
    }

    /// A deadline at an explicit monotonic timestamp (milliseconds since boot).
    pub const fn at(end_ms: u64) -> Self {
        Self { end_ms: Some(end_ms) }
    }

    /// True once the given uptime has reached the deadline.
    pub const fn is_past(&self, now_ms: u64) -> bool {
        match self.end_ms {
            Some(end) => now_ms >= end,
            None => true,
        }
    }

    /// True once the clock read `now` has reached the deadline, or failed.
    pub const fn reached(&self, now: Option<u64>) -> bool {
        match now {
            Some(now) => self.is_past(now),
            None => true,
        }
    }

    /// True once the current uptime has reached the deadline.
    pub fn expired(&self) -> bool {
        self.reached(read_uptime_ms())
    }
}

fn read_uptime_ms() -> Option<u64> {
    // A negative answer is the syscall's errno.
    u64::try_from(mk_uptime_ms()).ok()
}
